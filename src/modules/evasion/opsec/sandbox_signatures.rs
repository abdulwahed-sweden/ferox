//! Sandbox Detection Signatures
//!
//! Contains signatures for malware analysis sandboxes.
//!
//! MITRE ATT&CK: T1497 (Virtualization/Sandbox Evasion)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};

/// Known sandbox types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SandboxType {
    // Open source / Community
    /// Cuckoo Sandbox
    CuckooSandbox,
    /// CAPE Sandbox (Cuckoo fork)
    CAPEv2,
    /// Drakvuf
    Drakvuf,

    // Commercial / Online
    /// Joe Sandbox
    JoeSandbox,
    /// ANY.RUN
    AnyRun,
    /// Hybrid Analysis
    HybridAnalysis,
    /// VirusTotal
    VirusTotal,
    /// Intezer Analyze
    Intezer,
    /// Hatching Triage
    Hatching,

    // Enterprise
    /// FireEye
    FireEye,
    /// Palo Alto WildFire
    PaloAltoWildfire,
    /// CrowdStrike Falcon X
    CrowdStrikeFalconX,
    /// Symantec Content Analysis
    SymantecContentAnalysis,

    // Windows Built-in
    /// Windows Sandbox
    WindowsSandbox,
    /// App Container
    AppContainer,

    /// Unknown sandbox
    Unknown,
}

impl SandboxType {
    /// Get sandbox vendor/name
    pub fn name(&self) -> &'static str {
        match self {
            Self::CuckooSandbox => "Cuckoo Sandbox",
            Self::JoeSandbox => "Joe Sandbox",
            Self::AnyRun => "ANY.RUN",
            Self::HybridAnalysis => "Hybrid Analysis",
            Self::VirusTotal => "VirusTotal",
            Self::Intezer => "Intezer",
            Self::Hatching => "Hatching Triage",
            Self::FireEye => "FireEye",
            Self::PaloAltoWildfire => "Palo Alto WildFire",
            Self::CrowdStrikeFalconX => "CrowdStrike Falcon X",
            Self::SymantecContentAnalysis => "Symantec Content Analysis",
            Self::WindowsSandbox => "Windows Sandbox",
            Self::AppContainer => "App Container",
            Self::CAPEv2 => "CAPEv2",
            Self::Drakvuf => "Drakvuf",
            Self::Unknown => "Unknown Sandbox",
        }
    }

    /// Get sandbox category
    pub fn category(&self) -> &'static str {
        match self {
            Self::CuckooSandbox | Self::CAPEv2 | Self::Drakvuf => "Open Source",
            Self::JoeSandbox | Self::AnyRun | Self::HybridAnalysis | Self::VirusTotal
            | Self::Intezer | Self::Hatching => "Commercial",
            Self::FireEye | Self::PaloAltoWildfire | Self::CrowdStrikeFalconX
            | Self::SymantecContentAnalysis => "Enterprise",
            Self::WindowsSandbox | Self::AppContainer => "Built-in",
            Self::Unknown => "Unknown",
        }
    }

    /// Get evasion priority (1-10, higher = more important to evade)
    pub fn priority(&self) -> u8 {
        match self {
            Self::VirusTotal => 10,
            Self::AnyRun => 9,
            Self::JoeSandbox => 9,
            Self::CuckooSandbox => 8,
            Self::HybridAnalysis => 8,
            Self::FireEye => 9,
            Self::PaloAltoWildfire => 8,
            Self::CrowdStrikeFalconX => 9,
            Self::CAPEv2 => 7,
            Self::Intezer => 7,
            Self::Hatching => 7,
            Self::Drakvuf => 6,
            Self::WindowsSandbox => 5,
            Self::AppContainer => 4,
            Self::SymantecContentAnalysis => 7,
            Self::Unknown => 6,
        }
    }

    /// Is this a public/online sandbox?
    pub fn is_public(&self) -> bool {
        matches!(
            self,
            Self::AnyRun | Self::HybridAnalysis | Self::VirusTotal | Self::Hatching
        )
    }
}

/// Sandbox detection signature
#[derive(Debug, Clone)]
pub struct SandboxSignature {
    /// Type of sandbox
    pub sandbox_type: SandboxType,
    /// Process names to look for
    pub processes: &'static [&'static str],
    /// File paths to check
    pub files: &'static [&'static str],
    /// Suspicious usernames
    pub usernames: &'static [&'static str],
    /// Suspicious computer names
    pub computer_names: &'static [&'static str],
    /// DLLs to check
    pub dlls: &'static [&'static str],
    /// Registry keys to check
    pub registry_keys: &'static [&'static str],
    /// Mutex names to check
    pub mutex_names: &'static [&'static str],
}

/// Get all sandbox signatures
pub fn get_sandbox_signatures() -> Vec<SandboxSignature> {
    vec![
        // Cuckoo Sandbox
        SandboxSignature {
            sandbox_type: SandboxType::CuckooSandbox,
            processes: &["agent.py", "analyzer.py", "python.exe"],
            files: &[r"C:\analysis", r"C:\cuckoo", r"C:\logs\analysis.log"],
            usernames: &["cuckoo", "sandbox", "malware", "virus", "sample"],
            computer_names: &["CUCKOO", "SANDBOX", "ANALYSIS"],
            dlls: &["cuckoomon.dll", "monitor-x64.dll"],
            registry_keys: &[],
            mutex_names: &["CuckooAnalysis"],
        },
        // Joe Sandbox
        SandboxSignature {
            sandbox_type: SandboxType::JoeSandbox,
            processes: &["joeboxcontrol.exe", "joeboxserver.exe"],
            files: &[r"C:\joe", r"C:\jbox"],
            usernames: &["joe", "joebox"],
            computer_names: &["JOEBOX", "JOE-PC"],
            dlls: &["SbieDll.dll"],
            registry_keys: &[r"HKLM\SOFTWARE\Joe Security"],
            mutex_names: &["JoeBox"],
        },
        // ANY.RUN
        SandboxSignature {
            sandbox_type: SandboxType::AnyRun,
            processes: &[],
            files: &[r"C:\anyrun"],
            usernames: &["anyrun", "user"],
            computer_names: &["ANYRUN"],
            dlls: &[],
            registry_keys: &[],
            mutex_names: &[],
        },
        // Hybrid Analysis
        SandboxSignature {
            sandbox_type: SandboxType::HybridAnalysis,
            processes: &["sample.exe"],
            files: &[r"C:\analysis", r"C:\sample"],
            usernames: &["hybrid", "analysis"],
            computer_names: &["HYBRID", "HA-DESKTOP"],
            dlls: &[],
            registry_keys: &[],
            mutex_names: &[],
        },
        // VirusTotal
        SandboxSignature {
            sandbox_type: SandboxType::VirusTotal,
            processes: &[],
            files: &[],
            usernames: &["vt", "virustotal"],
            computer_names: &["VT-DESKTOP"],
            dlls: &[],
            registry_keys: &[],
            mutex_names: &[],
        },
        // FireEye
        SandboxSignature {
            sandbox_type: SandboxType::FireEye,
            processes: &["fireeyed.exe"],
            files: &[r"C:\fireeye"],
            usernames: &["fireeye"],
            computer_names: &["FIREEYE"],
            dlls: &[],
            registry_keys: &[r"HKLM\SOFTWARE\FireEye"],
            mutex_names: &[],
        },
        // CAPEv2
        SandboxSignature {
            sandbox_type: SandboxType::CAPEv2,
            processes: &["analyzer.py", "monitor.py"],
            files: &[r"C:\cape", r"C:\analysis"],
            usernames: &["cape"],
            computer_names: &["CAPE"],
            dlls: &["capemon.dll", "capemon_x64.dll"],
            registry_keys: &[],
            mutex_names: &["CAPEAnalysis"],
        },
        // Windows Sandbox
        SandboxSignature {
            sandbox_type: SandboxType::WindowsSandbox,
            processes: &["WindowsSandbox.exe"],
            files: &[r"C:\Windows\Containers"],
            usernames: &["WDAGUtilityAccount"],
            computer_names: &["DESKTOP-"],
            dlls: &[],
            registry_keys: &[
                r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Containers",
            ],
            mutex_names: &[],
        },
        // Intezer
        SandboxSignature {
            sandbox_type: SandboxType::Intezer,
            processes: &[],
            files: &[],
            usernames: &["intezer"],
            computer_names: &["INTEZER"],
            dlls: &[],
            registry_keys: &[],
            mutex_names: &[],
        },
        // Hatching Triage
        SandboxSignature {
            sandbox_type: SandboxType::Hatching,
            processes: &[],
            files: &[],
            usernames: &["triage", "hatching"],
            computer_names: &["TRIAGE"],
            dlls: &[],
            registry_keys: &[],
            mutex_names: &[],
        },
    ]
}

/// Common analysis-related artifacts
pub struct AnalysisArtifacts;

impl AnalysisArtifacts {
    /// Common analysis tool processes
    pub fn analysis_processes() -> &'static [&'static str] {
        &[
            // Debuggers
            "ollydbg.exe",
            "x64dbg.exe",
            "x32dbg.exe",
            "idaq.exe",
            "idaq64.exe",
            "windbg.exe",
            "immunitydebugger.exe",
            "ida.exe",
            "ida64.exe",
            // Network analyzers
            "wireshark.exe",
            "tcpdump.exe",
            "fiddler.exe",
            "charles.exe",
            "burpsuite.exe",
            // Process monitors
            "procmon.exe",
            "procmon64.exe",
            "procexp.exe",
            "procexp64.exe",
            "processhacker.exe",
            "pestudio.exe",
            // Malware analysis
            "regshot.exe",
            "autoruns.exe",
            "fakenet.exe",
            "apimonitor.exe",
            // System tools
            "resourcehacker.exe",
            "hxd.exe",
            "dumpbin.exe",
            // Sysinternals
            "tcpview.exe",
            "strings.exe",
            "handle.exe",
            "listdlls.exe",
        ]
    }

    /// Common analysis-related DLLs
    pub fn analysis_dlls() -> &'static [&'static str] {
        &[
            "sbiedll.dll",   // Sandboxie
            "dbghelp.dll",   // Debug helper
            "api_log.dll",   // API logging
            "dir_watch.dll", // Directory watch
            "pstorec.dll",   // Password storage
            "vmcheck.dll",   // VM check
            "cmdvrt32.dll",  // Comodo
            "cmdvrt64.dll",
            "SxIn.dll",       // 360 sandbox
            "Sf2.dll",        // Avast sandbox
            "snxhk.dll",      // Avast
            "cuckoomon.dll",  // Cuckoo
            "capemon.dll",    // CAPE
        ]
    }

    /// Suspicious usernames (commonly used in analysis environments)
    pub fn suspicious_usernames() -> &'static [&'static str] {
        &[
            "malware",
            "sandbox",
            "virus",
            "sample",
            "test",
            "analysis",
            "cuckoo",
            "joe",
            "admin",
            "user",
            "analyst",
            "maltest",
            "lab",
            "vmware",
            "vbox",
            "qemu",
            "currentuser",
            "john",
            "peter",
            "honey",
            "sand",
            "emily",
            "wilbert",
        ]
    }

    /// Suspicious computer names
    pub fn suspicious_computer_names() -> &'static [&'static str] {
        &[
            "SANDBOX",
            "ANALYSIS",
            "MALWARE",
            "SAMPLE",
            "VIRUS",
            "TEST",
            "CUCKOO",
            "CAPE",
            "HYBRID",
            "JOE",
            "ANYRUN",
            "LAB",
            "VMWARE",
            "VBOX",
            "QEMU",
            "PC",
            "USER-PC",
            "WIN7",
            "WIN10",
            "DESKTOP-",
            "HAPPYPC",
            "SANDBOXIE",
        ]
    }

    /// Low entropy file counts (sandboxes often have few files)
    pub fn min_expected_desktop_files() -> usize {
        5
    }

    /// Minimum expected recent documents
    pub fn min_expected_recent_files() -> usize {
        10
    }

    /// Minimum expected uptime in minutes
    pub fn min_expected_uptime_minutes() -> u64 {
        30
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_signatures_not_empty() {
        let sigs = get_sandbox_signatures();
        assert!(!sigs.is_empty());
        assert!(sigs.len() >= 8);
    }

    #[test]
    fn test_sandbox_type_name() {
        assert_eq!(SandboxType::CuckooSandbox.name(), "Cuckoo Sandbox");
        assert_eq!(SandboxType::VirusTotal.name(), "VirusTotal");
    }

    #[test]
    fn test_sandbox_type_category() {
        assert_eq!(SandboxType::CuckooSandbox.category(), "Open Source");
        assert_eq!(SandboxType::JoeSandbox.category(), "Commercial");
        assert_eq!(SandboxType::FireEye.category(), "Enterprise");
        assert_eq!(SandboxType::WindowsSandbox.category(), "Built-in");
    }

    #[test]
    fn test_sandbox_type_priority() {
        assert_eq!(SandboxType::VirusTotal.priority(), 10);
        assert!(SandboxType::AnyRun.priority() >= 8);
    }

    #[test]
    fn test_sandbox_is_public() {
        assert!(SandboxType::AnyRun.is_public());
        assert!(SandboxType::VirusTotal.is_public());
        assert!(!SandboxType::CuckooSandbox.is_public());
        assert!(!SandboxType::FireEye.is_public());
    }

    #[test]
    fn test_analysis_processes() {
        let processes = AnalysisArtifacts::analysis_processes();
        assert!(!processes.is_empty());
        assert!(processes.contains(&"wireshark.exe"));
        assert!(processes.contains(&"x64dbg.exe"));
    }

    #[test]
    fn test_analysis_dlls() {
        let dlls = AnalysisArtifacts::analysis_dlls();
        assert!(!dlls.is_empty());
        assert!(dlls.contains(&"sbiedll.dll"));
    }

    #[test]
    fn test_suspicious_usernames() {
        let usernames = AnalysisArtifacts::suspicious_usernames();
        assert!(!usernames.is_empty());
        assert!(usernames.contains(&"sandbox"));
        assert!(usernames.contains(&"malware"));
    }

    #[test]
    fn test_suspicious_computer_names() {
        let names = AnalysisArtifacts::suspicious_computer_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"SANDBOX"));
        assert!(names.contains(&"CUCKOO"));
    }

    #[test]
    fn test_thresholds() {
        assert!(AnalysisArtifacts::min_expected_desktop_files() > 0);
        assert!(AnalysisArtifacts::min_expected_recent_files() > 0);
        assert!(AnalysisArtifacts::min_expected_uptime_minutes() > 0);
    }
}
