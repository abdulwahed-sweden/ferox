//! Credential Harvesting CLI Commands
//!
//! Command-line interface for the Credential Harvesting Engine.

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};

use crate::cli::theme::Theme;
use crate::core::module::Platform;
use crate::modules::post::credential_harvester::{CredentialHarvestEngine, SourceCategory};

#[derive(Subcommand, Debug, Clone)]
pub enum CredsCommands {
    /// Harvest credentials from all sources (safe mode)
    Harvest(CredsHarvestArgs),
    /// List available harvesters
    List(CredsListArgs),
    /// Show details for a specific harvester
    Describe(CredsDescribeArgs),
    /// Show harvested credentials summary
    Show,
}

#[derive(Args, Debug, Clone)]
pub struct CredsHarvestArgs {
    /// Target platform (auto, windows, linux, macos)
    #[arg(short = 'P', long, default_value = "auto")]
    pub platform: CredsPlatformArg,

    /// Category filter (all, os, browser, cloud, application, memory)
    #[arg(short, long, default_value = "all")]
    pub category: CategoryArg,

    /// Redact sensitive output
    #[arg(short, long, default_value = "true")]
    pub redact: bool,
}

#[derive(Args, Debug, Clone)]
pub struct CredsListArgs {
    /// Filter by platform
    #[arg(short = 'P', long)]
    pub platform: Option<CredsPlatformArg>,

    /// Filter by category
    #[arg(short, long)]
    pub category: Option<CategoryArg>,

    /// Show detailed information
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct CredsDescribeArgs {
    /// Harvester name
    pub harvester: String,
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum CredsPlatformArg {
    #[default]
    Auto,
    Windows,
    Linux,
    Macos,
}

impl From<CredsPlatformArg> for Platform {
    fn from(arg: CredsPlatformArg) -> Self {
        match arg {
            CredsPlatformArg::Auto => Platform::Any,
            CredsPlatformArg::Windows => Platform::Windows,
            CredsPlatformArg::Linux => Platform::Linux,
            CredsPlatformArg::Macos => Platform::MacOS,
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum CategoryArg {
    #[default]
    All,
    Os,
    Browser,
    Cloud,
    Application,
    Memory,
    Filesystem,
}

impl CategoryArg {
    fn matches(&self, category: SourceCategory) -> bool {
        match self {
            CategoryArg::All => true,
            CategoryArg::Os => category == SourceCategory::OsCredentialStore,
            CategoryArg::Browser => category == SourceCategory::Browser,
            CategoryArg::Cloud => category == SourceCategory::Cloud,
            CategoryArg::Application => category == SourceCategory::Application,
            CategoryArg::Memory => category == SourceCategory::Memory,
            CategoryArg::Filesystem => category == SourceCategory::FileSystem,
        }
    }
}

pub struct CredsCommandHandler {
    engine: CredentialHarvestEngine,
}

impl CredsCommandHandler {
    pub fn new() -> Self {
        Self {
            engine: CredentialHarvestEngine::new(),
        }
    }

    pub fn describe() -> &'static str {
        "Credential harvesting engine commands"
    }

    pub fn print_usage() {
        Theme::section("Creds CLI");
        Theme::command_help(
            "ferox creds harvest --platform windows",
            "Harvest credentials (safe mode)",
        );
        Theme::command_help("ferox creds list", "List available harvesters");
        Theme::command_help(
            "ferox creds describe <harvester>",
            "Show harvester details",
        );
        Theme::command_help("ferox creds show", "Show harvested credentials");
    }

    pub fn run(&self, command: CredsCommands) -> Result<()> {
        match command {
            CredsCommands::Harvest(args) => self.harvest_creds(args),
            CredsCommands::List(args) => self.list_harvesters(args),
            CredsCommands::Describe(args) => self.describe_harvester(args),
            CredsCommands::Show => self.show_harvested(),
        }
    }

    fn harvest_creds(&self, args: CredsHarvestArgs) -> Result<()> {
        Theme::section("Credential Harvesting Engine (SAFE MODE)");

        let platform: Platform = args.platform.into();

        Theme::info(&format!("Platform: {:?}", platform));
        Theme::info(&format!("Category: {:?}", args.category));
        Theme::info(&format!("Redact Output: {}", args.redact));

        println!();
        Theme::section("Available Harvesters");

        let harvesters = self.engine.harvesters_for_platform(&platform);
        let filtered: Vec<_> = harvesters
            .iter()
            .filter(|h| args.category.matches(h.category()))
            .collect();

        if filtered.is_empty() {
            Theme::warning("No harvesters found for the specified criteria");
            return Ok(());
        }

        for harvester in &filtered {
            let admin_str = if harvester.requires_admin() {
                "[ADMIN]"
            } else {
                "[USER]"
            };

            println!(
                "  {} {} - {} (MITRE: {})",
                harvester.name(),
                admin_str,
                harvester.category().as_str(),
                harvester.mitre_id()
            );
        }

        println!();
        Theme::section("Sample Output (SAFE MODE)");

        // Show reference for first few harvesters
        for harvester in filtered.iter().take(2) {
            println!();
            Theme::info(&format!("=== {} ===", harvester.name()));
            println!("{}", harvester.description());
            println!();

            let reference = harvester.generate_reference();
            for line in reference.lines().take(15) {
                println!("  {}", line);
            }
            println!("  ...");
        }

        println!();
        Theme::success(&format!(
            "[SAFE MODE] Would harvest credentials from {} sources",
            filtered.len()
        ));

        Ok(())
    }

    fn list_harvesters(&self, args: CredsListArgs) -> Result<()> {
        Theme::section("Credential Harvesters");

        let platform: Platform = args
            .platform
            .map(|p| p.into())
            .unwrap_or(Platform::Any);

        let harvesters = self.engine.harvesters_for_platform(&platform);

        // Filter by category if specified
        let filtered: Vec<_> = if let Some(cat) = args.category {
            harvesters
                .into_iter()
                .filter(|h| cat.matches(h.category()))
                .collect()
        } else {
            harvesters
        };

        println!(
            "{:<25} {:<10} {:<20} {:<8} {}",
            "NAME", "PLATFORM", "CATEGORY", "ADMIN", "MITRE ID"
        );
        println!("{}", "-".repeat(85));

        for harvester in filtered {
            println!(
                "{:<25} {:<10} {:<20} {:<8} {}",
                harvester.name(),
                format!("{:?}", harvester.platform()),
                harvester.category().as_str(),
                if harvester.requires_admin() {
                    "Yes"
                } else {
                    "No"
                },
                harvester.mitre_id()
            );

            if args.verbose {
                println!("   └─ {}", harvester.description());
            }
        }

        Ok(())
    }

    fn describe_harvester(&self, args: CredsDescribeArgs) -> Result<()> {
        let harvesters = self.engine.harvesters_for_platform(&Platform::Any);

        if let Some(harvester) = harvesters.iter().find(|h| h.name() == args.harvester) {
            Theme::section(&format!("Harvester: {}", harvester.name()));

            println!("Platform:    {:?}", harvester.platform());
            println!("Category:    {}", harvester.category().as_str());
            println!("Admin:       {}", harvester.requires_admin());
            println!("MITRE ID:    {}", harvester.mitre_id());
            println!("Description: {}", harvester.description());
            println!();

            Theme::info("Reference Documentation:");
            let reference = harvester.generate_reference();
            println!("{}", reference);
        } else {
            Theme::error(&format!("Unknown harvester: {}", args.harvester));
            Theme::info("Use 'ferox creds list' to see available harvesters");

            println!();
            Theme::info("Available harvesters:");
            for h in harvesters {
                println!("  - {}", h.name());
            }
        }

        Ok(())
    }

    fn show_harvested(&self) -> Result<()> {
        Theme::section("Harvested Credentials (SAFE MODE)");

        let creds = self.engine.harvested_credentials();

        if creds.is_empty() {
            Theme::info("[SAFE MODE] No credentials harvested in this session");
            Theme::info("Run 'ferox creds harvest' first to collect credentials");
            return Ok(());
        }

        println!(
            "{:<5} {:<15} {:<20} {:<15} {}",
            "#", "TYPE", "SOURCE", "SENSITIVITY", "USERNAME"
        );
        println!("{}", "-".repeat(70));

        for (i, cred) in creds.iter().enumerate() {
            let redacted = cred.redacted();
            println!(
                "{:<5} {:<15} {:<20} {:<15} {}",
                i + 1,
                redacted.cred_type.as_str(),
                redacted.source,
                redacted.sensitivity.as_str(),
                redacted.username.unwrap_or_else(|| "-".to_string())
            );
        }

        Ok(())
    }
}

impl Default for CredsCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}
