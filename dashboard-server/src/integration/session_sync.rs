//! Session Synchronizer - Keeps Dashboard state in sync with Ferox Core
//!
//! Periodically syncs sessions from FeroxBridge to AppState.

use crate::integration::FeroxBridge;
use crate::state::AppState;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{debug, error, info};

/// Synchronizes sessions between FeroxBridge and AppState
pub struct SessionSynchronizer {
    bridge: Arc<FeroxBridge>,
    state: Arc<AppState>,
    sync_interval: Duration,
}

impl SessionSynchronizer {
    /// Create a new SessionSynchronizer
    pub fn new(bridge: Arc<FeroxBridge>, state: Arc<AppState>) -> Self {
        Self {
            bridge,
            state,
            sync_interval: Duration::from_secs(5),
        }
    }

    /// Set custom sync interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.sync_interval = interval;
        self
    }

    /// Start the synchronization loop
    pub async fn start(self) {
        info!(
            "Starting session synchronizer with {}s interval",
            self.sync_interval.as_secs()
        );

        let mut interval = interval(self.sync_interval);

        loop {
            interval.tick().await;

            if let Err(e) = self.sync_sessions().await {
                error!("Session sync error: {}", e);
            }
        }
    }

    /// Perform a single sync operation
    async fn sync_sessions(&self) -> anyhow::Result<()> {
        // Get all sessions from Ferox core
        let ferox_sessions = self.bridge.list_sessions().await;

        debug!("Syncing {} sessions from Ferox core", ferox_sessions.len());

        // Convert and update each session
        for session in ferox_sessions {
            let dashboard_session = FeroxBridge::to_dashboard_session(&session);

            // Check if session already exists
            let existing = self.state.get_session(session.id).await;

            if existing.is_none() {
                // New session - add it
                self.state.add_session(dashboard_session.clone()).await;
                info!("Added new session: {} ({})", dashboard_session.hostname, session.id);

                // Broadcast session created event
                let _ = self.state.event_tx.send(crate::types::ServerEvent::SessionCreated(
                    dashboard_session,
                ));
            } else {
                // Existing session - update it
                self.state.update_session(dashboard_session.clone()).await;
            }
        }

        Ok(())
    }
}

/// Start the session synchronizer as a background task
pub fn spawn_session_sync(bridge: Arc<FeroxBridge>, state: Arc<AppState>) {
    let synchronizer = SessionSynchronizer::new(bridge, state);

    tokio::spawn(async move {
        synchronizer.start().await;
    });
}
