//! Lateral Movement CLI Commands
//!
//! Command-line interface for the Lateral Movement Engine.

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};

use crate::cli::theme::Theme;
use crate::core::module::Platform;
use crate::modules::post::credential_harvester::{
    CredentialType, HarvestedCredential, Sensitivity, SourceCategory,
};
use crate::modules::post::lateral_movement::{LateralMovementEngine, StealthLevel, Target};

#[derive(Subcommand, Debug, Clone)]
pub enum LateralCommands {
    /// List available lateral movement methods
    List(LateralListArgs),
    /// Describe a specific lateral movement method
    Describe(LateralDescribeArgs),
    /// Discover potential targets in the network
    Discover(LateralDiscoverArgs),
    /// Spread to targets using harvested credentials
    Spread(LateralSpreadArgs),
    /// Test credentials against discovered targets
    Test(LateralTestArgs),
}

#[derive(Args, Debug, Clone)]
pub struct LateralListArgs {
    /// Filter by platform (windows, linux, any)
    #[arg(short, long)]
    pub platform: Option<PlatformArg>,

    /// Show detailed information
    #[arg(short, long)]
    pub verbose: bool,

    /// Filter by minimum stealth level
    #[arg(short, long)]
    pub stealth: Option<StealthArg>,
}

#[derive(Args, Debug, Clone)]
pub struct LateralDescribeArgs {
    /// Method name to describe
    pub method: String,
}

#[derive(Args, Debug, Clone)]
pub struct LateralDiscoverArgs {
    /// Target network/subnet (CIDR notation)
    #[arg(short, long, default_value = "192.168.1.0/24")]
    pub network: String,

    /// Discovery methods to use (arp, ad, dns, shares)
    #[arg(short, long, value_delimiter = ',', default_value = "arp,ad,dns")]
    pub methods: Vec<String>,

    /// Platform filter for targets
    #[arg(short = 'P', long)]
    pub platform: Option<PlatformArg>,

    /// Show verbose discovery output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct LateralSpreadArgs {
    /// Auto-spread mode (discovers targets and spreads automatically)
    #[arg(long)]
    pub auto: bool,

    /// Specific target IP or hostname
    #[arg(short, long)]
    pub target: Option<String>,

    /// Maximum number of targets to spread to
    #[arg(short = 'n', long, default_value = "5")]
    pub max_targets: usize,

    /// Lateral movement method to use (or 'auto' for best selection)
    #[arg(short = 'm', long, default_value = "auto")]
    pub method: String,

    /// Target platform
    #[arg(short = 'P', long, default_value = "auto")]
    pub platform: PlatformArg,

    /// Minimum stealth level
    #[arg(short, long, default_value = "low")]
    pub stealth: StealthArg,

    /// Domain for AD attacks
    #[arg(short, long)]
    pub domain: Option<String>,

    /// Credentials file (JSON format)
    #[arg(short, long)]
    pub creds_file: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct LateralTestArgs {
    /// Test credentials against specific target
    #[arg(short, long)]
    pub target: Option<String>,

    /// Test against all discovered targets
    #[arg(long)]
    pub all: bool,

    /// Credentials file (JSON format)
    #[arg(short, long)]
    pub creds_file: Option<String>,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum PlatformArg {
    #[default]
    Auto,
    Windows,
    Linux,
}

impl From<PlatformArg> for Platform {
    fn from(arg: PlatformArg) -> Self {
        match arg {
            PlatformArg::Auto => Platform::Any,
            PlatformArg::Windows => Platform::Windows,
            PlatformArg::Linux => Platform::Linux,
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

pub struct LateralCommandHandler {
    engine: LateralMovementEngine,
}

impl LateralCommandHandler {
    pub fn new() -> Self {
        Self {
            engine: LateralMovementEngine::new(),
        }
    }

    pub fn describe() -> &'static str {
        "Lateral movement engine commands"
    }

    pub fn print_usage() {
        Theme::section("Lateral Movement CLI");
        Theme::command_help(
            "ferox lateral list",
            "List available lateral movement methods",
        );
        Theme::command_help(
            "ferox lateral list --platform windows",
            "List Windows-specific methods",
        );
        Theme::command_help(
            "ferox lateral describe psexec",
            "Show detailed info for PSExec method",
        );
        Theme::command_help("ferox lateral discover", "Discover potential targets");
        Theme::command_help(
            "ferox lateral spread --auto",
            "Auto-spread using discovered targets",
        );
        Theme::command_help(
            "ferox lateral spread --target 192.168.1.10",
            "Spread to specific target",
        );
        Theme::command_help(
            "ferox lateral test --all",
            "Test credentials against all targets",
        );
    }

    pub async fn run(&self, command: LateralCommands) -> Result<()> {
        match command {
            LateralCommands::List(args) => self.list_methods(args),
            LateralCommands::Describe(args) => self.describe_method(args),
            LateralCommands::Discover(args) => self.discover_targets(args).await,
            LateralCommands::Spread(args) => self.spread_targets(args).await,
            LateralCommands::Test(args) => self.test_credentials(args).await,
        }
    }

    fn list_methods(&self, args: LateralListArgs) -> Result<()> {
        Theme::section("Lateral Movement Methods");

        let platform: Platform = args.platform.map(|p| p.into()).unwrap_or(Platform::Any);
        let min_stealth: StealthLevel = args.stealth.map(|s| s.into()).unwrap_or(StealthLevel::VeryLow);
        let methods = self.engine.methods_for_platform(&platform);

        // Filter by stealth if specified
        let methods: Vec<_> = methods
            .into_iter()
            .filter(|m| m.stealth_level() >= min_stealth)
            .collect();

        println!(
            "{:<20} {:<10} {:<12} {:<12} {:<10} {}",
            "NAME", "PLATFORM", "STEALTH", "MITRE ID", "ADMIN", "DESCRIPTION"
        );
        println!("{}", "-".repeat(100));

        for method in &methods {
            let platform_str = match method.platform() {
                Platform::Windows => "Windows",
                Platform::Linux => "Linux",
                Platform::MacOS => "macOS",
                Platform::Any => "Any",
            };

            println!(
                "{:<20} {:<10} {:<12} {:<12} {:<10} {}",
                method.name(),
                platform_str,
                method.stealth_level().as_str(),
                method.mitre_id(),
                if method.requires().needs_admin {
                    "Yes"
                } else {
                    "No"
                },
                if args.verbose {
                    method.description()
                } else {
                    &method.description()[..method.description().len().min(40)]
                }
            );

            if args.verbose {
                let reqs = method.requires();
                println!(
                    "   Requirements: Admin={}, DomainAdmin={}, Creds={}, Hash={}, Ticket={}, SSHKey={}",
                    reqs.needs_admin,
                    reqs.needs_domain_admin,
                    reqs.needs_credentials,
                    reqs.needs_hash,
                    reqs.needs_ticket,
                    reqs.needs_ssh_key
                );
                if !reqs.required_ports.is_empty() {
                    println!("   Ports: {:?}", reqs.required_ports);
                }
                println!();
            }
        }

        println!();
        Theme::info(&format!("Total: {} methods available", methods.len()));

        Ok(())
    }

    fn describe_method(&self, args: LateralDescribeArgs) -> Result<()> {
        if let Some(method) = self.engine.get_method(&args.method) {
            Theme::section(&format!("Lateral Movement: {}", method.name()));

            println!("Platform:    {:?}", method.platform());
            println!("Stealth:     {}", method.stealth_level().as_str());
            println!("MITRE ID:    {}", method.mitre_id());
            println!("Description: {}", method.description());
            println!();

            let reqs = method.requires();
            println!("Requirements:");
            println!("  Admin Required:        {}", reqs.needs_admin);
            println!("  Domain Admin Required: {}", reqs.needs_domain_admin);
            println!("  Needs Credentials:     {}", reqs.needs_credentials);
            println!("  Needs Hash:            {}", reqs.needs_hash);
            println!("  Needs Kerberos Ticket: {}", reqs.needs_ticket);
            println!("  Needs SSH Key:         {}", reqs.needs_ssh_key);
            if !reqs.required_ports.is_empty() {
                println!("  Required Ports:        {:?}", reqs.required_ports);
            }
            println!();

            // Create a demo target for reference generation
            let demo_target = Target::new("192.168.1.10".parse().unwrap())
                .with_hostname("TARGET01")
                .with_platform(method.platform())
                .with_domain("CORP.LOCAL");

            Theme::info("Reference Implementation:");
            let reference = method.generate_reference(&demo_target, None);
            println!("{}", reference);
        } else {
            Theme::error(&format!("Unknown method: {}", args.method));
            println!();
            Theme::info("Available methods:");
            for method in self.engine.methods_for_platform(&Platform::Any) {
                println!("  - {}", method.name());
            }
        }

        Ok(())
    }

    async fn discover_targets(&self, args: LateralDiscoverArgs) -> Result<()> {
        Theme::section("Target Discovery (SAFE MODE)");

        Theme::info(&format!("Network: {}", args.network));
        Theme::info(&format!("Methods: {}", args.methods.join(", ")));

        if let Some(platform) = args.platform {
            Theme::info(&format!("Platform filter: {:?}", Platform::from(platform)));
        }

        println!();

        // Create demo session for discovery
        let session = crate::core::module::Session::new(
            "lateral_movement".to_string(),
            "localhost".to_string(),
            Platform::Any,
        );

        // Run discovery in safe mode
        let result = self.engine.discover_targets(&session, true).await?;

        Theme::section("Discovered Targets");
        println!(
            "{:<16} {:<20} {:<10} {:<30} {:<8} {}",
            "IP ADDRESS", "HOSTNAME", "PLATFORM", "SERVICES", "DC", "CONFIDENCE"
        );
        println!("{}", "-".repeat(100));

        for target in &result.targets {
            let platform_str = match target.platform {
                Platform::Windows => "Windows",
                Platform::Linux => "Linux",
                Platform::MacOS => "macOS",
                Platform::Any => "Unknown",
            };

            let services = target.services.join(", ");
            let services_truncated = if services.len() > 28 {
                format!("{}...", &services[..25])
            } else {
                services
            };

            println!(
                "{:<16} {:<20} {:<10} {:<30} {:<8} {:.0}%",
                target.ip_address,
                target.hostname.clone().unwrap_or_else(|| "-".to_string()),
                platform_str,
                services_truncated,
                if target.is_domain_controller {
                    "Yes"
                } else {
                    "No"
                },
                target.confidence * 100.0
            );

            if args.verbose {
                println!("   Ports: {:?}", target.ports);
                if let Some(domain) = &target.domain {
                    println!("   Domain: {}", domain);
                }
                println!();
            }
        }

        println!();

        // Show domain info if available
        if let Some(domain_info) = &result.domain_info {
            Theme::section("Domain Information");
            println!("Domain Name:      {}", domain_info.domain_name);
            if let Some(forest) = &domain_info.forest_name {
                println!("Forest Name:      {}", forest);
            }
            if let Some(level) = &domain_info.functional_level {
                println!("Functional Level: {}", level);
            }
            println!(
                "Domain Controllers: {}",
                domain_info
                    .domain_controllers
                    .iter()
                    .map(|dc| dc.display_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            if !domain_info.trusts.is_empty() {
                println!("Domain Trusts:    {}", domain_info.trusts.join(", "));
            }
            println!();
        }

        Theme::success(&format!(
            "[SAFE MODE] Discovered {} potential targets",
            result.targets.len()
        ));
        Theme::info(&format!(
            "Discovery methods used: {}",
            result.discovery_methods.join(", ")
        ));

        Ok(())
    }

    async fn spread_targets(&self, args: LateralSpreadArgs) -> Result<()> {
        Theme::section("Lateral Movement Spread (SAFE MODE)");

        let platform: Platform = args.platform.into();
        let stealth: StealthLevel = args.stealth.into();

        Theme::info(&format!("Mode: {}", if args.auto { "Auto" } else { "Manual" }));
        Theme::info(&format!("Platform: {:?}", platform));
        Theme::info(&format!("Min Stealth: {}", stealth.as_str()));
        Theme::info(&format!("Max Targets: {}", args.max_targets));
        Theme::info(&format!(
            "Method: {}",
            if args.method == "auto" {
                "Auto-select best"
            } else {
                &args.method
            }
        ));

        if let Some(domain) = &args.domain {
            Theme::info(&format!("Domain: {}", domain));
        }

        println!();

        // Create demo credentials
        let demo_creds = self.get_demo_credentials();

        Theme::section("Available Credentials");
        for cred in &demo_creds {
            println!(
                "  - {} ({:?}): {}",
                cred.username.clone().unwrap_or_else(|| "unknown".to_string()),
                cred.cred_type,
                cred.domain.clone().unwrap_or_else(|| "local".to_string())
            );
        }
        println!();

        // Create session
        let session = crate::core::module::Session::new(
            "lateral_movement".to_string(),
            "localhost".to_string(),
            platform.clone(),
        );

        // Get targets - either specific or discovered
        let targets = if let Some(target_ip) = &args.target {
            vec![Target::new(target_ip.parse().unwrap_or_else(|_| "192.168.1.1".parse().unwrap()))
                .with_platform(platform.clone())
                .with_ports(vec![445, 135, 3389, 22])]
        } else {
            // Use discovery
            let result = self.engine.discover_targets(&session, true).await?;
            result.targets
        };

        let targets: Vec<_> = targets.into_iter().take(args.max_targets).collect();

        Theme::section("Spread Results");

        if args.method == "auto" {
            // Show which methods would be used for each target
            println!(
                "{:<16} {:<20} {:<20} {:<12} {}",
                "TARGET", "METHOD", "MITRE ID", "STATUS", "DETAILS"
            );
            println!("{}", "-".repeat(90));

            for target in &targets {
                // Find best method for this target
                let methods = self.engine.methods_for_platform(&target.platform);
                let viable_methods: Vec<_> = methods
                    .into_iter()
                    .filter(|m| m.can_target(target, &demo_creds) && m.stealth_level() >= stealth)
                    .collect();

                if let Some(best_method) = viable_methods.first() {
                    let prob = best_method.success_probability(target, &demo_creds);
                    println!(
                        "{:<16} {:<20} {:<20} {:<12} Success probability: {:.0}%",
                        target.display_name(),
                        best_method.name(),
                        best_method.mitre_id(),
                        "[SAFE MODE]",
                        prob * 100.0
                    );
                } else {
                    println!(
                        "{:<16} {:<20} {:<20} {:<12} {}",
                        target.display_name(),
                        "-",
                        "-",
                        "SKIPPED",
                        "No viable method found"
                    );
                }
            }
        } else {
            // Specific method requested
            if let Some(method) = self.engine.get_method(&args.method) {
                for target in &targets {
                    if method.can_target(target, &demo_creds) {
                        let prob = method.success_probability(target, &demo_creds);
                        println!(
                            "{:<16} {:<20} {:<20} {:<12} Success probability: {:.0}%",
                            target.display_name(),
                            method.name(),
                            method.mitre_id(),
                            "[SAFE MODE]",
                            prob * 100.0
                        );
                    } else {
                        println!(
                            "{:<16} {:<20} {:<20} {:<12} {}",
                            target.display_name(),
                            method.name(),
                            "-",
                            "INCOMPAT",
                            "Target not compatible with method"
                        );
                    }
                }
            } else {
                Theme::error(&format!("Unknown method: {}", args.method));
                return Ok(());
            }
        }

        println!();
        Theme::success(&format!(
            "[SAFE MODE] Would spread to {} targets",
            targets.len()
        ));

        // Show attack chain example
        println!();
        Theme::section("Attack Chain Example");
        println!(
            r#"
// Full attack chain integration
let session = payload_system.deploy(target).await?;
let elevated = privesc_engine.escalate(session).await?;
let creds = cred_harvester.harvest_all(elevated).await?;
let persistence = persist_engine.install(elevated).await?;

// Lateral movement
let new_sessions = lateral_engine
    .auto_spread(elevated, &creds, {})
    .await?;

// Exponential spread
for new_session in new_sessions {{
    let more_creds = cred_harvester.harvest_all(new_session).await?;
    // Continue spreading...
}}
"#,
            args.max_targets
        );

        Ok(())
    }

    async fn test_credentials(&self, args: LateralTestArgs) -> Result<()> {
        Theme::section("Credential Testing (SAFE MODE)");

        // Get demo credentials
        let creds = self.get_demo_credentials();

        Theme::info(&format!("Testing {} credentials", creds.len()));

        // Get targets
        let targets = if let Some(target_ip) = &args.target {
            vec![Target::new(target_ip.parse().unwrap_or_else(|_| "192.168.1.1".parse().unwrap()))
                .with_platform(Platform::Windows)
                .with_ports(vec![445, 135, 3389])]
        } else if args.all {
            // Use discovery
            let session = crate::core::module::Session::new(
                "lateral_movement".to_string(),
                "localhost".to_string(),
                Platform::Any,
            );
            let result = self.engine.discover_targets(&session, true).await?;
            result.targets
        } else {
            Theme::warning("No target specified. Use --target IP or --all");
            return Ok(());
        };

        println!();
        Theme::section("Credential-Target Matrix");

        println!(
            "{:<25} {:<15} {:<25} {:<10} {}",
            "CREDENTIAL", "TYPE", "TARGETS", "PROB", "METHODS"
        );
        println!("{}", "-".repeat(90));

        // Test each credential
        let cred_map = self.engine.test_credentials(&creds, &targets, true).await?;

        for mapping in &cred_map.mappings {
            let cred_name = mapping
                .credential
                .username
                .clone()
                .unwrap_or_else(|| "unknown".to_string());

            let target_count = mapping.valid_targets.len();

            // Find which methods work
            let mut methods = Vec::new();
            for method in self.engine.methods_for_platform(&Platform::Any) {
                if method.can_target(&targets[0], std::slice::from_ref(&mapping.credential)) {
                    methods.push(method.name().to_string());
                }
            }

            println!(
                "{:<25} {:<15} {:<25} {:<10} {}",
                cred_name,
                format!("{:?}", mapping.credential.cred_type),
                format!("{} targets", target_count),
                format!("{:.0}%", mapping.success_probability * 100.0),
                methods.join(", ")
            );

            if args.verbose {
                println!(
                    "   Domain: {}",
                    mapping
                        .credential
                        .domain
                        .clone()
                        .unwrap_or_else(|| "local".to_string())
                );
                println!(
                    "   Sensitivity: {:?}",
                    mapping.credential.sensitivity
                );
                println!();
            }
        }

        println!();
        Theme::success(&format!(
            "[SAFE MODE] Tested {} credentials against {} targets",
            creds.len(),
            targets.len()
        ));

        Ok(())
    }

    fn get_demo_credentials(&self) -> Vec<HarvestedCredential> {
        vec![
            HarvestedCredential::new(CredentialType::PlainText, "LSASS", SourceCategory::Memory)
                .with_username("Administrator")
                .with_password("[SAFE_MODE_DEMO]")
                .with_domain("CORP")
                .with_sensitivity(Sensitivity::Critical),
            HarvestedCredential::new(CredentialType::Hash, "SAM", SourceCategory::OsCredentialStore)
                .with_username("Administrator")
                .with_hash("aad3b435b51404eeaad3b435b51404ee:8846f7eaee8fb117ad06bdd830b7586c")
                .with_domain("CORP")
                .with_sensitivity(Sensitivity::Critical),
            HarvestedCredential::new(
                CredentialType::KerberosTicket,
                "LSASS",
                SourceCategory::Memory,
            )
            .with_username("admin")
            .with_domain("CORP.LOCAL")
            .with_sensitivity(Sensitivity::Critical)
            .with_metadata("ticket_type", "TGT"),
            HarvestedCredential::new(
                CredentialType::SshKey,
                "~/.ssh/id_rsa",
                SourceCategory::FileSystem,
            )
            .with_username("root")
            .with_sensitivity(Sensitivity::Critical)
            .with_metadata("encrypted", "false"),
            HarvestedCredential::new(
                CredentialType::PlainText,
                "Credential Manager",
                SourceCategory::OsCredentialStore,
            )
            .with_username("svc_backup")
            .with_password("[SAFE_MODE_DEMO]")
            .with_domain("CORP")
            .with_sensitivity(Sensitivity::High),
        ]
    }
}

impl Default for LateralCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}
