//! Persistence Modules
//!
//! SECURITY NOTICE: For authorized security testing only.

use crate::core::module::*;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;

/// Macro for persistence modules
macro_rules! define_persist_module {
    ($struct_name:ident, $name:expr, $category:expr, $description:expr, $mitre:expr) => {
        pub struct $struct_name {
            options: HashMap<String, String>,
        }

        impl $struct_name {
            pub fn new() -> Self {
                let mut options = HashMap::new();
                options.insert("SESSION".to_string(), String::new());
                options.insert("PAYLOAD".to_string(), String::new());
                options.insert("NAME".to_string(), "WindowsUpdate".to_string());
                options.insert("STARTUP_TYPE".to_string(), "auto".to_string());
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
                        name: "PAYLOAD".to_string(),
                        description: "Payload or command to persist".to_string(),
                        required: true,
                        default_value: None,
                        current_value: self.get_option("PAYLOAD"),
                    },
                    ModuleOption {
                        name: "NAME".to_string(),
                        description: "Name for the persistence entry".to_string(),
                        required: false,
                        default_value: Some("WindowsUpdate".to_string()),
                        current_value: self.get_option("NAME"),
                    },
                    ModuleOption {
                        name: "STARTUP_TYPE".to_string(),
                        description: "Startup type (auto, manual, disabled)".to_string(),
                        required: false,
                        default_value: Some("auto".to_string()),
                        current_value: self.get_option("STARTUP_TYPE"),
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
                if self.get_option("PAYLOAD").unwrap_or_default().is_empty() {
                    return Err(anyhow!("PAYLOAD is required"));
                }
                Ok(())
            }

            async fn check(&self) -> Result<CheckResult> {
                Ok(CheckResult {
                    vulnerable: false,
                    confidence: 1.0,
                    details: format!("[{}] Ready for persistence installation", $category),
                    fingerprint: HashMap::new(),
                })
            }

            async fn run(&mut self) -> Result<ModuleResult> {
                self.validate()?;

                let name = self.get_option("NAME").unwrap();
                let startup = self.get_option("STARTUP_TYPE").unwrap();

                let mut result = ModuleResult::success(format!(
                    "[SKELETON] {} persistence simulation completed",
                    $name
                ));

                result = result
                    .with_data("name", json!(name))
                    .with_data("startup_type", json!(startup))
                    .with_data("mitre", json!($mitre))
                    .with_data("status", json!("skeleton_only"));

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

// Windows Persistence
define_persist_module!(
    PersistRegistry,
    "registry",
    "persist/windows",
    "Registry Run key persistence",
    "T1547.001"
);

define_persist_module!(
    PersistScheduledTask,
    "scheduled_task",
    "persist/windows",
    "Scheduled task persistence",
    "T1053.005"
);

define_persist_module!(
    PersistService,
    "service",
    "persist/windows",
    "Windows service persistence",
    "T1543.003"
);

// Linux Persistence
define_persist_module!(
    PersistCron,
    "cron",
    "persist/linux",
    "Cron job persistence",
    "T1053.003"
);

define_persist_module!(
    PersistSystemd,
    "systemd",
    "persist/linux",
    "Systemd service persistence",
    "T1543.002"
);

// Multi-platform
define_persist_module!(
    PersistSshKey,
    "ssh_key",
    "persist/multi",
    "SSH authorized_keys persistence",
    "T1098.004"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let module = PersistRegistry::new();
        assert_eq!(module.info().name, "registry");
        assert!(module.info().description.contains("T1547"));
    }

    #[test]
    fn test_cron_creation() {
        let module = PersistCron::new();
        assert_eq!(module.info().category, "persist/linux");
    }
}
