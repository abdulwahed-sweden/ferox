# Ferox Memory Forensics Plugin Development Guide

## Volatility 3 Adapters

Custom Volatility adapters are stored under `plugins/volatility_plugins/`.

```text
plugins/
└── volatility_plugins/
    ├── ferox_pslist.py
    ├── ferox_malfind.py
    └── ferox_netscan.py
```

Each adapter extends a Volatility plugin and exposes results through the PyO3 bridge.

### Example Skeleton

```python
from volatility3.plugins.windows import pslist

class FeroxPsList(pslist.PsList):
    """Expose pslist data"""
```

## YARA Rules

YARA rules reside in `plugins/yara_rules/`. To add a new rule:

1. Drop the `.yar` file into the directory.
2. Run `ferox memory yarascan memory.dmp --rules plugins/yara_rules/my_rules.yar`.

## Rust Module Extensions

Add new analyzers under `src/memory_forensics/` and register them inside `mod.rs` for re-export.
