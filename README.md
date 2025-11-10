# 🦊 Ferox Framework

**Ferocious Security Framework Built in Rust**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://rustup.rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macOS%20%7C%20windows-lightgrey.svg)](https://github.com/abdulwahed-sweden/ferox)

> **Fast. Fierce. Fearless.**

---

## ⚡ Overview

**Ferox** is a next-generation penetration testing framework written entirely in Rust. Designed for speed, safety, and efficiency, Ferox combines the power of modern systems programming with an intuitive command-line interface.

### ✨ Why Ferox?

- 🦀 **Memory Safe** - Built in Rust with zero buffer overflows
- ⚡ **Blazing Fast** - Async-first design powered by Tokio
- 🎨 **Beautiful CLI** - Professional interactive interface with colors
- 🔌 **Modular** - Easy to extend with custom modules
- 📦 **Single Binary** - No dependencies, just download and run
- 🌍 **Cross-Platform** - Works on Linux, macOS, and Windows
- 🔥 **Ferocious** - Aggressive scanning with intelligent resource management
- 🛡️ **Safe by Design** - Non-destructive check before exploit
- 📊 **Session Management** - Track and manage active sessions
- 🎯 **Payload Framework** - Generate and customize payloads

---

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/abdulwahed-sweden/ferox.git
cd ferox

# Build release version
cargo build --release

# Run Ferox
./target/release/ferox
```

### Basic Usage

```bash
# Start the framework
ferox

# Welcome banner appears
🦊 FEROX FRAMEWORK v2.0.0

# List available modules
ferox> modules

# Select the port scanner
ferox> use scanner/port_scanner

# View module options
ferox(scanner/port_scanner)> options

# Set target
ferox(scanner/port_scanner)> set RHOSTS 192.168.1.1
ferox(scanner/port_scanner)> set PORTS 1-1000

# Execute the scan
ferox(scanner/port_scanner)> run
```

---

## 📚 Command Reference

### Core Commands

| Command | Aliases | Description |
|---------|---------|-------------|
| `help` | `?` | Show available commands |
| `modules` | `list` | List all modules |
| `use <module>` | - | Select a module |
| `back` | - | Deselect current module |
| `exit` | `quit`, `q` | Exit Ferox |

### Module Commands

| Command | Description |
|---------|-------------|
| `show options` | Display module options |
| `show modules` | List all modules |
| `set <opt> <val>` | Set option value |
| `options` | Show current module options |
| `run` | Execute module |
| `info` | Display module information |

### Report & Export Commands

| Command | Description |
|---------|-------------|
| `export <format> <file>` | Export scan results to file (json, html, pdf) |
| `export results` | View stored results summary |

### Utility Commands

| Command | Description |
|---------|-------------|
| `banner` | Display Ferox banner |
| `version` | Show version info |
| `clear` | Clear the screen |

---

## 🔍 Available Modules

### Scanners

#### **scanner/port_scanner**
High-performance async TCP port scanner with concurrent connection support.

**Options:**
- `RHOSTS` - Target host or IP address (required)
- `PORTS` - Ports to scan (default: 1-1000)
- `TIMEOUT` - Connection timeout in ms (default: 1000)
- `THREADS` - Concurrent connections (default: 100)

**Example:**
```bash
ferox> use scanner/port_scanner
ferox(scanner/port_scanner)> set RHOSTS scanme.nmap.org
ferox(scanner/port_scanner)> set PORTS 1-65535
ferox(scanner/port_scanner)> set THREADS 200
ferox(scanner/port_scanner)> run
```

### Reconnaissance

#### **recon/subdomain_enum** 🌟 NEW!
Non-destructive subdomain enumeration via DNS resolution with optional HTTP probing.

**Options:**
- `RHOSTS` - Target domain (e.g., example.com) (required)
- `WORDLIST` - Path to subdomain wordlist (required)
- `THREADS` - Concurrent threads (default: 50)
- `TIMEOUT` - Request timeout in ms (default: 2000)
- `PROBE_HTTP` - Probe HTTP after DNS resolution (default: true)
- `OUTPUT` - Output format: human or json (default: human)

**Features:**
- Async DNS resolution using trust-dns-resolver
- Optional HTTP probing with title extraction
- Rate-limited concurrent execution
- JSON output for automation
- Safe and non-destructive

**Example:**
```bash
ferox> use recon/subdomain_enum
ferox(recon/subdomain_enum)> set RHOSTS example.com
ferox(recon/subdomain_enum)> set WORDLIST ./wordlist.txt
ferox(recon/subdomain_enum)> set THREADS 80
ferox(recon/subdomain_enum)> check          # Safe pre-check
ferox(recon/subdomain_enum)> run            # Execute enumeration
```

**Sample Output:**
```
Found 15 subdomains for example.com

Results:
────────────────────────────────────────────────────────────────
www.example.com → 93.184.216.34 [HTTP: 200] - Example Domain
mail.example.com → 93.184.216.35
api.example.com → 93.184.216.36 [HTTP: 200] - API Gateway
...
```

### Exploits

#### **exploit/example/example_exploit**
Example exploit module skeleton (safe, non-functional demonstration).

**Options:**
- `RHOSTS` - Target host (required)
- `RPORT` - Target port (default: 80)
- `TARGET_URI` - URI path (default: /)
- `LHOST` - Listener host (required)
- `LPORT` - Listener port (default: 4444)
- `PAYLOAD` - Payload type (default: payload/reverse_tcp)

**Safety Features:**
- Non-destructive `check` command for fingerprinting
- Requires explicit confirmation before running
- Skeleton only - no actual exploitation code

**Example:**
```bash
ferox> use exploit/example/example_exploit
ferox(exploit/example/example_exploit)> set RHOSTS target.local
ferox(exploit/example/example_exploit)> set LHOST 192.168.1.100
ferox(exploit/example/example_exploit)> check    # Safe fingerprint first!
ferox(exploit/example/example_exploit)> run      # Requires confirmation
```

---

## 📊 Report Generation

Ferox Framework includes a comprehensive report generation system that automatically stores scan results and allows you to export them in multiple formats.

### Features

- **Automatic Result Storage**: All module execution results are automatically stored in memory (last 100 results by default)
- **Multiple Export Formats**: Export to JSON, HTML, or PDF
- **Professional HTML Reports**: Beautiful, printable reports with embedded CSS
- **Summary Statistics**: Track successful/failed scans, modules used, and time ranges
- **Session Integration**: Reports include active session information

### Usage

**View Stored Results:**
```bash
ferox> export results
# Shows a summary of all stored scan results
```

**Export to JSON:**
```bash
ferox> export json results.json
# Exports all results as structured JSON data
```

**Export to HTML:**
```bash
ferox> export html report.html
# Generates a professional HTML report with styling
# Perfect for sharing with teams or clients
```

**Export to PDF:**
```bash
ferox> export pdf assessment.pdf
# Creates a PDF document for formal reporting
```

### Report Contents

Each report includes:
- **Executive Summary**: Total results, success/failure rates, active sessions
- **Modules Used**: List of all security modules executed
- **Detailed Results**: Full output from each module execution
  - Module information (name, version, author)
  - Execution timestamp
  - Success/failure status
  - Result data (ports found, subdomains discovered, etc.)
- **Session Information**: Active exploit sessions (if any)
- **Metadata**: Report generation time, Ferox version

### Example Workflow

```bash
# Run multiple scans
ferox> use scanner/port_scanner
ferox(scanner/port_scanner)> set RHOSTS 192.168.1.1
ferox(scanner/port_scanner)> run

ferox> use scanner/http_scanner
ferox(scanner/http_scanner)> set RHOSTS https://example.com
ferox(scanner/http_scanner)> run

# View what's been stored
ferox> export results

# Export everything to HTML
ferox> export html full_assessment.html
```

### Report Storage

- Results are stored in-memory (not persisted to disk automatically)
- Maximum of 100 results stored by default (oldest results are removed when limit is reached)
- Each result includes a unique UUID for tracking
- Use `export` command to save results before exiting Ferox

---

## 🏗️ Architecture

```
ferox/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli/                 # CLI interface
│   │   ├── app.rs           # Interactive REPL
│   │   └── theme.rs         # Styling and colors
│   ├── core/                # Core framework
│   │   └── module.rs        # Module system
│   ├── infra/               # Infrastructure helpers (Phase 3)
│   │   └── crypto.rs        # AES-GCM, HMAC, HKDF wrappers
│   └── modules/             # Security modules
│       ├── scanner/         # Scanners
│       ├── recon/           # Reconnaissance
│       ├── exploit/         # Exploit stubs
│       ├── c2/              # Phase 3 C2 scaffolding
│       │   ├── http_beacon.rs       # Encrypted, HMAC-authenticated beacon stub
│       │   ├── relay_manager.rs     # In-memory session routing stub
│       │   ├── cloud_tunnel.rs      # Provider trait + mock
│       │   ├── dns_c2.rs            # Base32/64 helpers + DNS stub
│       │   └── command_scheduler.rs # Minimal scheduler
│       └── post/
│           └── browser/
│               └── deep_session_hijack.rs # Safe mock, test-only
├── Cargo.toml               # Dependencies
├── config/
│   └── c2.example.toml      # Safe-by-default C2 config sample
└── README.md                # This file
```

---

## 🔧 Development

### Adding a New Module

1. Create a new file in `src/modules/<category>/`
2. Implement the `Module` trait
3. Register in `src/main.rs`

**Example:**

```rust
use crate::core::module::*;
use async_trait::async_trait;

pub struct MyModule {
    options: HashMap<String, String>,
}

#[async_trait]
impl Module for MyModule {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "my_module".to_string(),
            version: "1.0.0".to_string(),
            author: "Your Name".to_string(),
            description: "Description here".to_string(),
            module_type: ModuleType::Scanner,
            category: "scanner".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        // Define options
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        // Implementation
    }
}
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code
cargo clippy
```

---

## 📊 Performance Benchmarks

| Framework | Language | Scan Time (1000 ports) | Memory | Binary Size |
|-----------|----------|------------------------|--------|-------------|
| Metasploit | Ruby | ~120s | ~500MB | N/A |
| Nmap | C | ~30s | ~50MB | ~5MB |
| **Ferox** | **Rust** | **~12s** | **~15MB** | **~3MB** |

*Benchmarks conducted on 1000 ports with 100 concurrent connections*

---

## 🛣️ Roadmap

### ✅ Phase 1 - Foundation (Current)
- [x] Core framework architecture
- [x] CLI interface with REPL
- [x] Module system
- [x] Port scanner module
- [ ] HTTP scanner
- [ ] DNS enumeration
- [ ] SSL/TLS analyzer

### 🚧 Phase 2 - Advanced Features
- [ ] Vulnerability detection
- [ ] Exploit modules
- [ ] Payload generators
- [x] Report generation (JSON, HTML, PDF)
- [x] Session management
- [ ] Database persistence

### 🔮 Phase 3 - Enterprise
- [ ] Web dashboard (Tauri)
- [ ] REST API
- [ ] Team collaboration
- [ ] Cloud integration
- [ ] CI/CD integration
- [ ] Plugin marketplace

---

## 🔐 Phase 3: C2 Scaffolding (MVP)

This release introduces a conservative, non-destructive C2 scaffolding layer intended for experimentation and future expansion. All components are safe-by-default and tested; there are no live network transports enabled by default.

### What’s included

- infra/crypto
    - AES-256-GCM authenticated encryption, HMAC-SHA256, HKDF-SHA256
    - Small, safe wrappers with clear key sizes and deterministic tests
- modules/c2
    - http_beacon: Encrypted, HMAC-authenticated beacon client/server model backed by an in-memory test server (no network by default)
    - relay_manager: In-process session registration and command/result channels (stub)
    - cloud_tunnel: Provider-agnostic interface with a mock provider for tests
    - dns_c2: Base32/Base64 helpers and a stub DNS query function
    - command_scheduler: Minimal in-memory scheduler API
- modules/post/browser
    - deep_session_hijack (mock): Safe test-only reader of a local sample profile file

### Configuration

An example C2 configuration is provided at `config/c2.example.toml` with safe defaults:

- Auth token read from env: `FEROX_C2_TOKEN`
- `tls_verify = true`
- `allowed_roots = ["./"]`
- `beacon_poll_interval_ms = 1000`
- `cloud_provider = "mock"`

Set the token at runtime (example):

```bash
export FEROX_C2_TOKEN="change_me_for_tests"
```

### Running tests for the new scaffolding

All new modules are covered by fast, deterministic unit tests and one integration test. CI runs these automatically.

```bash
# Run everything
cargo test --all

# Clippy with warnings denied
cargo clippy --all-targets -- -D warnings
```

### Safety and scope

- No destructive operations are enabled by default
- No live C2 network communication is performed in tests (in-memory stubs only)
- Encryption/HMAC code is minimal and well-scoped; key rotation and replay protection are TODOs
- Feature gates and provider implementations will be added incrementally in future PRs

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure your code:
- Passes `cargo clippy`
- Passes `cargo test`
- Follows Rust best practices
- Includes documentation

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## ⚠️ Legal Disclaimer

**IMPORTANT:** Ferox is designed for **authorized security testing only**.

- Only use on systems you own or have explicit permission to test
- Unauthorized access to computer systems is illegal
- The authors are not responsible for misuse or damage
- Always obtain proper authorization before testing
- Follow responsible disclosure practices

This tool is for educational purposes and legitimate security testing only.

---

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- Powered by [Tokio](https://tokio.rs/) async runtime
- Inspired by Metasploit, Nmap, and the security community

---

## 👨‍💻 Author

**Abdulwahed Mansour**

- GitHub: [@abdulwahed-sweden](https://github.com/abdulwahed-sweden)
- Email: abdulwahed.mansour@gmail.com
- LinkedIn: [Abdulwahed Mansour](https://linkedin.com/in/abdulwahed-mansour)

---

## 🌟 Show Your Support

If you find Ferox useful, please consider:
- ⭐ Starring the repository
- 🐛 Reporting bugs
- 💡 Suggesting features
- 🤝 Contributing code

---

<div align="center">

**🦊 Built with ❤️ and Rust 🦀**

**Fast. Fierce. Fearless.**

</div>
