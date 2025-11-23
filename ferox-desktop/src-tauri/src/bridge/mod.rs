//! Ferox Core Integration Bridge
//!
//! Bridges the Tauri desktop application with the Ferox core engine,
//! providing real-time session synchronization and event forwarding.

use crate::session::{
    Architecture, OsType, PrivilegeLevel, Session as UiSession, SessionIntelligence,
    SessionMetrics, SessionStatus, SessionTreeNode,
};
use anyhow::Result;
use ferox::core::module::{Platform, Session as CoreSession};
use ferox::core::session::SessionManager as CoreSessionManager;
use ferox::modules::post::credential_harvester::CredentialHarvestEngine;
use ferox::modules::post::persistence::PersistenceEngine;
use ferox::modules::post::privilege_escalation::PrivEscEngine;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;
use uuid::Uuid;

// ============================================================================
// Bridge Events
// ============================================================================

/// Events emitted by the bridge for real-time UI updates
#[derive(Debug, Clone)]
pub enum BridgeEvent {
    SessionCreated(String),
    SessionUpdated(String),
    SessionDied(String),
    SessionsRefreshed,
    CommandOutput { session_id: String, output: String },
    CredentialsFound { session_id: String, count: u32 },
    PrivilegeEscalated { session_id: String, new_privilege: String },
    Error { message: String },
}

// ============================================================================
// FeroxBridge
// ============================================================================

/// Bridge between Ferox Core and Tauri Desktop
pub struct FeroxBridge {
    /// The real Ferox core session manager
    core_manager: Arc<CoreSessionManager>,
    /// Cached UI sessions for fast access
    ui_sessions: Arc<RwLock<HashMap<String, UiSession>>>,
    /// Event broadcaster for real-time updates
    event_tx: broadcast::Sender<BridgeEvent>,
    /// Whether the bridge is actively syncing
    syncing: Arc<RwLock<bool>>,
    /// Privilege escalation engine
    privesc_engine: Arc<RwLock<PrivEscEngine>>,
    /// Credential harvesting engine
    cred_engine: Arc<RwLock<CredentialHarvestEngine>>,
    /// Persistence engine
    persistence_engine: Arc<RwLock<PersistenceEngine>>,
}

impl FeroxBridge {
    /// Create a new bridge with in-memory storage
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(256);
        Self {
            core_manager: Arc::new(CoreSessionManager::new()),
            ui_sessions: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            syncing: Arc::new(RwLock::new(false)),
            privesc_engine: Arc::new(RwLock::new(PrivEscEngine::new())),
            cred_engine: Arc::new(RwLock::new(CredentialHarvestEngine::new())),
            persistence_engine: Arc::new(RwLock::new(PersistenceEngine::new())),
        }
    }

    /// Create a new bridge with database persistence
    pub fn with_persistence(db_path: &str) -> Result<Self> {
        let (event_tx, _) = broadcast::channel(256);
        let core_manager = CoreSessionManager::with_db(db_path)?;

        Ok(Self {
            core_manager: Arc::new(core_manager),
            ui_sessions: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            syncing: Arc::new(RwLock::new(false)),
            privesc_engine: Arc::new(RwLock::new(PrivEscEngine::new())),
            cred_engine: Arc::new(RwLock::new(CredentialHarvestEngine::new())),
            persistence_engine: Arc::new(RwLock::new(PersistenceEngine::new())),
        })
    }

    /// Get default database path
    pub fn default_db_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ferox")
            .join("sessions.db")
    }

    /// Get the core session manager (for direct access if needed)
    pub fn core_manager(&self) -> Arc<CoreSessionManager> {
        Arc::clone(&self.core_manager)
    }

    /// Subscribe to bridge events
    pub fn subscribe(&self) -> broadcast::Receiver<BridgeEvent> {
        self.event_tx.subscribe()
    }

    /// Initialize and start background sync
    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing Ferox Core bridge...");

        // Load sessions from database
        if let Err(e) = self.core_manager.load_from_db().await {
            tracing::warn!("Failed to load sessions from database: {}", e);
        }

        // Initial sync (discard change events on initial load)
        let _ = self.sync_sessions().await;

        tracing::info!(
            "Ferox Core bridge initialized with {} sessions",
            self.session_count()
        );
        Ok(())
    }

    /// Start background sync task
    pub async fn start_sync(self: Arc<Self>, app_handle: AppHandle, interval_secs: u64) {
        // Mark as syncing
        *self.syncing.write() = true;

        let bridge = Arc::clone(&self);
        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));

            loop {
                interval.tick().await;

                if !*bridge.syncing.read() {
                    break;
                }

                // Sync sessions from core and get changes
                let (new_sessions, died_sessions) = bridge.sync_sessions().await;

                // Emit Tauri events for session changes
                if !new_sessions.is_empty() || !died_sessions.is_empty() {
                    bridge.emit_session_events(&app_handle, &new_sessions, &died_sessions);
                    tracing::debug!(
                        "Emitted events: {} new, {} died sessions",
                        new_sessions.len(),
                        died_sessions.len()
                    );
                }

                // Emit refresh event to Tauri
                let _ = app_handle.emit("sessions:refreshed", ());
                let _ = bridge.event_tx.send(BridgeEvent::SessionsRefreshed);
            }
        });

        tracing::info!("Ferox Bridge sync started (interval: {}s)", interval_secs);
    }

    /// Stop background sync
    pub fn stop_sync(&self) {
        *self.syncing.write() = false;
        tracing::info!("Ferox Bridge sync stopped");
    }

    /// Sync sessions from core to UI cache and return changes for event emission
    /// Returns (new_sessions, died_sessions) for Tauri event emission
    pub async fn sync_sessions(&self) -> (Vec<UiSession>, Vec<(String, String)>) {
        let core_sessions = self.core_manager.list_all().await;
        let mut ui_sessions = self.ui_sessions.write();

        // Track changes for events
        let old_ids: std::collections::HashSet<_> = ui_sessions.keys().cloned().collect();
        let mut new_ids = std::collections::HashSet::new();
        let mut new_sessions = Vec::new();
        let mut died_sessions = Vec::new();

        for core_session in core_sessions {
            let ui_session = Self::convert_session(&core_session);
            let id = ui_session.id.clone();
            new_ids.insert(id.clone());

            // Check if session is new
            if !old_ids.contains(&id) {
                let _ = self.event_tx.send(BridgeEvent::SessionCreated(id.clone()));
                new_sessions.push(ui_session.clone());
            }

            ui_sessions.insert(id, ui_session);
        }

        // Check for died sessions
        for old_id in old_ids.difference(&new_ids) {
            if let Some(old_session) = ui_sessions.get(old_id) {
                if old_session.status != SessionStatus::Dead {
                    let _ = self.event_tx.send(BridgeEvent::SessionDied(old_id.clone()));
                    died_sessions.push((old_id.clone(), old_session.hostname.clone()));
                }
            }
        }

        drop(ui_sessions); // Release the lock before returning
        (new_sessions, died_sessions)
    }

    /// Emit Tauri events for session changes
    pub fn emit_session_events(&self, app_handle: &AppHandle, new_sessions: &[UiSession], died_sessions: &[(String, String)]) {
        for session in new_sessions {
            let _ = app_handle.emit("session:created", serde_json::json!({
                "session": session
            }));
        }

        for (session_id, hostname) in died_sessions {
            let _ = app_handle.emit("session:died", serde_json::json!({
                "session_id": session_id,
                "hostname": hostname
            }));
        }
    }

    /// Get all UI sessions
    pub fn get_all_sessions(&self) -> Vec<UiSession> {
        self.ui_sessions.read().values().cloned().collect()
    }

    /// Get a single UI session by ID
    pub fn get_session(&self, id: &str) -> Option<UiSession> {
        self.ui_sessions.read().get(id).cloned()
    }

    /// Get session count
    pub fn session_count(&self) -> usize {
        self.ui_sessions.read().len()
    }

    /// Get active session count
    pub fn active_session_count(&self) -> usize {
        self.ui_sessions
            .read()
            .values()
            .filter(|s| s.status == SessionStatus::Active)
            .count()
    }

    /// Create a new session via core
    pub async fn create_session(
        &self,
        hostname: String,
        ip_address: String,
        os: OsType,
        username: String,
        privileges: PrivilegeLevel,
        parent_id: Option<String>,
    ) -> Result<UiSession> {
        // Convert to core types
        let platform = match os {
            OsType::Windows => Platform::Windows,
            OsType::Linux => Platform::Linux,
            OsType::MacOS => Platform::MacOS,
            OsType::Unknown => Platform::Any,
        };

        // Create core session
        let mut core_session =
            CoreSession::new(format!("desktop/{}", hostname), ip_address.clone(), platform);
        core_session.user = Some(username.clone());

        // Add metadata for UI-specific fields
        core_session
            .metadata
            .insert("hostname".to_string(), serde_json::json!(hostname));
        core_session
            .metadata
            .insert("privileges".to_string(), serde_json::json!(privileges.as_str()));
        if let Some(ref pid) = parent_id {
            core_session
                .metadata
                .insert("parent_id".to_string(), serde_json::json!(pid));
        }

        // Add to core manager
        let id = self.core_manager.add(core_session.clone()).await;

        // Convert and cache
        let ui_session = Self::convert_session(&core_session);
        self.ui_sessions
            .write()
            .insert(ui_session.id.clone(), ui_session.clone());

        // Emit event
        let _ = self.event_tx.send(BridgeEvent::SessionCreated(id.to_string()));

        Ok(ui_session)
    }

    /// Terminate a session
    pub async fn terminate_session(&self, id: &str) -> Result<()> {
        let uuid =
            Uuid::parse_str(id).map_err(|e| anyhow::anyhow!("Invalid session ID: {}", e))?;

        // Kill in core
        self.core_manager.kill(uuid).await?;

        // Update UI cache
        if let Some(session) = self.ui_sessions.write().get_mut(id) {
            session.status = SessionStatus::Dead;
        }

        // Emit event
        let _ = self.event_tx.send(BridgeEvent::SessionDied(id.to_string()));

        Ok(())
    }

    /// Update session heartbeat
    pub async fn heartbeat(&self, id: &str) -> Result<()> {
        let uuid =
            Uuid::parse_str(id).map_err(|e| anyhow::anyhow!("Invalid session ID: {}", e))?;

        self.core_manager.heartbeat(uuid).await?;

        // Update UI cache
        if let Some(session) = self.ui_sessions.write().get_mut(id) {
            session.last_seen = chrono::Utc::now();
        }

        Ok(())
    }

    /// Execute command on session via Ferox Core
    pub async fn execute_command(&self, id: &str, command: &str) -> Result<CommandOutput> {
        let uuid =
            Uuid::parse_str(id).map_err(|e| anyhow::anyhow!("Invalid session ID: {}", e))?;

        let output = self.core_manager.execute_command(uuid, command).await?;

        // Update metrics in UI cache
        if let Some(session) = self.ui_sessions.write().get_mut(id) {
            session.metrics.commands_executed += 1;
        }

        // Emit event
        let _ = self.event_tx.send(BridgeEvent::CommandOutput {
            session_id: id.to_string(),
            output: output.clone(),
        });

        Ok(CommandOutput {
            stdout: output,
            stderr: String::new(),
            exit_code: 0,
        })
    }

    /// Update session note
    pub fn update_note(&self, id: &str, note: Option<String>) {
        if let Some(session) = self.ui_sessions.write().get_mut(id) {
            session.note = note;
        }
    }

    /// Build session tree
    pub fn get_session_tree(&self) -> Vec<SessionTreeNode> {
        let sessions = self.ui_sessions.read();
        let mut roots: Vec<SessionTreeNode> = Vec::new();
        let mut children_map: HashMap<String, Vec<UiSession>> = HashMap::new();

        // Group sessions by parent
        for session in sessions.values() {
            if let Some(ref parent_id) = session.parent_id {
                children_map
                    .entry(parent_id.clone())
                    .or_default()
                    .push(session.clone());
            }
        }

        // Build tree from root sessions
        for session in sessions.values() {
            if session.parent_id.is_none() {
                let node = Self::build_tree_node(session.clone(), &children_map);
                roots.push(node);
            }
        }

        roots
    }

    fn build_tree_node(
        session: UiSession,
        children_map: &HashMap<String, Vec<UiSession>>,
    ) -> SessionTreeNode {
        let children = children_map
            .get(&session.id)
            .map(|child_sessions| {
                child_sessions
                    .iter()
                    .map(|s| Self::build_tree_node(s.clone(), children_map))
                    .collect()
            })
            .unwrap_or_default();

        SessionTreeNode { session, children }
    }

    /// Convert Ferox core Session to UI Session
    fn convert_session(core: &CoreSession) -> UiSession {
        // Extract metadata
        let hostname = core
            .metadata
            .get("hostname")
            .and_then(|v| v.as_str())
            .unwrap_or(&core.target)
            .to_string();

        let privileges = core
            .metadata
            .get("privileges")
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "administrator" => PrivilegeLevel::Administrator,
                "system" => PrivilegeLevel::System,
                "root" => PrivilegeLevel::Root,
                _ => PrivilegeLevel::User,
            })
            .unwrap_or(PrivilegeLevel::User);

        let parent_id = core
            .metadata
            .get("parent_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let os = match core.platform {
            Platform::Windows => OsType::Windows,
            Platform::Linux => OsType::Linux,
            Platform::MacOS => OsType::MacOS,
            Platform::Any => OsType::Unknown,
        };

        let status = if core.active {
            SessionStatus::Active
        } else {
            SessionStatus::Dead
        };

        UiSession {
            id: core.id.to_string(),
            hostname,
            ip_address: core.target.clone(),
            os,
            os_version: core
                .metadata
                .get("os_version")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            architecture: Architecture::X64,
            username: core.user.clone().unwrap_or_else(|| "unknown".to_string()),
            privileges,
            status,
            established_at: core.established_at,
            last_seen: core.last_seen,
            parent_id,
            intelligence: SessionIntelligence::default(),
            metrics: SessionMetrics {
                credentials_count: core
                    .metadata
                    .get("creds_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                commands_executed: core
                    .metadata
                    .get("cmds_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                files_transferred: 0,
                persistence_methods: 0,
            },
            tags: Vec::new(),
            note: core
                .metadata
                .get("note")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        }
    }

    // ========================================================================
    // Post-Exploitation Methods
    // ========================================================================

    /// Deploy a payload to establish a new session
    pub async fn deploy_payload(&self, target: &str, payload_type: &str) -> Result<DeployResult> {
        tracing::info!("Deploying {} payload to {}", payload_type, target);

        // TODO: Phase 4 - Use ferox::PayloadSystem

        Ok(DeployResult {
            success: false,
            session_id: None,
            message: "Payload deployment pending Phase 4 implementation".to_string(),
        })
    }

    /// Run privilege escalation scan using Ferox Core
    pub async fn scan_privesc(&self, session_id: &str, _safe_mode: bool) -> Result<Vec<PrivEscVector>> {
        tracing::debug!("Running privesc scan on session {}", session_id);

        let uuid = Uuid::parse_str(session_id)
            .map_err(|e| anyhow::anyhow!("Invalid session ID: {}", e))?;

        // Verify session exists
        let _core_session = self
            .core_manager
            .get(uuid)
            .await
            .ok_or_else(|| anyhow::anyhow!("Session not found: {}", session_id))?;

        // Run enumeration with the privesc engine
        // Note: Using placeholder for now since enumerate_all is async and parking_lot lock can't be held across await
        // TODO: Refactor to use tokio::sync::RwLock for async support
        let result: Vec<PrivEscVector> = {
            let _engine = self.privesc_engine.read();
            // Return empty for now - structure is in place for future async integration
            Vec::new()
        };

        // Emit event
        let _ = self.event_tx.send(BridgeEvent::PrivilegeEscalated {
            session_id: session_id.to_string(),
            new_privilege: format!("{} vectors found", result.len()),
        });

        Ok(result)
    }

    /// Harvest credentials from session using Ferox Core
    pub async fn harvest_credentials(
        &self,
        session_id: &str,
        _sources: &[String],
        _safe_mode: bool,
    ) -> Result<Vec<HarvestedCredential>> {
        tracing::debug!("Harvesting credentials from session {}", session_id);

        let uuid = Uuid::parse_str(session_id)
            .map_err(|e| anyhow::anyhow!("Invalid session ID: {}", e))?;

        // Verify session exists
        let _core_session = self
            .core_manager
            .get(uuid)
            .await
            .ok_or_else(|| anyhow::anyhow!("Session not found: {}", session_id))?;

        // Run credential harvest
        // Note: Using placeholder for now since harvest_all is async and parking_lot lock can't be held across await
        // TODO: Refactor to use tokio::sync::RwLock for full async support
        let credentials: Vec<HarvestedCredential> = {
            let _engine = self.cred_engine.read();
            // Return empty for now - structure is in place for future async integration
            Vec::new()
        };

        // Emit event
        let _ = self.event_tx.send(BridgeEvent::CredentialsFound {
            session_id: session_id.to_string(),
            count: credentials.len() as u32,
        });

        // Update session metrics
        if let Some(session) = self.ui_sessions.write().get_mut(session_id) {
            session.metrics.credentials_count = credentials.len() as u32;
        }

        Ok(credentials)
    }

    /// Install persistence on a session using Ferox Core
    pub async fn install_persistence(
        &self,
        session_id: &str,
        _payload_path: &str,
        _persistence_name: &str,
        _safe_mode: bool,
    ) -> Result<PersistenceResult> {
        tracing::debug!("Installing persistence on session {}", session_id);

        let uuid = Uuid::parse_str(session_id)
            .map_err(|e| anyhow::anyhow!("Invalid session ID: {}", e))?;

        // Verify session exists
        let _core_session = self
            .core_manager
            .get(uuid)
            .await
            .ok_or_else(|| anyhow::anyhow!("Session not found: {}", session_id))?;

        // Install persistence
        // Note: Using placeholder for now since install_auto is async and parking_lot lock can't be held across await
        // TODO: Refactor to use tokio::sync::RwLock for full async support
        let (success, handles, message) = {
            let _engine = self.persistence_engine.read();
            // Return placeholder - structure is in place for future async integration
            (false, Vec::new(), "Persistence installation pending async lock refactor".to_string())
        };

        Ok(PersistenceResult {
            success,
            handles,
            message,
        })
    }
}

impl Default for FeroxBridge {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Bridge Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployResult {
    pub success: bool,
    pub session_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivEscVector {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: String,
    pub exploitable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestedCredential {
    pub username: String,
    pub domain: Option<String>,
    pub cred_type: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceHandle {
    pub id: String,
    pub method: String,
    pub name: String,
    pub location: String,
    pub status: String,
    pub mitre_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceResult {
    pub success: bool,
    pub handles: Vec<PersistenceHandle>,
    pub message: String,
}
