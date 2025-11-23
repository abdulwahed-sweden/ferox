//! Simulation Telemetry Commands
//!
//! This module provides SIMULATED telemetry data for all UI modules.
//! All data is fake and generated for demo/training purposes only.
//! No real system access, network scanning, or credential harvesting occurs.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use rand::Rng;
use uuid::Uuid;

// ============================================================================
// Network Scanner Simulation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedHost {
    pub id: String,
    pub ip: String,
    pub hostname: String,
    pub mac: String,
    pub os: String,
    pub os_version: String,
    pub vendor: String,
    pub ports: Vec<SimulatedPort>,
    pub status: String, // "up" or "down"
    pub latency_ms: f64,
    pub ttl: u8,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedPort {
    pub port: u16,
    pub protocol: String,
    pub service: String,
    pub version: String,
    pub state: String, // "open", "closed", "filtered"
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkScanResult {
    pub hosts: Vec<SimulatedHost>,
    pub scan_duration_ms: u64,
    pub total_hosts_scanned: u32,
    pub hosts_up: u32,
    pub hosts_down: u32,
}

#[tauri::command]
pub async fn simulate_network_scan(
    subnet: String,
    _session_id: String,
) -> Result<NetworkScanResult, String> {
    // Simulate scan delay
    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

    let mut rng = rand::thread_rng();
    let mut hosts = Vec::new();

    // Generate realistic simulated hosts
    let host_templates = vec![
        ("gateway.local", "Linux 4.x", "Router/Firewall", "Cisco", vec![
            (22, "SSH", "OpenSSH 8.4", "open"),
            (80, "HTTP", "nginx 1.18", "open"),
            (443, "HTTPS", "nginx 1.18", "open"),
        ]),
        ("dc01.corp.local", "Windows Server 2019", "Domain Controller", "Dell", vec![
            (53, "DNS", "Microsoft DNS", "open"),
            (88, "Kerberos", "Microsoft Kerberos", "open"),
            (135, "MSRPC", "Microsoft RPC", "open"),
            (389, "LDAP", "Microsoft LDAP", "open"),
            (445, "SMB", "SMBv3", "open"),
            (636, "LDAPS", "Microsoft LDAPS", "open"),
            (3268, "LDAP-GC", "Global Catalog", "open"),
            (3389, "RDP", "Microsoft RDP", "open"),
        ]),
        ("web01.corp.local", "Ubuntu 22.04 LTS", "Web Server", "HP", vec![
            (22, "SSH", "OpenSSH 8.9", "open"),
            (80, "HTTP", "Apache 2.4.52", "open"),
            (443, "HTTPS", "Apache 2.4.52", "open"),
            (3306, "MySQL", "MySQL 8.0.32", "filtered"),
            (8080, "HTTP-Alt", "Tomcat 9.0", "open"),
        ]),
        ("db01.corp.local", "CentOS 8", "Database Server", "IBM", vec![
            (22, "SSH", "OpenSSH 8.0", "open"),
            (1433, "MSSQL", "SQL Server 2019", "open"),
            (3306, "MySQL", "MySQL 8.0", "filtered"),
            (5432, "PostgreSQL", "PostgreSQL 14", "open"),
        ]),
        ("mail.corp.local", "Windows Server 2016", "Mail Server", "Dell", vec![
            (25, "SMTP", "Microsoft SMTP", "open"),
            (110, "POP3", "Microsoft POP3", "open"),
            (143, "IMAP", "Microsoft IMAP", "open"),
            (443, "HTTPS", "OWA", "open"),
            (587, "SMTP", "Microsoft SMTP", "open"),
        ]),
        ("fileserver.corp.local", "Windows Server 2019", "File Server", "HP", vec![
            (135, "MSRPC", "Microsoft RPC", "open"),
            (139, "NetBIOS", "NetBIOS-SSN", "open"),
            (445, "SMB", "SMBv3", "open"),
            (3389, "RDP", "Microsoft RDP", "filtered"),
        ]),
        ("workstation01", "Windows 10 Pro", "Workstation", "Lenovo", vec![
            (135, "MSRPC", "Microsoft RPC", "open"),
            (445, "SMB", "SMBv3", "open"),
            (3389, "RDP", "Microsoft RDP", "closed"),
        ]),
        ("workstation02", "Windows 11 Pro", "Workstation", "Dell", vec![
            (135, "MSRPC", "Microsoft RPC", "open"),
            (445, "SMB", "SMBv3", "open"),
        ]),
        ("dev-linux01", "Debian 11", "Development Server", "Custom Build", vec![
            (22, "SSH", "OpenSSH 8.4", "open"),
            (3000, "HTTP", "Node.js", "open"),
            (5000, "HTTP", "Flask", "open"),
            (8000, "HTTP", "Django", "open"),
            (9000, "HTTP", "PHP-FPM", "filtered"),
        ]),
        ("printer01.corp.local", "Embedded Linux", "Network Printer", "HP", vec![
            (80, "HTTP", "HP Web Interface", "open"),
            (443, "HTTPS", "HP Web Interface", "open"),
            (515, "LPD", "Line Printer Daemon", "open"),
            (631, "IPP", "CUPS", "open"),
            (9100, "JetDirect", "HP JetDirect", "open"),
        ]),
    ];

    // Parse subnet base (simplified)
    let base_ip = subnet.split('/').next().unwrap_or("192.168.1.0");
    let octets: Vec<&str> = base_ip.split('.').collect();
    let base = format!("{}.{}.{}.", octets.get(0).unwrap_or(&"192"),
                                    octets.get(1).unwrap_or(&"168"),
                                    octets.get(2).unwrap_or(&"1"));

    for (i, (hostname, os, os_ver, vendor, ports)) in host_templates.iter().enumerate() {
        let ip = format!("{}{}", base, i + 1);
        let is_up = rng.gen_bool(0.85); // 85% chance host is up

        let simulated_ports: Vec<SimulatedPort> = ports.iter().map(|(port, svc, ver, state)| {
            SimulatedPort {
                port: *port,
                protocol: "tcp".into(),
                service: svc.to_string(),
                version: ver.to_string(),
                state: state.to_string(),
                banner: if *state == "open" && rng.gen_bool(0.3) {
                    Some(format!("{} ready", svc))
                } else {
                    None
                },
            }
        }).collect();

        hosts.push(SimulatedHost {
            id: Uuid::new_v4().to_string(),
            ip,
            hostname: hostname.to_string(),
            mac: format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(),
                rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()),
            os: os.to_string(),
            os_version: os_ver.to_string(),
            vendor: vendor.to_string(),
            ports: if is_up { simulated_ports } else { vec![] },
            status: if is_up { "up".into() } else { "down".into() },
            latency_ms: if is_up { rng.gen_range(0.5..50.0) } else { 0.0 },
            ttl: if is_up { rng.gen_range(32..128) } else { 0 },
            last_seen: Utc::now(),
        });
    }

    let hosts_up = hosts.iter().filter(|h| h.status == "up").count() as u32;

    Ok(NetworkScanResult {
        hosts,
        scan_duration_ms: rng.gen_range(2000..8000),
        total_hosts_scanned: host_templates.len() as u32,
        hosts_up,
        hosts_down: host_templates.len() as u32 - hosts_up,
    })
}

// ============================================================================
// Credentials Viewer Simulation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedCredential {
    pub id: String,
    pub cred_type: String, // "password", "hash", "token", "certificate", "ticket"
    pub username: String,
    pub domain: Option<String>,
    pub value: String,
    pub source: String,
    pub sensitivity: String, // "low", "medium", "high", "critical"
    pub cracked: bool,
    pub cracked_value: Option<String>,
    pub last_used: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialDumpResult {
    pub credentials: Vec<SimulatedCredential>,
    pub total_found: u32,
    pub by_type: std::collections::HashMap<String, u32>,
    pub by_sensitivity: std::collections::HashMap<String, u32>,
}

#[tauri::command]
pub async fn simulate_credential_dump(
    _session_id: String,
    sources: Vec<String>,
) -> Result<CredentialDumpResult, String> {
    tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;

    let mut rng = rand::thread_rng();
    let mut credentials = Vec::new();

    // Simulated credential templates
    let cred_templates = vec![
        ("hash", "Administrator", Some("CORP"), "aad3b435b51404eeaad3b435b51404ee:31d6cfe0d16ae931b73c59d7e0c089c0", "LSASS Memory", "critical", true, Some("Password123!")),
        ("hash", "krbtgt", Some("CORP"), "aad3b435b51404eeaad3b435b51404ee:b21c99fc068e3ab2ca789bccbef67de4", "NTDS.dit", "critical", false, None),
        ("password", "svc_backup", Some("CORP"), "B@ckup2024!", "Credential Manager", "high", true, None),
        ("password", "svc_sql", Some("CORP"), "SQLServer@2024", "Registry (LSA Secrets)", "high", true, None),
        ("hash", "john.doe", Some("CORP"), "aad3b435b51404eeaad3b435b51404ee:e19ccf75ee54e06b06a5907af13cef42", "SAM Database", "medium", true, Some("Welcome1")),
        ("hash", "jane.smith", Some("CORP"), "aad3b435b51404eeaad3b435b51404ee:8846f7eaee8fb117ad06bdd830b7586c", "SAM Database", "medium", true, Some("password")),
        ("token", "github-actions", None, "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "Environment Variables", "high", true, None),
        ("token", "aws-cli", None, "AKIAIOSFODNN7EXAMPLE", "AWS Credentials File", "critical", true, None),
        ("password", "admin", None, "admin123", "Browser (Chrome)", "low", true, None),
        ("password", "root", None, "toor", "SSH Config", "medium", true, None),
        ("certificate", "web-server", Some("corp.local"), "-----BEGIN CERTIFICATE-----\nMIIBkTCB+wIJAKHBfp...", "Certificate Store", "high", false, None),
        ("ticket", "Administrator", Some("CORP"), "doIFqjCCBaagAwIBBaEDAgEW...", "Kerberos Cache", "critical", false, None),
        ("password", "backup_user", None, "Backup#2024", "Scheduled Tasks", "high", true, None),
        ("hash", "WORKSTATION01$", Some("CORP"), "aad3b435b51404eeaad3b435b51404ee:a87f3a337d73085c45f9416be5787d86", "Machine Account", "medium", false, None),
    ];

    for (i, (cred_type, username, domain, value, source, sensitivity, cracked, cracked_val)) in cred_templates.iter().enumerate() {
        // Filter by source if specified
        if !sources.is_empty() && !sources.iter().any(|s| source.to_lowercase().contains(&s.to_lowercase())) {
            continue;
        }

        credentials.push(SimulatedCredential {
            id: format!("cred-{}", i + 1),
            cred_type: cred_type.to_string(),
            username: username.to_string(),
            domain: domain.map(|d| d.to_string()),
            value: value.to_string(),
            source: source.to_string(),
            sensitivity: sensitivity.to_string(),
            cracked: *cracked,
            cracked_value: cracked_val.map(|v| v.to_string()),
            last_used: if rng.gen_bool(0.6) {
                Some(Utc::now() - Duration::hours(rng.gen_range(1..720)))
            } else {
                None
            },
            expires_at: if *cred_type == "ticket" || *cred_type == "token" {
                Some(Utc::now() + Duration::hours(rng.gen_range(1..168)))
            } else {
                None
            },
            notes: None,
        });
    }

    let mut by_type = std::collections::HashMap::new();
    let mut by_sensitivity = std::collections::HashMap::new();

    for cred in &credentials {
        *by_type.entry(cred.cred_type.clone()).or_insert(0) += 1;
        *by_sensitivity.entry(cred.sensitivity.clone()).or_insert(0) += 1;
    }

    Ok(CredentialDumpResult {
        total_found: credentials.len() as u32,
        credentials,
        by_type,
        by_sensitivity,
    })
}

// ============================================================================
// Event Log Simulation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedLogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: String, // "info", "warn", "error", "success", "debug"
    pub module: String,
    pub message: String,
    pub details: Option<String>,
    pub session_id: Option<String>,
}

#[tauri::command]
pub async fn simulate_event_log(
    count: Option<u32>,
) -> Result<Vec<SimulatedLogEntry>, String> {
    let mut rng = rand::thread_rng();
    let count = count.unwrap_or(50).min(200);
    let mut logs = Vec::new();

    let modules = vec!["Scanner", "Payload", "Session", "C2", "PrivEsc", "Creds", "Lateral", "Persist", "System", "Network"];

    let messages_by_module: std::collections::HashMap<&str, Vec<&str>> = [
        ("Scanner", vec![
            "Port scan initiated on {ip}",
            "Host discovered: {ip} ({os})",
            "Service identified: {service} on port {port}",
            "Scan completed: {n} hosts found",
            "Vulnerability detected: {cve}",
            "Banner grabbed from {ip}:{port}",
        ]),
        ("Payload", vec![
            "Payload generated: {type} ({size} bytes)",
            "Obfuscation applied: {method}",
            "Signature check passed",
            "Shellcode encoded: {encoder}",
            "Payload staged for delivery",
        ]),
        ("Session", vec![
            "New session established from {ip}",
            "Session heartbeat received",
            "Session migrated to PID {pid}",
            "Session elevated to {priv}",
            "Session terminated gracefully",
            "Interactive shell spawned",
        ]),
        ("C2", vec![
            "Beacon received from {id}",
            "Command queued: {cmd}",
            "Response encrypted (AES-256)",
            "Channel established via {protocol}",
            "Jitter applied: {jitter}%",
            "Sleep timer set: {seconds}s",
        ]),
        ("PrivEsc", vec![
            "Enumeration complete: {n} vectors found",
            "UAC bypass attempted via {method}",
            "Token impersonation successful",
            "Privilege escalation to SYSTEM",
            "SeDebugPrivilege enabled",
            "Service permissions exploited",
        ]),
        ("Creds", vec![
            "Credential dump initiated",
            "Hash extracted: {user}",
            "Token captured from process {pid}",
            "Browser passwords retrieved: {n}",
            "Kerberos ticket harvested",
            "DPAPI masterkey decrypted",
        ]),
        ("Lateral", vec![
            "PSExec to {host} initiated",
            "WMI execution on {host}",
            "RDP session established to {host}",
            "SMB share enumerated: \\\\{host}\\{share}",
            "Pass-the-hash attempted to {host}",
            "WinRM connection established",
        ]),
        ("Persist", vec![
            "Registry persistence installed",
            "Scheduled task created: {name}",
            "Service installed: {service}",
            "Startup folder entry added",
            "WMI subscription created",
            "DLL search order hijack configured",
        ]),
        ("System", vec![
            "Module loaded: {module}",
            "Configuration updated",
            "Database synchronized",
            "Memory optimized",
            "Cleanup completed",
            "Health check passed",
        ]),
        ("Network", vec![
            "Connection established to {ip}:{port}",
            "DNS query: {domain}",
            "Traffic encrypted via {protocol}",
            "Proxy configured: {proxy}",
            "Firewall rule detected",
            "Network interface enumerated",
        ]),
    ].iter().cloned().collect();

    let levels = ["info", "info", "info", "success", "warn", "debug", "error"];
    let level_weights = [30, 30, 20, 10, 5, 4, 1]; // Weighted distribution

    for i in 0..count {
        let module = modules[rng.gen_range(0..modules.len())];
        let templates = messages_by_module.get(module).unwrap();
        let template = templates[rng.gen_range(0..templates.len())];

        // Generate message with placeholder replacements
        let message = template
            .replace("{ip}", &format!("192.168.1.{}", rng.gen_range(1..255)))
            .replace("{port}", &rng.gen_range(1..65535).to_string())
            .replace("{os}", &["Windows 10", "Ubuntu 22.04", "Windows Server 2019"][rng.gen_range(0..3)])
            .replace("{service}", &["SSH", "HTTP", "SMB", "RDP", "MySQL"][rng.gen_range(0..5)])
            .replace("{n}", &rng.gen_range(1..50).to_string())
            .replace("{cve}", &format!("CVE-2024-{}", rng.gen_range(1000..9999)))
            .replace("{type}", &["reverse_tcp", "reverse_https", "bind_tcp"][rng.gen_range(0..3)])
            .replace("{size}", &rng.gen_range(10000..100000).to_string())
            .replace("{method}", &["CMSTP", "Fodhelper", "EventViewer"][rng.gen_range(0..3)])
            .replace("{encoder}", &["shikata_ga_nai", "xor", "base64"][rng.gen_range(0..3)])
            .replace("{pid}", &rng.gen_range(100..10000).to_string())
            .replace("{priv}", &["Administrator", "SYSTEM", "root"][rng.gen_range(0..3)])
            .replace("{id}", &format!("session-{}", rng.gen_range(1..100)))
            .replace("{cmd}", &["whoami", "ipconfig", "netstat"][rng.gen_range(0..3)])
            .replace("{protocol}", &["HTTPS", "DNS", "ICMP"][rng.gen_range(0..3)])
            .replace("{jitter}", &rng.gen_range(5..30).to_string())
            .replace("{seconds}", &rng.gen_range(30..300).to_string())
            .replace("{user}", &["Administrator", "john.doe", "svc_backup"][rng.gen_range(0..3)])
            .replace("{host}", &format!("192.168.1.{}", rng.gen_range(1..255)))
            .replace("{share}", &["C$", "ADMIN$", "IPC$"][rng.gen_range(0..3)])
            .replace("{name}", &["UpdateTask", "SystemCheck", "Maintenance"][rng.gen_range(0..3)])
            .replace("{service}", &["UpdateService", "SystemHelper"][rng.gen_range(0..2)])
            .replace("{module}", &["scanner", "payload", "session"][rng.gen_range(0..3)])
            .replace("{domain}", &["corp.local", "example.com"][rng.gen_range(0..2)])
            .replace("{proxy}", &format!("socks5://127.0.0.1:{}", rng.gen_range(1080..9050)));

        // Select level with weights
        let total_weight: i32 = level_weights.iter().sum();
        let mut random_weight = rng.gen_range(0..total_weight);
        let mut selected_level = "info";
        for (level, &weight) in levels.iter().zip(level_weights.iter()) {
            random_weight -= weight;
            if random_weight < 0 {
                selected_level = level;
                break;
            }
        }

        logs.push(SimulatedLogEntry {
            id: format!("log-{}-{}", Utc::now().timestamp_millis(), i),
            timestamp: Utc::now() - Duration::seconds((count - i) as i64 * rng.gen_range(1..5)),
            level: selected_level.to_string(),
            module: module.to_string(),
            message,
            details: if rng.gen_bool(0.2) { Some("Additional context available".into()) } else { None },
            session_id: if rng.gen_bool(0.7) { Some(format!("session-{}", rng.gen_range(1..10))) } else { None },
        });
    }

    // Sort by timestamp descending (newest first)
    logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    Ok(logs)
}

// ============================================================================
// Task Scheduler Simulation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedTask {
    pub id: String,
    pub name: String,
    pub command: String,
    pub schedule: String,
    pub status: String, // "pending", "running", "completed", "failed", "paused"
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub run_count: u32,
    pub last_result: Option<String>,
    pub priority: String, // "low", "normal", "high", "critical"
}

#[tauri::command]
pub async fn simulate_scheduled_tasks(
    _session_id: String,
) -> Result<Vec<SimulatedTask>, String> {
    let mut rng = rand::thread_rng();

    let task_templates = vec![
        ("Beacon Heartbeat", "send_heartbeat()", "Every 30 seconds", "running", "critical"),
        ("Credential Harvest", "harvest_creds --all", "Every 5 minutes", "pending", "high"),
        ("Network Discovery", "scan 192.168.1.0/24 --quick", "Every 15 minutes", "completed", "normal"),
        ("Keylogger Dump", "keylog --dump", "Every 10 minutes", "running", "high"),
        ("Screenshot Capture", "screenshot --quality 80", "Every 2 minutes", "paused", "low"),
        ("Process Monitor", "ps --watch", "Every 1 minute", "running", "normal"),
        ("Exfil Queue", "exfil --process-queue", "Every 30 minutes", "pending", "high"),
        ("AV Evasion Check", "av_check --stealth", "Every 1 hour", "completed", "critical"),
        ("Persistence Verify", "persist --verify", "Every 6 hours", "pending", "normal"),
        ("Log Cleanup", "cleanup --logs --older-than 24h", "Every 12 hours", "completed", "low"),
    ];

    let tasks: Vec<SimulatedTask> = task_templates.iter().enumerate().map(|(i, (name, cmd, schedule, status, priority))| {
        let created_at = Utc::now() - Duration::hours(rng.gen_range(1..168));
        let last_run = if *status != "pending" {
            Some(Utc::now() - Duration::minutes(rng.gen_range(1..60)))
        } else {
            None
        };

        SimulatedTask {
            id: format!("task-{}", i + 1),
            name: name.to_string(),
            command: cmd.to_string(),
            schedule: schedule.to_string(),
            status: status.to_string(),
            last_run,
            next_run: Utc::now() + Duration::seconds(rng.gen_range(30..3600)),
            created_at,
            run_count: rng.gen_range(0..100),
            last_result: if *status == "completed" {
                Some("Success".into())
            } else if *status == "failed" {
                Some("Error: timeout".into())
            } else {
                None
            },
            priority: priority.to_string(),
        }
    }).collect();

    Ok(tasks)
}

// ============================================================================
// Notes Simulation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedNote {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pinned: bool,
    pub color: Option<String>,
}

#[tauri::command]
pub async fn simulate_session_notes(
    _session_id: String,
) -> Result<Vec<SimulatedNote>, String> {
    let mut rng = rand::thread_rng();

    let note_templates = vec![
        ("Initial Access Vector", "Gained access via phishing email to john.doe@corp.local\n\nAttachment: invoice_march.docx.exe\nExecution time: 2024-01-15 14:32:00 UTC\nInitial beacon to C2 confirmed.", vec!["initial-access", "phishing"], true),
        ("Domain Admin Path", "Identified path to DA:\n1. john.doe → svc_backup (password reuse)\n2. svc_backup → DC01 (backup operator privs)\n3. DCSync to extract krbtgt hash\n\nEstimated time: 2-3 hours", vec!["privilege-escalation", "attack-path"], true),
        ("Interesting Files Found", "Discovered on FILESERVER01:\n- \\\\fileserver\\IT$\\passwords.xlsx\n- \\\\fileserver\\Finance$\\budget_2024.xlsx\n- \\\\fileserver\\HR$\\employee_ssn.csv\n\nQueued for exfiltration.", vec!["loot", "sensitive-data"], false),
        ("OPSEC Notes", "Current detection status:\n- EDR: CrowdStrike (not alerting)\n- SIEM: Splunk (some DNS queries flagged)\n- Firewall: Palo Alto (egress allowed)\n\nRecommendation: Reduce beacon frequency to 5min", vec!["opsec", "evasion"], true),
        ("Network Segmentation", "Identified VLANs:\n- VLAN 10: Servers (192.168.10.0/24)\n- VLAN 20: Workstations (192.168.20.0/24)\n- VLAN 30: DMZ (10.0.30.0/24)\n- VLAN 99: Management (192.168.99.0/24)\n\nNo filtering between VLAN 10 and 20!", vec!["recon", "network"], false),
        ("Persistence Methods", "Installed persistence:\n1. Registry Run key (user-level)\n2. Scheduled task 'WindowsUpdate' (SYSTEM)\n3. WMI subscription (backup)\n\nAll methods verified working after reboot.", vec!["persistence", "backup"], false),
    ];

    let notes: Vec<SimulatedNote> = note_templates.iter().enumerate().map(|(i, (title, content, tags, pinned))| {
        let created = Utc::now() - Duration::hours(rng.gen_range(1..720));
        SimulatedNote {
            id: format!("note-{}", i + 1),
            title: title.to_string(),
            content: content.to_string(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
            created_at: created,
            updated_at: created + Duration::minutes(rng.gen_range(0..120)),
            pinned: *pinned,
            color: if rng.gen_bool(0.3) {
                Some(["#ff6b6b", "#4ecdc4", "#ffe66d", "#95e1d3"][rng.gen_range(0..4)].into())
            } else {
                None
            },
        }
    }).collect();

    Ok(notes)
}

// ============================================================================
// File Browser Simulation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedFileEntry {
    pub name: String,
    pub path: String,
    pub file_type: String, // "file" or "directory"
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub permissions: String,
    pub owner: String,
    pub group: String,
    pub hidden: bool,
    pub executable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryListing {
    pub path: String,
    pub entries: Vec<SimulatedFileEntry>,
    pub total_size: u64,
    pub parent: Option<String>,
}

#[tauri::command]
pub async fn simulate_directory_listing(
    path: String,
    _session_id: String,
) -> Result<DirectoryListing, String> {
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    let mut rng = rand::thread_rng();

    // Simulated directory structures based on path
    let entries = match path.as_str() {
        "/" | "/home" | "/home/user" | "C:\\" | "C:\\Users" | "C:\\Users\\Administrator" => {
            generate_home_directory(&mut rng, &path)
        }
        p if p.contains("Documents") => generate_documents_directory(&mut rng, &path),
        p if p.contains(".ssh") || p.contains("ssh") => generate_ssh_directory(&mut rng, &path),
        p if p.contains("Desktop") => generate_desktop_directory(&mut rng, &path),
        p if p.contains("Downloads") => generate_downloads_directory(&mut rng, &path),
        _ => generate_generic_directory(&mut rng, &path),
    };

    let total_size = entries.iter().map(|e| e.size).sum();
    let parent = if path == "/" || path == "C:\\" {
        None
    } else {
        Some(path.rsplit_once(['/', '\\']).map(|(p, _)| p.to_string()).unwrap_or("/".into()))
    };

    Ok(DirectoryListing {
        path: path.clone(),
        entries,
        total_size,
        parent,
    })
}

fn generate_home_directory(rng: &mut impl Rng, base_path: &str) -> Vec<SimulatedFileEntry> {
    let is_windows = base_path.contains("\\");
    let sep = if is_windows { "\\" } else { "/" };

    vec![
        SimulatedFileEntry {
            name: "Documents".into(),
            path: format!("{}{}{}", base_path, sep, "Documents"),
            file_type: "directory".into(),
            size: 0,
            modified: Utc::now() - Duration::days(rng.gen_range(1..30)),
            permissions: if is_windows { "drwxr-xr-x".into() } else { "drwxr-xr-x".into() },
            owner: "user".into(),
            group: "user".into(),
            hidden: false,
            executable: true,
        },
        SimulatedFileEntry {
            name: "Downloads".into(),
            path: format!("{}{}{}", base_path, sep, "Downloads"),
            file_type: "directory".into(),
            size: 0,
            modified: Utc::now() - Duration::hours(rng.gen_range(1..72)),
            permissions: "drwxr-xr-x".into(),
            owner: "user".into(),
            group: "user".into(),
            hidden: false,
            executable: true,
        },
        SimulatedFileEntry {
            name: "Desktop".into(),
            path: format!("{}{}{}", base_path, sep, "Desktop"),
            file_type: "directory".into(),
            size: 0,
            modified: Utc::now() - Duration::hours(rng.gen_range(1..24)),
            permissions: "drwxr-xr-x".into(),
            owner: "user".into(),
            group: "user".into(),
            hidden: false,
            executable: true,
        },
        SimulatedFileEntry {
            name: if is_windows { ".ssh" } else { ".ssh" }.into(),
            path: format!("{}{}{}", base_path, sep, ".ssh"),
            file_type: "directory".into(),
            size: 0,
            modified: Utc::now() - Duration::days(rng.gen_range(30..365)),
            permissions: "drwx------".into(),
            owner: "user".into(),
            group: "user".into(),
            hidden: true,
            executable: true,
        },
        SimulatedFileEntry {
            name: if is_windows { "NTUSER.DAT" } else { ".bashrc" }.into(),
            path: format!("{}{}{}", base_path, sep, if is_windows { "NTUSER.DAT" } else { ".bashrc" }),
            file_type: "file".into(),
            size: rng.gen_range(1000..50000),
            modified: Utc::now() - Duration::days(rng.gen_range(1..60)),
            permissions: "-rw-r--r--".into(),
            owner: "user".into(),
            group: "user".into(),
            hidden: !is_windows,
            executable: false,
        },
        SimulatedFileEntry {
            name: if is_windows { "ntuser.ini" } else { ".bash_history" }.into(),
            path: format!("{}{}{}", base_path, sep, if is_windows { "ntuser.ini" } else { ".bash_history" }),
            file_type: "file".into(),
            size: rng.gen_range(5000..100000),
            modified: Utc::now() - Duration::minutes(rng.gen_range(1..60)),
            permissions: "-rw-------".into(),
            owner: "user".into(),
            group: "user".into(),
            hidden: true,
            executable: false,
        },
    ]
}

fn generate_documents_directory(rng: &mut impl Rng, base_path: &str) -> Vec<SimulatedFileEntry> {
    let sep = if base_path.contains("\\") { "\\" } else { "/" };

    vec![
        ("passwords.txt", 2048, "-rw-r--r--", false),
        ("network_diagram.png", 524288, "-rw-r--r--", false),
        ("vpn_config.ovpn", 4096, "-rw-r--r--", false),
        ("budget_2024.xlsx", 102400, "-rw-r--r--", false),
        ("meeting_notes.docx", 51200, "-rw-r--r--", false),
        ("Private", 0, "drwx------", true),
        ("Work", 0, "drwxr-xr-x", true),
    ].iter().map(|(name, size, perms, is_dir)| {
        SimulatedFileEntry {
            name: name.to_string(),
            path: format!("{}{}{}", base_path, sep, name),
            file_type: if *is_dir { "directory" } else { "file" }.into(),
            size: *size,
            modified: Utc::now() - Duration::days(rng.gen_range(1..90)),
            permissions: perms.to_string(),
            owner: "user".into(),
            group: "user".into(),
            hidden: name.starts_with('.'),
            executable: *is_dir,
        }
    }).collect()
}

fn generate_ssh_directory(rng: &mut impl Rng, base_path: &str) -> Vec<SimulatedFileEntry> {
    let sep = if base_path.contains("\\") { "\\" } else { "/" };

    vec![
        ("id_rsa", 3243, "-rw-------", false),
        ("id_rsa.pub", 743, "-rw-r--r--", false),
        ("known_hosts", 8192, "-rw-r--r--", false),
        ("authorized_keys", 1536, "-rw-r--r--", false),
        ("config", 512, "-rw-r--r--", false),
    ].iter().map(|(name, size, perms, _)| {
        SimulatedFileEntry {
            name: name.to_string(),
            path: format!("{}{}{}", base_path, sep, name),
            file_type: "file".into(),
            size: *size,
            modified: Utc::now() - Duration::days(rng.gen_range(30..365)),
            permissions: perms.to_string(),
            owner: "user".into(),
            group: "user".into(),
            hidden: false,
            executable: false,
        }
    }).collect()
}

fn generate_desktop_directory(rng: &mut impl Rng, base_path: &str) -> Vec<SimulatedFileEntry> {
    let sep = if base_path.contains("\\") { "\\" } else { "/" };
    let is_windows = base_path.contains("\\");

    vec![
        (if is_windows { "Chrome.lnk" } else { "chrome.desktop" }, 2048, "-rw-r--r--"),
        (if is_windows { "passwords.txt" } else { "passwords.txt" }, 1024, "-rw-r--r--"),
        ("screenshot_2024.png", 1048576, "-rw-r--r--"),
        ("notes.txt", 4096, "-rw-r--r--"),
    ].iter().map(|(name, size, perms)| {
        SimulatedFileEntry {
            name: name.to_string(),
            path: format!("{}{}{}", base_path, sep, name),
            file_type: "file".into(),
            size: *size,
            modified: Utc::now() - Duration::hours(rng.gen_range(1..168)),
            permissions: perms.to_string(),
            owner: "user".into(),
            group: "user".into(),
            hidden: false,
            executable: false,
        }
    }).collect()
}

fn generate_downloads_directory(rng: &mut impl Rng, base_path: &str) -> Vec<SimulatedFileEntry> {
    let sep = if base_path.contains("\\") { "\\" } else { "/" };

    vec![
        ("putty.exe", 1048576, "-rw-r--r--"),
        ("nmap-setup.exe", 26214400, "-rw-r--r--"),
        ("document.pdf", 524288, "-rw-r--r--"),
        ("invoice_march.docx", 102400, "-rw-r--r--"),
        ("setup.exe", 5242880, "-rw-r--r--"),
    ].iter().map(|(name, size, perms)| {
        SimulatedFileEntry {
            name: name.to_string(),
            path: format!("{}{}{}", base_path, sep, name),
            file_type: "file".into(),
            size: *size,
            modified: Utc::now() - Duration::days(rng.gen_range(1..30)),
            permissions: perms.to_string(),
            owner: "user".into(),
            group: "user".into(),
            hidden: false,
            executable: name.ends_with(".exe"),
        }
    }).collect()
}

fn generate_generic_directory(rng: &mut impl Rng, base_path: &str) -> Vec<SimulatedFileEntry> {
    let sep = if base_path.contains("\\") { "\\" } else { "/" };

    vec![
        ("file1.txt", 1024, "-rw-r--r--"),
        ("file2.log", 8192, "-rw-r--r--"),
        ("data", 0, "drwxr-xr-x"),
        ("config.json", 2048, "-rw-r--r--"),
    ].iter().map(|(name, size, perms)| {
        SimulatedFileEntry {
            name: name.to_string(),
            path: format!("{}{}{}", base_path, sep, name),
            file_type: if *size == 0 && perms.starts_with('d') { "directory" } else { "file" }.into(),
            size: *size,
            modified: Utc::now() - Duration::days(rng.gen_range(1..60)),
            permissions: perms.to_string(),
            owner: "user".into(),
            group: "user".into(),
            hidden: false,
            executable: perms.starts_with('d'),
        }
    }).collect()
}

// ============================================================================
// Process Viewer Simulation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedProcess {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub user: String,
    pub cpu: f32,
    pub memory: f32,
    pub memory_bytes: u64,
    pub status: String, // "running", "sleeping", "stopped", "zombie"
    pub command: String,
    pub threads: u32,
    pub start_time: DateTime<Utc>,
    pub is_implant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessListResult {
    pub processes: Vec<SimulatedProcess>,
    pub total_cpu: f32,
    pub total_memory: f32,
    pub process_count: u32,
}

#[tauri::command]
pub async fn simulate_process_list(
    _session_id: String,
) -> Result<ProcessListResult, String> {
    let mut rng = rand::thread_rng();

    let process_templates = vec![
        (1, 0, "systemd", "root", 0.1, 0.5, "running", "/sbin/init", false),
        (2, 1, "kthreadd", "root", 0.0, 0.0, "sleeping", "[kthreadd]", false),
        (234, 1, "sshd", "root", 0.0, 0.3, "running", "/usr/sbin/sshd -D", false),
        (456, 234, "sshd", "user", 0.0, 0.2, "running", "sshd: user [priv]", false),
        (457, 456, "bash", "user", 0.0, 0.2, "sleeping", "-bash", false),
        (512, 1, "cron", "root", 0.0, 0.1, "running", "/usr/sbin/cron -f", false),
        (789, 1, "nginx", "root", 0.0, 0.4, "running", "nginx: master process", false),
        (790, 789, "nginx", "www-data", 0.5, 1.2, "running", "nginx: worker process", false),
        (791, 789, "nginx", "www-data", 0.3, 1.1, "running", "nginx: worker process", false),
        (1024, 457, "python3", "user", 2.3, 4.5, "running", "python3 app.py", false),
        (1337, 457, "update_svc", "user", 0.1, 0.8, "running", "./update_service", true), // Our implant
        (2048, 1, "mysqld", "mysql", 1.5, 8.2, "running", "/usr/sbin/mysqld", false),
        (2200, 1, "postgres", "postgres", 0.8, 3.4, "running", "/usr/lib/postgresql/14/bin/postgres", false),
        (3000, 457, "node", "user", 3.2, 5.6, "running", "node server.js", false),
        (3100, 3000, "node", "user", 1.1, 2.3, "running", "node worker.js", false),
        (4000, 1, "dockerd", "root", 0.4, 2.1, "running", "/usr/bin/dockerd", false),
        (4100, 4000, "containerd", "root", 0.2, 1.5, "running", "containerd", false),
        (5000, 1, "rsyslogd", "syslog", 0.0, 0.3, "running", "/usr/sbin/rsyslogd", false),
        (5500, 1, "NetworkManager", "root", 0.1, 0.6, "running", "/usr/sbin/NetworkManager", false),
        (6000, 1, "systemd-journal", "root", 0.2, 1.8, "running", "/lib/systemd/systemd-journald", false),
    ];

    let processes: Vec<SimulatedProcess> = process_templates.iter().map(|(pid, ppid, name, user, cpu, mem, status, cmd, is_implant)| {
        SimulatedProcess {
            pid: *pid,
            ppid: *ppid,
            name: name.to_string(),
            user: user.to_string(),
            cpu: cpu + rng.gen_range(-0.1..0.5) as f32,
            memory: mem + rng.gen_range(-0.2..0.5) as f32,
            memory_bytes: (mem * 1024.0 * 1024.0 * 10.0) as u64 + rng.gen_range(0..1000000),
            status: status.to_string(),
            command: cmd.to_string(),
            threads: rng.gen_range(1..20),
            start_time: Utc::now() - Duration::hours(rng.gen_range(1..720)),
            is_implant: *is_implant,
        }
    }).collect();

    let total_cpu: f32 = processes.iter().map(|p| p.cpu).sum();
    let total_memory: f32 = processes.iter().map(|p| p.memory).sum();

    Ok(ProcessListResult {
        process_count: processes.len() as u32,
        processes,
        total_cpu,
        total_memory,
    })
}
