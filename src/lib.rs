//! Ferox v2.0 - Ferocious Security Framework
//!
//! Fast, Fierce, Fearless offensive security framework built in Rust.

// Allow dead code for framework APIs that will be used in future phases
#![allow(dead_code)]

pub mod cli;
pub mod core;
pub mod handlers;
pub mod modules;
pub mod tools;
// Phase 3 infrastructure modules
pub mod infra; // new lightweight infrastructure namespace (crypto, etc.)

#[cfg(feature = "memory-forensics")]
pub mod memory_forensics;
