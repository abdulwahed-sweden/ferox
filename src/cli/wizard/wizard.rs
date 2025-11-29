//! Attack Wizard Main Implementation
//!
//! Orchestrates the wizard flow and handles CLI arguments

use super::executor::{PlanExecutor, export_results};
use super::plan::AttackPlan;
use super::steps::*;
use super::templates::{AttackTemplate, TemplateType};
use super::types::*;
use crate::core::module::ModuleRegistry;
use anyhow::{Result, bail};
use colored::Colorize;
use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Attack Wizard CLI arguments
#[derive(Debug, Clone, Default)]
pub struct WizardArgs {
    /// Skip to module selection (quick mode)
    pub quick: bool,
    /// Use a pre-built template
    pub template: Option<String>,
    /// Pre-fill target
    pub target: Option<String>,
    /// Load a saved plan
    pub load: Option<String>,
    /// Resume interrupted plan
    pub resume: bool,
    /// Export plan to file without executing
    pub export: Option<String>,
    /// Execute plan from file
    pub execute: Option<String>,
    /// Run in non-interactive mode (requires --execute)
    pub non_interactive: bool,
}

/// Attack Wizard
pub struct AttackWizard {
    state: WizardState,
    registry: Arc<Mutex<ModuleRegistry>>,
    args: WizardArgs,
}

impl AttackWizard {
    /// Create new wizard instance
    pub fn new(registry: Arc<Mutex<ModuleRegistry>>) -> Self {
        Self {
            state: WizardState::default(),
            registry,
            args: WizardArgs::default(),
        }
    }

    /// Create wizard with arguments
    pub fn with_args(mut self, args: WizardArgs) -> Self {
        self.args = args;
        self
    }

    /// Run the wizard
    pub async fn run(&mut self) -> Result<()> {
        // Check for non-interactive mode
        if self.args.non_interactive {
            return self.run_non_interactive().await;
        }

        // Check for TTY
        if !atty::is(atty::Stream::Stdin) {
            println!("{}", "━".repeat(80).dimmed());
            println!("{}", "⚠ Attack Wizard requires an interactive terminal.".yellow());
            println!();
            println!("For non-interactive use, provide a plan file:");
            println!("  {} {}", "ferox wizard --execute".cyan(), "plan.yaml".dimmed());
            println!();
            println!("Or create a plan interactively first:");
            println!("  {} {}", "ferox wizard --export".cyan(), "plan.yaml".dimmed());
            println!("{}", "━".repeat(80).dimmed());
            bail!("Non-interactive mode requires --execute with a plan file");
        }

        // Handle --resume
        if self.args.resume {
            return self.handle_resume().await;
        }

        // Handle --load
        if let Some(path) = self.args.load.clone() {
            return self.load_and_execute(&path).await;
        }

        // Handle --execute (non-interactive)
        if let Some(path) = self.args.execute.clone() {
            return self.execute_plan_file(&path).await;
        }

        // Interactive wizard flow
        self.run_interactive().await
    }

    /// Run interactive wizard
    async fn run_interactive(&mut self) -> Result<()> {
        self.print_banner();

        // Handle --template
        if let Some(template_name) = self.args.template.clone() {
            self.apply_template(&template_name)?;
        }

        // Handle --target (pre-fill)
        if let Some(ref target) = self.args.target {
            self.state.target.target = target.clone();
            self.state.target.target_type = detect_target_type(target);
        }

        // Skip target step if template and target provided
        let start_step = if self.args.quick || (self.args.template.is_some() && self.args.target.is_some()) {
            WizardStep::Modules
        } else {
            WizardStep::Target
        };

        // Run wizard steps
        if start_step == WizardStep::Target {
            step_target(&mut self.state)?;
        }

        if start_step == WizardStep::Target || start_step == WizardStep::Scope {
            step_scope(&mut self.state)?;
        }

        step_modules(&mut self.state)?;
        step_config(&mut self.state)?;

        let execution_mode = step_review(&mut self.state)?;

        // Handle execution mode
        match execution_mode {
            ExecutionMode::ExportOnly => {
                self.export_plan().await?;
            }
            ExecutionMode::Automatic | ExecutionMode::StepByStep => {
                let step_by_step = execution_mode == ExecutionMode::StepByStep;
                self.execute_plan(step_by_step).await?;
            }
        }

        Ok(())
    }

    /// Run in non-interactive mode
    async fn run_non_interactive(&mut self) -> Result<()> {
        if let Some(path) = self.args.execute.clone() {
            self.execute_plan_file(&path).await
        } else {
            bail!("Non-interactive mode requires --execute with a plan file");
        }
    }

    /// Handle --resume flag
    async fn handle_resume(&mut self) -> Result<()> {
        match AttackPlan::load_interrupted()? {
            Some(mut plan) => {
                println!();
                println!("{} Found interrupted plan:", "ℹ".blue());
                println!("  Name:   {}", plan.name);
                println!("  Target: {}", plan.target.target);
                println!("  Progress: {}/{} modules completed",
                    plan.completed_modules(),
                    plan.total_modules()
                );
                println!();

                print!("Resume this plan? [Y/n]: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() != "n" {
                    let executor = PlanExecutor::new(self.registry.clone());
                    let step_by_step = plan.execution_mode == ExecutionMode::StepByStep;
                    let report = executor.execute(&mut plan, step_by_step).await?;

                    if let Some(ref path) = self.args.export {
                        export_results(&report, path)?;
                    }
                } else {
                    AttackPlan::clear_interrupted()?;
                    println!("{} Interrupted plan cleared.", "✓".green());
                }
            }
            None => {
                println!("{} No interrupted plan found.", "ℹ".blue());
            }
        }
        Ok(())
    }

    /// Load and execute a plan file
    async fn load_and_execute(&mut self, path: &str) -> Result<()> {
        println!("{} Loading plan from: {}", "ℹ".blue(), path);

        let mut plan = AttackPlan::load(path)?;

        println!();
        println!("Plan Summary:");
        println!("{}", plan.summary());
        println!();

        print!("Execute this plan? [Y/n]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "n" {
            let executor = PlanExecutor::new(self.registry.clone());
            let step_by_step = plan.execution_mode == ExecutionMode::StepByStep;
            let report = executor.execute(&mut plan, step_by_step).await?;

            if let Some(ref export_path) = self.args.export {
                export_results(&report, export_path)?;
            }
        }

        Ok(())
    }

    /// Execute plan file without interaction
    async fn execute_plan_file(&mut self, path: &str) -> Result<()> {
        println!("{} Executing plan from: {}", "→".cyan(), path);

        let mut plan = AttackPlan::load(path)?;

        // Verify authorization
        if !plan.target.authorized {
            bail!("Plan target is not marked as authorized. Cannot execute.");
        }

        let executor = PlanExecutor::new(self.registry.clone());
        let report = executor.execute(&mut plan, false).await?;

        // Auto-export results
        let results_path = format!("{}_results.json",
            std::path::Path::new(path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("plan")
        );

        export_results(&report, &results_path)?;

        Ok(())
    }

    /// Apply a template
    fn apply_template(&mut self, template_name: &str) -> Result<()> {
        let template_type = TemplateType::from_name(template_name)
            .ok_or_else(|| anyhow::anyhow!("Unknown template: {}", template_name))?;

        let template = AttackTemplate::from_type(template_type);

        println!("{} Using template: {} - {}", "ℹ".blue(), template.name, template.description);

        // Set scope based on template
        self.state.scope.intensity = template.default_intensity;

        // If target already provided, generate modules
        if !self.state.target.target.is_empty() {
            self.state.modules = template.to_selected_modules(
                &self.state.target.target,
                self.state.scope.intensity
            );
        }

        Ok(())
    }

    /// Execute the current plan
    async fn execute_plan(&mut self, step_by_step: bool) -> Result<()> {
        let mut plan = AttackPlan::from_wizard_state(&self.state);

        let executor = PlanExecutor::new(self.registry.clone());
        let report = executor.execute(&mut plan, step_by_step).await?;

        // Export if requested
        if let Some(ref path) = self.args.export {
            export_results(&report, path)?;
        }

        Ok(())
    }

    /// Export plan without executing
    async fn export_plan(&mut self) -> Result<()> {
        let plan = AttackPlan::from_wizard_state(&self.state);

        // Ask for export path if not provided
        let path = if let Some(ref p) = self.args.export {
            p.clone()
        } else {
            print!("Export path [plan.yaml]: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                "plan.yaml".to_string()
            } else {
                input.to_string()
            }
        };

        // Determine format from extension
        if path.ends_with(".json") {
            plan.save_json(&path)?;
        } else {
            plan.save_yaml(&path)?;
        }

        println!();
        println!("{} Plan exported to: {}", "✓".green(), path.cyan());
        println!();
        println!("To execute this plan later, run:");
        println!("  {} {}", "ferox wizard --load".cyan(), path.dimmed());
        println!();
        println!("Or in non-interactive mode:");
        println!("  {} {}", "ferox wizard --execute".cyan(), path.dimmed());

        Ok(())
    }

    /// Print wizard banner
    fn print_banner(&self) {
        println!();
        println!(
            "{}",
            "┌──────────────────────────────────────────────────────────────────────────────┐".cyan()
        );
        println!(
            "{}",
            "│                         🦊 FEROX ATTACK WIZARD                               │".cyan()
        );
        println!(
            "{}",
            "│                                                                              │".cyan()
        );
        println!(
            "{}",
            "│  This wizard will guide you through building an attack plan.                │".cyan()
        );
        println!(
            "{}",
            "│  All actions require explicit confirmation before execution.                │".cyan()
        );
        println!(
            "{}",
            "│                                                                              │".cyan()
        );
        println!(
            "{}",
            "│  Press Ctrl+C at any time to abort.                                         │".cyan()
        );
        println!(
            "{}",
            "└──────────────────────────────────────────────────────────────────────────────┘".cyan()
        );
        println!();
    }
}

/// List available templates
pub fn list_templates() {
    println!();
    println!("{}", "Available Attack Templates:".bold());
    println!();

    for template_type in TemplateType::all() {
        let template = AttackTemplate::from_type(template_type);
        println!("  {} - {}",
            template_type.name().cyan().bold(),
            template.description.dimmed()
        );

        // Show phases
        for phase in &template.phases {
            println!("    Phase {}: {} ({} modules)",
                phase.phase_number,
                phase.name,
                phase.modules.len()
            );
        }
        println!();
    }

    println!("Use: {} {}", "ferox wizard --template".cyan(), "<template-name>".dimmed());
}

/// Detect target type from string
fn detect_target_type(target: &str) -> TargetType {
    if target.parse::<std::net::IpAddr>().is_ok() {
        TargetType::SingleHost
    } else if target.contains('/') && target.chars().all(|c| c.is_numeric() || c == '.' || c == '/') {
        TargetType::IpRange
    } else if std::path::Path::new(target).exists() {
        TargetType::MultipleTargets
    } else {
        TargetType::Domain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_type_detection() {
        assert_eq!(detect_target_type("192.168.1.1"), TargetType::SingleHost);
        assert_eq!(detect_target_type("example.com"), TargetType::Domain);
        assert_eq!(detect_target_type("192.168.1.0/24"), TargetType::IpRange);
    }
}
