# 🦊 Ferox Framework

<div align="center">

**Fast. Fierce. Fearless.**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://rustup.rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macOS%20%7C%20windows-lightgrey.svg)](https://github.com/abdulwahed-sweden/ferox)
[![Tests](https://img.shields.io/badge/tests-56%20passing-brightgreen.svg)](tests/)
[![Clippy](https://img.shields.io/badge/clippy-clean-brightgreen.svg)](Cargo.toml)

*A next-generation, memory-safe penetration testing framework written in Rust*

[Quick Start](#quick-start) • [Features](#features) • [Documentation](#documentation) • [Modules](#modules) • [Examples](#examples)

</div>

---

## 🎯 What is Ferox?

**Ferox** is a production-ready offensive security framework that combines the memory safety of Rust with modern penetration testing capabilities. Built for 2025's threat landscape, Ferox specializes in cloud-native operations, EDR evasion, and stealthy C2 channels.

### 🌟 Key Highlights

- 🦀 **Memory Safe** - Zero buffer overflows, written in Rust
- ⚡ **Async-First** - High-performance concurrent operations with Tokio
- ☁️ **Cloud-Native** - First-class Microsoft Teams C2 & OneDrive exfiltration
- 🛡️ **Safe by Default** - All dangerous modules include mock mode
- 🔐 **Production Crypto** - AES-256-GCM, HMAC-SHA256, HKDF-SHA256
- 📊 **100% Tested** - 56 tests passing, clippy clean
- 🎨 **Beautiful CLI** - Professional terminal interface with colors

---

## 🚀 Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/abdulwahed-sweden/ferox.git
cd ferox

# Build release
cargo build --release

# Run Ferox
./target/release/ferox
```

### First Command

```bash
$ ./target/release/ferox

    ███████╗███████╗██████╗  ██████╗ ██╗  ██╗
    ██╔════╝██╔════╝██╔══██╗██╔═══██╗╚██╗██╔╝
    █████╗  █████╗  ██████╔╝██║   ██║ ╚███╔╝
    ██╔══╝  ██╔══╝  ██╔══██╗██║   ██║ ██╔██╗
    ██║     ███████╗██║  ██║╚██████╔╝██╔╝ ██╗
    ╚═╝     ╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝

    ferox v2.0.0 - Ferocious Security Framework

ferox> modules
ferox> use scanner/port
ferox> set target 192.168.1.1
ferox> run
```

---

## ✨ Features

### Core Capabilities

| Feature | Status | Description |
|---------|--------|-------------|
| **Port Scanning** | ✅ | Fast async TCP port scanner |
| **DNS Enumeration** | ✅ | Multi-record type DNS recon |
| **HTTP Scanning** | ✅ | Web service fingerprinting |
| **Session Management** | ✅ | Track compromised hosts |
| **Report Generation** | ✅ | JSON, HTML, PDF exports |
| **Local Shell** | ✅ | Secure command execution |
| **Remote Shell** | ✅ | Reverse/bind shell handlers |
| **File Operations** | ✅ | Upload/download with security |

### Phase 3: Advanced Modules

| Module | Category | Description |
|--------|----------|-------------|
| **Teams Tunnel** | C2 | Microsoft Teams-based command & control |
| **Deep Session Hijack** | Post-Exploit | Browser cookie extraction (Chrome/Edge/Firefox) |
| **OneDrive Sync Exfil** | Auxiliary | Cloud-based data exfiltration |
| **Silent Shadow** | Evasion | EDR detection & bypass simulation |

---

## 📦 Modules

### Scanner Modules

- **`scanner/port`** - Async TCP port scanner with service detection
- **`scanner/http_scanner`** - HTTP/HTTPS service enumeration

### Reconnaissance Modules

- **`recon/dns`** - DNS enumeration (A, AAAA, MX, TXT, CNAME)
- **`recon/subdomains`** - Subdomain discovery
- **`recon/whois`** - Domain registration lookup
- **`recon/asn`** - ASN and IP range discovery

### C2 Modules (Phase 3)

- **`c2/teams_tunnel`** - ⭐ Microsoft Teams C2 channel
  - AES-256-GCM encrypted communications
  - Graph API integration
  - Phantom meeting creation
  - Safe mock mode

### Post-Exploitation Modules (Phase 3)

- **`post/browser/deep_session_hijack`** - 🍪 Browser cookie extraction
  - Chrome/Edge/Firefox support
  - In-memory SQLite parsing
  - Targets: *.microsoft.com, *.google.com, *.okta.com
  - JSON/CSV output

### Auxiliary Modules (Phase 3)

- **`auxiliary/cloud/onedrive_sync_exfil`** - ☁️ OneDrive exfiltration
  - Uses victim's OAuth token
  - Mimics legitimate sync traffic
  - Uploads to Backups/ folder
  - Rate limiting for stealth

### Evasion Modules (Phase 3)

- **`evasion/edr/silent_shadow`** - 🛡️ EDR detection & evasion
  - Detects 5 major EDR products
  - Hook detection in NTDLL
  - Direct syscall simulation (mock)
  - NTDLL unhooking simulation (mock)

---

## 📚 Documentation

### Complete Documentation

📖 **[FEROX_COMPLETE_DOCUMENTATION.md](FEROX_COMPLETE_DOCUMENTATION.md)** - Comprehensive 3,000+ line guide covering:
- Architecture & design
- Module system
- CLI reference
- Configuration
- Development guide
- API reference
- Security & legal

### Quick References

- 🚀 **[Quick Start Guide](#quick-start)** - Get started in 5 minutes
- 📝 **[CLI Reference](FEROX_COMPLETE_DOCUMENTATION.md#cli-reference)** - All commands and aliases
- 🔧 **[Configuration](FEROX_COMPLETE_DOCUMENTATION.md#configuration)** - Config file reference
- 🛠️ **[Development Guide](FEROX_COMPLETE_DOCUMENTATION.md#development-guide)** - Create modules
- 🔐 **[Security & Legal](FEROX_COMPLETE_DOCUMENTATION.md#security--legal)** - Authorization requirements

### Phase 3 Documentation

- 🎯 **[PHASE3_IMPLEMENTATION.md](PHASE3_IMPLEMENTATION.md)** - Phase 3 technical details
- 💡 **[examples/phase3_examples.md](examples/phase3_examples.md)** - CLI usage examples
- 📋 **[info.txt](info.txt)** - Implementation summary

---

## 💡 Examples

### Example 1: Port Scanning

```bash
ferox> use scanner/port
ferox> set target 192.168.1.0/24
ferox> set ports 22,80,443,3389
ferox> set threads 100
ferox> run
```

### Example 2: DNS Enumeration

```bash
ferox> use recon/dns
ferox> set domain example.com
ferox> set record_types A,AAAA,MX,TXT
ferox> run
```

### Example 3: Teams C2 (Mock Mode)

```bash
ferox> use c2/teams_tunnel
ferox> set access_token mock-token
ferox> set encryption_key MySecurePassword
ferox> set mock_mode true
ferox> run
```

### Example 4: Browser Session Hijack

```bash
ferox> use post/browser/deep_session_hijack
ferox> set browser chrome
ferox> set target_domains *.microsoft.com
ferox> set mock_mode true
ferox> run
```

**More examples:** See [examples/phase3_examples.md](examples/phase3_examples.md)

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Ferox CLI                            │
│                  (Interactive Terminal)                      │
└──────────────────┬──────────────────────────────────────────┘
                   │
    ┌──────────────┼──────────────┐
    │              │              │
┌───▼────┐    ┌───▼────┐    ┌───▼────┐
│ Module │    │Handler │    │Session │
│Registry│    │Registry│    │Manager │
└───┬────┘    └───┬────┘    └───┬────┘
    │             │              │
┌───▼─────────────▼──────────────▼───────┐
│         Core Infrastructure            │
│  • Crypto (AES-GCM, HMAC, HKDF)       │
│  • Database (SQLite)                   │
│  • Networking (Tokio, reqwest)         │
│  • Audit System                        │
└────────────────────────────────────────┘
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Test specific module
cargo test modules::c2::teams_tunnel

# Code quality
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

**Test Coverage:**
- ✅ 56 tests passing (100%)
- ✅ Clippy clean (0 warnings)
- ✅ All modules tested

---

## 🔐 Security & Legal

### ⚠️ IMPORTANT LEGAL NOTICE

**This software is for AUTHORIZED SECURITY TESTING ONLY.**

Before using Ferox, you **MUST** have:
- ✅ Written penetration testing engagement
- ✅ Red team exercise authorization
- ✅ Explicit permission for security research
- ✅ Understanding of legal implications

**Unauthorized use is ILLEGAL and UNETHICAL.**

### Safe by Default

- ✅ Mock mode enabled on all dangerous modules
- ✅ Explicit confirmation required for destructive operations
- ✅ Comprehensive audit logging
- ✅ Memory safety through Rust
- ✅ No unsafe code in production modules

**See [Security & Legal](FEROX_COMPLETE_DOCUMENTATION.md#security--legal) for complete details.**

---

## 🚧 Roadmap

### Completed ✅

- [x] Core framework & CLI
- [x] Scanner modules (port, HTTP)
- [x] Reconnaissance modules (DNS, WHOIS, ASN)
- [x] Handler system (local, remote, file ops)
- [x] Session management
- [x] Report generation (JSON, HTML, PDF)
- [x] Crypto infrastructure
- [x] Phase 3 advanced modules

### Planned 🔜

- [ ] Real Teams C2 provider (production mode)
- [ ] Token extraction from memory
- [ ] Chunked upload for OneDrive (>4MB)
- [ ] Real EDR bypass techniques (feature-flagged)
- [ ] Process injection primitives
- [ ] AMSI bypass detection

---

## 📊 Comparison

| Feature | Ferox | Metasploit | Cobalt Strike | Sliver |
|---------|-------|------------|---------------|--------|
| **Memory Safe** | ✅ Rust | ❌ Ruby | ❌ Java | ✅ Go |
| **Cloud C2** | ✅ Teams | ⚠️ Limited | ⚠️ Limited | ✅ Multiple |
| **Mock Mode** | ✅ Yes | ❌ No | ❌ No | ❌ No |
| **Browser Hijack** | ✅ Yes | ⚠️ Limited | ⚠️ Limited | ❌ No |
| **Open Source** | ✅ Yes | ✅ Yes | ❌ No | ✅ Yes |

---

## 🤝 Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Follow Rust style guidelines
4. Add tests for new features
5. Submit a pull request

See [Development Guide](FEROX_COMPLETE_DOCUMENTATION.md#development-guide) for details.

---

## 📄 License

MIT License - See [LICENSE](LICENSE) file

---

## 🙏 Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Tokio](https://tokio.rs/) - Async runtime
- [RustCrypto](https://github.com/RustCrypto) - Cryptographic libraries
- Many other open-source projects

---

## 📞 Support

- **Documentation:** [FEROX_COMPLETE_DOCUMENTATION.md](FEROX_COMPLETE_DOCUMENTATION.md)
- **Examples:** [examples/](examples/)
- **Issues:** [GitHub Issues](https://github.com/abdulwahed-sweden/ferox/issues)

---

<div align="center">

**Ferox v2.0.0** - "The best payload is the one that never looks like a payload."

Made with 🦀 and ❤️ by the Ferox Security Team

*Last Updated: 2025-11-10*

</div>
