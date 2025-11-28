//! Subdomain Enumeration Test
//!
//! Tests the subdomain enumeration module on example.com (authorized)

use anyhow::Result;
use ferox::core::module::Module;
use ferox::modules::recon::subdomains::SubdomainEnum;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║              SUBDOMAIN ENUMERATION TEST on example.com                       ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    let start = Instant::now();

    let mut subdomain_enum = SubdomainEnum::new();
    subdomain_enum.set_option("RHOSTS", "example.com")?;
    subdomain_enum.set_option("WORDLIST", "./test_wordlist.txt")?;
    subdomain_enum.set_option("THREADS", "10")?;
    subdomain_enum.set_option("TIMEOUT", "3000")?;
    subdomain_enum.set_option("PROBE_HTTP", "true")?;
    subdomain_enum.set_option("OUTPUT", "human")?;

    println!("\nOptions set:");
    for opt in subdomain_enum.options() {
        println!("  {} = {:?}", opt.name, opt.current_value);
    }

    println!("\nValidating options...");
    if let Err(e) = subdomain_enum.validate() {
        println!("❌ Validation failed: {}", e);
        return Ok(());
    }
    println!("✅ Validation passed\n");

    println!("Running subdomain enumeration...\n");

    match tokio::time::timeout(
        std::time::Duration::from_secs(60),
        subdomain_enum.run()
    ).await {
        Ok(Ok(result)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n✅ SUCCESS in {}ms", elapsed);
            println!("Message: {}", result.message);

            if let Some(found) = result.data.get("subdomains_found") {
                println!("\nSubdomains found: {}", found);
            }
            if let Some(subdomains) = result.data.get("subdomains") {
                println!("\nSubdomain details:");
                println!("{}", serde_json::to_string_pretty(subdomains).unwrap_or_default());
            }
        }
        Ok(Err(e)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n❌ ERROR in {}ms: {}", elapsed, e);
        }
        Err(_) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n⏰ TIMEOUT after {}ms", elapsed);
        }
    }

    Ok(())
}
