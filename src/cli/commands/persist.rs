//! Persistence CLI Commands
//!
//! Command-line interface for the Persistence Engine.

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};

use crate::cli::theme::Theme;
use crate::core::module::Platform;
use crate::modules::post::persistence::{PersistenceEngine, StealthLevel};

#[derive(Subcommand, Debug, Clone)]
pub enum PersistCommands {
    /// Auto-select and install persistence (safe mode)
    Auto(PersistAutoArgs),
    /// List available persistence methods
    List(PersistListArgs),
    /// Show details for a specific method
    Describe(PersistDescribeArgs),
    /// Verify installed persistence mechanisms
    Verify,
    /// Remove all installed persistence
    Remove,
}

#[derive(Args, Debug, Clone)]
pub struct PersistAutoArgs {
    /// Target platform (auto, windows, linux, macos)
    #[arg(short = 'P', long, default_value = "auto")]
    pub platform: PlatformArg,

    /// Payload path to persist
    #[arg(short = 'p', long, default_value = "/path/to/payload")]
    pub payload: String,

    /// Persistence name/identifier
    #[arg(short = 'n', long, default_value = "ferox_persist")]
    pub name: String,

    /// Number of redundant persistence mechanisms
    #[arg(short = 'r', long, default_value = "2")]
    pub redundancy: usize,

    /// Assume admin/root privileges
    #[arg(long)]
    pub admin: bool,

    /// Minimum stealth level (verylow, low, medium, high, veryhigh)
    #[arg(short = 's', long, default_value = "low")]
    pub stealth: StealthArg,
}

#[derive(Args, Debug, Clone)]
pub struct PersistListArgs {
    /// Filter by platform
    #[arg(short, long)]
    pub platform: Option<PlatformArg>,

    /// Show detailed information
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct PersistDescribeArgs {
    /// Method name
    pub method: String,
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum PlatformArg {
    #[default]
    Auto,
    Windows,
    Linux,
    Macos,
}

impl From<PlatformArg> for Platform {
    fn from(arg: PlatformArg) -> Self {
        match arg {
            PlatformArg::Auto => Platform::Any,
            PlatformArg::Windows => Platform::Windows,
            PlatformArg::Linux => Platform::Linux,
            PlatformArg::Macos => Platform::MacOS,
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum StealthArg {
    Verylow,
    #[default]
    Low,
    Medium,
    High,
    Veryhigh,
}

impl From<StealthArg> for StealthLevel {
    fn from(arg: StealthArg) -> Self {
        match arg {
            StealthArg::Verylow => StealthLevel::VeryLow,
            StealthArg::Low => StealthLevel::Low,
            StealthArg::Medium => StealthLevel::Medium,
            StealthArg::High => StealthLevel::High,
            StealthArg::Veryhigh => StealthLevel::VeryHigh,
        }
    }
}

pub struct PersistCommandHandler {
    engine: PersistenceEngine,
}

impl PersistCommandHandler {
    pub fn new() -> Self {
        Self {
            engine: PersistenceEngine::new(),
        }
    }

    pub fn describe() -> &'static str {
        "Persistence engine commands"
    }

    pub fn print_usage() {
        Theme::section("Persist CLI");
        Theme::command_help(
            "ferox persist auto --platform windows",
            "Auto-install persistence",
        );
        Theme::command_help("ferox persist list", "List persistence methods");
        Theme::command_help(
            "ferox persist describe <method>",
            "Show method details",
        );
        Theme::command_help("ferox persist verify", "Verify installed persistence");
        Theme::command_help("ferox persist remove", "Remove all persistence");
    }

    pub fn run(&self, command: PersistCommands) -> Result<()> {
        match command {
            PersistCommands::Auto(args) => self.auto_persist(args),
            PersistCommands::List(args) => self.list_methods(args),
            PersistCommands::Describe(args) => self.describe_method(args),
            PersistCommands::Verify => self.verify_persistence(),
            PersistCommands::Remove => self.remove_persistence(),
        }
    }

    fn auto_persist(&self, args: PersistAutoArgs) -> Result<()> {
        Theme::section("Persistence Engine - Auto Mode (SAFE MODE)");

        let platform: Platform = args.platform.into();
        let stealth: StealthLevel = args.stealth.into();

        Theme::info(&format!("Platform: {:?}", platform));
        Theme::info(&format!("Payload: {}", args.payload));
        Theme::info(&format!("Persistence Name: {}", args.name));
        Theme::info(&format!("Redundancy: {}", args.redundancy));
        Theme::info(&format!("Admin Privileges: {}", args.admin));
        Theme::info(&format!("Min Stealth: {}", stealth.as_str()));

        println!();
        Theme::section("Selected Methods");

        let methods = self.engine.auto_select(&platform, args.admin, stealth);

        if methods.is_empty() {
            Theme::warning("No suitable persistence methods found for this configuration");
            return Ok(());
        }

        for (i, method) in methods.iter().take(args.redundancy).enumerate() {
            println!(
                "{}. {} [{}] - {} (MITRE: {})",
                i + 1,
                method.name(),
                method.stealth_level().as_str(),
                if method.requires_admin() {
                    "Admin Required"
                } else {
                    "User-Level"
                },
                method.mitre_id()
            );
            println!("   {}", method.description());

            // Show reference implementation
            println!();
            Theme::info("Reference Implementation:");
            let reference = method.generate_reference(&args.payload, &args.name);
            for line in reference.lines().take(15) {
                println!("   {}", line);
            }
            println!();
        }

        Theme::success(&format!(
            "[SAFE MODE] Would install {} persistence mechanism(s)",
            methods.len().min(args.redundancy)
        ));

        Ok(())
    }

    fn list_methods(&self, args: PersistListArgs) -> Result<()> {
        Theme::section("Persistence Methods");

        let platform: Platform = args.platform.map(|p| p.into()).unwrap_or(Platform::Any);
        let methods = self.engine.methods_for_platform(&platform);

        println!(
            "{:<30} {:<10} {:<12} {:<10} {}",
            "NAME", "PLATFORM", "STEALTH", "ADMIN", "MITRE ID"
        );
        println!("{}", "-".repeat(80));

        for method in methods {
            println!(
                "{:<30} {:<10} {:<12} {:<10} {}",
                method.name(),
                format!("{:?}", method.platform()),
                method.stealth_level().as_str(),
                if method.requires_admin() { "Yes" } else { "No" },
                method.mitre_id()
            );

            if args.verbose {
                println!("   └─ {}", method.description());
            }
        }

        Ok(())
    }

    fn describe_method(&self, args: PersistDescribeArgs) -> Result<()> {
        let methods = self.engine.methods_for_platform(&Platform::Any);

        if let Some(method) = methods.iter().find(|m| m.name() == args.method) {
            Theme::section(&format!("Persistence Method: {}", method.name()));

            println!("Platform:    {:?}", method.platform());
            println!("Stealth:     {}", method.stealth_level().as_str());
            println!("Admin:       {}", method.requires_admin());
            println!("MITRE ID:    {}", method.mitre_id());
            println!("Description: {}", method.description());
            println!();

            Theme::info("Reference Implementation:");
            let reference = method.generate_reference("/path/to/payload", "ferox_persist");
            println!("{}", reference);
        } else {
            Theme::error(&format!("Unknown method: {}", args.method));
            Theme::info("Use 'ferox persist list' to see available methods");
        }

        Ok(())
    }

    fn verify_persistence(&self) -> Result<()> {
        Theme::section("Persistence Verification (SAFE MODE)");
        Theme::info("[SAFE MODE] No persistence currently installed");
        Theme::info("In production mode, this would verify all installed persistence mechanisms");
        Ok(())
    }

    fn remove_persistence(&self) -> Result<()> {
        Theme::section("Persistence Removal (SAFE MODE)");
        Theme::info("[SAFE MODE] No persistence to remove");
        Theme::info("In production mode, this would remove all installed persistence mechanisms");
        Ok(())
    }
}

impl Default for PersistCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}
