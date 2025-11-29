//! Progress Tracker - Core trait and types for progress reporting

use std::sync::Arc;
use tokio::sync::mpsc;

/// Progress update message
#[derive(Debug, Clone)]
pub enum ProgressUpdate {
    /// Started a new operation
    Started {
        operation: String,
        total: Option<u64>,
    },
    /// Progress increment
    Progress {
        current: u64,
        total: u64,
        message: Option<String>,
    },
    /// Discovery found during operation
    Discovery {
        item: String,
        details: Option<String>,
    },
    /// Status message update
    Status {
        message: String,
    },
    /// Stage transition in multi-stage operation
    Stage {
        stage: usize,
        total_stages: usize,
        name: String,
    },
    /// Operation completed
    Completed {
        message: String,
        success: bool,
    },
    /// Error occurred
    Error {
        message: String,
    },
}

/// Callback function type for progress updates
pub type ProgressCallback = Arc<dyn Fn(ProgressUpdate) + Send + Sync>;

/// Progress tracker trait for modules to implement
pub trait ProgressTracker: Send + Sync {
    /// Set the progress callback
    fn set_progress_callback(&mut self, callback: ProgressCallback);

    /// Report progress update
    fn report_progress(&self, update: ProgressUpdate);

    /// Check if progress reporting is enabled
    fn has_progress_callback(&self) -> bool;
}

/// Channel-based progress sender for async operations
#[derive(Clone)]
pub struct ProgressSender {
    tx: mpsc::UnboundedSender<ProgressUpdate>,
}

impl ProgressSender {
    pub fn new(tx: mpsc::UnboundedSender<ProgressUpdate>) -> Self {
        Self { tx }
    }

    pub fn start(&self, operation: &str, total: Option<u64>) {
        let _ = self.tx.send(ProgressUpdate::Started {
            operation: operation.to_string(),
            total,
        });
    }

    pub fn progress(&self, current: u64, total: u64, message: Option<&str>) {
        let _ = self.tx.send(ProgressUpdate::Progress {
            current,
            total,
            message: message.map(|s| s.to_string()),
        });
    }

    pub fn discovery(&self, item: &str, details: Option<&str>) {
        let _ = self.tx.send(ProgressUpdate::Discovery {
            item: item.to_string(),
            details: details.map(|s| s.to_string()),
        });
    }

    pub fn status(&self, message: &str) {
        let _ = self.tx.send(ProgressUpdate::Status {
            message: message.to_string(),
        });
    }

    pub fn stage(&self, stage: usize, total: usize, name: &str) {
        let _ = self.tx.send(ProgressUpdate::Stage {
            stage,
            total_stages: total,
            name: name.to_string(),
        });
    }

    pub fn complete(&self, message: &str, success: bool) {
        let _ = self.tx.send(ProgressUpdate::Completed {
            message: message.to_string(),
            success,
        });
    }

    pub fn error(&self, message: &str) {
        let _ = self.tx.send(ProgressUpdate::Error {
            message: message.to_string(),
        });
    }
}

/// Progress receiver for handling updates
pub struct ProgressReceiver {
    rx: mpsc::UnboundedReceiver<ProgressUpdate>,
}

impl ProgressReceiver {
    pub fn new(rx: mpsc::UnboundedReceiver<ProgressUpdate>) -> Self {
        Self { rx }
    }

    pub async fn recv(&mut self) -> Option<ProgressUpdate> {
        self.rx.recv().await
    }

    pub fn try_recv(&mut self) -> Option<ProgressUpdate> {
        self.rx.try_recv().ok()
    }
}

/// Create a progress channel pair
pub fn progress_channel() -> (ProgressSender, ProgressReceiver) {
    let (tx, rx) = mpsc::unbounded_channel();
    (ProgressSender::new(tx), ProgressReceiver::new(rx))
}
