use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Args, Subcommand};
use toml::Value;

use crate::cli::theme::Theme;
use crate::core::module::{Module, ModuleInfo, ModuleOption};
use crate::modules::c2::{
    github_c2::GitHubC2, relay_manager::RelayManagerModule, teams_tunnel::TeamsTunnel,
};

#[derive(Subcommand, Debug, Clone)]
pub enum C2Commands {
    /// List available C2 implants/transports
    List,
    /// Show detailed options for a specific module
    Describe(C2DescribeArgs),
    /// Validate config files, tokens, and runtime expectations
    Health(C2HealthArgs),
}

#[derive(Args, Debug, Clone)]
pub struct C2DescribeArgs {
    /// Optional module name (defaults to all)
    pub module: Option<String>,
    /// Include module option metadata
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct C2HealthArgs {
    /// Path to the C2 configuration file
    #[arg(long, default_value = "config/c2.example.toml")]
    pub config: PathBuf,
}

pub struct C2CommandHandler;

impl C2CommandHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn describe() -> &'static str {
        "Surface C2 modules and config health"
    }

    pub fn print_usage() {
        Theme::section("C2 CLI");
        Theme::command_help("ferox c2 list", "Show registered C2 modules");
        Theme::command_help("ferox c2 describe <module>", "Inspect module options");
        Theme::command_help(
            "ferox c2 health --config ferox_c2.toml",
            "Config and token check",
        );
    }

    pub fn run(&self, command: C2Commands) -> Result<()> {
        match command {
            C2Commands::List => self.list_modules(),
            C2Commands::Describe(args) => self.describe_modules(args),
            C2Commands::Health(args) => self.health_check(args),
        }
    }

    fn list_modules(&self) -> Result<()> {
        Theme::section("C2 Modules");
        for info in self.module_infos() {
            println!(
                "{:<16} {:<12} {}",
                info.name,
                format!("{:?}", info.module_type),
                info.description
            );
        }
        Ok(())
    }

    fn describe_modules(&self, args: C2DescribeArgs) -> Result<()> {
        let infos = self.module_infos();
        if let Some(name) = args.module {
            if let Some(info) = infos.iter().find(|m| m.name == name) {
                Theme::section(&format!("{}", info.name));
                self.print_info(info);
                if args.verbose {
                    self.print_options(info);
                }
            } else {
                Theme::error(&format!("Unknown module: {}", name));
            }
            return Ok(());
        }

        Theme::section("C2 Catalog");
        for info in infos {
            self.print_info(&info);
            if args.verbose {
                self.print_options(&info);
            }
        }
        Ok(())
    }

    fn health_check(&self, args: C2HealthArgs) -> Result<()> {
        Theme::section("C2 Environment Health");
        self.check_config(&args.config);
        self.check_auth_token();
        Ok(())
    }

    fn module_infos(&self) -> Vec<ModuleInfo> {
        self.modules().into_iter().map(|m| m.info()).collect()
    }

    fn modules(&self) -> Vec<Box<dyn Module>> {
        vec![
            Box::new(TeamsTunnel::new()),
            Box::new(GitHubC2::new()),
            Box::new(RelayManagerModule::new()),
        ]
    }

    fn print_info(&self, info: &ModuleInfo) {
        Theme::status(
            "ready",
            &format!("{} [{}] - {}", info.name, info.category, info.description),
        );
    }

    fn print_options(&self, info: &ModuleInfo) {
        Theme::section(&format!("{} Options", info.name));
        for opt in self
            .modules()
            .into_iter()
            .find(|m| m.info().name == info.name)
            .map(|m| m.options())
            .unwrap_or_default()
        {
            self.describe_option(&opt);
        }
    }

    fn describe_option(&self, opt: &ModuleOption) {
        println!(
            "  {:<20} required={} default={}\n      {}",
            opt.name,
            opt.required,
            opt.default_value
                .clone()
                .unwrap_or_else(|| "<none>".to_string()),
            opt.description
        );
    }

    fn check_config(&self, path: &Path) {
        if !path.exists() {
            Theme::warning(&format!("Config not found: {}", path.display()));
            return;
        }

        match fs::read_to_string(path) {
            Ok(contents) => match contents.parse::<Value>() {
                Ok(value) => {
                    Theme::success(&format!("Loaded config {}", path.display()));
                    if let Some(interval) = value
                        .get("beacon_poll_interval_ms")
                        .and_then(|v| v.as_integer())
                    {
                        Theme::info(&format!("Beacon interval: {interval} ms"));
                    }
                    if let Some(provider) = value.get("cloud_provider").and_then(|v| v.as_str()) {
                        Theme::info(&format!("Cloud provider: {provider}"));
                    }
                }
                Err(err) => {
                    Theme::error(&format!(
                        "Config {} is invalid TOML: {}",
                        path.display(),
                        err
                    ));
                }
            },
            Err(err) => {
                Theme::error(&format!("Unable to read {}: {}", path.display(), err));
            }
        }
    }

    fn check_auth_token(&self) {
        match std::env::var("FEROX_C2_TOKEN") {
            Ok(val) if !val.is_empty() => {
                Theme::success("FEROX_C2_TOKEN present (value redacted)");
            }
            _ => Theme::warning("FEROX_C2_TOKEN not set"),
        }
    }
}
