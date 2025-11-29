//! Credential Harvesting and Dumping Modules
//!
//! SECURITY NOTICE: For authorized security testing only.

use crate::core::module::*;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;

/// Macro for credential modules
macro_rules! define_creds_module {
    ($struct_name:ident, $name:expr, $category:expr, $description:expr, $mitre:expr) => {
        pub struct $struct_name {
            options: HashMap<String, String>,
        }

        impl $struct_name {
            pub fn new() -> Self {
                let mut options = HashMap::new();
                options.insert("SESSION".to_string(), String::new());
                options.insert("OUTPUT_FORMAT".to_string(), "json".to_string());
                options.insert("OUTPUT_PATH".to_string(), "./loot/creds/".to_string());
                Self { options }
            }
        }

        #[async_trait]
        impl Module for $struct_name {
            fn info(&self) -> ModuleInfo {
                ModuleInfo {
                    name: $name.to_string(),
                    version: "1.0.0".to_string(),
                    author: "Ferox Team".to_string(),
                    description: format!("{} MITRE: {}", $description, $mitre),
                    module_type: ModuleType::PostExploit,
                    category: $category.to_string(),
                }
            }

            fn options(&self) -> Vec<ModuleOption> {
                vec![
                    ModuleOption {
                        name: "SESSION".to_string(),
                        description: "Session ID".to_string(),
                        required: false,
                        default_value: None,
                        current_value: self.get_option("SESSION"),
                    },
                    ModuleOption {
                        name: "OUTPUT_FORMAT".to_string(),
                        description: "Output format".to_string(),
                        required: false,
                        default_value: Some("json".to_string()),
                        current_value: self.get_option("OUTPUT_FORMAT"),
                    },
                    ModuleOption {
                        name: "OUTPUT_PATH".to_string(),
                        description: "Output path".to_string(),
                        required: false,
                        default_value: Some("./loot/creds/".to_string()),
                        current_value: self.get_option("OUTPUT_PATH"),
                    },
                ]
            }

            fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
                if self.options.contains_key(name) {
                    self.options.insert(name.to_string(), value.to_string());
                    Ok(())
                } else {
                    Err(anyhow!("Unknown option: {}", name))
                }
            }

            fn get_option(&self, name: &str) -> Option<String> {
                self.options.get(name).cloned()
            }

            fn validate(&self) -> Result<()> {
                Ok(())
            }

            async fn check(&self) -> Result<CheckResult> {
                Ok(CheckResult {
                    vulnerable: false,
                    confidence: 1.0,
                    details: format!("[{}] Ready", $category),
                    fingerprint: HashMap::new(),
                })
            }

            async fn run(&mut self) -> Result<ModuleResult> {
                let output = self.get_option("OUTPUT_PATH").unwrap();

                let mut result = ModuleResult::success(format!(
                    "[SKELETON] {} completed",
                    $name
                ));

                result = result
                    .with_data("module", json!($name))
                    .with_data("mitre", json!($mitre))
                    .with_data("output_path", json!(output))
                    .with_data("credentials", json!([]));

                Ok(result)
            }

            fn requires_confirmation(&self) -> bool {
                true
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

// Credential Harvesting Modules
define_creds_module!(
    BrowserCredsHarvest,
    "browser",
    "creds/harvest",
    "Harvest credentials from browsers",
    "T1555.003"
);

define_creds_module!(
    WifiCredsHarvest,
    "wifi",
    "creds/harvest",
    "Harvest saved WiFi passwords",
    "T1552.001"
);

define_creds_module!(
    SshKeysHarvest,
    "ssh_keys",
    "creds/harvest",
    "Harvest SSH private keys",
    "T1552.004"
);

define_creds_module!(
    CloudTokensHarvest,
    "cloud_tokens",
    "creds/harvest",
    "Harvest cloud service tokens (AWS, Azure, GCP)",
    "T1552.001"
);

// Credential Dumping Modules
define_creds_module!(
    SamDump,
    "sam",
    "creds/dump",
    "Dump SAM database hashes",
    "T1003.002"
);

define_creds_module!(
    LsassDump,
    "lsass",
    "creds/dump",
    "Dump LSASS process memory",
    "T1003.001"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_creds_creation() {
        let module = BrowserCredsHarvest::new();
        assert_eq!(module.info().name, "browser");
        assert!(module.info().description.contains("T1555"));
    }

    #[test]
    fn test_sam_dump_creation() {
        let module = SamDump::new();
        assert_eq!(module.info().category, "creds/dump");
    }
}
