use anyhow::Result;
use clap::{Parser, Subcommand};
use ferox::cli::app::FeroxCli;
use ferox::cli::doctor::{DoctorCommands, handle_doctor_command};
#[cfg(feature = "memory-forensics")]
use ferox::cli::memory::MemoryCli;
use ferox::cli::theme::Theme;
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

// Phase 3 modules
use ferox::modules::auxiliary::cloud::onedrive_sync_exfil::OneDriveSyncExfil;
use ferox::modules::c2::teams_tunnel::TeamsTunnel;
use ferox::modules::evasion::edr::silent_shadow::SilentShadow;
use ferox::modules::post::browser::deep_session_hijack::DeepSessionHijack;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Ferox Offensive Security Framework",
    propagate_version = true
)]
struct FeroxArgs {
    #[command(subcommand)]
    command: Option<FeroxCommand>,
}

#[derive(Subcommand, Debug)]
enum FeroxCommand {
    /// Run Ferox Doctor checks without launching the interactive console
    #[command(subcommand)]
    Doctor(DoctorCommands),
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

    tracing_subscriber::fmt()
        .with_env_filter("ferox=info")
        .init();

    #[cfg(windows)]
    {
        Theme::enable_ansi_support()?;
    }

    Theme::init();

    if let Some(command) = args.command {
        match command {
            FeroxCommand::Doctor(cmd) => {
                handle_doctor_command(cmd, &cli_theme)?;
                return Ok(());
            }
        }
    }

    #[cfg(feature = "memory-forensics")]
    {
        let mut args = std::env::args().skip(1);
        if let Some(first) = args.next() {
            if first.eq_ignore_ascii_case("memory") {
                let remaining: Vec<String> = args.collect();
                let ref_args: Vec<&str> = remaining.iter().map(|s| s.as_str()).collect();
                MemoryCli::handle(&ref_args)?;
                return Ok(());
            }
        }
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

    let mut app = FeroxCli::new(registry)?;
    app.run().await?;

    Ok(())
}
