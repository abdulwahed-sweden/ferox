use anyhow::Result;

use crate::cli::doctor::{DoctorCommands, handle_doctor_command};
use crate::cli::theme::Theme;
use crate::tools::theme::CliThemeApplier;

pub struct DoctorCommandHandler<'a> {
    theme: &'a CliThemeApplier,
}

impl<'a> DoctorCommandHandler<'a> {
    pub fn new(theme: &'a CliThemeApplier) -> Self {
        Self { theme }
    }

    pub fn describe() -> &'static str {
        "Ferox Doctor diagnostics and dependency validation"
    }

    pub fn print_usage() {
        Theme::section("Doctor CLI");
        Theme::command_help(
            "ferox doctor check",
            "Run comprehensive system health checks",
        );
        Theme::command_help(
            "ferox doctor dependency <name>",
            "Inspect a single dependency (python, rust, ...)",
        );
        Theme::command_help(
            "ferox doctor report --output report.json",
            "Write JSON report to disk",
        );
    }

    pub fn run(&self, command: DoctorCommands) -> Result<()> {
        Theme::section("Doctor");
        handle_doctor_command(command, self.theme)
    }
}
