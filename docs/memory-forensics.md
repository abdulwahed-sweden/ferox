# Memory Forensics

Ferox 2.0.0 ships a Volatility3-inspired memory pipeline with optional YARA enrichment. The `ferox memory` subcommands can run from the CLI router, within the console, or via automation scripts.

## Toolchain Requirements
- Python 3.10+
- Volatility3 installed on the PATH (`volatility3` or `vol.py`)
- `yara` binary for YARA scans
- Optional: SQLite storage for analysis persistence (`ferox_memory.db`)

The CLI router probes these binaries at startup and reports readiness.

## Command Overview
| Command | Purpose |
| --- | --- |
| `ferox memory analyze <dump>` | Full pipeline (processes, malware strings, network artifacts, registry evidence). |
| `ferox memory pslist <dump>` | Fast process inventory. |
| `ferox memory pstree <dump>` | Hierarchical view with parent-child relationships. |
| `ferox memory malfind <dump>` | Injection + malware heuristics. |
| `ferox memory netscan <dump>` | Network sockets, DNS cache, URL extraction. |
| `ferox memory hashdump <dump>` | Credential artifact extraction. |
| `ferox memory yarascan <dump> --rules ruleset.yar` | YARA pattern matching over raw pages. |
| `ferox memory mitre <dump>` | MITRE ATT&CK mapping from collected evidence. |

Each command supports JSON output for downstream automation. `ferox memory analyze` and `ferox memory mitre` also accept `--output <path>` to persist findings.

## Typical Workflow
```bash
ferox memory analyze dumps/workstation.raw --output reports/workstation.json --json
ferox memory malfind dumps/workstation.raw
ferox memory yarascan dumps/workstation.raw --rules plugins/yara_rules/windows_malware.yar
ferox memory mitre dumps/workstation.raw --output reports/mitre.json
```

## Integration Points
- **Session Manager:** Findings can be associated with sessions to tie implants back to forensic artifacts.
- **Maintenance:** Ferox Doctor ensures Volatility3/YARA readiness before memory commands run.
- **Reporting:** JSON outputs feed into the console result store or external SIEM pipelines.

## Performance Tips
- Keep dumps on fast local storage; avoid network-mounted images.
- Use `--json` for machine parsing, `--output` for archival, and `--database` (via `MemoryCli`) when persistent comparisons are needed.
- Run `ferox doctor dependency volatility` if CLI probes fail.

Ferox memory tooling remains non-destructive and suitable for IR labs, hunt teams, and compromise assessments.
