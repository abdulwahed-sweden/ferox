//! Credential Harvesting Engine - Multi-Platform Credential Extraction Framework
//!
//! Comprehensive credential harvesting framework for authorized penetration testing
//! and red team exercises. Supports Windows, Linux, and macOS.
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY
//! All techniques require explicit authorization for deployment.
//!
//! Features:
//! - Multi-source credential harvesting
//! - Platform-specific harvesters
//! - Safe mode demonstrations
//! - MITRE ATT&CK mapping
//!
//! MITRE ATT&CK Coverage:
//! - T1003: OS Credential Dumping
//! - T1003.001: LSASS Memory
//! - T1003.002: Security Account Manager
//! - T1003.004: LSA Secrets
//! - T1003.005: Cached Domain Credentials
//! - T1555: Credentials from Password Stores
//! - T1555.001: Keychain
//! - T1555.003: Credentials from Web Browsers
//! - T1555.004: Windows Credential Manager
//! - T1552: Unsecured Credentials
//! - T1552.001: Credentials in Files
//! - T1552.004: Private Keys

use anyhow::{bail, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};
use uuid::Uuid;

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType, Platform, Session,
};

// ============================================================================
// Core Types
// ============================================================================

/// Type of credential harvested
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CredentialType {
    /// Plain text password
    PlainText,
    /// Password hash (NTLM, SHA, etc.)
    Hash,
    /// Authentication token (API key, JWT, etc.)
    Token,
    /// Session cookie
    Cookie,
    /// Certificate/key pair
    Certificate,
    /// SSH private key
    SshKey,
    /// Kerberos ticket
    KerberosTicket,
    /// AWS/Cloud credentials
    CloudCredential,
}

impl CredentialType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PlainText => "Plain Text",
            Self::Hash => "Hash",
            Self::Token => "Token",
            Self::Cookie => "Cookie",
            Self::Certificate => "Certificate",
            Self::SshKey => "SSH Key",
            Self::KerberosTicket => "Kerberos Ticket",
            Self::CloudCredential => "Cloud Credential",
        }
    }
}

/// Source category for credentials
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceCategory {
    /// Operating system credential stores
    OsCredentialStore,
    /// Browser stored passwords
    Browser,
    /// Application-specific storage
    Application,
    /// Memory extraction
    Memory,
    /// File system
    FileSystem,
    /// Network capture
    Network,
    /// Cloud/environment
    Cloud,
}

impl SourceCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OsCredentialStore => "OS Credential Store",
            Self::Browser => "Browser",
            Self::Application => "Application",
            Self::Memory => "Memory",
            Self::FileSystem => "File System",
            Self::Network => "Network",
            Self::Cloud => "Cloud/Environment",
        }
    }
}

/// Sensitivity level of harvested credentials
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Sensitivity {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

impl Sensitivity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }
}

/// A harvested credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestedCredential {
    pub id: Uuid,
    pub cred_type: CredentialType,
    pub source: String,
    pub source_category: SourceCategory,
    pub sensitivity: Sensitivity,
    pub username: Option<String>,
    pub password: Option<String>,
    pub hash: Option<String>,
    pub domain: Option<String>,
    pub target: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl HarvestedCredential {
    pub fn new(cred_type: CredentialType, source: &str, category: SourceCategory) -> Self {
        Self {
            id: Uuid::new_v4(),
            cred_type,
            source: source.to_string(),
            source_category: category,
            sensitivity: Sensitivity::Medium,
            username: None,
            password: None,
            hash: None,
            domain: None,
            target: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }

    pub fn with_password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

    pub fn with_hash(mut self, hash: &str) -> Self {
        self.hash = Some(hash.to_string());
        self
    }

    pub fn with_domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    pub fn with_sensitivity(mut self, sensitivity: Sensitivity) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Redact sensitive information for display
    pub fn redacted(&self) -> Self {
        let mut redacted = self.clone();
        if let Some(ref pass) = redacted.password {
            let visible = pass.len().min(2);
            redacted.password = Some(format!("{}***REDACTED***", &pass[..visible]));
        }
        if let Some(ref hash) = redacted.hash {
            let len = hash.len();
            if len > 16 {
                redacted.hash = Some(format!("{}...{}", &hash[..8], &hash[len - 8..]));
            }
        }
        redacted
    }
}

/// Result of a harvest operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestResult {
    pub success: bool,
    pub harvester_name: String,
    pub credentials: Vec<HarvestedCredential>,
    pub message: String,
    pub errors: Vec<String>,
}

// ============================================================================
// Harvester Trait
// ============================================================================

/// Core trait for credential harvesters
#[async_trait]
pub trait CredentialHarvester: Send + Sync {
    /// Harvester name identifier
    fn name(&self) -> &str;

    /// Target platform
    fn platform(&self) -> Platform;

    /// Source category
    fn category(&self) -> SourceCategory;

    /// MITRE ATT&CK technique ID
    fn mitre_id(&self) -> &str;

    /// Description of what this harvester targets
    fn description(&self) -> &str;

    /// Whether this requires admin/root privileges
    fn requires_admin(&self) -> bool;

    /// Harvest credentials
    async fn harvest(&self, session: &Session, safe_mode: bool) -> Result<HarvestResult>;

    /// Generate reference implementation documentation
    fn generate_reference(&self) -> String;
}

// ============================================================================
// Windows Harvesters
// ============================================================================

/// LSASS Memory Harvester (Mimikatz-style)
pub struct LsassHarvester;

#[async_trait]
impl CredentialHarvester for LsassHarvester {
    fn name(&self) -> &str {
        "lsass_memory"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::Memory
    }

    fn mitre_id(&self) -> &str {
        "T1003.001"
    }

    fn description(&self) -> &str {
        "Extract credentials from LSASS process memory"
    }

    fn requires_admin(&self) -> bool {
        true
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            info!("[SAFE MODE] Would extract credentials from LSASS memory");

            let credentials = vec![
                HarvestedCredential::new(CredentialType::PlainText, "lsass.exe", SourceCategory::Memory)
                    .with_username("Administrator")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_domain("DOMAIN")
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("process", "lsass.exe")
                    .with_metadata("pid", "672"),
                HarvestedCredential::new(CredentialType::Hash, "lsass.exe", SourceCategory::Memory)
                    .with_username("Administrator")
                    .with_hash("aad3b435b51404eeaad3b435b51404ee:8846f7eaee8fb117ad06bdd830b7586c")
                    .with_domain("DOMAIN")
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("hash_type", "NTLM"),
                HarvestedCredential::new(CredentialType::KerberosTicket, "lsass.exe", SourceCategory::Memory)
                    .with_username("Administrator")
                    .with_domain("DOMAIN.LOCAL")
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("ticket_type", "TGT")
                    .with_metadata("encryption", "AES256"),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] LSASS memory extraction simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== LSASS Memory Credential Harvesting ===

DESCRIPTION:
Extracts credentials from the Local Security Authority Subsystem Service (LSASS)
process memory. This is the primary target for Windows credential theft.

TECHNIQUE:
1. Open LSASS process with PROCESS_VM_READ access
2. Enumerate and read memory regions
3. Parse credential structures (Wdigest, Kerberos, NTLM, etc.)
4. Extract plaintext passwords, hashes, and tickets

TOOLS:
- Mimikatz sekurlsa::logonpasswords
- Procdump + offline analysis
- Direct memory reading

DETECTION:
- Access to LSASS process
- Suspicious memory reads
- Windows Defender Credential Guard bypass attempts

PREREQUISITES:
- Administrator/SYSTEM privileges
- SeDebugPrivilege

MITRE ATT&CK: T1003.001 - LSASS Memory
"#
        .to_string()
    }
}

/// SAM Database Harvester
pub struct SamHarvester;

#[async_trait]
impl CredentialHarvester for SamHarvester {
    fn name(&self) -> &str {
        "sam_database"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::OsCredentialStore
    }

    fn mitre_id(&self) -> &str {
        "T1003.002"
    }

    fn description(&self) -> &str {
        "Extract password hashes from SAM database"
    }

    fn requires_admin(&self) -> bool {
        true
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            info!("[SAFE MODE] Would extract hashes from SAM database");

            let credentials = vec![
                HarvestedCredential::new(CredentialType::Hash, "SAM", SourceCategory::OsCredentialStore)
                    .with_username("Administrator")
                    .with_hash("aad3b435b51404eeaad3b435b51404ee:31d6cfe0d16ae931b73c59d7e0c089c0")
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("rid", "500"),
                HarvestedCredential::new(CredentialType::Hash, "SAM", SourceCategory::OsCredentialStore)
                    .with_username("Guest")
                    .with_hash("aad3b435b51404eeaad3b435b51404ee:31d6cfe0d16ae931b73c59d7e0c089c0")
                    .with_sensitivity(Sensitivity::Low)
                    .with_metadata("rid", "501"),
                HarvestedCredential::new(CredentialType::Hash, "SAM", SourceCategory::OsCredentialStore)
                    .with_username("DefaultAccount")
                    .with_hash("aad3b435b51404eeaad3b435b51404ee:31d6cfe0d16ae931b73c59d7e0c089c0")
                    .with_sensitivity(Sensitivity::Low)
                    .with_metadata("rid", "503"),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] SAM database extraction simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== SAM Database Credential Harvesting ===

DESCRIPTION:
Extracts password hashes from the Security Account Manager (SAM) database.
Contains local user account NTLM hashes.

TECHNIQUE:
1. Access SAM hive from registry or file
2. Decrypt using SYSKEY from SYSTEM hive
3. Extract NTLM hashes for each local account

LOCATIONS:
- Live: HKLM\SAM\SAM\Domains\Account\Users
- Offline: %SystemRoot%\System32\config\SAM

TOOLS:
- Mimikatz lsadump::sam
- secretsdump.py (Impacket)
- reg save + offline extraction

PREREQUISITES:
- SYSTEM privileges or offline access

MITRE ATT&CK: T1003.002 - Security Account Manager
"#
        .to_string()
    }
}

/// Windows Credential Manager Harvester
pub struct CredentialManagerHarvester;

#[async_trait]
impl CredentialHarvester for CredentialManagerHarvester {
    fn name(&self) -> &str {
        "credential_manager"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::OsCredentialStore
    }

    fn mitre_id(&self) -> &str {
        "T1555.004"
    }

    fn description(&self) -> &str {
        "Extract credentials from Windows Credential Manager"
    }

    fn requires_admin(&self) -> bool {
        false // User-level access possible
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            info!("[SAFE MODE] Would extract from Credential Manager");

            let credentials = vec![
                HarvestedCredential::new(CredentialType::PlainText, "Credential Manager", SourceCategory::OsCredentialStore)
                    .with_username("user@domain.com")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("https://login.microsoftonline.com")
                    .with_sensitivity(Sensitivity::High)
                    .with_metadata("type", "Generic"),
                HarvestedCredential::new(CredentialType::PlainText, "Credential Manager", SourceCategory::OsCredentialStore)
                    .with_username("DOMAIN\\serviceaccount")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("TERMSRV/server.domain.local")
                    .with_sensitivity(Sensitivity::High)
                    .with_metadata("type", "Domain Password"),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] Credential Manager extraction simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Windows Credential Manager Harvesting ===

DESCRIPTION:
Extracts stored credentials from Windows Credential Manager.
Stores web passwords, Windows credentials, and generic credentials.

TECHNIQUE:
1. Enumerate credentials using CredEnumerate API
2. Read credential blobs
3. Decrypt using DPAPI (user context)

TOOLS:
- cmdkey /list
- Mimikatz vault::cred
- PowerShell Get-StoredCredential

CREDENTIAL TYPES:
- Generic credentials (web, application)
- Domain passwords
- Certificate-based credentials

MITRE ATT&CK: T1555.004 - Windows Credential Manager
"#
        .to_string()
    }
}

/// Browser Credential Harvester (Windows)
pub struct WindowsBrowserHarvester;

#[async_trait]
impl CredentialHarvester for WindowsBrowserHarvester {
    fn name(&self) -> &str {
        "windows_browser"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::Browser
    }

    fn mitre_id(&self) -> &str {
        "T1555.003"
    }

    fn description(&self) -> &str {
        "Extract saved passwords from Chrome, Firefox, and Edge"
    }

    fn requires_admin(&self) -> bool {
        false
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            info!("[SAFE MODE] Would extract browser credentials");

            let credentials = vec![
                HarvestedCredential::new(CredentialType::PlainText, "Chrome", SourceCategory::Browser)
                    .with_username("user@gmail.com")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("https://accounts.google.com")
                    .with_sensitivity(Sensitivity::High),
                HarvestedCredential::new(CredentialType::PlainText, "Firefox", SourceCategory::Browser)
                    .with_username("admin")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("https://internal.company.com")
                    .with_sensitivity(Sensitivity::High),
                HarvestedCredential::new(CredentialType::Cookie, "Edge", SourceCategory::Browser)
                    .with_target("https://office365.com")
                    .with_sensitivity(Sensitivity::Medium)
                    .with_metadata("cookie_name", "ESTSAUTHPERSISTENT"),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] Browser credential extraction simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Browser Credential Harvesting (Windows) ===

DESCRIPTION:
Extracts saved passwords and cookies from web browsers.

BROWSERS SUPPORTED:
- Google Chrome
- Mozilla Firefox
- Microsoft Edge
- Brave

TECHNIQUE:
Chrome/Edge:
1. Read Login Data SQLite database
2. Decrypt passwords using DPAPI or AES-GCM (v80+)

Firefox:
1. Read logins.json
2. Decrypt using key4.db (NSS)

LOCATIONS:
- Chrome: %LOCALAPPDATA%\Google\Chrome\User Data\Default\Login Data
- Firefox: %APPDATA%\Mozilla\Firefox\Profiles\*.default\logins.json
- Edge: %LOCALAPPDATA%\Microsoft\Edge\User Data\Default\Login Data

MITRE ATT&CK: T1555.003 - Credentials from Web Browsers
"#
        .to_string()
    }
}

// ============================================================================
// Linux Harvesters
// ============================================================================

/// Shadow File Harvester
pub struct ShadowHarvester;

#[async_trait]
impl CredentialHarvester for ShadowHarvester {
    fn name(&self) -> &str {
        "shadow_file"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::OsCredentialStore
    }

    fn mitre_id(&self) -> &str {
        "T1003.008"
    }

    fn description(&self) -> &str {
        "Extract password hashes from /etc/shadow"
    }

    fn requires_admin(&self) -> bool {
        true
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            info!("[SAFE MODE] Would extract hashes from /etc/shadow");

            let credentials = vec![
                HarvestedCredential::new(CredentialType::Hash, "/etc/shadow", SourceCategory::OsCredentialStore)
                    .with_username("root")
                    .with_hash("$6$rounds=5000$salt$hashhashhashhash...")
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("hash_type", "SHA-512"),
                HarvestedCredential::new(CredentialType::Hash, "/etc/shadow", SourceCategory::OsCredentialStore)
                    .with_username("admin")
                    .with_hash("$y$j9T$salt$hashhashhashhash...")
                    .with_sensitivity(Sensitivity::High)
                    .with_metadata("hash_type", "yescrypt"),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] /etc/shadow extraction simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== /etc/shadow Credential Harvesting ===

DESCRIPTION:
Extracts password hashes from the Linux shadow password file.

TECHNIQUE:
1. Read /etc/shadow (requires root)
2. Parse hash format ($id$salt$hash)
3. Identify hash algorithm

HASH TYPES:
- $1$ = MD5
- $5$ = SHA-256
- $6$ = SHA-512
- $y$ = yescrypt

COMMAND:
cat /etc/shadow | grep -v '!' | grep -v '*'

CRACKING:
- John the Ripper
- Hashcat mode 1800 (SHA-512)

MITRE ATT&CK: T1003.008 - /etc/passwd and /etc/shadow
"#
        .to_string()
    }
}

/// SSH Key Harvester
pub struct SshKeyHarvester;

#[async_trait]
impl CredentialHarvester for SshKeyHarvester {
    fn name(&self) -> &str {
        "ssh_keys"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::FileSystem
    }

    fn mitre_id(&self) -> &str {
        "T1552.004"
    }

    fn description(&self) -> &str {
        "Harvest SSH private keys from user directories"
    }

    fn requires_admin(&self) -> bool {
        false // Can harvest own keys, root for others
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            info!("[SAFE MODE] Would harvest SSH private keys");

            let credentials = vec![
                HarvestedCredential::new(CredentialType::SshKey, "~/.ssh/id_rsa", SourceCategory::FileSystem)
                    .with_username("user")
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("key_type", "RSA")
                    .with_metadata("bits", "4096")
                    .with_metadata("encrypted", "false"),
                HarvestedCredential::new(CredentialType::SshKey, "~/.ssh/id_ed25519", SourceCategory::FileSystem)
                    .with_username("user")
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("key_type", "ED25519")
                    .with_metadata("encrypted", "true"),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] SSH key harvesting simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== SSH Private Key Harvesting ===

DESCRIPTION:
Searches for and extracts SSH private keys from user directories.

LOCATIONS:
- ~/.ssh/id_rsa
- ~/.ssh/id_dsa
- ~/.ssh/id_ecdsa
- ~/.ssh/id_ed25519
- /etc/ssh/ssh_host_*_key (host keys)

TECHNIQUE:
1. Enumerate user home directories
2. Check .ssh directories for private keys
3. Identify encrypted vs unencrypted keys
4. Extract and optionally crack passphrases

KEY IDENTIFICATION:
- BEGIN RSA PRIVATE KEY (unencrypted RSA)
- BEGIN ENCRYPTED PRIVATE KEY (encrypted)
- BEGIN OPENSSH PRIVATE KEY (new format)

PASSPHRASE CRACKING:
- ssh2john + John the Ripper
- Hashcat with ssh2john output

MITRE ATT&CK: T1552.004 - Private Keys
"#
        .to_string()
    }
}

/// GNOME Keyring Harvester
pub struct GnomeKeyringHarvester;

#[async_trait]
impl CredentialHarvester for GnomeKeyringHarvester {
    fn name(&self) -> &str {
        "gnome_keyring"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::Application
    }

    fn mitre_id(&self) -> &str {
        "T1555"
    }

    fn description(&self) -> &str {
        "Extract credentials from GNOME Keyring"
    }

    fn requires_admin(&self) -> bool {
        false
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            info!("[SAFE MODE] Would extract from GNOME Keyring");

            let credentials = vec![
                HarvestedCredential::new(CredentialType::PlainText, "GNOME Keyring", SourceCategory::Application)
                    .with_username("user@server.com")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("smtp://mail.server.com")
                    .with_sensitivity(Sensitivity::High),
                HarvestedCredential::new(CredentialType::Token, "GNOME Keyring", SourceCategory::Application)
                    .with_target("Google Chrome Safe Storage")
                    .with_sensitivity(Sensitivity::High),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] GNOME Keyring extraction simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== GNOME Keyring Credential Harvesting ===

DESCRIPTION:
Extracts stored credentials from GNOME Keyring, the default
secret storage on many Linux desktop distributions.

LOCATION:
~/.local/share/keyrings/

TECHNIQUE:
1. Access keyring via D-Bus Secret Service API
2. Or decrypt keyring files directly if session unlocked
3. Parse stored secrets

TOOLS:
- secret-tool lookup
- keyring Python library
- gnome-keyring-dump

MITRE ATT&CK: T1555 - Credentials from Password Stores
"#
        .to_string()
    }
}

/// Linux Browser Harvester
pub struct LinuxBrowserHarvester;

#[async_trait]
impl CredentialHarvester for LinuxBrowserHarvester {
    fn name(&self) -> &str {
        "linux_browser"
    }

    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::Browser
    }

    fn mitre_id(&self) -> &str {
        "T1555.003"
    }

    fn description(&self) -> &str {
        "Extract saved passwords from Linux browsers"
    }

    fn requires_admin(&self) -> bool {
        false
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            let credentials = vec![
                HarvestedCredential::new(CredentialType::PlainText, "Firefox", SourceCategory::Browser)
                    .with_username("user@example.com")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("https://github.com")
                    .with_sensitivity(Sensitivity::High),
                HarvestedCredential::new(CredentialType::PlainText, "Chrome", SourceCategory::Browser)
                    .with_username("admin")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("https://aws.amazon.com")
                    .with_sensitivity(Sensitivity::Critical),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] Linux browser credential simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Linux Browser Credential Harvesting ===

DESCRIPTION:
Extracts saved credentials from browsers on Linux systems.

BROWSERS:
- Firefox: ~/.mozilla/firefox/*.default/logins.json
- Chrome: ~/.config/google-chrome/Default/Login Data
- Chromium: ~/.config/chromium/Default/Login Data

TECHNIQUE:
Firefox:
1. Read logins.json
2. Get key from key4.db
3. Decrypt using NSS

Chrome/Chromium:
1. Get encryption key from GNOME Keyring or kwallet
2. Read Login Data SQLite
3. Decrypt AES-GCM encrypted passwords

MITRE ATT&CK: T1555.003 - Credentials from Web Browsers
"#
        .to_string()
    }
}

// ============================================================================
// macOS Harvesters
// ============================================================================

/// macOS Keychain Harvester
pub struct KeychainHarvester;

#[async_trait]
impl CredentialHarvester for KeychainHarvester {
    fn name(&self) -> &str {
        "macos_keychain"
    }

    fn platform(&self) -> Platform {
        Platform::MacOS
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::OsCredentialStore
    }

    fn mitre_id(&self) -> &str {
        "T1555.001"
    }

    fn description(&self) -> &str {
        "Extract credentials from macOS Keychain"
    }

    fn requires_admin(&self) -> bool {
        false // User-level for login keychain
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            info!("[SAFE MODE] Would extract from macOS Keychain");

            let credentials = vec![
                HarvestedCredential::new(CredentialType::PlainText, "login.keychain", SourceCategory::OsCredentialStore)
                    .with_username("user@icloud.com")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("appleid.apple.com")
                    .with_sensitivity(Sensitivity::Critical),
                HarvestedCredential::new(CredentialType::PlainText, "login.keychain", SourceCategory::OsCredentialStore)
                    .with_username("admin")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("server.local (AFP)")
                    .with_sensitivity(Sensitivity::High),
                HarvestedCredential::new(CredentialType::Certificate, "System.keychain", SourceCategory::OsCredentialStore)
                    .with_target("Developer Certificate")
                    .with_sensitivity(Sensitivity::High)
                    .with_metadata("issuer", "Apple Development"),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] Keychain extraction simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== macOS Keychain Credential Harvesting ===

DESCRIPTION:
Extracts stored credentials from macOS Keychain, including
login passwords, certificates, and secure notes.

KEYCHAINS:
- ~/Library/Keychains/login.keychain-db (user)
- /Library/Keychains/System.keychain (system)

TECHNIQUE:
1. Use security command-line tool
2. Or parse keychain database directly
3. Requires user password or session unlock

COMMANDS:
- security dump-keychain -d login.keychain
- security find-generic-password -ga "service"
- security find-internet-password -ga "server"

TOOLS:
- Chainbreaker (offline parsing)
- KeychainDump

MITRE ATT&CK: T1555.001 - Keychain
"#
        .to_string()
    }
}

/// macOS Browser Harvester
pub struct MacosBrowserHarvester;

#[async_trait]
impl CredentialHarvester for MacosBrowserHarvester {
    fn name(&self) -> &str {
        "macos_browser"
    }

    fn platform(&self) -> Platform {
        Platform::MacOS
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::Browser
    }

    fn mitre_id(&self) -> &str {
        "T1555.003"
    }

    fn description(&self) -> &str {
        "Extract saved passwords from Safari, Chrome, and Firefox"
    }

    fn requires_admin(&self) -> bool {
        false
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            let credentials = vec![
                HarvestedCredential::new(CredentialType::PlainText, "Safari", SourceCategory::Browser)
                    .with_username("user@apple.com")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("https://apple.com")
                    .with_sensitivity(Sensitivity::High),
                HarvestedCredential::new(CredentialType::PlainText, "Chrome", SourceCategory::Browser)
                    .with_username("developer")
                    .with_password("[SAFE_MODE_DEMO]")
                    .with_target("https://console.cloud.google.com")
                    .with_sensitivity(Sensitivity::Critical),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] macOS browser credential simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== macOS Browser Credential Harvesting ===

DESCRIPTION:
Extracts saved passwords from browsers on macOS.

BROWSERS:
- Safari: Uses Keychain (see macos_keychain)
- Chrome: ~/Library/Application Support/Google/Chrome/Default/Login Data
- Firefox: ~/Library/Application Support/Firefox/Profiles/*.default/logins.json

CHROME TECHNIQUE:
1. Get Safe Storage key from Keychain
2. Read Login Data SQLite database
3. Decrypt using AES-CBC with derived key

SAFARI:
Safari passwords are stored in Keychain, use the keychain harvester.

MITRE ATT&CK: T1555.003 - Credentials from Web Browsers
"#
        .to_string()
    }
}

// ============================================================================
// Cross-Platform Harvesters
// ============================================================================

/// Cloud Credentials Harvester
pub struct CloudCredentialHarvester;

#[async_trait]
impl CredentialHarvester for CloudCredentialHarvester {
    fn name(&self) -> &str {
        "cloud_credentials"
    }

    fn platform(&self) -> Platform {
        Platform::Any
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::Cloud
    }

    fn mitre_id(&self) -> &str {
        "T1552.001"
    }

    fn description(&self) -> &str {
        "Harvest cloud provider credentials from files and environment"
    }

    fn requires_admin(&self) -> bool {
        false
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            info!("[SAFE MODE] Would harvest cloud credentials");

            let credentials = vec![
                HarvestedCredential::new(CredentialType::CloudCredential, "~/.aws/credentials", SourceCategory::Cloud)
                    .with_username("default")
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("provider", "AWS")
                    .with_metadata("access_key_id", "AKIA...DEMO"),
                HarvestedCredential::new(CredentialType::CloudCredential, "~/.azure/credentials", SourceCategory::Cloud)
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("provider", "Azure")
                    .with_metadata("subscription", "demo-subscription"),
                HarvestedCredential::new(CredentialType::CloudCredential, "~/.config/gcloud/credentials.db", SourceCategory::Cloud)
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("provider", "GCP"),
                HarvestedCredential::new(CredentialType::Token, "ENV:AWS_SECRET_ACCESS_KEY", SourceCategory::Cloud)
                    .with_sensitivity(Sensitivity::Critical)
                    .with_metadata("source", "environment"),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] Cloud credential harvesting simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Cloud Credential Harvesting ===

DESCRIPTION:
Harvests credentials for cloud providers from configuration files
and environment variables.

AWS:
- ~/.aws/credentials
- ~/.aws/config
- Environment: AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY

AZURE:
- ~/.azure/credentials
- ~/.azure/accessTokens.json
- Environment: AZURE_CLIENT_ID, AZURE_CLIENT_SECRET

GCP:
- ~/.config/gcloud/credentials.db
- ~/.config/gcloud/application_default_credentials.json
- Environment: GOOGLE_APPLICATION_CREDENTIALS

KUBERNETES:
- ~/.kube/config
- Service account tokens

MITRE ATT&CK: T1552.001 - Credentials in Files
"#
        .to_string()
    }
}

/// Git Credentials Harvester
pub struct GitCredentialHarvester;

#[async_trait]
impl CredentialHarvester for GitCredentialHarvester {
    fn name(&self) -> &str {
        "git_credentials"
    }

    fn platform(&self) -> Platform {
        Platform::Any
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::Application
    }

    fn mitre_id(&self) -> &str {
        "T1552.001"
    }

    fn description(&self) -> &str {
        "Harvest Git credentials and tokens"
    }

    fn requires_admin(&self) -> bool {
        false
    }

    async fn harvest(&self, _session: &Session, safe_mode: bool) -> Result<HarvestResult> {
        if safe_mode {
            let credentials = vec![
                HarvestedCredential::new(CredentialType::Token, "~/.git-credentials", SourceCategory::Application)
                    .with_username("user")
                    .with_target("https://github.com")
                    .with_sensitivity(Sensitivity::High)
                    .with_metadata("type", "Personal Access Token"),
                HarvestedCredential::new(CredentialType::SshKey, "~/.ssh/id_ed25519", SourceCategory::Application)
                    .with_target("git@github.com")
                    .with_sensitivity(Sensitivity::High)
                    .with_metadata("type", "SSH Deploy Key"),
            ];

            return Ok(HarvestResult {
                success: true,
                harvester_name: self.name().to_string(),
                credentials,
                message: "[SAFE MODE] Git credential harvesting simulation".to_string(),
                errors: vec![],
            });
        }

        bail!("Production mode not available in reference implementation");
    }

    fn generate_reference(&self) -> String {
        r#"=== Git Credential Harvesting ===

DESCRIPTION:
Harvests Git credentials including stored passwords and tokens.

LOCATIONS:
- ~/.git-credentials (plaintext)
- ~/.gitconfig (credential helper config)
- Git credential manager stores

TECHNIQUE:
1. Check git config for credential helpers
2. Read credential storage files
3. Query credential managers (Windows, macOS Keychain)

TOKENS TO FIND:
- GitHub Personal Access Tokens (ghp_*)
- GitLab Personal Access Tokens
- Bitbucket App Passwords

MITRE ATT&CK: T1552.001 - Credentials in Files
"#
        .to_string()
    }
}

// ============================================================================
// Credential Harvesting Engine
// ============================================================================

/// Main credential harvesting engine
pub struct CredentialHarvestEngine {
    harvesters: Vec<Box<dyn CredentialHarvester>>,
    harvested: Vec<HarvestedCredential>,
}

impl Default for CredentialHarvestEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialHarvestEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            harvesters: Vec::new(),
            harvested: Vec::new(),
        };

        // Windows harvesters
        engine.harvesters.push(Box::new(LsassHarvester));
        engine.harvesters.push(Box::new(SamHarvester));
        engine.harvesters.push(Box::new(CredentialManagerHarvester));
        engine.harvesters.push(Box::new(WindowsBrowserHarvester));

        // Linux harvesters
        engine.harvesters.push(Box::new(ShadowHarvester));
        engine.harvesters.push(Box::new(SshKeyHarvester));
        engine.harvesters.push(Box::new(GnomeKeyringHarvester));
        engine.harvesters.push(Box::new(LinuxBrowserHarvester));

        // macOS harvesters
        engine.harvesters.push(Box::new(KeychainHarvester));
        engine.harvesters.push(Box::new(MacosBrowserHarvester));

        // Cross-platform harvesters
        engine.harvesters.push(Box::new(CloudCredentialHarvester));
        engine.harvesters.push(Box::new(GitCredentialHarvester));

        engine
    }

    /// Get harvesters for a specific platform
    pub fn harvesters_for_platform(&self, platform: &Platform) -> Vec<&dyn CredentialHarvester> {
        self.harvesters
            .iter()
            .filter(|h| h.platform() == *platform || h.platform() == Platform::Any)
            .map(|h| h.as_ref())
            .collect()
    }

    /// Get harvesters by category
    pub fn harvesters_by_category(&self, category: SourceCategory) -> Vec<&dyn CredentialHarvester> {
        self.harvesters
            .iter()
            .filter(|h| h.category() == category)
            .map(|h| h.as_ref())
            .collect()
    }

    /// Harvest from all sources for a platform
    pub async fn harvest_all(
        &mut self,
        session: &Session,
        safe_mode: bool,
    ) -> Result<Vec<HarvestResult>> {
        let mut results = Vec::new();

        let harvester_names: Vec<String> = self
            .harvesters
            .iter()
            .filter(|h| h.platform() == session.platform || h.platform() == Platform::Any)
            .map(|h| h.name().to_string())
            .collect();

        for name in harvester_names {
            let harvester = self
                .harvesters
                .iter()
                .find(|h| h.name() == name)
                .unwrap();

            match harvester.harvest(session, safe_mode).await {
                Ok(result) => {
                    info!(
                        harvester = name,
                        credentials = result.credentials.len(),
                        "Harvest completed"
                    );
                    self.harvested.extend(result.credentials.clone());
                    results.push(result);
                }
                Err(e) => {
                    warn!(harvester = name, error = %e, "Harvest failed");
                    results.push(HarvestResult {
                        success: false,
                        harvester_name: name,
                        credentials: vec![],
                        message: format!("Failed: {}", e),
                        errors: vec![e.to_string()],
                    });
                }
            }
        }

        Ok(results)
    }

    /// Get all harvested credentials
    pub fn harvested_credentials(&self) -> &[HarvestedCredential] {
        &self.harvested
    }

    /// Clear harvested credentials
    pub fn clear(&mut self) {
        self.harvested.clear();
    }
}

// ============================================================================
// Module Implementation
// ============================================================================

/// Credential Harvesting Module (Module trait implementation)
pub struct CredentialHarvestModule {
    options: HashMap<String, String>,
    engine: CredentialHarvestEngine,
}

impl CredentialHarvestModule {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("safe_mode".to_string(), "true".to_string());
        options.insert("platform".to_string(), "auto".to_string());
        options.insert("redact_output".to_string(), "true".to_string());
        options.insert("category".to_string(), "all".to_string());

        Self {
            options,
            engine: CredentialHarvestEngine::new(),
        }
    }
}

impl Default for CredentialHarvestModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for CredentialHarvestModule {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "credential_harvester".to_string(),
            version: "2.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description: "Credential Harvesting Engine - Multi-platform credential extraction. \
                         AUTHORIZED USE ONLY."
                .to_string(),
            module_type: ModuleType::PostExploit,
            category: "credential".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "safe_mode".to_string(),
                description: "Use safe mode (true/false) - demo only".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("safe_mode").cloned(),
            },
            ModuleOption {
                name: "platform".to_string(),
                description: "Target platform: auto, windows, linux, macos".to_string(),
                required: false,
                default_value: Some("auto".to_string()),
                current_value: self.options.get("platform").cloned(),
            },
            ModuleOption {
                name: "redact_output".to_string(),
                description: "Redact sensitive data in output".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("redact_output").cloned(),
            },
            ModuleOption {
                name: "category".to_string(),
                description: "Category: all, os, browser, cloud, application".to_string(),
                required: false,
                default_value: Some("all".to_string()),
                current_value: self.options.get("category").cloned(),
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
            bail!("Production mode requires explicit authorization");
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let platform = match self.options.get("platform").map(|s| s.as_str()) {
            Some("windows") => Platform::Windows,
            Some("linux") => Platform::Linux,
            Some("macos") => Platform::MacOS,
            _ => Platform::Any,
        };

        let harvester_count = self.engine.harvesters_for_platform(&platform).len();

        let mut fingerprint = HashMap::new();
        fingerprint.insert("platform".to_string(), format!("{:?}", platform));
        fingerprint.insert("harvesters".to_string(), harvester_count.to_string());

        Ok(CheckResult {
            vulnerable: true,
            confidence: 0.8,
            details: format!(
                "Credential Harvesting Engine ready\n\
                 Platform: {:?}\n\
                 Available Harvesters: {}",
                platform, harvester_count
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
            Some("macos") => Platform::MacOS,
            _ => Platform::Windows, // Default for demo
        };

        let redact = self
            .options
            .get("redact_output")
            .map(|s| s == "true")
            .unwrap_or(true);

        // Create mock session
        let session = Session::new(
            "credential_harvester".to_string(),
            "target".to_string(),
            platform,
        );

        // Run harvest
        let results = self.engine.harvest_all(&session, safe_mode).await?;

        let total_creds: usize = results.iter().map(|r| r.credentials.len()).sum();
        let successful = results.iter().filter(|r| r.success).count();

        let mut module_result = ModuleResult::success(format!(
            "Credential harvesting complete - found {} credentials from {} sources",
            total_creds, successful
        ));

        // Build output
        let creds_json: Vec<serde_json::Value> = self
            .engine
            .harvested_credentials()
            .iter()
            .map(|c| {
                let c = if redact { c.redacted() } else { c.clone() };
                serde_json::json!({
                    "type": c.cred_type.as_str(),
                    "source": c.source,
                    "category": c.source_category.as_str(),
                    "sensitivity": c.sensitivity.as_str(),
                    "username": c.username,
                    "domain": c.domain,
                    "target": c.target,
                })
            })
            .collect();

        module_result = module_result
            .with_data("total_credentials", serde_json::json!(total_creds))
            .with_data("harvesters_run", serde_json::json!(results.len()))
            .with_data("successful_harvesters", serde_json::json!(successful))
            .with_data("credentials", serde_json::json!(creds_json))
            .with_data("safe_mode", serde_json::json!(true));

        Ok(module_result)
    }

    async fn cleanup(&mut self) -> Result<()> {
        self.engine.clear();
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
    fn test_credential_type_display() {
        assert_eq!(CredentialType::PlainText.as_str(), "Plain Text");
        assert_eq!(CredentialType::Hash.as_str(), "Hash");
        assert_eq!(CredentialType::SshKey.as_str(), "SSH Key");
    }

    #[test]
    fn test_sensitivity_ordering() {
        assert!(Sensitivity::Critical > Sensitivity::High);
        assert!(Sensitivity::High > Sensitivity::Medium);
        assert!(Sensitivity::Medium > Sensitivity::Low);
    }

    #[test]
    fn test_credential_redaction() {
        let cred = HarvestedCredential::new(
            CredentialType::PlainText,
            "test",
            SourceCategory::Browser,
        )
        .with_username("user")
        .with_password("SuperSecretPassword123!");

        let redacted = cred.redacted();
        assert!(redacted.password.as_ref().unwrap().contains("REDACTED"));
        assert_eq!(redacted.username, Some("user".to_string()));
    }

    #[test]
    fn test_engine_creation() {
        let engine = CredentialHarvestEngine::new();
        assert!(!engine.harvesters.is_empty());
    }

    #[test]
    fn test_platform_filtering() {
        let engine = CredentialHarvestEngine::new();

        let windows = engine.harvesters_for_platform(&Platform::Windows);
        let linux = engine.harvesters_for_platform(&Platform::Linux);
        let macos = engine.harvesters_for_platform(&Platform::MacOS);

        assert!(!windows.is_empty());
        assert!(!linux.is_empty());
        assert!(!macos.is_empty());

        // Cross-platform harvesters should be in all
        assert!(windows.iter().any(|h| h.name() == "cloud_credentials"));
        assert!(linux.iter().any(|h| h.name() == "cloud_credentials"));
    }

    #[test]
    fn test_category_filtering() {
        let engine = CredentialHarvestEngine::new();

        let browser = engine.harvesters_by_category(SourceCategory::Browser);
        let cloud = engine.harvesters_by_category(SourceCategory::Cloud);

        assert!(!browser.is_empty());
        assert!(!cloud.is_empty());
    }

    #[tokio::test]
    async fn test_harvest_safe_mode() {
        let mut engine = CredentialHarvestEngine::new();
        let session = Session::new(
            "test".to_string(),
            "target".to_string(),
            Platform::Windows,
        );

        let results = engine.harvest_all(&session, true).await.unwrap();
        assert!(!results.is_empty());

        let total: usize = results.iter().map(|r| r.credentials.len()).sum();
        assert!(total > 0);
    }

    #[tokio::test]
    async fn test_module_safe_mode() {
        let mut module = CredentialHarvestModule::new();
        module.set_option("safe_mode", "true").unwrap();
        module.set_option("platform", "windows").unwrap();

        assert!(module.validate().is_ok());

        let result = module.run().await.unwrap();
        assert!(result.success);
        assert!(result.data.contains_key("total_credentials"));
    }

    #[test]
    fn test_mitre_ids() {
        let lsass = LsassHarvester;
        assert_eq!(lsass.mitre_id(), "T1003.001");

        let keychain = KeychainHarvester;
        assert_eq!(keychain.mitre_id(), "T1555.001");

        let cloud = CloudCredentialHarvester;
        assert_eq!(cloud.mitre_id(), "T1552.001");
    }

    #[test]
    fn test_reference_generation() {
        let lsass = LsassHarvester;
        let reference = lsass.generate_reference();
        assert!(reference.contains("LSASS"));
        assert!(reference.contains("MITRE ATT&CK"));

        let shadow = ShadowHarvester;
        let reference = shadow.generate_reference();
        assert!(reference.contains("/etc/shadow"));
    }
}
