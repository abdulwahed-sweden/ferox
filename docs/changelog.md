---
title: Ferox Changelog
description: Highlights of Ferox releases leading up to version 2.0.0.
---

# Changelog

## 2.0.0 · 2025-11-12
- Introduced memory forensics suite with process, malware, network, registry, and credential analyzers.
- Added SQLite-backed `memory_analysis` storage with MITRE ATT&CK correlation.
- Expanded CLI with `memory` command group and safe-mode mock execution flows.
- Hardened authorization framework with scope validation and audit enrichment.
- Documented new architecture, usage patterns, and contribution standards.

## 1.5.x · 2025-06
- Delivered multi-channel C2 modules (Teams tunnel, HTTP beacon, relay manager).
- Enhanced session management with SQLite persistence and heartbeat tracking.
- Added silent shadow EDR evasion and browser post-exploitation modules.
- Improved configuration management via hierarchical TOML profiles.

## 1.2.x · 2024-11
- Formalized module metadata, dependency resolution, and option unification.
- Integrated audit logging with tamper-resistant append-only storage.
- Established safe-mode confirmation prompts for high-risk actions.

## 1.0.0 · 2024-03
- Initial release featuring scanner, recon, and auxiliary modules.
- Launched REPL-driven CLI with interactive help, theming, and scripting support.
- Shipped foundational testing harness with integration scenarios.

---
_Version 2.0.0 • Updated 2025-11-12 • Contact: security@ferox.local_
