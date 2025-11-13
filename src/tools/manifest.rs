/// Module manifest and registry management
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCategory {
    pub description: String,
    pub modules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleManifest {
    pub version: String,
    pub last_updated: String,
    pub categories: HashMap<String, ModuleCategory>,
}

impl ModuleManifest {
    /// Load manifest from default location or create default
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_path = "src/modules/manifest.json";

        if Path::new(manifest_path).exists() {
            let content = std::fs::read_to_string(manifest_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    /// Get all modules across all categories
    pub fn all_modules(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        for (category, data) in &self.categories {
            for module in &data.modules {
                result.push((category.clone(), module.clone()));
            }
        }
        result
    }

    /// Check if module file exists
    pub fn module_exists(&self, category: &str, module: &str) -> bool {
        let path = format!("src/{}/{}.rs", category, module);
        Path::new(&path).exists()
    }

    /// Get all missing module files
    pub fn missing_modules(&self) -> Vec<(String, String)> {
        self.all_modules()
            .into_iter()
            .filter(|(cat, mod_name)| !self.module_exists(cat, mod_name))
            .collect()
    }

    /// Save manifest to file
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

impl Default for ModuleManifest {
    fn default() -> Self {
        let mut categories = HashMap::new();

        categories.insert(
            "memory_forensics".to_string(),
            ModuleCategory {
                description: "Advanced memory analysis capabilities".to_string(),
                modules: vec![
                    "dump_parser".to_string(),
                    "process_analyzer".to_string(),
                    "malware_detector".to_string(),
                    "network_analyzer".to_string(),
                    "registry_analyzer".to_string(),
                    "credential_extractor".to_string(),
                    "mitre_mapper".to_string(),
                    "volatility_bridge".to_string(),
                ],
            },
        );

        categories.insert(
            "scanner".to_string(),
            ModuleCategory {
                description: "Network and service scanning".to_string(),
                modules: vec!["port".to_string(), "http".to_string()],
            },
        );

        categories.insert(
            "recon".to_string(),
            ModuleCategory {
                description: "Reconnaissance and information gathering".to_string(),
                modules: vec![
                    "asn".to_string(),
                    "dns".to_string(),
                    "subdomains".to_string(),
                    "whois".to_string(),
                ],
            },
        );

        categories.insert(
            "c2".to_string(),
            ModuleCategory {
                description: "Command and control infrastructure".to_string(),
                modules: vec![
                    "teams_tunnel".to_string(),
                    "http_beacon".to_string(),
                    "dns_c2".to_string(),
                    "github_c2".to_string(),
                    "command_scheduler".to_string(),
                    "relay_manager".to_string(),
                ],
            },
        );

        categories.insert(
            "evasion".to_string(),
            ModuleCategory {
                description: "Detection evasion techniques".to_string(),
                modules: vec!["edr".to_string(), "browser".to_string()],
            },
        );

        categories.insert(
            "post".to_string(),
            ModuleCategory {
                description: "Post-exploitation capabilities".to_string(),
                modules: vec![
                    "persistence".to_string(),
                    "credential_collector".to_string(),
                ],
            },
        );

        categories.insert(
            "exploit".to_string(),
            ModuleCategory {
                description: "Exploitation modules".to_string(),
                modules: vec!["example".to_string()],
            },
        );

        categories.insert(
            "auxiliary".to_string(),
            ModuleCategory {
                description: "Auxiliary utilities".to_string(),
                modules: vec!["cloud".to_string()],
            },
        );

        Self {
            version: "2.0.0".to_string(),
            last_updated: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            categories,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_manifest() {
        let manifest = ModuleManifest::default();
        assert_eq!(manifest.version, "2.0.0");
        assert!(!manifest.categories.is_empty());
    }

    #[test]
    fn test_all_modules() {
        let manifest = ModuleManifest::default();
        let modules = manifest.all_modules();
        assert!(!modules.is_empty());
    }
}
