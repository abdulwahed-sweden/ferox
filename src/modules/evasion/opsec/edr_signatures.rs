//! EDR/AV Signatures Database
//!
//! Contains detection signatures for major security products.
//! MITRE ATT&CK: T1518.001 (Security Software Discovery)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};

/// Known EDR/AV Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdrType {
    // Microsoft
    WindowsDefender,
    DefenderATP,

    // Enterprise EDRs
    CrowdStrike,
    SentinelOne,
    CarbonBlack,
    Cylance,
    Cybereason,
    Elastic,

    // Traditional AV with EDR
    TrendMicro,
    Sophos,
    Kaspersky,
    ESET,
    McAfee,
    Symantec,
    Bitdefender,
    Avast,
    AVG,
    Norton,

    // Open Source / Other
    Wazuh,
    OSSEC,
    Velociraptor,
    LimaCharlie,

    // Unknown product
    Unknown,
}

impl EdrType {
    /// Get threat level (1-10, higher = harder to evade)
    pub fn threat_level(&self) -> u8 {
        match self {
            Self::CrowdStrike => 10,
            Self::SentinelOne => 10,
            Self::DefenderATP => 9,
            Self::CarbonBlack => 9,
            Self::Cybereason => 8,
            Self::Elastic => 8,
            Self::Cylance => 7,
            Self::TrendMicro => 7,
            Self::Sophos => 7,
            Self::Kaspersky => 7,
            Self::WindowsDefender => 6,
            Self::ESET => 6,
            Self::Bitdefender => 6,
            Self::Velociraptor => 6,
            Self::LimaCharlie => 6,
            Self::McAfee => 5,
            Self::Symantec => 5,
            Self::Wazuh | Self::OSSEC => 5,
            Self::Avast | Self::AVG => 4,
            Self::Norton => 4,
            Self::Unknown => 5,
        }
    }

    /// Get vendor name
    pub fn vendor(&self) -> &'static str {
        match self {
            Self::WindowsDefender | Self::DefenderATP => "Microsoft",
            Self::CrowdStrike => "CrowdStrike",
            Self::SentinelOne => "SentinelOne",
            Self::CarbonBlack => "VMware",
            Self::Cylance => "BlackBerry",
            Self::Cybereason => "Cybereason",
            Self::Elastic => "Elastic",
            Self::TrendMicro => "Trend Micro",
            Self::Sophos => "Sophos",
            Self::Kaspersky => "Kaspersky",
            Self::ESET => "ESET",
            Self::McAfee => "McAfee",
            Self::Symantec => "Broadcom",
            Self::Bitdefender => "Bitdefender",
            Self::Avast | Self::AVG => "Avast",
            Self::Norton => "NortonLifeLock",
            Self::Wazuh => "Wazuh",
            Self::OSSEC => "OSSEC",
            Self::Velociraptor => "Rapid7",
            Self::LimaCharlie => "LimaCharlie",
            Self::Unknown => "Unknown",
        }
    }

    /// Get product name
    pub fn product_name(&self) -> &'static str {
        match self {
            Self::WindowsDefender => "Windows Defender",
            Self::DefenderATP => "Microsoft Defender for Endpoint",
            Self::CrowdStrike => "CrowdStrike Falcon",
            Self::SentinelOne => "SentinelOne",
            Self::CarbonBlack => "VMware Carbon Black",
            Self::Cylance => "Cylance",
            Self::Cybereason => "Cybereason",
            Self::Elastic => "Elastic Security",
            Self::TrendMicro => "Trend Micro",
            Self::Sophos => "Sophos",
            Self::Kaspersky => "Kaspersky",
            Self::ESET => "ESET",
            Self::McAfee => "McAfee",
            Self::Symantec => "Symantec Endpoint Protection",
            Self::Bitdefender => "Bitdefender",
            Self::Avast => "Avast",
            Self::AVG => "AVG",
            Self::Norton => "Norton",
            Self::Wazuh => "Wazuh",
            Self::OSSEC => "OSSEC",
            Self::Velociraptor => "Velociraptor",
            Self::LimaCharlie => "LimaCharlie",
            Self::Unknown => "Unknown Security Product",
        }
    }

    /// Get MITRE ATT&CK technique for discovery
    pub fn mitre_discovery_id(&self) -> &'static str {
        "T1518.001"
    }

    /// Check if this is an enterprise-grade EDR
    pub fn is_enterprise_edr(&self) -> bool {
        matches!(
            self,
            Self::CrowdStrike
                | Self::SentinelOne
                | Self::CarbonBlack
                | Self::DefenderATP
                | Self::Cybereason
                | Self::Elastic
        )
    }
}

/// EDR Signature for detection
#[derive(Debug, Clone)]
pub struct EdrSignature {
    pub edr_type: EdrType,
    pub processes: Vec<&'static str>,
    pub services: Vec<&'static str>,
    pub drivers: Vec<&'static str>,
    pub registry_keys: Vec<&'static str>,
    pub files: Vec<&'static str>,
    pub dlls: Vec<&'static str>,
    pub pipes: Vec<&'static str>,
}

impl EdrSignature {
    /// Create a new signature
    pub fn new(edr_type: EdrType) -> Self {
        Self {
            edr_type,
            processes: Vec::new(),
            services: Vec::new(),
            drivers: Vec::new(),
            registry_keys: Vec::new(),
            files: Vec::new(),
            dlls: Vec::new(),
            pipes: Vec::new(),
        }
    }
}

/// Get all EDR signatures
pub fn get_all_signatures() -> Vec<EdrSignature> {
    vec![
        // CrowdStrike Falcon
        EdrSignature {
            edr_type: EdrType::CrowdStrike,
            processes: vec![
                "CSFalconService.exe",
                "CSFalconContainer.exe",
                "csagent.exe",
                "falcond.exe",
            ],
            services: vec!["CSFalconService", "CSAgent"],
            drivers: vec!["csagent.sys", "csdevicecontrol.sys", "csboot.sys"],
            registry_keys: vec![
                r"HKLM\SYSTEM\CrowdStrike",
                r"HKLM\SOFTWARE\CrowdStrike",
            ],
            files: vec![
                r"C:\Windows\System32\drivers\CrowdStrike",
                r"C:\Program Files\CrowdStrike",
            ],
            dlls: vec!["csagent.dll"],
            pipes: vec![r"\\.\pipe\CrowdStrike"],
        },
        // SentinelOne
        EdrSignature {
            edr_type: EdrType::SentinelOne,
            processes: vec![
                "SentinelAgent.exe",
                "SentinelServiceHost.exe",
                "SentinelStaticEngine.exe",
                "SentinelHelperService.exe",
            ],
            services: vec!["SentinelAgent", "SentinelStaticEngine"],
            drivers: vec!["sentinelmonitor.sys"],
            registry_keys: vec![
                r"HKLM\SOFTWARE\Sentinel Labs",
                r"HKLM\SYSTEM\CurrentControlSet\Services\SentinelAgent",
            ],
            files: vec![r"C:\Program Files\SentinelOne"],
            dlls: vec!["SentinelOne.dll"],
            pipes: vec![],
        },
        // Microsoft Defender
        EdrSignature {
            edr_type: EdrType::WindowsDefender,
            processes: vec![
                "MsMpEng.exe",
                "MpCmdRun.exe",
                "NisSrv.exe",
                "SecurityHealthService.exe",
                "SecurityHealthSystray.exe",
            ],
            services: vec!["WinDefend", "WdNisSvc", "SecurityHealthService"],
            drivers: vec!["WdFilter.sys", "WdNisDrv.sys", "WdBoot.sys"],
            registry_keys: vec![
                r"HKLM\SOFTWARE\Microsoft\Windows Defender",
                r"HKLM\SOFTWARE\Policies\Microsoft\Windows Defender",
            ],
            files: vec![r"C:\ProgramData\Microsoft\Windows Defender"],
            dlls: vec!["mpclient.dll", "mpengine.dll"],
            pipes: vec![],
        },
        // Microsoft Defender ATP
        EdrSignature {
            edr_type: EdrType::DefenderATP,
            processes: vec![
                "MsSense.exe",
                "SenseCncProxy.exe",
                "SenseIR.exe",
                "SenseSampleUploader.exe",
            ],
            services: vec!["Sense", "WdNisSvc"],
            drivers: vec!["MsSecFlt.sys", "SenseSC.sys"],
            registry_keys: vec![
                r"HKLM\SOFTWARE\Microsoft\Windows Advanced Threat Protection",
            ],
            files: vec![r"C:\Program Files\Windows Defender Advanced Threat Protection"],
            dlls: vec![],
            pipes: vec![],
        },
        // Carbon Black
        EdrSignature {
            edr_type: EdrType::CarbonBlack,
            processes: vec![
                "cb.exe",
                "cbcomms.exe",
                "cbdefense.exe",
                "RepMgr.exe",
                "RepUtils.exe",
                "RepWAV.exe",
                "RepWSC.exe",
            ],
            services: vec!["CbDefense", "CbDefenseWSC", "carbonblack", "Cb Defense"],
            drivers: vec!["cbk7.sys", "ctifile.sys", "ctinet.sys"],
            registry_keys: vec![
                r"HKLM\SOFTWARE\CarbonBlack",
                r"HKLM\SOFTWARE\VMware Carbon Black",
            ],
            files: vec![
                r"C:\Program Files\Confer",
                r"C:\Program Files (x86)\CarbonBlack",
            ],
            dlls: vec!["cbdll.dll"],
            pipes: vec![r"\\.\pipe\ProtectedFileAccess"],
        },
        // Cylance
        EdrSignature {
            edr_type: EdrType::Cylance,
            processes: vec![
                "CylanceSvc.exe",
                "CylanceUI.exe",
                "CylanceProtectSetup.exe",
            ],
            services: vec!["CylanceSvc"],
            drivers: vec!["CyProtectDrv64.sys", "CyDevFlt64.sys"],
            registry_keys: vec![r"HKLM\SOFTWARE\Cylance"],
            files: vec![r"C:\Program Files\Cylance"],
            dlls: vec!["CylanceProtect.dll"],
            pipes: vec![],
        },
        // Cybereason
        EdrSignature {
            edr_type: EdrType::Cybereason,
            processes: vec![
                "CybereasonActiveProbe.exe",
                "CybereasonCRS.exe",
                "minionhost.exe",
            ],
            services: vec!["CybereasonActiveProbe", "CybereasonCRS"],
            drivers: vec!["crabortracer.sys"],
            registry_keys: vec![r"HKLM\SOFTWARE\Cybereason"],
            files: vec![r"C:\Program Files\Cybereason ActiveProbe"],
            dlls: vec![],
            pipes: vec![],
        },
        // Sophos
        EdrSignature {
            edr_type: EdrType::Sophos,
            processes: vec![
                "SavService.exe",
                "ALsvc.exe",
                "SophosFS.exe",
                "SophosHealth.exe",
                "SSPService.exe",
                "SophosNtpService.exe",
            ],
            services: vec!["SAVService", "SAVAdminService", "SophosFS"],
            drivers: vec!["savonaccess.sys", "sophosbootdriver.sys"],
            registry_keys: vec![
                r"HKLM\SOFTWARE\Sophos",
                r"HKLM\SOFTWARE\WOW6432Node\Sophos",
            ],
            files: vec![
                r"C:\Program Files\Sophos",
                r"C:\Program Files (x86)\Sophos",
            ],
            dlls: vec!["sophos_if.dll"],
            pipes: vec![],
        },
        // Kaspersky
        EdrSignature {
            edr_type: EdrType::Kaspersky,
            processes: vec!["avp.exe", "avpui.exe", "ksde.exe", "ksdecas.exe"],
            services: vec!["AVP", "klvssbrigde64", "KLIF"],
            drivers: vec!["klif.sys", "klhk.sys", "klkbdflt.sys", "kltdi.sys"],
            registry_keys: vec![r"HKLM\SOFTWARE\KasperskyLab"],
            files: vec![r"C:\Program Files (x86)\Kaspersky Lab"],
            dlls: vec!["klsihk.dll"],
            pipes: vec![],
        },
        // ESET
        EdrSignature {
            edr_type: EdrType::ESET,
            processes: vec!["ekrn.exe", "egui.exe", "eguiProxy.exe", "esets_gui.exe"],
            services: vec!["ekrn", "EraAgentSvc"],
            drivers: vec!["eamonm.sys", "edevmon.sys", "ehdrv.sys"],
            registry_keys: vec![r"HKLM\SOFTWARE\ESET"],
            files: vec![r"C:\Program Files\ESET"],
            dlls: vec!["esets_hips.dll"],
            pipes: vec![],
        },
        // McAfee
        EdrSignature {
            edr_type: EdrType::McAfee,
            processes: vec![
                "mcshield.exe",
                "mfetp.exe",
                "mfemms.exe",
                "mfevtps.exe",
                "mfefire.exe",
            ],
            services: vec!["McShield", "McAfeeFramework", "mfefire", "mfevtp"],
            drivers: vec!["mfefirek.sys", "mfehidk.sys", "mfencbdc.sys"],
            registry_keys: vec![r"HKLM\SOFTWARE\McAfee"],
            files: vec![
                r"C:\Program Files\McAfee",
                r"C:\Program Files\Common Files\McAfee",
            ],
            dlls: vec!["mfehida.dll"],
            pipes: vec![],
        },
        // Symantec
        EdrSignature {
            edr_type: EdrType::Symantec,
            processes: vec!["ccSvcHst.exe", "Rtvscan.exe", "smc.exe", "SmcGui.exe"],
            services: vec!["SepMasterService", "SmcService", "SNAC"],
            drivers: vec!["symefasi64.sys", "symevent64x86.sys", "srtsp64.sys"],
            registry_keys: vec![
                r"HKLM\SOFTWARE\Symantec",
                r"HKLM\SOFTWARE\Norton",
            ],
            files: vec![r"C:\Program Files\Symantec"],
            dlls: vec!["ccVrTrst.dll"],
            pipes: vec![],
        },
        // Bitdefender
        EdrSignature {
            edr_type: EdrType::Bitdefender,
            processes: vec![
                "bdagent.exe",
                "bdservicehost.exe",
                "bdredline.exe",
                "vsserv.exe",
            ],
            services: vec!["VSSERV", "BDAuxSrv", "BDRedline"],
            drivers: vec!["bdselfpr.sys", "bdfndisf6.sys", "bdfwfpf.sys"],
            registry_keys: vec![r"HKLM\SOFTWARE\Bitdefender"],
            files: vec![r"C:\Program Files\Bitdefender"],
            dlls: vec!["bdhook64.dll"],
            pipes: vec![],
        },
        // Elastic Security
        EdrSignature {
            edr_type: EdrType::Elastic,
            processes: vec![
                "elastic-agent.exe",
                "elastic-endpoint.exe",
                "filebeat.exe",
                "winlogbeat.exe",
            ],
            services: vec!["Elastic Agent", "Elastic Endpoint"],
            drivers: vec!["ElasticEndpoint.sys"],
            registry_keys: vec![r"HKLM\SOFTWARE\Elastic"],
            files: vec![r"C:\Program Files\Elastic"],
            dlls: vec![],
            pipes: vec![],
        },
        // Trend Micro
        EdrSignature {
            edr_type: EdrType::TrendMicro,
            processes: vec![
                "TMBMSRV.exe",
                "PccNTMon.exe",
                "TmListen.exe",
                "TmCCSF.exe",
            ],
            services: vec!["TrendMicro", "TmFilter", "TmCCSF"],
            drivers: vec!["tmactmon.sys", "tmcomm.sys", "tmevtmgr.sys"],
            registry_keys: vec![r"HKLM\SOFTWARE\TrendMicro"],
            files: vec![r"C:\Program Files\Trend Micro"],
            dlls: vec!["tmmon.dll"],
            pipes: vec![],
        },
    ]
}

/// Detection method used to find EDR
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetectionMethod {
    /// Scan running processes
    ProcessScan,
    /// Scan installed services
    ServiceScan,
    /// Scan loaded drivers
    DriverScan,
    /// Scan registry keys
    RegistryScan,
    /// Scan file system
    FileScan,
    /// Scan loaded DLLs
    DllScan,
    /// Scan named pipes
    PipeScan,
    /// WMI query
    WmiQuery,
    /// Check for ntdll hooks
    NtdllHookScan,
}

impl DetectionMethod {
    /// Get noise level (how likely to trigger detection, 1-10)
    pub fn noise_level(&self) -> u8 {
        match self {
            Self::ProcessScan => 2,
            Self::ServiceScan => 2,
            Self::DriverScan => 3,
            Self::FileScan => 4,
            Self::DllScan => 3,
            Self::PipeScan => 3,
            Self::RegistryScan => 5,
            Self::WmiQuery => 6,
            Self::NtdllHookScan => 7,
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::ProcessScan => "Scan running processes",
            Self::ServiceScan => "Scan Windows services",
            Self::DriverScan => "Scan loaded drivers",
            Self::RegistryScan => "Scan registry keys",
            Self::FileScan => "Scan file system paths",
            Self::DllScan => "Scan loaded DLLs",
            Self::PipeScan => "Scan named pipes",
            Self::WmiQuery => "WMI query",
            Self::NtdllHookScan => "Scan for ntdll hooks",
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edr_type_threat_levels() {
        assert_eq!(EdrType::CrowdStrike.threat_level(), 10);
        assert_eq!(EdrType::SentinelOne.threat_level(), 10);
        assert_eq!(EdrType::WindowsDefender.threat_level(), 6);
        assert!(EdrType::CrowdStrike.threat_level() > EdrType::Norton.threat_level());
    }

    #[test]
    fn test_edr_type_vendor() {
        assert_eq!(EdrType::CrowdStrike.vendor(), "CrowdStrike");
        assert_eq!(EdrType::WindowsDefender.vendor(), "Microsoft");
        assert_eq!(EdrType::DefenderATP.vendor(), "Microsoft");
    }

    #[test]
    fn test_is_enterprise_edr() {
        assert!(EdrType::CrowdStrike.is_enterprise_edr());
        assert!(EdrType::SentinelOne.is_enterprise_edr());
        assert!(!EdrType::WindowsDefender.is_enterprise_edr());
        assert!(!EdrType::Norton.is_enterprise_edr());
    }

    #[test]
    fn test_get_all_signatures() {
        let signatures = get_all_signatures();
        assert!(!signatures.is_empty());
        assert!(signatures.len() >= 15);

        // Check that each signature has at least one detection method
        for sig in &signatures {
            let has_detection = !sig.processes.is_empty()
                || !sig.services.is_empty()
                || !sig.drivers.is_empty();
            assert!(has_detection, "Signature for {:?} has no detection methods", sig.edr_type);
        }
    }

    #[test]
    fn test_detection_method_noise_levels() {
        assert!(DetectionMethod::ProcessScan.noise_level() < DetectionMethod::RegistryScan.noise_level());
        assert!(DetectionMethod::RegistryScan.noise_level() < DetectionMethod::NtdllHookScan.noise_level());
    }
}
