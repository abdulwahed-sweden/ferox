//! Terminal Management Module
//!
//! Command-based terminal implementation for C2 session interaction.
//! This provides a simpler, thread-safe approach suitable for remote command execution.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Terminal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub rows: u16,
    pub cols: u16,
    pub shell: Option<String>,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            rows: 24,
            cols: 80,
            shell: None,
        }
    }
}

/// Command history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub command: String,
    pub output: String,
    pub timestamp: i64,
    pub success: bool,
}

/// Terminal instance state (simplified, thread-safe)
#[derive(Debug, Clone)]
pub struct TerminalInstance {
    pub id: String,
    pub session_id: String,
    pub history: Vec<HistoryEntry>,
    pub config: TerminalConfig,
}

/// Terminal Manager for managing command-based terminals
pub struct TerminalManager {
    terminals: RwLock<HashMap<String, TerminalInstance>>,
}

impl TerminalManager {
    pub fn new() -> Self {
        Self {
            terminals: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new terminal for a session
    pub fn create(&self, session_id: &str, config: TerminalConfig) -> Result<String, String> {
        let terminal_id = Uuid::new_v4().to_string();

        let instance = TerminalInstance {
            id: terminal_id.clone(),
            session_id: session_id.to_string(),
            history: Vec::new(),
            config,
        };

        self.terminals.write().insert(terminal_id.clone(), instance);

        tracing::info!("Created terminal {} for session {}", terminal_id, session_id);
        Ok(terminal_id)
    }

    /// Get terminal instance
    pub fn get(&self, terminal_id: &str) -> Option<TerminalInstance> {
        self.terminals.read().get(terminal_id).cloned()
    }

    /// Close terminal
    pub fn close(&self, terminal_id: &str) -> Result<(), String> {
        self.terminals.write().remove(terminal_id);
        tracing::info!("Closed terminal {}", terminal_id);
        Ok(())
    }

    /// Get terminal history
    pub fn get_history(&self, terminal_id: &str) -> Result<Vec<HistoryEntry>, String> {
        let terminals = self.terminals.read();
        let terminal = terminals.get(terminal_id).ok_or("Terminal not found")?;
        Ok(terminal.history.clone())
    }

    /// Add history entry
    pub fn add_history(
        &self,
        terminal_id: &str,
        command: String,
        output: String,
        success: bool,
    ) -> Result<(), String> {
        let mut terminals = self.terminals.write();
        let terminal = terminals
            .get_mut(terminal_id)
            .ok_or("Terminal not found")?;

        terminal.history.push(HistoryEntry {
            command,
            output,
            timestamp: chrono::Utc::now().timestamp(),
            success,
        });

        Ok(())
    }

    /// Get all terminal IDs for a session
    pub fn get_terminals_for_session(&self, session_id: &str) -> Vec<String> {
        self.terminals
            .read()
            .iter()
            .filter_map(|(id, term)| {
                if term.session_id == session_id {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self::new()
    }
}

// Note: Real PTY terminal support can be added later using tokio tasks
// that communicate via channels, avoiding the Sync requirement on PtyPair.
