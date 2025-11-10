use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Module types in Ferox
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleType {
    Scanner,
    Exploit,
    PostExploit,
    Auxiliary,
    Payload,
    Encoder,
    Handler,
}

/// Target platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    Linux,
    Windows,
    MacOS,
    Any,
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub module: String,
    pub target: String,
    pub platform: Platform,
    pub user: Option<String>,
    pub established_at: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub active: bool,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Session {
    #[allow(dead_code)]
    pub fn new(module: String, target: String, platform: Platform) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            module,
            target,
            platform,
            user: None,
            established_at: now,
            last_seen: now,
            active: true,
            metadata: HashMap::new(),
        }
    }
}

/// Module metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub module_type: ModuleType,
    pub category: String,
}

/// Module options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleOption {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub current_value: Option<String>,
}

/// Module execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleResult {
    pub success: bool,
    pub message: String,
    pub data: HashMap<String, serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub session_id: Option<Uuid>,
}

impl ModuleResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: HashMap::new(),
            timestamp: chrono::Utc::now(),
            session_id: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: HashMap::new(),
            timestamp: chrono::Utc::now(),
            session_id: None,
        }
    }

    pub fn with_data(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }

    #[allow(dead_code)]
    pub fn with_session(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }
}

/// Check result for non-destructive fingerprinting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub vulnerable: bool,
    pub confidence: f32, // 0.0 to 1.0
    pub details: String,
    pub fingerprint: HashMap<String, String>,
}

/// Core trait that all Ferox modules must implement
#[async_trait]
pub trait Module: Send + Sync {
    /// Get module information
    fn info(&self) -> ModuleInfo;

    /// Get module options
    fn options(&self) -> Vec<ModuleOption>;

    /// Set an option value
    fn set_option(&mut self, name: &str, value: &str) -> Result<()>;

    /// Get an option value
    fn get_option(&self, name: &str) -> Option<String>;

    /// Validate options before execution
    fn validate(&self) -> Result<()>;

    /// Non-destructive check (safe fingerprinting)
    /// Default implementation returns "not implemented"
    async fn check(&self) -> Result<CheckResult> {
        Ok(CheckResult {
            vulnerable: false,
            confidence: 0.0,
            details: "Check not implemented for this module".to_string(),
            fingerprint: HashMap::new(),
        })
    }

    /// Execute the module
    async fn run(&mut self) -> Result<ModuleResult>;

    /// Clean up after execution
    #[allow(dead_code)]
    async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Requires explicit user confirmation (for destructive operations)
    #[allow(dead_code)]
    fn requires_confirmation(&self) -> bool {
        matches!(self.info().module_type, ModuleType::Exploit)
    }
}

/// Ferox module registry
pub struct ModuleRegistry {
    modules: HashMap<String, Box<dyn Module>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register(&mut self, module: Box<dyn Module>) {
        let info = module.info();
        let key = format!("{}/{}", info.category, info.name);
        self.modules.insert(key, module);
    }

    #[allow(clippy::borrowed_box)]
    pub fn get(&self, path: &str) -> Option<&Box<dyn Module>> {
        self.modules.get(path)
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_mut(&mut self, path: &str) -> Option<&mut Box<dyn Module>> {
        self.modules.get_mut(path)
    }

    pub fn list(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }

    pub fn list_by_type(&self, module_type: ModuleType) -> Vec<String> {
        self.modules
            .iter()
            .filter(|(_, module)| module.info().module_type == module_type)
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn count(&self) -> usize {
        self.modules.len()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
