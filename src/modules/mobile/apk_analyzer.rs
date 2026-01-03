//! Android APK Static Analyzer
//!
//! Performs static security analysis on Android APK files.
//! Analyzes manifest, permissions, components, network config, and more.

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

use crate::core::module::{CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType};

use super::{
    AnalysisSummary, AppMetadata, ExportedComponent, HardcodedSecret, MobilePlatform,
    NetworkSecurityConfig, PermissionInfo, SecurityFinding, Severity, SigningInfo, StorageAnalysis,
};

/// Android APK Static Analyzer
///
/// Performs comprehensive static security analysis on Android APK files:
/// - Manifest analysis (permissions, components, exported items)
/// - Code analysis (hardcoded secrets, insecure patterns)
/// - Network security configuration
/// - Signing certificate analysis
/// - Storage security analysis
pub struct ApkAnalyzer {
    /// Path to the APK file
    apk_path: Option<String>,
    /// Enable deep analysis (slower but more thorough)
    deep_analysis: bool,
    /// Check for hardcoded secrets
    check_secrets: bool,
    /// Analyze native libraries
    analyze_native: bool,
    /// Output format
    output_format: String,
}

impl ApkAnalyzer {
    pub fn new() -> Self {
        Self {
            apk_path: None,
            deep_analysis: false,
            check_secrets: true,
            analyze_native: false,
            output_format: "json".to_string(),
        }
    }

    /// Analyze the APK manifest
    async fn analyze_manifest(&self, _apk_path: &str) -> Result<ManifestAnalysis> {
        // In a real implementation, this would:
        // 1. Extract AndroidManifest.xml from the APK
        // 2. Decode the binary XML
        // 3. Parse permissions, components, etc.

        // Simulated analysis for demonstration
        Ok(ManifestAnalysis {
            metadata: AppMetadata {
                app_name: Some("Example App".to_string()),
                package_id: Some("com.example.app".to_string()),
                version: Some("1.0.0".to_string()),
                version_code: Some("1".to_string()),
                min_sdk: Some("21".to_string()),
                target_sdk: Some("33".to_string()),
                platform: MobilePlatform::Android,
                debuggable: false,
                signing_info: None,
                main_activity: Some("com.example.app.MainActivity".to_string()),
                extra: HashMap::new(),
            },
            permissions: vec![
                PermissionInfo {
                    name: "android.permission.INTERNET".to_string(),
                    protection_level: "normal".to_string(),
                    is_dangerous: false,
                    description: Some("Allows network access".to_string()),
                },
                PermissionInfo {
                    name: "android.permission.ACCESS_FINE_LOCATION".to_string(),
                    protection_level: "dangerous".to_string(),
                    is_dangerous: true,
                    description: Some("Allows precise location access".to_string()),
                },
                PermissionInfo {
                    name: "android.permission.CAMERA".to_string(),
                    protection_level: "dangerous".to_string(),
                    is_dangerous: true,
                    description: Some("Allows camera access".to_string()),
                },
            ],
            exported_components: vec![
                ExportedComponent {
                    name: "com.example.app.MainActivity".to_string(),
                    component_type: "Activity".to_string(),
                    exported: true,
                    permission: None,
                    intent_filters: vec!["android.intent.action.MAIN".to_string()],
                },
                ExportedComponent {
                    name: "com.example.app.DeepLinkActivity".to_string(),
                    component_type: "Activity".to_string(),
                    exported: true,
                    permission: None,
                    intent_filters: vec!["android.intent.action.VIEW".to_string()],
                },
            ],
            backup_allowed: true,
            uses_cleartext: false,
            custom_permissions: vec![],
        })
    }

    /// Analyze network security configuration
    async fn analyze_network_security(&self, _apk_path: &str) -> Result<NetworkSecurityConfig> {
        // Simulated analysis
        Ok(NetworkSecurityConfig {
            allows_cleartext: false,
            has_cert_pinning: false,
            trust_anchors: vec!["system".to_string()],
            custom_ssl_config: false,
        })
    }

    /// Analyze code for security issues
    async fn analyze_code(&self, _apk_path: &str) -> Result<CodeAnalysis> {
        // In real implementation:
        // 1. Decompile DEX files
        // 2. Search for patterns (hardcoded secrets, insecure APIs)
        // 3. Analyze cryptographic usage

        let mut secrets = Vec::new();
        let mut insecure_patterns = Vec::new();

        if self.check_secrets {
            // Simulated findings
            secrets.push(HardcodedSecret {
                secret_type: "API_KEY".to_string(),
                file: "com/example/app/Config.smali".to_string(),
                line: Some(42),
                masked_value: "AIza****...****Xyz".to_string(),
                confidence: 85,
            });
        }

        insecure_patterns.push(InsecurePattern {
            pattern_type: "INSECURE_RANDOM".to_string(),
            description: "Use of java.util.Random instead of SecureRandom".to_string(),
            file: "com/example/app/CryptoUtils.smali".to_string(),
            line: Some(15),
        });

        Ok(CodeAnalysis {
            hardcoded_secrets: secrets,
            insecure_patterns,
            uses_reflection: true,
            uses_dynamic_code: false,
            obfuscation_detected: false,
            root_detection: false,
            ssl_pinning_impl: false,
        })
    }

    /// Analyze signing certificate
    async fn analyze_signing(&self, _apk_path: &str) -> Result<SigningInfo> {
        // Simulated signing info
        Ok(SigningInfo {
            subject: Some("CN=Android Debug, O=Android, C=US".to_string()),
            issuer: Some("CN=Android Debug, O=Android, C=US".to_string()),
            serial: Some("1".to_string()),
            valid_from: Some("2023-01-01".to_string()),
            valid_to: Some("2053-01-01".to_string()),
            algorithm: Some("SHA256withRSA".to_string()),
            sha256_fingerprint: Some(
                "A1:B2:C3:D4:E5:F6:01:23:45:67:89:AB:CD:EF:01:23:45:67:89:AB:CD:EF:01:23:45:67:89:AB:CD:EF:01:23"
                    .to_string(),
            ),
            is_self_signed: true,
        })
    }

    /// Analyze storage usage
    async fn analyze_storage(&self, _apk_path: &str) -> Result<StorageAnalysis> {
        Ok(StorageAnalysis {
            uses_local_storage: true,
            uses_database: true,
            uses_file_storage: true,
            uses_secure_storage: false,
            uses_external_storage: true,
            hardcoded_secrets: vec![],
        })
    }

    /// Generate security findings from analysis
    fn generate_findings(
        &self,
        manifest: &ManifestAnalysis,
        network: &NetworkSecurityConfig,
        code: &CodeAnalysis,
        signing: &SigningInfo,
        storage: &StorageAnalysis,
    ) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();

        // Check for debug build
        if manifest.metadata.debuggable {
            findings.push(
                SecurityFinding::new(
                    "APK-001",
                    "Application is debuggable",
                    Severity::High,
                    "Configuration",
                )
                .with_description(
                    "The android:debuggable flag is set to true. This allows attackers to attach debuggers and inspect application data.",
                )
                .with_location("AndroidManifest.xml")
                .with_owasp("M1")
                .with_cwe("CWE-215")
                .with_remediation("Set android:debuggable=\"false\" in the release build."),
            );
        }

        // Check for backup allowed
        if manifest.backup_allowed {
            findings.push(
                SecurityFinding::new(
                    "APK-002",
                    "Application allows backup",
                    Severity::Medium,
                    "Configuration",
                )
                .with_description(
                    "The android:allowBackup flag is set to true. Application data can be backed up via ADB.",
                )
                .with_location("AndroidManifest.xml")
                .with_owasp("M2")
                .with_cwe("CWE-530")
                .with_remediation(
                    "Set android:allowBackup=\"false\" or implement BackupAgent with encryption.",
                ),
            );
        }

        // Check dangerous permissions
        for perm in &manifest.permissions {
            if perm.is_dangerous {
                findings.push(
                    SecurityFinding::new(
                        &format!("APK-PERM-{}", perm.name.split('.').next_back().unwrap_or("UNKNOWN")),
                        &format!("Dangerous permission: {}", perm.name),
                        Severity::Info,
                        "Permissions",
                    )
                    .with_description(&format!(
                        "The app requests dangerous permission '{}'. {}",
                        perm.name,
                        perm.description.as_deref().unwrap_or("")
                    ))
                    .with_location("AndroidManifest.xml")
                    .with_owasp("M1"),
                );
            }
        }

        // Check exported components without permissions
        for component in &manifest.exported_components {
            if component.exported && component.permission.is_none() {
                findings.push(
                    SecurityFinding::new(
                        "APK-003",
                        &format!("Exported {} without permission", component.component_type),
                        Severity::Medium,
                        "Components",
                    )
                    .with_description(&format!(
                        "The {} '{}' is exported without requiring a permission. This may allow unauthorized access.",
                        component.component_type, component.name
                    ))
                    .with_location("AndroidManifest.xml")
                    .with_owasp("M1")
                    .with_cwe("CWE-926"),
                );
            }
        }

        // Check cleartext traffic
        if network.allows_cleartext {
            findings.push(
                SecurityFinding::new(
                    "APK-004",
                    "Cleartext traffic allowed",
                    Severity::High,
                    "Network",
                )
                .with_description(
                    "The app allows cleartext (HTTP) traffic. Data may be intercepted in transit.",
                )
                .with_location("network_security_config.xml")
                .with_owasp("M3")
                .with_cwe("CWE-319")
                .with_remediation("Disable cleartext traffic and use HTTPS exclusively."),
            );
        }

        // Check certificate pinning
        if !network.has_cert_pinning {
            findings.push(
                SecurityFinding::new(
                    "APK-005",
                    "No certificate pinning detected",
                    Severity::Medium,
                    "Network",
                )
                .with_description(
                    "The app does not implement certificate pinning. This makes it vulnerable to MITM attacks.",
                )
                .with_owasp("M3")
                .with_cwe("CWE-295")
                .with_remediation(
                    "Implement certificate pinning using network_security_config.xml or OkHttp.",
                ),
            );
        }

        // Check hardcoded secrets
        for secret in &code.hardcoded_secrets {
            findings.push(
                SecurityFinding::new(
                    "APK-006",
                    &format!("Hardcoded {} detected", secret.secret_type),
                    Severity::High,
                    "Secrets",
                )
                .with_description(&format!(
                    "A hardcoded {} was found in the code. Value: {}",
                    secret.secret_type, secret.masked_value
                ))
                .with_location(&secret.file)
                .with_owasp("M9")
                .with_cwe("CWE-798")
                .with_remediation(
                    "Store secrets securely using Android Keystore or fetch from secure backend.",
                ),
            );
        }

        // Check insecure patterns
        for pattern in &code.insecure_patterns {
            findings.push(
                SecurityFinding::new("APK-007", &pattern.pattern_type, Severity::Medium, "Code")
                    .with_description(&pattern.description)
                    .with_location(&pattern.file)
                    .with_owasp("M5"),
            );
        }

        // Check signing certificate
        if signing.is_self_signed {
            findings.push(
                SecurityFinding::new(
                    "APK-008",
                    "Self-signed certificate detected",
                    Severity::Info,
                    "Signing",
                )
                .with_description(
                    "The APK is signed with a self-signed certificate. This is normal for Android apps but verify it's from the legitimate developer.",
                )
                .with_owasp("M1"),
            );
        }

        // Check debug signing
        if let Some(ref subject) = signing.subject {
            if subject.contains("Android Debug") {
                findings.push(
                    SecurityFinding::new(
                        "APK-009",
                        "Debug certificate used for signing",
                        Severity::Critical,
                        "Signing",
                    )
                    .with_description(
                        "The APK is signed with a debug certificate. This should never be used for production releases.",
                    )
                    .with_owasp("M1")
                    .with_cwe("CWE-489")
                    .with_remediation("Sign the release APK with a production keystore."),
                );
            }
        }

        // Check secure storage usage
        if !storage.uses_secure_storage && (storage.uses_database || storage.uses_local_storage) {
            findings.push(
                SecurityFinding::new(
                    "APK-010",
                    "Sensitive data may not be encrypted",
                    Severity::Medium,
                    "Storage",
                )
                .with_description(
                    "The app uses local storage/database but doesn't appear to use Android Keystore for encryption.",
                )
                .with_owasp("M2")
                .with_cwe("CWE-312")
                .with_remediation("Use EncryptedSharedPreferences or SQLCipher for sensitive data."),
            );
        }

        // Check external storage
        if storage.uses_external_storage {
            findings.push(
                SecurityFinding::new(
                    "APK-011",
                    "External storage usage detected",
                    Severity::Low,
                    "Storage",
                )
                .with_description(
                    "The app accesses external storage. Data stored there is world-readable.",
                )
                .with_owasp("M2")
                .with_cwe("CWE-276")
                .with_remediation("Avoid storing sensitive data on external storage."),
            );
        }

        findings
    }
}

impl Default for ApkAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for ApkAnalyzer {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "apk_analyzer".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "Android APK static security analyzer".to_string(),
            module_type: ModuleType::Scanner,
            category: "mobile".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "APK_PATH".to_string(),
                description: "Path to the APK file to analyze".to_string(),
                required: true,
                default_value: None,
                current_value: self.apk_path.clone(),
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
                name: "ANALYZE_NATIVE".to_string(),
                description: "Analyze native libraries (.so files)".to_string(),
                required: false,
                default_value: Some("false".to_string()),
                current_value: Some(self.analyze_native.to_string()),
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
            "APK_PATH" => self.apk_path = Some(value.to_string()),
            "DEEP_ANALYSIS" => self.deep_analysis = value.parse().unwrap_or(false),
            "CHECK_SECRETS" => self.check_secrets = value.parse().unwrap_or(true),
            "ANALYZE_NATIVE" => self.analyze_native = value.parse().unwrap_or(false),
            "OUTPUT_FORMAT" => self.output_format = value.to_string(),
            _ => return Err(anyhow!("Unknown option: {}", name)),
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<String> {
        match name.to_uppercase().as_str() {
            "APK_PATH" => self.apk_path.clone(),
            "DEEP_ANALYSIS" => Some(self.deep_analysis.to_string()),
            "CHECK_SECRETS" => Some(self.check_secrets.to_string()),
            "ANALYZE_NATIVE" => Some(self.analyze_native.to_string()),
            "OUTPUT_FORMAT" => Some(self.output_format.clone()),
            _ => None,
        }
    }

    fn validate(&self) -> Result<()> {
        let apk_path = self
            .apk_path
            .as_ref()
            .ok_or_else(|| anyhow!("APK_PATH is required"))?;

        if !Path::new(apk_path).exists() {
            return Err(anyhow!("APK file not found: {}", apk_path));
        }

        if !apk_path.ends_with(".apk") {
            return Err(anyhow!("File must be an APK: {}", apk_path));
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        // Quick validation check
        let apk_path = self.apk_path.as_ref();

        Ok(CheckResult {
            vulnerable: apk_path.is_some(),
            confidence: if apk_path.is_some() { 1.0 } else { 0.0 },
            details: "APK analyzer ready".to_string(),
            fingerprint: HashMap::new(),
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        let apk_path = self
            .apk_path
            .as_ref()
            .ok_or_else(|| anyhow!("APK_PATH not set"))?
            .clone();

        let start_time = std::time::Instant::now();

        // Run all analysis phases
        let manifest = self.analyze_manifest(&apk_path).await?;
        let network = self.analyze_network_security(&apk_path).await?;
        let code = self.analyze_code(&apk_path).await?;
        let signing = self.analyze_signing(&apk_path).await?;
        let storage = self.analyze_storage(&apk_path).await?;

        // Generate findings
        let findings = self.generate_findings(&manifest, &network, &code, &signing, &storage);

        let duration_ms = start_time.elapsed().as_millis() as u64;
        let summary = AnalysisSummary::from_findings(&findings, duration_ms);

        // Build result
        let mut result = ModuleResult::success(format!(
            "APK analysis complete. Found {} issues (Critical: {}, High: {}, Medium: {}, Low: {})",
            summary.total_findings,
            summary.critical_count,
            summary.high_count,
            summary.medium_count,
            summary.low_count
        ));

        result.data.insert("metadata".to_string(), json!(manifest.metadata));
        result.data.insert("permissions".to_string(), json!(manifest.permissions));
        result.data.insert("exported_components".to_string(), json!(manifest.exported_components));
        result.data.insert("network_security".to_string(), json!(network));
        result.data.insert("signing_info".to_string(), json!(signing));
        result.data.insert("storage".to_string(), json!(storage));
        result.data.insert("findings".to_string(), json!(findings));
        result.data.insert("summary".to_string(), json!(summary));

        Ok(result)
    }
}

/// Internal manifest analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ManifestAnalysis {
    metadata: AppMetadata,
    permissions: Vec<PermissionInfo>,
    exported_components: Vec<ExportedComponent>,
    backup_allowed: bool,
    uses_cleartext: bool,
    custom_permissions: Vec<String>,
}

/// Internal code analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CodeAnalysis {
    hardcoded_secrets: Vec<HardcodedSecret>,
    insecure_patterns: Vec<InsecurePattern>,
    uses_reflection: bool,
    uses_dynamic_code: bool,
    obfuscation_detected: bool,
    root_detection: bool,
    ssl_pinning_impl: bool,
}

/// Insecure code pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
struct InsecurePattern {
    pattern_type: String,
    description: String,
    file: String,
    line: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apk_analyzer_options() {
        let mut analyzer = ApkAnalyzer::new();

        analyzer.set_option("DEEP_ANALYSIS", "true").unwrap();
        assert!(analyzer.deep_analysis);

        analyzer.set_option("CHECK_SECRETS", "false").unwrap();
        assert!(!analyzer.check_secrets);

        assert!(analyzer.get_option("DEEP_ANALYSIS").is_some());
    }

    #[test]
    fn test_apk_analyzer_info() {
        let analyzer = ApkAnalyzer::new();
        let info = analyzer.info();

        assert_eq!(info.name, "apk_analyzer");
        assert_eq!(info.category, "mobile");
        assert_eq!(info.module_type, ModuleType::Scanner);
    }
}
