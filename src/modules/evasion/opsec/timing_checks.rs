//! Timing-based Detection
//!
//! Detect sandboxes through timing anomalies and behavior checks.
//!
//! MITRE ATT&CK: T1497.003 (Time Based Evasion)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Timing check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingCheckResult {
    /// Name of the check performed
    pub check_name: String,
    /// Whether the result is suspicious
    pub is_suspicious: bool,
    /// Measured value
    pub measured_value: f64,
    /// Expected range (min, max)
    pub expected_range: (f64, f64),
    /// Human-readable message
    pub message: String,
}

/// Timing-based detection engine
#[derive(Debug, Default)]
pub struct TimingChecker {
    results: Vec<TimingCheckResult>,
    safe_mode: bool,
}

impl TimingChecker {
    /// Create new TimingChecker
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable safe mode (simulated checks)
    pub fn with_safe_mode(mut self, enabled: bool) -> Self {
        self.safe_mode = enabled;
        self
    }

    /// Check RDTSC timing (detect accelerated execution)
    pub fn check_rdtsc_timing(&mut self) -> TimingCheckResult {
        if self.safe_mode {
            let result = TimingCheckResult {
                check_name: "RDTSC Timing".to_string(),
                is_suspicious: false,
                measured_value: 5.0,
                expected_range: (0.5, 100.0),
                message: "[SAFE MODE] Timing check simulated".to_string(),
            };
            self.results.push(result.clone());
            return result;
        }

        let iterations = 1000;
        let start = Instant::now();

        // Perform many iterations
        for _ in 0..iterations {
            let _ = std::hint::black_box(1 + 1);
        }

        let elapsed = start.elapsed();
        let ns_per_iter = elapsed.as_nanos() as f64 / iterations as f64;

        // In real hardware, this should be consistent
        // Sandboxes often show irregular timing
        let is_suspicious = !(0.1..=1000.0).contains(&ns_per_iter);

        let result = TimingCheckResult {
            check_name: "RDTSC Timing".to_string(),
            is_suspicious,
            measured_value: ns_per_iter,
            expected_range: (0.5, 100.0),
            message: if is_suspicious {
                "Timing anomaly detected - possible sandbox".to_string()
            } else {
                format!("Timing normal ({:.2} ns/iter)", ns_per_iter)
            },
        };

        self.results.push(result.clone());
        result
    }

    /// Check sleep accuracy (detect accelerated sleep)
    pub fn check_sleep_accuracy(&mut self) -> TimingCheckResult {
        if self.safe_mode {
            let result = TimingCheckResult {
                check_name: "Sleep Accuracy".to_string(),
                is_suspicious: false,
                measured_value: 1.0,
                expected_range: (0.9, 1.5),
                message: "[SAFE MODE] Sleep check simulated".to_string(),
            };
            self.results.push(result.clone());
            return result;
        }

        let sleep_duration = Duration::from_millis(100);
        let start = Instant::now();

        std::thread::sleep(sleep_duration);

        let actual_elapsed = start.elapsed();
        let expected_ms = sleep_duration.as_millis() as f64;
        let actual_ms = actual_elapsed.as_millis() as f64;
        let accuracy = if expected_ms > 0.0 {
            actual_ms / expected_ms
        } else {
            1.0
        };

        // Sleep should be roughly accurate (0.9 - 2.0x)
        // Sandboxes often accelerate sleep
        let is_suspicious = !(0.5..=3.0).contains(&accuracy);

        let result = TimingCheckResult {
            check_name: "Sleep Accuracy".to_string(),
            is_suspicious,
            measured_value: accuracy,
            expected_range: (0.9, 1.5),
            message: if is_suspicious {
                format!(
                    "Sleep accelerated/skipped ({:.2}x) - possible sandbox",
                    accuracy
                )
            } else {
                format!("Sleep accuracy: {:.2}x", accuracy)
            },
        };

        self.results.push(result.clone());
        result
    }

    /// Check system uptime (sandboxes often have short uptime)
    pub fn check_uptime(&mut self) -> TimingCheckResult {
        if self.safe_mode {
            let result = TimingCheckResult {
                check_name: "System Uptime".to_string(),
                is_suspicious: false,
                measured_value: 120.0,
                expected_range: (30.0, f64::MAX),
                message: "[SAFE MODE] Uptime check simulated".to_string(),
            };
            self.results.push(result.clone());
            return result;
        }

        let uptime_ms = Self::get_system_uptime_ms();
        let uptime_minutes = uptime_ms as f64 / 60000.0;

        // Less than 10 minutes uptime is suspicious
        let is_suspicious = uptime_minutes < 10.0;

        let result = TimingCheckResult {
            check_name: "System Uptime".to_string(),
            is_suspicious,
            measured_value: uptime_minutes,
            expected_range: (30.0, f64::MAX),
            message: if is_suspicious {
                format!(
                    "Low uptime ({:.1} minutes) - possible sandbox",
                    uptime_minutes
                )
            } else {
                format!("Uptime: {:.1} minutes", uptime_minutes)
            },
        };

        self.results.push(result.clone());
        result
    }

    /// Get system uptime in milliseconds
    #[cfg(windows)]
    fn get_system_uptime_ms() -> u64 {
        // Use GetTickCount64 on Windows
        unsafe {
            #[link(name = "kernel32")]
            extern "system" {
                fn GetTickCount64() -> u64;
            }
            GetTickCount64()
        }
    }

    #[cfg(not(windows))]
    fn get_system_uptime_ms() -> u64 {
        // Read from /proc/uptime on Linux
        if let Some(uptime_secs) = std::fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|content| content.split_whitespace().next().map(|s| s.to_string()))
            .and_then(|uptime_str| uptime_str.parse::<f64>().ok())
        {
            return (uptime_secs * 1000.0) as u64;
        }
        // Fallback: assume reasonable uptime
        3600000 // 1 hour
    }

    /// Get CPU count
    fn get_cpu_count() -> usize {
        std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1)
    }

    /// Check for recent user activity (mouse, keyboard)
    pub fn check_user_activity(&mut self) -> TimingCheckResult {
        if self.safe_mode {
            let result = TimingCheckResult {
                check_name: "User Activity".to_string(),
                is_suspicious: false,
                measured_value: 1.0,
                expected_range: (1.0, 1.0),
                message: "[SAFE MODE] User activity check simulated".to_string(),
            };
            self.results.push(result.clone());
            return result;
        }

        let has_activity = Self::detect_recent_activity();

        let result = TimingCheckResult {
            check_name: "User Activity".to_string(),
            is_suspicious: !has_activity,
            measured_value: if has_activity { 1.0 } else { 0.0 },
            expected_range: (1.0, 1.0),
            message: if !has_activity {
                "No user activity detected - possible sandbox".to_string()
            } else {
                "User activity detected".to_string()
            },
        };

        self.results.push(result.clone());
        result
    }

    /// Detect recent user activity
    #[cfg(windows)]
    fn detect_recent_activity() -> bool {
        use std::mem::size_of;

        #[repr(C)]
        struct LASTINPUTINFO {
            cb_size: u32,
            dw_time: u32,
        }

        unsafe {
            #[link(name = "user32")]
            extern "system" {
                fn GetLastInputInfo(plii: *mut LASTINPUTINFO) -> i32;
            }

            #[link(name = "kernel32")]
            extern "system" {
                fn GetTickCount() -> u32;
            }

            let mut lii = LASTINPUTINFO {
                cb_size: size_of::<LASTINPUTINFO>() as u32,
                dw_time: 0,
            };

            if GetLastInputInfo(&mut lii) != 0 {
                let current_tick = GetTickCount();
                let idle_time_ms = current_tick.wrapping_sub(lii.dw_time);

                // Less than 5 minutes of idle is considered active
                return idle_time_ms < 300_000;
            }
        }

        true // Assume active if we can't check
    }

    #[cfg(not(windows))]
    fn detect_recent_activity() -> bool {
        // On Linux, check X11 idle time or assume active
        // For simplicity, assume active on non-Windows
        true
    }

    /// Check CPU count (sandboxes often have few CPUs)
    pub fn check_cpu_count(&mut self) -> TimingCheckResult {
        if self.safe_mode {
            let result = TimingCheckResult {
                check_name: "CPU Count".to_string(),
                is_suspicious: false,
                measured_value: 4.0,
                expected_range: (2.0, 128.0),
                message: "[SAFE MODE] CPU count check simulated".to_string(),
            };
            self.results.push(result.clone());
            return result;
        }

        let cpu_count = Self::get_cpu_count() as f64;

        // Single CPU is suspicious for modern systems
        let is_suspicious = cpu_count < 2.0;

        let result = TimingCheckResult {
            check_name: "CPU Count".to_string(),
            is_suspicious,
            measured_value: cpu_count,
            expected_range: (2.0, 128.0),
            message: if is_suspicious {
                format!("Low CPU count ({}) - possible sandbox", cpu_count as usize)
            } else {
                format!("CPU count: {}", cpu_count as usize)
            },
        };

        self.results.push(result.clone());
        result
    }

    /// Check memory size (sandboxes often have limited RAM)
    pub fn check_memory_size(&mut self) -> TimingCheckResult {
        if self.safe_mode {
            let result = TimingCheckResult {
                check_name: "Memory Size".to_string(),
                is_suspicious: false,
                measured_value: 8192.0,
                expected_range: (4096.0, f64::MAX),
                message: "[SAFE MODE] Memory check simulated".to_string(),
            };
            self.results.push(result.clone());
            return result;
        }

        let mem_mb = Self::get_total_memory_mb();

        // Less than 2GB is suspicious
        let is_suspicious = mem_mb < 2048.0;

        let result = TimingCheckResult {
            check_name: "Memory Size".to_string(),
            is_suspicious,
            measured_value: mem_mb,
            expected_range: (4096.0, f64::MAX),
            message: if is_suspicious {
                format!("Low memory ({:.0} MB) - possible sandbox", mem_mb)
            } else {
                format!("Memory: {:.0} MB", mem_mb)
            },
        };

        self.results.push(result.clone());
        result
    }

    /// Get total system memory in MB
    #[cfg(windows)]
    fn get_total_memory_mb() -> f64 {
        #[repr(C)]
        struct MEMORYSTATUSEX {
            dw_length: u32,
            dw_memory_load: u32,
            ull_total_phys: u64,
            ull_avail_phys: u64,
            ull_total_page_file: u64,
            ull_avail_page_file: u64,
            ull_total_virtual: u64,
            ull_avail_virtual: u64,
            ull_avail_extended_virtual: u64,
        }

        unsafe {
            #[link(name = "kernel32")]
            extern "system" {
                fn GlobalMemoryStatusEx(lpBuffer: *mut MEMORYSTATUSEX) -> i32;
            }

            let mut status = MEMORYSTATUSEX {
                dw_length: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                dw_memory_load: 0,
                ull_total_phys: 0,
                ull_avail_phys: 0,
                ull_total_page_file: 0,
                ull_avail_page_file: 0,
                ull_total_virtual: 0,
                ull_avail_virtual: 0,
                ull_avail_extended_virtual: 0,
            };

            if GlobalMemoryStatusEx(&mut status) != 0 {
                return (status.ull_total_phys / (1024 * 1024)) as f64;
            }
        }

        4096.0 // Default fallback
    }

    #[cfg(not(windows))]
    fn get_total_memory_mb() -> f64 {
        // Read from /proc/meminfo on Linux
        if let Some(kb) = std::fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|line| line.starts_with("MemTotal:"))
                    .and_then(|line| line.split_whitespace().nth(1))
                    .and_then(|kb_str| kb_str.parse::<u64>().ok())
            })
        {
            return (kb / 1024) as f64;
        }
        4096.0 // Default fallback
    }

    /// Run all timing checks
    pub fn run_all_checks(&mut self) -> Vec<TimingCheckResult> {
        self.results.clear();

        self.check_uptime();
        self.check_cpu_count();
        self.check_memory_size();
        self.check_sleep_accuracy();
        self.check_rdtsc_timing();
        self.check_user_activity();

        self.results.clone()
    }

    /// Get overall suspicion score (0.0 - 1.0)
    pub fn get_suspicion_score(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }

        let suspicious_count = self.results.iter().filter(|r| r.is_suspicious).count();
        suspicious_count as f64 / self.results.len() as f64
    }

    /// Get results
    pub fn get_results(&self) -> &[TimingCheckResult] {
        &self.results
    }

    /// Clear results
    pub fn clear(&mut self) {
        self.results.clear();
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timing_checker_creation() {
        let checker = TimingChecker::new();
        assert!(checker.results.is_empty());
    }

    #[test]
    fn test_safe_mode_rdtsc() {
        let mut checker = TimingChecker::new().with_safe_mode(true);
        let result = checker.check_rdtsc_timing();
        assert!(!result.is_suspicious);
        assert!(result.message.contains("SAFE MODE"));
    }

    #[test]
    fn test_safe_mode_sleep() {
        let mut checker = TimingChecker::new().with_safe_mode(true);
        let result = checker.check_sleep_accuracy();
        assert!(!result.is_suspicious);
    }

    #[test]
    fn test_safe_mode_uptime() {
        let mut checker = TimingChecker::new().with_safe_mode(true);
        let result = checker.check_uptime();
        assert!(result.measured_value >= 0.0);
    }

    #[test]
    fn test_safe_mode_cpu_count() {
        let mut checker = TimingChecker::new().with_safe_mode(true);
        let result = checker.check_cpu_count();
        assert!(result.measured_value >= 1.0);
    }

    #[test]
    fn test_safe_mode_memory_size() {
        let mut checker = TimingChecker::new().with_safe_mode(true);
        let result = checker.check_memory_size();
        assert!(result.measured_value > 0.0);
    }

    #[test]
    fn test_safe_mode_user_activity() {
        let mut checker = TimingChecker::new().with_safe_mode(true);
        let result = checker.check_user_activity();
        assert!(!result.is_suspicious);
    }

    #[test]
    fn test_run_all_checks_safe_mode() {
        let mut checker = TimingChecker::new().with_safe_mode(true);
        let results = checker.run_all_checks();
        assert_eq!(results.len(), 6);
    }

    #[test]
    fn test_suspicion_score_no_results() {
        let checker = TimingChecker::new();
        assert_eq!(checker.get_suspicion_score(), 0.0);
    }

    #[test]
    fn test_suspicion_score_safe_mode() {
        let mut checker = TimingChecker::new().with_safe_mode(true);
        checker.run_all_checks();
        // Safe mode should have no suspicious results
        assert_eq!(checker.get_suspicion_score(), 0.0);
    }

    #[test]
    fn test_clear_results() {
        let mut checker = TimingChecker::new().with_safe_mode(true);
        checker.run_all_checks();
        assert!(!checker.get_results().is_empty());

        checker.clear();
        assert!(checker.get_results().is_empty());
    }
}
