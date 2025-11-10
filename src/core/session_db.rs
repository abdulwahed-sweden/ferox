use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, params};
use serde_json;
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::core::module::{Platform, Session};

/// Command history entry for a session
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandHistory {
    pub id: i64,
    pub session_id: Uuid,
    pub command: String,
    pub output: String,
    pub executed_at: DateTime<Utc>,
}

/// Session database for persistence
pub struct SessionDB {
    conn: Arc<Mutex<Connection>>,
}

impl SessionDB {
    /// Create a new session database
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path).with_context(|| "Failed to open session database")?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        db.init_schema()?;
        Ok(db)
    }

    /// Create an in-memory database (for testing)
    pub fn new_in_memory() -> Result<Self> {
        let conn =
            Connection::open_in_memory().with_context(|| "Failed to create in-memory database")?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        db.init_schema()?;
        Ok(db)
    }

    /// Initialize database schema
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                module TEXT NOT NULL,
                target TEXT NOT NULL,
                platform TEXT NOT NULL,
                user TEXT,
                established_at TEXT NOT NULL,
                last_seen TEXT NOT NULL,
                active INTEGER NOT NULL,
                metadata TEXT NOT NULL
            )",
            [],
        )
        .with_context(|| "Failed to create sessions table")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS session_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                command TEXT NOT NULL,
                output TEXT NOT NULL,
                executed_at TEXT NOT NULL,
                FOREIGN KEY(session_id) REFERENCES sessions(id) ON DELETE CASCADE
            )",
            [],
        )
        .with_context(|| "Failed to create session_history table")?;

        // Create index for faster queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_session_history_session_id
             ON session_history(session_id)",
            [],
        )
        .ok();

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_sessions_active
             ON sessions(active)",
            [],
        )
        .ok();

        Ok(())
    }

    /// Save a session to the database
    pub fn save_session(&self, session: &Session) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        let platform_str = match session.platform {
            Platform::Linux => "Linux",
            Platform::Windows => "Windows",
            Platform::MacOS => "MacOS",
            Platform::Any => "Any",
        };

        let metadata_json = serde_json::to_string(&session.metadata)
            .with_context(|| "Failed to serialize metadata")?;

        conn.execute(
            "INSERT OR REPLACE INTO sessions
             (id, module, target, platform, user, established_at, last_seen, active, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                session.id.to_string(),
                &session.module,
                &session.target,
                platform_str,
                &session.user,
                session.established_at.to_rfc3339(),
                session.last_seen.to_rfc3339(),
                if session.active { 1 } else { 0 },
                metadata_json,
            ],
        )
        .with_context(|| format!("Failed to save session {}", session.id))?;

        Ok(())
    }

    /// Load a session from the database
    pub fn load_session(&self, session_id: Uuid) -> Result<Option<Session>> {
        let conn = self.conn.lock().unwrap();

        let result = conn
            .query_row(
                "SELECT id, module, target, platform, user, established_at, last_seen, active, metadata
                 FROM sessions WHERE id = ?1",
                params![session_id.to_string()],
                |row| {
                    let id_str: String = row.get(0)?;
                    let module: String = row.get(1)?;
                    let target: String = row.get(2)?;
                    let platform_str: String = row.get(3)?;
                    let user: Option<String> = row.get(4)?;
                    let established_at_str: String = row.get(5)?;
                    let last_seen_str: String = row.get(6)?;
                    let active: i32 = row.get(7)?;
                    let metadata_json: String = row.get(8)?;

                    let id = Uuid::parse_str(&id_str).map_err(|e| {
                        rusqlite::Error::ToSqlConversionFailure(Box::new(e))
                    })?;

                    let platform = match platform_str.as_str() {
                        "Linux" => Platform::Linux,
                        "Windows" => Platform::Windows,
                        "MacOS" => Platform::MacOS,
                        _ => Platform::Any,
                    };

                    let established_at = DateTime::parse_from_rfc3339(&established_at_str)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?
                        .with_timezone(&Utc);

                    let last_seen = DateTime::parse_from_rfc3339(&last_seen_str)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?
                        .with_timezone(&Utc);

                    let metadata: std::collections::HashMap<String, serde_json::Value> =
                        serde_json::from_str(&metadata_json)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                    Ok(Session {
                        id,
                        module,
                        target,
                        platform,
                        user,
                        established_at,
                        last_seen,
                        active: active != 0,
                        metadata,
                    })
                },
            )
            .optional()
            .with_context(|| format!("Failed to load session {}", session_id))?;

        Ok(result)
    }

    /// Load all sessions from the database
    pub fn load_all_sessions(&self) -> Result<Vec<Session>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT id, module, target, platform, user, established_at, last_seen, active, metadata
                 FROM sessions ORDER BY established_at DESC",
            )
            .with_context(|| "Failed to prepare query")?;

        let sessions = stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                let module: String = row.get(1)?;
                let target: String = row.get(2)?;
                let platform_str: String = row.get(3)?;
                let user: Option<String> = row.get(4)?;
                let established_at_str: String = row.get(5)?;
                let last_seen_str: String = row.get(6)?;
                let active: i32 = row.get(7)?;
                let metadata_json: String = row.get(8)?;

                let id = Uuid::parse_str(&id_str)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let platform = match platform_str.as_str() {
                    "Linux" => Platform::Linux,
                    "Windows" => Platform::Windows,
                    "MacOS" => Platform::MacOS,
                    _ => Platform::Any,
                };

                let established_at = DateTime::parse_from_rfc3339(&established_at_str)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?
                    .with_timezone(&Utc);

                let last_seen = DateTime::parse_from_rfc3339(&last_seen_str)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?
                    .with_timezone(&Utc);

                let metadata: std::collections::HashMap<String, serde_json::Value> =
                    serde_json::from_str(&metadata_json)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                Ok(Session {
                    id,
                    module,
                    target,
                    platform,
                    user,
                    established_at,
                    last_seen,
                    active: active != 0,
                    metadata,
                })
            })
            .with_context(|| "Failed to query sessions")?
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| "Failed to collect sessions")?;

        Ok(sessions)
    }

    /// Load active sessions only
    pub fn load_active_sessions(&self) -> Result<Vec<Session>> {
        let all_sessions = self.load_all_sessions()?;
        Ok(all_sessions.into_iter().filter(|s| s.active).collect())
    }

    /// Delete a session from the database
    pub fn delete_session(&self, session_id: Uuid) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "DELETE FROM sessions WHERE id = ?1",
            params![session_id.to_string()],
        )
        .with_context(|| format!("Failed to delete session {}", session_id))?;

        Ok(())
    }

    /// Update session last_seen timestamp
    pub fn update_heartbeat(&self, session_id: Uuid, last_seen: DateTime<Utc>) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "UPDATE sessions SET last_seen = ?1 WHERE id = ?2",
            params![last_seen.to_rfc3339(), session_id.to_string()],
        )
        .with_context(|| format!("Failed to update heartbeat for session {}", session_id))?;

        Ok(())
    }

    /// Mark session as inactive
    pub fn mark_inactive(&self, session_id: Uuid) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "UPDATE sessions SET active = 0 WHERE id = ?1",
            params![session_id.to_string()],
        )
        .with_context(|| format!("Failed to mark session {} as inactive", session_id))?;

        Ok(())
    }

    /// Save command history
    pub fn save_command(&self, session_id: Uuid, command: &str, output: &str) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

        let executed_at = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO session_history (session_id, command, output, executed_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![session_id.to_string(), command, output, executed_at],
        )
        .with_context(|| "Failed to save command history")?;

        Ok(conn.last_insert_rowid())
    }

    /// Load command history for a session
    pub fn load_history(&self, session_id: Uuid) -> Result<Vec<CommandHistory>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT id, session_id, command, output, executed_at
                 FROM session_history
                 WHERE session_id = ?1
                 ORDER BY executed_at ASC",
            )
            .with_context(|| "Failed to prepare history query")?;

        let history = stmt
            .query_map(params![session_id.to_string()], |row| {
                let id: i64 = row.get(0)?;
                let session_id_str: String = row.get(1)?;
                let command: String = row.get(2)?;
                let output: String = row.get(3)?;
                let executed_at_str: String = row.get(4)?;

                let session_id = Uuid::parse_str(&session_id_str)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let executed_at = DateTime::parse_from_rfc3339(&executed_at_str)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?
                    .with_timezone(&Utc);

                Ok(CommandHistory {
                    id,
                    session_id,
                    command,
                    output,
                    executed_at,
                })
            })
            .with_context(|| "Failed to query history")?
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| "Failed to collect history")?;

        Ok(history)
    }

    /// Clear all history for a session
    pub fn clear_history(&self, session_id: Uuid) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "DELETE FROM session_history WHERE session_id = ?1",
            params![session_id.to_string()],
        )
        .with_context(|| format!("Failed to clear history for session {}", session_id))?;

        Ok(())
    }

    /// Clean up stale sessions (older than specified hours)
    pub fn cleanup_stale(&self, hours: i64) -> Result<usize> {
        let conn = self.conn.lock().unwrap();

        let cutoff = Utc::now() - chrono::Duration::hours(hours);
        let cutoff_str = cutoff.to_rfc3339();

        let count = conn
            .execute(
                "DELETE FROM sessions WHERE last_seen < ?1",
                params![cutoff_str],
            )
            .with_context(|| "Failed to cleanup stale sessions")?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_session() -> Session {
        Session {
            id: Uuid::new_v4(),
            module: "test/module".to_string(),
            target: "192.168.1.1".to_string(),
            platform: Platform::Linux,
            user: Some("root".to_string()),
            established_at: Utc::now(),
            last_seen: Utc::now(),
            active: true,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_create_db() {
        let db = SessionDB::new_in_memory();
        assert!(db.is_ok());
    }

    #[test]
    fn test_save_and_load_session() {
        let db = SessionDB::new_in_memory().unwrap();
        let session = create_test_session();
        let session_id = session.id;

        // Save session
        db.save_session(&session).unwrap();

        // Load session
        let loaded = db.load_session(session_id).unwrap();
        assert!(loaded.is_some());

        let loaded_session = loaded.unwrap();
        assert_eq!(loaded_session.id, session_id);
        assert_eq!(loaded_session.module, "test/module");
        assert_eq!(loaded_session.target, "192.168.1.1");
        assert!(loaded_session.active);
    }

    #[test]
    fn test_load_all_sessions() {
        let db = SessionDB::new_in_memory().unwrap();

        // Create and save multiple sessions
        let session1 = create_test_session();
        let session2 = create_test_session();

        db.save_session(&session1).unwrap();
        db.save_session(&session2).unwrap();

        // Load all
        let sessions = db.load_all_sessions().unwrap();
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn test_delete_session() {
        let db = SessionDB::new_in_memory().unwrap();
        let session = create_test_session();
        let session_id = session.id;

        db.save_session(&session).unwrap();
        db.delete_session(session_id).unwrap();

        let loaded = db.load_session(session_id).unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_command_history() {
        let db = SessionDB::new_in_memory().unwrap();
        let session = create_test_session();
        let session_id = session.id;

        db.save_session(&session).unwrap();

        // Save commands
        db.save_command(session_id, "whoami", "root").unwrap();
        db.save_command(session_id, "uname -a", "Linux...").unwrap();

        // Load history
        let history = db.load_history(session_id).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].command, "whoami");
        assert_eq!(history[1].command, "uname -a");
    }

    #[test]
    fn test_heartbeat() {
        let db = SessionDB::new_in_memory().unwrap();
        let session = create_test_session();
        let session_id = session.id;

        db.save_session(&session).unwrap();

        let new_time = Utc::now();
        db.update_heartbeat(session_id, new_time).unwrap();

        let loaded = db.load_session(session_id).unwrap().unwrap();
        // Timestamps should be close (within a second)
        assert!((loaded.last_seen - new_time).num_seconds().abs() < 1);
    }
}
