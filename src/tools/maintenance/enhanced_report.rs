// src/tools/maintenance/enhanced_report.rs
// Enhanced maintenance reporting with self-diagnosis and scoring

use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub severity: IssueSeverity,
    pub component: String,
    pub description: String,
    pub auto_fixable: bool,
    pub suggestion: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IssueSeverity::Info => write!(f, "INFO"),
            IssueSeverity::Warning => write!(f, "WARNING"),
            IssueSeverity::Error => write!(f, "ERROR"),
            IssueSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReportFormat {
    Text,
    Json,
    Markdown,
    Dashboard,
}

impl fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReportFormat::Text => write!(f, "text"),
            ReportFormat::Json => write!(f, "json"),
            ReportFormat::Markdown => write!(f, "markdown"),
            ReportFormat::Dashboard => write!(f, "dashboard"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceReport {
    pub timestamp: String,
    pub version: String,
    pub framework_status: FrameworkStatus,
    pub structure_health: HealthScore,
    pub build_health: HealthScore,
    pub modules_health: HealthScore,
    pub tests_health: HealthScore,
    pub issues: Vec<Issue>,
    pub auto_fixes_applied: usize,
    pub integrity_score: IntegrityScore,
    pub status_summary: String,
    pub recommendations: Vec<String>,
    pub execution_time_ms: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FrameworkStatus {
    Operational,
    Degraded,
    Critical,
    Initializing,
}

impl fmt::Display for FrameworkStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FrameworkStatus::Operational => write!(f, "✅ Operational"),
            FrameworkStatus::Degraded => write!(f, "⚠️ Degraded"),
            FrameworkStatus::Critical => write!(f, "❌ Critical"),
            FrameworkStatus::Initializing => write!(f, "🔄 Initializing"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthScore {
    pub component: String,
    pub score: f64, // 0.0 to 100.0
    pub status: String,
    pub checks_total: usize,
    pub checks_passed: usize,
}

impl HealthScore {
    pub fn new(component: &str) -> Self {
        Self {
            component: component.to_string(),
            score: 100.0,
            status: "Healthy".to_string(),
            checks_total: 0,
            checks_passed: 0,
        }
    }

    pub fn update(&mut self, passed: usize, total: usize) {
        self.checks_total = total;
        self.checks_passed = passed;
        self.score = if total == 0 {
            100.0
        } else {
            (passed as f64 / total as f64) * 100.0
        };
        self.status = self.calculate_status();
    }

    fn calculate_status(&self) -> String {
        match self.score as i32 {
            90..=100 => "Excellent".to_string(),
            75..=89 => "Good".to_string(),
            60..=74 => "Fair".to_string(),
            40..=59 => "Poor".to_string(),
            _ => "Critical".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntegrityScore {
    pub overall: u32, // 0-100
    pub module_integrity: u32,
    pub build_integrity: u32,
    pub test_integrity: u32,
    pub security_integrity: u32,
    pub trend: ScoreTrend,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScoreTrend {
    Improving,
    Stable,
    Degrading,
    Unknown,
}

impl fmt::Display for ScoreTrend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScoreTrend::Improving => write!(f, "📈 Improving"),
            ScoreTrend::Stable => write!(f, "➡️ Stable"),
            ScoreTrend::Degrading => write!(f, "📉 Degrading"),
            ScoreTrend::Unknown => write!(f, "❓ Unknown"),
        }
    }
}

impl MaintenanceReport {
    pub fn new(version: &str) -> Self {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        Self {
            timestamp: now,
            version: version.to_string(),
            framework_status: FrameworkStatus::Initializing,
            structure_health: HealthScore::new("Structure"),
            build_health: HealthScore::new("Build"),
            modules_health: HealthScore::new("Modules"),
            tests_health: HealthScore::new("Tests"),
            issues: Vec::new(),
            auto_fixes_applied: 0,
            integrity_score: IntegrityScore {
                overall: 100,
                module_integrity: 100,
                build_integrity: 100,
                test_integrity: 100,
                security_integrity: 100,
                trend: ScoreTrend::Stable,
            },
            status_summary: "Initializing...".to_string(),
            recommendations: Vec::new(),
            execution_time_ms: 0,
        }
    }

    pub fn calculate_overall_status(&mut self) {
        let avg_health = (
            self.structure_health.score +
            self.build_health.score +
            self.modules_health.score +
            self.tests_health.score
        ) / 4.0;

        let critical_count = self.issues.iter()
            .filter(|i| i.severity == IssueSeverity::Critical)
            .count();
        let error_count = self.issues.iter()
            .filter(|i| i.severity == IssueSeverity::Error)
            .count();
        let warning_count = self.issues.iter()
            .filter(|i| i.severity == IssueSeverity::Warning)
            .count();

        self.framework_status = match (critical_count, error_count) {
            (0, 0) => FrameworkStatus::Operational,
            (0, 1..=2) => FrameworkStatus::Degraded,
            _ => FrameworkStatus::Critical,
        };

        // Calculate integrity score
        let issue_penalty = (critical_count as u32 * 10) + (error_count as u32 * 5) + (warning_count as u32 * 2);
        self.integrity_score.overall = (100u32).saturating_sub(issue_penalty);
        self.integrity_score.module_integrity = self.modules_health.score as u32;
        self.integrity_score.build_integrity = self.build_health.score as u32;
        self.integrity_score.test_integrity = self.tests_health.score as u32;

        // Generate status summary
        self.status_summary = self.generate_status_summary(
            critical_count,
            error_count,
            warning_count,
            self.auto_fixes_applied,
        );
    }

    fn generate_status_summary(
        &self,
        critical: usize,
        errors: usize,
        warnings: usize,
        fixes: usize,
    ) -> String {
        match (critical, errors, warnings, fixes) {
            (0, 0, 0, 0) => "✅ All systems operational - Framework at peak condition".to_string(),
            (0, 0, 0, n) if n > 0 => format!("✅ All systems operational - Auto-fixed {} issues", n),
            (0, 0, w, 0) => format!("⚠️ {} warnings detected - Review recommended", w),
            (0, 0, w, n) => format!("⚠️ {} warnings found, auto-fixed {} issues", w, n),
            (0, e, _, _) => format!("⚠️ {} errors detected - Action required", e),
            (c, _, _, _) => format!("❌ {} critical issues - Immediate action required", c),
        }
    }

    pub fn add_issue(&mut self, issue: Issue) {
        self.issues.push(issue);
    }

    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# 🩺 Ferox Maintenance Report\n\n");
        md.push_str(&format!("**Generated:** {}\n", self.timestamp));
        md.push_str(&format!("**Version:** {}\n", self.version));
        md.push_str(&format!("**Execution Time:** {}ms\n\n", self.execution_time_ms));

        md.push_str("## System Status\n\n");
        md.push_str(&format!("- **Framework Status:** {}\n", self.framework_status));
        md.push_str(&format!("- **Integrity Score:** {}/100\n", self.integrity_score.overall));
        md.push_str(&format!("- **Trend:** {}\n\n", self.integrity_score.trend));

        md.push_str("## Health Scores\n\n");
        md.push_str("| Component | Score | Status | Tests |\n");
        md.push_str("|-----------|-------|--------|-------|\n");
        md.push_str(&format!(
            "| {} | {:.1}% | {} | {}/{} |\n",
            self.structure_health.component,
            self.structure_health.score,
            self.structure_health.status,
            self.structure_health.checks_passed,
            self.structure_health.checks_total
        ));
        md.push_str(&format!(
            "| {} | {:.1}% | {} | {}/{} |\n",
            self.build_health.component,
            self.build_health.score,
            self.build_health.status,
            self.build_health.checks_passed,
            self.build_health.checks_total
        ));
        md.push_str(&format!(
            "| {} | {:.1}% | {} | {}/{} |\n",
            self.modules_health.component,
            self.modules_health.score,
            self.modules_health.status,
            self.modules_health.checks_passed,
            self.modules_health.checks_total
        ));
        md.push_str(&format!(
            "| {} | {:.1}% | {} | {}/{} |\n\n",
            self.tests_health.component,
            self.tests_health.score,
            self.tests_health.status,
            self.tests_health.checks_passed,
            self.tests_health.checks_total
        ));

        if !self.issues.is_empty() {
            md.push_str("## Issues Found\n\n");
            for issue in &self.issues {
                md.push_str(&format!(
                    "- **[{}]** {}: {}\n  - Suggestion: {}\n",
                    issue.severity, issue.component, issue.description, issue.suggestion
                ));
            }
            md.push_str("\n");
        }

        if self.auto_fixes_applied > 0 {
            md.push_str(&format!("## Auto-fixes Applied\n\n**{} issues automatically fixed**\n\n", self.auto_fixes_applied));
        }

        if !self.recommendations.is_empty() {
            md.push_str("## Recommendations\n\n");
            for (i, rec) in self.recommendations.iter().enumerate() {
                md.push_str(&format!("{}. {}\n", i + 1, rec));
            }
        }

        md
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_score_calculation() {
        let mut score = HealthScore::new("Test");
        score.update(95, 100);
        assert_eq!(score.score, 95.0);
        assert_eq!(score.status, "Excellent");
    }

    #[test]
    fn test_integrity_score_with_issues() {
        let mut report = MaintenanceReport::new("2.0.0");
        report.issues.push(Issue {
            id: "CRIT-001".to_string(),
            severity: IssueSeverity::Critical,
            component: "Module".to_string(),
            description: "Missing module".to_string(),
            auto_fixable: false,
            suggestion: "Add module".to_string(),
        });
        report.calculate_overall_status();
        assert!(report.integrity_score.overall < 100);
    }

    #[test]
    fn test_status_summary_generation() {
        let mut report = MaintenanceReport::new("2.0.0");
        report.calculate_overall_status();
        assert!(report.status_summary.contains("All systems operational"));
    }
}
