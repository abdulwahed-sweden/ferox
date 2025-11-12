---
title: Ferox Modules Catalog
description: Categorized overview of Ferox 2.0.0 modules and their primary capabilities.
---

# Modules Catalog

This catalog organizes the Ferox module ecosystem to help operators plan engagements and automate playbooks. All modules inherit shared option handling, authorization checks, and audit logging.

## Command & Control (C2)
| Module | Description | Notes |
| --- | --- | --- |
| `c2/http_beacon` | Resilient HTTP(S) beacon with jitter and task queueing. | Supports proxy chains and TLS pinning. |
| `c2/dns_c2` | DNS tunneling channel for low-bandwidth exfiltration. | Encodes tasks via TXT records with rate limiting. |
| `c2/relay_manager` | Relay control plane for multi-operator workflows. | Handles peer registration and encrypted fan-out. |
| `c2/teams_tunnel` | Microsoft Teams-based C2 over Graph API. | Requires tenant app registration and scoped auth. |
| `c2/cloud_tunnel` | Cloud broker route, optimized for covert SaaS pivoting. | Designed for red-teaming cloud-first environments. |
| `c2/command_scheduler` | Engagement-wide job scheduler for periodic actions. | Coordinates time-based delivery with audit linkage. |

## Evasion
| Module | Description | Notes |
| --- | --- | --- |
| `evasion/edr/silent_shadow` | Endpoint detection response evasion primitives. | Ships with process hollowing, DLL stomp, and delay guards. |

## Reconnaissance
| Module | Description | Notes |
| --- | --- | --- |
| `recon/asn` | Autonomous system number discovery and attribution. | Maps target IPs to companies and regions. |
| `recon/dns` | DNS record enumeration with subdomain brute force. | Supports custom resolvers and threading controls. |
| `recon/subdomains` | Aggregates wordlists, certificates, and APIs for subdomains. | Integrates with `wordlist.txt` and remote feeds. |
| `recon/whois` | WHOIS lookup and contact detail extraction. | Normalizes registry outputs for reporting. |

## Scanning
| Module | Description | Notes |
| --- | --- | --- |
| `scanner/http_scanner` | Enumerates HTTP verbs, headers, and technology hints. | Supports concurrency tuning and reporting integration. |
| `scanner/port` | High-speed TCP port scanner with banner detection. | Built on async runtime with rate-limit controls. |

## Post-Exploitation
| Module | Description | Notes |
| --- | --- | --- |
| `post/browser/deep_session_hijack` | Harvests browser session state for lateral movement. | Includes mock mode for tabletop exercises. |

## Auxiliary
| Module | Description | Notes |
| --- | --- | --- |
| `auxiliary/cloud/onedrive_sync_exfil` | OneDrive sync-exfiltration workflow. | Uses safe-mode guardrails to prevent accidental exfil. |

## Memory Forensics Suite
| Component | Description | Notes |
| --- | --- | --- |
| `memory analyze` | End-to-end dump analysis with SQLite evidence storage. | Generates consolidated JSON/CSV reports. |
| `memory pslist` | Process inventory with parent-child reconstruction. | Flags suspicious parentage and integrity gaps. |
| `memory malfind` | Injection detection with YARA integration. | Outputs code injection summaries and heuristics. |
| `memory netscan` | Socket timeline and beacon clustering. | Matches MITRE ATT&CK techniques for command staging. |
| `memory hashdump` | Credential artifact extraction across SAM/LSA secrets. | Auto-scrubs results in safe-mode runs. |
| `memory mitre` | MITRE ATT&CK technique mapping and scoring. | Adds tags to `memory_analysis` database tables. |

---
_Version 2.0.0 • Updated 2025-11-12 • Contact: security@ferox.local_
