//! Payload Modules for Ferox Phase 4
//!
//! This module contains payload generation modules that leverage the
//! Smart Payload Engine for creating encrypted, fileless payloads
//! with C2 integration.
//!
//! **SECURITY NOTICE**: All payload modules are designed for AUTHORIZED
//! penetration testing, red team exercises, and security research ONLY.
//!
//! Available Modules:
//! - `rev_tcp_fileless` - Fileless reverse TCP payload with AES-GCM encryption
//!
//! Features:
//! - Memory-only execution (no disk writes)
//! - AES-256-GCM encryption
//! - Multi-stage architecture
//! - Cross-platform support (Windows/Linux/macOS)
//! - C2 channel integration

pub mod rev_tcp_fileless;

// Re-export main module structs for convenience
pub use rev_tcp_fileless::FilelessRevTcp;
