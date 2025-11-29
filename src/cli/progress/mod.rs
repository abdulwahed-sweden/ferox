//! Progress Tracking System
//!
//! Provides real-time progress feedback for module execution including:
//! - Progress bars with ETA
//! - Spinners for unknown duration operations
//! - Multi-stage progress tracking
//! - Live discovery notifications

mod tracker;
mod bars;
mod spinner;
mod multi;

pub use tracker::{ProgressTracker, ProgressUpdate, ProgressCallback};
pub use bars::{ProgressBar, ProgressStyle};
pub use spinner::Spinner;
pub use multi::{MultiProgress, PhaseProgress};
