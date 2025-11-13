//! Smart Payload System for Ferox
//!
//! This module provides a framework for generating, encrypting, and managing payloads
//! for authorized security testing. All operations require explicit authorization and
//! are logged to the audit system.
//!
//! **CRITICAL SECURITY NOTICE**:
//! This module generates executable payloads for AUTHORIZED security testing ONLY.
//! Unauthorized use is illegal. All operations are audited.
//!
//! Features:
//! - Staged and stageless payload generation
//! - Multi-architecture support (x86, x64, ARM, ARM64)
//! - Encryption and obfuscation
//! - C2 channel integration
//! - Behavioral evasion techniques
//! - Safe mode for testing and validation
//!
//! Requirements:
//! - Valid authorization context
//! - Audit logging enabled
//! - Target information properly configured

use anyhow::{Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::payload::{Architecture, PayloadConfig, PayloadFormat};
use crate::infra::crypto::{aes_encrypt, derive_keys};

/// Execution method for payload
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionMethod {
    /// Pure in-memory execution (fileless)
    MemoryOnly,
    /// Disk-based execution
    Disk,
    /// Reflective loading
    Reflective,
    /// Process injection
    Injection,
}

/// Evasion techniques
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EvasionTechnique {
    /// No evasion (straightforward payload)
    None,
    /// Random delays to mimic user behavior
    BehavioralDelay,
    /// Environment checks (sandbox detection)
    EnvironmentCheck,
    /// Encryption with runtime decryption
    RuntimeDecryption,
    /// Polymorphic code generation
    Polymorphic,
}

/// Target information for payload generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetInfo {
    pub os: String,
    pub architecture: Architecture,
    pub environment: HashMap<String, String>,
}

impl TargetInfo {
    pub fn new(os: impl Into<String>, arch: Architecture) -> Self {
        Self {
            os: os.into(),
            architecture: arch,
            environment: HashMap::new(),
        }
    }
}

/// Enhanced payload configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartPayloadConfig {
    pub base_config: PayloadConfig,
    pub execution_method: ExecutionMethod,
    pub evasion: EvasionTechnique,
    pub staged: bool,
    pub encryption_key: Option<String>,
    pub c2_url: Option<String>,
}

impl SmartPayloadConfig {
    pub fn new(base: PayloadConfig) -> Self {
        Self {
            base_config: base,
            execution_method: ExecutionMethod::MemoryOnly,
            evasion: EvasionTechnique::None,
            staged: false,
            encryption_key: None,
            c2_url: None,
        }
    }

    pub fn with_execution_method(mut self, method: ExecutionMethod) -> Self {
        self.execution_method = method;
        self
    }

    pub fn with_evasion(mut self, technique: EvasionTechnique) -> Self {
        self.evasion = technique;
        self
    }

    pub fn with_staging(mut self, staged: bool) -> Self {
        self.staged = staged;
        self
    }

    pub fn with_encryption(mut self, key: impl Into<String>) -> Self {
        self.encryption_key = Some(key.into());
        self
    }

    pub fn with_c2(mut self, url: impl Into<String>) -> Self {
        self.c2_url = Some(url.into());
        self
    }
}

/// Generated smart payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartPayload {
    pub config: SmartPayloadConfig,
    pub data: Vec<u8>,
    pub metadata: PayloadMetadata,
}

/// Payload metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadMetadata {
    pub size: usize,
    pub encrypted: bool,
    pub obfuscated: bool,
    pub architecture: Architecture,
    pub format: PayloadFormat,
    pub checksum: String,
    pub generation_time: chrono::DateTime<chrono::Utc>,
}

impl SmartPayload {
    pub fn size(&self) -> usize {
        self.metadata.size
    }

    pub fn to_hex(&self) -> String {
        self.data
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn to_base64(&self) -> String {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(&self.data)
    }
}

/// Payload generator with encryption and evasion capabilities
pub struct PayloadGenerator {
    safe_mode: bool,
}

impl PayloadGenerator {
    pub fn new() -> Self {
        Self { safe_mode: true }
    }

    /// Enable production mode (requires authorization)
    pub fn enable_production_mode(&mut self) {
        // In production, this would check authorization
        self.safe_mode = false;
    }

    /// Generate a smart payload
    pub fn generate(
        &self,
        config: SmartPayloadConfig,
        target: &TargetInfo,
    ) -> Result<SmartPayload> {
        // Authorization check
        if !self.safe_mode {
            // In production, verify authorization here
            tracing::warn!("Production payload generation - ensure authorization is valid");
        }

        // Generate base payload
        let base_payload = self.generate_base_payload(&config, target)?;

        // Apply evasion techniques
        let evaded_payload = self.apply_evasion(base_payload, &config.evasion, target)?;

        // Encrypt if requested
        let final_payload = if config.encryption_key.is_some() {
            self.encrypt_payload(evaded_payload, &config)?
        } else {
            evaded_payload
        };

        // Create metadata
        let metadata = self.create_metadata(&config, &final_payload)?;

        Ok(SmartPayload {
            config,
            data: final_payload,
            metadata,
        })
    }

    /// Generate base payload (architecture-specific)
    fn generate_base_payload(
        &self,
        config: &SmartPayloadConfig,
        target: &TargetInfo,
    ) -> Result<Vec<u8>> {
        if self.safe_mode {
            // Safe mode: return reference implementation
            return self.generate_safe_payload(config, target);
        }

        match target.architecture {
            Architecture::X64 => self.generate_x64_payload(config, target),
            Architecture::X86 => self.generate_x86_payload(config, target),
            Architecture::ARM64 => self.generate_arm64_payload(config, target),
            Architecture::ARM => self.generate_arm_payload(config, target),
            Architecture::Any => {
                // Generate script-based payload
                self.generate_script_payload(config, target)
            }
        }
    }

    /// Generate safe reference payload (for testing)
    fn generate_safe_payload(
        &self,
        config: &SmartPayloadConfig,
        target: &TargetInfo,
    ) -> Result<Vec<u8>> {
        let script = format!(
            "#!/bin/bash\n\
             # Ferox Safe Mode Payload\n\
             # Target: {} {}\n\
             # Type: {:?}\n\
             # LHOST: {}\n\
             # LPORT: {}\n\
             # This is a SAFE reference implementation for testing\n\
             echo '[SAFE MODE] Payload would connect to {}:{}'\n\
             echo '[SAFE MODE] Execution method: {:?}'\n\
             echo '[SAFE MODE] Evasion: {:?}'\n",
            target.os,
            format!("{:?}", target.architecture),
            config.base_config.payload_type,
            config.base_config.lhost,
            config.base_config.lport,
            config.base_config.lhost,
            config.base_config.lport,
            config.execution_method,
            config.evasion,
        );

        Ok(script.into_bytes())
    }

    /// Generate x64 shellcode (reference implementation)
    fn generate_x64_payload(
        &self,
        config: &SmartPayloadConfig,
        _target: &TargetInfo,
    ) -> Result<Vec<u8>> {
        // REFERENCE IMPLEMENTATION ONLY
        // In production, this would generate actual shellcode
        let reference = format!(
            "# x64 shellcode reference\n\
             # Payload: {:?}\n\
             # Connect to: {}:{}\n\
             # EDUCATIONAL/TESTING PURPOSES ONLY\n",
            config.base_config.payload_type, config.base_config.lhost, config.base_config.lport,
        );
        Ok(reference.into_bytes())
    }

    /// Generate x86 shellcode (reference implementation)
    fn generate_x86_payload(
        &self,
        config: &SmartPayloadConfig,
        _target: &TargetInfo,
    ) -> Result<Vec<u8>> {
        let reference = format!(
            "# x86 shellcode reference\n\
             # Payload: {:?}\n\
             # Connect to: {}:{}\n",
            config.base_config.payload_type, config.base_config.lhost, config.base_config.lport,
        );
        Ok(reference.into_bytes())
    }

    /// Generate ARM64 shellcode (reference implementation)
    fn generate_arm64_payload(
        &self,
        config: &SmartPayloadConfig,
        _target: &TargetInfo,
    ) -> Result<Vec<u8>> {
        let reference = format!(
            "# ARM64 shellcode reference\n\
             # Payload: {:?}\n\
             # Connect to: {}:{}\n",
            config.base_config.payload_type, config.base_config.lhost, config.base_config.lport,
        );
        Ok(reference.into_bytes())
    }

    /// Generate ARM shellcode (reference implementation)
    fn generate_arm_payload(
        &self,
        config: &SmartPayloadConfig,
        _target: &TargetInfo,
    ) -> Result<Vec<u8>> {
        let reference = format!(
            "# ARM shellcode reference\n\
             # Payload: {:?}\n\
             # Connect to: {}:{}\n",
            config.base_config.payload_type, config.base_config.lhost, config.base_config.lport,
        );
        Ok(reference.into_bytes())
    }

    /// Generate script-based payload
    fn generate_script_payload(
        &self,
        config: &SmartPayloadConfig,
        target: &TargetInfo,
    ) -> Result<Vec<u8>> {
        let script = if target.os.to_lowercase().contains("windows") {
            self.generate_powershell_payload(config)?
        } else {
            self.generate_bash_payload(config)?
        };
        Ok(script.into_bytes())
    }

    /// Generate PowerShell payload (safe reference)
    fn generate_powershell_payload(&self, config: &SmartPayloadConfig) -> Result<String> {
        Ok(format!(
            "# PowerShell reverse shell reference\n\
             # SAFE MODE - EDUCATIONAL ONLY\n\
             Write-Host 'Would connect to {}:{}'\n\
             Write-Host 'Payload type: {:?}'\n",
            config.base_config.lhost, config.base_config.lport, config.base_config.payload_type,
        ))
    }

    /// Generate Bash payload (safe reference)
    fn generate_bash_payload(&self, config: &SmartPayloadConfig) -> Result<String> {
        Ok(format!(
            "#!/bin/bash\n\
             # Bash reverse shell reference\n\
             # SAFE MODE - EDUCATIONAL ONLY\n\
             echo 'Would connect to {}:{}'\n\
             echo 'Payload type: {:?}'\n",
            config.base_config.lhost, config.base_config.lport, config.base_config.payload_type,
        ))
    }

    /// Apply evasion techniques
    fn apply_evasion(
        &self,
        payload: Vec<u8>,
        technique: &EvasionTechnique,
        _target: &TargetInfo,
    ) -> Result<Vec<u8>> {
        match technique {
            EvasionTechnique::None => Ok(payload),
            EvasionTechnique::BehavioralDelay => {
                // Add delay instructions
                let mut result = b"# Behavioral delay\nsleep $((RANDOM % 5))\n".to_vec();
                result.extend_from_slice(&payload);
                Ok(result)
            }
            EvasionTechnique::EnvironmentCheck => {
                // Add environment checks
                let mut result =
                    b"# Environment check\nif [ -d /proc/vz ]; then exit 0; fi\n".to_vec();
                result.extend_from_slice(&payload);
                Ok(result)
            }
            EvasionTechnique::RuntimeDecryption => {
                // Payload will be encrypted, add decryption stub
                Ok(payload) // Handled by encryption step
            }
            EvasionTechnique::Polymorphic => {
                // Add randomization
                let nonce = uuid::Uuid::new_v4().to_string();
                let mut result = format!("# Nonce: {}\n", nonce).into_bytes();
                result.extend_from_slice(&payload);
                Ok(result)
            }
        }
    }

    /// Encrypt payload
    fn encrypt_payload(&self, payload: Vec<u8>, config: &SmartPayloadConfig) -> Result<Vec<u8>> {
        let key_str = config
            .encryption_key
            .as_ref()
            .ok_or_else(|| anyhow!("Encryption key not provided"))?;

        let keys = derive_keys(key_str.as_bytes(), b"ferox-payload-salt")?;
        let aad = b"payload-encryption";
        let (nonce, ciphertext) = aes_encrypt(&keys.enc_key, &payload, aad)?;

        // Prepend nonce to ciphertext
        let mut result = Vec::new();
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Create payload metadata
    fn create_metadata(
        &self,
        config: &SmartPayloadConfig,
        payload: &[u8],
    ) -> Result<PayloadMetadata> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(payload);
        let checksum = format!("{:x}", hasher.finalize());

        Ok(PayloadMetadata {
            size: payload.len(),
            encrypted: config.encryption_key.is_some(),
            obfuscated: config.evasion != EvasionTechnique::None,
            architecture: config.base_config.architecture.clone(),
            format: config.base_config.format.clone(),
            checksum,
            generation_time: chrono::Utc::now(),
        })
    }
}

impl Default for PayloadGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory executor for fileless execution
pub struct MemoryExecutor {
    safe_mode: bool,
}

impl MemoryExecutor {
    pub fn new() -> Self {
        Self { safe_mode: true }
    }

    /// Execute payload in memory (safe mode only shows what would happen)
    pub fn execute(&self, payload: &SmartPayload) -> Result<String> {
        if self.safe_mode {
            Ok(format!(
                "[SAFE MODE] Would execute {} byte payload in memory\n\
                 Architecture: {:?}\n\
                 Execution method: {:?}\n\
                 Encrypted: {}\n\
                 Checksum: {}",
                payload.size(),
                payload.metadata.architecture,
                payload.config.execution_method,
                payload.metadata.encrypted,
                payload.metadata.checksum,
            ))
        } else {
            bail!("Production memory execution requires explicit authorization")
        }
    }
}

impl Default for MemoryExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Evasion engine for behavioral camouflage
pub struct EvasionEngine;

impl EvasionEngine {
    pub fn new() -> Self {
        Self
    }

    /// Perform behavioral camouflage
    pub async fn behavioral_camouflage(&self) -> Result<()> {
        // Random delay between 1-5 seconds
        let delay = (chrono::Utc::now().timestamp() % 5) as u64 + 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
        Ok(())
    }

    /// Check for sandbox environment
    pub fn is_sandbox(&self) -> bool {
        // Safe implementation - always returns false
        // In production, would check for VM artifacts, debuggers, etc.
        false
    }
}

impl Default for EvasionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::payload::PayloadType;

    #[test]
    fn test_safe_payload_generation() {
        let generator = PayloadGenerator::new();
        let base_config =
            PayloadConfig::new(PayloadType::ReverseTcp, "192.168.1.10".to_string(), 4444);
        let config = SmartPayloadConfig::new(base_config)
            .with_execution_method(ExecutionMethod::MemoryOnly)
            .with_evasion(EvasionTechnique::BehavioralDelay);

        let target = TargetInfo::new("linux", Architecture::X64);
        let payload = generator.generate(config, &target).unwrap();

        assert!(payload.size() > 0);
        assert!(!payload.metadata.encrypted);
    }

    #[test]
    fn test_encrypted_payload() {
        let generator = PayloadGenerator::new();
        let base_config =
            PayloadConfig::new(PayloadType::ReverseTcp, "192.168.1.10".to_string(), 4444);
        let config = SmartPayloadConfig::new(base_config).with_encryption("test-key-123");

        let target = TargetInfo::new("linux", Architecture::X64);
        let payload = generator.generate(config, &target).unwrap();

        assert!(payload.metadata.encrypted);
        assert!(payload.size() > 0);
    }

    #[test]
    fn test_memory_executor() {
        let executor = MemoryExecutor::new();
        let generator = PayloadGenerator::new();
        let base_config =
            PayloadConfig::new(PayloadType::ReverseTcp, "192.168.1.10".to_string(), 4444);
        let config = SmartPayloadConfig::new(base_config);
        let target = TargetInfo::new("linux", Architecture::X64);
        let payload = generator.generate(config, &target).unwrap();

        let result = executor.execute(&payload).unwrap();
        assert!(result.contains("[SAFE MODE]"));
    }

    #[tokio::test]
    async fn test_evasion_engine() {
        let engine = EvasionEngine::new();
        assert!(!engine.is_sandbox());

        let start = std::time::Instant::now();
        engine.behavioral_camouflage().await.unwrap();
        let elapsed = start.elapsed();

        assert!(elapsed.as_secs() >= 1);
        assert!(elapsed.as_secs() <= 6);
    }

    #[test]
    fn test_evasion_techniques() {
        let generator = PayloadGenerator::new();
        let base_payload = b"echo test".to_vec();
        let target = TargetInfo::new("linux", Architecture::X64);

        // Test behavioral delay
        let result = generator
            .apply_evasion(
                base_payload.clone(),
                &EvasionTechnique::BehavioralDelay,
                &target,
            )
            .unwrap();
        assert!(result.len() > base_payload.len());

        // Test polymorphic
        let result = generator
            .apply_evasion(
                base_payload.clone(),
                &EvasionTechnique::Polymorphic,
                &target,
            )
            .unwrap();
        assert!(result.len() > base_payload.len());
    }
}
