# 🦊 Ferox 2.0.0 — Fast, Fierce, Fearless

[![Rust Edition](https://img.shields.io/badge/Rust-Edition%202024-orange)]()
[![Status](https://img.shields.io/badge/Status-Production%20Ready-success)]()
[![License](https://img.shields.io/badge/License-MIT-blue)]()

Ferox is a next-generation offensive operations and memory-forensics framework written in Rust. It unifies the Ferox CLI Integration Layer, interactive console, module system, and diagnostics into a single workflow that prioritizes safety, observability, and speed for authorized operators.

## Highlights
- **Unified entrypoint:** The Ferox CLI Integration Layer exposes doctor, memory, C2, sessions, and console targets with consistent help output.
- **Interactive console:** Rich module runner with tab completion, Mixed Predator Theme styling, and module-aware prompts.
- **Memory forensics:** Volatility3-backed dump parsing, YARA scanning, and MITRE ATT&CK mapping with JSON or report exports.
- **Session manager:** Async registry backed by SQLite with history, cleanup, and exec helpers.
- **Ferox Doctor:** Dependency verification, integrity scoring, health checks, and remediation hints.
- **C2 helpers:** Safe-by-default orchestration for Teams tunnels, GitHub C2, relay manager, and future transports.

## Architecture at a Glance
```
┌──────────────────────── Ferox CLI Integration Layer ────────────────────────┐
│ ferox doctor • ferox memory • ferox c2 • ferox sessions • ferox console     │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │
                        ┌──────────▼──────────┐
                        │ Ferox Console       │
                        │  • Module registry  │
                        │  • Mixed Predator UI│
                        └──────────┬──────────┘
                                   │
        ┌──────────────┬───────────┼──────────────┬──────────────┬──────────────┐
        │ Recon/Scan   │ Exploit   │ Memory Ops   │ C2 Layer     │ Maintenance  │
        │ Modules      │ Modules   │ (Volatility) │ (Teams, etc) │ (Doctor)     │
        └──────────────┴───────────┴──────────────┴──────────────┴──────────────┘
                                   │
                          Session Manager & Result Store
```

## Quick Start
1. **Install prerequisites**
   - macOS, Linux, or WSL
   - Rust toolchain with Edition 2024 support (`rustup default stable`)
   - Python 3, Volatility3, and YARA for memory workflows (optional)
2. **Clone and build**
   ```bash
   git clone https://github.com/abdulwahed-sweden/ferox
   cd ferox
   cargo build --release --features memory-forensics
   ```
3. **Run commands**
   ```bash
   # Run diagnostics or memory helpers without opening the console
   cargo run --bin ferox -- doctor check
   cargo run --bin ferox -- memory analyze dumps/host.raw --json

   # Launch the interactive console
   cargo run --bin ferox -- console
   ```
4. **Enable safe mode (optional)**
   ```bash
   SAFE_MODE=1 cargo run --bin ferox -- c2 list
   ```

## What’s New in 2.0.0
- Ferox CLI Integration Layer with rich startup banner, routed subcommands, and dependency probes.
- Volatility3 + YARA memory pipeline with exportable reports and MITRE ATT&CK summaries.
- Session Manager revamp with persistent SQLite backing, command history, and cleanup API.
- Expanded C2 toolkit (Teams tunnel, GitHub Gist C2, relay manager scaffolding).
- Ferox Doctor upgrades: integrity score, critical vs. full health checks, remediation hints.
- Mixed Predator Theme applied consistently across CLI/console for readability.

## Documentation
| File | Summary |
| --- | --- |
| [docs/overview.md](docs/overview.md) | Mission, architecture layers, and security posture. |
| [docs/getting-started.md](docs/getting-started.md) | Environment prep, builds, safe mode, and workspace layout. |
| [docs/console.md](docs/console.md) | Ferox console anatomy, commands, and automation tips. |
| [docs/modules.md](docs/modules.md) | Module catalog across recon, exploit, memory, and C2. |
| [docs/memory-forensics.md](docs/memory-forensics.md) | Dump handling, Volatility3 bridge, YARA, MITRE mapping. |
| [docs/c2.md](docs/c2.md) | Communication helpers, configuration, and mock operations. |
| [docs/sessions.md](docs/sessions.md) | Session lifecycle, execution helpers, cleanup strategies. |
| [docs/maintenance.md](docs/maintenance.md) | Ferox Doctor, integrity scoring, and health automation. |
| [docs/troubleshooting.md](docs/troubleshooting.md) | Common issues, integrity score failures, and support paths. |

## Community & License
- License: [MIT](LICENSE)
- Contact: `abdulwahed.mansour@gmail.com`
- Security questions: `security@ferox.local`
- Follow releases and discussions on [GitHub](https://github.com/abdulwahed-sweden/ferox)

Use Ferox solely for authorized operations. Respect engagement boundaries and applicable laws.
[![Tests](https://img.shields.io/badge/tests-88%20passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()
[![Status](https://img.shields.io/badge/status-production%20ready-success)]()

> Ferox is a Rust-native offensive security framework for authorized operators who demand uncompromising speed, safety, and observability.

- ⚡ **Mission:** Deliver a trustworthy alternative to legacy exploitation stacks for red teams, defenders, and researchers.
- 🛡️ **Guardrails:** Authorization gating, immutable audit trails, safe-mode confirmations, and policy enforcement.
- 🧠 **Intelligence:** Integrated memory forensics, MITRE ATT&CK mapping, and evidence persistence.

📚 **Quick navigation:** [Overview](docs/overview.md) · [Modules](docs/modules.md) · [Usage Guide](docs/usage-guide.md) · [Developer Guide](docs/developer-guide.md) · [Testing & CI](docs/testing-and-ci.md) · [Memory Forensics](docs/memory-forensics.md) · [Changelog](docs/changelog.md)

## 🔑 Core Capabilities

| Pillar | Highlights |
| --- | --- |
| Command & Control | HTTP beacon, DNS tunnel, Teams tunnel, relay manager, cloud pivots |
| Reconnaissance | ASN discovery, DNS/subdomain enumeration, WHOIS intelligence |
| Scanning | High-speed TCP/HTTP scanners with asynchronous execution |
| Evasion & Post | Silent Shadow EDR bypass, deep session hijacking, credential workflows |
| Memory Forensics | Dump parsing, process tree scoring, malware heuristics, MITRE correlation |
| Governance | Time-boxed authorization, safe mode, tamper-proof audit logging |

## 🚀 Getting Started

```bash
# Clone and enter the repo
git clone https://github.com/abdulwahed-sweden/ferox
cd ferox

# Build with memory forensics enabled
cargo build --release --features memory-forensics

# Initialize workspace (optional)
mkdir -p ~/.ferox && cp ferox_security.toml ~/.ferox/config.toml
```

### Run the CLI
```bash
./target/release/ferox --help
./target/release/ferox memory --help
```

### Mock / Safe Testing Mode
Run dangerous modules with full auditing but without touching live infrastructure.
```bash
SAFE_MODE=1 ./target/release/ferox --mock run c2/teams_tunnel
```

Audit entries for mock executions appear under `~/.ferox/logs/audit.log` and are tagged accordingly.

## 🖥️ CLI Snapshots

```text
ferox> help
ferox> use scanner/port
ferox (scanner/port)> set RHOSTS 10.0.5.0/24
ferox (scanner/port)> set PORTS 1-1000
ferox (scanner/port)> run --json > reports/port-scan.json
```

```bash
# Memory forensics workflow
ferox memory analyze dumps/workstation.dmp --database analysis.db --output reports/workstation.json
ferox memory malfind dumps/workstation.dmp --min-score 0.6 --mitre --format table
ferox memory mitre dumps/workstation.dmp --database analysis.db --format markdown
```

## 🗃️ Build & Test Matrix

```bash
# Standard build
cargo build

# Full feature build
cargo build --all-features

# Test suites
cargo test --lib
cargo test --features memory-forensics --tests
cargo test --test integration_tests
```

Add `SAFE_MODE=1` for smoke validation in CI environments.

## 🧭 Documentation Highlights

- [docs/overview.md](docs/overview.md) — architecture, mission, security posture.
- [docs/modules.md](docs/modules.md) — categorized module catalog.
- [docs/usage-guide.md](docs/usage-guide.md) — CLI walkthroughs and automation tips.
- [docs/developer-guide.md](docs/developer-guide.md) — coding standards and contribution workflow.
- [docs/testing-and-ci.md](docs/testing-and-ci.md) — verification pipeline and quality gates.
- [docs/memory-forensics.md](docs/memory-forensics.md) — in-depth analysis workflow.
- [docs/changelog.md](docs/changelog.md) — release evolution from v1.x to v2.0.0.

## 🛣️ Roadmap

| Status | Initiative |
| --- | --- |
| ✅ | v2.0.0 memory forensics launch |
| 🔄 | Plugin marketplace with signed module distribution |
| 🔄 | Extended payload library and automation templates |
| 🔄 | Web operator console with real-time telemetry |
| 🗓️ | Hardware-backed credential vault integration |

Have ideas? See the [Developer Guide](docs/developer-guide.md) and open a discussion.

## 🤝 Contributing

We welcome issues, proposals, and pull requests that respect the project's safety-first ethos.

1. Fork and branch (`git checkout -b feat/<short-description>`).
2. Follow the test matrix, including `cargo test --features memory-forensics --tests`.
3. Document new modules and update metadata.
4. Reference authorization and audit implications in your PR description.

Please ensure all work complies with applicable laws and engagement contracts.

## 📜 License & Compliance

Ferox is released under the [MIT License](LICENSE). Usage is restricted to authorized security testing, defensive research, and educational scenarios. Unauthorized or malicious use is strictly prohibited.

## 📬 Support & Contact

- Maintainer: Abdulwahed Mansour — `abdulwahed.mansour@gmail.com`
- Security inquiries: `security@ferox.local`
- GitHub: [@abdulwahed-sweden](https://github.com/abdulwahed-sweden)

---
