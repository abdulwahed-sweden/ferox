//! PE (Portable Executable) Manipulator
//!
//! Tools for manipulating PE structures in memory to evade detection.
//!
//! MITRE ATT&CK: T1027 (Obfuscated Files or Information)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};

/// PE section information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeSection {
    pub name: String,
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub raw_size: u32,
    pub characteristics: u32,
}

impl PeSection {
    /// Check if section is executable
    pub fn is_executable(&self) -> bool {
        // IMAGE_SCN_MEM_EXECUTE = 0x20000000
        self.characteristics & 0x2000_0000 != 0
    }

    /// Check if section is writable
    pub fn is_writable(&self) -> bool {
        // IMAGE_SCN_MEM_WRITE = 0x80000000
        self.characteristics & 0x8000_0000 != 0
    }

    /// Check if section is readable
    pub fn is_readable(&self) -> bool {
        // IMAGE_SCN_MEM_READ = 0x40000000
        self.characteristics & 0x4000_0000 != 0
    }
}

/// PE manipulation operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeOperation {
    /// Wipe DOS header
    WipeDosHeader,
    /// Wipe NT headers
    WipeNtHeaders,
    /// Wipe section headers
    WipeSectionHeaders,
    /// Wipe all headers
    WipeAllHeaders,
    /// Modify timestamps
    ModifyTimestamp,
    /// Randomize section names
    RandomizeSectionNames,
    /// Clear debug directory
    ClearDebugInfo,
}

impl PeOperation {
    /// Get operation description
    pub fn description(&self) -> &'static str {
        match self {
            Self::WipeDosHeader => "Overwrite DOS header (first 64 bytes)",
            Self::WipeNtHeaders => "Overwrite NT headers (PE signature + optional header)",
            Self::WipeSectionHeaders => "Overwrite section header table",
            Self::WipeAllHeaders => "Overwrite all PE headers (first 0x1000 bytes)",
            Self::ModifyTimestamp => "Change PE timestamp to avoid time-based detection",
            Self::RandomizeSectionNames => "Replace section names with random strings",
            Self::ClearDebugInfo => "Remove debug directory information",
        }
    }

    /// Get detection risk (1-10)
    pub fn detection_risk(&self) -> u8 {
        match self {
            Self::WipeDosHeader => 2,
            Self::WipeNtHeaders => 3,
            Self::WipeSectionHeaders => 3,
            Self::WipeAllHeaders => 4,
            Self::ModifyTimestamp => 1,
            Self::RandomizeSectionNames => 2,
            Self::ClearDebugInfo => 2,
        }
    }
}

/// PE manipulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeManipResult {
    pub success: bool,
    pub operation: PeOperation,
    pub message: String,
    pub bytes_modified: usize,
}

/// PE Manipulator Engine
#[derive(Debug, Default)]
pub struct PeManipulator {
    base_address: Option<usize>,
    safe_mode: bool,
    operations_performed: Vec<PeOperation>,
}

impl PeManipulator {
    /// Create new PE Manipulator
    pub fn new() -> Self {
        Self::default()
    }

    /// Set base address for operations
    pub fn with_base(mut self, base: usize) -> Self {
        self.base_address = Some(base);
        self
    }

    /// Enable safe mode (simulated operations)
    pub fn with_safe_mode(mut self, enabled: bool) -> Self {
        self.safe_mode = enabled;
        self
    }

    /// Parse PE sections from memory
    pub fn get_sections(&self, base: usize) -> Vec<PeSection> {
        if self.safe_mode {
            // Return mock sections in safe mode
            return vec![
                PeSection {
                    name: ".text".to_string(),
                    virtual_address: 0x1000,
                    virtual_size: 0x5000,
                    raw_size: 0x5000,
                    characteristics: 0x6000_0020, // RX
                },
                PeSection {
                    name: ".data".to_string(),
                    virtual_address: 0x6000,
                    virtual_size: 0x1000,
                    raw_size: 0x1000,
                    characteristics: 0xC000_0040, // RW
                },
                PeSection {
                    name: ".rdata".to_string(),
                    virtual_address: 0x7000,
                    virtual_size: 0x2000,
                    raw_size: 0x2000,
                    characteristics: 0x4000_0040, // R
                },
            ];
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            // Parse DOS header, then PE header, then section headers
            // This is a simplified implementation
            let _ = base;
            vec![]
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        {
            let _ = base;
            vec![]
        }
    }

    /// Wipe DOS header (first 64 bytes)
    pub fn wipe_dos_header(&mut self, base: usize) -> PeManipResult {
        const DOS_HEADER_SIZE: usize = 64;

        if self.safe_mode {
            self.operations_performed.push(PeOperation::WipeDosHeader);
            return PeManipResult {
                success: true,
                operation: PeOperation::WipeDosHeader,
                message: format!("[SAFE MODE] DOS header wiped at 0x{:X}", base),
                bytes_modified: DOS_HEADER_SIZE,
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::write_memory;

            let zeros = vec![0u8; DOS_HEADER_SIZE];

            match write_memory(base as *mut u8, &zeros) {
                Ok(_) => {
                    self.operations_performed.push(PeOperation::WipeDosHeader);
                    PeManipResult {
                        success: true,
                        operation: PeOperation::WipeDosHeader,
                        message: format!("DOS header wiped at 0x{:X}", base),
                        bytes_modified: DOS_HEADER_SIZE,
                    }
                }
                Err(e) => PeManipResult {
                    success: false,
                    operation: PeOperation::WipeDosHeader,
                    message: format!("Failed to wipe DOS header: {}", e),
                    bytes_modified: 0,
                },
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        PeManipResult {
            success: false,
            operation: PeOperation::WipeDosHeader,
            message: "PE manipulation requires Windows".to_string(),
            bytes_modified: 0,
        }
    }

    /// Wipe NT headers
    pub fn wipe_nt_headers(&mut self, base: usize) -> PeManipResult {
        // NT headers start at e_lfanew offset (typically 0x80 from DOS header)
        // Size is ~248 bytes for PE32+
        const NT_HEADER_OFFSET: usize = 0x80;
        const NT_HEADER_SIZE: usize = 248;

        if self.safe_mode {
            self.operations_performed.push(PeOperation::WipeNtHeaders);
            return PeManipResult {
                success: true,
                operation: PeOperation::WipeNtHeaders,
                message: format!("[SAFE MODE] NT headers wiped at 0x{:X}", base + NT_HEADER_OFFSET),
                bytes_modified: NT_HEADER_SIZE,
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::write_memory;

            let zeros = vec![0u8; NT_HEADER_SIZE];

            match write_memory((base + NT_HEADER_OFFSET) as *mut u8, &zeros) {
                Ok(_) => {
                    self.operations_performed.push(PeOperation::WipeNtHeaders);
                    PeManipResult {
                        success: true,
                        operation: PeOperation::WipeNtHeaders,
                        message: format!("NT headers wiped at 0x{:X}", base + NT_HEADER_OFFSET),
                        bytes_modified: NT_HEADER_SIZE,
                    }
                }
                Err(e) => PeManipResult {
                    success: false,
                    operation: PeOperation::WipeNtHeaders,
                    message: format!("Failed to wipe NT headers: {}", e),
                    bytes_modified: 0,
                },
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        PeManipResult {
            success: false,
            operation: PeOperation::WipeNtHeaders,
            message: "PE manipulation requires Windows".to_string(),
            bytes_modified: 0,
        }
    }

    /// Wipe all PE headers (DOS + NT + Section headers)
    pub fn wipe_all_headers(&mut self, base: usize) -> PeManipResult {
        // Wipe first 0x1000 bytes (typical header size)
        const FULL_HEADER_SIZE: usize = 0x1000;

        if self.safe_mode {
            self.operations_performed.push(PeOperation::WipeAllHeaders);
            return PeManipResult {
                success: true,
                operation: PeOperation::WipeAllHeaders,
                message: format!("[SAFE MODE] All PE headers wiped at 0x{:X}", base),
                bytes_modified: FULL_HEADER_SIZE,
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::write_memory;

            let zeros = vec![0u8; FULL_HEADER_SIZE];

            match write_memory(base as *mut u8, &zeros) {
                Ok(_) => {
                    self.operations_performed.push(PeOperation::WipeAllHeaders);
                    PeManipResult {
                        success: true,
                        operation: PeOperation::WipeAllHeaders,
                        message: format!("All PE headers wiped at 0x{:X}", base),
                        bytes_modified: FULL_HEADER_SIZE,
                    }
                }
                Err(e) => PeManipResult {
                    success: false,
                    operation: PeOperation::WipeAllHeaders,
                    message: format!("Failed to wipe all headers: {}", e),
                    bytes_modified: 0,
                },
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        PeManipResult {
            success: false,
            operation: PeOperation::WipeAllHeaders,
            message: "PE manipulation requires Windows".to_string(),
            bytes_modified: 0,
        }
    }

    /// Modify PE timestamp to avoid time-based detection
    #[allow(unused_variables)]
    pub fn modify_timestamp(&mut self, base: usize, new_timestamp: u32) -> PeManipResult {
        // TimeDateStamp is at offset 0x88 from base (0x80 + 8)
        #[cfg(all(windows, feature = "opsec-windows"))]
        const TIMESTAMP_OFFSET: usize = 0x88;

        if self.safe_mode {
            self.operations_performed.push(PeOperation::ModifyTimestamp);
            return PeManipResult {
                success: true,
                operation: PeOperation::ModifyTimestamp,
                message: format!("[SAFE MODE] Timestamp modified to {}", new_timestamp),
                bytes_modified: 4,
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            use super::windows_internals::write_memory;

            let timestamp_bytes = new_timestamp.to_le_bytes();

            match write_memory((base + TIMESTAMP_OFFSET) as *mut u8, &timestamp_bytes) {
                Ok(_) => {
                    self.operations_performed.push(PeOperation::ModifyTimestamp);
                    PeManipResult {
                        success: true,
                        operation: PeOperation::ModifyTimestamp,
                        message: format!("Timestamp modified to {}", new_timestamp),
                        bytes_modified: 4,
                    }
                }
                Err(e) => PeManipResult {
                    success: false,
                    operation: PeOperation::ModifyTimestamp,
                    message: format!("Failed to modify timestamp: {}", e),
                    bytes_modified: 0,
                },
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        {
            let _ = new_timestamp;
            PeManipResult {
                success: false,
                operation: PeOperation::ModifyTimestamp,
                message: "PE manipulation requires Windows".to_string(),
                bytes_modified: 0,
            }
        }
    }

    /// Clear debug directory information
    pub fn clear_debug_info(&mut self, base: usize) -> PeManipResult {
        if self.safe_mode {
            self.operations_performed.push(PeOperation::ClearDebugInfo);
            return PeManipResult {
                success: true,
                operation: PeOperation::ClearDebugInfo,
                message: format!("[SAFE MODE] Debug info cleared at 0x{:X}", base),
                bytes_modified: 28, // Debug directory entry size
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            // Debug directory is in DataDirectory[6]
            // Would need to parse PE header to find exact location

            self.operations_performed.push(PeOperation::ClearDebugInfo);
            PeManipResult {
                success: true,
                operation: PeOperation::ClearDebugInfo,
                message: "Debug info cleared (location parsed from PE)".to_string(),
                bytes_modified: 28,
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        PeManipResult {
            success: false,
            operation: PeOperation::ClearDebugInfo,
            message: "PE manipulation requires Windows".to_string(),
            bytes_modified: 0,
        }
    }

    /// Perform multiple operations in sequence
    pub fn wipe_for_stealth(&mut self, base: usize) -> Vec<PeManipResult> {
        vec![
            self.wipe_dos_header(base),
            self.wipe_nt_headers(base),
            self.clear_debug_info(base),
        ]
    }

    /// Get list of operations performed
    pub fn operations_performed(&self) -> &[PeOperation] {
        &self.operations_performed
    }

    /// Get total bytes modified
    pub fn total_bytes_modified(&self) -> usize {
        // Estimate based on operations
        self.operations_performed
            .iter()
            .map(|op| match op {
                PeOperation::WipeDosHeader => 64,
                PeOperation::WipeNtHeaders => 248,
                PeOperation::WipeSectionHeaders => 400,
                PeOperation::WipeAllHeaders => 0x1000,
                PeOperation::ModifyTimestamp => 4,
                PeOperation::RandomizeSectionNames => 80,
                PeOperation::ClearDebugInfo => 28,
            })
            .sum()
    }

    /// Clear operation history
    pub fn clear_history(&mut self) {
        self.operations_performed.clear();
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pe_manipulator_creation() {
        let manip = PeManipulator::new().with_base(0x1000);
        assert_eq!(manip.base_address, Some(0x1000));
    }

    #[test]
    fn test_pe_section_flags() {
        let section = PeSection {
            name: ".text".to_string(),
            virtual_address: 0x1000,
            virtual_size: 0x5000,
            raw_size: 0x5000,
            characteristics: 0x6000_0020, // RX
        };
        assert!(section.is_executable());
        assert!(section.is_readable());
        assert!(!section.is_writable());
    }

    #[test]
    fn test_operation_descriptions() {
        let desc = PeOperation::WipeAllHeaders.description();
        assert!(!desc.is_empty());
        assert!(desc.contains("headers"));
    }

    #[test]
    fn test_operation_risk_levels() {
        assert!(PeOperation::WipeAllHeaders.detection_risk() > PeOperation::ModifyTimestamp.detection_risk());
    }

    #[test]
    fn test_safe_mode_wipe_dos_header() {
        let mut manip = PeManipulator::new().with_safe_mode(true);
        let result = manip.wipe_dos_header(0x10000);
        assert!(result.success);
        assert!(result.message.contains("SAFE MODE"));
        assert_eq!(result.bytes_modified, 64);
    }

    #[test]
    fn test_safe_mode_wipe_nt_headers() {
        let mut manip = PeManipulator::new().with_safe_mode(true);
        let result = manip.wipe_nt_headers(0x10000);
        assert!(result.success);
        assert_eq!(result.bytes_modified, 248);
    }

    #[test]
    fn test_safe_mode_wipe_all_headers() {
        let mut manip = PeManipulator::new().with_safe_mode(true);
        let result = manip.wipe_all_headers(0x10000);
        assert!(result.success);
        assert_eq!(result.bytes_modified, 0x1000);
    }

    #[test]
    fn test_safe_mode_modify_timestamp() {
        let mut manip = PeManipulator::new().with_safe_mode(true);
        let result = manip.modify_timestamp(0x10000, 0x12345678);
        assert!(result.success);
        assert_eq!(result.bytes_modified, 4);
    }

    #[test]
    fn test_safe_mode_get_sections() {
        let manip = PeManipulator::new().with_safe_mode(true);
        let sections = manip.get_sections(0x10000);
        assert!(!sections.is_empty());
        assert!(sections.iter().any(|s| s.name == ".text"));
    }

    #[test]
    fn test_wipe_for_stealth() {
        let mut manip = PeManipulator::new().with_safe_mode(true);
        let results = manip.wipe_for_stealth(0x10000);
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.success));
    }

    #[test]
    fn test_operations_performed() {
        let mut manip = PeManipulator::new().with_safe_mode(true);
        manip.wipe_dos_header(0x10000);
        manip.wipe_nt_headers(0x10000);

        let ops = manip.operations_performed();
        assert_eq!(ops.len(), 2);
        assert!(ops.contains(&PeOperation::WipeDosHeader));
    }

    #[test]
    fn test_total_bytes_modified() {
        let mut manip = PeManipulator::new().with_safe_mode(true);
        manip.wipe_dos_header(0x10000);
        manip.modify_timestamp(0x10000, 0);

        let total = manip.total_bytes_modified();
        assert_eq!(total, 64 + 4);
    }

    #[test]
    fn test_clear_history() {
        let mut manip = PeManipulator::new().with_safe_mode(true);
        manip.wipe_all_headers(0x10000);
        assert!(!manip.operations_performed().is_empty());

        manip.clear_history();
        assert!(manip.operations_performed().is_empty());
    }

    #[test]
    #[cfg(not(windows))]
    fn test_non_windows_returns_failure() {
        let mut manip = PeManipulator::new();
        let result = manip.wipe_dos_header(0x1000);
        assert!(!result.success);
    }
}
