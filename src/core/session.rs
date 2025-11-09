use crate::core::module::Session;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Session manager for handling active sessions
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<Uuid, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a new session
    #[allow(dead_code)]
    pub async fn add(&self, session: Session) -> Uuid {
        let id = session.id;
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

    /// Update session last_seen time
    pub async fn heartbeat(&self, id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(&id) {
            session.last_seen = chrono::Utc::now();
            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", id))
        }
    }

    /// Kill a session
    pub async fn kill(&self, id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(&id) {
            session.active = false;
            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", id))
        }
    }

    /// Remove a session completely
    pub async fn remove(&self, id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        sessions
            .remove(&id)
            .ok_or_else(|| anyhow!("Session not found: {}", id))?;
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
        }
        count
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
}
