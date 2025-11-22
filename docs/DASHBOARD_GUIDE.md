# Ferox Dashboard - Complete User Guide

## Table of Contents
1. [Getting Started](#getting-started)
2. [Dashboard Overview](#dashboard-overview)
3. [Session Management](#session-management)
4. [Command Terminal](#command-terminal)
5. [Network Visualization](#network-visualization)
6. [Credentials Vault](#credentials-vault)
7. [MITRE ATT&CK Matrix](#mitre-attck-matrix)
8. [Reports Generation](#reports-generation)
9. [Keyboard Shortcuts](#keyboard-shortcuts)
10. [Troubleshooting](#troubleshooting)

---

## Getting Started

### Prerequisites
- Ferox backend server running
- Modern web browser (Chrome, Firefox, Safari, Edge)

### Starting the Dashboard

```bash
# Start backend server
cd ferox
cargo run --release -p ferox-dashboard-server

# Dashboard automatically serves at:
# http://localhost:8080
```

### First Launch
1. Open browser at `http://localhost:8080`
2. Dashboard loads with available sessions
3. Green connection indicator shows WebSocket is connected
4. You're ready to start!

---

## Dashboard Overview

### Main Interface

```
+----------------------------------------------+
| FEROX C2    [Sessions: 5] [Creds: 127]  [OP] |
+--------+-------------------------------------+
|        |                                     |
|  Nav   |         Main Content                |
|        |                                     |
| Dashboard |  [Stats Cards]                   |
| Sessions  |  [Activity Timeline]             |
| Terminal  |  [Session List]                  |
| Network   |                                  |
| Creds     |                                  |
| MITRE     |                                  |
| Reports   |                                  |
+--------+-------------------------------------+
```

### Navigation
- **Dashboard**: Overview with statistics
- **Sessions**: Detailed session management
- **Terminal**: Interactive command execution
- **Network**: Network topology visualization
- **Credentials**: Credential vault
- **MITRE**: ATT&CK coverage matrix
- **Reports**: Generate and download reports

### Connection Status
- **Green indicator**: Connected to server
- **Red indicator**: Disconnected (will auto-reconnect)

---

## Session Management

### Session List

Each session displays:
- **Status Indicator**: Active (green) | Sleeping (yellow) | Dead (red)
- **Hostname**: Computer name
- **IP Address**: Network address
- **Operating System**: Windows/Linux/macOS
- **User**: Current username
- **Privileges**: User/Administrator/SYSTEM/Root
- **Credentials**: Count of captured credentials
- **Last Seen**: Time since last activity

### Quick Actions (Dropdown Menu)
- **Execute Command**: Open terminal
- **Escalate Privileges**: Run privilege escalation
- **Harvest Credentials**: Capture credentials
- **Lateral Movement**: Spread to other systems
- **View Details**: Full session information
- **Terminate**: Close session

### Session Detail Modal
Click any session to view:
- System information (OS, architecture)
- Domain membership and AV detection
- Session metrics (commands, credentials, files)
- Intelligence data

---

## Command Terminal

### Opening Terminal
1. Click session from list, OR
2. Use Quick Actions > "Execute Command"

### Terminal Interface

```
+------------------------------------------+
| [DC01.corp.local]$ _             [Clear] |
+------------------------------------------+
|                                          |
| [21:45:32] $ whoami                      |
| NT AUTHORITY\SYSTEM                      |
| Completed                                |
|                                          |
| [21:46:05] $ ipconfig /all               |
| [output...]                              |
|                                          |
+------------------------------------------+
  [whoami] [hostname] [ipconfig] [net user]
```

### Features
- **Command Input**: Type commands at prompt
- **Enter**: Execute command
- **Arrow Up/Down**: Navigate command history
- **Streaming Output**: See output as it arrives
- **Timestamps**: Each command shows execution time
- **Status Indicators**: Success (green) | Error (red)
- **Copy Output**: Click copy icon on any output
- **Quick Commands**: One-click common commands

### Quick Command Buttons
- **whoami**: Current user
- **hostname**: Computer name
- **ipconfig**: Network configuration
- **net user**: Domain users

### Ferox Commands
- **Auto PrivEsc**: `ferox privesc --auto`
- **Harvest Creds**: `ferox creds harvest --all`
- **Install Persist**: `ferox persist install --stealth`
- **Network Scan**: `ferox lateral discover`

---

## Network Visualization

### Graph Overview
Interactive network topology showing all discovered hosts and relationships.

### Node Types
- **Diamond**: Domain Controller (critical)
- **Rectangle**: Server
- **Ellipse**: Workstation

### Node Colors
- **Green border**: Compromised (active session)
- **Gray border**: Discovered (no session)

### Controls
- **Zoom In/Out**: Buttons or mouse wheel
- **Pan**: Click and drag
- **Fit View**: Center all nodes
- **Layout**: Change arrangement (Hierarchical/Circle/Grid)

### Interactions
1. **Click Node**: Shows details panel on right
2. **Details Panel**:
   - Hostname, IP, OS information
   - Session status and privileges
   - "Open Terminal" for compromised nodes
   - "Attack Target" for discovered nodes

---

## Credentials Vault

### Two-Column Layout

```
+----------------+--------------------+
| Credential     | Selected Details   |
| List           |                    |
|                | Username: admin    |
| Plain Text (45)| Password: *****    |
| NTLM Hash (32) | [Show] [Copy]      |
| Kerberos (8)   |                    |
| SSH Keys (15)  | Source: DC01       |
| Cloud (9)      | Reusable: Yes      |
+----------------+--------------------+
```

### Credential Types
- **Plain Text**: Ready to use passwords
- **NTLM Hashes**: For Pass-the-Hash attacks
- **Kerberos Tickets**: For Pass-the-Ticket
- **SSH Keys**: For SSH authentication
- **Cloud Credentials**: AWS, Azure, GCP
- **Tokens**: API tokens, certificates

### Search & Filter
- **Search Box**: Filter by username/hostname
- **Type Filter**: Show specific credential types
- **Sensitivity Filter**: Critical/High/Medium/Low
- **Reusable Only**: Show credentials usable for lateral movement

### Actions
- **Show Secret**: Reveal password (redacted by default)
- **Copy**: Copy to clipboard
- **Test**: Test against discovered targets
- **Lateral**: Use for lateral movement
- **Delete**: Remove credential
- **Note**: Add custom notes

### Intelligence Panel
Shows automatic analysis:
- Password reuse detection
- Domain admin credentials (critical)
- Cloud admin access
- Recommendations

---

## MITRE ATT&CK Matrix

### Coverage View
Interactive heat map showing technique usage.

### Color Legend
- **Green**: Used successfully, low detection risk
- **Yellow**: Used, medium detection risk
- **Red**: Used, high detection risk
- **Gray**: Available but not used

### Statistics
- **Overall Coverage**: Percentage of techniques used
- **Techniques Used**: Count
- **Tactics Covered**: Count
- **High Risk**: Count of high-risk techniques

### Technique List
Shows all techniques used in current session with:
- Technique ID and name
- Tactic category
- Detection risk level

---

## Reports Generation

### Creating Reports

1. **Select Template**:
   - Executive Summary
   - Technical Findings
   - Attack Path Analysis
   - Credentials Report
   - MITRE Mapping
   - Session Activity Log

2. **Select Format**:
   - PDF
   - HTML
   - JSON

3. **Generate and Download**

### Report Contents

**Executive Report**:
- Campaign summary
- Key findings
- Risk assessment

**Technical Report**:
- Detailed timeline
- Command logs
- Credentials (redacted)
- MITRE technique breakdown

---

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Escape` | Close modal |
| `Enter` | Execute command (in terminal) |
| `Arrow Up` | Previous command (terminal) |
| `Arrow Down` | Next command (terminal) |

---

## Troubleshooting

### Dashboard Won't Load
**Solutions**:
1. Check backend is running: `curl http://localhost:8080/api/sessions`
2. Check browser console for errors (F12)
3. Clear browser cache
4. Try different browser

### WebSocket Not Connecting
**Solutions**:
1. Check connection indicator color
2. Verify backend is running
3. Check firewall settings
4. Refresh page

### Commands Not Executing
**Solutions**:
1. Check session is Active (green)
2. Verify WebSocket connection
3. Try simple command (e.g., `whoami`)

### Network Graph Not Rendering
**Solutions**:
1. Check browser console for errors
2. Refresh page
3. Clear browser cache

---

## Best Practices

### Performance
- Close unused terminals
- Use filters to reduce displayed items
- Clear terminal output periodically

### Security
- Use HTTPS in production
- Enable authentication when available
- Use stealth modes in sensitive environments

### Workflow
1. Start with Dashboard overview
2. Review sessions and select target
3. Execute reconnaissance commands
4. Harvest credentials
5. Visualize network topology
6. Plan lateral movement
7. Generate report

---

**Happy Hunting!**
