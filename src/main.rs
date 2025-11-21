use anyhow::Result;
use clap::Parser;
use ferox::cli::app::FeroxCli;
use ferox::cli::theme::Theme;
use ferox::cli::{CommandRouter, RouterCommand, RouterDispatch};
use ferox::core::module::ModuleRegistry;
use ferox::core::theme::MixedPredatorTheme;
use ferox::modules::exploit::example::ExampleExploit;
use ferox::modules::recon::asn::AsnDiscovery;
use ferox::modules::recon::dns::DnsEnumerator;
use ferox::modules::recon::subdomains::SubdomainEnum;
use ferox::modules::recon::whois::WhoisLookup;
use ferox::modules::scanner::http_scanner::HttpScanner;
use ferox::modules::scanner::port::PortScanner;
use ferox::tools::theme::{CliThemeApplier, ThemeConfig};
use std::io::{stdin, stdout, IsTerminal};

// Phase 3 modules
use ferox::modules::auxiliary::cloud::onedrive_sync_exfil::OneDriveSyncExfil;
use ferox::modules::c2::teams_tunnel::TeamsTunnel;
use ferox::modules::evasion::edr::silent_shadow::SilentShadow;
use ferox::modules::post::browser::deep_session_hijack::DeepSessionHijack;

// Phase 4 modules - Smart Payload System
use ferox::modules::payloads::rev_tcp_fileless::FilelessRevTcp;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Ferox Offensive Security Framework",
    propagate_version = true
)]
struct FeroxArgs {
    #[command(subcommand)]
    command: Option<RouterCommand>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = FeroxArgs::parse();

    let theme = ThemeConfig::load_default()
        .map(|cfg| cfg.into_theme())
        .unwrap_or_else(|_| MixedPredatorTheme::new());
    theme.apply_to_ui();
    let cli_theme = CliThemeApplier::new(theme.clone());
    cli_theme.apply_colors();

    let router = CommandRouter::initialize(cli_theme).await?;

    tracing_subscriber::fmt()
        .with_env_filter("ferox=info")
        .init();

    #[cfg(windows)]
    {
        Theme::enable_ansi_support()?;
    }

    Theme::init();

    match router.dispatch(args.command).await? {
        RouterDispatch::Handled => return Ok(()),
        RouterDispatch::Fallthrough => {}
    }

    // Gracefully handle non-interactive environments (CI/pipes) without dropping into the console.
    if !stdin().is_terminal() || !stdout().is_terminal() {
        println!(
            "Ferox console requires an interactive TTY. \
             Use subcommands like `ferox doctor <cmd>` or `ferox memory <cmd>` in non-interactive modes."
        );
        return Ok(());
    }

    let mut registry = ModuleRegistry::new();

    registry.register(Box::new(PortScanner::new()));
    registry.register(Box::new(HttpScanner::new()));

    registry.register(Box::new(SubdomainEnum::new()));
    registry.register(Box::new(DnsEnumerator::new()));
    registry.register(Box::new(WhoisLookup::new()));
    registry.register(Box::new(AsnDiscovery::new()));

    registry.register(Box::new(ExampleExploit::new()));

    registry.register(Box::new(TeamsTunnel::new()));
    registry.register(Box::new(DeepSessionHijack::new()));
    registry.register(Box::new(OneDriveSyncExfil::new()));
    registry.register(Box::new(SilentShadow::new()));

    // Phase 4: Smart Payload System modules
    registry.register(Box::new(FilelessRevTcp::new()));

    let mut app = FeroxCli::new(registry)?;
    app.run().await?;

    Ok(())
}
