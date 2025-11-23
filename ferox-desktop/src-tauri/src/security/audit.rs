//! Audit Logging
//!
//! Logs all security-relevant events for operational security.

use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// Audit event types
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    // Session events
    SessionCreated,
    SessionTerminated,
    SessionPrivilegeChange,

    // Command events
    CommandExecuted,
    CommandFailed,

    // Module events
    PrivescAttempt,
    CredentialHarvest,
    PersistenceInstall,
    LateralMovement,
    NetworkDiscovery,

    // Terminal events
    TerminalCreated,
    TerminalClosed,

    // Security events
    ValidationFailed,
    UnauthorizedAccess,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub session_id: Option<String>,
    pub details: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl AuditEntry {
    pub fn new(event_type: AuditEventType, details: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            event_type,
            session_id: None,
            details: details.into(),
            metadata: None,
        }
    }

    pub fn with_session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Audit logger for security events
pub struct AuditLogger {
    file: Mutex<Option<File>>,
    #[allow(dead_code)]
    log_path: PathBuf,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new() -> Self {
        let log_path = Self::default_log_path();
        let file = Self::open_log_file(&log_path);

        Self {
            file: Mutex::new(file),
            log_path,
        }
    }

    /// Get default log path (~/.ferox/audit.log)
    fn default_log_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".ferox")
            .join("audit.log")
    }

    /// Open or create the log file
    fn open_log_file(path: &PathBuf) -> Option<File> {
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .ok()
    }

    /// Log an audit entry
    pub fn log(&self, entry: AuditEntry) {
        // Log to tracing first
        tracing::info!(
            target: "audit",
            event_type = ?entry.event_type,
            session_id = ?entry.session_id,
            details = %entry.details,
            "Audit event"
        );

        // Write to file
        let mut file_guard = self.file.lock();
        if let Some(ref mut file) = *file_guard {
            if let Ok(json) = serde_json::to_string(&entry) {
                let _ = writeln!(file, "{}", json);
                let _ = file.flush();
            }
        }
    }

    /// Log a session creation event
    pub fn log_session_created(&self, session_id: &str, hostname: &str, ip: &str) {
        self.log(
            AuditEntry::new(
                AuditEventType::SessionCreated,
                format!("Session created: {} ({})", hostname, ip),
            )
            .with_session(session_id),
        );
    }

    /// Log a session termination event
    pub fn log_session_terminated(&self, session_id: &str) {
        self.log(
            AuditEntry::new(AuditEventType::SessionTerminated, "Session terminated")
                .with_session(session_id),
        );
    }

    /// Log a command execution
    pub fn log_command_executed(&self, session_id: &str, command: &str, success: bool) {
        let event_type = if success {
            AuditEventType::CommandExecuted
        } else {
            AuditEventType::CommandFailed
        };

        // Truncate command for logging (don't log full command in case of sensitive data)
        let truncated_cmd = if command.len() > 100 {
            format!("{}...", &command[..100])
        } else {
            command.to_string()
        };

        self.log(
            AuditEntry::new(event_type, format!("Command: {}", truncated_cmd))
                .with_session(session_id),
        );
    }

    /// Log a privilege escalation attempt
    pub fn log_privesc_attempt(&self, session_id: &str, success: bool) {
        self.log(
            AuditEntry::new(
                AuditEventType::PrivescAttempt,
                format!("Privilege escalation attempt: {}", if success { "SUCCESS" } else { "FAILED" }),
            )
            .with_session(session_id),
        );
    }

    /// Log credential harvesting
    pub fn log_credential_harvest(&self, session_id: &str, count: usize) {
        self.log(
            AuditEntry::new(
                AuditEventType::CredentialHarvest,
                format!("Credentials harvested: {}", count),
            )
            .with_session(session_id),
        );
    }

    /// Log persistence installation
    pub fn log_persistence_install(&self, session_id: &str, method: &str, success: bool) {
        self.log(
            AuditEntry::new(
                AuditEventType::PersistenceInstall,
                format!("Persistence via {}: {}", method, if success { "SUCCESS" } else { "FAILED" }),
            )
            .with_session(session_id),
        );
    }

    /// Log lateral movement
    pub fn log_lateral_move(&self, session_id: &str, target: &str, success: bool) {
        self.log(
            AuditEntry::new(
                AuditEventType::LateralMovement,
                format!("Lateral movement to {}: {}", target, if success { "SUCCESS" } else { "FAILED" }),
            )
            .with_session(session_id),
        );
    }

    /// Log network discovery
    pub fn log_network_discovery(&self, session_id: &str, hosts_found: usize) {
        self.log(
            AuditEntry::new(
                AuditEventType::NetworkDiscovery,
                format!("Network discovery: {} hosts found", hosts_found),
            )
            .with_session(session_id),
        );
    }

    /// Log terminal creation
    pub fn log_terminal_created(&self, terminal_id: &str, session_id: &str) {
        self.log(
            AuditEntry::new(
                AuditEventType::TerminalCreated,
                format!("Terminal {} created", terminal_id),
            )
            .with_session(session_id),
        );
    }

    /// Log terminal closure
    pub fn log_terminal_closed(&self, terminal_id: &str) {
        self.log(AuditEntry::new(
            AuditEventType::TerminalClosed,
            format!("Terminal {} closed", terminal_id),
        ));
    }

    /// Log validation failure
    pub fn log_validation_failed(&self, details: &str) {
        self.log(AuditEntry::new(
            AuditEventType::ValidationFailed,
            details,
        ));
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}
