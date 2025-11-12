---
title: Ferox Developer Guide
description: Engineering practices, contribution workflow, and internal architecture notes for Ferox 2.0.0.
---

# Developer Guide

The Ferox codebase balances rapid module development with rigorous safety controls. This guide outlines expectations for contributors and maintainers.

## Core Principles
- **Safety first:** prefer safe Rust; justify any `unsafe` blocks with inline rationale.
- **Auditability:** ensure every privileged action can be traced via the audit log.
- **Feature isolation:** gate experimental components behind Cargo features.
- **Deterministic tests:** favor hermetic mocks over network dependencies.

## Repository Layout
```
src/
  cli/              # REPL, theming, command routing
  core/             # Config, sessions, exploit framework, result store
  handlers/         # Local/remote shell handlers and security policies
  infra/            # Cryptography helpers, shared infrastructure
  memory_forensics/ # Dump parsing, analyzers, MITRE mapping
  modules/          # Operational modules (C2, recon, post, etc.)
tests/              # End-to-end integration suites
plugins/            # YARA rules and optional Volatility bridge
```

## Coding Standards
- Rustfmt is mandatory (`cargo fmt`).
- Clippy is encouraged (`cargo clippy --all-targets --all-features`).
- Keep modules under 500 lines or split into submodules.
- Document public APIs with Rustdoc comments and real examples.
- Use descriptive error contexts via `anyhow::Context`.

## Contribution Workflow
1. Fork and branch from `main` (`git checkout -b feat/<short-description>`).
2. Enable guardrails: `pre-commit install` (if available) and run `cargo fmt`.
3. Implement changes with feature flags when adding optional dependencies.
4. Update documentation and module metadata (`src/core/module_metadata.rs`).
5. Run the full test matrix (see [Testing & CI](testing-and-ci.md)).
6. Open a pull request linking to relevant issues and design docs.

## Review Checklist
- Authorization paths exercised and covered by tests.
- No logging of sensitive data beyond hashed identifiers.
- Memory analyzers scrub temporary files and drop privileges when possible.
- CLI help output remains concise and up to date.

## Extending Modules
- Add new modules under the appropriate category in `src/modules`.
- Register metadata in `src/modules/mod.rs` and update [Modules Catalog](modules.md).
- Provide example usage snippets in [Usage Guide](usage-guide.md).
- Supply regression tests in `tests/` or via `#[cfg(test)]` blocks.

## Support & Communication
- Primary maintainer: Abdulwahed Mansour (`@abdulwahed-sweden`).
- Security advisories: security@ferox.local.
- Community discussions: GitHub Discussions (planned).

---
_Version 2.0.0 • Updated 2025-11-12 • Contact: security@ferox.local_
