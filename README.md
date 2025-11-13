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

_Version 2.0.0 · Updated 2025-11-12 · Ferox — Fast. Fierce. Fearless._ 🦊# 🦊 Ferox 2.0.0 — Fast, Fierce, Fearless
# 🦊 Ferox - Fast, Fierce, Fearless Security Framework

## 🔑 Core Capabilities
## 🔑 Core Capabilities
| Pillar | Highlights |
| --- | --- |
| ⚡ **Mission:** Deliver a trustworthy alternative to legacy exploitation stacks for red teams, defenders, and researchers. |
# 🦊 Ferox 2.0.0 — Fast, Fierce, Fearless# 🦊 Ferox 2.0.0 — Fast, Fierce, Fearless# 🦊 Ferox - Fast, Fierce, Fearless Security Framework



[![Tests](https://img.shields.io/badge/tests-88%20passing-brightgreen)]()

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)]()

[![License](https://img.shields.io/badge/license-MIT-blue)]()[![Tests](https://img.shields.io/badge/tests-88%20passing-brightgreen)]()[![Tests](https://img.shields.io/badge/tests-88%20passing-brightgreen)]()

[![Status](https://img.shields.io/badge/status-production%20ready-success)]()

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)]()[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)]()

> Ferox is a Rust-native offensive security framework for authorized operators who demand uncompromising speed, safety, and observability.

[![License](https://img.shields.io/badge/license-MIT-blue)]()[![License](https://img.shields.io/badge/license-MIT-blue)]()

- ⚡ **Mission:** Deliver a trustworthy alternative to legacy exploitation stacks for red teams, defenders, and researchers.

- 🛡️ **Guardrails:** Authorization gating, immutable audit trails, safe-mode confirmations, and policy enforcement.[![Status](https://img.shields.io/badge/status-production%20ready-success)]()[![Status](https://img.shields.io/badge/status-production%20ready-success)]()

- 🧠 **Intelligence:** Integrated memory forensics, MITRE ATT&CK mapping, and evidence persistence.



📚 **Quick navigation:** [Overview](docs/overview.md) · [Modules](docs/modules.md) · [Usage Guide](docs/usage-guide.md) · [Developer Guide](docs/developer-guide.md) · [Testing & CI](docs/testing-and-ci.md) · [Memory Forensics](docs/memory-forensics.md) · [Changelog](docs/changelog.md)

> Ferox is a Rust-native offensive security framework built for authorized operators who demand uncompromising speed, safety, and observability.> A modern offensive security framework built in Rust - designed for authorized penetration testing, CTF competitions, and security research.

## 🔑 Core Capabilities



| Pillar | Highlights |

| --- | --- |- ⚡ **Mission:** Deliver a trustworthy alternative to legacy exploitation stacks for red teams, defenders, and researchers.## ⚡ Why Ferox?

| Command & Control | HTTP beacon, DNS tunnel, Teams tunnel, relay manager, cloud pivots |

| Reconnaissance | ASN discovery, DNS/subdomain enumeration, WHOIS intelligence |- 🛡️ **Guardrails:** Authorization gating, immutable audit trails, safe-mode confirmations, and policy enforcement.

| Scanning | High-speed TCP/HTTP scanners with asynchronous execution |

| Evasion & Post | Silent Shadow EDR bypass, deep session hijacking, credential workflows |- 🧠 **Intelligence:** Integrated memory forensics, MITRE ATT&CK mapping, and evidence persistence.- **50-100x Faster** - Startup in 0.11s vs Metasploit's 5-10s

| Memory Forensics | Dump parsing, process tree scoring, malware heuristics, MITRE correlation |

| Governance | Time-boxed authorization, safe mode, tamper-proof audit logging |- **Memory Safe** - Built with Rust's safety guarantees



## 🚀 Getting Started📚 **Quick navigation:** [Overview](docs/overview.md) · [Modules](docs/modules.md) · [Usage Guide](docs/usage-guide.md) · [Developer Guide](docs/developer-guide.md) · [Testing & CI](docs/testing-and-ci.md) · [Memory Forensics](docs/memory-forensics.md) · [Changelog](docs/changelog.md)- **Authorization-First** - Built-in security controls



```bash- **Enterprise-Grade** - Professional infrastructure

# Clone and enter the repo

git clone https://github.com/abdulwahed-sweden/ferox## 🔑 Core Capabilities- **Type-Safe** - Catch errors at compile time

cd ferox

- **Modern Async** - High-performance I/O with Tokio

# Build with memory forensics enabled

cargo build --release --features memory-forensics| Pillar | Highlights |


Ferox is a modern Rust-based offensive operations and memory-forensics framework for authorized security teams.
It unifies CLI tooling, an interactive console, a modular security engine, and diagnostics into one fast, safe, and intelligent platform.

⭐ Core Features
🔧 Unified CLI Integration Layer

Run one entrypoint to access:

doctor — dependency validation & integrity scoring

memory — Volatility3 + YARA operations

c2 — communication helpers

sessions — SQLite-backed session manager

console — full interactive shell

🖥️ Interactive Console

Mixed Predator Theme

Module-aware prompt

Tab-completion

Safe-mode execution

Full session control

🧠 Memory Forensics (NEW IN 2.0)

Volatility3-backed dump parsing

YARA scanning engine

MITRE ATT&CK mapping

JSON / table / markdown export

🛰️ C2 & Operations

Teams Tunnel C2

HTTP beacon

DNS-based command & control

Relay manager foundation

🧩 Modular Architecture

Built-in module categories:

Reconnaissance

Scanning

Exploitation

Memory Forensics

Post-Exploitation

C2

Auxiliary

🩺 Ferox Doctor

Checks Python, YARA, Volatility3, permissions

Computes integrity score

Reports missing components

Suggests fixes

🚀 Quick Start
1. Install requirements
rustup default stable
pip3 install volatility3
brew install yara   # or apt install yara

2. Build Ferox
git clone https://github.com/abdulwahed-sweden/ferox
cd ferox
cargo build --release --features memory-forensics

3. Run the console
cargo run --bin ferox -- console

4. Examples
# Port scanner module
ferox> use scanner/port
ferox (scanner/port)> set RHOSTS 10.0.0.0/24
ferox (scanner/port)> run

# Memory forensics
cargo run --bin ferox -- memory analyze dumps/host.raw --output report.json

🏛️ Architecture Overview
┌────────────── Ferox CLI Integration Layer ───────────────┐
│ doctor • memory • c2 • sessions • console                │
└───────────────────────────────┬──────────────────────────┘
                                │
                          ┌─────▼─────┐
                          │  Console  │
                          │ (REPL UI) │
                          └─────┬─────┘
                                │
         ┌─────────── Modules / Engine (Async Rust) ─────────────┐
         │ Recon • Scan • Exploit • Memory • C2 • Post • Aux     │
         └───────────────────────┬───────────────────────────────┘
                                 │
                       Session Manager (SQLite)

🔥 CLI Examples
List modules
ferox> modules

Use a module
ferox> use scanner/http
ferox(scanner/http)> set RHOST example.com
ferox(scanner/http)> run

Analyze memory
cargo run --bin ferox -- memory malfind dump.raw

C2 check
cargo run --bin ferox -- c2 list

📦 Build & Test
cargo build
cargo test
cargo build --all-features

📄 License

MIT License — free for authorized, ethical security testing only.

👤 Maintainer

Abdulwahed Mansour
Email: abdulwahed.mansour@gmail.com
GitHub: @abdulwahed-sweden