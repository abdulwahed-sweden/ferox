//! WebSocket handler for real-time dashboard communication
//!
//! Handles WebSocket connections, message routing, and event broadcasting.

use crate::integration::FeroxBridge;
use crate::state::{AppState, WsClient};
use crate::types::*;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
    Extension,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Extension(bridge): Extension<Arc<FeroxBridge>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, bridge))
}

/// Handle an individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: Arc<AppState>, bridge: Arc<FeroxBridge>) {
    let client = WsClient::new();
    let client_id = client.id;

    info!(client_id = %client_id, "WebSocket client connected");

    // Register client
    state.add_ws_client(client).await;

    // Subscribe to broadcast channel
    let mut rx = state.event_tx.subscribe();

    // Split socket
    let (mut sender, mut receiver) = socket.split();

    // Send connected event
    let connected_event = ServerEvent::Connected { client_id };
    if let Ok(msg) = serde_json::to_string(&connected_event) {
        let _ = sender.send(Message::Text(msg.into())).await;
    }

    // Spawn task to forward broadcast events to this client
    let send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            match serde_json::to_string(&event) {
                Ok(msg) => {
                    if sender.send(Message::Text(msg.into())).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to serialize event: {}", e);
                }
            }
        }
    });

    // Handle incoming messages
    let state_clone2 = state.clone();
    let bridge_clone = bridge.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    handle_client_message(&state_clone2, &bridge_clone, client_id, &text).await;
                }
                Ok(Message::Ping(_)) => {
                    debug!("Received ping from client {}", client_id);
                    // Pong is automatically sent by axum
                }
                Ok(Message::Pong(_)) => {
                    debug!("Received pong from client {}", client_id);
                }
                Ok(Message::Close(_)) => {
                    info!(client_id = %client_id, "Client requested close");
                    break;
                }
                Ok(Message::Binary(_)) => {
                    warn!("Received binary message, ignoring");
                }
                Err(e) => {
                    error!(client_id = %client_id, error = %e, "WebSocket error");
                    break;
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // Cleanup
    state.remove_ws_client(client_id).await;
    info!(client_id = %client_id, "WebSocket client disconnected");
}

/// Handle incoming client messages
async fn handle_client_message(state: &Arc<AppState>, bridge: &Arc<FeroxBridge>, client_id: Uuid, text: &str) {
    match serde_json::from_str::<ClientEvent>(text) {
        Ok(event) => {
            debug!(client_id = %client_id, event = ?event, "Received client event");

            match event {
                ClientEvent::ExecuteCommand { session_id, command } => {
                    handle_execute_command(state, bridge, session_id, command).await;
                }
                ClientEvent::SubscribeToSession { session_id } => {
                    state.subscribe_to_session(client_id, session_id).await;
                    info!(client_id = %client_id, session_id = %session_id, "Subscribed to session");
                }
                ClientEvent::UnsubscribeFromSession { session_id } => {
                    // For simplicity, just log it - full implementation would remove from list
                    info!(client_id = %client_id, session_id = %session_id, "Unsubscribed from session");
                }
                ClientEvent::Ping => {
                    let _ = state.event_tx.send(ServerEvent::Pong);
                }
                ClientEvent::RequestSessions => {
                    // Send all sessions to this client
                    let sessions = state.get_sessions().await;
                    for session in sessions {
                        let _ = state.event_tx.send(ServerEvent::SessionUpdated(session));
                    }
                }
            }
        }
        Err(e) => {
            warn!(client_id = %client_id, error = %e, "Failed to parse client message: {}", text);
            let _ = state.event_tx.send(ServerEvent::Error {
                message: format!("Invalid message format: {}", e),
            });
        }
    }
}

/// Handle command execution request
async fn handle_execute_command(state: &Arc<AppState>, bridge: &Arc<FeroxBridge>, session_id: Uuid, command_str: String) {
    info!(session_id = %session_id, command = %command_str, "Executing command via FeroxBridge");

    // Verify session exists in dashboard state
    let session = state.get_session(session_id).await;
    if session.is_none() {
        let _ = state.event_tx.send(ServerEvent::Error {
            message: format!("Session {} not found", session_id),
        });
        return;
    }

    // Create command record
    let command = Command::new(session_id, command_str.clone());
    let command_id = command.id;
    state.add_command(command).await;

    // Update session metrics
    if let Some(session) = state.sessions.write().await.get_mut(&session_id) {
        session.metrics.commands_executed += 1;
        session.last_seen = chrono::Utc::now();
    }

    // Execute command via FeroxBridge (REAL execution)
    match bridge.execute_command(session_id, command_str.clone()).await {
        Ok((_bridge_command_id, output)) => {
            info!(session_id = %session_id, command = %command_str, "Command executed successfully via bridge");

            // Broadcast output to all WebSocket clients
            state
                .broadcast_command_output(command_id, session_id, output, true, Some(true))
                .await;
        }
        Err(e) => {
            // If bridge execution fails, send error to client
            error!(session_id = %session_id, error = %e, "Failed to execute command via bridge");
            let _ = state.event_tx.send(ServerEvent::CommandOutput {
                command_id,
                session_id,
                output: format!("Error executing command: {}", e),
                is_complete: true,
                success: Some(false),
            });
        }
    }
}

// Note: Command simulation has been moved to FeroxBridge
// The WebSocket handler now uses bridge.execute_command() for all command execution
