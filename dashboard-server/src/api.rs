//! REST API handlers for the dashboard server
//!
//! Provides endpoints for session management, credentials, statistics,
//! and post-exploitation modules (PrivEsc, Credentials, Persistence, Lateral Movement).

use crate::integration::modules::{
    CredentialHarvestRequest, CredentialHarvestResult, DiscoveryResult,
    LateralMoveRequest, LateralMoveResult, ModuleBridge,
    PersistenceRequest, PersistenceResult, PrivEscRequest, PrivEscResult,
};
use crate::state::AppState;
use crate::types::*;
use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tracing::{info, error};
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
    if let Some(session) = state.sessions.write().await.get_mut(&session_id) {
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

// ============================================================================
// Post-Exploitation Module Endpoints
// ============================================================================

/// POST /api/modules/privesc - Run privilege escalation
pub async fn run_privesc(
    Extension(module_bridge): Extension<Arc<ModuleBridge>>,
    Json(request): Json<PrivEscRequest>,
) -> Result<Json<ApiResponse<PrivEscResult>>, StatusCode> {
    info!(
        session_id = %request.session_id,
        auto_escalate = request.auto_escalate,
        safe_mode = request.safe_mode,
        "API: Running privilege escalation"
    );

    match module_bridge.run_privesc(request).await {
        Ok(result) => Ok(Json(ApiResponse::success(result))),
        Err(e) => {
            error!("PrivEsc failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// POST /api/modules/credentials - Harvest credentials
pub async fn harvest_credentials(
    Extension(module_bridge): Extension<Arc<ModuleBridge>>,
    Json(request): Json<CredentialHarvestRequest>,
) -> Result<Json<ApiResponse<CredentialHarvestResult>>, StatusCode> {
    info!(
        session_id = %request.session_id,
        sources = ?request.sources,
        safe_mode = request.safe_mode,
        "API: Harvesting credentials"
    );

    match module_bridge.harvest_credentials(request).await {
        Ok(result) => Ok(Json(ApiResponse::success(result))),
        Err(e) => {
            error!("Credential harvest failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// POST /api/modules/persistence - Install persistence
pub async fn install_persistence(
    Extension(module_bridge): Extension<Arc<ModuleBridge>>,
    Json(request): Json<PersistenceRequest>,
) -> Result<Json<ApiResponse<PersistenceResult>>, StatusCode> {
    info!(
        session_id = %request.session_id,
        method = %request.method,
        name = %request.name,
        safe_mode = request.safe_mode,
        "API: Installing persistence"
    );

    match module_bridge.install_persistence(request).await {
        Ok(result) => Ok(Json(ApiResponse::success(result))),
        Err(e) => {
            error!("Persistence installation failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// POST /api/modules/lateral - Perform lateral movement
pub async fn lateral_move(
    Extension(module_bridge): Extension<Arc<ModuleBridge>>,
    Json(request): Json<LateralMoveRequest>,
) -> Result<Json<ApiResponse<LateralMoveResult>>, StatusCode> {
    info!(
        session_id = %request.session_id,
        target_host = %request.target_host,
        method = %request.method,
        safe_mode = request.safe_mode,
        "API: Performing lateral movement"
    );

    match module_bridge.lateral_move(request).await {
        Ok(result) => Ok(Json(ApiResponse::success(result))),
        Err(e) => {
            error!("Lateral movement failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/modules/discovery/:session_id - Discover network targets
pub async fn discover_network(
    Extension(module_bridge): Extension<Arc<ModuleBridge>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ApiResponse<DiscoveryResult>>, StatusCode> {
    info!(session_id = %session_id, "API: Discovering network targets");

    match module_bridge.discover_network(session_id).await {
        Ok(result) => Ok(Json(ApiResponse::success(result))),
        Err(e) => {
            error!("Network discovery failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
