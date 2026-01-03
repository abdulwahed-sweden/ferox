//! Wizard Command Handler
//!
//! CLI handler for the attack wizard subcommand

use anyhow::Result;
use clap::Args;

use crate::cli::wizard::{AttackWizard, WizardArgs, list_templates};
use crate::core::module::ModuleRegistry;
use crate::modules::recon::asn::AsnDiscovery;
use crate::modules::recon::dns::DnsEnumerator;
use crate::modules::recon::subdomains::SubdomainEnum;
use crate::modules::recon::whois::WhoisLookup;
use crate::modules::scanner::http_scanner::HttpScanner;
use crate::modules::scanner::port::PortScanner;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Wizard CLI arguments
#[derive(Args, Debug, Clone)]
pub struct WizardCommands {
    /// Skip to module selection (quick mode)
    #[arg(short, long)]
    pub quick: bool,

    /// Use a pre-built template (web-app, network, domain, quick-scan)
    #[arg(short, long)]
    pub template: Option<String>,

    /// Pre-fill target
    #[arg(long)]
    pub target: Option<String>,

    /// Load a saved plan file
    #[arg(short, long)]
    pub load: Option<String>,

    /// Resume interrupted plan
    #[arg(short, long)]
    pub resume: bool,

    /// Export plan to file without executing
    #[arg(short, long)]
    pub export: Option<String>,

    /// Execute plan from file (non-interactive compatible)
    #[arg(short = 'x', long)]
    pub execute: Option<String>,

    /// Run in non-interactive mode (requires --execute)
    #[arg(short = 'n', long)]
    pub non_interactive: bool,

    /// List available templates
    #[arg(long)]
    pub list_templates: bool,
}

impl WizardCommands {
    /// Convert to WizardArgs for the wizard implementation
    pub fn to_wizard_args(&self) -> WizardArgs {
        WizardArgs {
            quick: self.quick,
            template: self.template.clone(),
            target: self.target.clone(),
            load: self.load.clone(),
            resume: self.resume,
            export: self.export.clone(),
            execute: self.execute.clone(),
            non_interactive: self.non_interactive,
        }
    }
}

/// Wizard command handler
pub struct WizardCommandHandler;

impl Default for WizardCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl WizardCommandHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn describe() -> &'static str {
        "Interactive attack wizard for guided penetration testing"
    }

    /// Create module registry with all available modules
    fn create_registry() -> ModuleRegistry {
        let mut registry = ModuleRegistry::new();

        // Scanner modules
        registry.register(Box::new(PortScanner::new()));
        registry.register(Box::new(HttpScanner::new()));

        // Recon modules
        registry.register(Box::new(SubdomainEnum::new()));
        registry.register(Box::new(DnsEnumerator::new()));
        registry.register(Box::new(WhoisLookup::new()));
        registry.register(Box::new(AsnDiscovery::new()));

        registry
    }

    /// Run the wizard command
    pub async fn run(&self, cmd: WizardCommands) -> Result<()> {
        // Handle --list-templates
        if cmd.list_templates {
            list_templates();
            return Ok(());
        }

        // Create registry for wizard
        let registry = Arc::new(Mutex::new(Self::create_registry()));

        // Run the wizard
        let args = cmd.to_wizard_args();
        let mut wizard = AttackWizard::new(registry).with_args(args);
        wizard.run().await
    }
}
