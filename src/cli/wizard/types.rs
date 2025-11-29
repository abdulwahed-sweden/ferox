//! Wizard Types and Enums

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Target type for the assessment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TargetType {
    /// Single IP address
    SingleHost,
    /// Domain name
    Domain,
    /// CIDR range (e.g., 192.168.1.0/24)
    IpRange,
    /// Multiple targets from file
    MultipleTargets,
}

impl std::fmt::Display for TargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetType::SingleHost => write!(f, "Single Host"),
            TargetType::Domain => write!(f, "Domain"),
            TargetType::IpRange => write!(f, "IP Range"),
            TargetType::MultipleTargets => write!(f, "Multiple Targets"),
        }
    }
}

/// Target configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    /// Target type
    pub target_type: TargetType,
    /// Primary target (IP, domain, CIDR, or file path)
    pub target: String,
    /// Additional resolved targets (for ranges/files)
    #[serde(default)]
    pub resolved_targets: Vec<String>,
    /// Authorization confirmed
    pub authorized: bool,
    /// Notes about the target
    #[serde(default)]
    pub notes: String,
}

impl Default for TargetConfig {
    fn default() -> Self {
        Self {
            target_type: TargetType::Domain,
            target: String::new(),
            resolved_targets: Vec::new(),
            authorized: false,
            notes: String::new(),
        }
    }
}

/// Assessment scope level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ScopeLevel {
    /// Passive reconnaissance only
    ReconOnly,
    /// Active discovery and enumeration
    Discovery,
    /// Full assessment including exploitation
    FullAssessment,
    /// Custom module selection
    Custom,
}

impl std::fmt::Display for ScopeLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeLevel::ReconOnly => write!(f, "Reconnaissance Only"),
            ScopeLevel::Discovery => write!(f, "Discovery & Enumeration"),
            ScopeLevel::FullAssessment => write!(f, "Full Assessment"),
            ScopeLevel::Custom => write!(f, "Custom"),
        }
    }
}

/// Intensity level for scanning
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum IntensityLevel {
    /// Slow, minimal footprint
    Stealth,
    /// Balanced speed and stealth
    Normal,
    /// Fast, higher detection risk
    Aggressive,
}

impl std::fmt::Display for IntensityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntensityLevel::Stealth => write!(f, "Stealth"),
            IntensityLevel::Normal => write!(f, "Normal"),
            IntensityLevel::Aggressive => write!(f, "Aggressive"),
        }
    }
}

impl IntensityLevel {
    /// Get recommended thread count
    pub fn threads(&self) -> usize {
        match self {
            IntensityLevel::Stealth => 3,
            IntensityLevel::Normal => 10,
            IntensityLevel::Aggressive => 50,
        }
    }

    /// Get recommended timeout in ms
    pub fn timeout_ms(&self) -> u64 {
        match self {
            IntensityLevel::Stealth => 5000,
            IntensityLevel::Normal => 3000,
            IntensityLevel::Aggressive => 1000,
        }
    }

    /// Get recommended delay between requests in ms
    pub fn delay_ms(&self) -> u64 {
        match self {
            IntensityLevel::Stealth => 500,
            IntensityLevel::Normal => 100,
            IntensityLevel::Aggressive => 0,
        }
    }
}

/// Scope configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeConfig {
    /// Scope level
    pub level: ScopeLevel,
    /// Intensity level
    pub intensity: IntensityLevel,
    /// Include post-exploitation phase
    pub include_post_exploit: bool,
}

impl Default for ScopeConfig {
    fn default() -> Self {
        Self {
            level: ScopeLevel::Discovery,
            intensity: IntensityLevel::Normal,
            include_post_exploit: false,
        }
    }
}

/// Selected module with configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedModule {
    /// Module path (e.g., "scanner/port_scanner")
    pub path: String,
    /// Module name for display
    pub name: String,
    /// Module options
    #[serde(default)]
    pub options: HashMap<String, String>,
    /// Whether module is enabled
    pub enabled: bool,
    /// Phase this module belongs to
    pub phase: usize,
}

impl SelectedModule {
    pub fn new(path: &str, name: &str, phase: usize) -> Self {
        Self {
            path: path.to_string(),
            name: name.to_string(),
            options: HashMap::new(),
            enabled: true,
            phase,
        }
    }

    pub fn with_option(mut self, key: &str, value: &str) -> Self {
        self.options.insert(key.to_string(), value.to_string());
        self
    }
}

/// Execution mode for the attack plan
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ExecutionMode {
    /// Run all phases automatically
    Automatic,
    /// Confirm each phase before execution
    StepByStep,
    /// Export plan without executing
    ExportOnly,
}

impl std::fmt::Display for ExecutionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionMode::Automatic => write!(f, "Automatic"),
            ExecutionMode::StepByStep => write!(f, "Step-by-Step"),
            ExecutionMode::ExportOnly => write!(f, "Export Only"),
        }
    }
}

/// Current wizard step
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WizardStep {
    Target,
    Scope,
    Modules,
    Config,
    Review,
    Execute,
}

impl WizardStep {
    pub fn number(&self) -> usize {
        match self {
            WizardStep::Target => 1,
            WizardStep::Scope => 2,
            WizardStep::Modules => 3,
            WizardStep::Config => 4,
            WizardStep::Review => 5,
            WizardStep::Execute => 6,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            WizardStep::Target => "TARGET DEFINITION",
            WizardStep::Scope => "ATTACK SCOPE",
            WizardStep::Modules => "MODULE SELECTION",
            WizardStep::Config => "CONFIGURATION",
            WizardStep::Review => "REVIEW & EXECUTE",
            WizardStep::Execute => "EXECUTION",
        }
    }

    pub fn total() -> usize {
        5 // Excluding Execute which is after confirmation
    }
}

/// Wizard state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardState {
    /// Target configuration
    pub target: TargetConfig,
    /// Scope configuration
    pub scope: ScopeConfig,
    /// Selected modules
    pub modules: Vec<SelectedModule>,
    /// Execution mode
    pub execution_mode: ExecutionMode,
    /// Plan name
    #[serde(default)]
    pub plan_name: String,
    /// Timestamp when created
    #[serde(default)]
    pub created_at: String,
    /// Whether plan was interrupted
    #[serde(default)]
    pub interrupted: bool,
    /// Last completed phase (for resume)
    #[serde(default)]
    pub last_completed_phase: Option<usize>,
}

impl Default for WizardState {
    fn default() -> Self {
        Self {
            target: TargetConfig::default(),
            scope: ScopeConfig::default(),
            modules: Vec::new(),
            execution_mode: ExecutionMode::StepByStep,
            plan_name: String::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            interrupted: false,
            last_completed_phase: None,
        }
    }
}

/// Module execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleExecutionResult {
    pub module_path: String,
    pub module_name: String,
    pub success: bool,
    pub message: String,
    pub data: HashMap<String, serde_json::Value>,
    pub duration_ms: u64,
    pub discoveries: Vec<String>,
}

/// Phase execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseExecutionResult {
    pub phase_name: String,
    pub phase_number: usize,
    pub modules: Vec<ModuleExecutionResult>,
    pub success_count: usize,
    pub total_count: usize,
    pub duration_ms: u64,
}

/// Complete execution report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub plan_name: String,
    pub target: String,
    pub phases: Vec<PhaseExecutionResult>,
    pub total_modules: usize,
    pub successful_modules: usize,
    pub failed_modules: usize,
    pub total_duration_ms: u64,
    pub started_at: String,
    pub completed_at: String,
}
