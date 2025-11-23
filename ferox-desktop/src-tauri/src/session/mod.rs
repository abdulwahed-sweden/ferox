//! Session Management Module
//!
//! Manages C2 sessions with tree hierarchy support for pivoted sessions.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Privilege level of a session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrivilegeLevel {
    User,
    Administrator,
    System,
    Root,
}

impl PrivilegeLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Administrator => "administrator",
            Self::System => "system",
            Self::Root => "root",
        }
    }
}

/// Status of a session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Active,
    Sleeping,
    Dead,
}

/// Operating system type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsType {
    Windows,
    Linux,
    MacOS,
    Unknown,
}

/// Architecture type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    X64,
    X86,
    Arm64,
    Unknown,
}

/// Intelligence gathered about a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionIntelligence {
    pub domain: Option<String>,
    pub is_domain_joined: bool,
    pub detected_av: Vec<String>,
    pub stealth_mode: String,
    pub network_segment: Option<String>,
}

impl Default for SessionIntelligence {
    fn default() -> Self {
        Self {
            domain: None,
            is_domain_joined: false,
            detected_av: Vec::new(),
            stealth_mode: "normal".to_string(),
            network_segment: None,
        }
    }
}

/// Metrics for a session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionMetrics {
    pub credentials_count: u32,
    pub commands_executed: u32,
    pub files_transferred: u32,
    pub persistence_methods: u32,
}

/// A C2 session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub hostname: String,
    pub ip_address: String,
    pub os: OsType,
    pub os_version: Option<String>,
    pub architecture: Architecture,
    pub username: String,
    pub privileges: PrivilegeLevel,
    pub status: SessionStatus,
    pub established_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub parent_id: Option<String>,
    pub intelligence: SessionIntelligence,
    pub metrics: SessionMetrics,
    pub tags: Vec<String>,
    pub note: Option<String>,
}

impl Session {
    pub fn new(
        hostname: String,
        ip_address: String,
        os: OsType,
        username: String,
        privileges: PrivilegeLevel,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            hostname,
            ip_address,
            os,
            os_version: None,
            architecture: Architecture::X64,
            username,
            privileges,
            status: SessionStatus::Active,
            established_at: now,
            last_seen: now,
            parent_id: None,
            intelligence: SessionIntelligence::default(),
            metrics: SessionMetrics::default(),
            tags: Vec::new(),
            note: None,
        }
    }
}

/// Tree node for session hierarchy display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTreeNode {
    pub session: Session,
    pub children: Vec<SessionTreeNode>,
}

/// Session Manager for managing all sessions
pub struct SessionManager {
    sessions: RwLock<HashMap<String, Session>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    /// Get all sessions
    pub fn get_all(&self) -> Vec<Session> {
        self.sessions.read().values().cloned().collect()
    }

    /// Get a session by ID
    pub fn get(&self, id: &str) -> Option<Session> {
        self.sessions.read().get(id).cloned()
    }

    /// Add a new session
    pub fn add(&self, session: Session) -> String {
        let id = session.id.clone();
        self.sessions.write().insert(id.clone(), session);
        id
    }

    /// Remove a session
    pub fn remove(&self, id: &str) -> Option<Session> {
        self.sessions.write().remove(id)
    }

    /// Update session status
    pub fn update_status(&self, id: &str, status: SessionStatus) {
        if let Some(session) = self.sessions.write().get_mut(id) {
            session.status = status;
            session.last_seen = Utc::now();
        }
    }

    /// Update session note
    pub fn update_note(&self, id: &str, note: Option<String>) {
        if let Some(session) = self.sessions.write().get_mut(id) {
            session.note = note;
        }
    }

    /// Update last seen timestamp
    pub fn heartbeat(&self, id: &str) {
        if let Some(session) = self.sessions.write().get_mut(id) {
            session.last_seen = Utc::now();
        }
    }

    /// Increment command count
    pub fn increment_commands(&self, id: &str) {
        if let Some(session) = self.sessions.write().get_mut(id) {
            session.metrics.commands_executed += 1;
        }
    }

    /// Build session tree (hierarchical view)
    pub fn get_tree(&self) -> Vec<SessionTreeNode> {
        let sessions = self.sessions.read();
        let mut roots: Vec<SessionTreeNode> = Vec::new();
        let mut children_map: HashMap<String, Vec<Session>> = HashMap::new();

        // Group sessions by parent
        for session in sessions.values() {
            if let Some(ref parent_id) = session.parent_id {
                children_map
                    .entry(parent_id.clone())
                    .or_default()
                    .push(session.clone());
            }
        }

        // Build tree from root sessions (no parent)
        for session in sessions.values() {
            if session.parent_id.is_none() {
                let node = self.build_tree_node(session.clone(), &children_map);
                roots.push(node);
            }
        }

        roots
    }

    fn build_tree_node(
        &self,
        session: Session,
        children_map: &HashMap<String, Vec<Session>>,
    ) -> SessionTreeNode {
        let children = children_map
            .get(&session.id)
            .map(|child_sessions| {
                child_sessions
                    .iter()
                    .map(|s| self.build_tree_node(s.clone(), children_map))
                    .collect()
            })
            .unwrap_or_default();

        SessionTreeNode { session, children }
    }

    /// Get active session count
    pub fn active_count(&self) -> usize {
        self.sessions
            .read()
            .values()
            .filter(|s| s.status == SessionStatus::Active)
            .count()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
