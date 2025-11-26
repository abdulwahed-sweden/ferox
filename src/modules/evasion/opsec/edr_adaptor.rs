//! EDR Adaptation Engine
//!
//! Automatically adjusts OPSEC configuration based on detected EDRs.
//! MITRE ATT&CK: T1562 (Impair Defenses)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use super::edr_detector::EdrDetectionResult;
use super::edr_signatures::EdrType;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Stealth level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum AdaptedStealthLevel {
    /// No restrictions - standard operations
    Normal,
    /// Basic stealth - minimal delays and precautions
    #[default]
    Quiet,
    /// High stealth - significant delays and evasion
    Silent,
    /// Maximum stealth - extreme delays and all evasion techniques
    Ghost,
}

impl AdaptedStealthLevel {
    /// Get sleep time in milliseconds
    pub fn sleep_ms(&self) -> u64 {
        match self {
            Self::Normal => 100,
            Self::Quiet => 3_000,
            Self::Silent => 15_000,
            Self::Ghost => 45_000,
        }
    }

    /// Get sleep duration
    pub fn sleep_duration(&self) -> Duration {
        Duration::from_millis(self.sleep_ms())
    }

    /// Get jitter factor (0.0 - 1.0)
    pub fn jitter(&self) -> f64 {
        match self {
            Self::Normal => 0.1,
            Self::Quiet => 0.3,
            Self::Silent => 0.5,
            Self::Ghost => 0.8,
        }
    }

    /// Get maximum network packets per operation
    pub fn max_packets(&self) -> usize {
        match self {
            Self::Normal => 1000,
            Self::Quiet => 100,
            Self::Silent => 20,
            Self::Ghost => 5,
        }
    }

    /// Get as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Quiet => "Quiet",
            Self::Silent => "Silent",
            Self::Ghost => "Ghost",
        }
    }

    /// Create from threat level
    pub fn from_threat_level(level: u8) -> Self {
        match level {
            0..=4 => Self::Quiet,
            5..=7 => Self::Silent,
            _ => Self::Ghost,
        }
    }
}

/// Adapted OPSEC configuration based on detected EDRs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptedOpsecConfig {
    /// Overall stealth level
    pub stealth_level: AdaptedStealthLevel,
    /// Sleep time between operations (ms)
    pub sleep_ms: u64,
    /// Jitter factor (0.0 - 1.0)
    pub jitter: f64,
    /// Use direct syscalls instead of API
    pub use_syscalls: bool,
    /// Avoid touching LSASS
    pub avoid_lsass: bool,
    /// Disable ETW telemetry
    pub disable_etw: bool,
    /// Patch AMSI
    pub patch_amsi: bool,
    /// Use LOLBins for operations
    pub use_lolbins: bool,
    /// Avoid suspicious API calls
    pub avoid_suspicious_apis: bool,
    /// Encrypt in-memory artifacts
    pub encrypt_memory: bool,
    /// Maximum retry attempts
    pub max_attempts: u8,
    /// Maximum noise level for detection methods
    pub allowed_noise_level: u8,
    /// Avoid hooked functions
    pub avoid_hooks: bool,
    /// Use indirect syscalls
    pub use_indirect_syscalls: bool,
    /// Maximum packets per operation
    pub max_packets: usize,
    /// Work hours only mode
    pub work_hours_only: bool,
    /// Reason for this configuration
    pub reason: String,
    /// Detected EDR types
    pub detected_edrs: Vec<EdrType>,
}

impl Default for AdaptedOpsecConfig {
    fn default() -> Self {
        Self {
            stealth_level: AdaptedStealthLevel::Quiet,
            sleep_ms: 3_000,
            jitter: 0.3,
            use_syscalls: false,
            avoid_lsass: false,
            disable_etw: false,
            patch_amsi: true,
            use_lolbins: true,
            avoid_suspicious_apis: false,
            encrypt_memory: false,
            max_attempts: 5,
            allowed_noise_level: 5,
            avoid_hooks: false,
            use_indirect_syscalls: false,
            max_packets: 100,
            work_hours_only: false,
            reason: "Default configuration".to_string(),
            detected_edrs: Vec::new(),
        }
    }
}

impl AdaptedOpsecConfig {
    /// Create configuration for no EDR detected
    pub fn no_edr() -> Self {
        Self {
            stealth_level: AdaptedStealthLevel::Normal,
            sleep_ms: 100,
            jitter: 0.1,
            patch_amsi: false,
            reason: "No EDR detected - minimal stealth required".to_string(),
            ..Default::default()
        }
    }

    /// Create ghost mode configuration
    pub fn ghost() -> Self {
        Self {
            stealth_level: AdaptedStealthLevel::Ghost,
            sleep_ms: 60_000,
            jitter: 0.8,
            use_syscalls: true,
            avoid_lsass: true,
            disable_etw: true,
            patch_amsi: true,
            use_lolbins: true,
            avoid_suspicious_apis: true,
            encrypt_memory: true,
            max_attempts: 1,
            allowed_noise_level: 2,
            avoid_hooks: true,
            use_indirect_syscalls: true,
            max_packets: 3,
            work_hours_only: true,
            reason: "Ghost mode - maximum stealth".to_string(),
            detected_edrs: Vec::new(),
        }
    }

    /// Get sleep duration
    pub fn sleep_duration(&self) -> Duration {
        Duration::from_millis(self.sleep_ms)
    }

    /// Check if any high-threat EDR is present
    pub fn has_high_threat_edr(&self) -> bool {
        self.detected_edrs.iter().any(|e| e.threat_level() >= 8)
    }
}

/// EDR Adaptor Engine
#[derive(Debug, Clone, Default)]
pub struct EdrAdaptor {
    /// Override to always use specific stealth level
    force_stealth: Option<AdaptedStealthLevel>,
}

impl EdrAdaptor {
    /// Create new EDR Adaptor
    pub fn new() -> Self {
        Self { force_stealth: None }
    }

    /// Force specific stealth level regardless of detection
    pub fn force_stealth(mut self, level: AdaptedStealthLevel) -> Self {
        self.force_stealth = Some(level);
        self
    }

    /// Generate adapted configuration based on detection results
    pub fn adapt(&self, detection_result: &EdrDetectionResult) -> AdaptedOpsecConfig {
        // Collect detected EDR types
        let detected_edrs: Vec<EdrType> = detection_result
            .detected_edrs
            .iter()
            .map(|e| e.edr_type)
            .collect();

        // Handle forced stealth level
        if let Some(forced) = self.force_stealth {
            return AdaptedOpsecConfig {
                detected_edrs,
                stealth_level: forced,
                sleep_ms: forced.sleep_ms(),
                jitter: forced.jitter(),
                max_packets: forced.max_packets(),
                reason: format!("Forced stealth level: {}", forced.as_str()),
                ..Default::default()
            };
        }

        // No EDR detected
        if detection_result.detected_edrs.is_empty() {
            return AdaptedOpsecConfig::no_edr();
        }

        // Find the most threatening EDR
        let max_threat = detection_result
            .detected_edrs
            .iter()
            .max_by_key(|e| e.threat_level)
            .unwrap();

        // Set base stealth level from threat
        let stealth_level = AdaptedStealthLevel::from_threat_level(max_threat.threat_level);

        let mut config = AdaptedOpsecConfig {
            detected_edrs,
            stealth_level,
            sleep_ms: stealth_level.sleep_ms(),
            jitter: stealth_level.jitter(),
            max_packets: stealth_level.max_packets(),
            reason: String::new(),
            ..Default::default()
        };

        // Apply EDR-specific adaptations
        for edr in &detection_result.detected_edrs {
            self.apply_edr_specific_config(&mut config, &edr.edr_type);
        }

        config.reason = format!(
            "Adapted for {} EDR(s), highest threat: {} (level {})",
            detection_result.detected_edrs.len(),
            max_threat.edr_type.vendor(),
            max_threat.threat_level
        );

        config
    }

    /// Apply EDR-specific configuration overrides
    fn apply_edr_specific_config(&self, config: &mut AdaptedOpsecConfig, edr: &EdrType) {
        match edr {
            EdrType::CrowdStrike => {
                config.stealth_level = AdaptedStealthLevel::Ghost;
                config.use_syscalls = true;
                config.use_indirect_syscalls = true;
                config.avoid_lsass = true;
                config.disable_etw = true;
                config.patch_amsi = true;
                config.encrypt_memory = true;
                config.avoid_hooks = true;
                config.avoid_suspicious_apis = true;
                config.max_attempts = 1;
                config.allowed_noise_level = 2;
                config.sleep_ms = 60_000;
                config.jitter = 0.8;
                config.max_packets = 3;
                config.work_hours_only = true;
            }

            EdrType::SentinelOne => {
                config.stealth_level = AdaptedStealthLevel::Ghost;
                config.use_syscalls = true;
                config.avoid_lsass = true;
                config.disable_etw = true;
                config.avoid_hooks = true;
                config.avoid_suspicious_apis = true;
                config.max_attempts = 1;
                config.allowed_noise_level = 2;
                config.sleep_ms = 45_000;
                config.jitter = 0.7;
                config.max_packets = 5;
            }

            EdrType::DefenderATP => {
                config.stealth_level = AdaptedStealthLevel::Silent;
                config.disable_etw = true;
                config.patch_amsi = true;
                config.avoid_suspicious_apis = true;
                config.use_lolbins = true;
                config.max_attempts = 2;
                config.allowed_noise_level = 3;
                config.sleep_ms = 30_000;
                config.jitter = 0.5;
            }

            EdrType::WindowsDefender => {
                config.stealth_level = AdaptedStealthLevel::Silent;
                config.patch_amsi = true;
                config.disable_etw = true;
                config.use_lolbins = true;
                config.max_attempts = 3;
                config.allowed_noise_level = 4;
                config.sleep_ms = 15_000;
            }

            EdrType::CarbonBlack => {
                config.stealth_level = AdaptedStealthLevel::Silent;
                config.use_syscalls = true;
                config.disable_etw = true;
                config.avoid_suspicious_apis = true;
                config.max_attempts = 2;
                config.sleep_ms = 30_000;
            }

            EdrType::Cybereason => {
                config.stealth_level = AdaptedStealthLevel::Silent;
                config.use_syscalls = true;
                config.disable_etw = true;
                config.max_attempts = 2;
                config.sleep_ms = 25_000;
            }

            EdrType::Elastic => {
                config.stealth_level = AdaptedStealthLevel::Silent;
                config.disable_etw = true;
                config.avoid_suspicious_apis = true;
                config.max_attempts = 2;
                config.sleep_ms = 20_000;
            }

            EdrType::Cylance => {
                config.stealth_level = AdaptedStealthLevel::Quiet;
                config.encrypt_memory = true;
                config.max_attempts = 3;
                config.sleep_ms = 10_000;
            }

            // Default for other EDRs
            _ => {
                config.patch_amsi = true;
                config.use_lolbins = true;
                config.disable_etw = true;
            }
        }
    }

    /// Generate human-readable recommendations
    pub fn get_recommendations(&self, detection_result: &EdrDetectionResult) -> Vec<String> {
        let mut recommendations = Vec::new();

        if detection_result.detected_edrs.is_empty() {
            recommendations.push("No EDR detected - standard operations allowed".to_string());
            return recommendations;
        }

        recommendations.push(format!(
            "Detected {} security product(s)",
            detection_result.detected_edrs.len()
        ));

        for edr in &detection_result.detected_edrs {
            let rec = match edr.threat_level {
                9..=10 => format!(
                    "[CRITICAL] {} - Use Ghost mode, direct syscalls, maximum delays",
                    edr.edr_type.product_name()
                ),
                7..=8 => format!(
                    "[HIGH] {} - Use Silent mode, avoid LSASS, disable ETW",
                    edr.edr_type.product_name()
                ),
                5..=6 => format!(
                    "[MEDIUM] {} - Use Quiet mode, patch AMSI",
                    edr.edr_type.product_name()
                ),
                _ => format!(
                    "[LOW] {} - Standard precautions sufficient",
                    edr.edr_type.product_name()
                ),
            };
            recommendations.push(rec);
        }

        recommendations.push(format!(
            "Recommended stealth level: {}",
            detection_result.recommended_stealth
        ));

        // Add specific technique recommendations
        if detection_result.total_threat_level >= 8 {
            recommendations.push("Techniques: Direct syscalls, memory encryption, work hours only".to_string());
        } else if detection_result.total_threat_level >= 5 {
            recommendations.push("Techniques: ETW patching, AMSI bypass, LOLBins".to_string());
        }

        recommendations
    }

    /// Generate configuration summary
    pub fn summarize_config(&self, config: &AdaptedOpsecConfig) -> String {
        let mut lines = Vec::new();

        lines.push(format!("Stealth Level: {}", config.stealth_level.as_str()));
        lines.push(format!("Sleep: {}ms (jitter: {:.0}%)", config.sleep_ms, config.jitter * 100.0));
        lines.push(format!("Max Packets: {}", config.max_packets));

        let mut enabled = Vec::new();
        if config.use_syscalls {
            enabled.push("Syscalls");
        }
        if config.disable_etw {
            enabled.push("ETW Patch");
        }
        if config.patch_amsi {
            enabled.push("AMSI Bypass");
        }
        if config.use_lolbins {
            enabled.push("LOLBins");
        }
        if config.encrypt_memory {
            enabled.push("Memory Encryption");
        }
        if config.avoid_hooks {
            enabled.push("Hook Avoidance");
        }

        if !enabled.is_empty() {
            lines.push(format!("Enabled: {}", enabled.join(", ")));
        }

        lines.push(format!("Reason: {}", config.reason));

        lines.join("\n")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::super::edr_detector::DetectedEdr;
    use super::super::edr_signatures::DetectionMethod;
    use super::*;

    #[test]
    fn test_stealth_levels() {
        assert!(AdaptedStealthLevel::Ghost.sleep_ms() > AdaptedStealthLevel::Silent.sleep_ms());
        assert!(AdaptedStealthLevel::Silent.sleep_ms() > AdaptedStealthLevel::Quiet.sleep_ms());
        assert!(AdaptedStealthLevel::Quiet.sleep_ms() > AdaptedStealthLevel::Normal.sleep_ms());
    }

    #[test]
    fn test_stealth_from_threat() {
        assert_eq!(
            AdaptedStealthLevel::from_threat_level(10),
            AdaptedStealthLevel::Ghost
        );
        assert_eq!(
            AdaptedStealthLevel::from_threat_level(6),
            AdaptedStealthLevel::Silent
        );
        assert_eq!(
            AdaptedStealthLevel::from_threat_level(3),
            AdaptedStealthLevel::Quiet
        );
    }

    #[test]
    fn test_default_config() {
        let config = AdaptedOpsecConfig::default();
        assert_eq!(config.stealth_level, AdaptedStealthLevel::Quiet);
        assert!(config.patch_amsi);
        assert!(config.use_lolbins);
    }

    #[test]
    fn test_ghost_config() {
        let config = AdaptedOpsecConfig::ghost();
        assert_eq!(config.stealth_level, AdaptedStealthLevel::Ghost);
        assert!(config.use_syscalls);
        assert!(config.encrypt_memory);
        assert!(config.work_hours_only);
    }

    #[test]
    fn test_adaptor_no_edr() {
        let adaptor = EdrAdaptor::new();
        let result = EdrDetectionResult {
            detected_edrs: vec![],
            total_threat_level: 0,
            scan_time_ms: 10,
            recommended_stealth: "Normal".to_string(),
            summary: "No EDR".to_string(),
        };

        let config = adaptor.adapt(&result);
        assert_eq!(config.stealth_level, AdaptedStealthLevel::Normal);
        assert!(!config.patch_amsi);
    }

    #[test]
    fn test_adaptor_crowdstrike() {
        let adaptor = EdrAdaptor::new();
        let result = EdrDetectionResult {
            detected_edrs: vec![DetectedEdr {
                edr_type: EdrType::CrowdStrike,
                confidence: 0.9,
                threat_level: 10,
                detection_methods: vec![DetectionMethod::ProcessScan],
                evidence: vec!["CSFalconService.exe".to_string()],
            }],
            total_threat_level: 10,
            scan_time_ms: 50,
            recommended_stealth: "Ghost".to_string(),
            summary: "CrowdStrike detected".to_string(),
        };

        let config = adaptor.adapt(&result);
        assert_eq!(config.stealth_level, AdaptedStealthLevel::Ghost);
        assert!(config.use_syscalls);
        assert!(config.use_indirect_syscalls);
        assert!(config.avoid_lsass);
        assert!(config.work_hours_only);
    }

    #[test]
    fn test_forced_stealth() {
        let adaptor = EdrAdaptor::new().force_stealth(AdaptedStealthLevel::Ghost);
        let result = EdrDetectionResult {
            detected_edrs: vec![],
            total_threat_level: 0,
            scan_time_ms: 10,
            recommended_stealth: "Normal".to_string(),
            summary: "No EDR".to_string(),
        };

        let config = adaptor.adapt(&result);
        assert_eq!(config.stealth_level, AdaptedStealthLevel::Ghost);
    }

    #[test]
    fn test_recommendations() {
        let adaptor = EdrAdaptor::new();
        let result = EdrDetectionResult {
            detected_edrs: vec![],
            total_threat_level: 0,
            scan_time_ms: 10,
            recommended_stealth: "Normal".to_string(),
            summary: "No EDR".to_string(),
        };

        let recs = adaptor.get_recommendations(&result);
        assert!(!recs.is_empty());
        assert!(recs[0].contains("No EDR"));
    }

    #[test]
    fn test_config_summary() {
        let adaptor = EdrAdaptor::new();
        let config = AdaptedOpsecConfig::ghost();
        let summary = adaptor.summarize_config(&config);

        assert!(summary.contains("Ghost"));
        assert!(summary.contains("Syscalls"));
    }
}
