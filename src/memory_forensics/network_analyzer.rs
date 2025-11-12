use crate::memory_forensics::dump_parser::DumpParser;
use crate::memory_forensics::types::{NetworkArtifact, NetworkConnection};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref CONNECTION_REGEX: Regex = Regex::new(
        r"(?i)(\d{1,3}(?:\.\d{1,3}){3})(?::(\d{1,5}))?\s*(-?>)\s*(\d{1,3}(?:\.\d{1,3}){3})(?::(\d{1,5}))?(?:\s*\[(TCP|UDP)\])?"
    )
    .expect("valid connection regex");
    static ref IPV4_REGEX: Regex =
        Regex::new(r"\b(\d{1,3}(?:\.\d{1,3}){3})(?::(\d{1,5}))?\b").expect("valid ipv4 regex");
    static ref DOMAIN_REGEX: Regex = Regex::new(
        r"\b([a-z0-9](?:[a-z0-9\-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9\-]{0,61}[a-z0-9])?)+)\b"
    )
    .expect("valid domain regex");
    static ref URL_REGEX: Regex = Regex::new(
        r"\bhttps?://[a-z0-9\-._~:/?#\[\]@!$&'()*+,;=%]+"
    )
    .expect("valid url regex");
}

pub struct NetworkAnalyzer<'a> {
    dump: &'a DumpParser,
}

impl<'a> NetworkAnalyzer<'a> {
    pub fn new(dump: &'a DumpParser) -> Self {
        Self { dump }
    }

    pub fn list_connections(&self) -> Result<Vec<NetworkConnection>> {
        let window = self.dump.text_window(8 * 1024 * 1024);
        let mut seen = HashSet::new();
        let mut connections = Vec::new();

        for cap in CONNECTION_REGEX.captures_iter(&window) {
            let local_addr = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let local_port = cap
                .get(2)
                .and_then(|m| m.as_str().parse::<u16>().ok())
                .unwrap_or(0);
            let remote_addr = cap.get(4).map(|m| m.as_str().to_string());
            let remote_port = cap.get(5).and_then(|m| m.as_str().parse::<u16>().ok());
            let protocol = cap
                .get(6)
                .map(|m| m.as_str().to_uppercase())
                .unwrap_or_else(|| "TCP".to_string());

            let key = format!(
                "{local_addr}:{local_port}->{:?}:{:?}:{protocol}",
                remote_addr, remote_port
            );
            if !seen.insert(key) {
                continue;
            }

            connections.push(NetworkConnection {
                local_addr,
                local_port,
                remote_addr,
                remote_port,
                state: if cap.get(3).map(|m| m.as_str()).unwrap_or("") == "->" {
                    Some("ESTABLISHED".to_string())
                } else {
                    Some("UNKNOWN".to_string())
                },
                pid: None,
                protocol,
            });
        }

        if connections.is_empty() {
            // fallback: treat standalone IPs as listeners
            for cap in IPV4_REGEX.captures_iter(&window) {
                let addr = cap
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                let port = cap
                    .get(2)
                    .and_then(|m| m.as_str().parse::<u16>().ok())
                    .unwrap_or(0);
                let key = format!("{addr}:{port}");
                if !seen.insert(key.clone()) {
                    continue;
                }

                connections.push(NetworkConnection {
                    local_addr: addr,
                    local_port: port,
                    remote_addr: None,
                    remote_port: None,
                    state: Some("LISTENING".to_string()),
                    pid: None,
                    protocol: "TCP".to_string(),
                });
            }
        }

        Ok(connections)
    }

    pub fn list_listeners(&self) -> Result<Vec<NetworkConnection>> {
        let connections = self.list_connections()?;
        Ok(connections
            .into_iter()
            .filter(|conn| conn.remote_addr.is_none() || conn.state.as_deref() == Some("LISTENING"))
            .collect())
    }

    pub fn extract_dns_cache(&self) -> Result<Vec<NetworkArtifact>> {
        let window = self.dump.text_window(8 * 1024 * 1024);
        let mut seen = HashSet::new();
        let mut artifacts = Vec::new();

        for cap in DOMAIN_REGEX.captures_iter(&window) {
            let value = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            if value.len() <= 4 || !value.contains('.') {
                continue;
            }
            if !seen.insert(value.clone()) {
                continue;
            }
            artifacts.push(NetworkArtifact {
                artifact_type: "dns".to_string(),
                value,
                pid: None,
            });
        }

        Ok(artifacts)
    }

    pub fn extract_urls(&self) -> Result<Vec<NetworkArtifact>> {
        let window = self.dump.text_window(8 * 1024 * 1024);
        let mut seen = HashSet::new();
        let mut artifacts = Vec::new();

        for cap in URL_REGEX.captures_iter(&window) {
            let value = cap
                .get(0)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            if !seen.insert(value.clone()) {
                continue;
            }
            artifacts.push(NetworkArtifact {
                artifact_type: "url".to_string(),
                value,
                pid: None,
            });
        }

        Ok(artifacts)
    }
}
