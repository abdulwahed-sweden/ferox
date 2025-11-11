# 🦊 Ferox - Fast, Fierce, Fearless Security Framework

[![Tests](https://img.shields.io/badge/tests-88%20passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()
[![Status](https://img.shields.io/badge/status-production%20ready-success)]()

> A modern offensive security framework built in Rust - designed for authorized penetration testing, CTF competitions, and security research.

## ⚡ Why Ferox?

- **50-100x Faster** - Startup in 0.11s vs Metasploit's 5-10s
- **Memory Safe** - Built with Rust's safety guarantees
- **Authorization-First** - Built-in security controls
- **Enterprise-Grade** - Professional infrastructure
- **Type-Safe** - Catch errors at compile time
- **Modern Async** - High-performance I/O with Tokio

## 🎯 Features

### Security & Compliance
- ✅ **Authorization System** - Time-bound, target-scoped permissions
- ✅ **Audit Logging** - Tamper-proof security trail
- ✅ **Safe Mode** - Confirmation prompts for dangerous operations
- ✅ **Security Policies** - Module whitelisting/blacklisting
- ✅ **Session Management** - SQLite persistence with concurrent safety

### Infrastructure
- ✅ **Module Metadata** - Versioning, dependencies, platform support
- ✅ **Dependency Resolution** - Topological sort, circular detection
- ✅ **Configuration Management** - TOML-based, hierarchical
- ✅ **Advanced Options** - Type-safe, validated options

### Framework Capabilities
- ✅ **Exploit Framework** - Target analysis, payload selection
- ✅ **12 Security Modules** - Scanner, recon, exploit, C2, post-exploitation
- ✅ **Session Tracking** - Persistent sessions with command history
- ✅ **Report Generation** - JSON, HTML, PDF export

## 🚀 Quick Start

### Prerequisites
```bash
# Rust 1.70 or higher
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation
```bash
git clone https://github.com/abdulwahed-sweden/ferox
cd ferox
cargo build --release
```

### Verify Installation
```bash
./verify_phase1.sh
```

### Run Ferox
```bash
./target/release/ferox
```

## 📖 Usage Examples

### Basic Port Scanning
```bash
ferox> use scanner/port_scanner
ferox (scanner/port_scanner)> set RHOSTS 192.168.1.0/24
ferox (scanner/port_scanner)> set PORTS 1-1000
ferox (scanner/port_scanner)> run
```

### Authorized Exploit Framework
```rust
use ferox::core::exploit_framework::*;

// Create authorization for penetration testing
let auth = AuthorizationContext::new_pentest(
    "PENTEST-2025-001".to_string(),
    vec!["192.168.1.0/24".to_string()],
);

// Initialize framework (requires valid authorization)
let mut framework = ExploitFramework::new(auth)?;

// Analyze target
let target = TargetInfo {
    hostname: "target.example.com".to_string(),
    ip_address: Some("192.168.1.100".to_string()),
    operating_system: Some("Linux".to_string()),
    services: vec![/* ... */],
    metadata: HashMap::new(),
};

let profile = framework.analyze_target(&target)?;
let payload = framework.select_payload(&profile)?;
```

### Configuration
```toml
# ~/.ferox/config.toml

[global]
workspace = "/home/user/.ferox"
max_concurrent_operations = 100
verbose = false

[security]
require_confirmation = true
audit_logging = true
allowed_categories = ["scanner", "recon"]

[network]
user_agent = "Ferox/2.0.0"
connection_timeout = 10
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
