pub mod file_ops;
pub mod security;
pub mod shell_local;
pub mod shell_remote;

pub use file_ops::FileOperationsHandler;
pub use shell_local::{CommandOutput, LocalShellHandler};
pub use shell_remote::{RemoteShellHandler, ShellType};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Handler type enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandlerType {
    LocalShell,
    RemoteShell,
    FileOperations,
}

/// Session handler that manages active handler instances
pub struct HandlerRegistry {
    local_shells: Arc<Mutex<HashMap<Uuid, LocalShellHandler>>>,
    remote_shells: Arc<Mutex<HashMap<Uuid, RemoteShellHandler>>>,
    file_ops: Arc<Mutex<HashMap<Uuid, FileOperationsHandler>>>,
}

impl HandlerRegistry {
    /// Create a new handler registry
    pub fn new() -> Self {
        Self {
            local_shells: Arc::new(Mutex::new(HashMap::new())),
            remote_shells: Arc::new(Mutex::new(HashMap::new())),
            file_ops: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a local shell handler
    pub async fn register_local_shell(&self, handler: LocalShellHandler) -> Uuid {
        let id = Uuid::new_v4();
        let mut shells = self.local_shells.lock().await;
        shells.insert(id, handler);
        id
    }

    /// Register a remote shell handler
    pub async fn register_remote_shell(&self, handler: RemoteShellHandler) -> Uuid {
        let id = Uuid::new_v4();
        let mut shells = self.remote_shells.lock().await;
        shells.insert(id, handler);
        id
    }

    /// Register a file operations handler
    pub async fn register_file_ops(&self, handler: FileOperationsHandler) -> Uuid {
        let id = Uuid::new_v4();
        let mut ops = self.file_ops.lock().await;
        ops.insert(id, handler);
        id
    }

    /// Execute a command on a local shell handler by ID
    pub async fn execute_local_command(
        &self,
        id: Uuid,
        command: &str,
    ) -> Option<Result<CommandOutput, anyhow::Error>> {
        let shells = self.local_shells.lock().await;
        if let Some(handler) = shells.get(&id) {
            Some(handler.execute(command).await)
        } else {
            None
        }
    }

    /// Get a file operations handler by ID (cloneable)
    pub async fn get_file_ops(&self, id: Uuid) -> Option<FileOperationsHandler> {
        let ops = self.file_ops.lock().await;
        ops.get(&id).cloned()
    }

    /// Check if a handler exists
    pub async fn has_handler(&self, id: Uuid, handler_type: HandlerType) -> bool {
        match handler_type {
            HandlerType::LocalShell => {
                let shells = self.local_shells.lock().await;
                shells.contains_key(&id)
            }
            HandlerType::RemoteShell => {
                let shells = self.remote_shells.lock().await;
                shells.contains_key(&id)
            }
            HandlerType::FileOperations => {
                let ops = self.file_ops.lock().await;
                ops.contains_key(&id)
            }
        }
    }

    /// Remove a handler by ID and type
    pub async fn remove_handler(&self, id: Uuid, handler_type: HandlerType) -> bool {
        match handler_type {
            HandlerType::LocalShell => {
                let mut shells = self.local_shells.lock().await;
                shells.remove(&id).is_some()
            }
            HandlerType::RemoteShell => {
                let mut shells = self.remote_shells.lock().await;
                shells.remove(&id).is_some()
            }
            HandlerType::FileOperations => {
                let mut ops = self.file_ops.lock().await;
                ops.remove(&id).is_some()
            }
        }
    }

    /// List all handler IDs by type
    pub async fn list_handlers(&self, handler_type: HandlerType) -> Vec<Uuid> {
        match handler_type {
            HandlerType::LocalShell => {
                let shells = self.local_shells.lock().await;
                shells.keys().copied().collect()
            }
            HandlerType::RemoteShell => {
                let shells = self.remote_shells.lock().await;
                shells.keys().copied().collect()
            }
            HandlerType::FileOperations => {
                let ops = self.file_ops.lock().await;
                ops.keys().copied().collect()
            }
        }
    }

    /// Get total number of registered handlers
    pub async fn count(&self) -> usize {
        let local = self.local_shells.lock().await.len();
        let remote = self.remote_shells.lock().await.len();
        let file_ops = self.file_ops.lock().await.len();
        local + remote + file_ops
    }

    /// Clear all handlers
    pub async fn clear(&self) {
        let mut local = self.local_shells.lock().await;
        let mut remote = self.remote_shells.lock().await;
        let mut file_ops = self.file_ops.lock().await;
        local.clear();
        remote.clear();
        file_ops.clear();
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Handler statistics
#[derive(Debug, Clone)]
pub struct HandlerStats {
    pub local_shells: usize,
    pub remote_shells: usize,
    pub file_operations: usize,
    pub total: usize,
}

impl HandlerRegistry {
    /// Get handler statistics
    pub async fn get_stats(&self) -> HandlerStats {
        let local_shells = self.local_shells.lock().await.len();
        let remote_shells = self.remote_shells.lock().await.len();
        let file_operations = self.file_ops.lock().await.len();

        HandlerStats {
            local_shells,
            remote_shells,
            file_operations,
            total: local_shells + remote_shells + file_operations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler_registry() {
        let registry = HandlerRegistry::new();

        let local_handler = LocalShellHandler::new();
        let id = registry.register_local_shell(local_handler).await;

        assert_eq!(registry.count().await, 1);

        let handlers = registry.list_handlers(HandlerType::LocalShell).await;
        assert_eq!(handlers.len(), 1);
        assert_eq!(handlers[0], id);

        let removed = registry.remove_handler(id, HandlerType::LocalShell).await;
        assert!(removed);
        assert_eq!(registry.count().await, 0);
    }

    #[tokio::test]
    async fn test_handler_stats() {
        let registry = HandlerRegistry::new();

        registry
            .register_local_shell(LocalShellHandler::new())
            .await;
        registry
            .register_remote_shell(RemoteShellHandler::new(
                ShellType::Reverse,
                "127.0.0.1".to_string(),
                4444,
            ))
            .await;
        registry
            .register_file_ops(FileOperationsHandler::new())
            .await;

        let stats = registry.get_stats().await;
        assert_eq!(stats.local_shells, 1);
        assert_eq!(stats.remote_shells, 1);
        assert_eq!(stats.file_operations, 1);
        assert_eq!(stats.total, 3);
    }
}
