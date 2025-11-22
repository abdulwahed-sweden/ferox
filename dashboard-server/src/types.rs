//! Core types for the Ferox Dashboard Server
//!
//! Defines all data structures for sessions, WebSocket events,
//! credentials, and API responses.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Session Types
// ============================================================================

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
            Self::User => "User",
            Self::Administrator => "Administrator",
            Self::System => "SYSTEM",
            Self::Root => "root",
        }
    }

    pub fn badge_color(&self) -> &'static str {
        match self {
            Self::User => "gray",
            Self::Administrator => "orange",
            Self::System | Self::Root => "red",
        }
    }
}

/// Session status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Active,
    Sleeping,
    Dead,
}

impl SessionStatus {
    pub fn from_last_seen(last_seen: DateTime<Utc>) -> Self {
        let elapsed = Utc::now().signed_duration_since(last_seen);
        if elapsed.num_seconds() < 30 {
            Self::Active
        } else if elapsed.num_seconds() < 300 {
            Self::Sleeping
        } else {
            Self::Dead
        }
    }
}

/// Operating system type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsType {
    Windows,
    Linux,
    MacOS,
    Unknown,
}

/// System architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Architecture {
    #[serde(rename = "x64")]
    X64,
    #[serde(rename = "x86")]
    X86,
    #[serde(rename = "arm64")]
    Arm64,
    Unknown,
}

/// Intelligence data about a session
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

/// Session metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionMetrics {
    pub credentials_count: u32,
    pub commands_executed: u32,
    pub files_transferred: u32,
    pub persistence_methods: u32,
}

/// A compromised session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSession {
    pub id: Uuid,
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
    pub intelligence: SessionIntelligence,
    pub metrics: SessionMetrics,
    pub tags: Vec<String>,
}

impl DashboardSession {
    pub fn new(
        hostname: String,
        ip_address: String,
        os: OsType,
        username: String,
        privileges: PrivilegeLevel,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
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
            intelligence: SessionIntelligence::default(),
            metrics: SessionMetrics::default(),
            tags: Vec::new(),
        }
    }

    pub fn update_status(&mut self) {
        self.status = SessionStatus::from_last_seen(self.last_seen);
    }

    pub fn heartbeat(&mut self) {
        self.last_seen = Utc::now();
        self.status = SessionStatus::Active;
    }
}

// ============================================================================
// Command Types
// ============================================================================

/// A command executed on a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: Uuid,
    pub session_id: Uuid,
    pub command: String,
    pub output: String,
    pub timestamp: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub success: bool,
    pub execution_time_ms: Option<u64>,
}

impl Command {
    pub fn new(session_id: Uuid, command: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id,
            command,
            output: String::new(),
            timestamp: Utc::now(),
            completed_at: None,
            success: false,
            execution_time_ms: None,
        }
    }
}

// ============================================================================
// Credential Types
// ============================================================================

/// Type of credential
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialType {
    PlainText,
    NtlmHash,
    KerberosTicket,
    SshKey,
    CloudCredential,
    Token,
    Certificate,
}

/// Sensitivity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sensitivity {
    Low,
    Medium,
    High,
    Critical,
}

/// A harvested credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardCredential {
    pub id: Uuid,
    pub cred_type: CredentialType,
    pub username: String,
    pub domain: Option<String>,
    pub secret: String,  // Redacted in API responses
    pub source_hostname: String,
    pub source_session_id: Uuid,
    pub sensitivity: Sensitivity,
    pub collected_at: DateTime<Utc>,
    pub is_reusable: bool,
    pub notes: Option<String>,
}

impl DashboardCredential {
    pub fn redacted(&self) -> Self {
        let mut redacted = self.clone();
        redacted.secret = "********".to_string();
        redacted
    }
}

// ============================================================================
// Network Types
// ============================================================================

/// A discovered network host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkHost {
    pub id: Uuid,
    pub hostname: Option<String>,
    pub ip_address: String,
    pub os: Option<OsType>,
    pub services: Vec<String>,
    pub ports: Vec<u16>,
    pub is_compromised: bool,
    pub session_id: Option<Uuid>,
    pub credentials_available: u32,
    pub is_domain_controller: bool,
    pub is_high_value: bool,
    pub discovered_at: DateTime<Utc>,
}

/// Network connection between hosts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEdge {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub protocol: String,  // SMB, SSH, RDP, WMI
    pub port: u16,
    pub can_pivot: bool,
}

// ============================================================================
// MITRE ATT&CK Types
// ============================================================================

/// MITRE technique usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreTechniqueUsage {
    pub technique_id: String,
    pub technique_name: String,
    pub tactic: String,
    pub times_used: u32,
    pub success_rate: f32,
    pub detection_risk: String,  // low, medium, high
    pub last_used: Option<DateTime<Utc>>,
    pub sessions_used: Vec<Uuid>,
}

/// MITRE coverage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreCoverage {
    pub total_techniques: u32,
    pub techniques_used: u32,
    pub coverage_percentage: f32,
    pub tactics_covered: Vec<String>,
    pub techniques: Vec<MitreTechniqueUsage>,
}

// ============================================================================
// WebSocket Event Types
// ============================================================================

/// Events sent from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerEvent {
    /// A new session was created
    SessionCreated(DashboardSession),
    /// Session was updated
    SessionUpdated(DashboardSession),
    /// Session was closed/terminated
    SessionClosed { session_id: Uuid },
    /// Command output (supports chunking)
    CommandOutput {
        command_id: Uuid,
        session_id: Uuid,
        output: String,
        is_complete: bool,
        success: Option<bool>,
    },
    /// Credentials were found
    CredentialsFound {
        session_id: Uuid,
        credentials: Vec<DashboardCredential>,
    },
    /// OPSEC alert
    OpsecAlert {
        session_id: Uuid,
        level: String,
        message: String,
        recommendation: String,
    },
    /// Connection established
    Connected { client_id: Uuid },
    /// Heartbeat response
    Pong,
    /// Error message
    Error { message: String },
}

/// Events sent from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientEvent {
    /// Execute command on a session
    ExecuteCommand {
        session_id: Uuid,
        command: String,
    },
    /// Subscribe to session events
    SubscribeToSession { session_id: Uuid },
    /// Unsubscribe from session events
    UnsubscribeFromSession { session_id: Uuid },
    /// Heartbeat ping
    Ping,
    /// Request full session list
    RequestSessions,
}

// ============================================================================
// API Response Types
// ============================================================================

/// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

/// Session list response
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub sessions: Vec<DashboardSession>,
    pub total: usize,
    pub active_count: usize,
}

/// Execute command request
#[derive(Debug, Deserialize)]
pub struct ExecuteCommandRequest {
    pub command: String,
}

/// Dashboard statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub active_sessions: u32,
    pub total_sessions: u32,
    pub credentials_collected: u32,
    pub targets_discovered: u32,
    pub mitre_coverage: f32,
}
