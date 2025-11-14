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
First Run
bash
Copy code
# System diagnostics
./target/release/ferox doctor check

# Launch console
./target/release/ferox console

# Memory forensics
./target/release/ferox memory analyze dump.raw --output report.json
🎯 Core Features
🔍 Memory Forensics
Volatility3 Integration

YARA Scanning

MITRE ATT&CK Mapping

Export formats: JSON, Markdown, Tables

🛰️ Command & Control
Teams Tunnel C2

HTTP Beacon

DNS C2

Relay Manager

🔧 Security Operations
Reconnaissance (ASN, DNS, WHOIS)

High-speed scanning

Exploitation (authorized)

Post-exploitation: credentials, persistence, lateral movement

🗂 System Management
SQLite session tracking

Tamper-proof audit logs

Safe-mode for dangerous modules

Policy-based authorization

🛠️ Usage Examples
Interactive Console
bash
Copy code
ferox> use scanner/port
ferox (scanner/port)> set RHOSTS 10.0.0.0/24
ferox (scanner/port)> set PORTS 1-1000
ferox (scanner/port)> run --json > scan_results.json
Memory Analysis
bash
Copy code
ferox memory analyze memory.dmp --database analysis.db
ferox memory malfind memory.dmp --min-score 0.6 --mitre
ferox memory pslist memory.dmp --format table
System Diagnostics
bash
Copy code
ferox doctor check --critical
ferox doctor check --format json
ferox doctor dependency python
ferox doctor dependency volatility
C2 Operations
bash
Copy code
ferox c2 list --status active
ferox c2 setup teams_tunnel --team-id "security-team"
ferox c2 test http_beacon --target http://target.com
🎨 Interface — Mixed Predator Theme
Dark, high-contrast security palette

Minimal animations (calm UI)

Terminal-optimized typography

Clean status symbols (✅ ❌ ⚠️ ℹ️)

Safe Mode Example
bash
Copy code
SAFE_MODE=1 ferox --mock run c2/teams_tunnel
SAFE_MODE=1 ferox console
🔧 Architecture
scss
Copy code
Ferox CLI Integration Layer
       ↓
[doctor, memory, c2, sessions, console]
       ↓
Module Registry & Session Manager
       ↓
Security Engine (Async Rust)
       ↓
[Scanner, Recon, Exploit, Memory, C2, Post, Auxiliary]
📦 Module Categories
Category	#	Description
Scanner	8	Port & service detection
Recon	6	Information gathering
Exploit	4	Authorized exploitation
Memory Forensics	8	Volatility3 workflows
Post-Exploitation	7	Credentials, persistence
C2 & Evasion	12	Communication & stealth
Auxiliary	5	Utility modules

🩺 Ferox Doctor — Diagnostics Engine
bash
Copy code
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
🔒 Security & Authorization
Ferox is designed exclusively for authorized security work.

Permitted
Authorized penetration testing

Red team assessments

Security research

CTFs

Defensive training

Prohibited
Unauthorized access

Criminal activity

Malicious operations

Violating laws/regulations

Built-in Safeguards
Authorization context

Immutable audit logs

Safe mode for high-risk modules

Policy-based access control

📚 Documentation
docs/overview.md — Architecture & mission

docs/console.md — Console usage

docs/modules.md — Module catalog

docs/memory-forensics.md — Memory workflow

docs/c2.md — Command & control layer

docs/maintenance.md — Ferox Doctor

🛣️ Roadmap
 v2.0.0 – Memory forensics engine

 Signed plugin marketplace

 Expanded payload library

 Web operator dashboard

 Hardware-backed credential vault

🤝 Contributing
Fork and branch:

bash
Copy code
git checkout -b feat/description
Run full tests:

bash
Copy code
cargo test --features memory-forensics --tests
Document all new modules

Include authorization/audit notes in PR

📄 License
Released under the MIT License.
Usage limited to ethical and authorized scenarios.

📬 Support
Maintainer: Abdulwahed Mansour

Email: abdulwahed.mansour@gmail.com

Security Contact: security@ferox.local

GitHub: @abdulwahed-sweden

Fast. Fierce. Fearless. Authorized. 🦊