//! Session Management Commands
//!
//! Tauri commands for session management operations.
//! These commands now use FeroxBridge to connect to Ferox Core.

use crate::security::Validator;
use crate::session::{OsType, PrivilegeLevel, Session, SessionTreeNode};
use crate::AppState;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

/// Session list response
#[derive(Debug, Serialize)]
pub struct SessionListResponse {
    pub sessions: Vec<Session>,
    pub total: usize,
    pub active_count: usize,
}

/// Create session request
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub hostname: String,
    pub ip_address: String,
    pub os: String,
    pub username: String,
    pub privileges: String,
    pub parent_id: Option<String>,
}

/// Get all sessions (from Ferox Core via bridge)
#[tauri::command]
pub async fn get_sessions(
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<SessionListResponse, String> {
    let bridge = {
        let state = state.read();
        state.bridge.clone()
    };

    // Sync from core first to ensure fresh data (discard change events here)
    let _ = bridge.sync_sessions().await;

    let sessions = bridge.get_all_sessions();
    let active_count = bridge.active_session_count();

    Ok(SessionListResponse {
        total: sessions.len(),
        sessions,
        active_count,
    })
}

/// Get a single session by ID (from Ferox Core via bridge)
#[tauri::command]
pub async fn get_session(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Session, String> {
    // Validate input
    Validator::session_id(&id).map_err(|e| e.to_string())?;

    let bridge = {
        let state = state.read();
        state.bridge.clone()
    };

    bridge
        .get_session(&id)
        .ok_or_else(|| format!("Session not found: {}", id))
}

/// Create a new session (via Ferox Core bridge)
#[tauri::command]
pub async fn create_session(
    request: CreateSessionRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Session, String> {
    // Validate inputs
    Validator::hostname(&request.hostname).map_err(|e| e.to_string())?;
    Validator::ip_address(&request.ip_address).map_err(|e| e.to_string())?;
    if let Some(ref parent_id) = request.parent_id {
        Validator::session_id(parent_id).map_err(|e| e.to_string())?;
    }

    let os = match request.os.to_lowercase().as_str() {
        "windows" => OsType::Windows,
        "linux" => OsType::Linux,
        "macos" => OsType::MacOS,
        _ => OsType::Unknown,
    };

    let privileges = match request.privileges.to_lowercase().as_str() {
        "administrator" | "admin" => PrivilegeLevel::Administrator,
        "system" => PrivilegeLevel::System,
        "root" => PrivilegeLevel::Root,
        _ => PrivilegeLevel::User,
    };

    let (bridge, audit_logger) = {
        let state = state.read();
        (state.bridge.clone(), state.audit_logger.clone())
    };

    // Create session via bridge (which calls Ferox Core)
    let session = bridge
        .create_session(
            request.hostname.clone(),
            request.ip_address.clone(),
            os,
            request.username,
            privileges,
            request.parent_id,
        )
        .await
        .map_err(|e| e.to_string())?;

    // Audit log the session creation
    audit_logger.log_session_created(&session.id, &request.hostname, &request.ip_address);

    tracing::info!(
        "Created session {} for {} at {}",
        session.id,
        request.hostname,
        request.ip_address
    );

    Ok(session)
}

/// Terminate a session (via Ferox Core bridge)
#[tauri::command]
pub async fn terminate_session(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<(), String> {
    // Validate input
    Validator::session_id(&id).map_err(|e| e.to_string())?;

    let (bridge, audit_logger) = {
        let state = state.read();
        (state.bridge.clone(), state.audit_logger.clone())
    };

    // Terminate via bridge (which calls Ferox Core)
    bridge
        .terminate_session(&id)
        .await
        .map_err(|e| e.to_string())?;

    // Audit log the termination
    audit_logger.log_session_terminated(&id);

    tracing::info!("Terminated session: {}", id);
    Ok(())
}

/// Update session note
#[tauri::command]
pub async fn update_session_note(
    id: String,
    note: Option<String>,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<(), String> {
    // Validate inputs
    Validator::session_id(&id).map_err(|e| e.to_string())?;
    Validator::note(&note).map_err(|e| e.to_string())?;

    let bridge = {
        let state = state.read();
        state.bridge.clone()
    };

    bridge.update_note(&id, note);
    Ok(())
}

/// Get session tree (hierarchical view from bridge)
#[tauri::command]
pub async fn get_session_tree(
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<SessionTreeNode>, String> {
    let bridge = {
        let state = state.read();
        state.bridge.clone()
    };

    Ok(bridge.get_session_tree())
}
