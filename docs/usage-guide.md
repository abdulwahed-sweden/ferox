---
title: Ferox Usage Guide
description: Practical CLI workflows and operational guidance for Ferox 2.0.0 operators.
---

# Usage Guide

This guide showcases the most common Ferox workflows, from environment bootstrap to memory forensics. All commands assume Ferox was built with Rust 1.70+ and that you are operating in an authorized lab or assessment environment.

## Bootstrap & Configuration
```bash
# Install dependencies
rustup default stable
cargo fetch

# Optional: create a dedicated Ferox workspace
mkdir -p ~/.ferox && cp ferox_security.toml ~/.ferox/config.toml
```

Key configuration values live in `~/.ferox/config.toml`:
```toml
[global]
workspace = "~/.ferox"
max_concurrent_operations = 100

[security]
require_confirmation = true
allowed_categories = ["scanner", "recon", "memory"]
```

## Interactive CLI Primer
```bash
cargo run --features memory-forensics --
ferox> help
ferox> use scanner/port
ferox (scanner/port)> set RHOSTS 192.168.1.0/24
ferox (scanner/port)> run
```

| Shortcut | Action |
| --- | --- |
| `tab` | Autocomplete modules, options, and commands |
| `Ctrl + R` | Reverse search command history |
| `Ctrl + D` | Exit the Ferox shell |

## Safe Mode & Mock Operations
Enable `SAFE_MODE=1` (or `--mock`) to run high-risk modules without touching live targets:
```bash
SAFE_MODE=1 cargo run --features memory-forensics -- mock run c2/teams_tunnel
```

Mock mode records an entry in the audit log and returns simulated output while leaving infrastructure untouched—ideal for tabletop exercises or CI pipelines.

## Memory Forensics CLI
```bash
# Full dump analysis with report persistence
ferox memory analyze samples/workstation.dmp --output reports/workstation.json

# Narrow options using inline filters
ferox memory malfind samples/workstation.dmp --min-score 0.7 --limit 15
ferox memory netscan samples/workstation.dmp --protocol tcp --suspicious-only
ferox memory mitre samples/workstation.dmp --format table
```

All memory commands accept `--database ./analysis.db` to reuse the SQLite evidence store.

## Scripting & Automation
Generate JSON reports and pipe them into downstream tooling:
```bash
ferox --json "use recon/subdomains; set TARGET example.com; run" | jq '.results[]'
```

Use the `--config` flag to point at alternate policy sets:
```bash
ferox --config configs/policies/production.toml --memory-db ./engagement.db
```

## Reporting & Export
- Session transcripts live under `~/.ferox/sessions/*.json`.
- Audit records stream into `~/.ferox/logs/audit.log`.
- Memory analysis exports can be emitted as JSON, CSV, or Markdown via `--format`.

For deeper integration examples see [Developer Guide](developer-guide.md) and the [Modules Catalog](modules.md).

---
_Version 2.0.0 • Updated 2025-11-12 • Contact: security@ferox.local_
