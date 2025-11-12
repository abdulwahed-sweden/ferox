//! Memory forensics primitives for Ferox.
//! Provides lightweight parsing and enrichment helpers for Windows memory dumps.

pub mod credential_extractor;
pub mod dump_parser;
pub mod malware_detector;
pub mod mitre_mapper;
pub mod network_analyzer;
pub mod process_analyzer;
pub mod registry_analyzer;
pub mod types;

#[cfg(feature = "volatility-bridge")]
pub mod volatility_bridge;

pub use credential_extractor::CredentialExtractor;
pub use dump_parser::DumpParser;
pub use malware_detector::MalwareDetector;
pub use mitre_mapper::MitreMapper;
pub use network_analyzer::NetworkAnalyzer;
pub use process_analyzer::ProcessAnalyzer;
pub use registry_analyzer::RegistryAnalyzer;
pub use types::*;
