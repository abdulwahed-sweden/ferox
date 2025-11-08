mod cli;
mod core;
mod modules;

use anyhow::Result;
use cli::app::FeroxCli;
use cli::theme::Theme;
use core::module::ModuleRegistry;
use modules::exploit::example::ExampleExploit;
use modules::recon::subdomains::SubdomainEnum;
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

    // Register exploit modules (safe skeletons)
    registry.register(Box::new(ExampleExploit::new()));

    // TODO: Add more modules here as they are developed
    // registry.register(Box::new(HttpScanner::new()));
    // registry.register(Box::new(DnsEnum::new()));

    // Create and run Ferox CLI
    let mut app = FeroxCli::new(registry)?;
    app.run().await?;

    Ok(())
}
