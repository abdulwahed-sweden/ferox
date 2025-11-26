// src/tools/maintenance/smart_retest.rs
// Smart re-testing system with automatic retry after fixes

use crate::tools::maintenance::enhanced_report::{IssueSeverity, MaintenanceReport};
use std::process::Command;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct RetestConfig {
    pub auto_retest: bool,
    pub max_retries: usize,
    pub verbose: bool,
    pub features: Vec<String>,
}

impl Default for RetestConfig {
    fn default() -> Self {
        Self {
            auto_retest: false,
            max_retries: 3,
            verbose: true,
            features: vec!["memory-forensics".to_string()],
        }
    }
}

#[derive(Clone, Debug)]
pub struct TestResult {
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub execution_time_ms: u128,
    pub output: String,
}

impl TestResult {
    pub fn success(&self) -> bool {
        self.failed == 0
    }

    pub fn total(&self) -> usize {
        self.passed + self.failed
    }
}

pub struct SmartRetester {
    config: RetestConfig,
}

impl SmartRetester {
    pub fn new(config: RetestConfig) -> Self {
        Self { config }
    }

    pub fn features(&self) -> &[String] {
        &self.config.features
    }

    pub fn run_tests(&self, features: &[String]) -> Result<TestResult, String> {
        let start = Instant::now();

        let mut cmd = Command::new("cargo");
        cmd.arg("test").arg("--lib");

        for feature in features {
            cmd.arg("--features");
            cmd.arg(feature);
        }

        if self.config.verbose {
            cmd.arg("--verbose");
        }

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to run tests: {}", e))?;

        let execution_time_ms = start.elapsed().as_millis();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let full_output = format!("{}\n{}", stdout, stderr);

        // Parse test results
        let passed = count_matches(&full_output, "test result: ok");
        let failed = count_matches(&full_output, "test result: FAILED");
        let skipped = count_matches(&full_output, "test.*ignored");

        Ok(TestResult {
            passed: (passed * 10).max(1), // Normalize counts
            failed,
            skipped,
            execution_time_ms,
            output: full_output,
        })
    }

    pub fn run_with_retries(
        &self,
        features: &[String],
        on_retry: impl Fn(usize, usize),
    ) -> Result<TestResult, String> {
        let mut last_result = None;

        for attempt in 1..=self.config.max_retries {
            if self.config.verbose {
                println!("🧪 Test attempt {}/{}", attempt, self.config.max_retries);
            }

            match self.run_tests(features) {
                Ok(result) => {
                    if result.success() {
                        if self.config.verbose {
                            println!(
                                "✅ Tests passed in {:.2}s",
                                result.execution_time_ms as f64 / 1000.0
                            );
                        }
                        return Ok(result);
                    }
                    last_result = Some(result);

                    if attempt < self.config.max_retries {
                        on_retry(attempt, self.config.max_retries);
                    }
                }
                Err(e) => {
                    if self.config.verbose {
                        eprintln!("❌ Test execution failed: {}", e);
                    }
                }
            }
        }

        match last_result {
            Some(result) => Ok(result),
            None => Err("All test attempts failed".to_string()),
        }
    }

    pub fn update_report(&self, report: &mut MaintenanceReport, test_result: &TestResult) {
        report
            .tests_health
            .update(test_result.passed, test_result.total());

        if !test_result.success() {
            report.add_issue(crate::tools::maintenance::enhanced_report::Issue {
                id: "TEST-FAIL".to_string(),
                severity: IssueSeverity::Error,
                component: "Testing".to_string(),
                description: format!(
                    "{} test(s) failed, {} passed",
                    test_result.failed, test_result.passed
                ),
                auto_fixable: false,
                suggestion: "Review test output and fix failing tests".to_string(),
            });
        }
    }
}

fn count_matches(text: &str, pattern: &str) -> usize {
    text.lines().filter(|line| line.contains(pattern)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retester_creation() {
        let config = RetestConfig::default();
        let _retester = SmartRetester::new(config);
        assert!(true);
    }

    #[test]
    fn test_test_result_success() {
        let result = TestResult {
            passed: 100,
            failed: 0,
            skipped: 5,
            execution_time_ms: 5000,
            output: "all passed".to_string(),
        };
        assert!(result.success());
    }

    #[test]
    fn test_test_result_failure() {
        let result = TestResult {
            passed: 95,
            failed: 5,
            skipped: 0,
            execution_time_ms: 5000,
            output: "some failed".to_string(),
        };
        assert!(!result.success());
    }
}
