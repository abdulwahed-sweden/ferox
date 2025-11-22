//! REST API handlers for the dashboard server
//!
//! Provides endpoints for session management, credentials, and statistics.

use crate::state::AppState;
use crate::types::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

/// GET /api/sessions - List all sessions
pub async fn list_sessions(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<SessionListResponse>> {
    let sessions = state.get_sessions().await;
    let active_count = sessions
        .iter()
        .filter(|s| s.status == SessionStatus::Active)
        .count();

    Json(ApiResponse::success(SessionListResponse {
        total: sessions.len(),
        active_count,
        sessions,
    }))
}

/// GET /api/sessions/:id - Get session details
pub async fn get_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DashboardSession>>, StatusCode> {
    match state.get_session(id).await {
        Some(session) => Ok(Json(ApiResponse::success(session))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /api/sessions/:id/execute - Execute command on session
pub async fn execute_command(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    Json(request): Json<ExecuteCommandRequest>,
) -> Result<Json<ApiResponse<Command>>, StatusCode> {
    // Verify session exists
    let session = state.get_session(session_id).await;
    if session.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    info!(
        session_id = %session_id,
        command = %request.command,
        "API: Execute command"
    );

    // Create command
    let command = Command::new(session_id, request.command.clone());
    let command_clone = command.clone();
    state.add_command(command).await;

    // Update session
    if let Some(mut session) = state.sessions.write().await.get_mut(&session_id) {
        session.metrics.commands_executed += 1;
        session.last_seen = chrono::Utc::now();
    }

    // In production, this would trigger actual command execution
    // For now, broadcast that command was queued
    state
        .broadcast_command_output(
            command_clone.id,
            session_id,
            "Command queued for execution...".to_string(),
            false,
            None,
        )
        .await;

    Ok(Json(ApiResponse::success(command_clone)))
}

/// GET /api/sessions/:id/commands - Get command history for session
pub async fn get_session_commands(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Json<ApiResponse<Vec<Command>>> {
    let commands = state.get_commands(session_id).await;
    Json(ApiResponse::success(commands))
}

/// GET /api/credentials - List all credentials (redacted)
pub async fn list_credentials(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<DashboardCredential>>> {
    let credentials = state.get_credentials().await;
    Json(ApiResponse::success(credentials))
}

/// GET /api/stats - Get dashboard statistics
pub async fn get_stats(State(state): State<Arc<AppState>>) -> Json<ApiResponse<DashboardStats>> {
    let stats = state.get_stats().await;
    Json(ApiResponse::success(stats))
}

/// GET /api/mitre/coverage - Get MITRE ATT&CK coverage
pub async fn get_mitre_coverage(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<MitreCoverage>> {
    let coverage = state.get_mitre_coverage().await;
    Json(ApiResponse::success(coverage))
}

/// POST /api/sessions/:id/heartbeat - Session heartbeat
pub async fn session_heartbeat(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    if state.get_session(session_id).await.is_some() {
        state.session_heartbeat(session_id).await;
        Ok(Json(ApiResponse::success(())))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// DELETE /api/sessions/:id - Terminate a session
pub async fn terminate_session(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    if state.get_session(session_id).await.is_some() {
        state.remove_session(session_id).await;
        info!(session_id = %session_id, "Session terminated via API");
        Ok(Json(ApiResponse::success(())))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// GET /api/network/hosts - Get network topology hosts
pub async fn get_network_hosts(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<NetworkHost>>> {
    let hosts: Vec<NetworkHost> = state
        .network_hosts
        .read()
        .await
        .values()
        .cloned()
        .collect();
    Json(ApiResponse::success(hosts))
}

/// GET /api/network/edges - Get network topology edges
pub async fn get_network_edges(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<NetworkEdge>>> {
    let edges = state.network_edges.read().await.clone();
    Json(ApiResponse::success(edges))
}

/// Health check endpoint
pub async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("Ferox Dashboard Server is running"))
}
