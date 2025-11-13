use std::path::Path;
use std::process::Command;

use sysinfo::{Disks, System};

use super::report::DoctorReport;
use super::types::{CheckResult, SystemRequirements};

/// Performs Ferox Doctor dependency checks and aggregates the results.
pub struct DependencyChecker {
    pub python_checked: bool,
    pub volatility_checked: bool,
    pub system_requirements: SystemRequirements,
}

impl DependencyChecker {
    pub fn new() -> Self {
        Self {
            python_checked: false,
            volatility_checked: false,
            system_requirements: SystemRequirements::default(),
        }
    }

    /// Run python --version and capture output.
    pub fn check_python(&mut self) -> CheckResult {
        let output = Command::new("python").arg("--version").output();

        match output {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(if output.stdout.is_empty() {
                    &output.stderr
                } else {
                    &output.stdout
                });
                self.python_checked = true;
                CheckResult::success("Python", format!("Python detected: {}", version.trim()))
            }
            _ => CheckResult::error(
                "Python",
                "Python not found or not accessible".to_string(),
                Some("Install Python 3.8+ and ensure it is on PATH".into()),
            ),
        }
    }

    /// Attempt to import volatility via python -c.
    pub fn check_volatility(&mut self) -> CheckResult {
        let output = Command::new("python")
            .args(["-c", "import volatility; print(volatility.__version__)"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                self.volatility_checked = true;
                CheckResult::success(
                    "Volatility",
                    format!("Volatility detected: {}", version.trim()),
                )
            }
            _ => CheckResult::warning(
                "Volatility",
                "Volatility framework not found".to_string(),
                Some("Install volatility or enable the 'volatility-bridge' feature".into()),
            ),
        }
    }

    /// Check rustc availability and minimum version string presence.
    pub fn check_rust(&self) -> CheckResult {
        let output = Command::new("rustc").arg("--version").output();
        match output {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                let message = format!("Rust toolchain detected: {}", version.trim());
                self.version_gate(message, &self.system_requirements.min_rust_version, "Rust")
            }
            _ => CheckResult::error(
                "Rust",
                "rustc command not found".to_string(),
                Some("Install Rust via https://rustup.rs/".into()),
            ),
        }
    }

    /// Check cargo availability.
    pub fn check_cargo(&self) -> CheckResult {
        let output = Command::new("cargo").arg("--version").output();
        match output {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                CheckResult::success("Cargo", format!("Cargo detected: {}", version.trim()))
            }
            _ => CheckResult::error(
                "Cargo",
                "cargo command not found".to_string(),
                Some("Install Rust/Cargo via rustup".into()),
            ),
        }
    }

    /// Iterate through required tools and attempt to execute them with --version.
    pub fn check_system_libraries(&self) -> CheckResult {
        let mut missing = Vec::new();
        for tool in &self.system_requirements.required_tools {
            let output = Command::new(tool).arg("--version").output();
            if output.is_err() || !output.as_ref().unwrap().status.success() {
                missing.push(tool.clone());
            }
        }

        if missing.is_empty() {
            CheckResult::success(
                "System libraries",
                "All required command-line tools are available".into(),
            )
        } else {
            CheckResult::warning(
                "System libraries",
                format!("Missing tools: {}", missing.join(", ")),
                Some("Install the missing tools via your package manager".into()),
            )
        }
    }

    pub fn check_disk_space(&self) -> CheckResult {
        let disks = Disks::new_with_refreshed_list();

        if disks.is_empty() {
            return CheckResult::warning(
                "Disk space",
                "Unable to detect any mounted disks".into(),
                Some("Ensure Ferox Doctor is run with sufficient permissions".into()),
            );
        }

        let available_bytes: u64 = disks.iter().map(|disk| disk.available_space()).sum();
        let available_mb = available_bytes / 1024 / 1024;

        if available_mb >= self.system_requirements.min_disk_space_mb {
            CheckResult::success(
                "Disk space",
                format!("Disk space OK: {available_mb} MB free"),
            )
        } else {
            CheckResult::warning(
                "Disk space",
                format!(
                    "Only {available_mb} MB free (min {} MB)",
                    self.system_requirements.min_disk_space_mb
                ),
                Some("Free up disk space before running heavy modules".into()),
            )
        }
    }

    pub fn check_memory(&self) -> CheckResult {
        let mut system = System::new_all();
        system.refresh_memory();
        let total_mb = system.total_memory() / 1024;

        if total_mb >= self.system_requirements.min_ram_mb {
            CheckResult::success("Memory", format!("Memory OK: {total_mb} MB detected"))
        } else {
            CheckResult::warning(
                "Memory",
                format!(
                    "Only {total_mb} MB RAM available (min {} MB)",
                    self.system_requirements.min_ram_mb
                ),
                Some("Consider running Ferox on a system with more RAM".into()),
            )
        }
    }

    pub fn check_ferox_requirements(&self) -> CheckResult {
        let required_paths = [
            Path::new("config"),
            Path::new("modules"),
            Path::new("wordlist.txt"),
        ];

        let missing: Vec<String> = required_paths
            .iter()
            .filter(|path| !path.exists())
            .map(|path| path.display().to_string())
            .collect();

        if missing.is_empty() {
            CheckResult::success(
                "Ferox requirements",
                "All Ferox directories detected".into(),
            )
        } else {
            CheckResult::warning(
                "Ferox requirements",
                format!("Missing Ferox assets: {}", missing.join(", ")),
                Some("Run setup.sh to regenerate missing artifacts".into()),
            )
        }
    }

    pub fn check_system_requirements(&self) -> Vec<CheckResult> {
        vec![
            self.check_system_libraries(),
            self.check_disk_space(),
            self.check_memory(),
        ]
    }

    pub fn comprehensive_check(&mut self, critical_only: bool) -> DoctorReport {
        let mut report = DoctorReport::new();

        report.add_result(self.check_python());
        report.add_result(self.check_volatility());
        report.add_result(self.check_rust());
        report.add_result(self.check_cargo());

        if !critical_only {
            for check in self.check_system_requirements() {
                report.add_result(check);
            }
            report.add_result(self.check_ferox_requirements());
        }

        report
    }

    pub fn check_named(&mut self, dependency: &str) -> Option<CheckResult> {
        match dependency.to_ascii_lowercase().as_str() {
            "python" | "python3" => Some(self.check_python()),
            "volatility" => Some(self.check_volatility()),
            "rust" | "rustc" => Some(self.check_rust()),
            "cargo" => Some(self.check_cargo()),
            "disk" | "disk-space" => Some(self.check_disk_space()),
            "memory" | "ram" => Some(self.check_memory()),
            "ferox" => Some(self.check_ferox_requirements()),
            tool => self
                .system_requirements
                .required_tools
                .iter()
                .find(|t| t.eq_ignore_ascii_case(tool))
                .map(|_| self.check_system_libraries()),
        }
    }

    fn version_gate(&self, message: String, min_version: &str, name: &str) -> CheckResult {
        if message.contains(min_version) {
            CheckResult::success(name, message)
        } else {
            CheckResult::warning(
                name,
                format!(
                    "{message} (Ferox recommends >= {min_version})",
                    message = message.trim()
                ),
                Some(format!("Update {name} to at least version {min_version}")),
            )
        }
    }
}

impl Default for DependencyChecker {
    fn default() -> Self {
        Self::new()
    }
}
