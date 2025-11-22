//! Shared application state for the dashboard server
//!
//! Provides thread-safe access to sessions, WebSocket clients,
//! and other shared data structures.

use crate::types::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// WebSocket client information
#[derive(Debug, Clone)]
pub struct WsClient {
    pub id: Uuid,
    pub subscribed_sessions: Vec<Uuid>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

impl WsClient {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            subscribed_sessions: Vec::new(),
            connected_at: chrono::Utc::now(),
        }
    }
}

/// Shared application state
pub struct AppState {
    /// All active sessions
    pub sessions: RwLock<HashMap<Uuid, DashboardSession>>,
    /// Connected WebSocket clients
    pub ws_clients: RwLock<HashMap<Uuid, WsClient>>,
    /// Command history per session
    pub commands: RwLock<HashMap<Uuid, Vec<Command>>>,
    /// All harvested credentials
    pub credentials: RwLock<Vec<DashboardCredential>>,
    /// Network topology
    pub network_hosts: RwLock<HashMap<Uuid, NetworkHost>>,
    pub network_edges: RwLock<Vec<NetworkEdge>>,
    /// MITRE technique usage
    pub mitre_techniques: RwLock<HashMap<String, MitreTechniqueUsage>>,
    /// Broadcast channel for server events
    pub event_tx: broadcast::Sender<ServerEvent>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        let (event_tx, _) = broadcast::channel(1024);

        Arc::new(Self {
            sessions: RwLock::new(HashMap::new()),
            ws_clients: RwLock::new(HashMap::new()),
            commands: RwLock::new(HashMap::new()),
            credentials: RwLock::new(Vec::new()),
            network_hosts: RwLock::new(HashMap::new()),
            network_edges: RwLock::new(Vec::new()),
            mitre_techniques: RwLock::new(HashMap::new()),
            event_tx,
        })
    }

    /// Add a new session and broadcast event
    pub async fn add_session(&self, session: DashboardSession) {
        let session_clone = session.clone();
        self.sessions.write().await.insert(session.id, session);
        self.commands.write().await.insert(session_clone.id, Vec::new());
        let _ = self.event_tx.send(ServerEvent::SessionCreated(session_clone));
    }

    /// Update a session and broadcast event
    pub async fn update_session(&self, session: DashboardSession) {
        let session_clone = session.clone();
        self.sessions.write().await.insert(session.id, session);
        let _ = self.event_tx.send(ServerEvent::SessionUpdated(session_clone));
    }

    /// Remove a session and broadcast event
    pub async fn remove_session(&self, session_id: Uuid) {
        self.sessions.write().await.remove(&session_id);
        let _ = self.event_tx.send(ServerEvent::SessionClosed { session_id });
    }

    /// Get all sessions
    pub async fn get_sessions(&self) -> Vec<DashboardSession> {
        self.sessions.read().await.values().cloned().collect()
    }

    /// Get a single session
    pub async fn get_session(&self, id: Uuid) -> Option<DashboardSession> {
        self.sessions.read().await.get(&id).cloned()
    }

    /// Session heartbeat
    pub async fn session_heartbeat(&self, session_id: Uuid) {
        if let Some(session) = self.sessions.write().await.get_mut(&session_id) {
            session.heartbeat();
        }
    }

    /// Add a command to history
    pub async fn add_command(&self, command: Command) {
        let session_id = command.session_id;
        self.commands
            .write()
            .await
            .entry(session_id)
            .or_insert_with(Vec::new)
            .push(command);
    }

    /// Get commands for a session
    pub async fn get_commands(&self, session_id: Uuid) -> Vec<Command> {
        self.commands
            .read()
            .await
            .get(&session_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Add credentials and broadcast event
    pub async fn add_credentials(&self, session_id: Uuid, creds: Vec<DashboardCredential>) {
        let creds_clone = creds.clone();
        self.credentials.write().await.extend(creds);

        // Update session metrics
        if let Some(session) = self.sessions.write().await.get_mut(&session_id) {
            session.metrics.credentials_count += creds_clone.len() as u32;
        }

        let _ = self.event_tx.send(ServerEvent::CredentialsFound {
            session_id,
            credentials: creds_clone,
        });
    }

    /// Get all credentials (redacted)
    pub async fn get_credentials(&self) -> Vec<DashboardCredential> {
        self.credentials
            .read()
            .await
            .iter()
            .map(|c| c.redacted())
            .collect()
    }

    /// Get dashboard statistics
    pub async fn get_stats(&self) -> DashboardStats {
        let sessions = self.sessions.read().await;
        let active_count = sessions
            .values()
            .filter(|s| s.status == SessionStatus::Active)
            .count() as u32;

        let creds_count = self.credentials.read().await.len() as u32;
        let targets_count = self.network_hosts.read().await.len() as u32;

        let mitre = self.mitre_techniques.read().await;
        let mitre_coverage = if mitre.is_empty() {
            0.0
        } else {
            (mitre.len() as f32 / 200.0) * 100.0  // Approximate total techniques
        };

        DashboardStats {
            active_sessions: active_count,
            total_sessions: sessions.len() as u32,
            credentials_collected: creds_count,
            targets_discovered: targets_count,
            mitre_coverage,
        }
    }

    /// Add WebSocket client
    pub async fn add_ws_client(&self, client: WsClient) -> Uuid {
        let id = client.id;
        self.ws_clients.write().await.insert(id, client);
        id
    }

    /// Remove WebSocket client
    pub async fn remove_ws_client(&self, client_id: Uuid) {
        self.ws_clients.write().await.remove(&client_id);
    }

    /// Subscribe a client to session events
    pub async fn subscribe_to_session(&self, client_id: Uuid, session_id: Uuid) {
        if let Some(client) = self.ws_clients.write().await.get_mut(&client_id) {
            if !client.subscribed_sessions.contains(&session_id) {
                client.subscribed_sessions.push(session_id);
            }
        }
    }

    /// Broadcast command output
    pub async fn broadcast_command_output(
        &self,
        command_id: Uuid,
        session_id: Uuid,
        output: String,
        is_complete: bool,
        success: Option<bool>,
    ) {
        let _ = self.event_tx.send(ServerEvent::CommandOutput {
            command_id,
            session_id,
            output,
            is_complete,
            success,
        });
    }

    /// Get MITRE coverage
    pub async fn get_mitre_coverage(&self) -> MitreCoverage {
        let techniques = self.mitre_techniques.read().await;
        let techniques_list: Vec<_> = techniques.values().cloned().collect();

        let tactics: Vec<String> = techniques_list
            .iter()
            .map(|t| t.tactic.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        MitreCoverage {
            total_techniques: 200,  // Approximate
            techniques_used: techniques_list.len() as u32,
            coverage_percentage: (techniques_list.len() as f32 / 200.0) * 100.0,
            tactics_covered: tactics,
            techniques: techniques_list,
        }
    }

    /// Initialize with demo data for testing
    pub async fn init_demo_data(&self) {
        // Add demo sessions
        let sessions = vec![
            DashboardSession {
                id: Uuid::new_v4(),
                hostname: "DC01".to_string(),
                ip_address: "192.168.1.10".to_string(),
                os: OsType::Windows,
                os_version: Some("Windows Server 2019".to_string()),
                architecture: Architecture::X64,
                username: "Administrator".to_string(),
                privileges: PrivilegeLevel::Administrator,
                status: SessionStatus::Active,
                established_at: chrono::Utc::now() - chrono::Duration::hours(2),
                last_seen: chrono::Utc::now(),
                intelligence: SessionIntelligence {
                    domain: Some("CORP.LOCAL".to_string()),
                    is_domain_joined: true,
                    detected_av: vec!["Windows Defender".to_string()],
                    stealth_mode: "quiet".to_string(),
                    network_segment: Some("192.168.1.0/24".to_string()),
                },
                metrics: SessionMetrics {
                    credentials_count: 12,
                    commands_executed: 45,
                    files_transferred: 3,
                    persistence_methods: 2,
                },
                tags: vec!["domain_controller".to_string(), "high_value".to_string()],
            },
            DashboardSession {
                id: Uuid::new_v4(),
                hostname: "WS-DEV01".to_string(),
                ip_address: "192.168.1.50".to_string(),
                os: OsType::Windows,
                os_version: Some("Windows 11".to_string()),
                architecture: Architecture::X64,
                username: "john.doe".to_string(),
                privileges: PrivilegeLevel::User,
                status: SessionStatus::Active,
                established_at: chrono::Utc::now() - chrono::Duration::minutes(45),
                last_seen: chrono::Utc::now() - chrono::Duration::seconds(15),
                intelligence: SessionIntelligence {
                    domain: Some("CORP.LOCAL".to_string()),
                    is_domain_joined: true,
                    detected_av: vec!["CrowdStrike".to_string()],
                    stealth_mode: "ghost".to_string(),
                    network_segment: Some("192.168.1.0/24".to_string()),
                },
                metrics: SessionMetrics {
                    credentials_count: 3,
                    commands_executed: 12,
                    files_transferred: 1,
                    persistence_methods: 0,
                },
                tags: vec!["developer".to_string()],
            },
            DashboardSession {
                id: Uuid::new_v4(),
                hostname: "srv-db01".to_string(),
                ip_address: "192.168.2.100".to_string(),
                os: OsType::Linux,
                os_version: Some("Ubuntu 22.04".to_string()),
                architecture: Architecture::X64,
                username: "root".to_string(),
                privileges: PrivilegeLevel::Root,
                status: SessionStatus::Sleeping,
                established_at: chrono::Utc::now() - chrono::Duration::hours(1),
                last_seen: chrono::Utc::now() - chrono::Duration::minutes(2),
                intelligence: SessionIntelligence {
                    domain: None,
                    is_domain_joined: false,
                    detected_av: Vec::new(),
                    stealth_mode: "silent".to_string(),
                    network_segment: Some("192.168.2.0/24".to_string()),
                },
                metrics: SessionMetrics {
                    credentials_count: 8,
                    commands_executed: 23,
                    files_transferred: 5,
                    persistence_methods: 1,
                },
                tags: vec!["database".to_string(), "linux".to_string()],
            },
        ];

        for session in sessions {
            self.add_session(session).await;
        }

        // Add demo credentials
        let creds = vec![
            DashboardCredential {
                id: Uuid::new_v4(),
                cred_type: CredentialType::NtlmHash,
                username: "Administrator".to_string(),
                domain: Some("CORP".to_string()),
                secret: "aad3b435b51404eeaad3b435b51404ee:31d6cfe0d16ae931b73c59d7e0c089c0".to_string(),
                source_hostname: "DC01".to_string(),
                source_session_id: Uuid::new_v4(),
                sensitivity: Sensitivity::Critical,
                collected_at: chrono::Utc::now() - chrono::Duration::hours(1),
                is_reusable: true,
                notes: Some("Domain Admin hash".to_string()),
            },
            DashboardCredential {
                id: Uuid::new_v4(),
                cred_type: CredentialType::PlainText,
                username: "john.doe".to_string(),
                domain: Some("CORP".to_string()),
                secret: "Summer2024!".to_string(),
                source_hostname: "WS-DEV01".to_string(),
                source_session_id: Uuid::new_v4(),
                sensitivity: Sensitivity::High,
                collected_at: chrono::Utc::now() - chrono::Duration::minutes(30),
                is_reusable: true,
                notes: None,
            },
            DashboardCredential {
                id: Uuid::new_v4(),
                cred_type: CredentialType::SshKey,
                username: "root".to_string(),
                domain: None,
                secret: "-----BEGIN RSA PRIVATE KEY-----\n...".to_string(),
                source_hostname: "srv-db01".to_string(),
                source_session_id: Uuid::new_v4(),
                sensitivity: Sensitivity::Critical,
                collected_at: chrono::Utc::now() - chrono::Duration::minutes(45),
                is_reusable: true,
                notes: Some("Database server root key".to_string()),
            },
        ];

        self.credentials.write().await.extend(creds);

        // Add demo MITRE techniques
        let techniques = vec![
            MitreTechniqueUsage {
                technique_id: "T1059.001".to_string(),
                technique_name: "PowerShell".to_string(),
                tactic: "Execution".to_string(),
                times_used: 15,
                success_rate: 0.93,
                detection_risk: "medium".to_string(),
                last_used: Some(chrono::Utc::now() - chrono::Duration::minutes(10)),
                sessions_used: vec![],
            },
            MitreTechniqueUsage {
                technique_id: "T1003.001".to_string(),
                technique_name: "LSASS Memory".to_string(),
                tactic: "Credential Access".to_string(),
                times_used: 3,
                success_rate: 1.0,
                detection_risk: "high".to_string(),
                last_used: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
                sessions_used: vec![],
            },
            MitreTechniqueUsage {
                technique_id: "T1021.002".to_string(),
                technique_name: "SMB/Windows Admin Shares".to_string(),
                tactic: "Lateral Movement".to_string(),
                times_used: 5,
                success_rate: 0.8,
                detection_risk: "medium".to_string(),
                last_used: Some(chrono::Utc::now() - chrono::Duration::minutes(30)),
                sessions_used: vec![],
            },
        ];

        let mut mitre = self.mitre_techniques.write().await;
        for tech in techniques {
            mitre.insert(tech.technique_id.clone(), tech);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let (event_tx, _) = broadcast::channel(1024);
        Self {
            sessions: RwLock::new(HashMap::new()),
            ws_clients: RwLock::new(HashMap::new()),
            commands: RwLock::new(HashMap::new()),
            credentials: RwLock::new(Vec::new()),
            network_hosts: RwLock::new(HashMap::new()),
            network_edges: RwLock::new(Vec::new()),
            mitre_techniques: RwLock::new(HashMap::new()),
            event_tx,
        }
    }
}
