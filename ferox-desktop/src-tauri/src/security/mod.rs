//! Security Module
//!
//! Input validation, audit logging, and credential protection.

pub mod audit;
pub mod validation;

pub use audit::AuditLogger;
pub use validation::{ValidationError, Validator};
