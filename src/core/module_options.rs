//! Unified module options system
//!
//! Provides standard options and option management traits to reduce code duplication across modules.

use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Standard options used by most network-based modules
#[derive(Debug, Clone)]
pub struct StandardOptions {
    pub rhost: Option<String>,
    pub rport: Option<u16>,
    pub rhosts: Option<String>,
    pub timeout_ms: u64,
    pub threads: usize,
}

impl Default for StandardOptions {
    fn default() -> Self {
        Self {
            rhost: None,
            rport: None,
            rhosts: None,
            timeout_ms: 5000,
            threads: 10,
        }
    }
}

impl StandardOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate that required options are set
    pub fn validate_required(&self, require_target: bool) -> Result<()> {
        if require_target && self.rhost.is_none() && self.rhosts.is_none() {
            return Err(anyhow!("Either RHOST or RHOSTS must be set"));
        }
        Ok(())
    }

    /// Get the target host (RHOST or first from RHOSTS)
    pub fn get_target(&self) -> Option<String> {
        if let Some(rhost) = &self.rhost {
            Some(rhost.clone())
        } else if let Some(rhosts) = &self.rhosts {
            rhosts.split(',').next().map(|s| s.trim().to_string())
        } else {
            None
        }
    }

    /// Get all target hosts
    pub fn get_targets(&self) -> Vec<String> {
        if let Some(rhosts) = &self.rhosts {
            rhosts
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else if let Some(rhost) = &self.rhost {
            vec![rhost.clone()]
        } else {
            vec![]
        }
    }
}

/// Trait for managing module options
pub trait OptionManager {
    /// Validate options
    fn validate(&self) -> Result<()>;

    /// Set an option value
    fn set(&mut self, key: &str, value: &str) -> Result<()>;

    /// Get an option value
    fn get(&self, key: &str) -> Option<String>;

    /// List all options with their current values
    fn list(&self) -> HashMap<String, String>;
}

/// Helper for parsing common option values
pub struct OptionParser;

impl OptionParser {
    /// Parse a port number
    pub fn parse_port(value: &str) -> Result<u16> {
        value
            .parse::<u16>()
            .map_err(|_| anyhow!("Invalid port number: {}", value))
    }

    /// Parse a timeout value
    pub fn parse_timeout(value: &str) -> Result<u64> {
        value
            .parse::<u64>()
            .map_err(|_| anyhow!("Invalid timeout value: {}", value))
    }

    /// Parse thread count
    pub fn parse_threads(value: &str) -> Result<usize> {
        let threads = value
            .parse::<usize>()
            .map_err(|_| anyhow!("Invalid thread count: {}", value))?;

        if threads == 0 {
            return Err(anyhow!("Thread count must be greater than 0"));
        }

        if threads > 1000 {
            return Err(anyhow!("Thread count too high (max 1000)"));
        }

        Ok(threads)
    }

    /// Parse boolean value
    pub fn parse_bool(value: &str) -> Result<bool> {
        match value.to_lowercase().as_str() {
            "true" | "yes" | "1" | "y" => Ok(true),
            "false" | "no" | "0" | "n" => Ok(false),
            _ => Err(anyhow!("Invalid boolean value: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_options_default() {
        let opts = StandardOptions::default();
        assert_eq!(opts.timeout_ms, 5000);
        assert_eq!(opts.threads, 10);
        assert!(opts.rhost.is_none());
    }

    #[test]
    fn test_standard_options_get_target() {
        let mut opts = StandardOptions::default();
        assert!(opts.get_target().is_none());

        opts.rhost = Some("127.0.0.1".to_string());
        assert_eq!(opts.get_target(), Some("127.0.0.1".to_string()));

        opts.rhost = None;
        opts.rhosts = Some("10.0.0.1,10.0.0.2".to_string());
        assert_eq!(opts.get_target(), Some("10.0.0.1".to_string()));
    }

    #[test]
    fn test_standard_options_get_targets() {
        let mut opts = StandardOptions::default();
        assert_eq!(opts.get_targets().len(), 0);

        opts.rhosts = Some("10.0.0.1, 10.0.0.2, 10.0.0.3".to_string());
        let targets = opts.get_targets();
        assert_eq!(targets.len(), 3);
        assert_eq!(targets[0], "10.0.0.1");
        assert_eq!(targets[1], "10.0.0.2");
        assert_eq!(targets[2], "10.0.0.3");
    }

    #[test]
    fn test_option_parser_port() {
        assert_eq!(OptionParser::parse_port("80").unwrap(), 80);
        assert_eq!(OptionParser::parse_port("443").unwrap(), 443);
        assert!(OptionParser::parse_port("invalid").is_err());
        assert!(OptionParser::parse_port("99999").is_err());
    }

    #[test]
    fn test_option_parser_timeout() {
        assert_eq!(OptionParser::parse_timeout("1000").unwrap(), 1000);
        assert_eq!(OptionParser::parse_timeout("5000").unwrap(), 5000);
        assert!(OptionParser::parse_timeout("invalid").is_err());
    }

    #[test]
    fn test_option_parser_threads() {
        assert_eq!(OptionParser::parse_threads("10").unwrap(), 10);
        assert_eq!(OptionParser::parse_threads("100").unwrap(), 100);
        assert!(OptionParser::parse_threads("0").is_err());
        assert!(OptionParser::parse_threads("2000").is_err());
    }

    #[test]
    fn test_option_parser_bool() {
        assert!(OptionParser::parse_bool("true").unwrap());
        assert!(OptionParser::parse_bool("yes").unwrap());
        assert!(OptionParser::parse_bool("1").unwrap());
        assert!(!OptionParser::parse_bool("false").unwrap());
        assert!(!OptionParser::parse_bool("no").unwrap());
        assert!(!OptionParser::parse_bool("0").unwrap());
        assert!(OptionParser::parse_bool("invalid").is_err());
    }

    #[test]
    fn test_validate_required() {
        let mut opts = StandardOptions::default();
        assert!(opts.validate_required(false).is_ok());
        assert!(opts.validate_required(true).is_err());

        opts.rhost = Some("127.0.0.1".to_string());
        assert!(opts.validate_required(true).is_ok());
    }
}
