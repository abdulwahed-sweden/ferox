mod cli;
mod core;
mod modules;

use anyhow::Result;
use cli::app::FeroxCli;
use cli::theme::Theme;
use core::module::ModuleRegistry;
use modules::exploit::example::ExampleExploit;
use modules::recon::subdomains::SubdomainEnum;
use modules::recon::dns::DnsEnumerator;
use modules::recon::whois::WhoisLookup;
use modules::recon::asn::AsnDiscovery;
use modules::scanner::port::PortScanner;
use modules::scanner::http_scanner::HttpScanner;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("ferox=info")
        .init();

    #[cfg(windows)]
    {
        cli::theme::enable_ansi_support()?;
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

    // Create and run Ferox CLI
    let mut app = FeroxCli::new(registry)?;
    app.run().await?;

    Ok(())
}
