# 🦊 Ferox 2.0.0 — Fast, Fierce, Fearless# 🦊 Ferox - Fast, Fierce, Fearless Security Framework



[![Tests](https://img.shields.io/badge/tests-88%20passing-brightgreen)]()[![Tests](https://img.shields.io/badge/tests-88%20passing-brightgreen)]()

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)]()[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)]()

[![License](https://img.shields.io/badge/license-MIT-blue)]()[![License](https://img.shields.io/badge/license-MIT-blue)]()

[![Status](https://img.shields.io/badge/status-production%20ready-success)]()[![Status](https://img.shields.io/badge/status-production%20ready-success)]()



> Ferox is a Rust-native offensive security framework built for authorized operators who demand uncompromising speed, safety, and observability.> A modern offensive security framework built in Rust - designed for authorized penetration testing, CTF competitions, and security research.



- ⚡ **Mission:** Deliver a trustworthy alternative to legacy exploitation stacks for red teams, defenders, and researchers.## ⚡ Why Ferox?

- 🛡️ **Guardrails:** Authorization gating, immutable audit trails, safe-mode confirmations, and policy enforcement.

- 🧠 **Intelligence:** Integrated memory forensics, MITRE ATT&CK mapping, and evidence persistence.- **50-100x Faster** - Startup in 0.11s vs Metasploit's 5-10s

- **Memory Safe** - Built with Rust's safety guarantees

📚 **Quick navigation:** [Overview](docs/overview.md) · [Modules](docs/modules.md) · [Usage Guide](docs/usage-guide.md) · [Developer Guide](docs/developer-guide.md) · [Testing & CI](docs/testing-and-ci.md) · [Memory Forensics](docs/memory-forensics.md) · [Changelog](docs/changelog.md)- **Authorization-First** - Built-in security controls

- **Enterprise-Grade** - Professional infrastructure

## 🔑 Core Capabilities- **Type-Safe** - Catch errors at compile time

- **Modern Async** - High-performance I/O with Tokio

| Pillar | Highlights |

| --- | --- |## 🎯 Features

| Command & Control | HTTP beacon, DNS tunnel, Teams tunnel, relay manager, cloud pivots |

| Reconnaissance | ASN discovery, DNS/subdomain enumeration, WHOIS intelligence |### Security & Compliance

| Scanning | High-speed TCP/HTTP scanners with asynchronous execution |- ✅ **Authorization System** - Time-bound, target-scoped permissions

| Evasion & Post | Silent Shadow EDR bypass, deep session hijacking, credential workflows |- ✅ **Audit Logging** - Tamper-proof security trail

| Memory Forensics | Dump parsing, process tree scoring, malware heuristics, MITRE correlation |- ✅ **Safe Mode** - Confirmation prompts for dangerous operations

| Governance | Time-boxed authorization, safe mode, tamper-proof audit logging |- ✅ **Security Policies** - Module whitelisting/blacklisting

- ✅ **Session Management** - SQLite persistence with concurrent safety

## 🚀 Getting Started

### Infrastructure

```bash- ✅ **Module Metadata** - Versioning, dependencies, platform support

# Clone and enter the repo- ✅ **Dependency Resolution** - Topological sort, circular detection

git clone https://github.com/abdulwahed-sweden/ferox- ✅ **Configuration Management** - TOML-based, hierarchical

cd ferox- ✅ **Advanced Options** - Type-safe, validated options



# Build with memory forensics enabled### Framework Capabilities

cargo build --release --features memory-forensics- ✅ **Exploit Framework** - Target analysis, payload selection

- ✅ **12 Security Modules** - Scanner, recon, exploit, C2, post-exploitation

# Initialize workspace (optional)- ✅ **Memory Forensics** - Integrated Windows dump analysis (NEW!)

mkdir -p ~/.ferox && cp ferox_security.toml ~/.ferox/config.toml- ✅ **Session Tracking** - Persistent sessions with command history

```- ✅ **Report Generation** - JSON, HTML, PDF export



### Run the CLI## 🚀 Quick Start

```bash

./target/release/ferox --help### Prerequisites

./target/release/ferox memory --help```bash

```# Rust 1.70 or higher

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### Mock / Safe Testing Mode```

Run dangerous modules with full auditing but without touching live infrastructure.

```bash### Installation

SAFE_MODE=1 ./target/release/ferox --mock run c2/teams_tunnel```bash

```git clone https://github.com/abdulwahed-sweden/ferox

cd ferox

Audit entries for mock executions appear under `~/.ferox/logs/audit.log` and are tagged accordingly.cargo build --release

```

## 🖥️ CLI Snapshots

### Verify Installation

```text```bash

ferox> help./verify_phase1.sh

ferox> use scanner/port```

ferox (scanner/port)> set RHOSTS 10.0.5.0/24

ferox (scanner/port)> set PORTS 1-1000### Run Ferox

ferox (scanner/port)> run --json > reports/port-scan.json```bash

```./target/release/ferox

```

```bash

# Memory forensics workflow### Memory Forensics CLI

ferox memory analyze dumps/workstation.dmp --database analysis.db --output reports/workstation.json```bash

ferox memory malfind dumps/workstation.dmp --min-score 0.6 --mitre --format table# Full analysis with JSON export

ferox memory mitre dumps/workstation.dmp --database analysis.db --format markdown./target/release/ferox memory analyze memory.dmp --output report.json

```

# Process inventory

## 🗃️ Build & Test Matrix./target/release/ferox memory pslist memory.dmp

```

```bash

# Standard build## 📖 Usage Examples

cargo build

### Basic Port Scanning

# Full feature build```bash

cargo build --all-featuresferox> use scanner/port_scanner

ferox (scanner/port_scanner)> set RHOSTS 192.168.1.0/24

# Test suitesferox (scanner/port_scanner)> set PORTS 1-1000

cargo test --libferox (scanner/port_scanner)> run

cargo test --features memory-forensics --tests```

cargo test --test integration_tests

```### Authorized Exploit Framework

```rust

Add `SAFE_MODE=1` for smoke validation in CI environments.use ferox::core::exploit_framework::*;



## 🧭 Documentation Highlights// Create authorization for penetration testing

let auth = AuthorizationContext::new_pentest(

- [docs/overview.md](docs/overview.md) — architecture, mission, security posture.    "PENTEST-2025-001".to_string(),

- [docs/modules.md](docs/modules.md) — categorized module catalog.    vec!["192.168.1.0/24".to_string()],

- [docs/usage-guide.md](docs/usage-guide.md) — CLI walkthroughs and automation tips.);

- [docs/developer-guide.md](docs/developer-guide.md) — coding standards and contribution workflow.

- [docs/testing-and-ci.md](docs/testing-and-ci.md) — verification pipeline and quality gates.// Initialize framework (requires valid authorization)

- [docs/memory-forensics.md](docs/memory-forensics.md) — in-depth analysis workflow.let mut framework = ExploitFramework::new(auth)?;

- [docs/changelog.md](docs/changelog.md) — release evolution from v1.x to v2.0.0.

// Analyze target

## 🛣️ Roadmaplet target = TargetInfo {

    hostname: "target.example.com".to_string(),

| Status | Initiative |    ip_address: Some("192.168.1.100".to_string()),

| --- | --- |    operating_system: Some("Linux".to_string()),

| ✅ | v2.0.0 memory forensics launch |    services: vec![/* ... */],

| 🔄 | Plugin marketplace with signed module distribution |    metadata: HashMap::new(),

| 🔄 | Extended payload library and automation templates |};

| 🔄 | Web operator console with real-time telemetry |

| 🗓️ | Hardware-backed credential vault integration |let profile = framework.analyze_target(&target)?;

let payload = framework.select_payload(&profile)?;

Have ideas? See the [Developer Guide](docs/developer-guide.md) and open a discussion.```



## 🤝 Contributing### Memory Analysis (CLI)

```bash

We welcome issues, proposals, and pull requests that respect the project's safety-first ethos.ferox memory analyze memory.dmp --output report.json

ferox memory malfind memory.dmp

1. Fork and branch (`git checkout -b feat/<short-description>`).ferox memory mitre memory.dmp --output mitre.json

2. Follow the test matrix, including `cargo test --features memory-forensics --tests`.```

3. Document new modules and update metadata.

4. Reference authorization and audit implications in your PR description.### Configuration

```toml

Please ensure all work complies with applicable laws and engagement contracts.# ~/.ferox/config.toml



## 📜 License & Compliance[global]

workspace = "/home/user/.ferox"

Ferox is released under the [MIT License](LICENSE). Usage is restricted to authorized security testing, defensive research, and educational scenarios. Unauthorized or malicious use is strictly prohibited.max_concurrent_operations = 100

verbose = false

## 📬 Support & Contact

[security]

- Maintainer: Abdulwahed Mansour — `abdulwahed.mansour@gmail.com`require_confirmation = true

- Security inquiries: `security@ferox.local`audit_logging = true

- GitHub: [@abdulwahed-sweden](https://github.com/abdulwahed-sweden)allowed_categories = ["scanner", "recon"]



---[network]

user_agent = "Ferox/2.0.0"

_Version 2.0.0 · Updated 2025-11-12 · Ferox — Fast. Fierce. Fearless._ 🦊connection_timeout = 10

verify_tls = true
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│          CLI Interface                  │
│  (Interactive REPL + Tab Completion)    │
└─────────────────────────────────────────┘
                  │
    ┌─────────────┴─────────────┐
    │                           │
┌───▼──────────┐      ┌────────▼────────┐
│   Module     │      │    Session      │
│  Registry    │      │   Manager       │
└───┬──────────┘      └────────┬────────┘
    │                          │
┌───▼──────────────────────────▼────────┐
│         Core Framework                │
│  • Exploit Framework                  │
│  • Module Metadata                    │
│  • Dependency Resolution              │
│  • Configuration Management           │
│  • Authorization System               │
└───────────────────────────────────────┘
                  │
    ┌─────────────┼─────────────┐
    │             │             │
┌───▼───┐   ┌────▼────┐   ┌───▼────┐
│Scanner│   │  Recon  │   │Exploit │
│Modules│   │ Modules │   │Modules │
└───────┘   └─────────┘   └────────┘
```

## 📦 Available Modules

### Scanners
- `scanner/port_scanner` - High-performance async TCP port scanner
- `scanner/http_scanner` - HTTP service enumeration

### Reconnaissance
- `recon/subdomain_enum` - Subdomain discovery
- `recon/dns_enum` - DNS enumeration with hickory-resolver
- `recon/whois_lookup` - WHOIS information gathering
- `recon/asn_discovery` - ASN discovery and mapping

### Command & Control
- `c2/teams_tunnel` - Microsoft Teams-based C2 channel
- `c2/http_beacon` - HTTP-based beacon
- `c2/dns_c2` - DNS-based command and control

### Post-Exploitation
- `post/browser/deep_session_hijack` - Browser session hijacking

### Auxiliary
- `auxiliary/cloud/onedrive_sync_exfil` - OneDrive exfiltration

### Evasion
- `evasion/edr/silent_shadow` - EDR evasion techniques

## 🧪 Testing

```bash
# Run all tests
cargo test --lib

# Include memory forensics integration tests
cargo test --features memory-forensics --tests

# Run specific test suite
cargo test --lib core::exploit_framework

# Run with output
cargo test --lib -- --nocapture
```

**Current Status:** 88/88 tests passing ✅

## 📊 Performance

| Metric | Metasploit | Ferox | Improvement |
|--------|-----------|-------|-------------|
| Startup Time | ~5-10s | 0.11s | **50-100x faster** |
| Memory Usage | High | Low | Rust efficiency |
| Type Safety | Dynamic | Static | Compile-time checks |
| Async Support | Limited | Native (Tokio) | Modern performance |

## 🔒 Security & Authorization

⚠️ **AUTHORIZED USE ONLY**

Ferox is designed for legitimate security activities only:

**Permitted:**
- ✅ Authorized penetration testing engagements
- ✅ Capture The Flag (CTF) competitions
- ✅ Security research (academic/commercial)
- ✅ Defensive security training
- ✅ Authorized red team exercises

**Prohibited:**
- ❌ Unauthorized system access
- ❌ Malicious activities
- ❌ Criminal purposes
- ❌ Violating laws or regulations

### Authorization Enforcement

All exploit framework operations require explicit authorization:

```rust
// Time-bound authorization
let auth = AuthorizationContext::new_pentest(
    "ENGAGEMENT-ID".to_string(),
    vec!["192.168.1.0/24".to_string()],
);

// Framework validates authorization
let framework = ExploitFramework::new(auth)?;
// ✅ Only proceeds if authorization is valid
```

### Audit Logging

All security-critical operations are logged:

```bash
cat ~/.ferox/audit.log
# 2025-11-11T00:30:45Z | username | c2/teams_tunnel | confirmed=true
# 2025-11-11T00:31:12Z | username | auxiliary/cloud/onedrive_sync_exfil | confirmed=false
```

## 📚 Documentation

- [Phase 1: Critical Fixes](PHASE1_FIXES.md) - Safe mode, concurrency, options
- [Phase 2: Infrastructure](PHASE2_INFRASTRUCTURE.md) - Metadata, config, dependencies
- [Phase 3: Exploit Framework](PHASE3_COMPLETE.md) - Authorization, target analysis
- [Implementation Summary](README_IMPLEMENTATION.md) - Quick reference

## 🛠️ Development

### Building from Source
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with verbose logging
RUST_LOG=debug cargo run
```

### Project Structure
```
ferox/
├── src/
│   ├── core/              # Core framework
│   │   ├── audit.rs       # Audit logging
│   │   ├── config.rs      # Configuration
│   │   ├── exploit_framework.rs  # Exploit framework
│   │   ├── module.rs      # Module trait
│   │   ├── module_metadata.rs    # Metadata system
│   │   ├── module_options.rs     # Options system
│   │   └── session.rs     # Session management
│   ├── modules/           # Security modules
│   │   ├── scanner/       # Port & service scanning
│   │   ├── recon/         # Reconnaissance
│   │   ├── exploit/       # Exploits
│   │   ├── c2/            # Command & control
│   │   ├── post/          # Post-exploitation
│   │   └── auxiliary/     # Auxiliary modules
│   ├── handlers/          # Session handlers
│   └── cli/               # CLI interface
└── tests/                 # Integration tests
```

## 🤝 Contributing

Contributions are welcome! Please ensure:

1. All tests pass (`cargo test`)
2. Code follows Rust best practices
3. Documentation is updated
4. Security guidelines are followed
5. Only authorized use cases are supported

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Async runtime: [Tokio](https://tokio.rs/)
- CLI: [Rustyline](https://github.com/kkawakam/rustyline)
- Inspired by: Metasploit Framework

## 📞 Contact

- **Author:** Abdulwahed Mansour
- **Email:** abdulwahed.mansour@gmail.com
- **GitHub:** [@abdulwahed-sweden](https://github.com/abdulwahed-sweden)

## ⚡ Quick Stats

```
Language:       Rust
Lines of Code:  ~12,000+
Tests:          88 (all passing)
Modules:        12 security modules
Build Time:     6m 43s (release)
Startup Time:   0.11s
Binary Size:    12 MB
```

## 🎯 Roadmap

- [x] Phase 1: Critical Fixes (Complete)
- [x] Phase 2: Infrastructure (Complete)
- [x] Phase 3: Exploit Framework (Complete)
- [ ] Phase 4: Plugin Architecture
- [ ] Phase 5: Advanced Payloads
- [ ] Phase 6: Web UI
- [ ] Phase 7: Module Marketplace

---

**Fast. Fierce. Fearless. Authorized.** 🦊

Built with ❤️ using Rust for the security community.
