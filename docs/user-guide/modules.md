# Module Catalog

Ferox modules are grouped by category inside `src/modules`. Each module implements the `Module` trait, advertises metadata, exposes options, and runs asynchronously through the console or programmatic APIs.

## Listing Modules
```bash
ferox console
ferox> modules
ferox> modules | grep c2/
```
Or filter directly from the CLI:
```bash
ferox -- console <<'EOF'
modules
EOF
```

## Categories
### Recon & Scanning
- `scanner/http_scanner` — async HTTP capability scanner.
- `scanner/port` — TCP reachability with CIDR support.
- `recon/subdomains`, `recon/dns`, `recon/asn`, `recon/whois`.

### Exploit & Post-Exploitation
- `exploit/example` — reference exploit scaffold.
- `post/browser/deep_session_hijack` — browser session takeover techniques.
- `evasion/edr/silent_shadow` — research-grade EDR bypass flows.

### Auxiliary & Cloud
- `auxiliary/cloud/onedrive_sync_exfil` — exfiltration via cloud sync abuse.

### Command & Control
- `c2/teams_tunnel` — meeting-based covert channel.
- `c2/github_c2` — GitHub Gist dead-drop comms.
- `c2/relay_manager` — session fan-out scaffolding.

### Memory Forensics
Accessible via `ferox memory …` or `memory` console command:
- `memory analyze`, `pslist`, `pstree`, `netscan`, `malfind`, `hashdump`, `yarascan`, `mitre`, and more.

## Module Lifecycle
1. `modules` → discover.
2. `use <category/name>`.
3. `options` / `show options` → inspect required fields.
4. `set KEY VALUE`.
5. `run` or `check`.
6. Review `sessions`, `result_store`, or exported reports.

## Writing Modules
- Follow Rust Edition 2024 formatting.
- Put reusable helpers in `src/core` or `src/infra`.
- Document new modules inside `docs/modules.md` when adding categories.

Ferox discourages destructive defaults—prompt users before impactful actions and ensure safe-mode gating where possible.
