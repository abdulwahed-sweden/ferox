//! Ferox Dashboard Server - Real-time C2 Dashboard
//!
//! A WebSocket-based dashboard server for managing Ferox penetration testing
//! sessions in real-time.
//!
//! ## Features
//! - Real-time session management via WebSocket
//! - REST API for session data and commands
//! - Live command execution with streaming output
//! - CORS support for frontend development
//! - Static file serving for React dashboard
//!
//! ## Usage
//! ```bash
//! cargo run --package ferox-dashboard-server
//! # Server starts on http://0.0.0.0:8080
//! ```

mod api;
mod state;
mod types;
mod ws;

use axum::{
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing::{info, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("dashboard_server=debug,tower_http=debug")),
        )
        .init();

    info!("Starting Ferox Dashboard Server...");

    // Initialize shared state
    let state = AppState::new();

    // Initialize with demo data
    state.init_demo_data().await;
    info!("Demo data initialized");

    // Configure CORS (permissive for development)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build API routes
    let api_routes = Router::new()
        // Health check
        .route("/health", get(api::health_check))
        // Sessions
        .route("/sessions", get(api::list_sessions))
        .route("/sessions/:id", get(api::get_session))
        .route("/sessions/:id", delete(api::terminate_session))
        .route("/sessions/:id/execute", post(api::execute_command))
        .route("/sessions/:id/commands", get(api::get_session_commands))
        .route("/sessions/:id/heartbeat", post(api::session_heartbeat))
        // Credentials
        .route("/credentials", get(api::list_credentials))
        // Statistics
        .route("/stats", get(api::get_stats))
        // MITRE
        .route("/mitre/coverage", get(api::get_mitre_coverage))
        // Network topology
        .route("/network/hosts", get(api::get_network_hosts))
        .route("/network/edges", get(api::get_network_edges));

    // Build main router
    let app = Router::new()
        // WebSocket endpoint
        .route("/ws", get(ws::ws_handler))
        // API routes under /api prefix
        .nest("/api", api_routes)
        // Serve static files (React dashboard)
        .nest_service("/", ServeDir::new("static").append_index_html_on_directories(true))
        // Add CORS middleware
        .layer(cors)
        // Add shared state
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Dashboard server listening on http://{}", addr);
    info!("WebSocket endpoint: ws://{}/ws", addr);
    info!("API endpoints: http://{}/api/*", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_state_initialization() {
        let state = AppState::new();
        state.init_demo_data().await;

        let sessions = state.get_sessions().await;
        assert!(!sessions.is_empty(), "Should have demo sessions");

        let credentials = state.get_credentials().await;
        assert!(!credentials.is_empty(), "Should have demo credentials");
    }

    #[tokio::test]
    async fn test_session_operations() {
        let state = AppState::new();

        // Add a session
        let session = types::DashboardSession::new(
            "TEST-HOST".to_string(),
            "10.0.0.1".to_string(),
            types::OsType::Windows,
            "testuser".to_string(),
            types::PrivilegeLevel::User,
        );
        let session_id = session.id;

        state.add_session(session).await;

        // Verify session exists
        let retrieved = state.get_session(session_id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().hostname, "TEST-HOST");

        // Remove session
        state.remove_session(session_id).await;
        assert!(state.get_session(session_id).await.is_none());
    }

    #[tokio::test]
    async fn test_command_history() {
        let state = AppState::new();
        let session_id = uuid::Uuid::new_v4();

        // Add commands
        let cmd1 = types::Command::new(session_id, "whoami".to_string());
        let cmd2 = types::Command::new(session_id, "hostname".to_string());

        state.add_command(cmd1).await;
        state.add_command(cmd2).await;

        // Verify commands
        let commands = state.get_commands(session_id).await;
        assert_eq!(commands.len(), 2);
    }
}
