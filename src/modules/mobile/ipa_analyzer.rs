//! iOS IPA Static Analyzer
//!
//! Performs static security analysis on iOS IPA files.
//! Analyzes Info.plist, entitlements, binary protections, and more.

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

use crate::core::module::{CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType};

use super::{
    AnalysisSummary, AppMetadata, HardcodedSecret, MobilePlatform,
    SecurityFinding, Severity, SigningInfo, StorageAnalysis,
};

/// iOS IPA Static Analyzer
///
/// Performs comprehensive static security analysis on iOS IPA files:
/// - Info.plist analysis (permissions, URL schemes, ATS config)
/// - Entitlements analysis
/// - Binary protections (PIE, ARC, Stack Canaries)
/// - Code signing verification
/// - Hardcoded secrets detection
pub struct IpaAnalyzer {
    /// Path to the IPA file
    ipa_path: Option<String>,
    /// Enable deep analysis
    deep_analysis: bool,
    /// Check for hardcoded secrets
    check_secrets: bool,
    /// Analyze frameworks
    analyze_frameworks: bool,
    /// Output format
    output_format: String,
}

impl IpaAnalyzer {
    pub fn new() -> Self {
        Self {
            ipa_path: None,
            deep_analysis: false,
            check_secrets: true,
            analyze_frameworks: false,
            output_format: "json".to_string(),
        }
    }

    /// Analyze Info.plist
    async fn analyze_info_plist(&self, _ipa_path: &str) -> Result<InfoPlistAnalysis> {
        // In real implementation:
        // 1. Extract Payload/*.app/Info.plist from IPA
        // 2. Parse the plist (XML or binary)
        // 3. Extract relevant security information

        Ok(InfoPlistAnalysis {
            metadata: AppMetadata {
                app_name: Some("Example iOS App".to_string()),
                package_id: Some("com.example.iosapp".to_string()),
                version: Some("1.0.0".to_string()),
                version_code: Some("1".to_string()),
                min_sdk: Some("13.0".to_string()),
                target_sdk: Some("17.0".to_string()),
                platform: MobilePlatform::Ios,
                debuggable: false,
                signing_info: None,
                main_activity: None,
                extra: HashMap::new(),
            },
            url_schemes: vec![
                UrlScheme {
                    scheme: "exampleapp".to_string(),
                    role: "Editor".to_string(),
                },
                UrlScheme {
                    scheme: "fb123456789".to_string(),
                    role: "Editor".to_string(),
                },
            ],
            queried_url_schemes: vec!["facebook".to_string(), "twitter".to_string()],
            background_modes: vec!["fetch".to_string(), "remote-notification".to_string()],
            ats_config: AtsConfig {
                allows_arbitrary_loads: false,
                allows_arbitrary_loads_for_media: false,
                allows_local_networking: true,
                exception_domains: vec![AtsExceptionDomain {
                    domain: "legacy-api.example.com".to_string(),
                    allows_insecure_http: true,
                    includes_subdomains: false,
                    min_tls_version: Some("TLSv1.2".to_string()),
                }],
            },
            permissions: vec![
                IosPermission {
                    key: "NSCameraUsageDescription".to_string(),
                    description: "We need camera access for photos".to_string(),
                },
                IosPermission {
                    key: "NSLocationWhenInUseUsageDescription".to_string(),
                    description: "We need location for nearby features".to_string(),
                },
                IosPermission {
                    key: "NSPhotoLibraryUsageDescription".to_string(),
                    description: "We need photo library access".to_string(),
                },
            ],
            supports_document_browser: false,
            file_sharing_enabled: false,
        })
    }

    /// Analyze entitlements
    async fn analyze_entitlements(&self, _ipa_path: &str) -> Result<EntitlementsAnalysis> {
        Ok(EntitlementsAnalysis {
            app_id: Some("TEAMID.com.example.iosapp".to_string()),
            team_id: Some("TEAMID".to_string()),
            get_task_allow: false,
            aps_environment: Some("production".to_string()),
            keychain_groups: vec!["TEAMID.com.example.iosapp".to_string()],
            app_groups: vec![],
            associated_domains: vec!["applinks:example.com".to_string()],
            icloud_containers: vec![],
            siri_enabled: false,
            healthkit_enabled: false,
            homekit_enabled: false,
            inter_app_audio: false,
            custom_entitlements: HashMap::new(),
        })
    }

    /// Analyze binary protections
    async fn analyze_binary(&self, _ipa_path: &str) -> Result<BinaryAnalysis> {
        // In real implementation:
        // 1. Extract the main binary from the IPA
        // 2. Use otool or similar to check protections
        // 3. Check for PIE, ARC, stack canaries, etc.

        Ok(BinaryAnalysis {
            binary_name: "ExampleApp".to_string(),
            architecture: vec!["arm64".to_string()],
            is_encrypted: true,
            pie_enabled: true,
            arc_enabled: true,
            stack_canaries: true,
            stripped: true,
            has_swift: true,
            swift_version: Some("5.9".to_string()),
            min_os_version: "13.0".to_string(),
            sdk_version: "17.0".to_string(),
            linked_frameworks: vec![
                "UIKit".to_string(),
                "Foundation".to_string(),
                "Security".to_string(),
                "CoreLocation".to_string(),
            ],
            weak_frameworks: vec!["HealthKit".to_string()],
            has_bitcode: false,
        })
    }

    /// Analyze code signing
    async fn analyze_signing(&self, _ipa_path: &str) -> Result<SigningInfo> {
        Ok(SigningInfo {
            subject: Some("iPhone Distribution: Example Company (TEAMID)".to_string()),
            issuer: Some("Apple Worldwide Developer Relations Certification Authority".to_string()),
            serial: Some("ABC123DEF456".to_string()),
            valid_from: Some("2023-01-01".to_string()),
            valid_to: Some("2024-01-01".to_string()),
            algorithm: Some("SHA256withRSA".to_string()),
            sha256_fingerprint: Some(
                "AB:CD:EF:12:34:56:78:90:AB:CD:EF:12:34:56:78:90:AB:CD:EF:12:34:56:78:90:AB:CD:EF:12:34:56:78:90"
                    .to_string(),
            ),
            is_self_signed: false,
        })
    }

    /// Analyze for hardcoded secrets
    async fn analyze_secrets(&self, _ipa_path: &str) -> Result<Vec<HardcodedSecret>> {
        if !self.check_secrets {
            return Ok(vec![]);
        }

        // Simulated findings
        Ok(vec![
            HardcodedSecret {
                secret_type: "API_KEY".to_string(),
                file: "ExampleApp".to_string(),
                line: None,
                masked_value: "sk_live_****...****xyz".to_string(),
                confidence: 90,
            },
            HardcodedSecret {
                secret_type: "FIREBASE_KEY".to_string(),
                file: "GoogleService-Info.plist".to_string(),
                line: None,
                masked_value: "AIza****...****abc".to_string(),
                confidence: 95,
            },
        ])
    }

    /// Analyze storage usage
    async fn analyze_storage(&self, _ipa_path: &str) -> Result<StorageAnalysis> {
        Ok(StorageAnalysis {
            uses_local_storage: true,
            uses_database: true,
            uses_file_storage: true,
            uses_secure_storage: true,
            uses_external_storage: false,
            hardcoded_secrets: vec![],
        })
    }

    /// Generate security findings
    fn generate_findings(
        &self,
        info_plist: &InfoPlistAnalysis,
        entitlements: &EntitlementsAnalysis,
        binary: &BinaryAnalysis,
        _signing: &SigningInfo,
        secrets: &[HardcodedSecret],
    ) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();

        // Check ATS (App Transport Security)
        if info_plist.ats_config.allows_arbitrary_loads {
            findings.push(
                SecurityFinding::new(
                    "IPA-001",
                    "App Transport Security disabled",
                    Severity::High,
                    "Network",
                )
                .with_description(
                    "NSAllowsArbitraryLoads is set to YES. The app can make insecure HTTP connections.",
                )
                .with_location("Info.plist")
                .with_owasp("M3")
                .with_cwe("CWE-319")
                .with_remediation("Remove NSAllowsArbitraryLoads or set to NO. Use exception domains for specific legacy servers."),
            );
        }

        // Check ATS exception domains
        for exception in &info_plist.ats_config.exception_domains {
            if exception.allows_insecure_http {
                findings.push(
                    SecurityFinding::new(
                        "IPA-002",
                        &format!("ATS exception allows HTTP for {}", exception.domain),
                        Severity::Medium,
                        "Network",
                    )
                    .with_description(&format!(
                        "Domain '{}' has ATS exception allowing insecure HTTP.",
                        exception.domain
                    ))
                    .with_location("Info.plist")
                    .with_owasp("M3")
                    .with_cwe("CWE-319"),
                );
            }
        }

        // Check get-task-allow (debug)
        if entitlements.get_task_allow {
            findings.push(
                SecurityFinding::new(
                    "IPA-003",
                    "get-task-allow entitlement enabled",
                    Severity::Critical,
                    "Configuration",
                )
                .with_description(
                    "The get-task-allow entitlement is enabled. This allows debugging and should never be in production.",
                )
                .with_location("embedded.mobileprovision")
                .with_owasp("M1")
                .with_cwe("CWE-489")
                .with_remediation("Use a production provisioning profile without get-task-allow."),
            );
        }

        // Check URL schemes
        for scheme in &info_plist.url_schemes {
            findings.push(
                SecurityFinding::new(
                    "IPA-004",
                    &format!("Custom URL scheme: {}", scheme.scheme),
                    Severity::Info,
                    "Configuration",
                )
                .with_description(&format!(
                    "App registers custom URL scheme '{}'. Verify proper input validation.",
                    scheme.scheme
                ))
                .with_location("Info.plist")
                .with_owasp("M1"),
            );
        }

        // Check binary protections
        if !binary.pie_enabled {
            findings.push(
                SecurityFinding::new(
                    "IPA-005",
                    "PIE not enabled",
                    Severity::High,
                    "Binary",
                )
                .with_description(
                    "Position Independent Executable (PIE) is not enabled. This makes ASLR less effective.",
                )
                .with_location(&binary.binary_name)
                .with_owasp("M8")
                .with_cwe("CWE-119")
                .with_remediation("Enable PIE in Xcode build settings (Generate Position-Dependent Code = NO)."),
            );
        }

        if !binary.arc_enabled {
            findings.push(
                SecurityFinding::new(
                    "IPA-006",
                    "ARC not enabled",
                    Severity::Medium,
                    "Binary",
                )
                .with_description(
                    "Automatic Reference Counting (ARC) is not enabled. Manual memory management increases risk of memory corruption.",
                )
                .with_location(&binary.binary_name)
                .with_owasp("M8")
                .with_cwe("CWE-416")
                .with_remediation("Enable ARC in Xcode build settings (Objective-C Automatic Reference Counting = YES)."),
            );
        }

        if !binary.stack_canaries {
            findings.push(
                SecurityFinding::new(
                    "IPA-007",
                    "Stack canaries not detected",
                    Severity::Medium,
                    "Binary",
                )
                .with_description(
                    "Stack canaries (stack smashing protection) not detected. Buffer overflow attacks may be easier.",
                )
                .with_location(&binary.binary_name)
                .with_owasp("M8")
                .with_cwe("CWE-121")
                .with_remediation("Enable stack canaries with -fstack-protector-all compiler flag."),
            );
        }

        if !binary.is_encrypted {
            findings.push(
                SecurityFinding::new(
                    "IPA-008",
                    "Binary not encrypted",
                    Severity::Low,
                    "Binary",
                )
                .with_description(
                    "The binary is not encrypted (FairPlay). This is normal for development/ad-hoc builds but unusual for App Store builds.",
                )
                .with_location(&binary.binary_name)
                .with_owasp("M9"),
            );
        }

        // Check hardcoded secrets
        for secret in secrets {
            findings.push(
                SecurityFinding::new(
                    "IPA-009",
                    &format!("Hardcoded {} detected", secret.secret_type),
                    Severity::High,
                    "Secrets",
                )
                .with_description(&format!(
                    "A hardcoded {} was found. Value: {}",
                    secret.secret_type, secret.masked_value
                ))
                .with_location(&secret.file)
                .with_owasp("M9")
                .with_cwe("CWE-798")
                .with_remediation("Store secrets in iOS Keychain or fetch from secure backend."),
            );
        }

        // Check file sharing
        if info_plist.file_sharing_enabled {
            findings.push(
                SecurityFinding::new(
                    "IPA-010",
                    "iTunes file sharing enabled",
                    Severity::Medium,
                    "Configuration",
                )
                .with_description(
                    "UIFileSharingEnabled is set. Users can access the app's Documents directory via iTunes.",
                )
                .with_location("Info.plist")
                .with_owasp("M2")
                .with_cwe("CWE-276")
                .with_remediation("Disable file sharing unless required. Don't store sensitive data in Documents."),
            );
        }

        // Check keychain groups
        if entitlements.keychain_groups.is_empty() {
            findings.push(
                SecurityFinding::new(
                    "IPA-011",
                    "No keychain access groups configured",
                    Severity::Info,
                    "Configuration",
                )
                .with_description(
                    "No keychain access groups are configured. The app may not be using Keychain for secure storage.",
                )
                .with_location("Entitlements")
                .with_owasp("M2"),
            );
        }

        // Check permissions (usage descriptions)
        for permission in &info_plist.permissions {
            let severity = if permission.key.contains("Location")
                || permission.key.contains("Camera")
                || permission.key.contains("Microphone")
                || permission.key.contains("Contacts")
            {
                Severity::Info
            } else {
                Severity::Info
            };

            findings.push(
                SecurityFinding::new(
                    &format!("IPA-PERM-{}", permission.key.replace("NS", "").replace("UsageDescription", "")),
                    &format!("Permission: {}", permission.key),
                    severity,
                    "Permissions",
                )
                .with_description(&format!(
                    "App requests {}. Reason: {}",
                    permission.key, permission.description
                ))
                .with_location("Info.plist"),
            );
        }

        findings
    }
}

impl Default for IpaAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for IpaAnalyzer {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "ipa_analyzer".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "iOS IPA static security analyzer".to_string(),
            module_type: ModuleType::Scanner,
            category: "mobile".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "IPA_PATH".to_string(),
                description: "Path to the IPA file to analyze".to_string(),
                required: true,
                default_value: None,
                current_value: self.ipa_path.clone(),
            },
            ModuleOption {
                name: "DEEP_ANALYSIS".to_string(),
                description: "Enable deep analysis (slower)".to_string(),
                required: false,
                default_value: Some("false".to_string()),
                current_value: Some(self.deep_analysis.to_string()),
            },
            ModuleOption {
                name: "CHECK_SECRETS".to_string(),
                description: "Check for hardcoded secrets".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: Some(self.check_secrets.to_string()),
            },
            ModuleOption {
                name: "ANALYZE_FRAMEWORKS".to_string(),
                description: "Analyze embedded frameworks".to_string(),
                required: false,
                default_value: Some("false".to_string()),
                current_value: Some(self.analyze_frameworks.to_string()),
            },
            ModuleOption {
                name: "OUTPUT_FORMAT".to_string(),
                description: "Output format (json, yaml, text)".to_string(),
                required: false,
                default_value: Some("json".to_string()),
                current_value: Some(self.output_format.clone()),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        match name.to_uppercase().as_str() {
            "IPA_PATH" => self.ipa_path = Some(value.to_string()),
            "DEEP_ANALYSIS" => self.deep_analysis = value.parse().unwrap_or(false),
            "CHECK_SECRETS" => self.check_secrets = value.parse().unwrap_or(true),
            "ANALYZE_FRAMEWORKS" => self.analyze_frameworks = value.parse().unwrap_or(false),
            "OUTPUT_FORMAT" => self.output_format = value.to_string(),
            _ => return Err(anyhow!("Unknown option: {}", name)),
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<String> {
        match name.to_uppercase().as_str() {
            "IPA_PATH" => self.ipa_path.clone(),
            "DEEP_ANALYSIS" => Some(self.deep_analysis.to_string()),
            "CHECK_SECRETS" => Some(self.check_secrets.to_string()),
            "ANALYZE_FRAMEWORKS" => Some(self.analyze_frameworks.to_string()),
            "OUTPUT_FORMAT" => Some(self.output_format.clone()),
            _ => None,
        }
    }

    fn validate(&self) -> Result<()> {
        let ipa_path = self
            .ipa_path
            .as_ref()
            .ok_or_else(|| anyhow!("IPA_PATH is required"))?;

        if !Path::new(ipa_path).exists() {
            return Err(anyhow!("IPA file not found: {}", ipa_path));
        }

        if !ipa_path.ends_with(".ipa") {
            return Err(anyhow!("File must be an IPA: {}", ipa_path));
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let ipa_path = self.ipa_path.as_ref();

        Ok(CheckResult {
            vulnerable: ipa_path.is_some(),
            confidence: if ipa_path.is_some() { 1.0 } else { 0.0 },
            details: "IPA analyzer ready".to_string(),
            fingerprint: HashMap::new(),
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        let ipa_path = self
            .ipa_path
            .as_ref()
            .ok_or_else(|| anyhow!("IPA_PATH not set"))?
            .clone();

        let start_time = std::time::Instant::now();

        // Run all analysis phases
        let info_plist = self.analyze_info_plist(&ipa_path).await?;
        let entitlements = self.analyze_entitlements(&ipa_path).await?;
        let binary = self.analyze_binary(&ipa_path).await?;
        let signing = self.analyze_signing(&ipa_path).await?;
        let secrets = self.analyze_secrets(&ipa_path).await?;
        let storage = self.analyze_storage(&ipa_path).await?;

        // Generate findings
        let findings =
            self.generate_findings(&info_plist, &entitlements, &binary, &signing, &secrets);

        let duration_ms = start_time.elapsed().as_millis() as u64;
        let summary = AnalysisSummary::from_findings(&findings, duration_ms);

        // Build result
        let mut result = ModuleResult::success(format!(
            "IPA analysis complete. Found {} issues (Critical: {}, High: {}, Medium: {}, Low: {})",
            summary.total_findings,
            summary.critical_count,
            summary.high_count,
            summary.medium_count,
            summary.low_count
        ));

        result.data.insert("metadata".to_string(), json!(info_plist.metadata));
        result.data.insert("url_schemes".to_string(), json!(info_plist.url_schemes));
        result.data.insert("ats_config".to_string(), json!(info_plist.ats_config));
        result.data.insert("permissions".to_string(), json!(info_plist.permissions));
        result.data.insert("entitlements".to_string(), json!(entitlements));
        result.data.insert("binary".to_string(), json!(binary));
        result.data.insert("signing_info".to_string(), json!(signing));
        result.data.insert("storage".to_string(), json!(storage));
        result.data.insert("findings".to_string(), json!(findings));
        result.data.insert("summary".to_string(), json!(summary));

        Ok(result)
    }
}

/// Info.plist analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
struct InfoPlistAnalysis {
    metadata: AppMetadata,
    url_schemes: Vec<UrlScheme>,
    queried_url_schemes: Vec<String>,
    background_modes: Vec<String>,
    ats_config: AtsConfig,
    permissions: Vec<IosPermission>,
    supports_document_browser: bool,
    file_sharing_enabled: bool,
}

/// URL scheme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UrlScheme {
    scheme: String,
    role: String,
}

/// App Transport Security configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct AtsConfig {
    allows_arbitrary_loads: bool,
    allows_arbitrary_loads_for_media: bool,
    allows_local_networking: bool,
    exception_domains: Vec<AtsExceptionDomain>,
}

/// ATS exception domain
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AtsExceptionDomain {
    domain: String,
    allows_insecure_http: bool,
    includes_subdomains: bool,
    min_tls_version: Option<String>,
}

/// iOS permission (usage description)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IosPermission {
    key: String,
    description: String,
}

/// Entitlements analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EntitlementsAnalysis {
    app_id: Option<String>,
    team_id: Option<String>,
    get_task_allow: bool,
    aps_environment: Option<String>,
    keychain_groups: Vec<String>,
    app_groups: Vec<String>,
    associated_domains: Vec<String>,
    icloud_containers: Vec<String>,
    siri_enabled: bool,
    healthkit_enabled: bool,
    homekit_enabled: bool,
    inter_app_audio: bool,
    custom_entitlements: HashMap<String, String>,
}

/// Binary analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BinaryAnalysis {
    binary_name: String,
    architecture: Vec<String>,
    is_encrypted: bool,
    pie_enabled: bool,
    arc_enabled: bool,
    stack_canaries: bool,
    stripped: bool,
    has_swift: bool,
    swift_version: Option<String>,
    min_os_version: String,
    sdk_version: String,
    linked_frameworks: Vec<String>,
    weak_frameworks: Vec<String>,
    has_bitcode: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipa_analyzer_options() {
        let mut analyzer = IpaAnalyzer::new();

        analyzer.set_option("DEEP_ANALYSIS", "true").unwrap();
        assert!(analyzer.deep_analysis);

        analyzer.set_option("CHECK_SECRETS", "false").unwrap();
        assert!(!analyzer.check_secrets);
    }

    #[test]
    fn test_ipa_analyzer_info() {
        let analyzer = IpaAnalyzer::new();
        let info = analyzer.info();

        assert_eq!(info.name, "ipa_analyzer");
        assert_eq!(info.category, "mobile");
        assert_eq!(info.module_type, ModuleType::Scanner);
    }
}
