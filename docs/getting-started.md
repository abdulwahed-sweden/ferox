# Getting Started

This guide walks through preparing a Ferox 2.0.0 environment, building from source, and validating the toolchain before entering the console.

## Prerequisites
- macOS 13+, modern Linux, or Windows Subsystem for Linux.
- Rust toolchain with Edition 2024 support (`rustup toolchain install stable && rustup default stable`).
- Python 3.10+, Volatility3, and YARA (only required for memory forensics commands).
- SQLite (bundled through `rusqlite`), Git, and `cargo`.

## Clone & Build
```bash
git clone https://github.com/abdulwahed-sweden/ferox
cd ferox

# Recommended build with memory tooling enabled
cargo build --release --features memory-forensics
```

The release binary lives at `target/release/ferox`. To run directly from the source tree:
```bash
cargo run --bin ferox -- --help
```

## Workspace Hygiene
```
ferox/
├─ ferox_security.toml      # Default policy (copy to ~/.ferox/config.toml)
├─ config/                  # Example C2 configs
├─ dumps/                   # Memory dump samples (keep offline)
├─ docs/                    # Unified documentation suite
└─ target/                  # Cargo build artifacts
```

## Safe Mode & Mocking
Use safe mode during demos or CI to disable destructive routines:
```bash
SAFE_MODE=1 cargo run --bin ferox -- console
```

Many C2 and exploit modules expose `mock_mode` options to exercise workflows without reaching external infrastructure.

## First Commands
```bash
# Show the startup banner and see dependency probes
cargo run --bin ferox -- console

# Run Ferox Doctor and print an integrity score
cargo run --bin ferox -- doctor check --format text

# Analyze a memory image (requires Volatility3 + YARA)
ferox memory analyze dumps/workstation.raw --output reports/workstation.json
```

If the CLI reports missing dependencies, see [docs/maintenance.md](maintenance.md) or [docs/troubleshooting.md](troubleshooting.md).
