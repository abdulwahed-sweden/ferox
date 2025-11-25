//! Tauri Commands Module
//!
//! All Tauri command handlers for frontend-backend communication.

pub mod session_commands;
pub mod terminal_commands;
pub mod module_commands;
pub mod payload_commands;
pub mod simulation_commands;

// Phase 2: New command modules for CLI-GUI integration
pub mod scanner_commands;
pub mod recon_commands;
pub mod opsec_commands;
pub mod network_commands;
