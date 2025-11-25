//! Test PDF export functionality
//!
//! Run with: cargo run --example test_pdf_export --features pdf-export

use ferox::core::module::{ModuleInfo, ModuleResult, ModuleType, Platform, Session};
#[cfg(feature = "pdf-export")]
use ferox::core::reporter::PdfReporter;
use ferox::core::reporter::{HtmlReporter, JsonReporter, ReportData, Reporter};
use ferox::core::result_store::StoredResult;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;
use chrono::Utc;

fn main() {
    let report_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/Users/mansour/Desktop/ferox-reports-20251125-162108".to_string());

    println!("Generating test reports to: {}", report_dir);

    // Create test data simulating a port scan
    let results = vec![
        create_result("port_scanner", "scanner", true, "Port 8080 open on 127.0.0.1 (HTTP)"),
        create_result("port_scanner", "scanner", true, "Port 8888 open on 127.0.0.1 (HTTP)"),
        create_result("port_scanner", "scanner", true, "Port 3000 open on 127.0.0.1 (HTTP)"),
        create_result("port_scanner", "scanner", false, "Port 22 closed on 127.0.0.1"),
        create_result("port_scanner", "scanner", false, "Port 80 closed on 127.0.0.1"),
        create_result("port_scanner", "scanner", false, "Port 443 closed on 127.0.0.1"),
        create_result("port_scanner", "scanner", false, "Port 5000 closed on 127.0.0.1"),
    ];

    let now = Utc::now();
    let sessions = vec![
        Session {
            id: Uuid::new_v4(),
            module: "scanner/port_scanner".to_string(),
            target: "127.0.0.1".to_string(),
            platform: Platform::Linux,
            established_at: now,
            last_seen: now,
            active: true,
            user: Some("tester".to_string()),
            metadata: HashMap::new(),
        },
    ];

    let report_data = ReportData::new(results, sessions);

    // Export JSON
    let json_path = format!("{}/scan-test.json", report_dir);
    match JsonReporter.export(&report_data, Path::new(&json_path)) {
        Ok(_) => println!("✓ JSON exported: {}", json_path),
        Err(e) => println!("✗ JSON export failed: {}", e),
    }

    // Export HTML
    let html_path = format!("{}/scan-test.html", report_dir);
    match HtmlReporter.export(&report_data, Path::new(&html_path)) {
        Ok(_) => println!("✓ HTML exported: {}", html_path),
        Err(e) => println!("✗ HTML export failed: {}", e),
    }

    // Export PDF
    #[cfg(feature = "pdf-export")]
    {
        let pdf_path = format!("{}/scan-test.pdf", report_dir);
        match PdfReporter.export(&report_data, Path::new(&pdf_path)) {
            Ok(_) => println!("✓ PDF exported: {}", pdf_path),
            Err(e) => println!("✗ PDF export failed: {}", e),
        }
    }
    #[cfg(not(feature = "pdf-export"))]
    {
        println!("⚠ PDF export disabled (compile with --features pdf-export)");
    }

    println!("\nDone! Check the reports in: {}", report_dir);
}

fn create_result(name: &str, category: &str, success: bool, message: &str) -> StoredResult {
    StoredResult {
        id: Uuid::new_v4(),
        module_info: ModuleInfo {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "TCP port scanner module - scans for open ports on target hosts".to_string(),
            module_type: ModuleType::Scanner,
            category: category.to_string(),
        },
        result: if success {
            ModuleResult::success(message)
        } else {
            ModuleResult::error(message)
        },
    }
}
