# Report Generation

Ferox provides comprehensive report generation capabilities for documenting security assessments, penetration testing results, and forensic analysis.

## Supported Formats

| Format | Feature Flag | Description |
|--------|--------------|-------------|
| JSON   | (default)    | Structured data for automation and integration |
| HTML   | (default)    | Cyber-neon themed web reports |
| PDF    | `pdf-export` | Professional PDF documents |

## Building with PDF Support

```bash
# Standard build (JSON + HTML only)
cargo build --release

# With PDF export
cargo build --release --features pdf-export

# Full feature build
cargo build --release --features "memory-forensics,pdf-export"
```

## Console Export Commands

```bash
# Launch console and run a scan
ferox console
ferox> use scanner/port_scanner
ferox> set RHOSTS 192.168.1.0/24
ferox> set PORTS 22,80,443,8080
ferox> run

# View results
ferox> results

# Export to different formats
ferox> export json /path/to/report.json
ferox> export html /path/to/report.html
ferox> export pdf /path/to/report.pdf    # Requires pdf-export feature
```

## Programmatic API

```rust
use ferox::core::reporter::{JsonReporter, HtmlReporter, PdfReporter, ReportData, Reporter};
use ferox::core::result_store::StoredResult;
use ferox::core::module::Session;
use std::path::Path;

// Collect your results and sessions
let results: Vec<StoredResult> = /* ... */;
let sessions: Vec<Session> = /* ... */;

// Create report data
let report_data = ReportData::new(results, sessions);

// Export to JSON
JsonReporter.export(&report_data, Path::new("report.json"))?;

// Export to HTML
HtmlReporter.export(&report_data, Path::new("report.html"))?;

// Export to PDF (requires pdf-export feature)
#[cfg(feature = "pdf-export")]
PdfReporter.export(&report_data, Path::new("report.pdf"))?;
```

## Report Content

All report formats include:

### Executive Summary
- Total results count
- Success/failure breakdown
- Active sessions count
- Modules used
- Time range of operations

### Results Section
- Module information (name, version, author, category)
- Operation status (success/failure)
- Detailed messages and data
- Timestamps

### Sessions Section
- Session ID
- Target information
- Platform details
- User context
- Establishment time
- Activity status

### Metadata
- Report generation timestamp
- Ferox version
- Module categories used

## JSON Report Structure

```json
{
  "results": [
    {
      "id": "uuid",
      "module_info": {
        "name": "port_scanner",
        "version": "1.0.0",
        "author": "Ferox Team",
        "description": "TCP port scanner",
        "module_type": "Scanner",
        "category": "scanner"
      },
      "result": {
        "success": true,
        "message": "Port 8080 open on 192.168.1.100",
        "data": { "service": "HTTP" },
        "timestamp": "2025-11-25T17:00:00Z"
      }
    }
  ],
  "sessions": [
    {
      "id": "uuid",
      "module": "scanner/port_scanner",
      "target": "192.168.1.100",
      "platform": "Linux",
      "user": "tester",
      "established_at": "2025-11-25T17:00:00Z",
      "last_seen": "2025-11-25T17:05:00Z",
      "active": true
    }
  ],
  "generated_at": "2025-11-25T17:10:00Z",
  "ferox_version": "2.0.0",
  "summary": {
    "total_results": 10,
    "successful_results": 8,
    "failed_results": 2,
    "total_sessions": 1,
    "active_sessions": 1,
    "modules_used": ["scanner/port_scanner"],
    "time_range": {
      "start": "2025-11-25T17:00:00Z",
      "end": "2025-11-25T17:05:00Z"
    }
  }
}
```

## HTML Report Features

- **Cyber-neon theme**: Dark background with neon accent colors
- **Responsive design**: Works on desktop and mobile
- **Collapsible sections**: For detailed result data
- **Status badges**: Color-coded success/failure indicators
- **Formatted timestamps**: Human-readable date/time

## PDF Report Features

- **A4 format**: Standard document size
- **Professional layout**: Title, sections, footer
- **Color-coded status**: Green for success, red for failures
- **Executive summary**: Quick overview stats
- **Detailed findings**: Up to 10 results per page
- **Footer**: Version and branding

## Test Examples

Ferox includes test examples that demonstrate report generation:

```bash
# Test 7 modules across scanner, recon, payload categories
cargo run --example test_multi_module --features pdf-export

# Test 5 advanced modules (exploit, c2, evasion, post)
cargo run --example test_advanced_modules --features pdf-export
```

These examples generate reports in `~/Desktop/ferox-*-test-*/` directories with:
- Per-module subdirectories
- JSON, HTML, and PDF reports for each module
- SUMMARY.md with validation results

## Module Categories Tested

| Category | Modules | Description |
|----------|---------|-------------|
| Scanner | port_scanner, http_scanner | Network reconnaissance |
| Recon | dns_enum, subdomain_enum, whois_lookup, asn_lookup | Information gathering |
| Payloads | rev_tcp_fileless | Payload configuration |
| Exploit | example_exploit | Exploitation modules |
| Auxiliary | onedrive_sync_exfil | Cloud operations |
| C2 | teams_tunnel | Command & control |
| Evasion | silent_shadow | Defense evasion |
| Post | deep_session_hijack | Post-exploitation |

## Best Practices

1. **JSON for automation**: Use JSON exports for CI/CD integration and automated processing
2. **HTML for sharing**: Share HTML reports with stakeholders who need interactive viewing
3. **PDF for formal reports**: Use PDF for official documentation and client deliverables
4. **Multiple formats**: Export to all formats for comprehensive documentation
5. **Timestamped directories**: Organize reports by date/time for historical tracking

## Troubleshooting

### PDF export not available
```
Error: PDF export requires the 'pdf-export' feature
```
Solution: Rebuild with `--features pdf-export`

### Empty PDF content
If PDF shows only the title, ensure you're using the latest printpdf v0.8.2 API with the `Op::AddLineBreak` approach for proper text layout.

### Large reports
For scans with many results, the PDF export limits detailed findings to the first 10 results. Full data is always available in JSON format.
