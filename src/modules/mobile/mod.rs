//! Mobile Application Security Analysis Modules
//!
//! This module provides security analysis capabilities for mobile applications.
//! Supports both Android (APK) and iOS (IPA) application analysis.
//!
//! ## Modules
//!
//! - `apk_analyzer` - Android APK static analysis
//! - `ipa_analyzer` - iOS IPA static analysis
//! - `app_recon` - Mobile app reconnaissance and information gathering
//!
//! ## Usage
//!
//! These modules are designed for **authorized security assessments only**.
//! Always ensure you have proper authorization before analyzing applications.
//!
//! ```rust,ignore
//! use ferox::modules::mobile::{ApkAnalyzer, IpaAnalyzer, AppRecon};
//!
//! // Analyze an Android APK
//! let mut analyzer = ApkAnalyzer::new();
//! analyzer.set_option("APK_PATH", "/path/to/app.apk")?;
//! let result = analyzer.run().await?;
//!
//! // Analyze an iOS IPA
//! let mut analyzer = IpaAnalyzer::new();
//! analyzer.set_option("IPA_PATH", "/path/to/app.ipa")?;
//! let result = analyzer.run().await?;
//! ```

pub mod apk_analyzer;
pub mod app_recon;
pub mod ipa_analyzer;

pub use apk_analyzer::ApkAnalyzer;
pub use app_recon::AppRecon;
pub use ipa_analyzer::IpaAnalyzer;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Common mobile platform types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MobilePlatform {
    Android,
    #[serde(rename = "iOS")]
    Ios,
    #[default]
    Unknown,
}

impl std::fmt::Display for MobilePlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MobilePlatform::Android => write!(f, "Android"),
            MobilePlatform::Ios => write!(f, "iOS"),
            MobilePlatform::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Security finding severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "Info"),
            Severity::Low => write!(f, "Low"),
            Severity::Medium => write!(f, "Medium"),
            Severity::High => write!(f, "High"),
            Severity::Critical => write!(f, "Critical"),
        }
    }
}

/// A security finding from mobile app analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFinding {
    /// Unique identifier for this finding type
    pub id: String,
    /// Finding title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Severity level
    pub severity: Severity,
    /// Category (e.g., "Permissions", "Network", "Storage", "Crypto")
    pub category: String,
    /// Affected component or file
    pub location: Option<String>,
    /// OWASP Mobile Top 10 mapping (e.g., "M1", "M2")
    pub owasp_mobile: Option<String>,
    /// CWE ID if applicable
    pub cwe_id: Option<String>,
    /// Remediation advice
    pub remediation: Option<String>,
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl SecurityFinding {
    pub fn new(id: &str, title: &str, severity: Severity, category: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            description: String::new(),
            severity,
            category: category.to_string(),
            location: None,
            owasp_mobile: None,
            cwe_id: None,
            remediation: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_location(mut self, loc: &str) -> Self {
        self.location = Some(loc.to_string());
        self
    }

    pub fn with_owasp(mut self, owasp: &str) -> Self {
        self.owasp_mobile = Some(owasp.to_string());
        self
    }

    pub fn with_cwe(mut self, cwe: &str) -> Self {
        self.cwe_id = Some(cwe.to_string());
        self
    }

    pub fn with_remediation(mut self, rem: &str) -> Self {
        self.remediation = Some(rem.to_string());
        self
    }
}

/// Mobile app metadata extracted during analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppMetadata {
    /// Application name
    pub app_name: Option<String>,
    /// Package/Bundle identifier
    pub package_id: Option<String>,
    /// Version string
    pub version: Option<String>,
    /// Version code (Android) or build number (iOS)
    pub version_code: Option<String>,
    /// Minimum SDK/OS version
    pub min_sdk: Option<String>,
    /// Target SDK/OS version
    pub target_sdk: Option<String>,
    /// Platform
    pub platform: MobilePlatform,
    /// Is debuggable
    pub debuggable: bool,
    /// Signing information
    pub signing_info: Option<SigningInfo>,
    /// Main activity/entry point
    pub main_activity: Option<String>,
    /// Additional metadata
    #[serde(default)]
    pub extra: HashMap<String, String>,
}

/// Application signing information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SigningInfo {
    /// Certificate subject
    pub subject: Option<String>,
    /// Certificate issuer
    pub issuer: Option<String>,
    /// Certificate serial number
    pub serial: Option<String>,
    /// Valid from date
    pub valid_from: Option<String>,
    /// Valid to date
    pub valid_to: Option<String>,
    /// Signature algorithm
    pub algorithm: Option<String>,
    /// SHA-256 fingerprint
    pub sha256_fingerprint: Option<String>,
    /// Is self-signed
    pub is_self_signed: bool,
}

/// Analysis result summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalysisSummary {
    /// Total findings count
    pub total_findings: usize,
    /// Critical findings
    pub critical_count: usize,
    /// High severity findings
    pub high_count: usize,
    /// Medium severity findings
    pub medium_count: usize,
    /// Low severity findings
    pub low_count: usize,
    /// Informational findings
    pub info_count: usize,
    /// Risk score (0-100)
    pub risk_score: u8,
    /// Analysis duration in milliseconds
    pub duration_ms: u64,
}

impl AnalysisSummary {
    pub fn from_findings(findings: &[SecurityFinding], duration_ms: u64) -> Self {
        let mut summary = Self {
            total_findings: findings.len(),
            duration_ms,
            ..Default::default()
        };

        for finding in findings {
            match finding.severity {
                Severity::Critical => summary.critical_count += 1,
                Severity::High => summary.high_count += 1,
                Severity::Medium => summary.medium_count += 1,
                Severity::Low => summary.low_count += 1,
                Severity::Info => summary.info_count += 1,
            }
        }

        // Calculate risk score
        summary.risk_score = Self::calculate_risk_score(&summary);
        summary
    }

    fn calculate_risk_score(summary: &Self) -> u8 {
        let score = (summary.critical_count * 25)
            + (summary.high_count * 15)
            + (summary.medium_count * 8)
            + (summary.low_count * 3)
            + summary.info_count;

        std::cmp::min(100, score) as u8
    }
}

/// Permission analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionInfo {
    /// Permission name
    pub name: String,
    /// Permission protection level
    pub protection_level: String,
    /// Is dangerous permission
    pub is_dangerous: bool,
    /// Description
    pub description: Option<String>,
}

/// Network security configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    /// Allows cleartext traffic
    pub allows_cleartext: bool,
    /// Certificate pinning enabled
    pub has_cert_pinning: bool,
    /// Trust anchors configured
    pub trust_anchors: Vec<String>,
    /// Custom SSL configurations
    pub custom_ssl_config: bool,
}

/// Storage analysis result
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageAnalysis {
    /// Uses shared preferences (Android) / UserDefaults (iOS)
    pub uses_local_storage: bool,
    /// Uses SQLite database
    pub uses_database: bool,
    /// Uses file storage
    pub uses_file_storage: bool,
    /// Uses keychain/keystore
    pub uses_secure_storage: bool,
    /// External storage access (Android)
    pub uses_external_storage: bool,
    /// Potential hardcoded secrets found
    pub hardcoded_secrets: Vec<HardcodedSecret>,
}

/// Hardcoded secret finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardcodedSecret {
    /// Type of secret (API_KEY, PASSWORD, TOKEN, etc.)
    pub secret_type: String,
    /// File where found
    pub file: String,
    /// Line number if applicable
    pub line: Option<usize>,
    /// Masked value (first/last chars visible)
    pub masked_value: String,
    /// Confidence level (0-100)
    pub confidence: u8,
}

/// Component export analysis (Android)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedComponent {
    /// Component name
    pub name: String,
    /// Component type (Activity, Service, Receiver, Provider)
    pub component_type: String,
    /// Is exported
    pub exported: bool,
    /// Required permission
    pub permission: Option<String>,
    /// Intent filters
    pub intent_filters: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_finding_builder() {
        let finding = SecurityFinding::new("TEST-001", "Test Finding", Severity::High, "Test")
            .with_description("A test finding")
            .with_location("TestFile.java")
            .with_owasp("M1")
            .with_cwe("CWE-312");

        assert_eq!(finding.id, "TEST-001");
        assert_eq!(finding.severity, Severity::High);
        assert_eq!(finding.owasp_mobile, Some("M1".to_string()));
    }

    #[test]
    fn test_analysis_summary() {
        let findings = vec![
            SecurityFinding::new("F1", "Critical", Severity::Critical, "Test"),
            SecurityFinding::new("F2", "High", Severity::High, "Test"),
            SecurityFinding::new("F3", "Medium", Severity::Medium, "Test"),
            SecurityFinding::new("F4", "Low", Severity::Low, "Test"),
        ];

        let summary = AnalysisSummary::from_findings(&findings, 1000);

        assert_eq!(summary.total_findings, 4);
        assert_eq!(summary.critical_count, 1);
        assert_eq!(summary.high_count, 1);
        assert!(summary.risk_score > 0);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
        assert!(Severity::Low > Severity::Info);
    }
}
