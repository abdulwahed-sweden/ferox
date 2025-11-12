# Ferox Memory Forensics Module

The Ferox memory forensics module provides rapid, in-framework analysis of Windows memory dumps.

## Quick Start

```bash
cargo build --release --features memory-forensics
./target/release/ferox memory analyze memory.dmp --output report.json
```

## Capabilities

- Process inventory (`pslist`, `pstree`)
- Code injection heuristics (`malfind`)
- Network artifact extraction (`netscan`)
- Registry context inspection (`hivelist`, `printkey`)
- Credential harvesting heuristics (`hashdump`)
- MITRE ATT&CK technique mapping (`mitre`)

## Architecture

```
src/memory_forensics/
├── dump_parser.rs        # Memory dump ingestion helpers
├── process_analyzer.rs   # Process heuristics
├── network_analyzer.rs   # Network artifact discovery
├── registry_analyzer.rs  # Registry parsing heuristics
├── credential_extractor.rs
├── malware_detector.rs   # Malware heuristics + YARA bridge
├── mitre_mapper.rs       # MITRE ATT&CK mapping
├── types.rs              # Shared data structures
└── mod.rs
```

## CLI Usage

```bash
ferox memory analyze memory.dmp --output full_report.json
ferox memory pslist memory.dmp
ferox memory malfind memory.dmp
ferox memory netscan memory.dmp
ferox memory mitre memory.dmp --output mitre_report.json
```

## Database Storage

Analysis reports are stored in `ferox_memory.db` via `MemoryAnalysisDB`, enabling offline triage and enrichment.
