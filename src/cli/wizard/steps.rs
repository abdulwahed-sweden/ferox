//! Wizard Steps Implementation
//!
//! Individual step handlers for the attack wizard

use super::types::*;
use super::templates::AttackTemplate;
use anyhow::{Result, bail};
use colored::Colorize;
use std::io::{self, Write};

/// Print step header
pub fn print_step_header(step: WizardStep) {
    println!();
    println!("{}", "━".repeat(80));
    println!(
        " STEP {} OF {}: {}",
        step.number(),
        WizardStep::total(),
        step.name()
    );
    println!("{}", "━".repeat(80));
    println!();
}

/// Print a menu option
fn print_option(num: usize, emoji: &str, label: &str, description: &str) {
    println!("  [{}] {} {} - {}", num, emoji, label.bold(), description.dimmed());
}

/// Read user choice (number)
fn read_choice(prompt: &str, min: usize, max: usize) -> Result<usize> {
    loop {
        print!("{} ", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input.parse::<usize>() {
            Ok(n) if n >= min && n <= max => return Ok(n),
            _ => {
                println!("{} Please enter a number between {} and {}", "⚠".yellow(), min, max);
            }
        }
    }
}

/// Read yes/no choice
fn read_yes_no(prompt: &str, default: bool) -> Result<bool> {
    let default_hint = if default { "[Y/n]" } else { "[y/N]" };
    print!("{} {}: ", prompt, default_hint);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input.is_empty() {
        return Ok(default);
    }

    Ok(input == "y" || input == "yes")
}

/// Read text input
fn read_text(prompt: &str, default: Option<&str>) -> Result<String> {
    if let Some(def) = default {
        print!("{} [{}]: ", prompt, def.dimmed());
    } else {
        print!("{}: ", prompt);
    }
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        if let Some(def) = default {
            return Ok(def.to_string());
        }
    }

    Ok(input.to_string())
}

/// Step 1: Target Definition
pub fn step_target(state: &mut WizardState) -> Result<()> {
    print_step_header(WizardStep::Target);

    println!("? What type of target are you assessing?\n");
    print_option(1, "🖥️", "Single Host", "IP address (e.g., 192.168.1.1)");
    print_option(2, "🌐", "Domain", "Domain name (e.g., example.com)");
    print_option(3, "📡", "IP Range", "CIDR notation (e.g., 192.168.1.0/24)");
    print_option(4, "📋", "Multiple Targets", "From file (one per line)");
    println!();

    let choice = read_choice("→ Enter choice [1-4]:", 1, 4)?;

    state.target.target_type = match choice {
        1 => TargetType::SingleHost,
        2 => TargetType::Domain,
        3 => TargetType::IpRange,
        4 => TargetType::MultipleTargets,
        _ => unreachable!(),
    };

    // Get target value
    println!();
    let target_prompt = match state.target.target_type {
        TargetType::SingleHost => "? Enter the target IP address",
        TargetType::Domain => "? Enter the target domain",
        TargetType::IpRange => "? Enter the CIDR range",
        TargetType::MultipleTargets => "? Enter the path to targets file",
    };

    loop {
        let target = read_text(target_prompt, None)?;
        if target.is_empty() {
            println!("{} Target cannot be empty", "⚠".yellow());
            continue;
        }

        // Basic validation
        let valid = match state.target.target_type {
            TargetType::SingleHost => {
                target.parse::<std::net::IpAddr>().is_ok() || is_valid_hostname(&target)
            }
            TargetType::Domain => is_valid_domain(&target),
            TargetType::IpRange => target.contains('/'),
            TargetType::MultipleTargets => std::path::Path::new(&target).exists(),
        };

        if !valid {
            println!("{} Invalid target format for {:?}", "⚠".yellow(), state.target.target_type);
            continue;
        }

        state.target.target = target;
        break;
    }

    // Authorization confirmation
    println!();
    println!("{}", "⚠ AUTHORIZATION CHECK".yellow().bold());
    println!();
    println!("Is this an {} penetration test?", "AUTHORIZED".bold());
    println!("You must have written permission to scan this target.");
    println!();

    state.target.authorized = read_yes_no("? Confirm authorization", false)?;

    if !state.target.authorized {
        println!();
        println!("{}", "❌ Unauthorized scanning is illegal. Aborting wizard.".red());
        bail!("User did not confirm authorization");
    }

    println!();
    println!("{} Target configured: {} ({})",
        "✓".green(),
        state.target.target.cyan(),
        state.target.target_type
    );

    Ok(())
}

/// Step 2: Attack Scope
pub fn step_scope(state: &mut WizardState) -> Result<()> {
    print_step_header(WizardStep::Scope);

    println!("? What is your assessment scope?\n");
    print_option(1, "🔍", "Reconnaissance Only", "Passive, no direct contact with target");
    print_option(2, "🎯", "Discovery & Enumeration", "Active scanning, service detection");
    print_option(3, "⚡", "Full Assessment", "Recon → Enum → Post-Exploitation");
    print_option(4, "🔧", "Custom", "Select individual phases");
    println!();

    let choice = read_choice("→ Enter choice [1-4]:", 1, 4)?;

    state.scope.level = match choice {
        1 => ScopeLevel::ReconOnly,
        2 => ScopeLevel::Discovery,
        3 => ScopeLevel::FullAssessment,
        4 => ScopeLevel::Custom,
        _ => unreachable!(),
    };

    // Include post-exploitation?
    if state.scope.level == ScopeLevel::FullAssessment {
        println!();
        state.scope.include_post_exploit = read_yes_no(
            "? Include post-exploitation phase (C2, persistence analysis)",
            true
        )?;
    } else if state.scope.level == ScopeLevel::Custom {
        println!();
        state.scope.include_post_exploit = read_yes_no(
            "? Include post-exploitation modules",
            false
        )?;
    }

    // Intensity level
    println!();
    println!("? Select intensity level:\n");
    print_option(1, "🐢", "Stealth", "Slow, minimal footprint, lower detection risk");
    print_option(2, "🦊", "Normal", "Balanced speed and stealth");
    print_option(3, "🚀", "Aggressive", "Fast, higher detection risk");
    println!();

    let intensity_choice = read_choice("→ Enter choice [1-3]:", 1, 3)?;

    state.scope.intensity = match intensity_choice {
        1 => IntensityLevel::Stealth,
        2 => IntensityLevel::Normal,
        3 => IntensityLevel::Aggressive,
        _ => unreachable!(),
    };

    println!();
    println!("{} Scope: {} @ {} intensity",
        "✓".green(),
        state.scope.level.to_string().cyan(),
        state.scope.intensity
    );

    Ok(())
}

/// Step 3: Module Selection
pub fn step_modules(state: &mut WizardState) -> Result<()> {
    print_step_header(WizardStep::Modules);

    // Recommend modules based on target type and scope
    let template = recommend_template(&state.target.target_type, &state.scope.level);

    println!("Based on your target ({}) and scope ({}), I recommend:\n",
        state.target.target_type.to_string().cyan(),
        state.scope.level.to_string().cyan()
    );

    // Convert template to modules
    state.modules = template.to_selected_modules(&state.target.target, state.scope.intensity);

    // Filter by scope
    if !state.scope.include_post_exploit {
        state.modules.retain(|m| m.phase <= 2);
    }

    // Display phases
    let mut current_phase = 0;
    for module in &state.modules {
        if module.phase != current_phase {
            current_phase = module.phase;
            let phase_name = match current_phase {
                1 => "RECONNAISSANCE",
                2 => "ENUMERATION",
                3 => "POST-EXPLOITATION",
                _ => "CUSTOM",
            };
            println!(" {} PHASE {}: {}", "📦".to_string(), current_phase, phase_name.bold());
            println!(" ┌────┬─────────────────────────┬────────────────────────────────┬──────────┐");
            println!(" │ ## │ Module                  │ Purpose                        │ Include? │");
            println!(" ├────┼─────────────────────────┼────────────────────────────────┼──────────┤");
        }

        let status = if module.enabled { "[✓]".green() } else { "[ ]".dimmed() };
        println!(" │ {:2} │ {:<23} │ {:<30} │ {}      │",
            state.modules.iter().position(|m| m.path == module.path).unwrap() + 1,
            truncate(&module.name, 23),
            truncate(&module.path, 30),
            status
        );
    }
    println!(" └────┴─────────────────────────┴────────────────────────────────┴──────────┘");

    println!();
    println!("? Modify selections?\n");
    println!("  [a] Accept all recommendations");
    println!("  [m] Modify (toggle modules on/off)");
    println!("  [c] Clear all and start fresh");
    println!();

    let modify_input = read_text("→ Enter choice [a/m/c]", Some("a"))?;

    match modify_input.to_lowercase().as_str() {
        "a" => {
            // Accept all
        }
        "m" => {
            // Toggle mode
            println!();
            println!("Enter module numbers to toggle (comma-separated), or 'done' to finish:");
            loop {
                let input = read_text("→ Toggle", Some("done"))?;
                if input == "done" {
                    break;
                }

                for part in input.split(',') {
                    if let Ok(idx) = part.trim().parse::<usize>() {
                        if idx > 0 && idx <= state.modules.len() {
                            state.modules[idx - 1].enabled = !state.modules[idx - 1].enabled;
                            let status = if state.modules[idx - 1].enabled { "enabled".green() } else { "disabled".red() };
                            println!("  {} {} {}", "→".dimmed(), state.modules[idx - 1].name, status);
                        }
                    }
                }
            }
        }
        "c" => {
            // Clear all
            for module in &mut state.modules {
                module.enabled = false;
            }
            println!("{} All modules cleared. Use 'm' to enable specific modules.", "ℹ".blue());
        }
        _ => {}
    }

    let enabled_count = state.modules.iter().filter(|m| m.enabled).count();
    let phase_count = state.modules.iter()
        .filter(|m| m.enabled)
        .map(|m| m.phase)
        .collect::<std::collections::HashSet<_>>()
        .len();

    println!();
    println!("{} Selected {} modules across {} phase(s)",
        "✓".green(),
        enabled_count.to_string().cyan(),
        phase_count
    );

    if enabled_count == 0 {
        bail!("No modules selected. Cannot proceed with empty plan.");
    }

    Ok(())
}

/// Step 4: Configuration
pub fn step_config(state: &mut WizardState) -> Result<()> {
    print_step_header(WizardStep::Config);

    println!("Configure module parameters (press Enter to accept defaults):\n");

    for module in &mut state.modules {
        if !module.enabled {
            continue;
        }

        println!(" {} {}", "📦".to_string(), module.name.bold());

        // Show key options for configuration
        let configurable_opts = get_configurable_options(&module.path);

        for (opt_name, default_val, _description) in configurable_opts {
            let current = module.options.get(&opt_name).cloned().unwrap_or_else(|| default_val.clone());
            let prompt = format!("    {} [{}]", opt_name, current.dimmed());
            let new_val = read_text(&prompt, Some(&current))?;

            if !new_val.is_empty() {
                module.options.insert(opt_name, new_val);
            }
        }
        println!();
    }

    println!("{} All modules configured", "✓".green());

    Ok(())
}

/// Step 5: Review and Execute
pub fn step_review(state: &mut WizardState) -> Result<ExecutionMode> {
    print_step_header(WizardStep::Review);

    // Build summary
    let enabled_modules: Vec<_> = state.modules.iter().filter(|m| m.enabled).collect();
    let _phase_count = enabled_modules.iter()
        .map(|m| m.phase)
        .collect::<std::collections::HashSet<_>>()
        .len();

    // Estimate time
    let est_time = enabled_modules.len() * 5; // 5 seconds per module rough estimate

    println!("┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ {:^76} │", "📋 ATTACK PLAN SUMMARY".bold());
    println!("├──────────────────────────────────────────────────────────────────────────────┤");
    println!("│ Target:      {:<63} │", state.target.target.cyan());
    println!("│ Type:        {:<63} │", state.target.target_type.to_string());
    println!("│ Scope:       {:<63} │", state.scope.level.to_string());
    println!("│ Intensity:   {:<63} │", state.scope.intensity.to_string());
    println!("│ Modules:     {:<63} │", enabled_modules.len());
    println!("│ Est. Time:   {:<63} │", format!("~{}-{} seconds", est_time, est_time * 2));
    println!("├──────────────────────────────────────────────────────────────────────────────┤");

    // Show phases
    let mut current_phase = 0;
    for module in &enabled_modules {
        if module.phase != current_phase {
            current_phase = module.phase;
            let phase_name = match current_phase {
                1 => "RECONNAISSANCE",
                2 => "ENUMERATION",
                3 => "POST-EXPLOITATION",
                _ => "CUSTOM",
            };
            println!("│                                                                              │");
            println!("│  PHASE {}: {:<64} │", current_phase, phase_name.bold());
        }

        let is_last_in_phase = enabled_modules.iter()
            .filter(|m| m.phase == current_phase)
            .last()
            .map(|m| m.path == module.path)
            .unwrap_or(false);

        let prefix = if is_last_in_phase {
            "└─"
        } else {
            "├─"
        };

        println!("│  {} [{:2}] {:<64} │",
            prefix.dimmed(),
            enabled_modules.iter().position(|m| m.path == module.path).unwrap() + 1,
            module.name
        );
    }

    println!("│                                                                              │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!();
    println!("? How would you like to proceed?\n");
    println!("  [r] Run all phases automatically");
    println!("  [s] Step-by-step (confirm each phase)");
    println!("  [e] Export plan to file (execute later)");
    println!("  [c] Cancel");
    println!();

    loop {
        let choice = read_text("→ Enter choice [r/s/e/c]", Some("s"))?;

        match choice.to_lowercase().as_str() {
            "r" => {
                state.execution_mode = ExecutionMode::Automatic;
                return Ok(ExecutionMode::Automatic);
            }
            "s" => {
                state.execution_mode = ExecutionMode::StepByStep;
                return Ok(ExecutionMode::StepByStep);
            }
            "e" => {
                state.execution_mode = ExecutionMode::ExportOnly;
                return Ok(ExecutionMode::ExportOnly);
            }
            "c" => {
                bail!("User cancelled execution");
            }
            _ => {
                println!("{} Invalid choice. Please enter r, s, e, or c.", "⚠".yellow());
            }
        }
    }
}

// Helper functions

fn is_valid_hostname(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-')
}

fn is_valid_domain(s: &str) -> bool {
    s.contains('.') && is_valid_hostname(s)
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

fn recommend_template(target_type: &TargetType, scope: &ScopeLevel) -> AttackTemplate {
    match (target_type, scope) {
        (TargetType::Domain, ScopeLevel::ReconOnly) => AttackTemplate::domain(),
        (TargetType::Domain, _) => AttackTemplate::web_app(),
        (TargetType::SingleHost, ScopeLevel::ReconOnly) => AttackTemplate::network(),
        (TargetType::SingleHost, ScopeLevel::Discovery) => AttackTemplate::quick_scan(),
        (TargetType::SingleHost, _) => AttackTemplate::network(),
        (TargetType::IpRange, _) => AttackTemplate::network(),
        (TargetType::MultipleTargets, _) => AttackTemplate::quick_scan(),
    }
}

fn get_configurable_options(module_path: &str) -> Vec<(String, String, String)> {
    // Return key configurable options for each module type
    match module_path {
        "scanner/port_scanner" => vec![
            ("PORTS".to_string(), "1-1000".to_string(), "Port range to scan".to_string()),
        ],
        "scanner/http_scanner" => vec![
            ("PATHS".to_string(), "/".to_string(), "Paths to probe".to_string()),
        ],
        "recon/subdomain_enum" => vec![
            ("WORDLIST".to_string(), "./wordlist.txt".to_string(), "Subdomain wordlist".to_string()),
        ],
        "recon/dns_enum" => vec![
            ("RECORD_TYPES".to_string(), "A,AAAA,MX,NS,TXT".to_string(), "DNS record types".to_string()),
        ],
        _ => vec![],
    }
}
