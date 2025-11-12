---
title: Ferox Memory Forensics
description: End-to-end workflow for memory acquisition analysis in Ferox 2.0.0.
---

# Memory Forensics

Ferox ships with a first-class memory forensics suite purpose-built for Windows crash dumps and raw memory images. Modules follow the same authorization, logging, and safe-mode conventions as the rest of the framework while contributing structured results to the SQLite evidence store.

## Components
| Component | Responsibility | Source |
| --- | --- | --- |
| Dump Parser | Identifies dump type, extracts system metadata, and pre-processes sections. | `src/memory_forensics/dump_parser.rs` |
| Process Analyzer | Reconstructs process trees, detects hollowing/injections, and annotates anomalies. | `src/memory_forensics/process_analyzer.rs` |
| Malware Detector | Applies YARA rules and heuristic scoring to suspicious regions. | `src/memory_forensics/malware_detector.rs` |
| Network Analyzer | Rebuilds socket inventory and beacon timelines. | `src/memory_forensics/network_analyzer.rs` |
| Registry Analyzer | Surfaces persistence artifacts and credential stores. | `src/memory_forensics/registry_analyzer.rs` |
| Credential Extractor | Dumps SAM/LSA secrets, DPAPI masters, and cached credentials. | `src/memory_forensics/credential_extractor.rs` |
| MITRE Mapper | Converts detector findings into ATT&CK techniques for reporting. | `src/memory_forensics/mitre_mapper.rs` |
| Volatility Bridge (optional) | Invokes custom Python plugins for advanced triage. | `src/memory_forensics/volatility_bridge.rs` |

## CLI Workflow
```bash
# Analyze a dump and store results in SQLite
ferox memory analyze images/workstation.dmp --database analysis.db --output reports/workstation.json

# Focus on process hollowing
ferox memory malfind images/workstation.dmp --min-score 0.6 --mitre --format table

# Review ATT&CK coverage
ferox memory mitre images/workstation.dmp --database analysis.db --format markdown
```

All commands support `--mock` to execute in safe mode without touching live dumps—useful for CI validation of parsing logic.

## Database Schema Overview
| Table | Purpose |
| --- | --- |
| `memory_dumps` | Tracks source files, hashes, acquisition notes, and analyzer versions. |
| `memory_processes` | Stores process metadata, parentage, integrity levels, and risk scores. |
| `memory_injections` | Captures suspicious regions, heuristics, and correlated techniques. |
| `memory_mitre_techniques` | Maintains ATT&CK IDs and references discovered during analysis. |
| `memory_malware_hits` | Links YARA matches to processes, modules, and confidence scores. |
| `memory_network_connections` | Records sockets, protocols, timestamps, and anomaly flags. |

SQLite migrations run automatically; however, export the database before upgrading for long-lived cases.

## Best Practices
- Acquire dumps with trusted tooling and record chain-of-custody metadata.
- Keep YARA rules current (`plugins/yara_rules/`) and version control changes.
- Enable the `volatility-bridge` feature only in controlled environments with Python 3.10+ available.
- Review audit logs (`~/.ferox/logs/audit.log`) to cross-reference analysis operations.
- Use `--limit` and filtering flags to maintain human-readable output for reports.

## Further Reading
- [Usage Guide](usage-guide.md) for CLI examples and scripting tips.
- [Testing & CI](testing-and-ci.md) for verification strategies.
- [Changelog](changelog.md) for highlights of the memory suite introduction.

---
_Version 2.0.0 • Updated 2025-11-12 • Contact: security@ferox.local_
