//! EDR Detection Engine
//!
//! Comprehensive detection of security products on target systems.
//! MITRE ATT&CK: T1518.001 (Security Software Discovery)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use super::edr_signatures::{get_all_signatures, DetectionMethod, EdrSignature, EdrType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Detected EDR with confidence and evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedEdr {
    /// Type of EDR detected
    pub edr_type: EdrType,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Threat level (1-10)
    pub threat_level: u8,
    /// Detection methods that found this EDR
    pub detection_methods: Vec<DetectionMethod>,
    /// Evidence strings (what was found)
    pub evidence: Vec<String>,
}

/// EDR Detection Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdrDetectionResult {
    /// List of detected EDRs
    pub detected_edrs: Vec<DetectedEdr>,
    /// Maximum threat level found
    pub total_threat_level: u8,
    /// Time taken to scan in milliseconds
    pub scan_time_ms: u64,
    /// Recommended stealth level
    pub recommended_stealth: String,
    /// Summary message
    pub summary: String,
}

impl EdrDetectionResult {
    /// Check if any EDR was detected
    pub fn has_detections(&self) -> bool {
        !self.detected_edrs.is_empty()
    }

    /// Get highest threat EDR
    pub fn highest_threat(&self) -> Option<&DetectedEdr> {
        self.detected_edrs.iter().max_by_key(|e| e.threat_level)
    }

    /// Check if enterprise EDR is present
    pub fn has_enterprise_edr(&self) -> bool {
        self.detected_edrs
            .iter()
            .any(|e| e.edr_type.is_enterprise_edr())
    }
}

/// Scan depth configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScanDepth {
    /// Quick scan - processes only (fastest, least noisy)
    Quick,
    /// Standard scan - processes, services, drivers
    #[default]
    Standard,
    /// Deep scan - everything including registry
    Deep,
    /// Full scan - all methods including hook detection
    Full,
}

impl ScanDepth {
    /// Get methods for this scan depth
    pub fn methods(&self) -> Vec<DetectionMethod> {
        match self {
            Self::Quick => vec![DetectionMethod::ProcessScan],
            Self::Standard => vec![
                DetectionMethod::ProcessScan,
                DetectionMethod::ServiceScan,
                DetectionMethod::DriverScan,
            ],
            Self::Deep => vec![
                DetectionMethod::ProcessScan,
                DetectionMethod::ServiceScan,
                DetectionMethod::DriverScan,
                DetectionMethod::FileScan,
                DetectionMethod::RegistryScan,
                DetectionMethod::DllScan,
            ],
            Self::Full => vec![
                DetectionMethod::ProcessScan,
                DetectionMethod::ServiceScan,
                DetectionMethod::DriverScan,
                DetectionMethod::FileScan,
                DetectionMethod::RegistryScan,
                DetectionMethod::DllScan,
                DetectionMethod::PipeScan,
            ],
        }
    }
}

/// EDR Detector Engine
#[derive(Debug, Clone)]
pub struct EdrDetector {
    signatures: Vec<EdrSignature>,
    scan_depth: ScanDepth,
    max_noise_level: u8,
    safe_mode: bool,
}

impl Default for EdrDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl EdrDetector {
    /// Create new EDR Detector
    pub fn new() -> Self {
        Self {
            signatures: get_all_signatures(),
            scan_depth: ScanDepth::Standard,
            max_noise_level: 6,
            safe_mode: false,
        }
    }

    /// Set scan depth
    pub fn with_depth(mut self, depth: ScanDepth) -> Self {
        self.scan_depth = depth;
        self
    }

    /// Set maximum noise level for detection methods
    pub fn with_max_noise(mut self, level: u8) -> Self {
        self.max_noise_level = level;
        self
    }

    /// Enable safe mode (returns mock results)
    pub fn with_safe_mode(mut self, enabled: bool) -> Self {
        self.safe_mode = enabled;
        self
    }

    /// Quick scan - processes only (fastest)
    pub fn quick_scan(&self) -> EdrDetectionResult {
        self.scan_with_methods(&[DetectionMethod::ProcessScan])
    }

    /// Standard scan - processes, services, drivers
    pub fn standard_scan(&self) -> EdrDetectionResult {
        self.scan_with_methods(&[
            DetectionMethod::ProcessScan,
            DetectionMethod::ServiceScan,
            DetectionMethod::DriverScan,
        ])
    }

    /// Deep scan - all file-based methods
    pub fn deep_scan(&self) -> EdrDetectionResult {
        self.scan_with_methods(&[
            DetectionMethod::ProcessScan,
            DetectionMethod::ServiceScan,
            DetectionMethod::DriverScan,
            DetectionMethod::FileScan,
            DetectionMethod::RegistryScan,
            DetectionMethod::DllScan,
        ])
    }

    /// Full scan with configured depth
    pub fn full_scan(&self) -> EdrDetectionResult {
        let start = std::time::Instant::now();

        // Get methods based on scan depth
        let methods = self.scan_depth.methods();

        // Filter by noise level
        let filtered_methods: Vec<DetectionMethod> = methods
            .into_iter()
            .filter(|m| m.noise_level() <= self.max_noise_level)
            .collect();

        let mut result = self.scan_with_methods(&filtered_methods);
        result.scan_time_ms = start.elapsed().as_millis() as u64;
        result
    }

    /// Scan with specific methods
    fn scan_with_methods(&self, methods: &[DetectionMethod]) -> EdrDetectionResult {
        // Safe mode returns demo data
        if self.safe_mode {
            return self.safe_mode_result();
        }

        let start = std::time::Instant::now();
        let mut detected: HashMap<EdrType, DetectedEdr> = HashMap::new();

        for signature in &self.signatures {
            let mut evidence = Vec::new();
            let mut detection_methods_used = Vec::new();

            for method in methods {
                let found = match method {
                    DetectionMethod::ProcessScan => self.scan_processes(&signature.processes),
                    DetectionMethod::ServiceScan => self.scan_services(&signature.services),
                    DetectionMethod::DriverScan => self.scan_drivers(&signature.drivers),
                    DetectionMethod::FileScan => self.scan_files(&signature.files),
                    DetectionMethod::RegistryScan => self.scan_registry(&signature.registry_keys),
                    DetectionMethod::DllScan => self.scan_dlls(&signature.dlls),
                    DetectionMethod::PipeScan => self.scan_pipes(&signature.pipes),
                    _ => vec![],
                };

                if !found.is_empty() {
                    evidence.extend(found);
                    detection_methods_used.push(*method);
                }
            }

            if !evidence.is_empty() {
                let confidence = self.calculate_confidence(&detection_methods_used, evidence.len());

                detected.insert(
                    signature.edr_type,
                    DetectedEdr {
                        edr_type: signature.edr_type,
                        confidence,
                        threat_level: signature.edr_type.threat_level(),
                        detection_methods: detection_methods_used,
                        evidence,
                    },
                );
            }
        }

        let detected_edrs: Vec<DetectedEdr> = detected.into_values().collect();
        let total_threat = detected_edrs
            .iter()
            .map(|e| e.threat_level)
            .max()
            .unwrap_or(0);

        let recommended_stealth = match total_threat {
            0 => "Normal",
            1..=4 => "Quiet",
            5..=7 => "Silent",
            _ => "Ghost",
        }
        .to_string();

        let summary = if detected_edrs.is_empty() {
            "No security products detected".to_string()
        } else {
            let names: Vec<String> = detected_edrs
                .iter()
                .map(|e| e.edr_type.product_name().to_string())
                .collect();
            format!("Detected: {}", names.join(", "))
        };

        EdrDetectionResult {
            detected_edrs,
            total_threat_level: total_threat,
            scan_time_ms: start.elapsed().as_millis() as u64,
            recommended_stealth,
            summary,
        }
    }

    /// Generate safe mode result (demo data)
    fn safe_mode_result(&self) -> EdrDetectionResult {
        EdrDetectionResult {
            detected_edrs: vec![DetectedEdr {
                edr_type: EdrType::WindowsDefender,
                confidence: 0.95,
                threat_level: 6,
                detection_methods: vec![DetectionMethod::ProcessScan],
                evidence: vec!["[SAFE MODE] MsMpEng.exe".to_string()],
            }],
            total_threat_level: 6,
            scan_time_ms: 1,
            recommended_stealth: "Silent".to_string(),
            summary: "[SAFE MODE] Demo detection: Windows Defender".to_string(),
        }
    }

    // ========== Platform-specific scanning ==========

    #[cfg(windows)]
    fn scan_processes(&self, targets: &[&str]) -> Vec<String> {
        use std::process::Command;

        let mut found = Vec::new();

        if let Ok(output) = Command::new("tasklist").output() {
            let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();

            for target in targets {
                if output_str.contains(&target.to_lowercase()) {
                    found.push(format!("Process: {}", target));
                }
            }
        }

        found
    }

    #[cfg(not(windows))]
    fn scan_processes(&self, targets: &[&str]) -> Vec<String> {
        use std::process::Command;

        let mut found = Vec::new();

        // Use ps on Unix systems
        if let Ok(output) = Command::new("ps").args(["aux"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();

            for target in targets {
                let target_name = target.trim_end_matches(".exe").to_lowercase();
                if output_str.contains(&target_name) {
                    found.push(format!("Process: {}", target));
                }
            }
        }

        found
    }

    #[cfg(windows)]
    fn scan_services(&self, targets: &[&str]) -> Vec<String> {
        use std::process::Command;

        let mut found = Vec::new();

        if let Ok(output) = Command::new("sc")
            .args(["query", "type=", "service"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();

            for target in targets {
                if output_str.contains(&target.to_lowercase()) {
                    found.push(format!("Service: {}", target));
                }
            }
        }

        found
    }

    #[cfg(not(windows))]
    fn scan_services(&self, targets: &[&str]) -> Vec<String> {
        let mut found = Vec::new();

        // Check systemd services on Linux
        for target in targets {
            let target_lower = target.to_lowercase();
            let service_path = format!("/etc/systemd/system/{}.service", target_lower);
            if std::path::Path::new(&service_path).exists() {
                found.push(format!("Service: {}", target));
            }
        }

        found
    }

    #[cfg(windows)]
    fn scan_drivers(&self, targets: &[&str]) -> Vec<String> {
        let mut found = Vec::new();
        let drivers_path = std::path::Path::new(r"C:\Windows\System32\drivers");

        for target in targets {
            if drivers_path.join(target).exists() {
                found.push(format!("Driver: {}", target));
            }
        }

        found
    }

    #[cfg(not(windows))]
    fn scan_drivers(&self, _targets: &[&str]) -> Vec<String> {
        Vec::new() // Drivers are Windows-specific
    }

    fn scan_files(&self, targets: &[&str]) -> Vec<String> {
        let mut found = Vec::new();

        for target in targets {
            if std::path::Path::new(target).exists() {
                found.push(format!("Path: {}", target));
            }
        }

        found
    }

    #[cfg(windows)]
    fn scan_registry(&self, targets: &[&str]) -> Vec<String> {
        use std::process::Command;

        let mut found = Vec::new();

        for target in targets {
            let result = Command::new("reg").args(["query", target]).output();

            if let Ok(output) = result {
                if output.status.success() {
                    found.push(format!("Registry: {}", target));
                }
            }
        }

        found
    }

    #[cfg(not(windows))]
    fn scan_registry(&self, _targets: &[&str]) -> Vec<String> {
        Vec::new() // Registry is Windows-specific
    }

    #[cfg(windows)]
    fn scan_dlls(&self, targets: &[&str]) -> Vec<String> {
        let mut found = Vec::new();
        let system32 = std::path::Path::new(r"C:\Windows\System32");
        for target in targets {
            if system32.join(target).exists() {
                found.push(format!("DLL: {}", target));
            }
        }
        found
    }

    #[cfg(not(windows))]
    fn scan_dlls(&self, _targets: &[&str]) -> Vec<String> {
        Vec::new() // DLLs are Windows-specific
    }

    #[cfg(windows)]
    fn scan_pipes(&self, targets: &[&str]) -> Vec<String> {
        let mut found = Vec::new();
        for target in targets {
            if std::path::Path::new(target).exists() {
                found.push(format!("Pipe: {}", target));
            }
        }
        found
    }

    #[cfg(not(windows))]
    fn scan_pipes(&self, _targets: &[&str]) -> Vec<String> {
        Vec::new() // Named pipes are Windows-specific
    }

    /// Calculate confidence based on detection methods and evidence count
    fn calculate_confidence(&self, methods: &[DetectionMethod], evidence_count: usize) -> f32 {
        let method_weight: f32 = methods.len() as f32 * 0.15;
        let evidence_weight: f32 = (evidence_count as f32 * 0.1).min(0.5);

        (method_weight + evidence_weight).min(1.0)
    }

    /// Get list of all supported EDR types
    pub fn supported_edrs(&self) -> Vec<EdrType> {
        self.signatures.iter().map(|s| s.edr_type).collect()
    }

    /// Get signature count
    pub fn signature_count(&self) -> usize {
        self.signatures.len()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edr_detector_creation() {
        let detector = EdrDetector::new();
        assert!(!detector.signatures.is_empty());
        assert!(detector.signature_count() >= 15);
    }

    #[test]
    fn test_scan_depth_options() {
        let detector = EdrDetector::new()
            .with_depth(ScanDepth::Quick)
            .with_max_noise(5);

        assert_eq!(detector.scan_depth, ScanDepth::Quick);
        assert_eq!(detector.max_noise_level, 5);
    }

    #[test]
    fn test_scan_depth_methods() {
        assert_eq!(ScanDepth::Quick.methods().len(), 1);
        assert_eq!(ScanDepth::Standard.methods().len(), 3);
        assert!(ScanDepth::Deep.methods().len() > 3);
        assert!(ScanDepth::Full.methods().len() >= ScanDepth::Deep.methods().len());
    }

    #[test]
    fn test_safe_mode() {
        let detector = EdrDetector::new().with_safe_mode(true);
        let result = detector.full_scan();

        assert!(result.has_detections());
        assert!(result.summary.contains("SAFE MODE"));
    }

    #[test]
    fn test_quick_scan() {
        let detector = EdrDetector::new();
        let result = detector.quick_scan();

        // Should complete without error
        assert!(result.total_threat_level <= 10);
    }

    #[test]
    fn test_supported_edrs() {
        let detector = EdrDetector::new();
        let supported = detector.supported_edrs();

        assert!(!supported.is_empty());
        assert!(supported.contains(&EdrType::WindowsDefender));
        assert!(supported.contains(&EdrType::CrowdStrike));
    }

    #[test]
    fn test_detection_result_methods() {
        let result = EdrDetectionResult {
            detected_edrs: vec![DetectedEdr {
                edr_type: EdrType::CrowdStrike,
                confidence: 0.9,
                threat_level: 10,
                detection_methods: vec![DetectionMethod::ProcessScan],
                evidence: vec!["Test".to_string()],
            }],
            total_threat_level: 10,
            scan_time_ms: 100,
            recommended_stealth: "Ghost".to_string(),
            summary: "Test".to_string(),
        };

        assert!(result.has_detections());
        assert!(result.has_enterprise_edr());
        assert_eq!(result.highest_threat().unwrap().edr_type, EdrType::CrowdStrike);
    }
}
