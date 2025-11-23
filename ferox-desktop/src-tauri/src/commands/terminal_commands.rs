//! Terminal Commands
//!
//! Tauri commands for terminal operations.

use crate::security::Validator;
use crate::terminal::{HistoryEntry, TerminalConfig};
use crate::AppState;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

/// Terminal creation response
#[derive(Debug, Serialize)]
pub struct TerminalResponse {
    pub terminal_id: String,
    pub session_id: String,
}

/// Create terminal request
#[derive(Debug, Deserialize)]
pub struct CreateTerminalRequest {
    pub session_id: String,
    pub rows: Option<u16>,
    pub cols: Option<u16>,
    pub shell: Option<String>,
}

/// Resize terminal request (placeholder for future use)
#[derive(Debug, Deserialize)]
pub struct ResizeTerminalRequest {
    pub terminal_id: String,
    pub rows: u16,
    pub cols: u16,
}

/// Create a new terminal for a session
#[tauri::command]
pub async fn create_terminal(
    request: CreateTerminalRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<TerminalResponse, String> {
    // Validate input
    Validator::session_id(&request.session_id).map_err(|e| e.to_string())?;

    let config = TerminalConfig {
        rows: request.rows.unwrap_or(24),
        cols: request.cols.unwrap_or(80),
        shell: request.shell,
    };

    let state = state.read();
    let terminal_id = state
        .terminal_manager
        .create(&request.session_id, config)?;

    // Audit log terminal creation
    state.audit_logger.log_terminal_created(&terminal_id, &request.session_id);

    Ok(TerminalResponse {
        terminal_id,
        session_id: request.session_id,
    })
}

/// Write data to terminal (for future PTY support)
#[tauri::command]
pub async fn write_terminal(
    terminal_id: String,
    data: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<(), String> {
    // Validate inputs
    Validator::terminal_id(&terminal_id).map_err(|e| e.to_string())?;
    Validator::terminal_data(&data).map_err(|e| e.to_string())?;

    let state = state.read();
    let _terminal = state.terminal_manager.get(&terminal_id)
        .ok_or("Terminal not found")?;

    tracing::debug!("Write to terminal {}: {} bytes", terminal_id, data.len());
    Ok(())
}

/// Resize terminal (placeholder for future PTY support)
#[tauri::command]
pub async fn resize_terminal(
    request: ResizeTerminalRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<(), String> {
    // Validate input
    Validator::terminal_id(&request.terminal_id).map_err(|e| e.to_string())?;

    let state = state.read();
    let _terminal = state.terminal_manager.get(&request.terminal_id)
        .ok_or("Terminal not found")?;

    tracing::debug!("Resize terminal {} to {}x{}", request.terminal_id, request.cols, request.rows);
    Ok(())
}

/// Close terminal
#[tauri::command]
pub async fn close_terminal(
    terminal_id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<(), String> {
    // Validate input
    Validator::terminal_id(&terminal_id).map_err(|e| e.to_string())?;

    let state = state.read();

    // Audit log terminal closure
    state.audit_logger.log_terminal_closed(&terminal_id);

    state.terminal_manager.close(&terminal_id)
}

/// Get terminal command history
#[tauri::command]
pub async fn get_terminal_history(
    terminal_id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<HistoryEntry>, String> {
    // Validate input
    Validator::terminal_id(&terminal_id).map_err(|e| e.to_string())?;

    let state = state.read();
    state.terminal_manager.get_history(&terminal_id)
}

/// Terminal command execution request
#[derive(Debug, Deserialize)]
pub struct ExecuteTerminalCommandRequest {
    pub terminal_id: String,
    pub command: String,
}

/// Terminal command execution response
#[derive(Debug, Serialize)]
pub struct ExecuteTerminalCommandResponse {
    pub output: String,
    pub success: bool,
    pub execution_time_ms: u64,
}

/// Execute command in terminal (via Ferox Core bridge)
#[tauri::command]
pub async fn execute_terminal_command(
    request: ExecuteTerminalCommandRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<ExecuteTerminalCommandResponse, String> {
    // Validate inputs
    Validator::terminal_id(&request.terminal_id).map_err(|e| e.to_string())?;
    Validator::command(&request.command).map_err(|e| e.to_string())?;

    let start = std::time::Instant::now();

    // Get terminal and session info, then release the lock
    let (session_id, bridge, audit_logger) = {
        let state = state.read();
        let terminal = state
            .terminal_manager
            .get(&request.terminal_id)
            .ok_or("Terminal not found")?;
        (
            terminal.session_id.clone(),
            state.bridge.clone(),
            state.audit_logger.clone(),
        )
    };

    // Execute command via bridge (which calls Ferox Core)
    let result = bridge
        .execute_command(&session_id, &request.command)
        .await;

    let (output, success) = match result {
        Ok(cmd_output) => (cmd_output.stdout, cmd_output.exit_code == 0),
        Err(e) => (format!("Error: {}", e), false),
    };

    let execution_time_ms = start.elapsed().as_millis() as u64;

    // Store result in terminal history
    {
        let state = state.read();
        let _ = state.terminal_manager.add_history(
            &request.terminal_id,
            request.command.clone(),
            output.clone(),
            success,
        );
    }

    // Audit log the command execution
    audit_logger.log_command_executed(&session_id, &request.command, success);

    Ok(ExecuteTerminalCommandResponse {
        output,
        success,
        execution_time_ms,
    })
}
