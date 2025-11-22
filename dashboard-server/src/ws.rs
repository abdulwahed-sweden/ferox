//! WebSocket handler for real-time dashboard communication
//!
//! Handles WebSocket connections, message routing, and event broadcasting.

use crate::state::{AppState, WsClient};
use crate::types::*;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handle an individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
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
    let state_clone = state.clone();
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
    let recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    handle_client_message(&state_clone2, client_id, &text).await;
                }
                Ok(Message::Ping(data)) => {
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
async fn handle_client_message(state: &Arc<AppState>, client_id: Uuid, text: &str) {
    match serde_json::from_str::<ClientEvent>(text) {
        Ok(event) => {
            debug!(client_id = %client_id, event = ?event, "Received client event");

            match event {
                ClientEvent::ExecuteCommand { session_id, command } => {
                    handle_execute_command(state, session_id, command).await;
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
async fn handle_execute_command(state: &Arc<AppState>, session_id: Uuid, command_str: String) {
    info!(session_id = %session_id, command = %command_str, "Executing command");

    // Verify session exists
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
    if let Some(mut session) = state.sessions.write().await.get_mut(&session_id) {
        session.metrics.commands_executed += 1;
        session.last_seen = chrono::Utc::now();
    }

    // Simulate command execution (in production, this would connect to actual agent)
    let output = simulate_command_output(&command_str).await;

    // Broadcast output
    state
        .broadcast_command_output(command_id, session_id, output, true, Some(true))
        .await;
}

/// Simulate command output for demo purposes
async fn simulate_command_output(command: &str) -> String {
    // Add realistic delay
    tokio::time::sleep(tokio::time::Duration::from_millis(100 + rand_delay())).await;

    match command.to_lowercase().as_str() {
        "whoami" => "NT AUTHORITY\\SYSTEM".to_string(),
        "hostname" => "DC01".to_string(),
        "ipconfig" | "ifconfig" => {
            r#"
Windows IP Configuration

Ethernet adapter Ethernet0:
   Connection-specific DNS Suffix  . : corp.local
   IPv4 Address. . . . . . . . . . . : 192.168.1.10
   Subnet Mask . . . . . . . . . . . : 255.255.255.0
   Default Gateway . . . . . . . . . : 192.168.1.1
"#
            .to_string()
        }
        "net user" => {
            r#"
User accounts for \\DC01

-------------------------------------------------------------------------------
Administrator            Guest                    krbtgt
john.doe                 jane.smith               svc_backup
svc_sql                  DefaultAccount
The command completed successfully.
"#
            .to_string()
        }
        cmd if cmd.starts_with("dir") || cmd.starts_with("ls") => {
            r#"
 Volume in drive C is System
 Volume Serial Number is 1234-5678

 Directory of C:\Windows\System32

11/22/2024  10:30 AM    <DIR>          .
11/22/2024  10:30 AM    <DIR>          ..
11/22/2024  10:30 AM           153,600 cmd.exe
11/22/2024  10:30 AM           384,512 notepad.exe
11/22/2024  10:30 AM         1,048,576 powershell.exe
               3 File(s)      1,586,688 bytes
"#
            .to_string()
        }
        _ => format!("Executed: {}\n[Command completed successfully]", command),
    }
}

/// Generate random delay for realistic simulation
fn rand_delay() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % 400) as u64
}
