use anyhow::Result;
use ferox::cli::app::FeroxCli;
use ferox::cli::theme::Theme;
use ferox::core::module::ModuleRegistry;
use ferox::modules::exploit::example::ExampleExploit;
use ferox::modules::recon::asn::AsnDiscovery;
use ferox::modules::recon::dns::DnsEnumerator;
use ferox::modules::recon::subdomains::SubdomainEnum;
use ferox::modules::recon::whois::WhoisLookup;
use ferox::modules::scanner::http_scanner::HttpScanner;
use ferox::modules::scanner::port::PortScanner;

// Phase 3 modules
use ferox::modules::c2::teams_tunnel::TeamsTunnel;
use ferox::modules::post::browser::deep_session_hijack::DeepSessionHijack;
use ferox::modules::auxiliary::cloud::onedrive_sync_exfil::OneDriveSyncExfil;
use ferox::modules::evasion::edr::silent_shadow::SilentShadow;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("ferox=info")
        .init();

    #[cfg(windows)]
    {
        Theme::enable_ansi_support()?;
    }

    Theme::init();

    // Create module registry
    let mut registry = ModuleRegistry::new();

    // Register scanner modules
    registry.register(Box::new(PortScanner::new()));
    registry.register(Box::new(HttpScanner::new()));

    // Register recon modules
    registry.register(Box::new(SubdomainEnum::new()));
    registry.register(Box::new(DnsEnumerator::new()));
    registry.register(Box::new(WhoisLookup::new()));
    registry.register(Box::new(AsnDiscovery::new()));

    // Register exploit modules (safe skeletons)
    registry.register(Box::new(ExampleExploit::new()));

    // Register Phase 3 modules
    // C2 modules
    registry.register(Box::new(TeamsTunnel::new()));

    // Post-exploitation modules
    registry.register(Box::new(DeepSessionHijack::new()));

    // Auxiliary modules
    registry.register(Box::new(OneDriveSyncExfil::new()));

    // Evasion modules
    registry.register(Box::new(SilentShadow::new()));

    // Create and run Ferox CLI
    let mut app = FeroxCli::new(registry)?;
    app.run().await?;

    Ok(())
}
