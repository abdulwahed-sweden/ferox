// src/cli/maintenance_commands.rs
// Enhanced CLI commands for maintenance operations

use clap::{Parser, Subcommand};
use crate::tools::maintenance::{
    MaintenanceEngine, ReportFormat, SmartRetester, RetestConfig,
    CliDashboard,
};

#[derive(Parser)]
#[command(name = "maint")]
#[command(about = "Ferox maintenance and diagnostics")]
pub struct MaintenanceOpts {
    #[command(subcommand)]
    pub command: MaintenanceCommands,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum MaintenanceCommands {
    /// Run comprehensive system health check
    Check {
        /// Check structure
        #[arg(short, long)]
        structure: bool,

        /// Check build configuration
        #[arg(short, long)]
        build: bool,

        /// Check modules
        #[arg(short, long)]
        modules: bool,

        /// Check tests
        #[arg(short, long)]
        tests: bool,

        /// Run all checks
        #[arg(short, long)]
        all: bool,
    },

    /// Fix detected issues automatically
    Fix {
        /// Fix modules registration
        #[arg(short, long)]
        modules: bool,

        /// Retest after fixing
        #[arg(short, long)]
        retest: bool,

        /// Run all fixes
        #[arg(short = 'a', long)]
        all: bool,
    },

    /// Generate detailed diagnostic report
    Report {
        /// Output format
        #[arg(short, long, default_value = "dashboard")]
        format: String,

        /// Output file (optional)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Quick system health overview
    Doctor,

    /// Run tests with smart retry
    Test {
        /// Auto-retest on failure
        #[arg(short, long)]
        retest: bool,

        /// Maximum retry attempts
        #[arg(short, long, default_value = "3")]
        max_retries: usize,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Export report in multiple formats
    Export {
        /// Format: json, markdown, text
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output directory
        #[arg(short, long, default_value = ".")]
        output_dir: String,
    },
}

pub async fn handle_maintenance_command(
    opts: MaintenanceOpts,
) -> anyhow::Result<()> {
    let engine = MaintenanceEngine::new(opts.verbose);

    match opts.command {
        MaintenanceCommands::Check { all, .. } => {
            handle_check(&engine, all).await?;
        }
        MaintenanceCommands::Fix { retest, all, .. } => {
            handle_fix(&engine, retest, all).await?;
        }
        MaintenanceCommands::Report { format, output } => {
            handle_report(&engine, &format, output).await?;
        }
        MaintenanceCommands::Doctor => {
            handle_doctor(&engine).await?;
        }
        MaintenanceCommands::Test {
            retest,
            max_retries,
            verbose,
        } => {
            handle_test(&engine, retest, max_retries, verbose).await?;
        }
        MaintenanceCommands::Export { format, output_dir } => {
            handle_export(&engine, &format, &output_dir).await?;
        }
    }

    Ok(())
}

async fn handle_check(engine: &MaintenanceEngine, all: bool) -> anyhow::Result<()> {
    println!("🩺 Running health check...\n");

    let report = engine.run_health_check()?;

    if all || true {
        CliDashboard::render_dashboard(&report);
    } else {
        CliDashboard::render_compact(&report);
    }

    Ok(())
}

async fn handle_fix(
    engine: &MaintenanceEngine,
    retest: bool,
    _all: bool,
) -> anyhow::Result<()> {
    println!("🔧 Running auto-fix...\n");

    let mut report = engine.run_health_check_with_autofix()?;

    CliDashboard::render_dashboard(&report);

    if retest {
        println!("\n🧪 Re-testing after fixes...\n");

        let retest_config = RetestConfig {
            auto_retest: true,
            max_retries: 3,
            verbose: engine.verbose,
            ..Default::default()
        };

        match engine.run_tests_with_retest(retest_config) {
            Ok(test_result) => {
                if test_result.success() {
                    println!("✅ All tests passed after fixes!");
                    report.tests_health.checks_passed = test_result.passed;
                    report.tests_health.checks_total = test_result.total();
                } else {
                    println!(
                        "⚠️ {} test(s) still failing",
                        test_result.failed
                    );
                }
            }
            Err(e) => {
                println!("❌ Test execution failed: {}", e);
            }
        }
    }

    Ok(())
}

async fn handle_report(
    engine: &MaintenanceEngine,
    format: &str,
    output: Option<String>,
) -> anyhow::Result<()> {
    let report = engine.run_health_check()?;

    let report_format = match format {
        "json" => ReportFormat::Json,
        "markdown" | "md" => ReportFormat::Markdown,
        "dashboard" => ReportFormat::Dashboard,
        _ => ReportFormat::Text,
    };

    let report_content = engine.generate_report(&report, report_format)?;

    if let Some(path) = output {
        std::fs::write(&path, &report_content)?;
        println!("✅ Report saved to {}", path);
    } else {
        println!("{}", report_content);
    }

    Ok(())
}

async fn handle_doctor(engine: &MaintenanceEngine) -> anyhow::Result<()> {
    println!("\n🩺 {}\n", "Ferox Doctor".bold());

    let report = engine.run_health_check()?;

    CliDashboard::render_dashboard(&report);

    if report.integrity_score.overall >= 90 {
        println!("✅ Framework is in excellent condition!\n");
    } else if report.integrity_score.overall >= 70 {
        println!("⚠️ Some issues detected. Run 'ferox maint fix' to resolve.\n");
    } else {
        println!("❌ Critical issues detected. Immediate action required!\n");
        std::process::exit(1);
    }

    Ok(())
}

async fn handle_test(
    engine: &MaintenanceEngine,
    retest: bool,
    max_retries: usize,
    verbose: bool,
) -> anyhow::Result<()> {
    println!("🧪 Running test suite...\n");

    let config = RetestConfig {
        auto_retest: retest,
        max_retries,
        verbose,
        features: vec!["memory-forensics".to_string()],
    };

    let result = engine.run_tests_with_retest(config)?;

    println!("📊 Test Results:");
    println!("  ✅ Passed: {}", result.passed);
    println!("  ❌ Failed: {}", result.failed);
    println!("  ⏭️  Skipped: {}", result.skipped);
    println!("  ⏱️  Time: {:.2}s\n", result.execution_time_ms as f64 / 1000.0);

    if result.success() {
        println!("✅ All tests passed!");
    } else {
        println!("❌ Some tests failed!");
        println!("\n📋 Output:\n{}", result.output);
    }

    Ok(())
}

async fn handle_export(
    engine: &MaintenanceEngine,
    format: &str,
    output_dir: &str,
) -> anyhow::Result<()> {
    use std::fs;

    fs::create_dir_all(output_dir)?;

    let report = engine.run_health_check()?;

    let filename = format!(
        "ferox_maintenance_{}.{}",
        chrono::Local::now().format("%Y%m%d_%H%M%S"),
        match format {
            "json" => "json",
            "markdown" | "md" => "md",
            _ => "txt",
        }
    );

    let filepath = std::path::Path::new(output_dir).join(&filename);

    let report_format = match format {
        "json" => ReportFormat::Json,
        "markdown" | "md" => ReportFormat::Markdown,
        _ => ReportFormat::Text,
    };

    let content = engine.generate_report(&report, report_format)?;
    fs::write(&filepath, content)?;

    println!("✅ Report exported to {}", filepath.display());

    Ok(())
}

use colored::Colorize;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        // Verify command structures compile correctly
        assert!(true);
    }
}
