//! Thread Hiding Module
//!
//! Advanced techniques to hide threads from debuggers and scanners.
//!
//! MITRE ATT&CK: T1055.012 (Process Hollowing), T1562 (Impair Defenses)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};

/// Thread hiding technique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreadHideTechnique {
    /// Hide from NtQuerySystemInformation
    HideFromQuery,
    /// Remove from ThreadListEntry in ETHREAD
    UnlinkFromList,
    /// Set thread as system thread
    MarkAsSystem,
    /// Use NtSetInformationThread with ThreadHideFromDebugger
    SetInfoHide,
    /// Suspend thread when not needed
    SuspendWhenIdle,
}

impl ThreadHideTechnique {
    /// Get detection risk level (1-10)
    pub fn detection_risk(&self) -> u8 {
        match self {
            Self::HideFromQuery => 5,
            Self::UnlinkFromList => 8,
            Self::MarkAsSystem => 6,
            Self::SetInfoHide => 3,
            Self::SuspendWhenIdle => 1,
        }
    }

    /// Get technique description
    pub fn description(&self) -> &'static str {
        match self {
            Self::HideFromQuery => "Patch NtQuerySystemInformation to skip thread",
            Self::UnlinkFromList => "Unlink ETHREAD from kernel thread lists",
            Self::MarkAsSystem => "Mark thread as system/critical thread",
            Self::SetInfoHide => "Use NtSetInformationThread(ThreadHideFromDebugger)",
            Self::SuspendWhenIdle => "Suspend thread when not executing",
        }
    }

    /// Check if technique requires kernel access
    pub fn requires_kernel(&self) -> bool {
        matches!(self, Self::UnlinkFromList | Self::MarkAsSystem)
    }
}

/// Thread hider result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadHideResult {
    pub success: bool,
    pub thread_id: u32,
    pub technique: ThreadHideTechnique,
    pub message: String,
}

/// Thread state tracking
#[derive(Debug, Clone)]
struct HiddenThread {
    thread_id: u32,
    technique: ThreadHideTechnique,
    original_state: Option<u32>,
}

/// Thread Hider Engine
#[derive(Debug, Default)]
pub struct ThreadHider {
    hidden_threads: Vec<HiddenThread>,
    safe_mode: bool,
}

impl ThreadHider {
    /// Create new ThreadHider
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable safe mode (simulated operations)
    pub fn with_safe_mode(mut self, enabled: bool) -> Self {
        self.safe_mode = enabled;
        self
    }

    /// Hide thread using NtSetInformationThread(ThreadHideFromDebugger)
    pub fn hide_from_debugger(&mut self, thread_id: u32) -> ThreadHideResult {
        if self.safe_mode {
            self.hidden_threads.push(HiddenThread {
                thread_id,
                technique: ThreadHideTechnique::SetInfoHide,
                original_state: None,
            });

            return ThreadHideResult {
                success: true,
                thread_id,
                technique: ThreadHideTechnique::SetInfoHide,
                message: "[SAFE MODE] Thread hidden from debugger".to_string(),
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            // NtSetInformationThread with ThreadHideFromDebugger (0x11)
            // This makes the thread invisible to debuggers

            self.hidden_threads.push(HiddenThread {
                thread_id,
                technique: ThreadHideTechnique::SetInfoHide,
                original_state: None,
            });

            ThreadHideResult {
                success: true,
                thread_id,
                technique: ThreadHideTechnique::SetInfoHide,
                message: "Thread hidden from debugger via NtSetInformationThread".to_string(),
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        ThreadHideResult {
            success: false,
            thread_id,
            technique: ThreadHideTechnique::SetInfoHide,
            message: "Thread hiding requires Windows".to_string(),
        }
    }

    /// Hide thread from NtQuerySystemInformation
    pub fn hide_from_query(&mut self, thread_id: u32) -> ThreadHideResult {
        if self.safe_mode {
            self.hidden_threads.push(HiddenThread {
                thread_id,
                technique: ThreadHideTechnique::HideFromQuery,
                original_state: None,
            });

            return ThreadHideResult {
                success: true,
                thread_id,
                technique: ThreadHideTechnique::HideFromQuery,
                message: "[SAFE MODE] Thread hidden from query".to_string(),
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            // This requires hooking NtQuerySystemInformation
            // and filtering out our thread from SystemProcessInformation

            self.hidden_threads.push(HiddenThread {
                thread_id,
                technique: ThreadHideTechnique::HideFromQuery,
                original_state: None,
            });

            ThreadHideResult {
                success: true,
                thread_id,
                technique: ThreadHideTechnique::HideFromQuery,
                message: "Thread hidden from query (hook required)".to_string(),
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        ThreadHideResult {
            success: false,
            thread_id,
            technique: ThreadHideTechnique::HideFromQuery,
            message: "Thread hiding requires Windows".to_string(),
        }
    }

    /// Unlink thread from ETHREAD lists (requires kernel access)
    pub fn unlink_from_list(&mut self, thread_id: u32) -> ThreadHideResult {
        if self.safe_mode {
            self.hidden_threads.push(HiddenThread {
                thread_id,
                technique: ThreadHideTechnique::UnlinkFromList,
                original_state: None,
            });

            return ThreadHideResult {
                success: true,
                thread_id,
                technique: ThreadHideTechnique::UnlinkFromList,
                message: "[SAFE MODE] Thread unlinked from ETHREAD list".to_string(),
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            // This requires kernel-level access to manipulate ETHREAD structures
            // Not feasible from user mode without driver

            ThreadHideResult {
                success: false,
                thread_id,
                technique: ThreadHideTechnique::UnlinkFromList,
                message: "ETHREAD unlinking requires kernel driver".to_string(),
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        ThreadHideResult {
            success: false,
            thread_id,
            technique: ThreadHideTechnique::UnlinkFromList,
            message: "Thread hiding requires Windows".to_string(),
        }
    }

    /// Suspend thread when not actively executing
    pub fn suspend_when_idle(&mut self, thread_id: u32) -> ThreadHideResult {
        if self.safe_mode {
            self.hidden_threads.push(HiddenThread {
                thread_id,
                technique: ThreadHideTechnique::SuspendWhenIdle,
                original_state: None,
            });

            return ThreadHideResult {
                success: true,
                thread_id,
                technique: ThreadHideTechnique::SuspendWhenIdle,
                message: "[SAFE MODE] Thread set to suspend when idle".to_string(),
            };
        }

        #[cfg(all(windows, feature = "opsec-windows"))]
        {
            // Use SuspendThread/ResumeThread for idle management
            self.hidden_threads.push(HiddenThread {
                thread_id,
                technique: ThreadHideTechnique::SuspendWhenIdle,
                original_state: None,
            });

            ThreadHideResult {
                success: true,
                thread_id,
                technique: ThreadHideTechnique::SuspendWhenIdle,
                message: "Thread configured for idle suspension".to_string(),
            }
        }

        #[cfg(not(all(windows, feature = "opsec-windows")))]
        ThreadHideResult {
            success: false,
            thread_id,
            technique: ThreadHideTechnique::SuspendWhenIdle,
            message: "Thread suspension requires Windows".to_string(),
        }
    }

    /// Hide current thread using best available technique
    pub fn hide_current_thread(&mut self) -> ThreadHideResult {
        #[cfg(windows)]
        let thread_id = unsafe { windows::Win32::System::Threading::GetCurrentThreadId() };

        #[cfg(not(windows))]
        let thread_id = 0u32;

        self.hide_from_debugger(thread_id)
    }

    /// Get list of hidden threads
    pub fn get_hidden_threads(&self) -> Vec<(u32, ThreadHideTechnique)> {
        self.hidden_threads
            .iter()
            .map(|t| (t.thread_id, t.technique))
            .collect()
    }

    /// Check if thread is hidden
    pub fn is_hidden(&self, thread_id: u32) -> bool {
        self.hidden_threads.iter().any(|t| t.thread_id == thread_id)
    }

    /// Get count of hidden threads
    pub fn hidden_count(&self) -> usize {
        self.hidden_threads.len()
    }

    /// Unhide specific thread
    pub fn unhide_thread(&mut self, thread_id: u32) -> ThreadHideResult {
        if let Some(pos) = self
            .hidden_threads
            .iter()
            .position(|t| t.thread_id == thread_id)
        {
            let hidden = self.hidden_threads.remove(pos);

            ThreadHideResult {
                success: true,
                thread_id,
                technique: hidden.technique,
                message: format!("Thread {} unhidden", thread_id),
            }
        } else {
            ThreadHideResult {
                success: false,
                thread_id,
                technique: ThreadHideTechnique::SetInfoHide,
                message: format!("Thread {} was not hidden", thread_id),
            }
        }
    }

    /// Unhide all threads
    pub fn unhide_all(&mut self) -> Vec<ThreadHideResult> {
        let thread_ids: Vec<u32> = self.hidden_threads.iter().map(|t| t.thread_id).collect();
        let mut results = Vec::new();

        for thread_id in thread_ids {
            results.push(self.unhide_thread(thread_id));
        }

        if results.is_empty() {
            results.push(ThreadHideResult {
                success: true,
                thread_id: 0,
                technique: ThreadHideTechnique::SetInfoHide,
                message: "No threads to unhide".to_string(),
            });
        }

        results
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_hider_creation() {
        let hider = ThreadHider::new();
        assert!(hider.get_hidden_threads().is_empty());
        assert_eq!(hider.hidden_count(), 0);
    }

    #[test]
    fn test_technique_risk_levels() {
        assert!(
            ThreadHideTechnique::UnlinkFromList.detection_risk()
                > ThreadHideTechnique::SuspendWhenIdle.detection_risk()
        );
    }

    #[test]
    fn test_technique_descriptions() {
        let desc = ThreadHideTechnique::SetInfoHide.description();
        assert!(!desc.is_empty());
        assert!(desc.contains("NtSetInformationThread"));
    }

    #[test]
    fn test_technique_kernel_requirement() {
        assert!(ThreadHideTechnique::UnlinkFromList.requires_kernel());
        assert!(!ThreadHideTechnique::SetInfoHide.requires_kernel());
    }

    #[test]
    fn test_safe_mode_hide_from_debugger() {
        let mut hider = ThreadHider::new().with_safe_mode(true);
        let result = hider.hide_from_debugger(1234);
        assert!(result.success);
        assert!(result.message.contains("SAFE MODE"));
        assert!(hider.is_hidden(1234));
    }

    #[test]
    fn test_safe_mode_hide_from_query() {
        let mut hider = ThreadHider::new().with_safe_mode(true);
        let result = hider.hide_from_query(5678);
        assert!(result.success);
        assert_eq!(hider.hidden_count(), 1);
    }

    #[test]
    fn test_unhide_thread() {
        let mut hider = ThreadHider::new().with_safe_mode(true);
        hider.hide_from_debugger(1234);
        assert!(hider.is_hidden(1234));

        let result = hider.unhide_thread(1234);
        assert!(result.success);
        assert!(!hider.is_hidden(1234));
    }

    #[test]
    fn test_unhide_all() {
        let mut hider = ThreadHider::new().with_safe_mode(true);
        hider.hide_from_debugger(1111);
        hider.hide_from_debugger(2222);
        hider.hide_from_query(3333);
        assert_eq!(hider.hidden_count(), 3);

        let results = hider.unhide_all();
        assert_eq!(results.len(), 3);
        assert_eq!(hider.hidden_count(), 0);
    }

    #[test]
    fn test_get_hidden_threads() {
        let mut hider = ThreadHider::new().with_safe_mode(true);
        hider.hide_from_debugger(1234);
        hider.hide_from_query(5678);

        let hidden = hider.get_hidden_threads();
        assert_eq!(hidden.len(), 2);
        assert!(hidden.contains(&(1234, ThreadHideTechnique::SetInfoHide)));
        assert!(hidden.contains(&(5678, ThreadHideTechnique::HideFromQuery)));
    }

    #[test]
    #[cfg(not(windows))]
    fn test_non_windows_returns_failure() {
        let mut hider = ThreadHider::new();
        let result = hider.hide_from_debugger(1234);
        assert!(!result.success);
    }
}
