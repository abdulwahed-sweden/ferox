---
title: Ferox Testing & CI
description: Verification strategy and continuous integration workflow for Ferox 2.0.0.
---

# Testing & CI

Ferox relies on layered testing to guarantee reliable, authorized operations. This document consolidates the verification pipeline and recommended automation hooks.

## Test Matrix
| Stage | Command | Purpose |
| --- | --- | --- |
| Lint | `cargo fmt --all` | Enforce style and import ordering. |
| Static Analysis | `cargo clippy --all-targets --all-features -- -D warnings` | Catch common mistakes and enforce idioms. |
| Unit Tests | `cargo test --lib` | Validate core services, handlers, and modules. |
| Feature Tests | `cargo test --features memory-forensics --tests` | Exercise memory analyzers and integration storage. |
| Integration | `cargo test --test integration_tests` | Run CLI and module orchestration end-to-end. |
| Smoke (Optional) | `SAFE_MODE=1 cargo run -- mock run c2/teams_tunnel` | Ensure CLI boots and core wiring works without touching targets. |

## Continuous Integration Blueprint
1. **Setup:** Install Rust stable via `rustup`, cache `target/` and `.cargo/` directories.
2. **Dependencies:** Fetch crates (`cargo fetch`) and install auxiliary tools (clippy, fmt).
3. **Build:** `cargo build --all-features --locked` to ensure reproducible builds.
4. **Lint & Test:** run the full matrix above; fail fast on any warning.
5. **Artifacts:** Publish `target/debug/ferox` or `target/release/ferox` binaries for manual QA.
6. **Reports:** Upload coverage (if using `cargo tarpaulin`) and attach audit logs for review.

## Local Developer Workflow
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --features memory-forensics --tests
cargo test --all -- --ignored
```

Optional: use `just test-all` if you maintain a `justfile` or `make ci` for more complex orchestrations.

## Quality Gates
- No `println!` debugging left in merges—use tracing or logging macros.
- 100% coverage for newly added memory analysis components.
- Ensure migrations to the SQLite schema ship with backward-compatible upgrades.
- Integration tests run in safe mode to avoid external network dependency.

For contribution guidance, see the [Developer Guide](developer-guide.md). For specific module behaviors reference the [Modules Catalog](modules.md).

---
_Version 2.0.0 • Updated 2025-11-12 • Contact: security@ferox.local_
