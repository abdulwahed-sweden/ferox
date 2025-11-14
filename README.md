# Ferox Framework Documentation

## Overview
Ferox is a Rust-native offensive security framework designed for authorized penetration testing, security research, and red team operations. It provides a comprehensive suite of tools for memory forensics, command & control operations, reconnaissance, and post-exploitation activities.

## Key Features
- **Memory Forensics**: Advanced analysis with Volatility3 integration and YARA scanning
- **Command & Control**: Multiple communication channels including Teams Tunnel and HTTP Beacon
- **Security Operations**: Complete reconnaissance, scanning, and exploitation capabilities
- **System Management**: Session tracking, audit logging, and policy enforcement
- **Interactive Console**: Professional dark-themed interface optimized for security work

## Installation Requirements
- Rust 1.70+
- Memory forensics features enabled during build
- Python dependencies for Volatility3 integration

## Core Components
1. **CLI Integration Layer**: Main command interface
2. **Module Registry**: Centralized module management
3. **Security Engine**: Async Rust-based processing core
4. **Module Categories**: Scanner, Recon, Exploit, Memory, C2, Post-Exploitation, Auxiliary

## Security Model
- Designed exclusively for authorized security testing
- Built-in authorization context and audit trails
- Safe mode prevents accidental dangerous operations
- Policy-based access controls and scope limitations

## Usage Patterns
- Interactive console for real-time operations
- Batch processing for automated tasks
- Memory analysis workflows for forensics
- C2 setup and management for red team exercises
- System diagnostics and health monitoring

## Authorization Notice
This framework is intended only for authorized penetration testing, security research, red team exercises, and defensive training. Unauthorized or malicious use is strictly prohibited and may violate applicable laws.
# 🦊 Ferox Framework

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)]()
[![License](https://img.shields.io/badge/License-MIT-blue)]()
[![Status](https://img.shields.io/badge/Status-Production%20Ready-success)]()

**Ferox** is a modern Rust-native offensive security framework designed for authorized penetration testing, security research, and red team operations. Built with performance and safety in mind.

## 🚀 Quick Start

### Installation
```bash
git clone https://github.com/abdulwahed-sweden/ferox
cd ferox
cargo build --release --features memory-forensics
# System diagnostics
./target/release/ferox doctor check

# Launch interactive console
./target/release/ferox console

# Memory forensics
./target/release/ferox memory analyze dump.raw --output report.json
🎯 Core Features
./target/release/ferox memory analyze dump.raw --output report.json
YARA Scanning: Malware detection and classification

MITRE ATT&CK Mapping: Technique identification and reporting

Multiple Formats: JSON, table, and markdown export

Command & Control
Teams Tunnel: Microsoft Teams-based C2 channels
Multiple Formats: JSON, table, and markdown export

### Command & Control
DNS C2: Covert DNS tunneling capabilities

Relay Management: Multi-hop infrastructure support

Security Operations
Reconnaissance: ASN discovery, DNS enumeration, WHOIS lookup
Relay Management: Multi-hop infrastructure support

### Security Operations
Exploitation: Authorized vulnerability testing

Post-Exploitation: Credential collection, persistence, lateral movement

System Management
Session Tracking: SQLite-backed operation history
Post-Exploitation: Credential collection, persistence, lateral movement

### System Management
Safe-Mode: Confirmation prompts for dangerous operations

Policy Enforcement: Scope-based authorization controls

🛠️ Usage Examples
Interactive Console
bash
ferox> use scanner/port
ferox (scanner/port)> set RHOSTS 10.0.0.0/24
ferox (scanner/port)> set PORTS 1-1000
ferox (scanner/port)> run --json > scan_results.json
Memory Analysis
bash
ferox memory analyze memory.dmp --database analysis.db
ferox (scanner/port)> run --json > scan_results.json
bash
ferox doctor check --critical
ferox doctor check --format json
ferox doctor dependency python
ferox memory pslist memory.dmp --format table
ferox c2 list --status active
ferox c2 setup teams_tunnel --team-id "security-team"
ferox c2 test http_beacon --target http://target.com
🎨 Interface
Mixed Predator Theme
ferox doctor dependency volatility

Typography: Clean, readable fonts for extended terminal sessions

Icons: Consistent status indicators (✅ ❌ ⚠️ ℹ️)
ferox c2 test http_beacon --target http://target.com
Safe Mode
bash
SAFE_MODE=1 ferox --mock run c2/teams_tunnel
SAFE_MODE=1 ferox console
🔧 Architecture
text
Ferox CLI Integration Layer
    ↓
[doctor, memory, c2, sessions, console]
    ↓
Layout: Minimal animations, maximum information density

### Safe Mode
[Scanner, Recon, Exploit, Memory, C2, Post, Auxiliary]
📦 Module Categories
SAFE_MODE=1 ferox console
Exploit	4 modules	Authorized vulnerability testing
Memory Forensics	8 modules	Volatility3-based analysis
Post-Exploitation	7 modules	Credentials, persistence, lateral movement
C2 & Evasion	12 modules	Communication channels, detection avoidance
Auxiliary	5 modules	Utilities and support functions
🩺 Ferox Doctor
Comprehensive system diagnostics and dependency checking:

bash
# Full system check
[Scanner, Recon, Exploit, Memory, C2, Post, Auxiliary]
# Check specific dependencies
ferox doctor dependency python
ferox doctor dependency volatility

# Auto-fix capabilities
ferox doctor check --fix

# Multiple output formats
Auxiliary	5 modules	Utilities and support functions
🔒 Security & Authorization
Comprehensive system diagnostics and dependency checking:
✅ Authorized penetration testing
✅ Red team exercises with written approval
✅ Security research in controlled environments
✅ Defensive security training
✅ CTF competitions

Prohibited Use
❌ Unauthorized system access
❌ Malicious activities
❌ Criminal purposes
❌ Violating laws or regulations

Security Features
Authorization Context: Time-bound, scope-limited operations
ferox doctor check --format markdown

Safe Mode: Prevents accidental dangerous operations

### Permitted Use

📚 Documentation
Overview - Architecture and mission

Console Guide - Interactive usage

### Prohibited Use

Memory Forensics - Analysis workflows

C2 Operations - Communication channels

### Security Features

🛣️ Roadmap
v2.0.0 - Memory forensics launch

Plugin marketplace with signed module distribution

Extended payload library and automation templates

Web operator console with real-time telemetry

Hardware-backed credential vault integration

🤝 Contributing
We welcome contributions that respect the project's security-first ethos:

Fork and branch: git checkout -b feat/description

Follow test matrix: cargo test --features memory-forensics --tests

Document new modules and update metadata

Reference authorization implications in PR descriptions

All work must comply with applicable laws and engagement contracts.

📄 License
Ferox is released under the MIT License. Usage is restricted to authorized security testing, defensive research, and educational scenarios. Unauthorized or malicious use is strictly prohibited.

📬 Support
Maintainer: Abdulwahed Mansour

Email: abdulwahed.mansour@gmail.com

Security: security@ferox.local

GitHub: @abdulwahed-sweden

Fast. Fierce. Fearless. Authorized. 🦊

Built with Rust for the security community.