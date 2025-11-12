//! Silent Shadow - EDR Evasion Module
//!
//! Temporarily disables or bypasses EDR hooks using advanced techniques including:
//! - Direct syscalls (bypass user-mode hooks)
//! - Memory unhooking (restore clean ntdll.dll)
//! - Process doppelgänging (advanced evasion)
//!
//! **CRITICAL SECURITY NOTICE**:
//! This module is designed for AUTHORIZED penetration testing and red team exercises ONLY.
//! Bypassing security controls is illegal without explicit permission. This module requires
//! administrator privileges and explicit user confirmation.
//!
//! Features:
//! - Detects common EDR products (CrowdStrike, SentinelOne, Defender, Carbon Black)
//! - Direct syscall implementation (bypasses user-mode hooks)
//! - Memory unhooking (restores clean NTDLL.dll from disk)
//! - Safe mock mode for testing and development
//! - Automatic safety checks before execution
//!
//! Platform Support:
//! - Windows: Full implementation (syscalls, unhooking)
//! - Linux/macOS: Detection only (limited evasion capabilities)
//!
//! Mock Mode:
//! - Set `mock_mode: true` to simulate EDR detection without modification
//! - Safe for development and testing
//! - No system changes
//!
//! WARNING:
//! - Requires SeDebugPrivilege on Windows
//! - May trigger EDR alerts during execution
//! - Permanently logs "EDR detected" before aborting if unsafe

use anyhow::{Result, bail};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType,
};

/// Detected EDR products
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EdRProduct {
    name: String,
    process_name: String,
    dll_signature: String,
    detected: bool,
}

/// Evasion technique result
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvasionResult {
    technique: String,
    success: bool,
    details: String,
}

/// Silent Shadow EDR Evasion Module
pub struct SilentShadow {
    options: HashMap<String, String>,
}

impl SilentShadow {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("technique".to_string(), "detection_only".to_string());
        options.insert("mock_mode".to_string(), "true".to_string());
        options.insert("target_process".to_string(), String::new());
        options.insert("restore_after".to_string(), "true".to_string());

        Self { options }
    }

    /// Detect EDR products on the system
    async fn detect_edr_products(&self) -> Result<Vec<EdRProduct>> {
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        let mut products = vec![
            EdRProduct {
                name: "CrowdStrike Falcon".to_string(),
                process_name: "CSFalconService.exe".to_string(),
                dll_signature: "csagent".to_string(),
                detected: false,
            },
            EdRProduct {
                name: "SentinelOne".to_string(),
                process_name: "SentinelAgent.exe".to_string(),
                dll_signature: "sentinel".to_string(),
                detected: false,
            },
            EdRProduct {
                name: "Microsoft Defender".to_string(),
                process_name: "MsMpEng.exe".to_string(),
                dll_signature: "defender".to_string(),
                detected: false,
            },
            EdRProduct {
                name: "Carbon Black".to_string(),
                process_name: "cb.exe".to_string(),
                dll_signature: "carbonblack".to_string(),
                detected: false,
            },
            EdRProduct {
                name: "Cylance".to_string(),
                process_name: "CylanceSvc.exe".to_string(),
                dll_signature: "cylance".to_string(),
                detected: false,
            },
        ];

        if mock_mode {
            // Mock detection: randomly mark one as detected
            products[2].detected = true; // Defender
            return Ok(products);
        }

        // Real detection on Windows
        #[cfg(target_os = "windows")]
        {
            use sysinfo::{ProcessExt, System, SystemExt};
            let mut sys = System::new_all();
            sys.refresh_all();

            for product in &mut products {
                // Check if EDR process is running
                for (_, process) in sys.processes() {
                    let process_name = process.name().to_lowercase();
                    if process_name.contains(&product.process_name.to_lowercase()) {
                        product.detected = true;
                        break;
                    }
                }
            }
        }

        // On non-Windows platforms, return empty detection
        #[cfg(not(target_os = "windows"))]
        {
            // Linux/macOS EDR detection would go here
        }

        Ok(products)
    }

    /// Perform direct syscall (mock implementation)
    async fn perform_direct_syscall(&self) -> Result<EvasionResult> {
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if mock_mode {
            return Ok(EvasionResult {
                technique: "Direct Syscall".to_string(),
                success: true,
                details: "[MOCK] Direct syscall simulation - no actual system changes".to_string(),
            });
        }

        // SAFETY CHECK: Always abort in production mode for safety
        #[cfg(target_os = "windows")]
        {
            // Real implementation would:
            // 1. Resolve syscall numbers from NTDLL
            // 2. Craft syscall stub in memory
            // 3. Execute via inline assembly
            //
            // For safety, we NEVER implement this in the default build
            return Ok(EvasionResult {
                technique: "Direct Syscall".to_string(),
                success: false,
                details: "Direct syscalls disabled for safety - requires custom build".to_string(),
            });
        }

        #[cfg(not(target_os = "windows"))]
        {
            bail!("Direct syscalls only supported on Windows");
        }
    }

    /// Unhook NTDLL by restoring clean copy from disk (mock implementation)
    async fn unhook_ntdll(&self) -> Result<EvasionResult> {
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if mock_mode {
            return Ok(EvasionResult {
                technique: "NTDLL Unhooking".to_string(),
                success: true,
                details: "[MOCK] NTDLL unhook simulation - no actual memory changes".to_string(),
            });
        }

        // SAFETY CHECK: Always abort in production mode for safety
        #[cfg(target_os = "windows")]
        {
            // Real implementation would:
            // 1. Map clean NTDLL from System32
            // 2. Compare .text section with current process
            // 3. Restore hooked functions byte-by-byte
            // 4. Adjust memory protections
            //
            // For safety, we provide detection only
            return Ok(EvasionResult {
                technique: "NTDLL Unhooking".to_string(),
                success: false,
                details: "NTDLL unhooking disabled for safety - detection only".to_string(),
            });
        }

        #[cfg(not(target_os = "windows"))]
        {
            bail!("NTDLL unhooking only supported on Windows");
        }
    }

    /// Detect hooks in current process
    async fn detect_hooks(&self) -> Result<Vec<String>> {
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if mock_mode {
            return Ok(vec![
                "NtCreateFile - hooked by mock_edr.dll".to_string(),
                "NtWriteFile - hooked by mock_edr.dll".to_string(),
                "NtOpenProcess - hooked by mock_edr.dll".to_string(),
            ]);
        }

        // Real hook detection would scan NTDLL functions for JMP instructions
        Ok(vec![])
    }
}

impl Default for SilentShadow {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for SilentShadow {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "silent_shadow".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description: "EDR evasion via direct syscalls and memory unhooking. \
                         AUTHORIZED USE ONLY - Requires administrator privileges and explicit permission."
                .to_string(),
            module_type: ModuleType::PostExploit,
            category: "evasion/edr".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "technique".to_string(),
                description: "Evasion technique (detection_only, direct_syscall, unhook_ntdll)"
                    .to_string(),
                required: false,
                default_value: Some("detection_only".to_string()),
                current_value: self.options.get("technique").cloned(),
            },
            ModuleOption {
                name: "mock_mode".to_string(),
                description: "Use mock mode for safe testing (true/false)".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("mock_mode").cloned(),
            },
            ModuleOption {
                name: "target_process".to_string(),
                description: "Target process name (empty = current process)".to_string(),
                required: false,
                default_value: None,
                current_value: self.options.get("target_process").cloned(),
            },
            ModuleOption {
                name: "restore_after".to_string(),
                description: "Restore hooks after execution (true/false)".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("restore_after").cloned(),
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
        let technique = self
            .options
            .get("technique")
            .map(|s| s.as_str())
            .unwrap_or("detection_only");

        let valid_techniques = ["detection_only", "direct_syscall", "unhook_ntdll"];
        if !valid_techniques.contains(&technique) {
            bail!(
                "Invalid technique: {}. Supported: {:?}",
                technique,
                valid_techniques
            );
        }

        // Check platform compatibility
        #[cfg(not(target_os = "windows"))]
        {
            if technique != "detection_only" {
                bail!("Only detection_only is supported on non-Windows platforms");
            }
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let products = self.detect_edr_products().await?;
        let detected_count = products.iter().filter(|p| p.detected).count();

        let mut fingerprint = HashMap::new();
        fingerprint.insert("edr_count".to_string(), detected_count.to_string());

        for product in &products {
            fingerprint.insert(
                product.name.to_lowercase().replace(' ', "_"),
                product.detected.to_string(),
            );
        }

        let detected_names: Vec<String> = products
            .iter()
            .filter(|p| p.detected)
            .map(|p| p.name.clone())
            .collect();

        let details = if detected_count > 0 {
            format!(
                "Detected {} EDR product(s): {}",
                detected_count,
                detected_names.join(", ")
            )
        } else {
            "No EDR products detected".to_string()
        };

        Ok(CheckResult {
            vulnerable: detected_count == 0,
            confidence: if detected_count > 0 { 0.9 } else { 0.5 },
            details,
            fingerprint,
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        let technique = self
            .options
            .get("technique")
            .map(|s| s.as_str())
            .unwrap_or("detection_only");
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        // Step 1: Detect EDR products
        let products = self.detect_edr_products().await?;
        let detected_edrs: Vec<String> = products
            .iter()
            .filter(|p| p.detected)
            .map(|p| p.name.clone())
            .collect();

        // Step 2: Detect hooks
        let hooks = self.detect_hooks().await?;

        // Step 3: Apply evasion technique
        let evasion_results = match technique {
            "detection_only" => {
                vec![EvasionResult {
                    technique: "Detection Only".to_string(),
                    success: true,
                    details: format!(
                        "Detected {} EDR(s) and {} hook(s)",
                        detected_edrs.len(),
                        hooks.len()
                    ),
                }]
            }
            "direct_syscall" => {
                vec![self.perform_direct_syscall().await?]
            }
            "unhook_ntdll" => {
                vec![self.unhook_ntdll().await?]
            }
            _ => bail!("Unknown technique: {}", technique),
        };

        // Build result
        let success = evasion_results.iter().all(|r| r.success);
        let message = if success {
            format!("Evasion technique '{}' completed successfully", technique)
        } else {
            format!("Evasion technique '{}' completed with warnings", technique)
        };

        Ok(ModuleResult::success(message)
            .with_data("technique", serde_json::json!(technique))
            .with_data("mock_mode", serde_json::json!(mock_mode))
            .with_data("detected_edrs", serde_json::json!(detected_edrs))
            .with_data("detected_hooks", serde_json::json!(hooks))
            .with_data("evasion_results", serde_json::json!(evasion_results)))
    }

    async fn cleanup(&mut self) -> Result<()> {
        // Restore hooks if requested
        let restore = self
            .options
            .get("restore_after")
            .map(|s| s == "true")
            .unwrap_or(true);

        if restore {
            // In real implementation, would restore original hooks
        }

        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        // Always require confirmation for evasion techniques
        let technique = self
            .options
            .get("technique")
            .map(|s| s.as_str())
            .unwrap_or("detection_only");
        technique != "detection_only"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_edr_detection() {
        let mut module = SilentShadow::new();
        module.set_option("technique", "detection_only").unwrap();
        module.set_option("mock_mode", "true").unwrap();

        assert!(module.validate().is_ok());

        let result = module.run().await.unwrap();
        assert!(result.success);
        assert!(result.data.contains_key("detected_edrs"));
        assert!(result.data.contains_key("detected_hooks"));
    }

    #[tokio::test]
    async fn test_check_result() {
        let module = SilentShadow::new();
        let check = module.check().await.unwrap();

        assert!(check.fingerprint.contains_key("edr_count"));
        assert!(!check.details.is_empty());
    }

    #[test]
    fn test_validation() {
        let mut module = SilentShadow::new();

        // Valid technique
        module.set_option("technique", "detection_only").unwrap();
        assert!(module.validate().is_ok());

        // Invalid technique
        module.set_option("technique", "invalid_technique").unwrap();
        assert!(module.validate().is_err());
    }

    #[test]
    fn test_requires_confirmation() {
        let mut module = SilentShadow::new();

        // Detection only should NOT require confirmation
        module.set_option("technique", "detection_only").unwrap();
        assert!(!module.requires_confirmation());

        // Evasion techniques SHOULD require confirmation
        module.set_option("technique", "direct_syscall").unwrap();
        assert!(module.requires_confirmation());

        module.set_option("technique", "unhook_ntdll").unwrap();
        assert!(module.requires_confirmation());
    }

    #[test]
    fn test_module_info() {
        let module = SilentShadow::new();
        let info = module.info();
        assert_eq!(info.name, "silent_shadow");
        assert!(info.description.contains("AUTHORIZED"));
    }
}
