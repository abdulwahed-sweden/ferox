//! Post-exploitation modules for authorized security testing.
//! All modules include safe modes and reference implementations for educational purposes.

pub mod browser;
pub mod capture;
pub mod credential_collector;
pub mod credential_harvester;
pub mod enum_modules;
pub mod lateral_movement;
pub mod persistence;
pub mod privilege_escalation;

// Re-export capture modules
pub use capture::{ClipboardCapture, KeylogCapture, ScreenshotCapture};

// Re-export enumeration modules
pub use enum_modules::{
    FileDownloadModule, FileSearchModule, FullSituationalModule, NetworkEnum, ProcessesEnum,
    SystemInfoEnum, UsersEnum,
};
