use crate::core::module::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub struct PortScanner {
    options: HashMap<String, String>,
}

impl PortScanner {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("RHOSTS".to_string(), String::new());
        options.insert("PORTS".to_string(), "1-1000".to_string());
        options.insert("TIMEOUT".to_string(), "1000".to_string());
        options.insert("THREADS".to_string(), "100".to_string());

        Self { options }
    }

    fn parse_ports(&self) -> Result<Vec<u16>> {
        let ports_str = self.get_option("PORTS").unwrap_or_else(|| "1-1000".to_string());
        let mut ports = Vec::new();

        for part in ports_str.split(',') {
            let part = part.trim();
            if part.contains('-') {
                let range: Vec<&str> = part.split('-').collect();
                if range.len() == 2 {
                    let start: u16 = range[0].parse()?;
                    let end: u16 = range[1].parse()?;
                    ports.extend(start..=end);
                }
            } else {
                ports.push(part.parse()?);
            }
        }

        Ok(ports)
    }

    async fn scan_port(&self, host: &str, port: u16) -> Option<u16> {
        let timeout_ms = self
            .get_option("TIMEOUT")
            .and_then(|t| t.parse::<u64>().ok())
            .unwrap_or(1000);

        let addr = format!("{}:{}", host, port);
        
        match timeout(
            Duration::from_millis(timeout_ms),
            TcpStream::connect(&addr),
        )
        .await
        {
            Ok(Ok(_)) => Some(port),
            _ => None,
        }
    }
}

#[async_trait]
impl Module for PortScanner {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "port_scanner".to_string(),
            version: "2.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "High-performance async TCP port scanner with concurrent connections".to_string(),
            module_type: ModuleType::Scanner,
            category: "scanner".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "RHOSTS".to_string(),
                description: "Target host or IP address".to_string(),
                required: true,
                default_value: None,
                current_value: self.get_option("RHOSTS"),
            },
            ModuleOption {
                name: "PORTS".to_string(),
                description: "Ports to scan (e.g., 80,443 or 1-1000)".to_string(),
                required: false,
                default_value: Some("1-1000".to_string()),
                current_value: self.get_option("PORTS"),
            },
            ModuleOption {
                name: "TIMEOUT".to_string(),
                description: "Connection timeout in milliseconds".to_string(),
                required: false,
                default_value: Some("1000".to_string()),
                current_value: self.get_option("TIMEOUT"),
            },
            ModuleOption {
                name: "THREADS".to_string(),
                description: "Number of concurrent connections".to_string(),
                required: false,
                default_value: Some("100".to_string()),
                current_value: self.get_option("THREADS"),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        if self.options.contains_key(name) {
            self.options.insert(name.to_string(), value.to_string());
            Ok(())
        } else {
            Err(anyhow!("Unknown option: {}", name))
        }
    }

    fn get_option(&self, name: &str) -> Option<String> {
        self.options.get(name).cloned()
    }

    fn validate(&self) -> Result<()> {
        if self.get_option("RHOSTS").unwrap_or_default().is_empty() {
            return Err(anyhow!("RHOSTS is required"));
        }
        Ok(())
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        self.validate()?;

        let host = self.get_option("RHOSTS").unwrap();
        let ports = self.parse_ports()?;
        let max_concurrent = self
            .get_option("THREADS")
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(100);

        let mut open_ports = Vec::new();
        let total = ports.len();

        // Scan ports in chunks for better performance
        for chunk in ports.chunks(max_concurrent) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|&port| self.scan_port(&host, port))
                .collect();

            let results = futures::future::join_all(futures).await;

            for result in results {
                if let Some(port) = result {
                    open_ports.push(port);
                }
            }
        }

        // Sort results
        open_ports.sort_unstable();

        let mut data = HashMap::new();
        data.insert("host".to_string(), serde_json::json!(host));
        data.insert("open_ports".to_string(), serde_json::json!(open_ports));
        data.insert("total_scanned".to_string(), serde_json::json!(total));
        data.insert("open_count".to_string(), serde_json::json!(open_ports.len()));

        Ok(ModuleResult {
            success: true,
            message: format!(
                "🎯 Found {} open ports out of {} scanned on {}",
                open_ports.len(),
                total,
                host
            ),
            data,
            timestamp: chrono::Utc::now(),
            session_id: None,
        })
    }
}

impl Default for PortScanner {
    fn default() -> Self {
        Self::new()
    }
}
