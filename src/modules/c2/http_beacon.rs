//! HTTP Beacon scaffold (Phase 3)
//!
//! Conservative stub that models a beacon loop with auth (token + HMAC) and
//! encrypted payloads. Network I/O is intentionally abstracted behind a simple
//! in-memory server trait for tests to remain fast and deterministic.
//!
//! TODO:
//! - Replace InMemoryBeaconServer with real HTTP endpoints using reqwest
//! - Add TLS pinning / stronger verification options
//! - Persist nonces and rotate keys periodically

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::infra::crypto::{self, aes_decrypt, aes_encrypt, derive_keys, hmac_sign, hmac_verify, AES_KEY_LEN, HMAC_KEY_LEN};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconConfig {
    pub poll_interval_ms: u64,
    pub auth_token_env: String, // env var name to read token
    pub tls_verify: bool,
}

impl Default for BeaconConfig {
    fn default() -> Self {
        Self { poll_interval_ms: 500, auth_token_env: "FEROX_C2_TOKEN".into(), tls_verify: true }
    }
}

/// Minimal beacon client holding derived keys and config
pub struct BeaconClient {
    cfg: BeaconConfig,
    enc_key: [u8; AES_KEY_LEN],
    mac_key: [u8; HMAC_KEY_LEN],
}

impl BeaconClient {
    pub fn new(cfg: BeaconConfig, token: &str) -> Result<Self> {
        let keys = derive_keys(token.as_bytes(), b"ferox-c2-salt")?;
        Ok(Self { cfg, enc_key: keys.enc_key, mac_key: keys.hmac_key })
    }

    /// Perform one beacon tick: fetch command, produce result, send back.
    /// Exposed publicly for integration tests; still subject to future auth expansion.
    pub async fn tick<S: BeaconServer>(&self, server: &S) -> Result<()> {
        // Request a command (authenticated request simulated)
        let req = AuthEnvelope::new(self, b"cmd?")?;
        let resp = server.fetch_command(req).await?;
        let cmd_bytes = resp.open(self)?;
        // Simulate execution: echo the command uppercased
        let result = String::from_utf8(cmd_bytes).unwrap_or_default().to_uppercase();
        let env = AuthEnvelope::new(self, result.as_bytes())?;
        server.submit_result(env).await?;
        Ok(())
    }

    /// Start background beacon loop talking to provided server.
    pub fn start_background<S: BeaconServer + Send + Sync + 'static>(self, server: S) -> (JoinHandle<()>, oneshot::Sender<()>) {
        let (tx_stop, mut rx_stop) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(self.cfg.poll_interval_ms));
            loop {
                tokio::select! {
                    _ = &mut rx_stop => { break; }
                    _ = interval.tick() => {
                        let _ = self.tick(&server).await; // swallow errors in stub
                    }
                }
            }
        });
        (handle, tx_stop)
    }
}

/// Authenticated, encrypted envelope used for requests and responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthEnvelope {
    pub nonce: [u8; crypto::NONCE_LEN],
    pub ciphertext: Vec<u8>,
    pub aad: Vec<u8>, // includes timestamp or role
    pub tag: Vec<u8>, // HMAC-SHA256 over (nonce || aad || ciphertext)
}

impl AuthEnvelope {
    pub fn new(client: &BeaconClient, plaintext: &[u8]) -> Result<Self> {
        let aad = b"ferox-http-beacon".to_vec();
        let (nonce, ct) = aes_encrypt(&client.enc_key, plaintext, &aad)?;
        let mut mac_input = Vec::with_capacity(nonce.len() + aad.len() + ct.len());
        mac_input.extend_from_slice(&nonce);
        mac_input.extend_from_slice(&aad);
        mac_input.extend_from_slice(&ct);
        let tag = hmac_sign(&client.mac_key, &mac_input);
        Ok(Self { nonce, ciphertext: ct, aad, tag })
    }

    pub fn open(self, client: &BeaconClient) -> Result<Vec<u8>> {
        let mut mac_input = Vec::with_capacity(self.nonce.len() + self.aad.len() + self.ciphertext.len());
        mac_input.extend_from_slice(&self.nonce);
        mac_input.extend_from_slice(&self.aad);
        mac_input.extend_from_slice(&self.ciphertext);
        if !hmac_verify(&client.mac_key, &mac_input, &self.tag) { return Err(anyhow!("HMAC verify failed")); }
        let pt = aes_decrypt(&client.enc_key, &self.nonce, &self.ciphertext, &self.aad)?;
        Ok(pt)
    }
}

/// Abstract server API for unit testing without network
#[async_trait::async_trait]
pub trait BeaconServer {
    async fn fetch_command(&self, auth_req: AuthEnvelope) -> Result<AuthEnvelope>;
    async fn submit_result(&self, auth_resp: AuthEnvelope) -> Result<()>;
}

/// Simple in-memory server using channels for commands and results
pub struct InMemoryBeaconServer {
    cmd_rx: tokio::sync::Mutex<mpsc::Receiver<String>>,
    res_tx: mpsc::Sender<String>,
    // NOTE: In a real server, we'd authenticate the request envelope contents too.
    client: BeaconClient,
}

impl InMemoryBeaconServer {
    pub fn new(client: BeaconClient) -> (Self, mpsc::Sender<String>, mpsc::Receiver<String>) {
        let (cmd_tx, cmd_rx) = mpsc::channel(8);
        let (res_tx, res_rx) = mpsc::channel(8);
        (Self { cmd_rx: tokio::sync::Mutex::new(cmd_rx), res_tx, client }, cmd_tx, res_rx)
    }
}

#[async_trait::async_trait]
impl BeaconServer for InMemoryBeaconServer {
    async fn fetch_command(&self, auth_req: AuthEnvelope) -> Result<AuthEnvelope> {
        // Verify request authenticity
        let _ = auth_req.clone().open(&self.client)?;
        // Provide next command if available; else default "nop"
        let mut rx = self.cmd_rx.lock().await;
        let cmd = rx.try_recv().unwrap_or_else(|_| "nop".to_string());
        let env = AuthEnvelope::new(&self.client, cmd.as_bytes())?;
        Ok(env)
    }

    async fn submit_result(&self, auth_resp: AuthEnvelope) -> Result<()> {
        let pt = auth_resp.open(&self.client)?;
        let s = String::from_utf8(pt).unwrap_or_default();
        let _ = self.res_tx.send(s).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn beacon_loop_simulation() {
        let cfg = BeaconConfig::default();
        let client = BeaconClient::new(cfg, "test_token").unwrap();
        let (server, cmd_tx, mut res_rx) = InMemoryBeaconServer::new(BeaconClient::new(BeaconConfig::default(), "test_token").unwrap());

        // Start a single tick manually (no background loop to keep test deterministic)
        cmd_tx.send("whoami".into()).await.unwrap();
        client.tick(&server).await.unwrap();
        let msg = res_rx.try_recv().unwrap();
        assert_eq!(msg, "WHOAMI");
    }
}
