use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRequirements {
    pub min_rust_version: String,
    pub required_python_version: String,
    pub min_ram_mb: u64,
    pub min_disk_space_mb: u64,
    pub required_tools: Vec<String>,
}

impl Default for SystemRequirements {
    fn default() -> Self {
        Self {
            min_rust_version: "1.70.0".to_string(),
            required_python_version: "3.8".to_string(),
            min_ram_mb: 2048,
            min_disk_space_mb: 512,
            required_tools: vec!["git".to_string(), "curl".to_string(), "python3".to_string()],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub check_name: String,
    pub status: CheckStatus,
    pub message: String,
    pub suggestion: Option<String>,
}

impl CheckResult {
    pub fn success(name: impl Into<String>, message: String) -> Self {
        Self {
            check_name: name.into(),
            status: CheckStatus::Success,
            message,
            suggestion: None,
        }
    }

    pub fn warning(name: impl Into<String>, message: String, suggestion: Option<String>) -> Self {
        Self {
            check_name: name.into(),
            status: CheckStatus::Warning,
            message,
            suggestion,
        }
    }

    pub fn error(name: impl Into<String>, message: String, suggestion: Option<String>) -> Self {
        Self {
            check_name: name.into(),
            status: CheckStatus::Error,
            message,
            suggestion,
        }
    }
}
