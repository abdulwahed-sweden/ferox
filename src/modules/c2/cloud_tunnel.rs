//! Cloud Tunnel scaffold (provider-agnostic)
//!
//! Provides a trait for provider backends (e.g., teams, github, doh) and a mock
//! implementation for tests. Real providers will implement data exfiltration
//! or command transport through legitimate-looking traffic.
//!
//! TODO:
//! - Implement concrete providers with rate limiting and jitter
//! - Add structured metrics & tracing spans

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CloudTunnelProvider: Send + Sync {
    async fn push(&self, channel: &str, data: &[u8]) -> Result<()>;
    async fn pull(&self, channel: &str) -> Result<Option<Vec<u8>>>;
}

#[derive(Default)]
pub struct MockCloudProvider {
    // For simplicity, store last pushed per channel (not a queue)
    store: tokio::sync::Mutex<std::collections::HashMap<String, Vec<u8>>>,
}

#[async_trait]
impl CloudTunnelProvider for MockCloudProvider {
    async fn push(&self, channel: &str, data: &[u8]) -> Result<()> {
        let mut g = self.store.lock().await;
        g.insert(channel.to_string(), data.to_vec());
        Ok(())
    }
    async fn pull(&self, channel: &str) -> Result<Option<Vec<u8>>> {
        let g = self.store.lock().await;
        Ok(g.get(channel).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_round_trip() {
        let p = MockCloudProvider::default();
        p.push("ch1", b"data").await.unwrap();
        let v = p.pull("ch1").await.unwrap();
        assert_eq!(v.unwrap(), b"data".to_vec());
    }
}
