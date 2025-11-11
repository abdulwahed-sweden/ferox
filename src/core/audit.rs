//! Audit logging for security-critical operations
//!
//! Maintains an append-only log of module executions requiring confirmation.

use anyhow::{Context, Result};
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Audit log entry for module execution confirmation
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub module_name: String,
    pub module_category: String,
    pub user: String,
    pub confirmed: bool,
}

impl AuditEntry {
    /// Format as append-only log line
    pub fn to_log_line(&self) -> String {
        format!(
            "{} | {} | {}/{} | confirmed={}\n",
            self.timestamp.to_rfc3339(),
            self.user,
            self.module_category,
            self.module_name,
            self.confirmed
        )
    }
}

/// Get the default audit log path
pub fn get_audit_log_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Failed to get home directory")?;
    let ferox_dir = home.join(".ferox");

    // Ensure directory exists
    std::fs::create_dir_all(&ferox_dir)
        .context("Failed to create .ferox directory")?;

    Ok(ferox_dir.join("audit.log"))
}

/// Append confirmation entry to audit log
pub fn append_confirmation(
    module_name: &str,
    module_category: &str,
    user: &str,
    confirmed: bool,
) -> Result<()> {
    let log_path = get_audit_log_path()?;

    let entry = AuditEntry {
        timestamp: Utc::now(),
        module_name: module_name.to_string(),
        module_category: module_category.to_string(),
        user: user.to_string(),
        confirmed,
    };

    append_entry(&log_path, &entry)
}

/// Append audit entry to log file
fn append_entry(log_path: &Path, entry: &AuditEntry) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .with_context(|| format!("Failed to open audit log: {}", log_path.display()))?;

    file.write_all(entry.to_log_line().as_bytes())
        .context("Failed to write audit log entry")?;

    file.sync_all()
        .context("Failed to sync audit log")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_audit_entry_format() {
        let entry = AuditEntry {
            timestamp: chrono::DateTime::parse_from_rfc3339("2025-01-15T10:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
            module_name: "teams_tunnel".to_string(),
            module_category: "c2".to_string(),
            user: "testuser".to_string(),
            confirmed: true,
        };

        let line = entry.to_log_line();
        assert!(line.contains("2025-01-15T10:30:00"));
        assert!(line.contains("testuser"));
        assert!(line.contains("c2/teams_tunnel"));
        assert!(line.contains("confirmed=true"));
    }

    #[test]
    fn test_append_entry() {
        let temp_file = NamedTempFile::new().unwrap();
        let log_path = temp_file.path();

        let entry = AuditEntry {
            timestamp: Utc::now(),
            module_name: "test_module".to_string(),
            module_category: "test".to_string(),
            user: "testuser".to_string(),
            confirmed: true,
        };

        append_entry(log_path, &entry).unwrap();

        let content = fs::read_to_string(log_path).unwrap();
        assert!(content.contains("test/test_module"));
        assert!(content.contains("confirmed=true"));
    }

    #[test]
    fn test_multiple_appends() {
        let temp_file = NamedTempFile::new().unwrap();
        let log_path = temp_file.path();

        for i in 0..3 {
            let entry = AuditEntry {
                timestamp: Utc::now(),
                module_name: format!("module_{}", i),
                module_category: "test".to_string(),
                user: "testuser".to_string(),
                confirmed: i % 2 == 0,
            };
            append_entry(log_path, &entry).unwrap();
        }

        let content = fs::read_to_string(log_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains("module_0"));
        assert!(lines[1].contains("module_1"));
        assert!(lines[2].contains("module_2"));
    }
}
