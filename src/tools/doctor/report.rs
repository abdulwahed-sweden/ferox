use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;
use serde_json::json;

use crate::tools::theme::CliThemeApplier;

use super::types::{CheckResult, CheckStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverallStatus {
    Healthy,
    Degraded,
    Critical,
}

pub struct DoctorReport {
    pub results: Vec<CheckResult>,
    pub overall_status: OverallStatus,
    pub timestamp: DateTime<Utc>,
}

impl DoctorReport {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            overall_status: OverallStatus::Healthy,
            timestamp: Utc::now(),
        }
    }

    pub fn add_result(&mut self, result: CheckResult) {
        match result.status {
            CheckStatus::Error => self.overall_status = OverallStatus::Critical,
            CheckStatus::Warning if matches!(self.overall_status, OverallStatus::Healthy) => {
                self.overall_status = OverallStatus::Degraded;
            }
            _ => {}
        }

        self.results.push(result);
    }

    pub fn print_report(&self, theme: Option<&CliThemeApplier>) {
        let header = "🩺 Ferox Doctor Report";
        if let Some(theme) = theme {
            println!("{}", theme.format_section_header(header));
        } else {
            println!("{}", header.bold().blue());
        }
        println!("{}", "=".repeat(50).dimmed());
        println!(
            "Timestamp: {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );
        println!();

        for result in &self.results {
            self.print_result(result, theme);
        }

        println!();
        match self.overall_status {
            OverallStatus::Healthy => {
                let line = "🎉 All systems operational - Ferox is ready to use!";
                if let Some(theme) = theme {
                    println!("{} {}", theme.success_icon(), theme.format_success(line));
                } else {
                    println!("{}", line.green().bold());
                }
            }
            OverallStatus::Degraded => {
                let line = "⚠️  Some issues detected - Review warnings above";
                if let Some(theme) = theme {
                    println!("{} {}", theme.warning_icon(), theme.format_warning(line));
                } else {
                    println!("{}", line.yellow().bold());
                }
            }
            OverallStatus::Critical => {
                let line = "❌ Critical issues found - Please fix errors above";
                if let Some(theme) = theme {
                    println!("{} {}", theme.error_icon(), theme.format_error(line));
                } else {
                    println!("{}", line.red().bold());
                }
            }
        }
    }

    fn print_result(&self, result: &CheckResult, theme: Option<&CliThemeApplier>) {
        match result.status {
            CheckStatus::Success => {
                if let Some(theme) = theme {
                    println!(
                        "{} {}",
                        theme.success_icon(),
                        theme.format_success(&result.message)
                    );
                } else {
                    println!("{} {}", "✅".green(), result.message.clone());
                }
            }
            CheckStatus::Warning => {
                if let Some(theme) = theme {
                    println!(
                        "{} {}",
                        theme.warning_icon(),
                        theme.format_warning(&result.message)
                    );
                } else {
                    println!("{} {}", "⚠️".yellow(), result.message.clone());
                }
                if let Some(suggestion) = &result.suggestion {
                    if let Some(theme) = theme {
                        println!("{}", theme.format_hint(suggestion));
                    } else {
                        println!("   💡 {}", suggestion.dimmed());
                    }
                }
            }
            CheckStatus::Error => {
                if let Some(theme) = theme {
                    println!(
                        "{} {}",
                        theme.error_icon(),
                        theme.format_error(&result.message)
                    );
                } else {
                    println!("{} {}", "❌".red(), result.message.clone());
                }
                if let Some(suggestion) = &result.suggestion {
                    if let Some(theme) = theme {
                        println!("{}", theme.format_hint(suggestion));
                    } else {
                        println!("   🛠️  {}", suggestion.dimmed());
                    }
                }
            }
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "timestamp": self.timestamp.to_rfc3339(),
            "overall_status": match self.overall_status {
                OverallStatus::Healthy => "healthy",
                OverallStatus::Degraded => "degraded",
                OverallStatus::Critical => "critical",
            },
            "results": self
                .results
                .iter()
                .map(|result| {
                    json!({
                        "check_name": result.check_name,
                        "status": match result.status {
                            CheckStatus::Success => "success",
                            CheckStatus::Warning => "warning",
                            CheckStatus::Error => "error",
                        },
                        "message": result.message,
                        "suggestion": result.suggestion,
                    })
                })
                .collect::<Vec<_>>()
        })
    }

    pub fn has_errors(&self) -> bool {
        self.results
            .iter()
            .any(|result| matches!(result.status, CheckStatus::Error))
    }
}

impl Default for DoctorReport {
    fn default() -> Self {
        Self::new()
    }
}
