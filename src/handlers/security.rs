use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use uuid::Uuid;

/// Security policy for file operations
#[derive(Clone, Debug)]
pub struct FileAccessPolicy {
    /// Allowed root directories (whitelist)
    pub allowed_roots: Vec<PathBuf>,
    /// Blocked paths (blacklist)
    pub blocked_paths: Vec<PathBuf>,
    /// Maximum file size for upload/download (bytes)
    pub max_file_size: u64,
    /// Enable sandbox mode
    pub sandbox_enabled: bool,
}

impl Default for FileAccessPolicy {
    fn default() -> Self {
        Self {
            allowed_roots: vec![
                PathBuf::from("/tmp"),
                PathBuf::from("/home"),
                std::env::temp_dir(),
            ],
            blocked_paths: vec![
                PathBuf::from("/etc/shadow"),
                PathBuf::from("/etc/passwd"),
                PathBuf::from("/boot"),
                PathBuf::from("/sys"),
                PathBuf::from("/proc"),
                PathBuf::from("/root/.ssh"),
            ],
            max_file_size: 100 * 1024 * 1024, // 100 MB
            sandbox_enabled: true,
        }
    }
}

impl FileAccessPolicy {
    /// Check if path is allowed for access
    pub fn is_path_allowed(&self, path: &Path) -> Result<()> {
        if !self.sandbox_enabled {
            return Ok(());
        }

        let canonical_path = path
            .canonicalize()
            .map_err(|_| anyhow!("Path does not exist or cannot be accessed"))?;

        // Check blacklist first
        for blocked in &self.blocked_paths {
            if canonical_path.starts_with(blocked) {
                return Err(anyhow!("Access denied: path is blocked"));
            }
        }

        // Check whitelist (if configured)
        if !self.allowed_roots.is_empty() {
            let allowed = self
                .allowed_roots
                .iter()
                .any(|root| canonical_path.starts_with(root));

            if !allowed {
                return Err(anyhow!("Access denied: path outside allowed roots"));
            }
        }

        Ok(())
    }

    /// Check if file size is within limits
    pub fn is_file_size_allowed(&self, size: u64) -> Result<()> {
        if size > self.max_file_size {
            return Err(anyhow!(
                "File size {} exceeds maximum allowed size {}",
                size,
                self.max_file_size
            ));
        }
        Ok(())
    }
}

/// Command execution policy
#[derive(Clone, Debug)]
pub struct CommandExecutionPolicy {
    /// Blocked commands
    pub blocked_commands: Vec<String>,
    /// Blocked command patterns (simple substring matching)
    pub blocked_patterns: Vec<String>,
    /// Maximum command length
    pub max_command_length: usize,
    /// Enable command validation
    pub validation_enabled: bool,
}

impl Default for CommandExecutionPolicy {
    fn default() -> Self {
        Self {
            blocked_commands: vec![
                "rm -rf /".to_string(),
                ":(){ :|:& };:".to_string(), // Fork bomb
                "dd if=/dev/zero of=/dev/sda".to_string(),
                "mkfs".to_string(),
            ],
            blocked_patterns: vec![
                "rm -rf /".to_string(),
                "dd if=/dev/zero".to_string(),
                "mkfs.".to_string(),
                "format ".to_string(),
            ],
            max_command_length: 4096,
            validation_enabled: true,
        }
    }
}

impl CommandExecutionPolicy {
    /// Validate command before execution
    pub fn validate_command(&self, command: &str) -> Result<()> {
        if !self.validation_enabled {
            return Ok(());
        }

        // Check length
        if command.len() > self.max_command_length {
            return Err(anyhow!("Command exceeds maximum length"));
        }

        // Check blocked commands (exact match)
        let cmd_lower = command.to_lowercase();
        for blocked in &self.blocked_commands {
            if cmd_lower == blocked.to_lowercase() {
                return Err(anyhow!("Command is explicitly blocked: {}", blocked));
            }
        }

        // Check substring patterns
        for pattern in &self.blocked_patterns {
            if cmd_lower.contains(&pattern.to_lowercase()) {
                return Err(anyhow!("Command contains blocked pattern: {}", pattern));
            }
        }

        Ok(())
    }
}

/// Audit logger for security events
pub struct AuditLogger {
    log_file: Option<PathBuf>,
    log_to_stdout: bool,
}

impl AuditLogger {
    pub fn new(log_file: Option<PathBuf>, log_to_stdout: bool) -> Self {
        Self {
            log_file,
            log_to_stdout,
        }
    }

    pub async fn log_command_execution(&self, handler_id: Uuid, command: &str, result: &str) {
        let entry = format!(
            "[{}] EXEC handler={} command=\"{}\" result=\"{}\"",
            chrono::Utc::now().to_rfc3339(),
            handler_id,
            command.replace('"', "\\\""),
            result
        );

        self.write_log(&entry).await;
    }

    pub async fn log_file_access(&self, operation: &str, path: &str, success: bool) {
        let entry = format!(
            "[{}] FILE operation={} path=\"{}\" success={}",
            chrono::Utc::now().to_rfc3339(),
            operation,
            path,
            success
        );

        self.write_log(&entry).await;
    }

    pub async fn log_shell_connection(&self, shell_type: &str, address: &str, success: bool) {
        let entry = format!(
            "[{}] SHELL type={} address={} success={}",
            chrono::Utc::now().to_rfc3339(),
            shell_type,
            address,
            success
        );

        self.write_log(&entry).await;
    }

    async fn write_log(&self, entry: &str) {
        // Log to stdout
        if self.log_to_stdout {
            println!("[AUDIT] {}", entry);
        }

        // Log to file if configured
        if let Some(log_path) = &self.log_file {
            if let Ok(mut file) = tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
                .await
            {
                use tokio::io::AsyncWriteExt;
                let _ = file.write_all(format!("{}\n", entry).as_bytes()).await;
            }
        }
    }
}

/// Rate limiter for command execution
pub struct RateLimiter {
    limits: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    /// Check if rate limit allows request
    pub async fn check_rate_limit(&self, key: &str) -> Result<()> {
        let mut limits = self.limits.lock().await;
        let now = Instant::now();

        let requests = limits.entry(key.to_string()).or_insert_with(Vec::new);

        // Remove old requests outside window
        requests.retain(|&time| now.duration_since(time) < self.window);

        // Check limit
        if requests.len() >= self.max_requests {
            return Err(anyhow!(
                "Rate limit exceeded: max {} requests per {:?}",
                self.max_requests,
                self.window
            ));
        }

        requests.push(now);
        Ok(())
    }

    /// Get current request count for key
    pub async fn get_request_count(&self, key: &str) -> usize {
        let mut limits = self.limits.lock().await;
        let now = Instant::now();

        if let Some(requests) = limits.get_mut(key) {
            requests.retain(|&time| now.duration_since(time) < self.window);
            requests.len()
        } else {
            0
        }
    }
}

/// Security configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecurityConfig {
    pub file_access: FileAccessPolicyConfig,
    pub command_execution: CommandExecutionPolicyConfig,
    pub audit: AuditConfig,
    pub remote_shell: RemoteShellConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileAccessPolicyConfig {
    pub sandbox_enabled: bool,
    pub max_file_size: u64,
    pub allowed_roots: Vec<String>,
    pub blocked_paths: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommandExecutionPolicyConfig {
    pub validation_enabled: bool,
    pub max_command_length: usize,
    pub blocked_commands: Vec<String>,
    pub blocked_patterns: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuditConfig {
    pub enabled: bool,
    pub log_file: Option<String>,
    pub log_to_stdout: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RemoteShellConfig {
    pub require_auth: bool,
    pub auth_token: String,
    pub enable_tls: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
}

impl SecurityConfig {
    /// Load configuration from TOML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents =
            std::fs::read_to_string(path).context("Failed to read security configuration file")?;
        let config = toml::from_str(&contents).context("Failed to parse security configuration")?;
        Ok(config)
    }

    /// Load from file or use defaults
    pub fn load_or_default() -> Self {
        Self::load_from_file("ferox_security.toml").unwrap_or_else(|_| {
            eprintln!("Warning: Failed to load ferox_security.toml, using defaults");
            Self::default()
        })
    }

    /// Convert to FileAccessPolicy
    pub fn to_file_policy(&self) -> FileAccessPolicy {
        FileAccessPolicy {
            allowed_roots: self
                .file_access
                .allowed_roots
                .iter()
                .map(PathBuf::from)
                .collect(),
            blocked_paths: self
                .file_access
                .blocked_paths
                .iter()
                .map(PathBuf::from)
                .collect(),
            max_file_size: self.file_access.max_file_size,
            sandbox_enabled: self.file_access.sandbox_enabled,
        }
    }

    /// Convert to CommandExecutionPolicy
    pub fn to_command_policy(&self) -> CommandExecutionPolicy {
        CommandExecutionPolicy {
            blocked_commands: self.command_execution.blocked_commands.clone(),
            blocked_patterns: self.command_execution.blocked_patterns.clone(),
            max_command_length: self.command_execution.max_command_length,
            validation_enabled: self.command_execution.validation_enabled,
        }
    }

    /// Create AuditLogger from config
    pub fn to_audit_logger(&self) -> Option<AuditLogger> {
        if self.audit.enabled {
            Some(AuditLogger::new(
                self.audit.log_file.as_ref().map(PathBuf::from),
                self.audit.log_to_stdout,
            ))
        } else {
            None
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            file_access: FileAccessPolicyConfig {
                sandbox_enabled: true,
                max_file_size: 100 * 1024 * 1024,
                allowed_roots: vec!["/tmp".to_string(), "/home".to_string()],
                blocked_paths: vec![
                    "/etc/shadow".to_string(),
                    "/etc/passwd".to_string(),
                    "/boot".to_string(),
                    "/sys".to_string(),
                    "/proc".to_string(),
                ],
            },
            command_execution: CommandExecutionPolicyConfig {
                validation_enabled: true,
                max_command_length: 4096,
                blocked_commands: vec!["rm -rf /".to_string()],
                blocked_patterns: vec!["rm -rf /".to_string(), "mkfs.".to_string()],
            },
            audit: AuditConfig {
                enabled: true,
                log_file: Some("/var/log/ferox_audit.log".to_string()),
                log_to_stdout: true,
            },
            remote_shell: RemoteShellConfig {
                require_auth: true,
                auth_token: "CHANGE_ME_IN_PRODUCTION".to_string(),
                enable_tls: false,
                tls_cert_path: None,
                tls_key_path: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_access_policy_blocked_paths() {
        let policy = FileAccessPolicy::default();

        // Should block /etc/shadow
        let result = policy.is_path_allowed(Path::new("/etc/shadow"));
        assert!(result.is_err());
    }

    #[test]
    fn test_command_validation() {
        let policy = CommandExecutionPolicy::default();

        // Should block dangerous commands
        assert!(policy.validate_command("rm -rf /").is_err());
        assert!(policy.validate_command(":(){ :|:& };:").is_err());

        // Should allow safe commands
        assert!(policy.validate_command("ls -la").is_ok());
        assert!(policy.validate_command("echo hello").is_ok());
    }

    #[test]
    fn test_command_length_limit() {
        let policy = CommandExecutionPolicy::default();

        let long_command = "a".repeat(5000);
        assert!(policy.validate_command(&long_command).is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(3, Duration::from_secs(1));

        // First 3 requests should succeed
        assert!(limiter.check_rate_limit("test").await.is_ok());
        assert!(limiter.check_rate_limit("test").await.is_ok());
        assert!(limiter.check_rate_limit("test").await.is_ok());

        // 4th request should fail
        assert!(limiter.check_rate_limit("test").await.is_err());

        // Wait for window to expire
        tokio::time::sleep(Duration::from_millis(1100)).await;

        // Should succeed again
        assert!(limiter.check_rate_limit("test").await.is_ok());
    }

    #[test]
    fn test_security_config_serialization() {
        let config = SecurityConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("sandbox_enabled"));
        assert!(toml_str.contains("validation_enabled"));
    }
}
