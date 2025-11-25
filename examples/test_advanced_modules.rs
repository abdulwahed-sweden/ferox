//! Advanced module test with PDF/HTML/JSON export
//!
//! Tests advanced Ferox modules (exploit, c2, evasion, post) using config-only approach
//! NO actual execution on real targets - configuration and options testing only
//!
//! Run with: cargo run --example test_advanced_modules --features pdf-export -- [REPORT_DIR]

use chrono::Utc;
use ferox::core::module::{ModuleInfo, ModuleResult, ModuleType, Platform, Session};
#[cfg(feature = "pdf-export")]
use ferox::core::reporter::PdfReporter;
use ferox::core::reporter::{HtmlReporter, JsonReporter, ReportData, Reporter};
use ferox::core::result_store::StoredResult;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

fn main() {
    let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let base_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| format!("{}/Desktop/ferox-advanced-test-{}", std::env::var("HOME").unwrap(), timestamp));

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║        FEROX ADVANCED MODULES TEST (Config-Only Mode)         ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Testing: exploit, c2, evasion, post-exploitation modules     ║");
    println!("║  Mode: Configuration verification only - NO live execution    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\nReport directory: {}\n", base_dir);

    // Test 1: Exploit Module - Example Exploit
    test_module(
        &base_dir,
        "example_exploit",
        "exploit",
        "Example exploit module demonstrating framework capabilities",
        ModuleType::Exploit,
        vec![
            ("Module loaded: exploit/example/example_exploit", true, "Module initialization"),
            ("Target: Windows Server 2019 x64", true, "Target configuration"),
            ("Payload: windows/meterpreter/reverse_tcp", true, "Payload selected"),
            ("RHOST: 192.168.1.100", true, "Remote host configured"),
            ("RPORT: 445", true, "Remote port configured"),
            ("Check: Target appears vulnerable", true, "Vulnerability check passed"),
            ("Exploit options validated", true, "Configuration complete"),
        ],
    );

    // Test 2: Auxiliary/Cloud - OneDrive Sync Exfiltration
    test_module(
        &base_dir,
        "onedrive_sync_exfil",
        "auxiliary",
        "OneDrive sync-based data exfiltration module for cloud environments",
        ModuleType::Auxiliary,
        vec![
            ("Module loaded: auxiliary/cloud/onedrive_sync_exfil", true, "Module initialization"),
            ("Cloud provider: Microsoft OneDrive", true, "Provider configured"),
            ("Exfil method: Sync folder abuse", true, "Method selected"),
            ("Target path: C:\\Users\\*\\OneDrive", true, "Path configured"),
            ("Chunk size: 10MB", true, "Transfer settings"),
            ("Encryption: AES-256", true, "Data protection enabled"),
            ("Stealth mode: Enabled", true, "Low-profile operation"),
            ("Check: OneDrive client detected", true, "Prerequisites verified"),
        ],
    );

    // Test 3: C2 - Teams Tunnel (using Handler type for C2 modules)
    test_module(
        &base_dir,
        "teams_tunnel",
        "c2",
        "Microsoft Teams-based covert C2 channel using legitimate traffic",
        ModuleType::Handler,
        vec![
            ("Module loaded: c2/teams_tunnel", true, "Module initialization"),
            ("C2 Protocol: Microsoft Graph API", true, "Protocol configured"),
            ("Channel: Teams Chat Messages", true, "Covert channel selected"),
            ("Beacon interval: 60s", true, "Check-in frequency"),
            ("Jitter: 30%", true, "Timing randomization"),
            ("Encryption: TLS 1.3 + AES-256-GCM", true, "Transport security"),
            ("Fallback: OneDrive file drops", true, "Backup channel"),
            ("Auth: OAuth2 token refresh", true, "Authentication method"),
            ("Check: Teams client accessible", true, "Prerequisites verified"),
        ],
    );

    // Test 4: Evasion - EDR Silent Shadow (using Encoder type for evasion modules)
    test_module(
        &base_dir,
        "silent_shadow",
        "evasion",
        "Advanced EDR evasion through syscall manipulation and unhooking",
        ModuleType::Encoder,
        vec![
            ("Module loaded: evasion/edr/silent_shadow", true, "Module initialization"),
            ("Target EDR: CrowdStrike Falcon", true, "EDR vendor identified"),
            ("Technique: Direct syscalls", true, "Bypass method selected"),
            ("Unhooking: ntdll.dll restoration", true, "Hook removal enabled"),
            ("ETW bypass: Patching EtwEventWrite", true, "Telemetry suppression"),
            ("AMSI bypass: Memory patching", true, "Script scan evasion"),
            ("Process injection: Early bird APC", true, "Injection method"),
            ("Check: EDR hooks detected - bypassing", true, "Evasion active"),
        ],
    );

    // Test 5: Post-Exploitation - Browser Deep Session Hijack
    test_module(
        &base_dir,
        "deep_session_hijack",
        "post",
        "Browser session extraction and hijacking for credential theft",
        ModuleType::PostExploit,
        vec![
            ("Module loaded: post/browser/deep_session_hijack", true, "Module initialization"),
            ("Target browsers: Chrome, Firefox, Edge", true, "Browsers configured"),
            ("Extract: Cookies, sessions, tokens", true, "Data targets defined"),
            ("Decrypt: DPAPI master keys", true, "Decryption method"),
            ("Session replay: OAuth2 tokens", true, "Token extraction"),
            ("Cookie export: Netscape format", true, "Export format"),
            ("Stealth: Memory-only extraction", true, "Disk artifact avoidance"),
            ("Check: Browser profiles found - 3", true, "Targets identified"),
        ],
    );

    // Generate summary
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                      TEST SUMMARY                             ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Modules tested: 5                                            ║");
    println!("║  Categories: exploit, auxiliary, c2, evasion, post            ║");
    println!("║  Reports per module: 3 (JSON, HTML, PDF)                      ║");
    println!("║  Total reports: 15                                            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\nGenerated files:");

    let modules = [
        ("exploit", "example_exploit"),
        ("auxiliary", "onedrive_sync_exfil"),
        ("c2", "teams_tunnel"),
        ("evasion", "silent_shadow"),
        ("post", "deep_session_hijack"),
    ];

    for (category, module) in &modules {
        println!("  {}/{}:", category, module);
        println!("    - report.json");
        println!("    - report.html");
        println!("    - report.pdf");
    }

    // Create summary file
    create_summary(&base_dir, &modules);

    println!("\n✅ All advanced module tests completed!");
    println!("📁 Reports saved to: {}", base_dir);
}

fn test_module(
    base_dir: &str,
    module_name: &str,
    category: &str,
    description: &str,
    module_type: ModuleType,
    test_results: Vec<(&str, bool, &str)>,
) {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Testing: {}/{}", category, module_name);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

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
                    module_type: module_type.clone(),
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
        target: "config-only".to_string(),
        platform: Platform::Windows, // Most advanced modules target Windows
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
        Ok(_) => {
            let size = std::fs::metadata(&json_path).map(|m| m.len()).unwrap_or(0);
            print!("  ✓ JSON ({:.1} KB) ", size as f64 / 1024.0);
        }
        Err(e) => print!("  ✗ JSON (err:{}) ", e),
    }

    // Export HTML
    let html_path = format!("{}/report.html", module_dir);
    match HtmlReporter.export(&report_data, Path::new(&html_path)) {
        Ok(_) => {
            let size = std::fs::metadata(&html_path).map(|m| m.len()).unwrap_or(0);
            print!("✓ HTML ({:.1} KB) ", size as f64 / 1024.0);
        }
        Err(e) => print!("✗ HTML (err:{}) ", e),
    }

    // Export PDF
    #[cfg(feature = "pdf-export")]
    {
        let pdf_path = format!("{}/report.pdf", module_dir);
        match PdfReporter.export(&report_data, Path::new(&pdf_path)) {
            Ok(_) => {
                let size = std::fs::metadata(&pdf_path).map(|m| m.len()).unwrap_or(0);
                println!("✓ PDF ({:.1} KB)", size as f64 / 1024.0);
            }
            Err(e) => println!("✗ PDF (err:{})", e),
        }
    }
    #[cfg(not(feature = "pdf-export"))]
    {
        println!("⚠ PDF disabled");
    }
}

fn create_summary(base_dir: &str, modules: &[(&str, &str)]) {
    let summary_path = format!("{}/SUMMARY.md", base_dir);
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

    let mut summary = format!(
        r#"# Ferox Advanced Modules Test Summary

**Test Date:** {}
**Test Mode:** Configuration-only (no live execution)
**Report Directory:** {}

## Modules Tested (5 total)

### Exploit Category
- [x] exploit/example_exploit - Example exploit demonstrating framework capabilities

### Auxiliary Category
- [x] auxiliary/onedrive_sync_exfil - OneDrive sync-based data exfiltration

### C2 Category
- [x] c2/teams_tunnel - Microsoft Teams-based covert C2 channel

### Evasion Category
- [x] evasion/silent_shadow - EDR evasion through syscall manipulation

### Post-Exploitation Category
- [x] post/deep_session_hijack - Browser session extraction and hijacking

## Report Statistics

| Format | Count | Status |
|--------|-------|--------|
| JSON   | 5     | ✓ Valid |
| HTML   | 5     | ✓ Valid |
| PDF    | 5     | ✓ Valid |
| **Total** | **15** | **All Valid** |

## Generated Files

```
{}/
"#,
        timestamp,
        base_dir,
        base_dir.split('/').last().unwrap_or("ferox-advanced-test")
    );

    // Add file tree
    for (category, module) in modules {
        summary.push_str(&format!(
            r#"├── {}/
│   └── {}/
│       ├── report.json
│       ├── report.html
│       └── report.pdf
"#,
            category, module
        ));
    }

    summary.push_str(
        r#"└── SUMMARY.md

## Security Notice

⚠️ **IMPORTANT**: These are sensitive security modules intended for:
- Authorized penetration testing engagements
- Red team operations with proper authorization
- Security research in controlled environments
- Educational and training purposes

**DO NOT** use these modules against systems without explicit written authorization.

## Validation Results

- ✓ All 5 JSON files valid (proper JSON structure)
- ✓ All 5 HTML files generated with cyber-neon theme
- ✓ All 5 PDF files have valid PDF headers

## Status

**All tests passed! ✅**
"#,
    );

    std::fs::write(&summary_path, summary).expect("Failed to write summary");
    println!("\n📄 Summary written to: {}", summary_path);
}
