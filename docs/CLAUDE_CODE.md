# Ferox Framework - Claude Code Project

## Project Overview
Ferox is a next-generation penetration testing framework built in Rust. This project includes:
- High-performance async port scanner
- Subdomain enumeration with DNS resolution
- Exploit framework (safe skeletons)
- Session management
- Payload generation framework

## Quick Start Commands

### Initial Setup
```bash
# Check Rust installation
cargo --version

# Build the project
cd ferox
cargo build --release

# Run Ferox
./target/release/ferox
```

### Development Commands
```bash
# Run in debug mode
cargo run

# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt

# Clean build artifacts
cargo clean
```

### Using Ferox
```bash
# Start the framework
./target/release/ferox

# Example: Port scanning
use scanner/port_scanner
set RHOSTS 127.0.0.1
set PORTS 1-100
run

# Example: Subdomain enumeration
use recon/subdomain_enum
set RHOSTS example.com
set WORDLIST ./wordlist.txt
check
run

# Example: Exploit module (safe)
use exploit/example/example_exploit
set RHOSTS target.local
set LHOST 192.168.1.100
check
run
```

## Project Structure
- `src/main.rs` - Entry point
- `src/cli/` - Interactive CLI
- `src/core/` - Core framework (modules, sessions, payloads)
- `src/modules/` - Security modules (scanner, recon, exploit)
- `Cargo.toml` - Dependencies
- `wordlist.txt` - Sample subdomain wordlist

## Key Features
- ⚡ Async-first design with Tokio
- 🦀 Memory-safe Rust implementation
- 🎨 Beautiful CLI with colors
- 🔐 Safety-first with check before run
- 📊 JSON output for automation
- 🌍 Cross-platform support

## Available Modules
1. **scanner/port_scanner** - High-performance TCP scanner
2. **recon/subdomain_enum** - DNS-based subdomain discovery
3. **exploit/example/example_exploit** - Safe exploit skeleton

## Safety Notice
⚠️ Only use on systems you own or have explicit permission to test.
Always use `check` before `run` for exploit modules.

## Support
- Author: Abdulwahed Mansour
- GitHub: abdulwahed-sweden/ferox
- Email: abdulwahed.mansour@gmail.com

🦊 Fast. Fierce. Fearless.
