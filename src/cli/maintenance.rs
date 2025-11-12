/// Maintenance CLI commands for Ferox framework
use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Subcommand)]
pub enum MaintenanceCommands {
    /// Run comprehensive system health check
    Check {
        /// Check module visibility and registration
        #[arg(short, long)]
        modules: bool,

        /// Check build configuration
        #[arg(short, long)]
        build: bool,

        /// Check directory structure
        #[arg(short, long)]
        structure: bool,

        /// Run all checks
        #[arg(short, long)]
        all: bool,
    },

    /// Fix detected issues automatically
    Fix {
        /// Fix module registration and missing files
        #[arg(short, long)]
        modules: bool,

        /// Fix build configuration
        #[arg(short, long)]
        config: bool,

        /// Apply all available fixes
        #[arg(short, long)]
        all: bool,
    },

    /// Generate detailed system diagnostic report
    Diagnose {
        /// Output format: text, json, or markdown
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Save report to file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Quick framework health status
    Doctor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticOutput {
    pub timestamp: String,
    pub framework_version: String,
    pub module_count: usize,
    pub missing_modules: Vec<String>,
    pub health_status: String,
}

impl DiagnosticOutput {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_markdown(&self) -> String {
        format!(
            r#"# Ferox Diagnostic Report

**Generated:** {}
**Framework Version:** {}

## Summary

- **Total Modules:** {}
- **Missing Modules:** {}
- **Health Status:** {}

## Missing Modules

{}
"#,
            self.timestamp,
            self.framework_version,
            self.module_count,
            self.missing_modules.len(),
            self.health_status,
            if self.missing_modules.is_empty() {
                "None - All modules present".to_string()
            } else {
                self.missing_modules
                    .iter()
                    .map(|m| format!("- {}", m))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        )
    }
}
