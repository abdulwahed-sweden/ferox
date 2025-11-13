# Ferox Overview

Ferox 2.0.0 is a Rust-native, authorization-first framework for red teams and defenders who need fast reconnaissance, disciplined tradecraft, and incident-response grade memory visibility. Every subsystem is designed to be observable, auditable, and scriptable.

## Mission & Principles
- **Speed with safeguards:** Async Rust modules, aggressive caching, and safe-mode switches keep operations fast while respecting engagement controls.
- **Single operator surface:** The Ferox CLI Integration Layer launches doctor, memory, C2, sessions, or the full console from one binary.
- **Evidence always-on:** Session history, memory reports, and Ferox Doctor integrity scores capture proof without bolting on external tooling.
- **Mixed Predator Theme:** A unified design language ensures the console, doctor output, and CLI banners remain readable in dark SOCs and air-gapped labs.

## Architecture Layers
```
Operator
  │
  ├─ Ferox CLI Integration Layer (doctor, memory, c2, sessions, console)
  │
  ├─ Ferox Console (interactive module runner + Mixed Predator Theme)
  │
  ├─ Module System
  │     ├─ Recon & Scanning
  │     ├─ Exploit & Post-Exploitation
  │     ├─ Memory Forensics (Volatility3 + YARA)
  │     ├─ Command & Control Helpers
  │     └─ Maintenance & Diagnostics (Ferox Doctor)
  │
  └─ Data Services
        ├─ Session Manager (SQLite)
        ├─ Result Store / Reporting
        └─ Integrity Score telemetry
```

## Key Components
- **Ferox CLI Integration Layer:** Displays the startup banner, probes dependencies (Python, Volatility3, YARA), and routes subcommands before handing control to the console.
- **Ferox Console:** Provides module discovery, option management, execution, and handler orchestration with tab completion and guarded destructive actions.
- **Memory Forensics Pipeline:** Parses raw dumps, extracts artifacts, runs Volatility3-inspired analyses, executes YARA rules, and maps MITRE ATT&CK techniques.
- **Session Manager:** Tracks live implants or module sessions, stores command history, cleans stale entries, and powers `ferox sessions` automation.
- **C2 Layer:** Includes Teams tunnel, GitHub Gist C2, relay manager scaffolding, and configuration helpers for new transports.
- **Maintenance & Diagnostics:** Ferox Doctor validates dependencies, calculates an integrity score, and surfaces remediation guidance for SOC handoff.

## Security Posture
- **Authorized use only:** Safe-mode, audit logging, and explicit user prompts reduce accidental misuse.
- **Configurable controls:** `ferox_security.toml` enforces file-system sandboxes, blocked commands, and TLS options.
- **Deterministic builds:** Rust Edition 2024 + reproducible release profiles produce verifiable binaries.

Use Ferox when you need an auditable alternative to legacy exploit or forensic stacks without sacrificing velocity.
