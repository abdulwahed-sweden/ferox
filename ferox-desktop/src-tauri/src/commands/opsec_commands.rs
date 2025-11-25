//! OPSEC Commands - Operational Security monitoring and countermeasures
//!
//! Provides OPSEC status checking, threat detection, and countermeasure management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;

// =============================================================================
// Types
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threat {
    pub id: String,
    pub category: String,       // network, process, behavioral, forensic, detection
    pub severity: String,       // low, medium, high, critical
    pub title: String,
    pub description: String,
    pub mitigation: String,
    pub timestamp: String,
    pub source: String,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Countermeasure {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub enabled: bool,
    pub risk_reduction: u8,     // 0-100
    pub performance_impact: String,  // none, low, medium, high
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentAnalysis {
    pub hostname: String,
    pub os: String,
    pub os_version: String,
    pub is_vm: bool,
    pub vm_type: Option<String>,
    pub is_sandbox: bool,
    pub sandbox_indicators: Vec<String>,
    pub av_detected: Vec<String>,
    pub edr_detected: Vec<String>,
    pub monitoring_tools: Vec<String>,
    pub network_monitoring: bool,
    pub debug_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsecStatus {
    pub score: u8,              // 0-100
    pub threat_level: String,   // low, medium, high, critical
    pub active_countermeasures: Vec<Countermeasure>,
    pub detected_threats: Vec<Threat>,
    pub recommendations: Vec<String>,
    pub last_check: String,
    pub environment: EnvironmentAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficAnalysis {
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub connections_active: u32,
    pub suspicious_patterns: Vec<String>,
    pub beacon_detected: bool,
    pub exfil_risk: String,     // low, medium, high, critical
}

// =============================================================================
// Global State (Countermeasure activation state)
// =============================================================================

lazy_static::lazy_static! {
    static ref COUNTERMEASURES: std::sync::RwLock<HashMap<String, bool>> = {
        let mut m = HashMap::new();
        m.insert("cm-001".to_string(), true);   // Traffic Encryption - enabled by default
        m.insert("cm-002".to_string(), false);  // Process Hollowing
        m.insert("cm-003".to_string(), false);  // AMSI Bypass
        m.insert("cm-004".to_string(), false);  // ETW Patching
        m.insert("cm-005".to_string(), false);  // Domain Fronting
        m.insert("cm-006".to_string(), false);  // Memory-only execution
        m.insert("cm-007".to_string(), false);  // Log cleanup
        std::sync::RwLock::new(m)
    };
}

// =============================================================================
// Environment Detection
// =============================================================================

/// Detect VM/Sandbox indicators
fn detect_vm() -> (bool, Option<String>) {
    let mut vm_type = None;
    let mut is_vm = false;

    // Check common VM indicators
    #[cfg(target_os = "windows")]
    {
        // Check for VM-specific registry keys, processes, etc.
        // This is simplified - real implementation would check:
        // - Registry: HKLM\SOFTWARE\VMware, HKLM\SOFTWARE\Oracle\VirtualBox
        // - MAC addresses with VM prefixes
        // - CPUID hypervisor bit
        // - Disk names containing "VBOX", "VMWARE"
    }

    #[cfg(target_os = "linux")]
    {
        // Check systemd-detect-virt or dmesg
        if let Ok(output) = std::process::Command::new("systemd-detect-virt").output() {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !result.is_empty() && result != "none" {
                is_vm = true;
                vm_type = Some(result);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Check for VM-specific hardware
        if let Ok(output) = std::process::Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
        {
            let cpu = String::from_utf8_lossy(&output.stdout).to_lowercase();
            if cpu.contains("virtual") || cpu.contains("vmware") || cpu.contains("qemu") {
                is_vm = true;
                vm_type = Some("VMware/QEMU".to_string());
            }
        }
    }

    (is_vm, vm_type)
}

/// Detect AV/EDR products
fn detect_security_products() -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut av = Vec::new();
    let edr = Vec::new();
    let monitoring = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // Check for running AV/EDR processes
        let av_processes = [
            ("MsMpEng.exe", "Windows Defender"),
            ("avp.exe", "Kaspersky"),
            ("avgnt.exe", "Avira"),
            ("mbam.exe", "Malwarebytes"),
        ];

        let edr_processes = [
            ("CrowdStrike", "CrowdStrike Falcon"),
            ("cb.exe", "Carbon Black"),
            ("SentinelAgent", "SentinelOne"),
            ("Tanium", "Tanium"),
        ];

        let monitoring_processes = [
            ("Sysmon.exe", "Sysmon"),
            ("Procmon.exe", "Process Monitor"),
            ("Wireshark.exe", "Wireshark"),
        ];

        // Would iterate processes and check - simplified
        av.push("Windows Defender".to_string());  // Assume always present on Windows
    }

    #[cfg(target_os = "linux")]
    {
        // Check for common Linux security tools
        let tools = ["clamav", "rkhunter", "chkrootkit", "auditd", "ossec"];
        for tool in tools {
            if std::process::Command::new("which")
                .arg(tool)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                monitoring.push(tool.to_string());
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // XProtect is always present on macOS
        av.push("XProtect".to_string());
    }

    (av, edr, monitoring)
}

/// Get system information
fn get_system_info() -> (String, String, String) {
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let os = std::env::consts::OS.to_string();

    let os_version = if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else if cfg!(target_os = "linux") {
        std::process::Command::new("uname")
            .arg("-r")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Linux".to_string())
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "macOS".to_string())
    } else {
        "Unknown".to_string()
    };

    (hostname, os, os_version)
}

// =============================================================================
// Commands
// =============================================================================

/// Check overall OPSEC status
#[command]
pub async fn check_opsec() -> Result<OpsecStatus, String> {
    let (hostname, os, os_version) = get_system_info();
    let (is_vm, vm_type) = detect_vm();
    let (av, edr, monitoring) = detect_security_products();

    // Build environment analysis
    let environment = EnvironmentAnalysis {
        hostname,
        os: os.clone(),
        os_version,
        is_vm,
        vm_type,
        is_sandbox: false,  // Would need deeper analysis
        sandbox_indicators: Vec::new(),
        av_detected: av.clone(),
        edr_detected: edr.clone(),
        monitoring_tools: monitoring.clone(),
        network_monitoring: false,
        debug_mode: cfg!(debug_assertions),
    };

    // Get active countermeasures
    let cm_state = COUNTERMEASURES.read().unwrap();
    let active_countermeasures: Vec<Countermeasure> = get_all_countermeasures()
        .into_iter()
        .filter(|cm| *cm_state.get(&cm.id).unwrap_or(&false))
        .collect();

    // Calculate OPSEC score
    let mut score: u8 = 50;  // Base score

    // Reduce score for detected threats
    if !av.is_empty() {
        score = score.saturating_sub(10);
    }
    if !edr.is_empty() {
        score = score.saturating_sub(20);
    }
    if is_vm {
        score = score.saturating_sub(5);
    }

    // Increase score for active countermeasures
    for cm in &active_countermeasures {
        score = score.saturating_add(cm.risk_reduction / 5);
    }

    score = score.min(100);

    // Determine threat level
    let threat_level = match score {
        80..=100 => "low",
        60..=79 => "medium",
        40..=59 => "high",
        _ => "critical",
    };

    // Generate detected threats
    let mut threats = Vec::new();

    if !av.is_empty() {
        threats.push(Threat {
            id: uuid::Uuid::new_v4().to_string(),
            category: "detection".to_string(),
            severity: "medium".to_string(),
            title: "Antivirus Detected".to_string(),
            description: format!("Antivirus software detected: {}", av.join(", ")),
            mitigation: "Consider using evasion techniques or encrypted payloads".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            source: "environment_scan".to_string(),
            indicators: av.clone(),
        });
    }

    if !edr.is_empty() {
        threats.push(Threat {
            id: uuid::Uuid::new_v4().to_string(),
            category: "detection".to_string(),
            severity: "high".to_string(),
            title: "EDR Solution Detected".to_string(),
            description: format!("EDR product detected: {}", edr.join(", ")),
            mitigation: "Enable advanced evasion countermeasures".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            source: "environment_scan".to_string(),
            indicators: edr.clone(),
        });
    }

    // Generate recommendations
    let mut recommendations = Vec::new();

    if !cm_state.get("cm-001").unwrap_or(&false) {
        recommendations.push("Enable traffic encryption for C2 communication".to_string());
    }
    if !edr.is_empty() && !cm_state.get("cm-004").unwrap_or(&false) {
        recommendations.push("Consider enabling ETW patching to reduce telemetry".to_string());
    }
    if is_vm {
        recommendations.push("Running in VM - some detection may be easier".to_string());
    }

    Ok(OpsecStatus {
        score,
        threat_level: threat_level.to_string(),
        active_countermeasures,
        detected_threats: threats,
        recommendations,
        last_check: chrono::Utc::now().to_rfc3339(),
        environment,
    })
}

/// Get all available countermeasures
fn get_all_countermeasures() -> Vec<Countermeasure> {
    let cm_state = COUNTERMEASURES.read().unwrap();

    vec![
        Countermeasure {
            id: "cm-001".to_string(),
            name: "Traffic Encryption".to_string(),
            description: "Encrypt all C2 traffic with AES-256-GCM".to_string(),
            category: "network".to_string(),
            enabled: *cm_state.get("cm-001").unwrap_or(&false),
            risk_reduction: 25,
            performance_impact: "low".to_string(),
        },
        Countermeasure {
            id: "cm-002".to_string(),
            name: "Process Hollowing".to_string(),
            description: "Hide implant inside legitimate process memory".to_string(),
            category: "evasion".to_string(),
            enabled: *cm_state.get("cm-002").unwrap_or(&false),
            risk_reduction: 30,
            performance_impact: "medium".to_string(),
        },
        Countermeasure {
            id: "cm-003".to_string(),
            name: "AMSI Bypass".to_string(),
            description: "Bypass Windows AMSI for script execution".to_string(),
            category: "evasion".to_string(),
            enabled: *cm_state.get("cm-003").unwrap_or(&false),
            risk_reduction: 20,
            performance_impact: "low".to_string(),
        },
        Countermeasure {
            id: "cm-004".to_string(),
            name: "ETW Patching".to_string(),
            description: "Disable Event Tracing for Windows telemetry".to_string(),
            category: "forensic".to_string(),
            enabled: *cm_state.get("cm-004").unwrap_or(&false),
            risk_reduction: 15,
            performance_impact: "none".to_string(),
        },
        Countermeasure {
            id: "cm-005".to_string(),
            name: "Domain Fronting".to_string(),
            description: "Use CDN domain fronting for C2 masquerading".to_string(),
            category: "network".to_string(),
            enabled: *cm_state.get("cm-005").unwrap_or(&false),
            risk_reduction: 35,
            performance_impact: "medium".to_string(),
        },
        Countermeasure {
            id: "cm-006".to_string(),
            name: "Memory-Only Execution".to_string(),
            description: "Execute payloads entirely in memory".to_string(),
            category: "forensic".to_string(),
            enabled: *cm_state.get("cm-006").unwrap_or(&false),
            risk_reduction: 25,
            performance_impact: "low".to_string(),
        },
        Countermeasure {
            id: "cm-007".to_string(),
            name: "Log Cleanup".to_string(),
            description: "Automatically clean relevant log entries".to_string(),
            category: "forensic".to_string(),
            enabled: *cm_state.get("cm-007").unwrap_or(&false),
            risk_reduction: 20,
            performance_impact: "low".to_string(),
        },
    ]
}

/// Get all countermeasures
#[command]
pub async fn get_countermeasures() -> Result<Vec<Countermeasure>, String> {
    Ok(get_all_countermeasures())
}

/// Activate a countermeasure
#[command]
pub async fn activate_countermeasure(id: String) -> Result<bool, String> {
    let mut cm_state = COUNTERMEASURES.write().unwrap();

    if cm_state.contains_key(&id) {
        cm_state.insert(id.clone(), true);
        tracing::info!("Activated countermeasure: {}", id);
        Ok(true)
    } else {
        Err(format!("Unknown countermeasure: {}", id))
    }
}

/// Deactivate a countermeasure
#[command]
pub async fn deactivate_countermeasure(id: String) -> Result<bool, String> {
    let mut cm_state = COUNTERMEASURES.write().unwrap();

    if cm_state.contains_key(&id) {
        cm_state.insert(id.clone(), false);
        tracing::info!("Deactivated countermeasure: {}", id);
        Ok(true)
    } else {
        Err(format!("Unknown countermeasure: {}", id))
    }
}

/// Analyze current environment for threats
#[command]
pub async fn analyze_environment() -> Result<EnvironmentAnalysis, String> {
    let (hostname, os, os_version) = get_system_info();
    let (is_vm, vm_type) = detect_vm();
    let (av, edr, monitoring) = detect_security_products();

    Ok(EnvironmentAnalysis {
        hostname,
        os,
        os_version,
        is_vm,
        vm_type,
        is_sandbox: false,
        sandbox_indicators: Vec::new(),
        av_detected: av,
        edr_detected: edr,
        monitoring_tools: monitoring,
        network_monitoring: false,
        debug_mode: cfg!(debug_assertions),
    })
}

/// Analyze network traffic patterns
#[command]
pub async fn analyze_traffic() -> Result<TrafficAnalysis, String> {
    // This would analyze actual network traffic in real implementation
    // For now, return simulated metrics

    Ok(TrafficAnalysis {
        total_bytes_sent: rand::random::<u64>() % (1024 * 1024 * 100),
        total_bytes_received: rand::random::<u64>() % (1024 * 1024 * 500),
        connections_active: (rand::random::<u32>() % 10) + 1,
        suspicious_patterns: Vec::new(),
        beacon_detected: false,
        exfil_risk: "low".to_string(),
    })
}
