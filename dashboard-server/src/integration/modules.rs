//! Module Bridges - Connect Dashboard to Ferox Core Modules
//!
//! Provides dashboard-friendly wrappers around Ferox post-exploitation modules.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::FeroxBridge;

// ============================================================================
// Request/Response Types for Dashboard API
// ============================================================================

/// Request to run privilege escalation
#[derive(Debug, Clone, Deserialize)]
pub struct PrivEscRequest {
    pub session_id: Uuid,
    pub auto_escalate: bool,
    pub safe_mode: bool,
}

/// Privilege escalation vector found
#[derive(Debug, Clone, Serialize)]
pub struct PrivEscVector {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: String,
    pub severity: String,
    pub confidence: f64,
    pub mitre_id: String,
    pub exploitable: bool,
}

/// Result of privilege escalation enumeration
#[derive(Debug, Clone, Serialize)]
pub struct PrivEscResult {
    pub session_id: Uuid,
    pub current_privilege: String,
    pub vectors_found: Vec<PrivEscVector>,
    pub escalation_attempted: bool,
    pub escalation_success: bool,
    pub new_privilege: Option<String>,
    pub output: String,
}

/// Request to harvest credentials
#[derive(Debug, Clone, Deserialize)]
pub struct CredentialHarvestRequest {
    pub session_id: Uuid,
    pub sources: Vec<String>,  // "lsass", "sam", "browsers", "files", "all"
    pub safe_mode: bool,
}

/// Harvested credential
#[derive(Debug, Clone, Serialize)]
pub struct HarvestedCred {
    pub id: Uuid,
    pub cred_type: String,
    pub username: String,
    pub domain: Option<String>,
    pub secret: String,  // Will be redacted in API response
    pub source: String,
    pub sensitivity: String,
    pub is_reusable: bool,
}

/// Result of credential harvesting
#[derive(Debug, Clone, Serialize)]
pub struct CredentialHarvestResult {
    pub session_id: Uuid,
    pub credentials: Vec<HarvestedCred>,
    pub total_found: usize,
    pub by_type: std::collections::HashMap<String, usize>,
    pub output: String,
}

/// Request to install persistence
#[derive(Debug, Clone, Deserialize)]
pub struct PersistenceRequest {
    pub session_id: Uuid,
    pub method: String,  // "registry", "scheduled_task", "wmi", "service", "startup", "auto"
    pub name: String,
    pub safe_mode: bool,
}

/// Installed persistence mechanism
#[derive(Debug, Clone, Serialize)]
pub struct PersistenceHandle {
    pub id: Uuid,
    pub method: String,
    pub name: String,
    pub location: String,
    pub status: String,
    pub mitre_id: String,
}

/// Result of persistence installation
#[derive(Debug, Clone, Serialize)]
pub struct PersistenceResult {
    pub session_id: Uuid,
    pub success: bool,
    pub handles: Vec<PersistenceHandle>,
    pub output: String,
}

/// Request for lateral movement
#[derive(Debug, Clone, Deserialize)]
pub struct LateralMoveRequest {
    pub session_id: Uuid,
    pub target_host: String,
    pub method: String,  // "smb", "wmi", "psexec", "winrm", "ssh", "auto"
    pub credential_id: Option<Uuid>,
    pub safe_mode: bool,
}

/// Result of lateral movement
#[derive(Debug, Clone, Serialize)]
pub struct LateralMoveResult {
    pub session_id: Uuid,
    pub target_host: String,
    pub success: bool,
    pub new_session_id: Option<Uuid>,
    pub method_used: String,
    pub output: String,
}

/// Network discovery result
#[derive(Debug, Clone, Serialize)]
pub struct DiscoveredHost {
    pub ip: String,
    pub hostname: Option<String>,
    pub os: Option<String>,
    pub open_ports: Vec<u16>,
    pub services: Vec<String>,
}

/// Result of network discovery
#[derive(Debug, Clone, Serialize)]
pub struct DiscoveryResult {
    pub session_id: Uuid,
    pub hosts: Vec<DiscoveredHost>,
    pub subnets_scanned: Vec<String>,
    pub output: String,
}

// ============================================================================
// Module Bridge Implementation
// ============================================================================

/// Bridge to Ferox post-exploitation modules
pub struct ModuleBridge {
    ferox_bridge: Arc<FeroxBridge>,
}

impl ModuleBridge {
    pub fn new(ferox_bridge: Arc<FeroxBridge>) -> Self {
        Self { ferox_bridge }
    }

    /// Run privilege escalation enumeration and optionally auto-escalate
    pub async fn run_privesc(&self, request: PrivEscRequest) -> Result<PrivEscResult> {
        // Verify session exists
        let _session = self.ferox_bridge
            .get_session(request.session_id)
            .await
            .context("Session not found")?;

        // Simulate privilege escalation (in production, this calls the actual module)
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        let vectors = vec![
            PrivEscVector {
                id: Uuid::new_v4(),
                name: "Unquoted Service Path".to_string(),
                description: "VulnService has an unquoted path with spaces".to_string(),
                category: "Service Exploit".to_string(),
                severity: "High".to_string(),
                confidence: 0.85,
                mitre_id: "T1574.009".to_string(),
                exploitable: true,
            },
            PrivEscVector {
                id: Uuid::new_v4(),
                name: "AlwaysInstallElevated".to_string(),
                description: "MSI packages install with elevated privileges".to_string(),
                category: "Configuration Weakness".to_string(),
                severity: "Critical".to_string(),
                confidence: 0.95,
                mitre_id: "T1548.002".to_string(),
                exploitable: true,
            },
            PrivEscVector {
                id: Uuid::new_v4(),
                name: "Writable Service Binary".to_string(),
                description: "UpdateService binary is writable by current user".to_string(),
                category: "Service Exploit".to_string(),
                severity: "High".to_string(),
                confidence: 0.90,
                mitre_id: "T1574.010".to_string(),
                exploitable: true,
            },
        ];

        let (escalation_success, new_privilege) = if request.auto_escalate && !request.safe_mode {
            (true, Some("NT AUTHORITY\\SYSTEM".to_string()))
        } else {
            (false, None)
        };

        let output = if request.auto_escalate && !request.safe_mode {
            r#"[*] Enumerating privilege escalation vectors...
[+] Found: Unquoted service path (VulnService)
[+] Found: AlwaysInstallElevated enabled
[+] Found: Writable service binary (UpdateService)
[!] 3 potential vectors found

[*] Attempting automatic escalation...
[+] Trying AlwaysInstallElevated...
[+] SUCCESS: Exploited AlwaysInstallElevated
[+] New privileges: NT AUTHORITY\SYSTEM"#
        } else {
            r#"[*] Enumerating privilege escalation vectors...
[+] Found: Unquoted service path (VulnService)
[+] Found: AlwaysInstallElevated enabled
[+] Found: Writable service binary (UpdateService)
[!] 3 potential vectors found

[*] Auto-escalate disabled. Use vectors manually or enable auto_escalate."#
        };

        Ok(PrivEscResult {
            session_id: request.session_id,
            current_privilege: "User".to_string(),
            vectors_found: vectors,
            escalation_attempted: request.auto_escalate,
            escalation_success,
            new_privilege,
            output: output.to_string(),
        })
    }

    /// Harvest credentials from the target system
    pub async fn harvest_credentials(&self, request: CredentialHarvestRequest) -> Result<CredentialHarvestResult> {
        // Verify session exists
        let _session = self.ferox_bridge
            .get_session(request.session_id)
            .await
            .context("Session not found")?;

        // Simulate credential harvesting
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

        let credentials = vec![
            HarvestedCred {
                id: Uuid::new_v4(),
                cred_type: "NTLM Hash".to_string(),
                username: "Administrator".to_string(),
                domain: Some("CORP".to_string()),
                secret: "aad3b435b51404eeaad3b435b51404ee:31d6cfe0d16ae931b73c59d7e0c089c0".to_string(),
                source: "LSASS Memory".to_string(),
                sensitivity: "Critical".to_string(),
                is_reusable: true,
            },
            HarvestedCred {
                id: Uuid::new_v4(),
                cred_type: "Plain Text".to_string(),
                username: "john.doe".to_string(),
                domain: Some("CORP".to_string()),
                secret: "Summer2024!".to_string(),
                source: "Chrome Browser".to_string(),
                sensitivity: "High".to_string(),
                is_reusable: true,
            },
            HarvestedCred {
                id: Uuid::new_v4(),
                cred_type: "Plain Text".to_string(),
                username: "svc_sql".to_string(),
                domain: Some("CORP".to_string()),
                secret: "SqlP@ss123".to_string(),
                source: "Windows Credential Manager".to_string(),
                sensitivity: "High".to_string(),
                is_reusable: true,
            },
            HarvestedCred {
                id: Uuid::new_v4(),
                cred_type: "Kerberos TGT".to_string(),
                username: "jane.smith".to_string(),
                domain: Some("CORP".to_string()),
                secret: "base64_ticket_data...".to_string(),
                source: "LSASS Memory".to_string(),
                sensitivity: "High".to_string(),
                is_reusable: true,
            },
            HarvestedCred {
                id: Uuid::new_v4(),
                cred_type: "SSH Key".to_string(),
                username: "root".to_string(),
                domain: None,
                secret: "-----BEGIN RSA PRIVATE KEY-----\n...".to_string(),
                source: "~/.ssh/id_rsa".to_string(),
                sensitivity: "Critical".to_string(),
                is_reusable: true,
            },
        ];

        let mut by_type: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for cred in &credentials {
            *by_type.entry(cred.cred_type.clone()).or_insert(0) += 1;
        }

        let output = r#"[*] Harvesting credentials...
[+] Dumping LSASS memory...
[+] Found: Administrator (NTLM Hash)
[+] Found: jane.smith (Kerberos TGT)
[+] Extracting browser credentials...
[+] Found: john.doe (Chrome - Plain Text)
[+] Checking Windows Credential Manager...
[+] Found: svc_sql (Plain Text)
[+] Scanning for SSH keys...
[+] Found: root SSH key

[+] Total credentials found: 5
  - NTLM Hash: 1
  - Plain Text: 2
  - Kerberos TGT: 1
  - SSH Key: 1"#;

        Ok(CredentialHarvestResult {
            session_id: request.session_id,
            total_found: credentials.len(),
            credentials,
            by_type,
            output: output.to_string(),
        })
    }

    /// Install persistence mechanism
    pub async fn install_persistence(&self, request: PersistenceRequest) -> Result<PersistenceResult> {
        // Verify session exists
        let _session = self.ferox_bridge
            .get_session(request.session_id)
            .await
            .context("Session not found")?;

        // Simulate persistence installation
        tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;

        let handles = if request.safe_mode {
            vec![]
        } else {
            vec![
                PersistenceHandle {
                    id: Uuid::new_v4(),
                    method: "Registry Run Key".to_string(),
                    name: request.name.clone(),
                    location: r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run".to_string(),
                    status: "Installed".to_string(),
                    mitre_id: "T1547.001".to_string(),
                },
                PersistenceHandle {
                    id: Uuid::new_v4(),
                    method: "Scheduled Task".to_string(),
                    name: format!("{}_task", request.name),
                    location: r"\Microsoft\Windows\Maintenance".to_string(),
                    status: "Installed".to_string(),
                    mitre_id: "T1053.005".to_string(),
                },
                PersistenceHandle {
                    id: Uuid::new_v4(),
                    method: "WMI Subscription".to_string(),
                    name: format!("{}_wmi", request.name),
                    location: "root\\subscription".to_string(),
                    status: "Installed".to_string(),
                    mitre_id: "T1546.003".to_string(),
                },
            ]
        };

        let output = if request.safe_mode {
            "[*] Safe mode enabled - no persistence installed\n[*] Would install: Registry Run Key, Scheduled Task, WMI Subscription"
        } else {
            r#"[*] Installing persistence mechanisms...
[+] Registry Run Key: Installed
[+] Scheduled Task: Installed (hidden)
[+] WMI Subscription: Installed

[+] 3 persistence methods installed
[*] Persistence will survive reboot"#
        };

        Ok(PersistenceResult {
            session_id: request.session_id,
            success: !request.safe_mode,
            handles,
            output: output.to_string(),
        })
    }

    /// Perform lateral movement to another host
    pub async fn lateral_move(&self, request: LateralMoveRequest) -> Result<LateralMoveResult> {
        // Verify session exists
        let _session = self.ferox_bridge
            .get_session(request.session_id)
            .await
            .context("Session not found")?;

        // Simulate lateral movement
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let (success, new_session_id) = if request.safe_mode {
            (false, None)
        } else {
            (true, Some(Uuid::new_v4()))
        };

        let method_used = if request.method == "auto" {
            "SMB/Admin Shares".to_string()
        } else {
            request.method.to_uppercase()
        };

        let output = if request.safe_mode {
            format!(
                "[*] Safe mode enabled - no lateral movement performed\n\
                 [*] Would attempt: {} to {}\n\
                 [*] Credential: {:?}",
                method_used, request.target_host, request.credential_id
            )
        } else {
            format!(
                r#"[*] Initiating lateral movement to {}...
[*] Method: {}
[+] Authenticating...
[+] SUCCESS: Authentication successful
[+] Deploying agent...
[+] Agent deployed and connected

[+] New session established on {} ({})
[+] Session ID: {}"#,
                request.target_host,
                method_used,
                request.target_host,
                request.target_host,
                new_session_id.map(|id| id.to_string()).unwrap_or_default()
            )
        };

        Ok(LateralMoveResult {
            session_id: request.session_id,
            target_host: request.target_host,
            success,
            new_session_id,
            method_used,
            output,
        })
    }

    /// Discover hosts on the network
    pub async fn discover_network(&self, session_id: Uuid) -> Result<DiscoveryResult> {
        // Verify session exists
        let _session = self.ferox_bridge
            .get_session(session_id)
            .await
            .context("Session not found")?;

        // Simulate network discovery
        tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

        let hosts = vec![
            DiscoveredHost {
                ip: "192.168.1.1".to_string(),
                hostname: Some("gateway".to_string()),
                os: Some("Network Device".to_string()),
                open_ports: vec![22, 80, 443],
                services: vec!["SSH".to_string(), "HTTP".to_string(), "HTTPS".to_string()],
            },
            DiscoveredHost {
                ip: "192.168.1.10".to_string(),
                hostname: Some("DC01".to_string()),
                os: Some("Windows Server 2019".to_string()),
                open_ports: vec![53, 88, 135, 139, 389, 445, 636, 3268, 3389],
                services: vec!["DNS".to_string(), "Kerberos".to_string(), "LDAP".to_string(), "SMB".to_string(), "RDP".to_string()],
            },
            DiscoveredHost {
                ip: "192.168.1.50".to_string(),
                hostname: Some("WS-DEV01".to_string()),
                os: Some("Windows 11".to_string()),
                open_ports: vec![135, 139, 445, 3389],
                services: vec!["RPC".to_string(), "SMB".to_string(), "RDP".to_string()],
            },
            DiscoveredHost {
                ip: "192.168.1.100".to_string(),
                hostname: Some("SRV-WEB01".to_string()),
                os: Some("Windows Server 2022".to_string()),
                open_ports: vec![80, 443, 3389],
                services: vec!["HTTP".to_string(), "HTTPS".to_string(), "RDP".to_string()],
            },
            DiscoveredHost {
                ip: "192.168.2.100".to_string(),
                hostname: Some("srv-db01".to_string()),
                os: Some("Ubuntu 22.04".to_string()),
                open_ports: vec![22, 3306, 5432],
                services: vec!["SSH".to_string(), "MySQL".to_string(), "PostgreSQL".to_string()],
            },
        ];

        let output = r#"[*] Discovering network targets...
[*] Scanning 192.168.1.0/24...
[+] Found: 192.168.1.1 (gateway) - Network Device
[+] Found: 192.168.1.10 (DC01) - Windows Server 2019 [DOMAIN CONTROLLER]
[+] Found: 192.168.1.50 (WS-DEV01) - Windows 11
[+] Found: 192.168.1.100 (SRV-WEB01) - Windows Server 2022

[*] Scanning 192.168.2.0/24...
[+] Found: 192.168.2.100 (srv-db01) - Ubuntu 22.04

[+] Total hosts discovered: 5
[+] High-value targets: DC01, srv-db01"#;

        Ok(DiscoveryResult {
            session_id,
            hosts,
            subnets_scanned: vec!["192.168.1.0/24".to_string(), "192.168.2.0/24".to_string()],
            output: output.to_string(),
        })
    }
}
