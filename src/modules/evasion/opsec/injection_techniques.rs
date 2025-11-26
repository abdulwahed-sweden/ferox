//! Process Injection Techniques
//!
//! Implementation of various injection methods with OPSEC considerations.
//!
//! MITRE ATT&CK: T1055.x (Process Injection sub-techniques)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};

/// Available injection techniques
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InjectionTechnique {
    /// Classic VirtualAllocEx + WriteProcessMemory + CreateRemoteThread
    ClassicRemoteThread,
    /// NtCreateThreadEx (less monitored than CreateRemoteThread)
    NtCreateThreadEx,
    /// Queue APC to alertable thread
    QueueUserApc,
    /// Early bird - inject before main thread runs
    EarlyBird,
    /// Thread hijacking - modify existing thread context
    ThreadHijack,
    /// Process hollowing - replace process memory
    ProcessHollowing,
    /// Module stomping - overwrite legitimate DLL
    ModuleStomping,
    /// Atom bombing - via global atom table
    AtomBombing,
    /// Callback injection - via window callbacks
    CallbackInjection,
    /// Fiber injection - via fibers
    FiberInjection,
    /// Direct syscall injection
    DirectSyscall,
}

impl InjectionTechnique {
    /// Get OPSEC rating (1-10, higher = more stealthy)
    pub fn opsec_rating(&self) -> u8 {
        match self {
            Self::ClassicRemoteThread => 2, // Heavily monitored
            Self::NtCreateThreadEx => 4,    // Less monitored
            Self::QueueUserApc => 6,        // Moderate stealth
            Self::EarlyBird => 7,           // Good stealth
            Self::ThreadHijack => 7,        // Good stealth
            Self::ProcessHollowing => 5,    // Known technique
            Self::ModuleStomping => 8,      // High stealth
            Self::AtomBombing => 6,         // Moderate
            Self::CallbackInjection => 7,   // Good stealth
            Self::FiberInjection => 6,      // Moderate
            Self::DirectSyscall => 9,       // Very stealthy
        }
    }

    /// Get reliability rating (1-10)
    pub fn reliability(&self) -> u8 {
        match self {
            Self::ClassicRemoteThread => 9,
            Self::NtCreateThreadEx => 8,
            Self::QueueUserApc => 7,
            Self::EarlyBird => 6,
            Self::ThreadHijack => 7,
            Self::ProcessHollowing => 6,
            Self::ModuleStomping => 5,
            Self::AtomBombing => 4,
            Self::CallbackInjection => 6,
            Self::FiberInjection => 5,
            Self::DirectSyscall => 8,
        }
    }

    /// Get MITRE ATT&CK technique ID
    pub fn mitre_id(&self) -> &'static str {
        match self {
            Self::ClassicRemoteThread => "T1055.002",
            Self::NtCreateThreadEx => "T1055.002",
            Self::QueueUserApc => "T1055.004",
            Self::EarlyBird => "T1055.004",
            Self::ThreadHijack => "T1055.003",
            Self::ProcessHollowing => "T1055.012",
            Self::ModuleStomping => "T1055.001",
            Self::AtomBombing => "T1055.011",
            Self::CallbackInjection => "T1055.014",
            Self::FiberInjection => "T1055",
            Self::DirectSyscall => "T1055",
        }
    }

    /// Description of the technique
    pub fn description(&self) -> &'static str {
        match self {
            Self::ClassicRemoteThread => "Allocate memory, write payload, create remote thread",
            Self::NtCreateThreadEx => "Use undocumented NtCreateThreadEx syscall",
            Self::QueueUserApc => "Queue APC to target thread, execute on alert",
            Self::EarlyBird => "Inject before process main thread starts",
            Self::ThreadHijack => "Suspend thread, modify context, resume",
            Self::ProcessHollowing => "Hollow out process and replace with payload",
            Self::ModuleStomping => "Overwrite legitimate DLL's code section",
            Self::AtomBombing => "Use atom table for code injection",
            Self::CallbackInjection => "Abuse window callback functions",
            Self::FiberInjection => "Convert thread to fiber and inject",
            Self::DirectSyscall => "Direct syscall to avoid hooks",
        }
    }

    /// Get all available techniques
    pub fn all() -> &'static [InjectionTechnique] {
        &[
            Self::ClassicRemoteThread,
            Self::NtCreateThreadEx,
            Self::QueueUserApc,
            Self::EarlyBird,
            Self::ThreadHijack,
            Self::ProcessHollowing,
            Self::ModuleStomping,
            Self::AtomBombing,
            Self::CallbackInjection,
            Self::FiberInjection,
            Self::DirectSyscall,
        ]
    }
}

/// Injection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionResult {
    pub success: bool,
    pub technique: InjectionTechnique,
    pub target_pid: u32,
    pub thread_id: Option<u32>,
    pub allocated_address: Option<usize>,
    pub message: String,
}

/// Shellcode wrapper for injection
#[derive(Debug, Clone)]
pub struct Shellcode {
    pub bytes: Vec<u8>,
    pub entry_offset: usize,
    pub is_encrypted: bool,
}

impl Shellcode {
    /// Create from raw bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            entry_offset: 0,
            is_encrypted: false,
        }
    }

    /// XOR encrypt shellcode
    pub fn encrypt(&mut self, key: &[u8]) {
        if key.is_empty() {
            return;
        }
        for (i, byte) in self.bytes.iter_mut().enumerate() {
            *byte ^= key[i % key.len()];
        }
        self.is_encrypted = true;
    }

    /// XOR decrypt shellcode
    pub fn decrypt(&mut self, key: &[u8]) {
        self.encrypt(key); // XOR is symmetric
        self.is_encrypted = false;
    }

    /// Get shellcode size
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

/// Injection technique implementations
pub struct InjectionMethods;

impl InjectionMethods {
    /// Classic CreateRemoteThread injection
    #[cfg(windows)]
    pub fn classic_remote_thread(pid: u32, shellcode: &Shellcode) -> InjectionResult {
        // This requires Windows API calls
        // OpenProcess -> VirtualAllocEx -> WriteProcessMemory -> CreateRemoteThread
        let _ = shellcode;

        InjectionResult {
            success: true,
            technique: InjectionTechnique::ClassicRemoteThread,
            target_pid: pid,
            thread_id: Some(0),
            allocated_address: Some(0),
            message: "Classic injection (simulated)".to_string(),
        }
    }

    /// NtCreateThreadEx injection
    #[cfg(windows)]
    pub fn nt_create_thread_ex(pid: u32, shellcode: &Shellcode) -> InjectionResult {
        // Use NtCreateThreadEx syscall directly
        // Less monitored than CreateRemoteThread
        let _ = shellcode;

        InjectionResult {
            success: true,
            technique: InjectionTechnique::NtCreateThreadEx,
            target_pid: pid,
            thread_id: Some(0),
            allocated_address: Some(0),
            message: "NtCreateThreadEx injection (simulated)".to_string(),
        }
    }

    /// Queue User APC injection
    #[cfg(windows)]
    pub fn queue_user_apc(pid: u32, tid: u32, shellcode: &Shellcode) -> InjectionResult {
        // Queue APC to alertable thread
        // Requires thread to enter alertable state
        let _ = shellcode;

        InjectionResult {
            success: true,
            technique: InjectionTechnique::QueueUserApc,
            target_pid: pid,
            thread_id: Some(tid),
            allocated_address: Some(0),
            message: "APC queued (simulated)".to_string(),
        }
    }

    /// Early Bird injection
    #[cfg(windows)]
    pub fn early_bird(process_path: &str, shellcode: &Shellcode) -> InjectionResult {
        // Create process suspended
        // Inject before main thread starts
        // Resume process
        let _ = (process_path, shellcode);

        InjectionResult {
            success: true,
            technique: InjectionTechnique::EarlyBird,
            target_pid: 0,
            thread_id: Some(0),
            allocated_address: Some(0),
            message: "Early bird injection (simulated)".to_string(),
        }
    }

    /// Thread hijacking
    #[cfg(windows)]
    pub fn thread_hijack(pid: u32, tid: u32, shellcode: &Shellcode) -> InjectionResult {
        // Suspend thread
        // Get thread context
        // Modify RIP/EIP to point to shellcode
        // Resume thread
        let _ = shellcode;

        InjectionResult {
            success: true,
            technique: InjectionTechnique::ThreadHijack,
            target_pid: pid,
            thread_id: Some(tid),
            allocated_address: Some(0),
            message: "Thread hijacked (simulated)".to_string(),
        }
    }

    /// Process hollowing
    #[cfg(windows)]
    pub fn process_hollowing(process_path: &str, payload: &[u8]) -> InjectionResult {
        // Create process suspended
        // Unmap original executable
        // Map payload in its place
        // Resume process
        let _ = (process_path, payload);

        InjectionResult {
            success: true,
            technique: InjectionTechnique::ProcessHollowing,
            target_pid: 0,
            thread_id: Some(0),
            allocated_address: Some(0),
            message: "Process hollowed (simulated)".to_string(),
        }
    }

    /// Module stomping
    #[cfg(windows)]
    pub fn module_stomping(pid: u32, dll_name: &str, shellcode: &Shellcode) -> InjectionResult {
        // Find loaded DLL in target
        // Overwrite .text section with shellcode
        // Execute via CreateRemoteThread pointing to DLL
        let _ = shellcode;

        InjectionResult {
            success: true,
            technique: InjectionTechnique::ModuleStomping,
            target_pid: pid,
            thread_id: None,
            allocated_address: Some(0),
            message: format!("Module {} stomped (simulated)", dll_name),
        }
    }

    /// Atom bombing injection
    #[cfg(windows)]
    pub fn atom_bombing(pid: u32, shellcode: &Shellcode) -> InjectionResult {
        // Use global atom table for injection
        let _ = shellcode;

        InjectionResult {
            success: true,
            technique: InjectionTechnique::AtomBombing,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Atom bombing (simulated)".to_string(),
        }
    }

    /// Callback injection
    #[cfg(windows)]
    pub fn callback_injection(pid: u32, shellcode: &Shellcode) -> InjectionResult {
        // Abuse window callback functions
        let _ = shellcode;

        InjectionResult {
            success: true,
            technique: InjectionTechnique::CallbackInjection,
            target_pid: pid,
            thread_id: None,
            allocated_address: Some(0),
            message: "Callback injection (simulated)".to_string(),
        }
    }

    /// Fiber injection
    #[cfg(windows)]
    pub fn fiber_injection(pid: u32, shellcode: &Shellcode) -> InjectionResult {
        // Convert thread to fiber and inject
        let _ = shellcode;

        InjectionResult {
            success: true,
            technique: InjectionTechnique::FiberInjection,
            target_pid: pid,
            thread_id: None,
            allocated_address: Some(0),
            message: "Fiber injection (simulated)".to_string(),
        }
    }

    /// Direct syscall injection (bypasses hooks)
    #[cfg(windows)]
    pub fn direct_syscall(pid: u32, shellcode: &Shellcode) -> InjectionResult {
        // Use direct syscalls to avoid EDR hooks
        // NtAllocateVirtualMemory, NtWriteVirtualMemory, NtCreateThreadEx
        let _ = shellcode;

        InjectionResult {
            success: true,
            technique: InjectionTechnique::DirectSyscall,
            target_pid: pid,
            thread_id: Some(0),
            allocated_address: Some(0),
            message: "Direct syscall injection (simulated)".to_string(),
        }
    }

    // Non-Windows stubs
    #[cfg(not(windows))]
    pub fn classic_remote_thread(pid: u32, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::ClassicRemoteThread,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn nt_create_thread_ex(pid: u32, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::NtCreateThreadEx,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn queue_user_apc(pid: u32, _tid: u32, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::QueueUserApc,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn early_bird(_process_path: &str, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::EarlyBird,
            target_pid: 0,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn thread_hijack(pid: u32, _tid: u32, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::ThreadHijack,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn process_hollowing(_process_path: &str, _payload: &[u8]) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::ProcessHollowing,
            target_pid: 0,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn module_stomping(pid: u32, _dll_name: &str, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::ModuleStomping,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn atom_bombing(pid: u32, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::AtomBombing,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn callback_injection(pid: u32, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::CallbackInjection,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn fiber_injection(pid: u32, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::FiberInjection,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }

    #[cfg(not(windows))]
    pub fn direct_syscall(pid: u32, _shellcode: &Shellcode) -> InjectionResult {
        InjectionResult {
            success: false,
            technique: InjectionTechnique::DirectSyscall,
            target_pid: pid,
            thread_id: None,
            allocated_address: None,
            message: "Injection requires Windows".to_string(),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_technique_ratings() {
        assert!(
            InjectionTechnique::DirectSyscall.opsec_rating()
                > InjectionTechnique::ClassicRemoteThread.opsec_rating()
        );
    }

    #[test]
    fn test_shellcode_creation() {
        let sc = Shellcode::from_bytes(vec![0x90, 0x90, 0xCC]);
        assert_eq!(sc.len(), 3);
        assert!(!sc.is_encrypted);
    }

    #[test]
    fn test_shellcode_encryption() {
        let mut sc = Shellcode::from_bytes(vec![0x90, 0x90, 0xCC]);
        let key = vec![0xAA];

        sc.encrypt(&key);
        assert!(sc.is_encrypted);
        assert_ne!(sc.bytes, vec![0x90, 0x90, 0xCC]);

        sc.decrypt(&key);
        assert!(!sc.is_encrypted);
        assert_eq!(sc.bytes, vec![0x90, 0x90, 0xCC]);
    }

    #[test]
    fn test_mitre_ids() {
        assert!(!InjectionTechnique::ProcessHollowing.mitre_id().is_empty());
        assert_eq!(
            InjectionTechnique::ProcessHollowing.mitre_id(),
            "T1055.012"
        );
    }

    #[test]
    fn test_all_techniques() {
        let all = InjectionTechnique::all();
        assert_eq!(all.len(), 11);
    }

    #[test]
    fn test_technique_description() {
        let desc = InjectionTechnique::DirectSyscall.description();
        assert!(!desc.is_empty());
    }

    #[test]
    fn test_reliability_ratings() {
        assert!(InjectionTechnique::ClassicRemoteThread.reliability() > 0);
        assert!(InjectionTechnique::DirectSyscall.reliability() > 0);
    }

    #[test]
    fn test_shellcode_empty() {
        let sc = Shellcode::from_bytes(vec![]);
        assert!(sc.is_empty());
        assert_eq!(sc.len(), 0);
    }

    #[test]
    fn test_injection_result_structure() {
        let result = InjectionResult {
            success: true,
            technique: InjectionTechnique::DirectSyscall,
            target_pid: 1234,
            thread_id: Some(5678),
            allocated_address: Some(0x12345678),
            message: "Test".to_string(),
        };
        assert!(result.success);
        assert_eq!(result.target_pid, 1234);
    }
}
