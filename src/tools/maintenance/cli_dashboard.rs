// src/tools/maintenance/cli_dashboard.rs
// Colorized CLI dashboard for maintenance reports

use crate::tools::maintenance::enhanced_report::{
    FrameworkStatus, IssueSeverity, MaintenanceReport,
};
use colored::Colorize;

pub struct CliDashboard;

impl CliDashboard {
    pub fn render_dashboard(report: &MaintenanceReport) {
        Self::print_header();
        Self::print_status_section(report);
        Self::print_health_scores_section(report);
        Self::print_integrity_section(report);
        Self::print_issues_section(report);
        Self::print_fixes_section(report);
        Self::print_recommendations_section(report);
        Self::print_footer(report);
    }

    fn print_header() {
        println!("\n{}", "═".repeat(70).cyan());
        println!(
            "{}",
            format!("🩺 Ferox Maintenance Health Report").bold().cyan()
        );
        println!("{}", "═".repeat(70).cyan());
    }

    fn print_status_section(report: &MaintenanceReport) {
        println!("\n{}", "📊 SYSTEM STATUS".bold().white());
        println!("{}", "─".repeat(70).dimmed());

        let status_emoji = match report.framework_status {
            FrameworkStatus::Operational => "✅".green(),
            FrameworkStatus::Degraded => "⚠️".yellow(),
            FrameworkStatus::Critical => "❌".red(),
            FrameworkStatus::Initializing => "🔄".cyan(),
        };

        println!(
            "{}  {} {}",
            status_emoji,
            "Status:".bold(),
            match report.framework_status {
                FrameworkStatus::Operational => "Operational".green().bold(),
                FrameworkStatus::Degraded => "Degraded".yellow().bold(),
                FrameworkStatus::Critical => "Critical".red().bold(),
                FrameworkStatus::Initializing => "Initializing".cyan().bold(),
            }
        );

        println!(
            "{}  {} {}",
            "📅".white(),
            "Timestamp:".bold(),
            report.timestamp.dimmed()
        );

        println!(
            "{}  {} {}",
            "⚡".yellow(),
            "Execution Time:".bold(),
            format!("{}ms", report.execution_time_ms).white()
        );

        println!(
            "{}  {} {}",
            "🔖".cyan(),
            "Version:".bold(),
            report.version.bright_cyan()
        );

        println!(
            "{}",
            format!("\n{}", report.status_summary).bold().bright_white()
        );
    }

    fn print_health_scores_section(report: &MaintenanceReport) {
        println!("\n{}", "💚 HEALTH SCORES".bold().white());
        println!("{}", "─".repeat(70).dimmed());

        let components = vec![
            &report.structure_health,
            &report.build_health,
            &report.modules_health,
            &report.tests_health,
        ];

        for health in components {
            let score_color = match health.score as i32 {
                90..=100 => health.score.to_string().green(),
                75..=89 => health.score.to_string().bright_green(),
                60..=74 => health.score.to_string().yellow(),
                40..=59 => health.score.to_string().bright_red(),
                _ => health.score.to_string().red(),
            };

            let status_color = match health.status.as_str() {
                "Excellent" => health.status.green(),
                "Good" => health.status.bright_green(),
                "Fair" => health.status.yellow(),
                "Poor" => health.status.bright_red(),
                "Critical" => health.status.red(),
                _ => health.status.white(),
            };

            println!(
                "  {} {:<15} {:.1}% {:<12} ({}/{})",
                Self::get_health_icon(health.score),
                health.component.bold().white(),
                score_color,
                status_color,
                health.checks_passed,
                health.checks_total
            );
        }
    }

    fn print_integrity_section(report: &MaintenanceReport) {
        println!("\n{}", "🔐 INTEGRITY SCORE".bold().white());
        println!("{}", "─".repeat(70).dimmed());

        let overall_color = match report.integrity_score.overall {
            90..=100 => report.integrity_score.overall.to_string().green(),
            75..=89 => report.integrity_score.overall.to_string().bright_green(),
            60..=74 => report.integrity_score.overall.to_string().yellow(),
            40..=59 => report.integrity_score.overall.to_string().bright_red(),
            _ => report.integrity_score.overall.to_string().red(),
        };

        println!(
            "  {} {} {}%",
            "🎯".cyan(),
            "Overall Score:".bold(),
            overall_color
        );

        println!(
            "  {} {} {}%",
            "🧩".blue(),
            "Module Integrity:".white(),
            report.integrity_score.module_integrity.to_string().cyan()
        );

        println!(
            "  {} {} {}%",
            "🔨".yellow(),
            "Build Integrity:".white(),
            report.integrity_score.build_integrity.to_string().cyan()
        );

        println!(
            "  {} {} {}%",
            "🧪".green(),
            "Test Integrity:".white(),
            report.integrity_score.test_integrity.to_string().cyan()
        );

        println!(
            "  {} {} {}",
            "📈".bright_yellow(),
            "Trend:".white(),
            report.integrity_score.trend.to_string().bold()
        );
    }

    fn print_issues_section(report: &MaintenanceReport) {
        if report.issues.is_empty() {
            println!("\n{}", "✅ NO ISSUES FOUND".bold().green());
            return;
        }

        println!(
            "\n{}",
            format!("⚠️  ISSUES ({} found)", report.issues.len())
                .bold()
                .yellow()
        );
        println!("{}", "─".repeat(70).dimmed());

        let mut issues_by_severity = vec![
            (IssueSeverity::Critical, Vec::new()),
            (IssueSeverity::Error, Vec::new()),
            (IssueSeverity::Warning, Vec::new()),
            (IssueSeverity::Info, Vec::new()),
        ];

        for issue in &report.issues {
            for (severity, issues_vec) in &mut issues_by_severity {
                if &issue.severity == severity {
                    issues_vec.push(issue);
                }
            }
        }

        for (severity, issues_vec) in issues_by_severity {
            if !issues_vec.is_empty() {
                println!("\n  {}", format!("[{}]", severity).bold().red());
                for issue in issues_vec {
                    println!(
                        "    • {} ({}) - {}",
                        issue.component.bright_white(),
                        issue.id.dimmed(),
                        issue.description
                    );
                    println!("      💡 {}", issue.suggestion.italic().bright_black());
                }
            }
        }
    }

    fn print_fixes_section(report: &MaintenanceReport) {
        if report.auto_fixes_applied > 0 {
            println!(
                "\n{}",
                format!("🔧 AUTO-FIXES APPLIED ({})", report.auto_fixes_applied)
                    .bold()
                    .green()
            );
            println!("{}", "─".repeat(70).dimmed());
            println!(
                "  {} Automatically resolved {} issue(s)",
                "✅".green(),
                report.auto_fixes_applied.to_string().bright_green()
            );
        }
    }

    fn print_recommendations_section(report: &MaintenanceReport) {
        if !report.recommendations.is_empty() {
            println!(
                "\n{}",
                format!("💡 RECOMMENDATIONS ({})", report.recommendations.len())
                    .bold()
                    .cyan()
            );
            println!("{}", "─".repeat(70).dimmed());
            for (i, rec) in report.recommendations.iter().enumerate() {
                println!("  {}. {}", (i + 1).to_string().cyan().bold(), rec);
            }
        }
    }

    fn print_footer(report: &MaintenanceReport) {
        println!("\n{}", "─".repeat(70).dimmed());

        let status_line = match report.framework_status {
            FrameworkStatus::Operational => {
                format!(
                    "{}  All systems operational - Framework at peak condition 💪",
                    "✅".green().bold()
                )
            }
            FrameworkStatus::Degraded => {
                format!(
                    "{}  Some issues detected - Review and resolve recommended",
                    "⚠️".yellow().bold()
                )
            }
            FrameworkStatus::Critical => {
                format!(
                    "{}  Critical issues detected - Immediate action required",
                    "❌".red().bold()
                )
            }
            FrameworkStatus::Initializing => {
                format!("{}  System initializing - Please wait", "🔄".cyan().bold())
            }
        };

        println!("{}", status_line);
        println!("{}", "═".repeat(70).cyan());
        println!();
    }

    fn get_health_icon(score: f64) -> &'static str {
        match score as i32 {
            90..=100 => "✅",
            75..=89 => "✅",
            60..=74 => "⚠️",
            40..=59 => "❌",
            _ => "🔴",
        }
    }

    pub fn render_compact(report: &MaintenanceReport) {
        println!("\n🩺 {}", "Quick Health Check".bold());
        println!("  Status: {}", report.status_summary);
        println!("  Integrity: {}%", report.integrity_score.overall);
        println!("  Issues: {}", report.issues.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::maintenance::enhanced_report::MaintenanceReport;

    #[test]
    fn test_dashboard_rendering() {
        let report = MaintenanceReport::new("2.0.0");
        // Just verify it doesn't panic
        CliDashboard::render_compact(&report);
    }
}
