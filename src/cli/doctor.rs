use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use clap::{Args, Subcommand, ValueEnum};
use serde_json::to_string_pretty;

use crate::tools::doctor::{DependencyChecker, DoctorReport};
use crate::tools::theme::CliThemeApplier;

#[derive(Subcommand, Debug, Clone)]
pub enum DoctorCommands {
    /// Run comprehensive system health check
    Check(DoctorCheckArgs),
    /// Check a single dependency by name
    Dependency(DoctorDependencyArgs),
    /// Generate a JSON system report
    Report(DoctorReportArgs),
}

#[derive(Args, Debug, Clone)]
pub struct DoctorCheckArgs {
    /// Check only critical dependencies (rust/python/cargo)
    #[arg(short, long)]
    pub critical: bool,

    /// Output format (text|json|markdown)
    #[arg(short, long, default_value = "text")]
    pub format: DoctorOutputFormat,

    /// Attempt to auto-fix issues with known remedies
    #[arg(short, long)]
    pub fix: bool,
}

#[derive(Args, Debug, Clone)]
pub struct DoctorDependencyArgs {
    /// Dependency name (python, rust, cargo, volatility, disk, memory)
    pub name: String,
}

#[derive(Args, Debug, Clone)]
pub struct DoctorReportArgs {
    /// Optional output file (writes JSON report)
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum DoctorOutputFormat {
    Text,
    Json,
    Markdown,
}

pub fn handle_doctor_command(cmd: DoctorCommands, theme: &CliThemeApplier) -> Result<()> {
    match cmd {
        DoctorCommands::Check(args) => run_check(args, theme),
        DoctorCommands::Dependency(args) => check_specific_dependency(args, theme),
        DoctorCommands::Report(args) => generate_system_report(args, theme),
    }
}

fn run_check(args: DoctorCheckArgs, theme: &CliThemeApplier) -> Result<()> {
    let mut checker = DependencyChecker::new();
    let report = checker.comprehensive_check(args.critical);

    match args.format {
        DoctorOutputFormat::Text => report.print_report(Some(theme)),
        DoctorOutputFormat::Json => {
            println!("{}", to_string_pretty(&report.to_json())?);
        }
        DoctorOutputFormat::Markdown => {
            println!("{}", generate_markdown_report(&report));
        }
    }

    if args.fix {
        auto_fix_issues(&report, theme);
    }

    Ok(())
}

fn check_specific_dependency(args: DoctorDependencyArgs, theme: &CliThemeApplier) -> Result<()> {
    let mut checker = DependencyChecker::new();
    let Some(result) = checker.check_named(&args.name) else {
        return Err(anyhow!("Unknown dependency: {}", args.name));
    };

    let mut report = DoctorReport::new();
    report.add_result(result);
    report.print_report(Some(theme));
    Ok(())
}

fn generate_system_report(args: DoctorReportArgs, theme: &CliThemeApplier) -> Result<()> {
    let mut checker = DependencyChecker::new();
    let report = checker.comprehensive_check(false);

    if let Some(output) = args.output {
        write_report(&report, Path::new(&output))?;
    }

    report.print_report(Some(theme));
    Ok(())
}

fn write_report(report: &DoctorReport, path: &Path) -> Result<()> {
    let json = to_string_pretty(&report.to_json())?;
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, json).with_context(|| format!("Unable to write report to {}", path.display()))
}

fn auto_fix_issues(report: &DoctorReport, theme: &CliThemeApplier) {
    for result in &report.results {
        if let Some(suggestion) = &result.suggestion {
            println!(
                "{}",
                theme.format_hint(&format!("{}: {}", result.check_name, suggestion))
            );
        }
    }
}

fn generate_markdown_report(report: &DoctorReport) -> String {
    let mut buffer = String::new();
    buffer.push_str("# Ferox Doctor Report\n\n");
    buffer.push_str(&format!(
        "**Timestamp:** {}\\n\n",
        report.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
    ));
    buffer.push_str(&format!(
        "**Overall Status:** {:?}\\n\n",
        report.overall_status
    ));
    buffer.push_str("| Check | Status | Message | Suggestion |\n");
    buffer.push_str("| --- | --- | --- | --- |\n");
    for result in &report.results {
        buffer.push_str(&format!(
            "| {} | {:?} | {} | {} |\n",
            result.check_name,
            result.status,
            result.message.replace('|', r"\|"),
            result
                .suggestion
                .clone()
                .unwrap_or_else(|| "—".to_string())
                .replace('|', r"\|")
        ));
    }

    buffer
}
