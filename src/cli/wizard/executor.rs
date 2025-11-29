//! Plan Executor
//!
//! Executes attack plans with real-time progress feedback

use super::plan::{AttackPlan, ModuleConfig};
use super::types::*;
use crate::cli::progress::{ProgressBar, ProgressStyle, PhaseProgress};
use crate::core::module::ModuleRegistry;
use anyhow::Result;
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

/// Plan executor for running attack plans
pub struct PlanExecutor {
    registry: Arc<Mutex<ModuleRegistry>>,
}

impl PlanExecutor {
    pub fn new(registry: Arc<Mutex<ModuleRegistry>>) -> Self {
        Self { registry }
    }

    /// Execute a complete attack plan
    pub async fn execute(&self, plan: &mut AttackPlan, step_by_step: bool) -> Result<ExecutionReport> {
        let start_time = Instant::now();
        let started_at = chrono::Utc::now().to_rfc3339();
        let mut phase_results = Vec::new();
        let mut total_success = 0;
        let mut total_failed = 0;

        println!();
        println!("{}",
            "╔══════════════════════════════════════════════════════════════════════════════╗".cyan()
        );
        println!("{}",
            format!("║ {:^76} ║", "🦊 FEROX ATTACK EXECUTION").cyan()
        );
        println!("{}",
            "╚══════════════════════════════════════════════════════════════════════════════╝".cyan()
        );
        println!();
        println!("  Target:  {}", plan.target.target.bold());
        println!("  Plan:    {}", plan.name);
        println!("  Phases:  {}", plan.phases.len());
        println!("  Modules: {}", plan.total_modules());
        println!();

        let target = plan.target.target.clone();
        let num_phases = plan.phases.len();

        // Execute each phase by index to avoid borrow issues
        for phase_idx in 0..num_phases {
            // Get phase info
            let (phase_completed, phase_number, phase_name, enabled_modules) = {
                let phase = &plan.phases[phase_idx];
                let enabled: Vec<ModuleConfig> = phase.modules.iter()
                    .filter(|m| m.enabled)
                    .cloned()
                    .collect();
                (phase.completed, phase.number, phase.name.clone(), enabled)
            };

            // Skip completed phases (for resume)
            if phase_completed {
                println!(" {} Phase {} already completed, skipping...", "→".dimmed(), phase_number);
                continue;
            }

            // Step-by-step confirmation
            if step_by_step && phase_number > 1 {
                println!();
                if !self.confirm_phase_continuation(phase_number, &phase_name)? {
                    println!("{} Execution stopped by user", "⚠".yellow());
                    plan.save_interrupted()?;
                    break;
                }
            }

            // Execute phase
            let phase_result = self.execute_phase_modules(
                &target,
                phase_number,
                &phase_name,
                num_phases,
                &enabled_modules,
            ).await?;

            // Store results back in plan
            for module_result in &phase_result.modules {
                plan.store_module_result(phase_number, &module_result.module_path, module_result.clone());
            }

            total_success += phase_result.success_count;
            total_failed += phase_result.total_count - phase_result.success_count;

            // Mark phase completed
            plan.phases[phase_idx].completed = true;
            phase_results.push(phase_result);

            // Save progress after each phase
            plan.modified_at = chrono::Utc::now().to_rfc3339();
        }

        let completed_at = chrono::Utc::now().to_rfc3339();
        let total_duration = start_time.elapsed().as_millis() as u64;

        // Clear interrupted status if complete
        if plan.phases.iter().all(|p| p.completed) {
            plan.interrupted = false;
            AttackPlan::clear_interrupted()?;
        }

        // Print final summary
        self.print_summary(plan, &phase_results, total_duration);

        Ok(ExecutionReport {
            plan_name: plan.name.clone(),
            target: plan.target.target.clone(),
            phases: phase_results,
            total_modules: plan.total_modules(),
            successful_modules: total_success,
            failed_modules: total_failed,
            total_duration_ms: total_duration,
            started_at,
            completed_at,
        })
    }

    /// Execute modules for a single phase
    async fn execute_phase_modules(
        &self,
        target: &str,
        phase_number: usize,
        phase_name: &str,
        total_phases: usize,
        enabled_modules: &[ModuleConfig],
    ) -> Result<PhaseExecutionResult> {
        let phase_start = Instant::now();
        let mut module_results = Vec::new();
        let total_modules = enabled_modules.len();

        // Print phase header
        let mut phase_progress = PhaseProgress::new(phase_name, phase_number, total_phases);
        for module in enabled_modules {
            phase_progress.add_module(&module.path, &module.name);
        }
        phase_progress.print_header();

        let mut success_count = 0;

        for (idx, module_config) in enabled_modules.iter().enumerate() {
            phase_progress.module_start(idx, &module_config.name);

            // Execute module
            let result = self.execute_module(module_config, target).await;

            match &result {
                Ok(exec_result) => {
                    if exec_result.success {
                        success_count += 1;
                        phase_progress.module_complete(true, &exec_result.message);
                    } else {
                        phase_progress.module_complete(false, &exec_result.message);
                    }

                    // Store discoveries
                    if !exec_result.discoveries.is_empty() {
                        println!();
                        for discovery in &exec_result.discoveries {
                            println!("   {} {}", "└─".dimmed(), discovery.green());
                        }
                        println!();
                    }

                    module_results.push(exec_result.clone());
                }
                Err(e) => {
                    let error_result = ModuleExecutionResult {
                        module_path: module_config.path.clone(),
                        module_name: module_config.name.clone(),
                        success: false,
                        message: e.to_string(),
                        data: HashMap::new(),
                        duration_ms: 0,
                        discoveries: Vec::new(),
                    };
                    phase_progress.module_complete(false, &e.to_string());
                    module_results.push(error_result);
                }
            }
        }

        phase_progress.phase_complete(success_count, total_modules);

        Ok(PhaseExecutionResult {
            phase_name: phase_name.to_string(),
            phase_number,
            modules: module_results,
            success_count,
            total_count: total_modules,
            duration_ms: phase_start.elapsed().as_millis() as u64,
        })
    }

    /// Execute a single module
    async fn execute_module(&self, config: &ModuleConfig, _target: &str) -> Result<ModuleExecutionResult> {
        let start_time = Instant::now();
        let mut discoveries = Vec::new();

        // Get module from registry
        let mut registry = self.registry.lock().await;
        let module = registry.get_mut(&config.path)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", config.path))?;

        // Apply options
        for (key, value) in &config.options {
            module.set_option(key, value)?;
        }

        // Progress bar for the module
        let mut progress = ProgressBar::new(100)
            .with_style(ProgressStyle::default().with_color("cyan"));

        progress.set_message(&format!("Running {}...", config.name));
        progress.set(10);

        // Run the module
        let result = module.run().await;
        progress.set(100);
        progress.clear();

        let duration_ms = start_time.elapsed().as_millis() as u64;

        match result {
            Ok(module_result) => {
                // Extract discoveries from result data
                if let Some(ports) = module_result.data.get("open_ports") {
                    if let Some(arr) = ports.as_array() {
                        for port in arr {
                            discoveries.push(format!("Port {} open", port));
                        }
                    }
                }

                if let Some(subdomains) = module_result.data.get("subdomains") {
                    if let Some(arr) = subdomains.as_array() {
                        for sub in arr.iter().take(5) {
                            if let Some(name) = sub.get("subdomain").and_then(|s| s.as_str()) {
                                discoveries.push(format!("Found: {}", name));
                            }
                        }
                    }
                }

                if let Some(records) = module_result.data.get("dns_records") {
                    if let Some(a_records) = records.get("A").and_then(|r| r.as_array()) {
                        discoveries.push(format!("A records: {}", a_records.len()));
                    }
                }

                Ok(ModuleExecutionResult {
                    module_path: config.path.clone(),
                    module_name: config.name.clone(),
                    success: module_result.success,
                    message: module_result.message,
                    data: module_result.data,
                    duration_ms,
                    discoveries,
                })
            }
            Err(e) => {
                Ok(ModuleExecutionResult {
                    module_path: config.path.clone(),
                    module_name: config.name.clone(),
                    success: false,
                    message: e.to_string(),
                    data: HashMap::new(),
                    duration_ms,
                    discoveries: Vec::new(),
                })
            }
        }
    }

    /// Ask user to confirm phase continuation
    fn confirm_phase_continuation(&self, phase_number: usize, phase_name: &str) -> Result<bool> {
        println!();
        println!("? Continue to Phase {} ({})?\n", phase_number, phase_name);
        println!("  [y] Yes, continue");
        println!("  [n] No, stop here");
        println!("  [r] Review previous results");
        println!();

        loop {
            print!("→ Enter choice [y/n/r]: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return Ok(true),
                "n" | "no" => return Ok(false),
                "r" => {
                    println!("\n{}", "Previous phase results are stored in the plan.".dimmed());
                    println!("{}", "Use 'export' after execution to view full details.".dimmed());
                    println!();
                }
                _ => {
                    println!("{} Invalid choice. Please enter y, n, or r.", "⚠".yellow());
                }
            }
        }
    }

    /// Print final execution summary
    fn print_summary(&self, plan: &AttackPlan, results: &[PhaseExecutionResult], duration_ms: u64) {
        println!();
        println!("{}",
            "╔══════════════════════════════════════════════════════════════════════════════╗".green()
        );
        println!("{}",
            format!("║ {:^76} ║", "📊 EXECUTION COMPLETE").green()
        );
        println!("{}",
            "╠══════════════════════════════════════════════════════════════════════════════╣".green()
        );

        let total_success: usize = results.iter().map(|r| r.success_count).sum();
        let total_modules: usize = results.iter().map(|r| r.total_count).sum();
        let total_failed = total_modules - total_success;

        println!("{}",
            format!("║  Target:     {:<63} ║", plan.target.target).green()
        );
        println!("{}",
            format!("║  Duration:   {:<63} ║", format!("{:.2}s", duration_ms as f64 / 1000.0)).green()
        );
        println!("{}",
            format!("║  Modules:    {:<63} ║", format!("{} total", total_modules)).green()
        );
        println!("{}",
            format!("║  Success:    {:<63} ║", format!("{} ({}%)", total_success, total_success * 100 / total_modules.max(1))).green()
        );
        if total_failed > 0 {
            println!("{}",
                format!("║  Failed:     {:<63} ║", total_failed).yellow()
            );
        }

        println!("{}",
            "╠══════════════════════════════════════════════════════════════════════════════╣".green()
        );

        // Phase breakdown
        for result in results {
            let status = if result.success_count == result.total_count {
                "✓".green()
            } else if result.success_count > 0 {
                "⚠".yellow()
            } else {
                "✗".red()
            };

            println!("{}",
                format!("║  {} Phase {}: {} ({}/{} modules)                                        ║",
                    status,
                    result.phase_number,
                    result.phase_name,
                    result.success_count,
                    result.total_count
                ).trim_end()
            );
        }

        println!("{}",
            "╚══════════════════════════════════════════════════════════════════════════════╝".green()
        );
        println!();

        // Tips
        println!("{} Use '{}' to save full results to a file",
            "ℹ".blue(),
            "wizard --export results.json".cyan()
        );
    }
}

/// Export execution results
pub fn export_results(report: &ExecutionReport, path: &str) -> Result<()> {
    let extension = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("json");

    match extension {
        "yaml" | "yml" => {
            let yaml = serde_yaml::to_string(report)?;
            std::fs::write(path, yaml)?;
        }
        _ => {
            let json = serde_json::to_string_pretty(report)?;
            std::fs::write(path, json)?;
        }
    }

    println!("{} Results exported to: {}", "✓".green(), path.cyan());
    Ok(())
}
