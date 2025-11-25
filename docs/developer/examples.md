# Test Examples

Ferox includes several example programs demonstrating report generation and module testing capabilities.

## Available Examples

| Example | Purpose | Features Required |
|---------|---------|-------------------|
| `test_pdf_export` | Single module PDF test | `pdf-export` |
| `test_multi_module` | 7 modules across 3 categories | `pdf-export` |
| `test_advanced_modules` | 5 advanced modules (c2, evasion, post) | `pdf-export` |

## Running Examples

```bash
# Single module test
cargo run --example test_pdf_export --features pdf-export

# Multi-module test (7 modules)
cargo run --example test_multi_module --features pdf-export

# Advanced modules test (5 modules)
cargo run --example test_advanced_modules --features pdf-export

# Specify custom output directory
cargo run --example test_multi_module --features pdf-export -- ~/custom/path
```

## test_pdf_export

Basic example demonstrating single-module report generation.

**Location**: `examples/test_pdf_export.rs`

**Output**: `~/Desktop/ferox-reports-{timestamp}/`
- `report.json` - JSON format
- `report.html` - HTML format
- `report.pdf` - PDF format

**Tests**:
- Port scanner module
- Basic result creation
- All three export formats

## test_multi_module

Comprehensive test covering 7 modules across 3 categories.

**Location**: `examples/test_multi_module.rs`

**Output**: `~/Desktop/ferox-multi-test-{timestamp}/`

**Directory Structure**:
```
ferox-multi-test-{timestamp}/
├── scanner/
│   ├── port_scanner/
│   │   ├── report.json
│   │   ├── report.html
│   │   └── report.pdf
│   └── http_scanner/
│       ├── report.json
│       ├── report.html
│       └── report.pdf
├── recon/
│   ├── dns_enum/
│   ├── subdomain_enum/
│   ├── whois_lookup/
│   └── asn_lookup/
└── payloads/
    └── rev_tcp_fileless/
```

**Modules Tested**:

| Category | Module | Test Data |
|----------|--------|-----------|
| Scanner | port_scanner | Port open/closed results |
| Scanner | http_scanner | HTTP response analysis |
| Recon | dns_enum | DNS record enumeration |
| Recon | subdomain_enum | Subdomain discovery |
| Recon | whois_lookup | WHOIS data |
| Recon | asn_lookup | ASN information |
| Payloads | rev_tcp_fileless | Payload configuration |

**Total Reports**: 21 (7 modules x 3 formats)

## test_advanced_modules

Tests sensitive security modules using config-only approach (no live execution).

**Location**: `examples/test_advanced_modules.rs`

**Output**: `~/Desktop/ferox-advanced-test-{timestamp}/`

**Directory Structure**:
```
ferox-advanced-test-{timestamp}/
├── exploit/
│   └── example_exploit/
├── auxiliary/
│   └── onedrive_sync_exfil/
├── c2/
│   └── teams_tunnel/
├── evasion/
│   └── silent_shadow/
├── post/
│   └── deep_session_hijack/
└── SUMMARY.md
```

**Modules Tested**:

| Category | Module | ModuleType | Description |
|----------|--------|------------|-------------|
| Exploit | example_exploit | Exploit | Framework exploit example |
| Auxiliary | onedrive_sync_exfil | Auxiliary | Cloud exfiltration |
| C2 | teams_tunnel | Handler | Covert C2 channel |
| Evasion | silent_shadow | Encoder | EDR evasion |
| Post | deep_session_hijack | PostExploit | Browser session hijack |

**Total Reports**: 15 (5 modules x 3 formats)

## Creating Custom Test Examples

### Basic Template

```rust
//! Custom module test
//! Run with: cargo run --example my_test --features pdf-export

use chrono::Utc;
use ferox::core::module::{ModuleInfo, ModuleResult, ModuleType, Platform, Session};
use ferox::core::reporter::{HtmlReporter, JsonReporter, PdfReporter, ReportData, Reporter};
use ferox::core::result_store::StoredResult;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

fn main() {
    // Create results
    let results = vec![create_result("my_module", "category", true, "Test message")];

    // Create sessions
    let now = Utc::now();
    let sessions = vec![Session {
        id: Uuid::new_v4(),
        module: "category/my_module".to_string(),
        target: "test-target".to_string(),
        platform: Platform::Linux,
        established_at: now,
        last_seen: now,
        active: true,
        user: Some("tester".to_string()),
        metadata: HashMap::new(),
    }];

    // Generate reports
    let report_data = ReportData::new(results, sessions);
    JsonReporter.export(&report_data, Path::new("report.json")).unwrap();
    HtmlReporter.export(&report_data, Path::new("report.html")).unwrap();
    PdfReporter.export(&report_data, Path::new("report.pdf")).unwrap();
}

fn create_result(name: &str, category: &str, success: bool, message: &str) -> StoredResult {
    StoredResult {
        id: Uuid::new_v4(),
        module_info: ModuleInfo {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            author: "Test".to_string(),
            description: "Test module".to_string(),
            module_type: ModuleType::Auxiliary,
            category: category.to_string(),
        },
        result: if success {
            ModuleResult::success(message)
        } else {
            ModuleResult::error(message)
        },
    }
}
```

### Adding to Cargo.toml

```toml
[[example]]
name = "my_test"
path = "examples/my_test.rs"
```

## Validation

Examples automatically validate generated reports:

- **JSON**: Valid JSON structure (parseable by `python3 -m json.tool`)
- **HTML**: Contains `<!DOCTYPE html>` header
- **PDF**: Contains `%PDF` magic bytes

## Security Notice

The advanced modules test uses **config-only** mock data. No actual:
- Exploit execution
- C2 communication
- EDR evasion
- Credential theft
- Network attacks

These examples are safe to run on any system for testing report generation functionality.
