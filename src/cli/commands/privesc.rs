//! Privilege Escalation CLI Commands
//!
//! Command-line interface for the Privilege Escalation Engine.

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};

use crate::cli::theme::Theme;
use crate::core::module::Platform;
use crate::modules::post::privilege_escalation::{PrivEscEngine, Severity, VectorCategory};

#[derive(Subcommand, Debug, Clone)]
pub enum PrivEscCommands {
    /// Auto-enumerate and exploit (safe mode)
    Auto(PrivEscAutoArgs),
    /// List available enumerators
    List(PrivEscListArgs),
    /// Enumerate privilege escalation vectors
    Enumerate(PrivEscEnumerateArgs),
    /// Show details for a specific technique
    Describe(PrivEscDescribeArgs),
}

#[derive(Args, Debug, Clone)]
pub struct PrivEscAutoArgs {
    /// Target platform (auto, windows, linux)
    #[arg(short, long, default_value = "auto")]
    pub platform: PrivEscPlatformArg,

    /// Command to execute with elevated privileges
    #[arg(short, long, default_value = "cmd.exe")]
    pub command: String,

    /// Minimum severity to consider (low, medium, high, critical)
    #[arg(short, long, default_value = "medium")]
    pub severity: SeverityArg,
}

#[derive(Args, Debug, Clone)]
pub struct PrivEscListArgs {
    /// Filter by platform
    #[arg(short, long)]
    pub platform: Option<PrivEscPlatformArg>,

    /// Show detailed information
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct PrivEscEnumerateArgs {
    /// Target platform
    #[arg(short, long, default_value = "windows")]
    pub platform: PrivEscPlatformArg,

    /// Category filter (uac, token, service, sudo, suid, kernel, capability)
    #[arg(short, long)]
    pub category: Option<CategoryArg>,
}

#[derive(Args, Debug, Clone)]
pub struct PrivEscDescribeArgs {
    /// Enumerator or technique name
    pub name: String,
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum PrivEscPlatformArg {
    #[default]
    Auto,
    Windows,
    Linux,
}

impl From<PrivEscPlatformArg> for Platform {
    fn from(arg: PrivEscPlatformArg) -> Self {
        match arg {
            PrivEscPlatformArg::Auto => Platform::Any,
            PrivEscPlatformArg::Windows => Platform::Windows,
            PrivEscPlatformArg::Linux => Platform::Linux,
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum SeverityArg {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

impl From<SeverityArg> for Severity {
    fn from(arg: SeverityArg) -> Self {
        match arg {
            SeverityArg::Low => Severity::Low,
            SeverityArg::Medium => Severity::Medium,
            SeverityArg::High => Severity::High,
            SeverityArg::Critical => Severity::Critical,
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum CategoryArg {
    Uac,
    Token,
    Service,
    Sudo,
    Suid,
    Kernel,
    Capability,
}

impl From<CategoryArg> for VectorCategory {
    fn from(arg: CategoryArg) -> Self {
        match arg {
            CategoryArg::Uac => VectorCategory::UacBypass,
            CategoryArg::Token => VectorCategory::TokenManipulation,
            CategoryArg::Service => VectorCategory::ServiceExploit,
            CategoryArg::Sudo | CategoryArg::Suid => VectorCategory::SudoSuid,
            CategoryArg::Kernel => VectorCategory::KernelExploit,
            CategoryArg::Capability => VectorCategory::CapabilityAbuse,
        }
    }
}

pub struct PrivEscCommandHandler {
    engine: PrivEscEngine,
}

impl PrivEscCommandHandler {
    pub fn new() -> Self {
        Self {
            engine: PrivEscEngine::new(),
        }
    }

    pub fn describe() -> &'static str {
        "Privilege escalation engine commands"
    }

    pub fn print_usage() {
        Theme::section("PrivEsc CLI");
        Theme::command_help(
            "ferox privesc auto --platform windows",
            "Auto-enumerate and exploit",
        );
        Theme::command_help("ferox privesc list", "List enumerators");
        Theme::command_help(
            "ferox privesc enumerate --platform linux",
            "Enumerate vectors",
        );
        Theme::command_help(
            "ferox privesc describe <technique>",
            "Show technique details",
        );
    }

    pub fn run(&self, command: PrivEscCommands) -> Result<()> {
        match command {
            PrivEscCommands::Auto(args) => self.auto_privesc(args),
            PrivEscCommands::List(args) => self.list_enumerators(args),
            PrivEscCommands::Enumerate(args) => self.enumerate_vectors(args),
            PrivEscCommands::Describe(args) => self.describe_technique(args),
        }
    }

    fn auto_privesc(&self, args: PrivEscAutoArgs) -> Result<()> {
        Theme::section("Privilege Escalation Engine - Auto Mode (SAFE MODE)");

        let platform: Platform = args.platform.into();
        let min_severity: Severity = args.severity.into();

        Theme::info(&format!("Platform: {:?}", platform));
        Theme::info(&format!("Command: {}", args.command));
        Theme::info(&format!("Min Severity: {}", min_severity.as_str()));

        println!();
        Theme::section("Available Enumerators");

        let enumerators = self.engine.enumerators_for_platform(&platform);

        for enumerator in &enumerators {
            println!(
                "  {} [{}] - {}",
                enumerator.name(),
                enumerator.category().as_str(),
                format!("{:?}", enumerator.platform())
            );
        }

        println!();
        Theme::section("Sample Vectors (SAFE MODE)");
        Theme::info("In safe mode, showing reference implementations only:");

        // Show reference for each enumerator
        for enumerator in enumerators.iter().take(3) {
            println!();
            Theme::info(&format!("=== {} ===", enumerator.name()));
            let reference = enumerator.generate_reference();
            for line in reference.lines().take(20) {
                println!("  {}", line);
            }
        }

        println!();
        Theme::success("[SAFE MODE] Would enumerate and exploit privilege escalation vectors");
        Theme::info("Use production mode (with authorization) for actual exploitation");

        Ok(())
    }

    fn list_enumerators(&self, args: PrivEscListArgs) -> Result<()> {
        Theme::section("Privilege Escalation Enumerators");

        let platform: Platform = args.platform.map(|p| p.into()).unwrap_or(Platform::Any);
        let enumerators = self.engine.enumerators_for_platform(&platform);

        println!(
            "{:<25} {:<12} {:<20}",
            "NAME", "PLATFORM", "CATEGORY"
        );
        println!("{}", "-".repeat(60));

        for enumerator in enumerators {
            println!(
                "{:<25} {:<12} {:<20}",
                enumerator.name(),
                format!("{:?}", enumerator.platform()),
                enumerator.category().as_str()
            );

            if args.verbose {
                let reference = enumerator.generate_reference();
                // Show first few lines of description
                for line in reference.lines().take(5) {
                    if !line.trim().is_empty() {
                        println!("   {}", line);
                    }
                }
                println!();
            }
        }

        Ok(())
    }

    fn enumerate_vectors(&self, args: PrivEscEnumerateArgs) -> Result<()> {
        Theme::section("Vector Enumeration (SAFE MODE)");

        let platform: Platform = args.platform.into();
        let enumerators = self.engine.enumerators_for_platform(&platform);

        // Filter by category if specified
        let filtered: Vec<_> = if let Some(cat) = args.category {
            let target_category: VectorCategory = cat.into();
            enumerators
                .into_iter()
                .filter(|e| e.category() == target_category)
                .collect()
        } else {
            enumerators
        };

        if filtered.is_empty() {
            Theme::warning("No enumerators found for the specified criteria");
            return Ok(());
        }

        for enumerator in filtered {
            Theme::section(&format!("{} ({})", enumerator.name(), enumerator.category().as_str()));

            println!("Platform: {:?}", enumerator.platform());
            println!();

            // Show reference implementation
            let reference = enumerator.generate_reference();
            println!("{}", reference);
        }

        Theme::success("[SAFE MODE] Would enumerate actual vectors in production mode");

        Ok(())
    }

    fn describe_technique(&self, args: PrivEscDescribeArgs) -> Result<()> {
        let enumerators = self.engine.enumerators_for_platform(&Platform::Any);

        if let Some(enumerator) = enumerators.iter().find(|e| e.name() == args.name) {
            Theme::section(&format!("Enumerator: {}", enumerator.name()));

            println!("Platform:  {:?}", enumerator.platform());
            println!("Category:  {}", enumerator.category().as_str());
            println!();

            Theme::info("Reference Documentation:");
            let reference = enumerator.generate_reference();
            println!("{}", reference);
        } else {
            Theme::error(&format!("Unknown technique: {}", args.name));
            Theme::info("Use 'ferox privesc list' to see available enumerators");

            // Show available techniques
            println!();
            Theme::info("Available enumerators:");
            for e in enumerators {
                println!("  - {}", e.name());
            }
        }

        Ok(())
    }
}

impl Default for PrivEscCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}
