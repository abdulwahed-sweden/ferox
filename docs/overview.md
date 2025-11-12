---
title: Ferox Overview
description: Architecture, mission, and guiding principles of the Ferox 2.0.0 offensive security framework.
---

# Ferox Overview

Ferox 2.0.0 is a Rust-based offensive security operations framework engineered for authorized red teams, incident responders, and research labs that demand speed, safety, and accountability. The platform blends high-performance task orchestration with a hardened security model, making it suitable for enterprise deployments and regulated environments.

## Mission & Scope
- Deliver a modern alternative to legacy exploitation frameworks with Rust-level guarantees.
- Provide analysts with modular C2, reconnaissance, post-exploitation, and memory forensics capabilities.
- Enforce authorization boundaries, auditability, and safe-mode workflows by default.

## Architecture Summary
| Layer | Responsibilities | Key Components |
| --- | --- | --- |
| Interface | REPL CLI, theming, interactive help, non-interactive commands | `src/cli` |
| Core Services | Session tracking, configuration, result storage, reporting, audit trail | `src/core` |
| Execution Engine | Exploit framework, module metadata, option resolution, job scheduler | `src/core/*` |
| Modules | Scanner, recon, C2, evasion, post-exploitation, memory forensics | `src/modules`, `src/memory_forensics` |
| Integrations | SQLite persistence, optional Volatility bridge, YARA rule packs | `src/core/memory_analysis.rs`, `plugins/` |

## Security Posture
- Authorization contexts define engagement scope, targets, and expiry windows.
- Safe mode prompts protect dangerous actions in production and lab deployments.
- Immutable audit logging records all privileged operations and command results.
- Memory forensics modules isolate analysis environments and limit data persistence.

## Documentation Map
- [Modules Catalog](modules.md)
- [Usage Guide](usage-guide.md)
- [Developer Guide](developer-guide.md)
- [Testing & CI](testing-and-ci.md)
- [Memory Forensics Deep Dive](memory-forensics.md)
- [Changelog](changelog.md)

---
_Version 2.0.0 • Updated 2025-11-12 • Contact: security@ferox.local_
