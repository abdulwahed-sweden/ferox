//! Security Assessment Workflow Manager
//!
//! A guided workflow system for conducting authorized security assessments.
//! Focuses on reconnaissance and vulnerability discovery (Phases 1-2 only).
//! Does NOT include exploitation or post-exploitation capabilities.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

// ============================================================================
// WORKFLOW TYPES
// ============================================================================

/// Target type for assessment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentTargetType {
    /// Single IP address (e.g., 192.168.1.1)
    IpAddress,
    /// Domain name (e.g., example.com)
    Domain,
    /// URL (e.g., https://example.com)
    Url,
    /// CIDR range (e.g., 192.168.1.0/24)
    CidrRange,
    /// Multiple targets from list
    MultiTarget,
}

impl std::fmt::Display for AssessmentTargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssessmentTargetType::IpAddress => write!(f, "IP Address"),
            AssessmentTargetType::Domain => write!(f, "Domain"),
            AssessmentTargetType::Url => write!(f, "URL"),
            AssessmentTargetType::CidrRange => write!(f, "CIDR Range"),
            AssessmentTargetType::MultiTarget => write!(f, "Multiple Targets"),
        }
    }
}

/// Assessment scope - what depth of scanning to perform
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentScope {
    /// Passive reconnaissance only (no direct target contact)
    PassiveRecon,
    /// Active reconnaissance (DNS, WHOIS, etc.)
    ActiveRecon,
    /// Discovery scanning (port scanning, service detection)
    Discovery,
    /// Full reconnaissance + discovery
    Comprehensive,
}

impl std::fmt::Display for AssessmentScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssessmentScope::PassiveRecon => write!(f, "Passive Reconnaissance"),
            AssessmentScope::ActiveRecon => write!(f, "Active Reconnaissance"),
            AssessmentScope::Discovery => write!(f, "Discovery Scanning"),
            AssessmentScope::Comprehensive => write!(f, "Comprehensive Assessment"),
        }
    }
}

impl AssessmentScope {
    pub fn description(&self) -> &str {
        match self {
            AssessmentScope::PassiveRecon => {
                "Gather information without directly contacting the target"
            }
            AssessmentScope::ActiveRecon => {
                "DNS enumeration, WHOIS lookup, subdomain discovery"
            }
            AssessmentScope::Discovery => {
                "Port scanning, service detection, HTTP fingerprinting"
            }
            AssessmentScope::Comprehensive => {
                "Full reconnaissance and discovery workflow"
            }
        }
    }
}

/// Scan intensity level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanIntensity {
    /// Slow and quiet - minimal footprint
    Quiet,
    /// Balanced - moderate speed and footprint
    Normal,
    /// Fast - higher resource usage
    Aggressive,
}

impl ScanIntensity {
    pub fn threads(&self) -> usize {
        match self {
            ScanIntensity::Quiet => 3,
            ScanIntensity::Normal => 10,
            ScanIntensity::Aggressive => 50,
        }
    }

    pub fn timeout_ms(&self) -> u64 {
        match self {
            ScanIntensity::Quiet => 5000,
            ScanIntensity::Normal => 3000,
            ScanIntensity::Aggressive => 1000,
        }
    }

    pub fn delay_ms(&self) -> u64 {
        match self {
            ScanIntensity::Quiet => 500,
            ScanIntensity::Normal => 100,
            ScanIntensity::Aggressive => 0,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ScanIntensity::Quiet => "Slow, minimal network footprint",
            ScanIntensity::Normal => "Balanced speed and resource usage",
            ScanIntensity::Aggressive => "Fast scanning, higher detection risk",
        }
    }
}

impl std::fmt::Display for ScanIntensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanIntensity::Quiet => write!(f, "Quiet"),
            ScanIntensity::Normal => write!(f, "Normal"),
            ScanIntensity::Aggressive => write!(f, "Aggressive"),
        }
    }
}

// ============================================================================
// WORKFLOW CONFIGURATION
// ============================================================================

/// Configuration for assessment target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    /// Type of target
    pub target_type: AssessmentTargetType,
    /// Primary target value
    pub target: String,
    /// Resolved/expanded targets (for CIDR or multi-target)
    #[serde(default)]
    pub resolved_targets: Vec<String>,
    /// Authorization confirmation
    pub authorized: bool,
    /// Authorization reference (ticket, contract, etc.)
    #[serde(default)]
    pub authorization_ref: String,
    /// Notes about the target
    #[serde(default)]
    pub notes: String,
}

impl Default for TargetConfig {
    fn default() -> Self {
        Self {
            target_type: AssessmentTargetType::Domain,
            target: String::new(),
            resolved_targets: Vec::new(),
            authorized: false,
            authorization_ref: String::new(),
            notes: String::new(),
        }
    }
}

/// Module to execute in workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowModule {
    /// Unique ID for this module instance
    pub id: String,
    /// Module path (e.g., "scanner/port_scanner")
    pub path: String,
    /// Display name
    pub name: String,
    /// Description of what this module does
    pub description: String,
    /// Module options/configuration
    #[serde(default)]
    pub options: HashMap<String, String>,
    /// Whether module is enabled
    pub enabled: bool,
    /// Phase number (1 = Recon, 2 = Discovery)
    pub phase: u8,
    /// Estimated duration in seconds
    #[serde(default)]
    pub estimated_duration_secs: u32,
}

impl WorkflowModule {
    pub fn new(path: &str, name: &str, description: &str, phase: u8) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            path: path.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            options: HashMap::new(),
            enabled: true,
            phase,
            estimated_duration_secs: 30,
        }
    }

    pub fn with_option(mut self, key: &str, value: &str) -> Self {
        self.options.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_duration(mut self, secs: u32) -> Self {
        self.estimated_duration_secs = secs;
        self
    }
}

/// Complete workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    /// Unique workflow ID
    pub id: String,
    /// Workflow name
    pub name: String,
    /// Target configuration
    pub target: TargetConfig,
    /// Assessment scope
    pub scope: AssessmentScope,
    /// Scan intensity
    pub intensity: ScanIntensity,
    /// Modules to execute
    pub modules: Vec<WorkflowModule>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "New Assessment".to_string(),
            target: TargetConfig::default(),
            scope: AssessmentScope::Discovery,
            intensity: ScanIntensity::Normal,
            modules: Vec::new(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        }
    }
}

// ============================================================================
// WORKFLOW EXECUTION
// ============================================================================

/// Status of a workflow phase
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PhaseStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Status of a module execution
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModuleStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Result from a single module execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleExecutionResult {
    /// Module ID
    pub module_id: String,
    /// Module path
    pub module_path: String,
    /// Module name
    pub module_name: String,
    /// Execution status
    pub status: ModuleStatus,
    /// Success flag
    pub success: bool,
    /// Result message
    pub message: String,
    /// Structured result data
    pub data: HashMap<String, serde_json::Value>,
    /// Discovered items (IPs, ports, services, etc.)
    #[serde(default)]
    pub discoveries: Vec<Discovery>,
    /// Execution start time
    pub started_at: DateTime<Utc>,
    /// Execution end time
    pub completed_at: Option<DateTime<Utc>>,
    /// Duration in milliseconds
    pub duration_ms: u64,
}

/// A discovery made during scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discovery {
    /// Type of discovery
    pub discovery_type: DiscoveryType,
    /// Primary value (IP, hostname, etc.)
    pub value: String,
    /// Additional details
    #[serde(default)]
    pub details: HashMap<String, String>,
    /// Severity/importance (0-10)
    pub importance: u8,
}

/// Types of discoveries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryType {
    OpenPort,
    HttpService,
    DnsRecord,
    Subdomain,
    Technology,
    Certificate,
    WhoisInfo,
    AsnInfo,
    Vulnerability,
    Misconfiguration,
}

impl std::fmt::Display for DiscoveryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryType::OpenPort => write!(f, "Open Port"),
            DiscoveryType::HttpService => write!(f, "HTTP Service"),
            DiscoveryType::DnsRecord => write!(f, "DNS Record"),
            DiscoveryType::Subdomain => write!(f, "Subdomain"),
            DiscoveryType::Technology => write!(f, "Technology"),
            DiscoveryType::Certificate => write!(f, "Certificate"),
            DiscoveryType::WhoisInfo => write!(f, "WHOIS Info"),
            DiscoveryType::AsnInfo => write!(f, "ASN Info"),
            DiscoveryType::Vulnerability => write!(f, "Vulnerability"),
            DiscoveryType::Misconfiguration => write!(f, "Misconfiguration"),
        }
    }
}

/// Phase execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseResult {
    /// Phase number
    pub phase: u8,
    /// Phase name
    pub name: String,
    /// Status
    pub status: PhaseStatus,
    /// Module results
    pub modules: Vec<ModuleExecutionResult>,
    /// Total modules in phase
    pub total_modules: usize,
    /// Completed modules
    pub completed_modules: usize,
    /// Started timestamp
    pub started_at: Option<DateTime<Utc>>,
    /// Completed timestamp
    pub completed_at: Option<DateTime<Utc>>,
}

/// Overall workflow execution state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
    /// Not started
    Idle,
    /// Currently executing
    Running,
    /// Paused by user
    Paused,
    /// Completed successfully
    Completed,
    /// Failed with error
    Failed,
    /// Cancelled by user
    Cancelled,
}

/// Workflow execution progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProgress {
    /// Workflow ID
    pub workflow_id: String,
    /// Overall status
    pub status: WorkflowStatus,
    /// Current phase (1 or 2)
    pub current_phase: u8,
    /// Current module index within phase
    pub current_module_index: usize,
    /// Total modules
    pub total_modules: usize,
    /// Completed modules
    pub completed_modules: usize,
    /// Overall progress percentage (0-100)
    pub progress_percent: f32,
    /// Phase 1 (Recon) result
    pub phase1_result: Option<PhaseResult>,
    /// Phase 2 (Discovery) result
    pub phase2_result: Option<PhaseResult>,
    /// All discoveries
    pub all_discoveries: Vec<Discovery>,
    /// Started timestamp
    pub started_at: Option<DateTime<Utc>>,
    /// Estimated completion time
    pub estimated_completion: Option<DateTime<Utc>>,
    /// Error message if failed
    pub error: Option<String>,
}

impl WorkflowProgress {
    pub fn new(workflow_id: &str, total_modules: usize) -> Self {
        Self {
            workflow_id: workflow_id.to_string(),
            status: WorkflowStatus::Idle,
            current_phase: 1,
            current_module_index: 0,
            total_modules,
            completed_modules: 0,
            progress_percent: 0.0,
            phase1_result: None,
            phase2_result: None,
            all_discoveries: Vec::new(),
            started_at: None,
            estimated_completion: None,
            error: None,
        }
    }

    pub fn update_progress(&mut self) {
        if self.total_modules > 0 {
            self.progress_percent =
                (self.completed_modules as f32 / self.total_modules as f32) * 100.0;
        }
    }
}

// ============================================================================
// WORKFLOW TEMPLATES
// ============================================================================

/// Pre-built workflow templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Description
    pub description: String,
    /// Recommended target type
    pub recommended_target_type: AssessmentTargetType,
    /// Default scope
    pub default_scope: AssessmentScope,
    /// Default intensity
    pub default_intensity: ScanIntensity,
    /// Modules included
    pub modules: Vec<WorkflowModule>,
    /// Icon name
    pub icon: String,
    /// Tags for filtering
    pub tags: Vec<String>,
}

impl WorkflowTemplate {
    /// Quick network scan template
    pub fn quick_scan() -> Self {
        Self {
            id: "quick-scan".to_string(),
            name: "Quick Scan".to_string(),
            description: "Fast port scan and HTTP fingerprinting".to_string(),
            recommended_target_type: AssessmentTargetType::IpAddress,
            default_scope: AssessmentScope::Discovery,
            default_intensity: ScanIntensity::Normal,
            icon: "zap".to_string(),
            tags: vec!["fast".to_string(), "ports".to_string(), "http".to_string()],
            modules: vec![
                WorkflowModule::new(
                    "scanner/port_scanner",
                    "Port Scanner",
                    "Scan for open TCP ports",
                    2,
                )
                .with_option("PORTS", "21,22,23,25,53,80,110,143,443,445,993,995,3306,3389,5432,8080,8443")
                .with_duration(60),
                WorkflowModule::new(
                    "scanner/http_scanner",
                    "HTTP Scanner",
                    "Fingerprint HTTP/HTTPS services",
                    2,
                )
                .with_option("FOLLOW_REDIRECTS", "true")
                .with_duration(45),
            ],
        }
    }

    /// Domain reconnaissance template
    pub fn domain_recon() -> Self {
        Self {
            id: "domain-recon".to_string(),
            name: "Domain Reconnaissance".to_string(),
            description: "Comprehensive domain information gathering".to_string(),
            recommended_target_type: AssessmentTargetType::Domain,
            default_scope: AssessmentScope::ActiveRecon,
            default_intensity: ScanIntensity::Quiet,
            icon: "globe".to_string(),
            tags: vec!["domain".to_string(), "dns".to_string(), "recon".to_string()],
            modules: vec![
                WorkflowModule::new(
                    "recon/dns_enum",
                    "DNS Enumeration",
                    "Query DNS records (A, AAAA, MX, NS, TXT, SOA)",
                    1,
                )
                .with_option("RECORD_TYPES", "A,AAAA,MX,NS,TXT,SOA,CNAME")
                .with_duration(30),
                WorkflowModule::new(
                    "recon/whois_lookup",
                    "WHOIS Lookup",
                    "Domain registration and ownership info",
                    1,
                )
                .with_option("FOLLOW_REFERRAL", "true")
                .with_duration(15),
                WorkflowModule::new(
                    "recon/subdomain_enum",
                    "Subdomain Enumeration",
                    "Discover subdomains via passive sources",
                    1,
                )
                .with_option("PROBE_HTTP", "false")
                .with_duration(90),
            ],
        }
    }

    /// Web application assessment template
    pub fn web_assessment() -> Self {
        Self {
            id: "web-assessment".to_string(),
            name: "Web Application Assessment".to_string(),
            description: "Web server reconnaissance and discovery".to_string(),
            recommended_target_type: AssessmentTargetType::Url,
            default_scope: AssessmentScope::Comprehensive,
            default_intensity: ScanIntensity::Normal,
            icon: "layout".to_string(),
            tags: vec!["web".to_string(), "http".to_string(), "comprehensive".to_string()],
            modules: vec![
                // Phase 1: Recon
                WorkflowModule::new(
                    "recon/dns_enum",
                    "DNS Enumeration",
                    "Query DNS records for the domain",
                    1,
                )
                .with_option("RECORD_TYPES", "A,AAAA,MX,NS,TXT,CNAME")
                .with_duration(30),
                WorkflowModule::new(
                    "recon/subdomain_enum",
                    "Subdomain Discovery",
                    "Find related subdomains",
                    1,
                )
                .with_option("PROBE_HTTP", "true")
                .with_duration(90),
                // Phase 2: Discovery
                WorkflowModule::new(
                    "scanner/port_scanner",
                    "Port Scanner",
                    "Scan common web ports",
                    2,
                )
                .with_option("PORTS", "80,443,8080,8443,8000,3000,5000,9000")
                .with_duration(45),
                WorkflowModule::new(
                    "scanner/http_scanner",
                    "HTTP Fingerprinting",
                    "Detect web technologies and frameworks",
                    2,
                )
                .with_option("FOLLOW_REDIRECTS", "true")
                .with_option("PATHS", "/,/robots.txt,/sitemap.xml,/.well-known/")
                .with_duration(60),
            ],
        }
    }

    /// Network infrastructure template
    pub fn network_infrastructure() -> Self {
        Self {
            id: "network-infra".to_string(),
            name: "Network Infrastructure".to_string(),
            description: "Network range reconnaissance and service discovery".to_string(),
            recommended_target_type: AssessmentTargetType::CidrRange,
            default_scope: AssessmentScope::Discovery,
            default_intensity: ScanIntensity::Quiet,
            icon: "network".to_string(),
            tags: vec!["network".to_string(), "infrastructure".to_string(), "ports".to_string()],
            modules: vec![
                // Phase 1: Recon
                WorkflowModule::new(
                    "recon/asn_discovery",
                    "ASN Discovery",
                    "Identify network ownership and prefixes",
                    1,
                )
                .with_option("LOOKUP_PREFIXES", "true")
                .with_duration(20),
                WorkflowModule::new(
                    "recon/whois_lookup",
                    "WHOIS Lookup",
                    "IP/network registration info",
                    1,
                )
                .with_duration(15),
                // Phase 2: Discovery
                WorkflowModule::new(
                    "scanner/port_scanner",
                    "Port Scanner",
                    "Comprehensive port scan",
                    2,
                )
                .with_option("PORTS", "1-1000")
                .with_duration(180),
                WorkflowModule::new(
                    "scanner/http_scanner",
                    "HTTP Scanner",
                    "Detect HTTP services on open ports",
                    2,
                )
                .with_duration(90),
            ],
        }
    }

    /// Get all available templates
    pub fn all_templates() -> Vec<Self> {
        vec![
            Self::quick_scan(),
            Self::domain_recon(),
            Self::web_assessment(),
            Self::network_infrastructure(),
        ]
    }

    /// Convert template to workflow config
    pub fn to_workflow_config(&self, target: &str) -> WorkflowConfig {
        let mut config = WorkflowConfig::default();
        config.name = format!("{} - {}", self.name, target);
        config.target.target_type = self.recommended_target_type.clone();
        config.target.target = target.to_string();
        config.scope = self.default_scope;
        config.intensity = self.default_intensity;

        // Clone modules and apply target
        config.modules = self
            .modules
            .iter()
            .map(|m| {
                let mut module = m.clone();
                module.id = Uuid::new_v4().to_string();
                // Apply target to common options
                if module.path.starts_with("scanner/") {
                    module.options.insert("RHOSTS".to_string(), target.to_string());
                } else if module.path.starts_with("recon/") {
                    module.options.insert("TARGET".to_string(), target.to_string());
                }
                module
            })
            .collect();

        config
    }
}

// ============================================================================
// ASSESSMENT REPORT
// ============================================================================

/// Summary statistics for the report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportSummary {
    /// Total scan duration in seconds
    pub total_duration_secs: u64,
    /// Number of modules executed
    pub modules_executed: usize,
    /// Number of successful modules
    pub modules_succeeded: usize,
    /// Number of failed modules
    pub modules_failed: usize,
    /// Total discoveries
    pub total_discoveries: usize,
    /// Open ports found
    pub open_ports: usize,
    /// HTTP services found
    pub http_services: usize,
    /// Subdomains found
    pub subdomains: usize,
    /// Technologies detected
    pub technologies: usize,
}

/// Complete assessment report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentReport {
    /// Report ID
    pub id: String,
    /// Report title
    pub title: String,
    /// Target assessed
    pub target: String,
    /// Assessment scope used
    pub scope: AssessmentScope,
    /// Generated timestamp
    pub generated_at: DateTime<Utc>,
    /// Assessment start time
    pub started_at: DateTime<Utc>,
    /// Assessment end time
    pub completed_at: DateTime<Utc>,
    /// Summary statistics
    pub summary: ReportSummary,
    /// All discoveries grouped by type
    pub discoveries_by_type: HashMap<String, Vec<Discovery>>,
    /// Phase results
    pub phases: Vec<PhaseResult>,
    /// Authorization reference
    pub authorization_ref: String,
    /// Assessor notes
    #[serde(default)]
    pub notes: String,
}

impl AssessmentReport {
    /// Generate report from workflow progress
    pub fn from_progress(config: &WorkflowConfig, progress: &WorkflowProgress) -> Self {
        let mut summary = ReportSummary::default();
        let mut discoveries_by_type: HashMap<String, Vec<Discovery>> = HashMap::new();

        // Process all discoveries
        for discovery in &progress.all_discoveries {
            let type_key = format!("{}", discovery.discovery_type);
            discoveries_by_type
                .entry(type_key.clone())
                .or_default()
                .push(discovery.clone());

            summary.total_discoveries += 1;
            match discovery.discovery_type {
                DiscoveryType::OpenPort => summary.open_ports += 1,
                DiscoveryType::HttpService => summary.http_services += 1,
                DiscoveryType::Subdomain => summary.subdomains += 1,
                DiscoveryType::Technology => summary.technologies += 1,
                _ => {}
            }
        }

        // Collect phase results
        let mut phases = Vec::new();
        if let Some(p1) = &progress.phase1_result {
            summary.modules_executed += p1.modules.len();
            summary.modules_succeeded += p1.modules.iter().filter(|m| m.success).count();
            summary.modules_failed += p1.modules.iter().filter(|m| !m.success).count();
            phases.push(p1.clone());
        }
        if let Some(p2) = &progress.phase2_result {
            summary.modules_executed += p2.modules.len();
            summary.modules_succeeded += p2.modules.iter().filter(|m| m.success).count();
            summary.modules_failed += p2.modules.iter().filter(|m| !m.success).count();
            phases.push(p2.clone());
        }

        // Calculate duration
        if let (Some(started), Some(completed)) = (progress.started_at, progress.estimated_completion) {
            summary.total_duration_secs = (completed - started).num_seconds() as u64;
        }

        Self {
            id: Uuid::new_v4().to_string(),
            title: format!("Security Assessment Report: {}", config.target.target),
            target: config.target.target.clone(),
            scope: config.scope,
            generated_at: Utc::now(),
            started_at: progress.started_at.unwrap_or_else(Utc::now),
            completed_at: progress.estimated_completion.unwrap_or_else(Utc::now),
            summary,
            discoveries_by_type,
            phases,
            authorization_ref: config.target.authorization_ref.clone(),
            notes: config.target.notes.clone(),
        }
    }

    /// Export report to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| anyhow!("Failed to serialize report: {}", e))
    }

    /// Export report to YAML
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self).map_err(|e| anyhow!("Failed to serialize report: {}", e))
    }
}

// ============================================================================
// WORKFLOW EVENTS
// ============================================================================

/// Events emitted during workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkflowEvent {
    /// Workflow started
    Started {
        workflow_id: String,
        total_modules: usize,
    },
    /// Phase started
    PhaseStarted {
        workflow_id: String,
        phase: u8,
        phase_name: String,
    },
    /// Module started
    ModuleStarted {
        workflow_id: String,
        module_id: String,
        module_name: String,
    },
    /// Module progress update
    ModuleProgress {
        workflow_id: String,
        module_id: String,
        progress_percent: f32,
        message: String,
    },
    /// Module completed
    ModuleCompleted {
        workflow_id: String,
        module_id: String,
        success: bool,
        message: String,
        discoveries_count: usize,
    },
    /// Phase completed
    PhaseCompleted {
        workflow_id: String,
        phase: u8,
        success: bool,
    },
    /// New discovery
    DiscoveryMade {
        workflow_id: String,
        discovery: Discovery,
    },
    /// Progress update
    ProgressUpdate {
        workflow_id: String,
        progress: WorkflowProgress,
    },
    /// Workflow paused
    Paused {
        workflow_id: String,
    },
    /// Workflow resumed
    Resumed {
        workflow_id: String,
    },
    /// Workflow completed
    Completed {
        workflow_id: String,
        success: bool,
        total_discoveries: usize,
    },
    /// Error occurred
    Error {
        workflow_id: String,
        error: String,
    },
}

// ============================================================================
// WORKFLOW MANAGER
// ============================================================================

/// Main workflow manager
pub struct WorkflowManager {
    /// Currently active workflow
    active_workflow: Arc<RwLock<Option<WorkflowConfig>>>,
    /// Current progress
    progress: Arc<RwLock<Option<WorkflowProgress>>>,
    /// Event sender
    event_tx: mpsc::UnboundedSender<WorkflowEvent>,
    /// Event receiver (for external subscribers)
    event_rx: Arc<RwLock<Option<mpsc::UnboundedReceiver<WorkflowEvent>>>>,
    /// Cancel flag
    cancel_flag: Arc<RwLock<bool>>,
    /// Pause flag
    pause_flag: Arc<RwLock<bool>>,
}

impl WorkflowManager {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Self {
            active_workflow: Arc::new(RwLock::new(None)),
            progress: Arc::new(RwLock::new(None)),
            event_tx,
            event_rx: Arc::new(RwLock::new(Some(event_rx))),
            cancel_flag: Arc::new(RwLock::new(false)),
            pause_flag: Arc::new(RwLock::new(false)),
        }
    }

    /// Take event receiver (can only be done once)
    pub async fn take_event_receiver(&self) -> Option<mpsc::UnboundedReceiver<WorkflowEvent>> {
        self.event_rx.write().await.take()
    }

    /// Get available templates
    pub fn get_templates() -> Vec<WorkflowTemplate> {
        WorkflowTemplate::all_templates()
    }

    /// Get recommended modules for a target type and scope
    pub fn get_recommended_modules(
        target_type: &AssessmentTargetType,
        scope: &AssessmentScope,
    ) -> Vec<WorkflowModule> {
        let mut modules = Vec::new();

        match scope {
            AssessmentScope::PassiveRecon => {
                // Minimal passive-only modules
                modules.push(
                    WorkflowModule::new(
                        "recon/whois_lookup",
                        "WHOIS Lookup",
                        "Domain/IP registration info",
                        1,
                    )
                    .with_duration(15),
                );
            }
            AssessmentScope::ActiveRecon => {
                modules.push(
                    WorkflowModule::new(
                        "recon/dns_enum",
                        "DNS Enumeration",
                        "Query DNS records",
                        1,
                    )
                    .with_option("RECORD_TYPES", "A,AAAA,MX,NS,TXT,SOA,CNAME")
                    .with_duration(30),
                );
                modules.push(
                    WorkflowModule::new(
                        "recon/whois_lookup",
                        "WHOIS Lookup",
                        "Domain/IP registration info",
                        1,
                    )
                    .with_duration(15),
                );
                if matches!(target_type, AssessmentTargetType::Domain | AssessmentTargetType::Url) {
                    modules.push(
                        WorkflowModule::new(
                            "recon/subdomain_enum",
                            "Subdomain Enumeration",
                            "Discover subdomains",
                            1,
                        )
                        .with_option("PROBE_HTTP", "false")
                        .with_duration(90),
                    );
                }
            }
            AssessmentScope::Discovery => {
                modules.push(
                    WorkflowModule::new(
                        "scanner/port_scanner",
                        "Port Scanner",
                        "Scan for open ports",
                        2,
                    )
                    .with_option("PORTS", "1-1000")
                    .with_duration(120),
                );
                modules.push(
                    WorkflowModule::new(
                        "scanner/http_scanner",
                        "HTTP Scanner",
                        "HTTP service fingerprinting",
                        2,
                    )
                    .with_option("FOLLOW_REDIRECTS", "true")
                    .with_duration(60),
                );
            }
            AssessmentScope::Comprehensive => {
                // Phase 1: Recon
                modules.push(
                    WorkflowModule::new(
                        "recon/dns_enum",
                        "DNS Enumeration",
                        "Query DNS records",
                        1,
                    )
                    .with_option("RECORD_TYPES", "A,AAAA,MX,NS,TXT,SOA,CNAME")
                    .with_duration(30),
                );
                modules.push(
                    WorkflowModule::new(
                        "recon/whois_lookup",
                        "WHOIS Lookup",
                        "Domain/IP registration info",
                        1,
                    )
                    .with_duration(15),
                );
                if matches!(target_type, AssessmentTargetType::Domain | AssessmentTargetType::Url) {
                    modules.push(
                        WorkflowModule::new(
                            "recon/subdomain_enum",
                            "Subdomain Enumeration",
                            "Discover subdomains",
                            1,
                        )
                        .with_option("PROBE_HTTP", "true")
                        .with_duration(90),
                    );
                }
                if matches!(
                    target_type,
                    AssessmentTargetType::IpAddress | AssessmentTargetType::CidrRange
                ) {
                    modules.push(
                        WorkflowModule::new(
                            "recon/asn_discovery",
                            "ASN Discovery",
                            "Network ownership info",
                            1,
                        )
                        .with_option("LOOKUP_PREFIXES", "true")
                        .with_duration(20),
                    );
                }
                // Phase 2: Discovery
                modules.push(
                    WorkflowModule::new(
                        "scanner/port_scanner",
                        "Port Scanner",
                        "Scan for open ports",
                        2,
                    )
                    .with_option("PORTS", "1-1000")
                    .with_duration(120),
                );
                modules.push(
                    WorkflowModule::new(
                        "scanner/http_scanner",
                        "HTTP Scanner",
                        "HTTP service fingerprinting",
                        2,
                    )
                    .with_option("FOLLOW_REDIRECTS", "true")
                    .with_option("PATHS", "/,/robots.txt,/sitemap.xml")
                    .with_duration(60),
                );
            }
        }

        modules
    }

    /// Start workflow execution
    pub async fn start(&self, config: WorkflowConfig) -> Result<()> {
        // Validate authorization
        if !config.target.authorized {
            return Err(anyhow!(
                "Authorization required. Please confirm you have permission to assess this target."
            ));
        }

        // Check if already running
        {
            let active = self.active_workflow.read().await;
            if active.is_some() {
                return Err(anyhow!("A workflow is already running"));
            }
        }

        // Reset flags
        *self.cancel_flag.write().await = false;
        *self.pause_flag.write().await = false;

        // Initialize progress
        let total_modules = config.modules.len();
        let progress = WorkflowProgress::new(&config.id, total_modules);

        // Store workflow and progress
        *self.active_workflow.write().await = Some(config.clone());
        *self.progress.write().await = Some(progress);

        // Emit start event
        let _ = self.event_tx.send(WorkflowEvent::Started {
            workflow_id: config.id.clone(),
            total_modules,
        });

        // Execute workflow in background
        let manager = self.clone_refs();
        tokio::spawn(async move {
            if let Err(e) = manager.execute_workflow().await {
                let _ = manager.event_tx.send(WorkflowEvent::Error {
                    workflow_id: config.id,
                    error: e.to_string(),
                });
            }
        });

        Ok(())
    }

    /// Clone references for async task
    fn clone_refs(&self) -> WorkflowManagerRef {
        WorkflowManagerRef {
            active_workflow: Arc::clone(&self.active_workflow),
            progress: Arc::clone(&self.progress),
            event_tx: self.event_tx.clone(),
            cancel_flag: Arc::clone(&self.cancel_flag),
            pause_flag: Arc::clone(&self.pause_flag),
        }
    }

    /// Pause execution
    pub async fn pause(&self) -> Result<()> {
        let workflow = self.active_workflow.read().await;
        if let Some(config) = workflow.as_ref() {
            *self.pause_flag.write().await = true;
            let _ = self.event_tx.send(WorkflowEvent::Paused {
                workflow_id: config.id.clone(),
            });
            Ok(())
        } else {
            Err(anyhow!("No active workflow"))
        }
    }

    /// Resume execution
    pub async fn resume(&self) -> Result<()> {
        let workflow = self.active_workflow.read().await;
        if let Some(config) = workflow.as_ref() {
            *self.pause_flag.write().await = false;
            let _ = self.event_tx.send(WorkflowEvent::Resumed {
                workflow_id: config.id.clone(),
            });
            Ok(())
        } else {
            Err(anyhow!("No active workflow"))
        }
    }

    /// Cancel execution
    pub async fn cancel(&self) -> Result<()> {
        let workflow = self.active_workflow.read().await;
        if let Some(config) = workflow.as_ref() {
            *self.cancel_flag.write().await = true;
            if let Some(progress) = self.progress.write().await.as_mut() {
                progress.status = WorkflowStatus::Cancelled;
            }
            let _ = self.event_tx.send(WorkflowEvent::Completed {
                workflow_id: config.id.clone(),
                success: false,
                total_discoveries: 0,
            });
            Ok(())
        } else {
            Err(anyhow!("No active workflow"))
        }
    }

    /// Get current progress
    pub async fn get_progress(&self) -> Option<WorkflowProgress> {
        self.progress.read().await.clone()
    }

    /// Get active workflow config
    pub async fn get_active_workflow(&self) -> Option<WorkflowConfig> {
        self.active_workflow.read().await.clone()
    }

    /// Generate assessment report
    pub async fn generate_report(&self) -> Result<AssessmentReport> {
        let config = self
            .active_workflow
            .read()
            .await
            .clone()
            .ok_or_else(|| anyhow!("No active workflow"))?;
        let progress = self
            .progress
            .read()
            .await
            .clone()
            .ok_or_else(|| anyhow!("No progress data"))?;
        Ok(AssessmentReport::from_progress(&config, &progress))
    }
}

impl Default for WorkflowManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Reference struct for background execution
struct WorkflowManagerRef {
    active_workflow: Arc<RwLock<Option<WorkflowConfig>>>,
    progress: Arc<RwLock<Option<WorkflowProgress>>>,
    event_tx: mpsc::UnboundedSender<WorkflowEvent>,
    cancel_flag: Arc<RwLock<bool>>,
    pause_flag: Arc<RwLock<bool>>,
}

impl WorkflowManagerRef {
    async fn execute_workflow(&self) -> Result<()> {
        let config = self
            .active_workflow
            .read()
            .await
            .clone()
            .ok_or_else(|| anyhow!("No active workflow"))?;

        // Update status to running
        if let Some(progress) = self.progress.write().await.as_mut() {
            progress.status = WorkflowStatus::Running;
            progress.started_at = Some(Utc::now());
        }

        // Group modules by phase
        let phase1_modules: Vec<_> = config.modules.iter().filter(|m| m.phase == 1).collect();
        let phase2_modules: Vec<_> = config.modules.iter().filter(|m| m.phase == 2).collect();

        // Execute Phase 1: Reconnaissance
        if !phase1_modules.is_empty() {
            self.execute_phase(1, "Reconnaissance", &phase1_modules, &config)
                .await?;
        }

        // Check for cancellation
        if *self.cancel_flag.read().await {
            return Ok(());
        }

        // Execute Phase 2: Discovery
        if !phase2_modules.is_empty() {
            self.execute_phase(2, "Discovery", &phase2_modules, &config)
                .await?;
        }

        // Mark as completed
        let total_discoveries = {
            let progress = self.progress.read().await;
            progress.as_ref().map(|p| p.all_discoveries.len()).unwrap_or(0)
        };

        if let Some(progress) = self.progress.write().await.as_mut() {
            progress.status = WorkflowStatus::Completed;
            progress.estimated_completion = Some(Utc::now());
            progress.progress_percent = 100.0;
        }

        let _ = self.event_tx.send(WorkflowEvent::Completed {
            workflow_id: config.id,
            success: true,
            total_discoveries,
        });

        // Clear active workflow
        *self.active_workflow.write().await = None;

        Ok(())
    }

    async fn execute_phase(
        &self,
        phase_num: u8,
        phase_name: &str,
        modules: &[&WorkflowModule],
        config: &WorkflowConfig,
    ) -> Result<()> {
        let _ = self.event_tx.send(WorkflowEvent::PhaseStarted {
            workflow_id: config.id.clone(),
            phase: phase_num,
            phase_name: phase_name.to_string(),
        });

        // Update progress
        if let Some(progress) = self.progress.write().await.as_mut() {
            progress.current_phase = phase_num;
        }

        let mut phase_result = PhaseResult {
            phase: phase_num,
            name: phase_name.to_string(),
            status: PhaseStatus::Running,
            modules: Vec::new(),
            total_modules: modules.len(),
            completed_modules: 0,
            started_at: Some(Utc::now()),
            completed_at: None,
        };

        for (idx, module) in modules.iter().enumerate() {
            // Check pause flag
            while *self.pause_flag.read().await {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }

            // Check cancel flag
            if *self.cancel_flag.read().await {
                phase_result.status = PhaseStatus::Skipped;
                break;
            }

            // Skip disabled modules
            if !module.enabled {
                continue;
            }

            // Update progress
            if let Some(progress) = self.progress.write().await.as_mut() {
                progress.current_module_index = idx;
            }

            // Execute module
            let result = self.execute_module(module, config).await;
            phase_result.modules.push(result);
            phase_result.completed_modules += 1;

            // Update overall progress
            if let Some(progress) = self.progress.write().await.as_mut() {
                progress.completed_modules += 1;
                progress.update_progress();
                let _ = self.event_tx.send(WorkflowEvent::ProgressUpdate {
                    workflow_id: config.id.clone(),
                    progress: progress.clone(),
                });
            }
        }

        phase_result.status = if phase_result.modules.iter().all(|m| m.success) {
            PhaseStatus::Completed
        } else {
            PhaseStatus::Failed
        };
        phase_result.completed_at = Some(Utc::now());

        // Store phase result
        if let Some(progress) = self.progress.write().await.as_mut() {
            match phase_num {
                1 => progress.phase1_result = Some(phase_result.clone()),
                2 => progress.phase2_result = Some(phase_result.clone()),
                _ => {}
            }
        }

        let _ = self.event_tx.send(WorkflowEvent::PhaseCompleted {
            workflow_id: config.id.clone(),
            phase: phase_num,
            success: phase_result.status == PhaseStatus::Completed,
        });

        Ok(())
    }

    async fn execute_module(
        &self,
        module: &WorkflowModule,
        config: &WorkflowConfig,
    ) -> ModuleExecutionResult {
        let started_at = Utc::now();

        let _ = self.event_tx.send(WorkflowEvent::ModuleStarted {
            workflow_id: config.id.clone(),
            module_id: module.id.clone(),
            module_name: module.name.clone(),
        });

        // Simulate module execution (in real implementation, this would call actual modules)
        let (success, message, discoveries) = self.simulate_module_execution(module, config).await;

        let completed_at = Utc::now();
        let duration_ms = (completed_at - started_at).num_milliseconds() as u64;

        // Store discoveries
        if let Some(progress) = self.progress.write().await.as_mut() {
            progress.all_discoveries.extend(discoveries.clone());
        }

        // Emit discovery events
        for discovery in &discoveries {
            let _ = self.event_tx.send(WorkflowEvent::DiscoveryMade {
                workflow_id: config.id.clone(),
                discovery: discovery.clone(),
            });
        }

        let _ = self.event_tx.send(WorkflowEvent::ModuleCompleted {
            workflow_id: config.id.clone(),
            module_id: module.id.clone(),
            success,
            message: message.clone(),
            discoveries_count: discoveries.len(),
        });

        ModuleExecutionResult {
            module_id: module.id.clone(),
            module_path: module.path.clone(),
            module_name: module.name.clone(),
            status: if success {
                ModuleStatus::Completed
            } else {
                ModuleStatus::Failed
            },
            success,
            message,
            data: HashMap::new(),
            discoveries,
            started_at,
            completed_at: Some(completed_at),
            duration_ms,
        }
    }

    /// Simulate module execution with realistic delays and sample discoveries
    async fn simulate_module_execution(
        &self,
        module: &WorkflowModule,
        _config: &WorkflowConfig,
    ) -> (bool, String, Vec<Discovery>) {
        // Simulate execution time
        let duration = std::cmp::max(module.estimated_duration_secs / 3, 2);
        tokio::time::sleep(tokio::time::Duration::from_secs(duration as u64)).await;

        let mut discoveries = Vec::new();

        // Generate sample discoveries based on module type
        match module.path.as_str() {
            "scanner/port_scanner" => {
                // Sample open ports
                for port in [22, 80, 443] {
                    discoveries.push(Discovery {
                        discovery_type: DiscoveryType::OpenPort,
                        value: format!("{}", port),
                        details: HashMap::from([
                            ("protocol".to_string(), "tcp".to_string()),
                            ("state".to_string(), "open".to_string()),
                        ]),
                        importance: if port == 22 { 7 } else { 5 },
                    });
                }
            }
            "scanner/http_scanner" => {
                discoveries.push(Discovery {
                    discovery_type: DiscoveryType::HttpService,
                    value: "HTTP/1.1 200 OK".to_string(),
                    details: HashMap::from([
                        ("server".to_string(), "nginx/1.18.0".to_string()),
                        ("content_type".to_string(), "text/html".to_string()),
                    ]),
                    importance: 5,
                });
                discoveries.push(Discovery {
                    discovery_type: DiscoveryType::Technology,
                    value: "nginx".to_string(),
                    details: HashMap::from([("version".to_string(), "1.18.0".to_string())]),
                    importance: 4,
                });
            }
            "recon/dns_enum" => {
                discoveries.push(Discovery {
                    discovery_type: DiscoveryType::DnsRecord,
                    value: "A: 93.184.216.34".to_string(),
                    details: HashMap::from([("record_type".to_string(), "A".to_string())]),
                    importance: 5,
                });
                discoveries.push(Discovery {
                    discovery_type: DiscoveryType::DnsRecord,
                    value: "MX: mail.example.com".to_string(),
                    details: HashMap::from([
                        ("record_type".to_string(), "MX".to_string()),
                        ("priority".to_string(), "10".to_string()),
                    ]),
                    importance: 4,
                });
            }
            "recon/subdomain_enum" => {
                for sub in ["www", "mail", "api", "dev"] {
                    discoveries.push(Discovery {
                        discovery_type: DiscoveryType::Subdomain,
                        value: format!("{}.example.com", sub),
                        details: HashMap::new(),
                        importance: 6,
                    });
                }
            }
            "recon/whois_lookup" => {
                discoveries.push(Discovery {
                    discovery_type: DiscoveryType::WhoisInfo,
                    value: "Registrar: Example Registrar".to_string(),
                    details: HashMap::from([
                        ("created".to_string(), "2000-01-01".to_string()),
                        ("expires".to_string(), "2025-01-01".to_string()),
                    ]),
                    importance: 3,
                });
            }
            "recon/asn_discovery" => {
                discoveries.push(Discovery {
                    discovery_type: DiscoveryType::AsnInfo,
                    value: "AS15169 - Google LLC".to_string(),
                    details: HashMap::from([
                        ("asn".to_string(), "15169".to_string()),
                        ("org".to_string(), "Google LLC".to_string()),
                    ]),
                    importance: 4,
                });
            }
            _ => {}
        }

        (
            true,
            format!("{} completed successfully", module.name),
            discoveries,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_template_creation() {
        let template = WorkflowTemplate::quick_scan();
        assert_eq!(template.id, "quick-scan");
        assert_eq!(template.modules.len(), 2);
    }

    #[test]
    fn test_template_to_config() {
        let template = WorkflowTemplate::domain_recon();
        let config = template.to_workflow_config("example.com");
        assert!(config.name.contains("example.com"));
        assert!(!config.modules.is_empty());
    }

    #[test]
    fn test_progress_calculation() {
        let mut progress = WorkflowProgress::new("test", 10);
        progress.completed_modules = 5;
        progress.update_progress();
        assert!((progress.progress_percent - 50.0).abs() < 0.1);
    }
}
