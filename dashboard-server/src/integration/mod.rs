//! Integration layer between Dashboard and Ferox Core
//!
//! This module provides the bridge between the dashboard server and
//! the Ferox core engine for real command execution and post-exploitation.

pub mod bridge;
pub mod modules;
pub mod session_sync;

pub use bridge::FeroxBridge;
pub use modules::ModuleBridge;
pub use session_sync::spawn_session_sync;
