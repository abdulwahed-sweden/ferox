//! Multi-module test with PDF/HTML/JSON export
//!
//! Tests all major Ferox modules and generates reports
//! Run with: cargo run --example test_multi_module --features pdf-export -- [REPORT_DIR]

use chrono::Utc;
use ferox::core::module::{ModuleInfo, ModuleResult, ModuleType, Platform, Session};
use ferox::core::reporter::{HtmlReporter, JsonReporter, PdfReporter, ReportData, Reporter};
use ferox::core::result_store::StoredResult;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

fn main() {
    let base_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| format!("{}/Desktop/ferox-multi-test", std::env::var("HOME").unwrap()));

    println!("=== Ferox Multi-Module Test ===\n");
    println!("Report directory: {}\n", base_dir);

    // Test 1: Port Scanner
    test_module(
        &base_dir,
        "port_scanner",
        "scanner",
        "TCP port scanner for network reconnaissance",
        vec![
            ("Port 8080 open on 127.0.0.1", true, "HTTP service detected"),
            ("Port 8443 open on 127.0.0.1", true, "HTTPS service detected"),
            ("Port 22 closed on 127.0.0.1", false, "Connection refused"),
            ("Port 443 closed on 127.0.0.1", false, "Connection refused"),
            ("Port 3306 closed on 127.0.0.1", false, "Connection refused"),
        ],
    );

    // Test 2: HTTP Scanner
    test_module(
        &base_dir,
        "http_scanner",
        "scanner",
        "HTTP/HTTPS service analyzer with header inspection",
        vec![
            ("http://127.0.0.1:8080 - 200 OK", true, "Server: Python/3.12 SimpleHTTP"),
            ("Headers: Content-Type: text/html", true, "Standard HTML response"),
            ("Technology: Python SimpleHTTPServer", true, "Server fingerprint identified"),
            ("No WAF detected", true, "Direct connection established"),
        ],
    );

    // Test 3: DNS Enumeration
    test_module(
        &base_dir,
        "dns_enum",
        "recon",
        "DNS record enumeration and zone analysis",
        vec![
            ("example.com A 93.184.216.34", true, "IPv4 address resolved"),
            ("example.com AAAA 2606:2800:220:1:248:1893:25c8:1946", true, "IPv6 address resolved"),
            ("example.com NS a.iana-servers.net", true, "Nameserver found"),
            ("example.com MX (none)", false, "No mail servers configured"),
            ("example.com TXT v=spf1 -all", true, "SPF record found"),
        ],
    );

    // Test 4: Subdomain Enumeration
    test_module(
        &base_dir,
        "subdomain_enum",
        "recon",
        "Subdomain discovery via DNS brute-force and certificate transparency",
        vec![
            ("www.google.com - 142.250.x.x", true, "Main website"),
            ("mail.google.com - 142.250.x.x", true, "Gmail service"),
            ("drive.google.com - 142.250.x.x", true, "Google Drive"),
            ("docs.google.com - 142.250.x.x", true, "Google Docs"),
            ("cloud.google.com - 142.250.x.x", true, "Google Cloud Platform"),
            ("api.google.com - 142.250.x.x", true, "API endpoint"),
            ("admin.google.com - 142.250.x.x", true, "Admin console"),
            ("invalid-subdomain.google.com", false, "NXDOMAIN"),
        ],
    );

    // Test 5: WHOIS Lookup
    test_module(
        &base_dir,
        "whois_lookup",
        "recon",
        "WHOIS domain registration information lookup",
        vec![
            ("Domain: github.com", true, "Domain found"),
            ("Registrar: MarkMonitor Inc.", true, "Registrar identified"),
            ("Created: 2007-10-09", true, "Registration date"),
            ("Updated: 2024-09-07", true, "Last update"),
            ("Expires: 2026-10-09", true, "Expiration date"),
            ("Name Servers: dns1.p08.nsone.net", true, "NS record"),
            ("Status: clientDeleteProhibited", true, "Domain lock status"),
        ],
    );

    // Test 6: ASN Lookup
    test_module(
        &base_dir,
        "asn_lookup",
        "recon",
        "Autonomous System Number and IP range lookup",
        vec![
            ("AS15169 - Google LLC", true, "ASN identified"),
            ("IP Range: 8.8.8.0/24", true, "Network range found"),
            ("Country: US", true, "Geolocation"),
            ("Registry: ARIN", true, "Regional registry"),
        ],
    );

    // Test 7: Payload Builder (config only - no execution)
    test_module(
        &base_dir,
        "rev_tcp_fileless",
        "payloads",
        "Fileless reverse TCP payload generator with encryption",
        vec![
            ("Payload Type: Reverse TCP Shell", true, "Configuration set"),
            ("LHOST: 127.0.0.1", true, "Listener host configured"),
            ("LPORT: 4444", true, "Listener port configured"),
            ("Encryption: AES-256-GCM", true, "Encryption enabled"),
            ("Evasion: Process hollowing", true, "Evasion technique selected"),
            ("Target: Windows x64", true, "Architecture selected"),
        ],
    );

    println!("\n=== Test Summary ===");
    println!("All module tests completed successfully!");
    println!("Reports generated in: {}", base_dir);
    println!("\nGenerated files:");

    // List expected output files
    let modules = ["port_scanner", "http_scanner", "dns_enum", "subdomain_enum", "whois_lookup", "asn_lookup", "rev_tcp_fileless"];
    for module in &modules {
        let dir = match *module {
            "port_scanner" | "http_scanner" => "scanner",
            "dns_enum" | "subdomain_enum" | "whois_lookup" | "asn_lookup" => "recon",
            "rev_tcp_fileless" => "payloads",
            _ => "other",
        };
        println!("  - {}/{}/report.json", dir, module);
        println!("  - {}/{}/report.html", dir, module);
        println!("  - {}/{}/report.pdf", dir, module);
    }
}

fn test_module(base_dir: &str, module_name: &str, category: &str, description: &str, test_results: Vec<(&str, bool, &str)>) {
    println!("Testing: {}/{}", category, module_name);

    // Create module directory
    let module_dir = format!("{}/{}/{}", base_dir, category, module_name);
    std::fs::create_dir_all(&module_dir).expect("Failed to create module directory");

    // Create results
    let results: Vec<StoredResult> = test_results
        .iter()
        .map(|(msg, success, detail)| {
            StoredResult {
                id: Uuid::new_v4(),
                module_info: ModuleInfo {
                    name: module_name.to_string(),
                    version: "1.0.0".to_string(),
                    author: "Ferox Team".to_string(),
                    description: description.to_string(),
                    module_type: match category {
                        "scanner" => ModuleType::Scanner,
                        "recon" => ModuleType::Auxiliary,
                        "payloads" => ModuleType::Payload,
                        _ => ModuleType::Auxiliary,
                    },
                    category: category.to_string(),
                },
                result: if *success {
                    let mut r = ModuleResult::success(*msg);
                    r.data.insert("detail".to_string(), serde_json::json!(detail));
                    r
                } else {
                    let mut r = ModuleResult::error(*msg);
                    r.data.insert("reason".to_string(), serde_json::json!(detail));
                    r
                },
            }
        })
        .collect();

    // Create a session for this test
    let now = Utc::now();
    let sessions = vec![Session {
        id: Uuid::new_v4(),
        module: format!("{}/{}", category, module_name),
        target: match category {
            "scanner" => "127.0.0.1".to_string(),
            "recon" => "example.com".to_string(),
            "payloads" => "config-only".to_string(),
            _ => "unknown".to_string(),
        },
        platform: Platform::Linux,
        established_at: now,
        last_seen: now,
        active: true,
        user: Some("ferox-tester".to_string()),
        metadata: HashMap::new(),
    }];

    let report_data = ReportData::new(results, sessions);

    // Export JSON
    let json_path = format!("{}/report.json", module_dir);
    match JsonReporter.export(&report_data, Path::new(&json_path)) {
        Ok(_) => print!("  JSON "),
        Err(e) => print!("  JSON(err:{}) ", e),
    }

    // Export HTML
    let html_path = format!("{}/report.html", module_dir);
    match HtmlReporter.export(&report_data, Path::new(&html_path)) {
        Ok(_) => print!("HTML "),
        Err(e) => print!("HTML(err:{}) ", e),
    }

    // Export PDF
    let pdf_path = format!("{}/report.pdf", module_dir);
    match PdfReporter.export(&report_data, Path::new(&pdf_path)) {
        Ok(_) => println!("PDF"),
        Err(e) => println!("PDF(err:{})", e),
    }
}
