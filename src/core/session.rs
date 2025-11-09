use crate::core::module::Session;
use crate::core::session_db::{CommandHistory, SessionDB};
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Session manager for handling active sessions with persistence
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<Uuid, Session>>>,
    db: Option<Arc<SessionDB>>,
}

impl SessionManager {
    /// Create a new session manager without persistence
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            db: None,
        }
    }

    /// Create a new session manager with database persistence
    pub fn with_db<P: Into<PathBuf>>(db_path: P) -> Result<Self> {
        let db = SessionDB::new(db_path.into())
            .with_context(|| "Failed to initialize session database")?;

        let mut manager = Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            db: Some(Arc::new(db)),
        };

        // Load existing sessions from database
        manager.load_from_db_sync()?;

        Ok(manager)
    }

    /// Load sessions from database (synchronous version for initialization)
    fn load_from_db_sync(&mut self) -> Result<()> {
        if let Some(db) = &self.db {
            let loaded_sessions = db
                .load_all_sessions()
                .with_context(|| "Failed to load sessions from database")?;

            let mut sessions = self.sessions.blocking_lock();
            for session in loaded_sessions {
                sessions.insert(session.id, session);
            }
        }
        Ok(())
    }

    /// Add a new session
    pub async fn add(&self, session: Session) -> Uuid {
        let id = session.id;

        // Save to database if available
        if let Some(db) = &self.db {
            if let Err(e) = db.save_session(&session) {
                eprintln!("Warning: Failed to save session to database: {}", e);
            }
        }

        // Add to in-memory map
        let mut sessions = self.sessions.lock().await;
        sessions.insert(id, session);

        id
    }

    /// Get a session by ID
    pub async fn get(&self, id: Uuid) -> Option<Session> {
        let sessions = self.sessions.lock().await;
        sessions.get(&id).cloned()
    }

    /// List all active sessions
    pub async fn list_active(&self) -> Vec<Session> {
        let sessions = self.sessions.lock().await;
        sessions.values().filter(|s| s.active).cloned().collect()
    }

    /// List all sessions
    pub async fn list_all(&self) -> Vec<Session> {
        let sessions = self.sessions.lock().await;
        sessions.values().cloned().collect()
    }

    /// Update session last_seen time (heartbeat)
    pub async fn heartbeat(&self, id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(&id) {
            let now = chrono::Utc::now();
            session.last_seen = now;

            // Update in database
            if let Some(db) = &self.db {
                db.update_heartbeat(id, now)
                    .with_context(|| "Failed to update heartbeat in database")?;
            }

            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", id))
        }
    }

    /// Kill a session (mark as inactive)
    pub async fn kill(&self, id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(&id) {
            session.active = false;

            // Update in database
            if let Some(db) = &self.db {
                db.mark_inactive(id)
                    .with_context(|| "Failed to mark session inactive in database")?;
            }

            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", id))
        }
    }

    /// Kill all sessions
    pub async fn kill_all(&self) -> Result<usize> {
        let mut sessions = self.sessions.lock().await;
        let mut count = 0;

        for session in sessions.values_mut() {
            if session.active {
                session.active = false;
                count += 1;

                // Update in database
                if let Some(db) = &self.db {
                    db.mark_inactive(session.id).ok();
                }
            }
        }

        Ok(count)
    }

    /// Remove a session completely
    pub async fn remove(&self, id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        sessions
            .remove(&id)
            .ok_or_else(|| anyhow!("Session not found: {}", id))?;

        // Remove from database
        if let Some(db) = &self.db {
            db.delete_session(id)
                .with_context(|| "Failed to delete session from database")?;
        }

        Ok(())
    }

    /// Get session count
    pub async fn count(&self) -> usize {
        let sessions = self.sessions.lock().await;
        sessions.len()
    }

    /// Get active session count
    pub async fn active_count(&self) -> usize {
        let sessions = self.sessions.lock().await;
        sessions.values().filter(|s| s.active).count()
    }

    /// Clean up inactive sessions older than threshold
    pub async fn cleanup_stale(&self, max_age_hours: i64) -> usize {
        let mut sessions = self.sessions.lock().await;
        let threshold = chrono::Utc::now() - chrono::Duration::hours(max_age_hours);

        let stale_ids: Vec<Uuid> = sessions
            .values()
            .filter(|s| !s.active && s.last_seen < threshold)
            .map(|s| s.id)
            .collect();

        let count = stale_ids.len();
        for id in stale_ids {
            sessions.remove(&id);

            // Remove from database
            if let Some(db) = &self.db {
                db.delete_session(id).ok();
            }
        }

        count
    }

    /// Execute a command on a session and store in history
    pub async fn execute_command(&self, session_id: Uuid, command: &str) -> Result<String> {
        // Update heartbeat
        self.heartbeat(session_id).await?;

        // For now, return a placeholder
        // In a real implementation, this would communicate with the actual session
        let output = format!("Command '{}' executed on session {}", command, session_id);

        // Save command to history
        if let Some(db) = &self.db {
            db.save_command(session_id, command, &output)
                .with_context(|| "Failed to save command to history")?;
        }

        Ok(output)
    }

    /// Get command history for a session
    pub async fn get_history(&self, session_id: Uuid) -> Result<Vec<CommandHistory>> {
        if let Some(db) = &self.db {
            db.load_history(session_id)
                .with_context(|| "Failed to load command history")
        } else {
            Ok(Vec::new())
        }
    }

    /// Clear command history for a session
    pub async fn clear_history(&self, session_id: Uuid) -> Result<()> {
        if let Some(db) = &self.db {
            db.clear_history(session_id)
                .with_context(|| "Failed to clear command history")
        } else {
            Ok(())
        }
    }

    /// Update session metadata
    pub async fn update_metadata(
        &self,
        session_id: Uuid,
        key: String,
        value: serde_json::Value,
    ) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.metadata.insert(key, value);

            // Save to database
            if let Some(db) = &self.db {
                db.save_session(session)
                    .with_context(|| "Failed to update session metadata in database")?;
            }

            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::module::Platform;

    #[tokio::test]
    async fn test_session_manager() {
        let manager = SessionManager::new();

        let session = Session::new(
            "test/module".to_string(),
            "127.0.0.1".to_string(),
            Platform::Linux,
        );
        let id = session.id;

        manager.add(session).await;
        assert_eq!(manager.count().await, 1);

        let retrieved = manager.get(id).await;
        assert!(retrieved.is_some());

        manager.kill(id).await.unwrap();
        assert_eq!(manager.active_count().await, 0);
    }

    #[tokio::test]
    async fn test_session_manager_with_db() {
        let manager = SessionManager::with_db(":memory:").unwrap();

        let session = Session::new(
            "test/module".to_string(),
            "127.0.0.1".to_string(),
            Platform::Linux,
        );
        let id = session.id;

        manager.add(session).await;
        assert_eq!(manager.count().await, 1);

        // Test persistence by creating a new manager with the same database
        // (Note: This won't work with :memory: but demonstrates the pattern)
        let retrieved = manager.get(id).await;
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_kill_all_sessions() {
        let manager = SessionManager::new();

        let session1 = Session::new(
            "test/module1".to_string(),
            "127.0.0.1".to_string(),
            Platform::Linux,
        );
        let session2 = Session::new(
            "test/module2".to_string(),
            "127.0.0.2".to_string(),
            Platform::Windows,
        );

        manager.add(session1).await;
        manager.add(session2).await;

        assert_eq!(manager.active_count().await, 2);

        let killed = manager.kill_all().await.unwrap();
        assert_eq!(killed, 2);
        assert_eq!(manager.active_count().await, 0);
    }

    #[tokio::test]
    async fn test_execute_command() {
        let manager = SessionManager::with_db(":memory:").unwrap();

        let session = Session::new(
            "test/module".to_string(),
            "127.0.0.1".to_string(),
            Platform::Linux,
        );
        let id = session.id;

        manager.add(session).await;

        let output = manager.execute_command(id, "whoami").await.unwrap();
        assert!(output.contains("whoami"));

        let history = manager.get_history(id).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].command, "whoami");
    }
}
