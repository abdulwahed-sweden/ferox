//! OPSEC CLI Commands
//!
//! Command-line interface for the OPSEC Engine.

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};

use crate::cli::theme::Theme;
use crate::core::module::Platform;
use crate::modules::evasion::opsec::{
    EdrDetector, LolbinExecutor, LogEvasion, MonitoredAction, OpsecConfig, OpsecEngine,
    StealthLevel,
};

#[derive(Subcommand, Debug, Clone)]
pub enum OpsecCommands {
    /// Show current OPSEC configuration
    Status,
    /// Configure OPSEC settings
    Config(OpsecConfigArgs),
    /// List available stealth levels
    Levels,
    /// Detect EDR/AV on target system
    Detect(OpsecDetectArgs),
    /// List LOLBin alternatives
    Lolbins(OpsecLolbinsArgs),
    /// Show evasion techniques for actions
    Evasion(OpsecEvasionArgs),
    /// Generate OPSEC report
    Report(OpsecReportArgs),
}

#[derive(Args, Debug, Clone)]
pub struct OpsecConfigArgs {
    /// Stealth level (ghost, silent, quiet, normal)
    #[arg(short, long)]
    pub stealth_level: Option<StealthArg>,

    /// Base sleep duration in seconds
    #[arg(long)]
    pub sleep: Option<u64>,

    /// Jitter factor (0.0-1.0)
    #[arg(short, long)]
    pub jitter: Option<f32>,

    /// Use LOLBins only
    #[arg(long)]
    pub lolbins_only: Option<bool>,

    /// Enable EDR awareness
    #[arg(long)]
    pub edr_aware: Option<bool>,

    /// Work hours only mode
    #[arg(long)]
    pub work_hours: Option<bool>,

    /// Memory-only execution
    #[arg(long)]
    pub memory_only: Option<bool>,

    /// Show configuration details
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct OpsecDetectArgs {
    /// Target platform
    #[arg(short, long, default_value = "windows")]
    pub platform: PlatformArg,

    /// Show verbose detection info
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct OpsecLolbinsArgs {
    /// Filter by dangerous tool name
    #[arg(short, long)]
    pub tool: Option<String>,

    /// Target platform
    #[arg(short = 'P', long, default_value = "windows")]
    pub platform: PlatformArg,

    /// Show verbose details
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct OpsecEvasionArgs {
    /// Action to check (process, file, registry, network, service, task, powershell, wmi, creds, dll)
    #[arg(short, long)]
    pub action: Option<String>,

    /// Show all monitored actions
    #[arg(long)]
    pub all: bool,

    /// Stealth level to check against
    #[arg(short, long, default_value = "quiet")]
    pub stealth: StealthArg,
}

#[derive(Args, Debug, Clone)]
pub struct OpsecReportArgs {
    /// Stealth level for report
    #[arg(short, long, default_value = "quiet")]
    pub stealth: StealthArg,

    /// Include EDR detection
    #[arg(long)]
    pub detect_edr: bool,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    pub format: String,
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum StealthArg {
    Ghost,
    Silent,
    #[default]
    Quiet,
    Normal,
}

impl From<StealthArg> for StealthLevel {
    fn from(arg: StealthArg) -> Self {
        match arg {
            StealthArg::Ghost => StealthLevel::Ghost,
            StealthArg::Silent => StealthLevel::Silent,
            StealthArg::Quiet => StealthLevel::Quiet,
            StealthArg::Normal => StealthLevel::Normal,
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum PlatformArg {
    #[default]
    Windows,
    Linux,
}

impl From<PlatformArg> for Platform {
    fn from(arg: PlatformArg) -> Self {
        match arg {
            PlatformArg::Windows => Platform::Windows,
            PlatformArg::Linux => Platform::Linux,
        }
    }
}

pub struct OpsecCommandHandler;

impl OpsecCommandHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn describe() -> &'static str {
        "OPSEC engine commands"
    }

    pub fn print_usage() {
        Theme::section("OPSEC CLI");
        Theme::command_help("ferox opsec status", "Show current OPSEC configuration");
        Theme::command_help(
            "ferox opsec config --stealth-level ghost",
            "Set stealth level",
        );
        Theme::command_help("ferox opsec levels", "List available stealth levels");
        Theme::command_help("ferox opsec detect", "Detect EDR/AV on system");
        Theme::command_help("ferox opsec lolbins", "List LOLBin alternatives");
        Theme::command_help("ferox opsec evasion --all", "Show evasion techniques");
        Theme::command_help("ferox opsec report", "Generate OPSEC report");
    }

    pub async fn run(&self, command: OpsecCommands) -> Result<()> {
        match command {
            OpsecCommands::Status => self.show_status().await,
            OpsecCommands::Config(args) => self.configure(args),
            OpsecCommands::Levels => self.list_levels(),
            OpsecCommands::Detect(args) => self.detect_edr(args).await,
            OpsecCommands::Lolbins(args) => self.list_lolbins(args),
            OpsecCommands::Evasion(args) => self.show_evasion(args),
            OpsecCommands::Report(args) => self.generate_report(args).await,
        }
    }

    async fn show_status(&self) -> Result<()> {
        Theme::section("OPSEC Status (SAFE MODE)");

        let config = OpsecConfig::quiet();

        println!("Stealth Level:    {}", config.stealth_level.as_str());
        println!("Sleep Duration:   {:?}", config.sleep);
        println!("Jitter:           {:.2}", config.jitter);
        println!(
            "Max Packets:      {}",
            config.max_network_noise.max_packets()
        );
        println!("LOLBins Only:     {}", config.use_lolbins_only);
        println!("Memory Only:      {}", config.memory_only);
        println!("Encrypt Traffic:  {}", config.encrypt_traffic);
        println!("User Simulation:  {}", config.simulate_user);
        println!("Work Hours Only:  {}", config.work_hours_only);
        println!("EDR Aware:        {}", config.edr_aware);

        println!();
        Theme::section("Advanced Features");
        println!("AMSI Bypass:      {}", config.amsi_bypass);
        println!("ETW Patch:        {}", config.etw_patch);
        println!("PPID Spoofing:    {}", config.ppid_spoofing);
        println!("Process Hollow:   {}", config.process_hollowing);
        println!("Unhook EDR:       {}", config.unhook_edr);
        println!("Disguise Traffic: {}", config.disguise_traffic);

        Ok(())
    }

    fn configure(&self, args: OpsecConfigArgs) -> Result<()> {
        Theme::section("OPSEC Configuration (SAFE MODE)");

        let base_level = args.stealth_level.map(|s| s.into()).unwrap_or(StealthLevel::Quiet);
        let mut config = OpsecConfig::from_level(base_level);

        if let Some(sleep) = args.sleep {
            config.sleep = std::time::Duration::from_secs(sleep);
        }
        if let Some(jitter) = args.jitter {
            config.jitter = jitter.clamp(0.0, 1.0);
        }
        if let Some(lolbins) = args.lolbins_only {
            config.use_lolbins_only = lolbins;
        }
        if let Some(edr) = args.edr_aware {
            config.edr_aware = edr;
        }
        if let Some(wh) = args.work_hours {
            config.work_hours_only = wh;
        }
        if let Some(mem) = args.memory_only {
            config.memory_only = mem;
        }

        Theme::success("OPSEC configuration updated:");
        println!();
        println!("Stealth Level:    {}", config.stealth_level.as_str());
        println!("Sleep Duration:   {:?}", config.sleep);
        println!("Jitter:           {:.2}", config.jitter);
        println!("LOLBins Only:     {}", config.use_lolbins_only);
        println!("EDR Aware:        {}", config.edr_aware);
        println!("Work Hours Only:  {}", config.work_hours_only);
        println!("Memory Only:      {}", config.memory_only);

        if args.verbose {
            println!();
            Theme::section("Full Configuration");
            println!("{:#?}", config);
        }

        Ok(())
    }

    fn list_levels(&self) -> Result<()> {
        Theme::section("OPSEC Stealth Levels");

        let levels = [
            (StealthLevel::Ghost, "Maximum stealth, very slow operations"),
            (StealthLevel::Silent, "High stealth, moderate speed"),
            (StealthLevel::Quiet, "Balanced stealth and speed"),
            (StealthLevel::Normal, "Standard operations with minimal restrictions"),
        ];

        println!(
            "{:<12} {:<25} {:<10} {:<10} {:<8} {}",
            "LEVEL", "DESCRIPTION", "SLEEP", "JITTER", "PACKETS", "FEATURES"
        );
        println!("{}", "-".repeat(100));

        for (level, desc) in levels {
            let config = OpsecConfig::from_level(level);
            let jitter_range = level.jitter_range();

            println!(
                "{:<12} {:<25} {:<10} {:<10} {:<8} {}",
                level.as_str().split(' ').next().unwrap_or(""),
                desc,
                format!("{:?}", config.sleep),
                format!("{:.1}-{:.1}", jitter_range.0, jitter_range.1),
                level.max_packets(),
                if config.use_lolbins_only {
                    "LOLBins, "
                } else {
                    ""
                }.to_string()
                    + if config.memory_only { "MemOnly, " } else { "" }
                    + if config.simulate_user { "UserSim" } else { "" }
            );
        }

        println!();
        Theme::info("Use 'ferox opsec config --stealth-level <level>' to set");

        Ok(())
    }

    async fn detect_edr(&self, args: OpsecDetectArgs) -> Result<()> {
        Theme::section("EDR Detection (SAFE MODE)");

        let detector = EdrDetector::new();
        let result = detector.detect(true).await?;

        Theme::section("Detected Security Software");
        println!(
            "{:<25} {:<20} {:<15} {}",
            "EDR/AV", "RECOMMENDED", "MITRE ID", "PROCESSES"
        );
        println!("{}", "-".repeat(80));

        for edr in &result.detected_edrs {
            println!(
                "{:<25} {:<20} {:<15} {}",
                edr.as_str(),
                edr.recommended_stealth().as_str(),
                edr.mitre_id(),
                result.processes.join(", ")
            );
        }

        if args.verbose {
            println!();
            Theme::section("Detection Details");
            println!("Processes: {:?}", result.processes);
            println!("Services:  {:?}", result.services);
            println!("Drivers:   {:?}", result.drivers);
            println!("Hooks:     {}", result.hooks_detected);
        }

        println!();
        Theme::section("Recommended Configuration");
        println!(
            "Stealth Level:  {}",
            result.recommended_config.stealth_level.as_str()
        );
        println!("Sleep:          {:?}", result.recommended_config.sleep);
        println!("Jitter:         {:.2}", result.recommended_config.jitter);
        println!("LOLBins Only:   {}", result.recommended_config.use_lolbins_only);
        println!("Memory Only:    {}", result.recommended_config.memory_only);

        Ok(())
    }

    fn list_lolbins(&self, args: OpsecLolbinsArgs) -> Result<()> {
        Theme::section("LOLBin Alternatives");

        let platform: Platform = args.platform.into();
        let executor = LolbinExecutor::new(platform);
        let mappings = executor.list_mappings();

        // Filter by tool if specified
        let mappings: Vec<_> = if let Some(tool) = &args.tool {
            mappings
                .iter()
                .filter(|m| m.dangerous_tool.to_lowercase().contains(&tool.to_lowercase()))
                .collect()
        } else {
            mappings.iter().collect()
        };

        if mappings.is_empty() {
            Theme::warning("No LOLBin mappings found");
            return Ok(());
        }

        println!(
            "{:<18} {:<18} {:<12} {}",
            "DANGEROUS TOOL", "LOLBIN", "MITRE ID", "DESCRIPTION"
        );
        println!("{}", "-".repeat(90));

        for mapping in &mappings {
            println!(
                "{:<18} {:<18} {:<12} {}",
                mapping.dangerous_tool,
                mapping.lolbin_alternative,
                mapping.mitre_id,
                mapping.description
            );

            if args.verbose {
                println!("   Command: {}", mapping.command_template);
                println!();
            }
        }

        println!();
        Theme::info(&format!("Total: {} LOLBin alternatives", mappings.len()));

        Ok(())
    }

    fn show_evasion(&self, args: OpsecEvasionArgs) -> Result<()> {
        Theme::section("Evasion Techniques");

        let stealth: StealthLevel = args.stealth.into();
        let config = OpsecConfig::from_level(stealth);
        let evasion = LogEvasion::new(config);

        let actions = [
            ("process", MonitoredAction::ProcessCreation),
            ("file", MonitoredAction::FileWrite),
            ("registry", MonitoredAction::RegistryModification),
            ("network", MonitoredAction::NetworkConnection),
            ("service", MonitoredAction::ServiceCreation),
            ("task", MonitoredAction::ScheduledTask),
            ("powershell", MonitoredAction::PowerShellExecution),
            ("wmi", MonitoredAction::WmiQuery),
            ("creds", MonitoredAction::CredentialAccess),
            ("dll", MonitoredAction::DllInjection),
        ];

        // Filter by action if specified
        let filtered: Vec<_> = if let Some(action_name) = &args.action {
            actions
                .iter()
                .filter(|(name, _)| name.contains(&action_name.to_lowercase()))
                .collect()
        } else if args.all {
            actions.iter().collect()
        } else {
            // Show only monitored actions for this stealth level
            actions
                .iter()
                .filter(|(_, action)| evasion.is_monitored(*action))
                .collect()
        };

        println!(
            "{:<15} {:<6} {:<10} {}",
            "ACTION", "RISK", "MONITORED", "EVASION TECHNIQUE"
        );
        println!("{}", "-".repeat(90));

        for (name, action) in filtered {
            let monitored = evasion.is_monitored(*action);
            let status = if monitored { "YES" } else { "No" };

            println!(
                "{:<15} {:<6} {:<10} {}",
                name,
                action.risk_level(),
                status,
                action.evasion_technique()
            );
        }

        println!();
        Theme::info(&format!(
            "Stealth level '{}' monitors actions with risk >= threshold",
            stealth.as_str()
        ));

        Ok(())
    }

    async fn generate_report(&self, args: OpsecReportArgs) -> Result<()> {
        Theme::section("OPSEC Report (SAFE MODE)");

        let stealth: StealthLevel = args.stealth.into();
        let platform = Platform::Windows;

        let engine = if args.detect_edr {
            OpsecEngine::with_auto_stealth(platform, true).await?
        } else {
            OpsecEngine::new(OpsecConfig::from_level(stealth), platform)
        };

        let report = engine.generate_report();

        if args.format == "json" {
            println!("{}", serde_json::to_string_pretty(&report)?);
        } else {
            println!("=== OPSEC Report ===\n");
            println!("Stealth Level:     {}", report.stealth_level.as_str());
            println!("Sleep Duration:    {:?}", report.sleep_duration);
            println!("Jitter:            {:.2}", report.jitter);
            println!("LOLBins Only:      {}", report.lolbins_only);
            println!("Memory Only:       {}", report.memory_only);
            println!("Traffic Encrypted: {}", report.traffic_encrypted);
            println!("User Simulation:   {}", report.user_simulation);
            println!("Work Hours Only:   {}", report.work_hours_only);
            println!("EDR Aware:         {}", report.edr_aware);

            if !report.detected_edrs.is_empty() {
                println!();
                println!(
                    "Detected EDRs:     {}",
                    report
                        .detected_edrs
                        .iter()
                        .map(|e| e.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }

            println!();
            println!("Features Enabled:");
            for feature in &report.features_enabled {
                println!("  - {}", feature);
            }

            println!();
            println!("=== MITRE ATT&CK Coverage ===");
            println!("T1562 - Impair Defenses");
            println!("T1070 - Indicator Removal");
            println!("T1027 - Obfuscation");
            println!("T1497 - Virtualization/Sandbox Evasion");
        }

        Ok(())
    }
}

impl Default for OpsecCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}
