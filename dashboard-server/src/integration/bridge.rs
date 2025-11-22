//! FeroxBridge - Connection between Dashboard and Ferox Core
//!
//! Provides unified access to Ferox session management and command execution.

use crate::types::{Architecture, DashboardSession, OsType, PrivilegeLevel, SessionMetrics, SessionIntelligence, SessionStatus};
use anyhow::{Context, Result};
use ferox::core::module::{Platform, Session};
use ferox::core::session::SessionManager;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Events broadcast from the bridge
#[derive(Debug, Clone)]
pub enum BridgeEvent {
    SessionCreated(Uuid),
    SessionUpdated(Uuid),
    SessionClosed(Uuid),
    CommandOutput {
        session_id: Uuid,
        command_id: Uuid,
        output: String,
        is_complete: bool,
    },
    CredentialsFound {
        session_id: Uuid,
        count: usize,
    },
    Error {
        message: String,
    },
}

/// Bridge between Dashboard Server and Ferox Core Engine
pub struct FeroxBridge {
    /// Ferox session manager
    session_manager: Arc<SessionManager>,
    /// Mapping from dashboard session IDs to ferox session IDs
    session_mapping: Arc<RwLock<HashMap<Uuid, Uuid>>>,
    /// Event broadcaster
    event_tx: broadcast::Sender<BridgeEvent>,
}

impl FeroxBridge {
    /// Create a new FeroxBridge
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(256);

        Self {
            session_manager: Arc::new(SessionManager::new()),
            session_mapping: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
        }
    }

    /// Create FeroxBridge with database persistence
    pub fn with_persistence(db_path: &str) -> Result<Self> {
        let (event_tx, _) = broadcast::channel(256);

        let session_manager = SessionManager::with_db(db_path)
            .context("Failed to create session manager with database")?;

        Ok(Self {
            session_manager: Arc::new(session_manager),
            session_mapping: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
        })
    }

    /// Subscribe to bridge events
    pub fn subscribe(&self) -> broadcast::Receiver<BridgeEvent> {
        self.event_tx.subscribe()
    }

    /// Get the session manager
    pub fn session_manager(&self) -> Arc<SessionManager> {
        self.session_manager.clone()
    }

    /// Register a new session from Ferox core
    pub async fn register_session(&self, session: Session) -> Uuid {
        let id = self.session_manager.add(session).await;

        // Notify subscribers
        let _ = self.event_tx.send(BridgeEvent::SessionCreated(id));

        id
    }

    /// Get session by ID
    pub async fn get_session(&self, id: Uuid) -> Option<Session> {
        self.session_manager.get(id).await
    }

    /// List all active sessions
    pub async fn list_sessions(&self) -> Vec<Session> {
        self.session_manager.list_all().await
    }

    /// Convert Ferox Session to Dashboard Session
    pub fn to_dashboard_session(session: &Session) -> DashboardSession {
        let os = match session.platform {
            Platform::Windows => OsType::Windows,
            Platform::Linux => OsType::Linux,
            Platform::MacOS => OsType::MacOS,
            Platform::Any => OsType::Windows,
        };

        // Extract metadata
        let hostname = session
            .metadata
            .get("hostname")
            .and_then(|v| v.as_str())
            .unwrap_or(&session.target)
            .to_string();

        let os_version = session
            .metadata
            .get("os_version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let architecture = session
            .metadata
            .get("arch")
            .and_then(|v| v.as_str())
            .map(|a| match a.to_lowercase().as_str() {
                "x86" | "i386" | "i686" => Architecture::X86,
                "arm64" | "aarch64" => Architecture::Arm64,
                _ => Architecture::X64,
            })
            .unwrap_or(Architecture::X64);

        let privileges = session
            .metadata
            .get("privileges")
            .and_then(|v| v.as_str())
            .map(|p| match p.to_lowercase().as_str() {
                "system" => PrivilegeLevel::System,
                "root" => PrivilegeLevel::Root,
                "administrator" | "admin" => PrivilegeLevel::Administrator,
                _ => PrivilegeLevel::User,
            })
            .unwrap_or(PrivilegeLevel::User);

        let domain = session
            .metadata
            .get("domain")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let is_domain_joined = domain.is_some();

        let detected_av: Vec<String> = session
            .metadata
            .get("detected_av")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let stealth_mode = session
            .metadata
            .get("stealth_mode")
            .and_then(|v| v.as_str())
            .unwrap_or("normal")
            .to_string();

        let network_segment = session
            .metadata
            .get("network_segment")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let tags: Vec<String> = session
            .metadata
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // Calculate status based on last_seen
        let now = chrono::Utc::now();
        let elapsed = now.signed_duration_since(session.last_seen);
        let status = if !session.active {
            SessionStatus::Dead
        } else if elapsed.num_seconds() > 300 {
            SessionStatus::Sleeping
        } else {
            SessionStatus::Active
        };

        DashboardSession {
            id: session.id,
            hostname,
            ip_address: session.target.clone(),
            os,
            os_version,
            architecture,
            username: session.user.clone().unwrap_or_else(|| "unknown".to_string()),
            privileges,
            status,
            established_at: session.established_at,
            last_seen: session.last_seen,
            intelligence: SessionIntelligence {
                domain,
                is_domain_joined,
                detected_av,
                stealth_mode,
                network_segment,
            },
            metrics: SessionMetrics {
                credentials_count: session
                    .metadata
                    .get("credentials_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                commands_executed: session
                    .metadata
                    .get("commands_executed")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                files_transferred: session
                    .metadata
                    .get("files_transferred")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                persistence_methods: session
                    .metadata
                    .get("persistence_methods")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
            },
            tags,
        }
    }

    /// Execute a command on a session
    /// Returns (command_id, output) on success
    pub async fn execute_command(
        &self,
        session_id: Uuid,
        command: String,
    ) -> Result<(Uuid, String)> {
        let _session = self
            .session_manager
            .get(session_id)
            .await
            .context("Session not found")?;

        let command_id = Uuid::new_v4();

        // In a real implementation, this would send the command to the agent
        // For now, simulate command execution
        let output = self.simulate_command(&command).await;

        // Broadcast output via bridge events (for internal subscribers)
        let _ = self.event_tx.send(BridgeEvent::CommandOutput {
            session_id,
            command_id,
            output: output.clone(),
            is_complete: true,
        });

        // Return output so caller can broadcast to WebSocket clients
        Ok((command_id, output))
    }

    /// Simulate command execution (demo mode)
    async fn simulate_command(&self, command: &str) -> String {
        // Add realistic delay
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        match command.to_lowercase().as_str() {
            "whoami" => "NT AUTHORITY\\SYSTEM".to_string(),
            "hostname" => "DC01".to_string(),
            "ipconfig" | "ifconfig" => {
                r#"Windows IP Configuration

Ethernet adapter Ethernet0:
   Connection-specific DNS Suffix  . : corp.local
   IPv4 Address. . . . . . . . . . . : 192.168.1.10
   Subnet Mask . . . . . . . . . . . : 255.255.255.0
   Default Gateway . . . . . . . . . : 192.168.1.1"#
                    .to_string()
            }
            "net user" => {
                r#"User accounts for \\DC01

-------------------------------------------------------------------------------
Administrator            Guest                    krbtgt
john.doe                 jane.smith               svc_backup
svc_sql                  DefaultAccount
The command completed successfully."#
                    .to_string()
            }
            cmd if cmd.starts_with("ferox privesc") => {
                r#"[*] Enumerating privilege escalation vectors...
[+] Found: Unquoted service path (VulnService)
[+] Found: Writable service binary (UpdateService)
[+] Found: AlwaysInstallElevated enabled
[!] 3 potential vectors found

[*] Attempting automatic escalation...
[+] SUCCESS: Exploited AlwaysInstallElevated
[+] New privileges: NT AUTHORITY\SYSTEM"#
                    .to_string()
            }
            cmd if cmd.starts_with("ferox creds") => {
                r#"[*] Harvesting credentials...
[+] Dumping LSASS memory...
[+] Extracting browser credentials...
[+] Checking credential files...

[+] Found 5 credentials:
  - Administrator:CORP (NTLM Hash)
  - john.doe:Summer2024! (Plaintext)
  - svc_sql:SqlP@ss123 (Plaintext)
  - jane.smith:CORP (Kerberos TGT)
  - root@srv-db01 (SSH Key)"#
                    .to_string()
            }
            cmd if cmd.starts_with("ferox persist") => {
                r#"[*] Installing persistence...
[+] Registry Run Key: Installed
[+] Scheduled Task: Installed (hidden)
[+] WMI Subscription: Installed

[+] 3 persistence methods installed
[*] Persistence will survive reboot"#
                    .to_string()
            }
            cmd if cmd.starts_with("ferox lateral") => {
                r#"[*] Discovering targets...
[+] Found 5 hosts in 192.168.1.0/24
[+] Found 3 hosts in 192.168.2.0/24

[*] Attempting lateral movement to WS-DEV01...
[+] SUCCESS: SMB/Admin Shares
[+] New session established on WS-DEV01 (192.168.1.50)"#
                    .to_string()
            }
            _ => format!("Executed: {}\n[Command completed successfully]", command),
        }
    }

    /// Heartbeat for a session
    pub async fn heartbeat(&self, session_id: Uuid) -> Result<()> {
        self.session_manager.heartbeat(session_id).await
    }

    /// Close a session
    pub async fn close_session(&self, session_id: Uuid) -> Result<()> {
        // In a real implementation, this would terminate the agent connection
        let _ = self.event_tx.send(BridgeEvent::SessionClosed(session_id));
        Ok(())
    }
}

impl Default for FeroxBridge {
    fn default() -> Self {
        Self::new()
    }
}
