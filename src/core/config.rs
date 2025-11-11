//! Enterprise-grade configuration management system
//!
//! Provides hierarchical configuration with:
//! - Global settings
//! - Module-specific settings
//! - Security policies
//! - Logging configuration
//! - Network configuration

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Global Ferox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeroxConfig {
    pub global: GlobalConfig,
    pub modules: HashMap<String, ModuleConfig>,
    pub security: SecurityPolicy,
    pub logging: LogConfig,
    pub network: NetworkConfig,
}

impl FeroxConfig {
    /// Load configuration from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        toml::from_str(&contents)
            .with_context(|| "Failed to parse configuration file")
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .with_context(|| "Failed to serialize configuration")?;

        std::fs::write(path.as_ref(), contents)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;

        Ok(())
    }

    /// Get default configuration path
    pub fn default_config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Failed to get home directory")?;
        Ok(home.join(".ferox").join("config.toml"))
    }

    /// Load configuration from default location or create default
    pub fn load_or_default() -> Result<Self> {
        let config_path = Self::default_config_path()?;

        if config_path.exists() {
            Self::load_from_file(&config_path)
        } else {
            Ok(Self::default())
        }
    }

    /// Get module-specific configuration
    pub fn get_module_config(&self, module_path: &str) -> Option<&ModuleConfig> {
        self.modules.get(module_path)
    }

    /// Set module-specific configuration
    pub fn set_module_config(&mut self, module_path: String, config: ModuleConfig) {
        self.modules.insert(module_path, config);
    }
}

impl Default for FeroxConfig {
    fn default() -> Self {
        Self {
            global: GlobalConfig::default(),
            modules: HashMap::new(),
            security: SecurityPolicy::default(),
            logging: LogConfig::default(),
            network: NetworkConfig::default(),
        }
    }
}

/// Global configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Workspace directory for Ferox data
    pub workspace: PathBuf,

    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,

    /// Default timeout for operations (in seconds)
    pub default_timeout: u64,

    /// Enable verbose output
    pub verbose: bool,

    /// Enable debug mode
    pub debug: bool,

    /// Automatically save sessions
    pub auto_save_sessions: bool,

    /// Session database path
    pub session_db_path: Option<PathBuf>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        let workspace = dirs::home_dir()
            .map(|h| h.join(".ferox"))
            .unwrap_or_else(|| PathBuf::from(".ferox"));

        Self {
            workspace,
            max_concurrent_operations: 100,
            default_timeout: 30,
            verbose: false,
            debug: false,
            auto_save_sessions: true,
            session_db_path: None,
        }
    }
}

/// Module-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    /// Whether module is enabled
    pub enabled: bool,

    /// Default options for this module
    pub default_options: HashMap<String, String>,

    /// Module-specific settings
    pub settings: HashMap<String, serde_json::Value>,
}

impl ModuleConfig {
    pub fn new() -> Self {
        Self {
            enabled: true,
            default_options: HashMap::new(),
            settings: HashMap::new(),
        }
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            default_options: HashMap::new(),
            settings: HashMap::new(),
        }
    }

    pub fn with_default_option(mut self, key: String, value: String) -> Self {
        self.default_options.insert(key, value);
        self
    }
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Security policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Require confirmation for dangerous operations
    pub require_confirmation: bool,

    /// Enable audit logging
    pub audit_logging: bool,

    /// Audit log path
    pub audit_log_path: Option<PathBuf>,

    /// Maximum allowed concurrent sessions
    pub max_sessions: usize,

    /// Session timeout (in seconds)
    pub session_timeout: u64,

    /// Allowed module categories
    pub allowed_categories: Vec<String>,

    /// Blocked module paths
    pub blocked_modules: Vec<String>,

    /// Require authorization tokens
    pub require_auth: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            require_confirmation: true,
            audit_logging: true,
            audit_log_path: None,
            max_sessions: 100,
            session_timeout: 3600,
            allowed_categories: vec![
                "scanner".to_string(),
                "recon".to_string(),
                "exploit".to_string(),
                "post".to_string(),
                "auxiliary".to_string(),
                "c2".to_string(),
                "evasion".to_string(),
            ],
            blocked_modules: Vec::new(),
            require_auth: false,
        }
    }
}

impl SecurityPolicy {
    /// Check if a module is allowed to execute
    pub fn is_module_allowed(&self, module_path: &str, category: &str) -> bool {
        // Check if module is explicitly blocked
        if self.blocked_modules.iter().any(|b| b == module_path) {
            return false;
        }

        // Check if category is allowed
        self.allowed_categories.is_empty()
            || self.allowed_categories.iter().any(|c| c == category)
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,

    /// Log to file
    pub log_to_file: bool,

    /// Log file path
    pub log_file_path: Option<PathBuf>,

    /// Log format (json, pretty, compact)
    pub format: String,

    /// Include timestamps
    pub include_timestamps: bool,

    /// Include module name
    pub include_module: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            log_to_file: false,
            log_file_path: None,
            format: "pretty".to_string(),
            include_timestamps: true,
            include_module: true,
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Default user agent for HTTP requests
    pub user_agent: String,

    /// Connection timeout (in seconds)
    pub connection_timeout: u64,

    /// Request timeout (in seconds)
    pub request_timeout: u64,

    /// Maximum redirects to follow
    pub max_redirects: usize,

    /// Enable TLS verification
    pub verify_tls: bool,

    /// Proxy configuration
    pub proxy: Option<String>,

    /// DNS servers
    pub dns_servers: Vec<String>,

    /// Enable IPv6
    pub enable_ipv6: bool,

    /// Rate limiting (requests per second)
    pub rate_limit: Option<f64>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            user_agent: "Ferox/2.0.0".to_string(),
            connection_timeout: 10,
            request_timeout: 30,
            max_redirects: 10,
            verify_tls: true,
            proxy: None,
            dns_servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            enable_ipv6: true,
            rate_limit: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = FeroxConfig::default();
        assert!(config.global.max_concurrent_operations > 0);
        assert!(config.security.require_confirmation);
        assert!(config.logging.include_timestamps);
        assert_eq!(config.network.user_agent, "Ferox/2.0.0");
    }

    #[test]
    fn test_config_serialization() {
        let config = FeroxConfig::default();
        let serialized = toml::to_string_pretty(&config).unwrap();
        assert!(serialized.contains("[global]"));
        assert!(serialized.contains("[security]"));
        assert!(serialized.contains("[logging]"));
        assert!(serialized.contains("[network]"));
    }

    #[test]
    fn test_config_save_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let config_path = temp_file.path();

        let mut config = FeroxConfig::default();
        config.global.verbose = true;
        config.global.max_concurrent_operations = 200;

        config.save_to_file(&config_path).unwrap();

        let loaded = FeroxConfig::load_from_file(&config_path).unwrap();
        assert_eq!(loaded.global.verbose, true);
        assert_eq!(loaded.global.max_concurrent_operations, 200);
    }

    #[test]
    fn test_module_config() {
        let mut config = FeroxConfig::default();

        let module_config = ModuleConfig::new()
            .with_default_option("RHOSTS".to_string(), "127.0.0.1".to_string())
            .with_default_option("TIMEOUT".to_string(), "5000".to_string());

        config.set_module_config("scanner/port_scanner".to_string(), module_config);

        let retrieved = config.get_module_config("scanner/port_scanner");
        assert!(retrieved.is_some());
        assert_eq!(
            retrieved.unwrap().default_options.get("RHOSTS"),
            Some(&"127.0.0.1".to_string())
        );
    }

    #[test]
    fn test_security_policy_module_allowed() {
        let mut policy = SecurityPolicy::default();
        policy.blocked_modules.push("exploit/dangerous".to_string());

        assert!(policy.is_module_allowed("scanner/port_scanner", "scanner"));
        assert!(!policy.is_module_allowed("exploit/dangerous", "exploit"));
    }

    #[test]
    fn test_security_policy_category_restriction() {
        let mut policy = SecurityPolicy::default();
        policy.allowed_categories = vec!["scanner".to_string(), "recon".to_string()];

        assert!(policy.is_module_allowed("scanner/port", "scanner"));
        assert!(policy.is_module_allowed("recon/dns", "recon"));
        assert!(!policy.is_module_allowed("exploit/test", "exploit"));
    }
}
