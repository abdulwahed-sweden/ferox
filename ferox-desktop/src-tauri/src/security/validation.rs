//! Input Validation
//!
//! Validates all inputs to Tauri commands before processing.

use thiserror::Error;

/// Maximum command length (10KB)
pub const MAX_COMMAND_LENGTH: usize = 10_000;

/// Maximum session ID length
pub const MAX_SESSION_ID_LENGTH: usize = 64;

/// Maximum hostname length
pub const MAX_HOSTNAME_LENGTH: usize = 255;

/// Maximum IP address length
pub const MAX_IP_LENGTH: usize = 45; // IPv6 max

/// Maximum note length (10KB)
pub const MAX_NOTE_LENGTH: usize = 10_000;

/// Maximum terminal data length per write (1MB)
pub const MAX_TERMINAL_DATA_LENGTH: usize = 1_048_576;

/// Validation errors
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Session ID is required")]
    EmptySessionId,

    #[error("Session ID too long (max {MAX_SESSION_ID_LENGTH} chars)")]
    SessionIdTooLong,

    #[error("Invalid session ID format")]
    InvalidSessionIdFormat,

    #[error("Command is required")]
    EmptyCommand,

    #[error("Command too long (max {MAX_COMMAND_LENGTH} chars)")]
    CommandTooLong,

    #[error("Hostname is required")]
    EmptyHostname,

    #[error("Hostname too long (max {MAX_HOSTNAME_LENGTH} chars)")]
    HostnameTooLong,

    #[error("IP address is required")]
    EmptyIpAddress,

    #[error("IP address too long (max {MAX_IP_LENGTH} chars)")]
    IpAddressTooLong,

    #[error("Invalid IP address format")]
    InvalidIpFormat,

    #[error("Terminal ID is required")]
    EmptyTerminalId,

    #[error("Terminal data too large (max {MAX_TERMINAL_DATA_LENGTH} bytes)")]
    TerminalDataTooLarge,

    #[error("Note too long (max {MAX_NOTE_LENGTH} chars)")]
    NoteTooLong,

    #[error("Invalid port number")]
    InvalidPort,

    #[error("Target host is required")]
    EmptyTargetHost,

    #[error("Method is required")]
    EmptyMethod,

    #[error("Persistence name is required")]
    EmptyPersistenceName,
}

impl From<ValidationError> for String {
    fn from(e: ValidationError) -> Self {
        e.to_string()
    }
}

/// Input validator
pub struct Validator;

impl Validator {
    /// Validate a session ID
    pub fn session_id(id: &str) -> Result<(), ValidationError> {
        if id.is_empty() {
            return Err(ValidationError::EmptySessionId);
        }
        if id.len() > MAX_SESSION_ID_LENGTH {
            return Err(ValidationError::SessionIdTooLong);
        }
        // Allow alphanumeric, hyphens, and underscores
        if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(ValidationError::InvalidSessionIdFormat);
        }
        Ok(())
    }

    /// Validate a command string
    pub fn command(cmd: &str) -> Result<(), ValidationError> {
        if cmd.is_empty() {
            return Err(ValidationError::EmptyCommand);
        }
        if cmd.len() > MAX_COMMAND_LENGTH {
            return Err(ValidationError::CommandTooLong);
        }
        Ok(())
    }

    /// Validate a hostname
    pub fn hostname(hostname: &str) -> Result<(), ValidationError> {
        if hostname.is_empty() {
            return Err(ValidationError::EmptyHostname);
        }
        if hostname.len() > MAX_HOSTNAME_LENGTH {
            return Err(ValidationError::HostnameTooLong);
        }
        Ok(())
    }

    /// Validate an IP address
    pub fn ip_address(ip: &str) -> Result<(), ValidationError> {
        if ip.is_empty() {
            return Err(ValidationError::EmptyIpAddress);
        }
        if ip.len() > MAX_IP_LENGTH {
            return Err(ValidationError::IpAddressTooLong);
        }
        // Basic IP format validation (allows IPv4 and IPv6)
        if !ip.chars().all(|c| c.is_alphanumeric() || c == '.' || c == ':') {
            return Err(ValidationError::InvalidIpFormat);
        }
        Ok(())
    }

    /// Validate a terminal ID
    pub fn terminal_id(id: &str) -> Result<(), ValidationError> {
        if id.is_empty() {
            return Err(ValidationError::EmptyTerminalId);
        }
        // Reuse session ID validation rules
        if id.len() > MAX_SESSION_ID_LENGTH {
            return Err(ValidationError::SessionIdTooLong);
        }
        Ok(())
    }

    /// Validate terminal data
    pub fn terminal_data(data: &str) -> Result<(), ValidationError> {
        if data.len() > MAX_TERMINAL_DATA_LENGTH {
            return Err(ValidationError::TerminalDataTooLarge);
        }
        Ok(())
    }

    /// Validate a note
    pub fn note(note: &Option<String>) -> Result<(), ValidationError> {
        if let Some(n) = note {
            if n.len() > MAX_NOTE_LENGTH {
                return Err(ValidationError::NoteTooLong);
            }
        }
        Ok(())
    }

    /// Validate a target host
    pub fn target_host(host: &str) -> Result<(), ValidationError> {
        if host.is_empty() {
            return Err(ValidationError::EmptyTargetHost);
        }
        Self::hostname(host)?;
        Ok(())
    }

    /// Validate a method name
    pub fn method(method: &str) -> Result<(), ValidationError> {
        if method.is_empty() {
            return Err(ValidationError::EmptyMethod);
        }
        Ok(())
    }

    /// Validate persistence name
    pub fn persistence_name(name: &str) -> Result<(), ValidationError> {
        if name.is_empty() {
            return Err(ValidationError::EmptyPersistenceName);
        }
        Ok(())
    }

    /// Validate port numbers
    pub fn ports(ports: &Option<Vec<u16>>) -> Result<(), ValidationError> {
        if let Some(p) = ports {
            for port in p {
                if *port == 0 {
                    return Err(ValidationError::InvalidPort);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_validation() {
        assert!(Validator::session_id("valid-id_123").is_ok());
        assert!(Validator::session_id("").is_err());
        assert!(Validator::session_id(&"a".repeat(100)).is_err());
        assert!(Validator::session_id("invalid id!").is_err());
    }

    #[test]
    fn test_command_validation() {
        assert!(Validator::command("whoami").is_ok());
        assert!(Validator::command("").is_err());
        assert!(Validator::command(&"a".repeat(20000)).is_err());
    }
}
