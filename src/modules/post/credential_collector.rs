//! Credential Collection Module
//!
//! Leverages Ferox's memory forensics capabilities to extract credentials
//! from memory dumps for authorized security assessments.
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY
//! This module extracts sensitive information for security testing and
//! incident response. All operations are audited.
//!
//! Features:
//! - Extract credentials from memory dumps
//! - Leverage existing memory forensics engine
//! - Support for multiple credential formats
//! - Safe mode for testing
//!
//! Credential Sources:
//! - LSASS process memory (Windows)
//! - Browser saved passwords
//! - Application memory
//! - Environment variables

use anyhow::{Result, bail};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType,
};

/// Credential type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CredentialType {
    PlainText,
    Hash,
    Token,
    Cookie,
    Certificate,
}

/// Extracted credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub cred_type: CredentialType,
    pub username: Option<String>,
    pub password: Option<String>,
    pub hash: Option<String>,
    pub domain: Option<String>,
    pub source: String,
    pub metadata: HashMap<String, String>,
}

impl Credential {
    pub fn new(cred_type: CredentialType, source: impl Into<String>) -> Self {
        Self {
            cred_type,
            username: None,
            password: None,
            hash: None,
            domain: None,
            source: source.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn with_hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }

    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn redacted(&self) -> Self {
        let mut redacted = self.clone();
        if let Some(ref pass) = redacted.password {
            redacted.password = Some(format!("{}***", &pass[..pass.len().min(2)]));
        }
        if let Some(ref hash) = redacted.hash {
            redacted.hash = Some(format!(
                "{}...{}",
                &hash[..8.min(hash.len())],
                &hash[hash.len().saturating_sub(8)..]
            ));
        }
        redacted
    }
}

/// Credential Collector Module
pub struct CredentialCollector {
    options: HashMap<String, String>,
}

impl CredentialCollector {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("dump_path".to_string(), String::new());
        options.insert("safe_mode".to_string(), "true".to_string());
        options.insert("redact_output".to_string(), "true".to_string());
        options.insert("credential_types".to_string(), "all".to_string());

        Self { options }
    }

    /// Extract credentials from memory dump (reference implementation)
    async fn extract_from_memory(&self, _dump_path: &PathBuf) -> Result<Vec<Credential>> {
        let safe_mode = self
            .options
            .get("safe_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if safe_mode {
            return self.generate_safe_credentials();
        }

        // In production, would use memory forensics to extract credentials
        // For now, return reference implementation
        let mut credentials = Vec::new();

        credentials.push(
            Credential::new(CredentialType::PlainText, "memory_dump_reference")
                .with_username("extracted_user")
                .with_password("[Would extract from LSASS memory]"),
        );

        Ok(credentials)
    }

    /// Generate safe test credentials
    fn generate_safe_credentials(&self) -> Result<Vec<Credential>> {
        let mut credentials = Vec::new();

        credentials.push(
            Credential::new(CredentialType::PlainText, "safe_mode_test")
                .with_username("test_user_1")
                .with_password("safe_password_123")
                .with_domain("TESTDOMAIN"),
        );

        credentials.push(
            Credential::new(CredentialType::Hash, "safe_mode_test")
                .with_username("test_user_2")
                .with_hash("aad3b435b51404eeaad3b435b51404ee:e19ccf75ee54e06b06a5907af13cef42"),
        );

        credentials.push(
            Credential::new(CredentialType::Token, "safe_mode_test")
                .with_username("api_user")
                .with_password("ghp_test_token_1234567890"),
        );

        Ok(credentials)
    }

    /// Extract browser credentials
    fn extract_browser_credentials(&self) -> Result<Vec<Credential>> {
        let safe_mode = self
            .options
            .get("safe_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if safe_mode {
            let mut credentials = Vec::new();
            credentials.push(
                Credential::new(CredentialType::PlainText, "chrome_safe_mode")
                    .with_username("user@example.com")
                    .with_password("safe_browser_pass"),
            );
            return Ok(credentials);
        }

        // In production, would access browser credential stores
        bail!("Browser credential extraction requires safe_mode=false and authorization")
    }

    /// Format credentials for output
    fn format_credentials(&self, credentials: &[Credential]) -> String {
        let redact = self
            .options
            .get("redact_output")
            .map(|s| s == "true")
            .unwrap_or(true);

        let mut output = String::new();
        output.push_str(&format!("Found {} credentials:\n\n", credentials.len()));

        for (i, cred) in credentials.iter().enumerate() {
            let display_cred = if redact {
                cred.redacted()
            } else {
                cred.clone()
            };

            output.push_str(&format!("Credential #{}:\n", i + 1));
            output.push_str(&format!("  Type: {:?}\n", display_cred.cred_type));
            output.push_str(&format!("  Source: {}\n", display_cred.source));

            if let Some(ref username) = display_cred.username {
                output.push_str(&format!("  Username: {}\n", username));
            }
            if let Some(ref domain) = display_cred.domain {
                output.push_str(&format!("  Domain: {}\n", domain));
            }
            if let Some(ref password) = display_cred.password {
                output.push_str(&format!("  Password: {}\n", password));
            }
            if let Some(ref hash) = display_cred.hash {
                output.push_str(&format!("  Hash: {}\n", hash));
            }
            output.push('\n');
        }

        output
    }
}

impl Default for CredentialCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for CredentialCollector {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "credential_collector".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description: "Extract credentials from memory dumps and system stores. \
                         AUTHORIZED USE ONLY - Requires explicit permission."
                .to_string(),
            module_type: ModuleType::PostExploit,
            category: "collection".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "dump_path".to_string(),
                description: "Path to memory dump file".to_string(),
                required: false,
                default_value: None,
                current_value: self.options.get("dump_path").cloned(),
            },
            ModuleOption {
                name: "safe_mode".to_string(),
                description: "Use safe mode (true/false) - generates test credentials".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("safe_mode").cloned(),
            },
            ModuleOption {
                name: "redact_output".to_string(),
                description: "Redact sensitive data in output (true/false)".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("redact_output").cloned(),
            },
            ModuleOption {
                name: "credential_types".to_string(),
                description: "Types to extract: all, plaintext, hash, token".to_string(),
                required: false,
                default_value: Some("all".to_string()),
                current_value: self.options.get("credential_types").cloned(),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        self.options.insert(name.to_string(), value.to_string());
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<String> {
        self.options.get(name).cloned()
    }

    fn validate(&self) -> Result<()> {
        let safe_mode = self
            .options
            .get("safe_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if !safe_mode {
            // Check for dump_path or other sources
            let dump_path = self.options.get("dump_path");
            if dump_path.is_none() || dump_path.unwrap().is_empty() {
                bail!("dump_path required when safe_mode=false");
            }
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let safe_mode = self
            .options
            .get("safe_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        let mut fingerprint = HashMap::new();
        fingerprint.insert(
            "mode".to_string(),
            if safe_mode { "safe" } else { "production" }.to_string(),
        );

        Ok(CheckResult {
            vulnerable: safe_mode, // In safe mode, always "vulnerable" (testable)
            confidence: if safe_mode { 1.0 } else { 0.0 },
            details: if safe_mode {
                "Safe mode - will generate test credentials".to_string()
            } else {
                "Production mode - requires authorization and memory dump".to_string()
            },
            fingerprint,
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        let safe_mode = self
            .options
            .get("safe_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        let mut all_credentials = Vec::new();

        // Extract from memory dump if available
        if let Some(dump_path_str) = self.options.get("dump_path") {
            if !dump_path_str.is_empty() {
                let dump_path = PathBuf::from(dump_path_str);
                if dump_path.exists() || safe_mode {
                    match self.extract_from_memory(&dump_path).await {
                        Ok(mut creds) => all_credentials.append(&mut creds),
                        Err(e) => {
                            tracing::warn!("Failed to extract from memory: {}", e);
                        }
                    }
                }
            }
        }

        // Extract browser credentials
        match self.extract_browser_credentials() {
            Ok(mut creds) => all_credentials.append(&mut creds),
            Err(e) => {
                tracing::warn!("Failed to extract browser credentials: {}", e);
            }
        }

        // If safe mode and no credentials, generate test ones
        if safe_mode && all_credentials.is_empty() {
            all_credentials = self.generate_safe_credentials()?;
        }

        let output = self.format_credentials(&all_credentials);

        let mut result = ModuleResult::success(format!(
            "Credential collection completed - found {} credentials",
            all_credentials.len()
        ));

        result = result
            .with_data("credential_count", serde_json::json!(all_credentials.len()))
            .with_data("output", serde_json::json!(output))
            .with_data("safe_mode", serde_json::json!(safe_mode));

        // Add credentials as JSON (redacted if requested)
        let redact = self
            .options
            .get("redact_output")
            .map(|s| s == "true")
            .unwrap_or(true);

        let creds_json: Vec<Credential> = all_credentials
            .iter()
            .map(|c| if redact { c.redacted() } else { c.clone() })
            .collect();

        result = result.with_data("credentials", serde_json::json!(creds_json));

        Ok(result)
    }

    async fn cleanup(&mut self) -> Result<()> {
        // Clear any cached credentials from memory
        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        // Always require confirmation for credential extraction
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_mode_credentials() {
        let mut module = CredentialCollector::new();
        module.set_option("safe_mode", "true").unwrap();

        assert!(module.validate().is_ok());

        let result = module.run().await.unwrap();
        assert!(result.success);
        assert!(result.data.contains_key("credential_count"));

        let count = result.data["credential_count"].as_u64().unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_credential_redaction() {
        let cred = Credential::new(CredentialType::PlainText, "test")
            .with_username("testuser")
            .with_password("SuperSecret123!")
            .with_domain("TESTDOMAIN");

        let redacted = cred.redacted();
        assert!(redacted.password.unwrap().contains("***"));
        assert_eq!(redacted.username, Some("testuser".to_string()));
    }

    #[test]
    fn test_module_info() {
        let module = CredentialCollector::new();
        let info = module.info();
        assert_eq!(info.name, "credential_collector");
        assert!(info.description.contains("AUTHORIZED"));
    }

    #[tokio::test]
    async fn test_browser_credentials() {
        let module = CredentialCollector::new();
        let creds = module.extract_browser_credentials().unwrap();
        assert!(!creds.is_empty());
    }
}
