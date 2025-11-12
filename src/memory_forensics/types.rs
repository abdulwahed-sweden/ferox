use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DumpType {
    Kernel,
    User,
    MiniDump,
    Full,
    Hybrid,
    Unknown,
}

impl Default for DumpType {
    fn default() -> Self {
        DumpType::Unknown
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Architecture {
    X86,
    X64,
    Arm64,
    Unknown,
}

impl Default for Architecture {
    fn default() -> Self {
        Architecture::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemInfo {
    pub os_version: String,
    pub architecture: Architecture,
    pub build_number: Option<u32>,
    pub hostname: Option<String>,
    pub uptime: Option<Duration>,
    pub memory_mb: Option<u64>,
    pub cpu_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub base_address: u64,
    pub size: u64,
    pub protection: Option<String>,
    pub state: Option<String>,
    pub is_executable: bool,
}

impl Default for MemoryRegion {
    fn default() -> Self {
        Self {
            base_address: 0,
            size: 0,
            protection: None,
            state: None,
            is_executable: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreadInfo {
    pub tid: u32,
    pub priority: Option<i32>,
    pub start_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HandleInfo {
    pub handle: u64,
    pub handle_type: Option<String>,
    pub name: Option<String>,
    pub access_mask: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub ppid: Option<u32>,
    pub command_line: Option<String>,
    pub architecture: Architecture,
    pub start_time: Option<DateTime<Utc>>,
    pub thread_count: usize,
    pub threads: Vec<ThreadInfo>,
    pub memory_regions: Vec<MemoryRegion>,
    pub handles: Vec<HandleInfo>,
    pub suspicious_tags: Vec<String>,
}

impl ProcessInfo {
    pub fn is_suspicious(&self) -> bool {
        !self.suspicious_tags.is_empty()
    }
}

impl Default for ProcessInfo {
    fn default() -> Self {
        Self {
            pid: 0,
            name: String::from("unknown"),
            ppid: None,
            command_line: None,
            architecture: Architecture::Unknown,
            start_time: None,
            thread_count: 0,
            threads: Vec::new(),
            memory_regions: Vec::new(),
            handles: Vec::new(),
            suspicious_tags: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessTreeNode {
    pub pid: u32,
    pub name: String,
    pub depth: usize,
    pub parent: Option<u32>,
    pub children: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessTree {
    pub nodes: Vec<ProcessTreeNode>,
}

impl ProcessTree {
    pub fn add_node(&mut self, node: ProcessTreeNode) {
        self.nodes.push(node);
    }

    pub fn iter(&self) -> impl Iterator<Item = &ProcessTreeNode> {
        self.nodes.iter()
    }

    pub fn print_hierarchical(&self) {
        for node in &self.nodes {
            let indent = "  ".repeat(node.depth);
            println!("{}[PID: {}] {}", indent, node.pid, node.name);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeInjectionFinding {
    pub pid: u32,
    pub description: String,
    pub address: Option<u64>,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalwareFinding {
    pub pid: Option<u32>,
    pub indicator: String,
    pub description: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Low
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreTechnique {
    pub id: String,
    pub name: String,
    pub tactic: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: Option<String>,
    pub remote_port: Option<u16>,
    pub state: Option<String>,
    pub pid: Option<u32>,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkArtifact {
    pub artifact_type: String,
    pub value: String,
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryKey {
    pub path: String,
    pub values: HashMap<String, String>,
    pub last_write_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CredentialType {
    NtlmHash,
    Sha1Hash,
    KerberosTicket,
    BrowserCredential,
    LsaSecret,
    Token,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialArtifact {
    pub credential_type: CredentialType,
    pub identifier: String,
    pub data: HashMap<String, String>,
    pub is_encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisReport {
    pub dump_path: String,
    pub analysis_time: DateTime<Utc>,
    pub dump_type: DumpType,
    pub system_info: SystemInfo,
    pub processes: Vec<ProcessInfo>,
    pub code_injections: Vec<CodeInjectionFinding>,
    pub malware_findings: Vec<MalwareFinding>,
    pub network_connections: Vec<NetworkConnection>,
    pub network_artifacts: Vec<NetworkArtifact>,
    pub registry_keys: Vec<RegistryKey>,
    pub credentials: Vec<CredentialArtifact>,
    pub mitre_techniques: Vec<MitreTechnique>,
    pub risk_score: Option<u8>,
}
