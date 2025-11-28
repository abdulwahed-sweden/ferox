//! Module Testing Binary
//!
//! Tests Ferox modules against authorized targets:
//! - scanme.nmap.org (Nmap Project - explicitly authorized for scanning)
//! - testphp.vulnweb.com (Acunetix - intentionally vulnerable)
//! - example.com (IANA - reserved for documentation)
//! - 8.8.8.8 (Google DNS - public information)

use anyhow::Result;
use ferox::core::module::Module;
use ferox::modules::recon::asn::AsnDiscovery;
use ferox::modules::recon::dns::DnsEnumerator;
use ferox::modules::recon::whois::WhoisLookup;
use ferox::modules::scanner::http_scanner::HttpScanner;
use ferox::modules::scanner::port::PortScanner;
use std::time::Instant;

fn print_separator() {
    println!("\n{}", "=".repeat(80));
}

fn print_test_header(name: &str, target: &str) {
    print_separator();
    println!("TEST: {} on {}", name, target);
    println!("{}", "-".repeat(80));
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                    FEROX MODULE TESTING SUITE                                ║");
    println!("║                                                                              ║");
    println!("║  Testing against AUTHORIZED targets only:                                   ║");
    println!("║  - scanme.nmap.org (Nmap Project - authorized for scanning)                 ║");
    println!("║  - testphp.vulnweb.com (Acunetix - intentionally vulnerable)                ║");
    println!("║  - example.com (IANA - reserved for documentation)                          ║");
    println!("║  - 8.8.8.8 (Google DNS - public information)                                ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    let mut results: Vec<(&str, &str, bool, String, u128)> = Vec::new();

    // ═══════════════════════════════════════════════════════════════════════════════
    // TEST 1: Port Scanner on scanme.nmap.org
    // ═══════════════════════════════════════════════════════════════════════════════
    print_test_header("Port Scanner", "scanme.nmap.org");
    let start = Instant::now();

    let mut port_scanner = PortScanner::new();
    port_scanner.set_option("RHOSTS", "scanme.nmap.org")?;
    port_scanner.set_option("PORTS", "22,80,443,9929,31337")?;
    port_scanner.set_option("TIMEOUT", "3000")?;
    port_scanner.set_option("THREADS", "5")?;

    println!("Options set:");
    for opt in port_scanner.options() {
        println!("  {} = {:?}", opt.name, opt.current_value);
    }
    println!("\nRunning scan...");

    match tokio::time::timeout(
        std::time::Duration::from_secs(30),
        port_scanner.run()
    ).await {
        Ok(Ok(result)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n✅ SUCCESS in {}ms", elapsed);
            println!("Message: {}", result.message);
            if let Some(ports) = result.data.get("open_ports") {
                println!("Open ports: {}", ports);
            }
            results.push(("port_scanner", "scanme.nmap.org", true, result.message.clone(), elapsed));
        }
        Ok(Err(e)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n❌ ERROR in {}ms: {}", elapsed, e);
            results.push(("port_scanner", "scanme.nmap.org", false, e.to_string(), elapsed));
        }
        Err(_) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n⏰ TIMEOUT after {}ms", elapsed);
            results.push(("port_scanner", "scanme.nmap.org", false, "Timeout after 30s".to_string(), elapsed));
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // TEST 2: HTTP Scanner on testphp.vulnweb.com
    // ═══════════════════════════════════════════════════════════════════════════════
    print_test_header("HTTP Scanner", "testphp.vulnweb.com");
    let start = Instant::now();

    let mut http_scanner = HttpScanner::new();
    http_scanner.set_option("RHOSTS", "http://testphp.vulnweb.com")?;
    http_scanner.set_option("TIMEOUT", "10000")?;

    println!("Options set:");
    for opt in http_scanner.options() {
        if opt.current_value.is_some() {
            println!("  {} = {:?}", opt.name, opt.current_value);
        }
    }
    println!("\nRunning scan...");

    match tokio::time::timeout(
        std::time::Duration::from_secs(30),
        http_scanner.run()
    ).await {
        Ok(Ok(result)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n✅ SUCCESS in {}ms", elapsed);
            println!("Message: {}", result.message);
            if let Some(results_data) = result.data.get("results") {
                if let Some(arr) = results_data.as_array() {
                    for item in arr {
                        if let Some(status) = item.get("status") {
                            println!("  Status: {}", status);
                        }
                        if let Some(server) = item.get("server") {
                            println!("  Server: {}", server);
                        }
                        if let Some(techs) = item.get("technologies") {
                            println!("  Technologies: {}", techs);
                        }
                    }
                }
            }
            results.push(("http_scanner", "testphp.vulnweb.com", true, result.message.clone(), elapsed));
        }
        Ok(Err(e)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n❌ ERROR in {}ms: {}", elapsed, e);
            results.push(("http_scanner", "testphp.vulnweb.com", false, e.to_string(), elapsed));
        }
        Err(_) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n⏰ TIMEOUT after {}ms", elapsed);
            results.push(("http_scanner", "testphp.vulnweb.com", false, "Timeout after 30s".to_string(), elapsed));
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // TEST 3: DNS Enumeration on example.com
    // ═══════════════════════════════════════════════════════════════════════════════
    print_test_header("DNS Enumeration", "example.com");
    let start = Instant::now();

    let mut dns_enum = DnsEnumerator::new();
    dns_enum.set_option("TARGET", "example.com")?;
    dns_enum.set_option("RECORD_TYPES", "A,AAAA,MX,NS,TXT")?;
    dns_enum.set_option("TIMEOUT", "5")?;

    println!("Options set:");
    for opt in dns_enum.options() {
        if opt.current_value.is_some() {
            println!("  {} = {:?}", opt.name, opt.current_value);
        }
    }
    println!("\nRunning enumeration...");

    match tokio::time::timeout(
        std::time::Duration::from_secs(30),
        dns_enum.run()
    ).await {
        Ok(Ok(result)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n✅ SUCCESS in {}ms", elapsed);
            println!("Message: {}", result.message);
            if let Some(records) = result.data.get("dns_records") {
                println!("DNS Records: {}", serde_json::to_string_pretty(records).unwrap_or_default());
            }
            results.push(("dns_enum", "example.com", true, result.message.clone(), elapsed));
        }
        Ok(Err(e)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n❌ ERROR in {}ms: {}", elapsed, e);
            results.push(("dns_enum", "example.com", false, e.to_string(), elapsed));
        }
        Err(_) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n⏰ TIMEOUT after {}ms", elapsed);
            results.push(("dns_enum", "example.com", false, "Timeout after 30s".to_string(), elapsed));
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // TEST 4: WHOIS Lookup on example.com
    // ═══════════════════════════════════════════════════════════════════════════════
    print_test_header("WHOIS Lookup", "example.com");
    let start = Instant::now();

    let mut whois = WhoisLookup::new();
    whois.set_option("TARGET", "example.com")?;
    whois.set_option("TIMEOUT", "10")?;

    println!("Options set:");
    for opt in whois.options() {
        if opt.current_value.is_some() {
            println!("  {} = {:?}", opt.name, opt.current_value);
        }
    }
    println!("\nRunning lookup...");

    match tokio::time::timeout(
        std::time::Duration::from_secs(30),
        whois.run()
    ).await {
        Ok(Ok(result)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n✅ SUCCESS in {}ms", elapsed);
            println!("Message: {}", result.message);
            if let Some(data) = result.data.get("whois_data") {
                println!("WHOIS Data: {}", serde_json::to_string_pretty(data).unwrap_or_default());
            }
            results.push(("whois_lookup", "example.com", true, result.message.clone(), elapsed));
        }
        Ok(Err(e)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n❌ ERROR in {}ms: {}", elapsed, e);
            results.push(("whois_lookup", "example.com", false, e.to_string(), elapsed));
        }
        Err(_) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n⏰ TIMEOUT after {}ms", elapsed);
            results.push(("whois_lookup", "example.com", false, "Timeout after 30s".to_string(), elapsed));
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // TEST 5: ASN Discovery on 8.8.8.8
    // ═══════════════════════════════════════════════════════════════════════════════
    print_test_header("ASN Discovery", "8.8.8.8");
    let start = Instant::now();

    let mut asn = AsnDiscovery::new();
    asn.set_option("TARGET", "8.8.8.8")?;
    asn.set_option("TIMEOUT", "10")?;

    println!("Options set:");
    for opt in asn.options() {
        if opt.current_value.is_some() {
            println!("  {} = {:?}", opt.name, opt.current_value);
        }
    }
    println!("\nRunning discovery...");

    match tokio::time::timeout(
        std::time::Duration::from_secs(30),
        asn.run()
    ).await {
        Ok(Ok(result)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n✅ SUCCESS in {}ms", elapsed);
            println!("Message: {}", result.message);
            if let Some(info) = result.data.get("asn_info") {
                println!("ASN Info: {}", serde_json::to_string_pretty(info).unwrap_or_default());
            }
            if let Some(details) = result.data.get("asn_details") {
                println!("ASN Details: {}", serde_json::to_string_pretty(details).unwrap_or_default());
            }
            results.push(("asn_discovery", "8.8.8.8", true, result.message.clone(), elapsed));
        }
        Ok(Err(e)) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n❌ ERROR in {}ms: {}", elapsed, e);
            results.push(("asn_discovery", "8.8.8.8", false, e.to_string(), elapsed));
        }
        Err(_) => {
            let elapsed = start.elapsed().as_millis();
            println!("\n⏰ TIMEOUT after {}ms", elapsed);
            results.push(("asn_discovery", "8.8.8.8", false, "Timeout after 30s".to_string(), elapsed));
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════════════════════════
    print_separator();
    println!("\n📊 TEST RESULTS SUMMARY\n");
    println!("{:<20} {:<25} {:<10} {:<10} {}", "Module", "Target", "Status", "Time(ms)", "Details");
    println!("{}", "-".repeat(100));

    let mut passed = 0;
    let mut failed = 0;

    for (module, target, success, details, time) in &results {
        let status = if *success {
            passed += 1;
            "✅ PASS"
        } else {
            failed += 1;
            "❌ FAIL"
        };
        let short_details = if details.len() > 40 {
            format!("{}...", &details[..40])
        } else {
            details.clone()
        };
        println!("{:<20} {:<25} {:<10} {:<10} {}", module, target, status, time, short_details);
    }

    println!("{}", "-".repeat(100));
    println!("\nTotal: {} passed, {} failed out of {} tests", passed, failed, results.len());

    if failed > 0 {
        println!("\n⚠️  Some tests failed. See details above.");
    } else {
        println!("\n🎉 All tests passed!");
    }

    Ok(())
}
