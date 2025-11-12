# Upgrade Guide - Ferox 2.0.0

Release date: 2025-11-12

This document walks existing Ferox 1.5.x deployments through upgrading to the 2.0.0 release that introduces the memory forensics suite, hardened authorization gates, and the expanded CLI experience. Follow the guidance below to keep evidence stores and configuration intact while adopting the new capabilities.

## What Changed in 2.0.0
- Memory forensics pipeline spanning dump parsing, process, malware, network, registry, credential, and MITRE mapping analyzers (`src/memory_forensics/*`).
- CLI `memory` command group with subcommands such as `analyze`, `pslist`, `malfind`, `netscan`, `mitre`, and `hashdump` (`src/cli/memory.rs`).
- Expanded core `ExploitFramework` with explicit authorization enforcement and payload selection (`src/core/exploit_framework.rs`).
- Hierarchical configuration loader (`src/core/config.rs`) and refreshed security policy template (`ferox_security.toml.example`).
- SQLite schema extensions for memory evidence (`src/core/memory_analysis.rs`) plus additional regression tests under `tests/`.

## Before You Start
- Install Rust 1.82.0 or newer (`rustup update stable` recommended).
- Back up your workspace directory (defaults to `~/.ferox/`) including `sessions.db`, `memory_analysis.db`, `config.toml`, and audit logs.
- Export any custom security policy from `ferox_security.toml`.
- Capture the current version output: `ferox --version`.
- Ensure you have at least 1 GB of free disk space for new dependencies and build artifacts.

## Breaking or Notable Changes
- The default Cargo feature set now enables `memory-forensics`; expect longer first builds and larger binaries.
- All exploit operations require a valid `AuthorizationContext`. Code that previously instantiated `ExploitFramework` without scope data must be updated.
- Configuration is now loaded via the structured `FeroxConfig` hierarchy. Legacy ad hoc environment variables are ignored.
- The security policy file expects `file_access`, `command_execution`, `audit`, and `remote_shell` tables. Older files missing these sections should be updated from `ferox_security.toml.example`.
- SQLite migrations run automatically on first launch, but they are one-way. Maintain backups before launching 2.0.0.

## Upgrade Steps
1. Fetch the latest code and check out the 2.0.0 tag:
   ```bash
   git fetch origin
   git checkout v2.0.0
   ```
2. Install optional native tooling as needed:
   - `brew install yara` if you intend to enable the `yara-support` feature.
   - Python 3.10 or newer with headers (`brew install python@3.11`) for the optional `volatility-bridge` feature.
3. Rebuild the project (defaults will compile the memory suite):
   ```bash
   cargo clean
   cargo build --release
   ```
4. If you were using a custom build feature set, update scripts to include `--features memory-forensics` or `--features full` as required.
5. Merge your configuration with the new schema:
   - Start from `ferox_security.toml.example` and reapply local changes.
   - Create or update `~/.ferox/config.toml` to include `[global]`, `[security]`, `[logging]`, `[network]`, and any `[modules."path"]` overrides.
6. Validate database migrations by running a dry-run analysis:
   ```bash
   target/release/ferox memory analyze /path/to/dump --mock --database ~/.ferox/memory_analysis.db
   ```
   The `--mock` flag exercises schema upgrades without touching live evidence.

## Post-Upgrade Verification
- Confirm the binary version: `target/release/ferox --version` (should report `Ferox 2.0.0`).
- List the new CLI group: `target/release/ferox memory --help`.
- Run the regression suite with memory features enabled:
  ```bash
  cargo test --features memory-forensics --tests
  ```
- Inspect audit logs (`~/.ferox/logs/audit.log`) to ensure operations are still recorded.

## Optional Feature Matrix
- `--features yara-support` requires the YARA 4.x shared library on your system.
- `--features volatility-bridge` requires Python 3.10 or newer, `pip install maturin`, and the ability to compile pyo3 bindings.
- `--features pdf-export` enables printable report generation via `printpdf`.

## Rollback Plan
- Stop all Ferox processes.
- Restore your `~/.ferox` backup (database, configs, logs).
- Check out the previous release tag (for example, `git checkout v1.5.4`).
- Rebuild with `cargo build --release` and verify `ferox --version` returns the expected value.

## Support and Reporting
- Documentation: `docs/` (start with `docs/overview.md` and `docs/usage-guide.md`).
- Security contact: security@ferox.local.
- File issues at https://github.com/abdulwahed-sweden/ferox/issues with detailed logs and configuration snippets.

Stay safe, operate with explicit authorization, and enjoy the expanded 2.0.0 feature set.
