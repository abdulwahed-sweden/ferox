use crate::core::module::*;
use crate::core::module_options::{OptionManager, OptionParser, StandardOptions};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub struct PortScanner {
    standard_opts: StandardOptions,
    ports: String,
}

impl PortScanner {
    pub fn new() -> Self {
        Self {
            standard_opts: StandardOptions::default(),
            ports: "1-1000".to_string(),
        }
    }

    fn parse_ports(&self) -> Result<Vec<u16>> {
        let mut ports = Vec::new();

        for part in self.ports.split(',') {
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
        let addr = format!("{}:{}", host, port);

        match timeout(
            Duration::from_millis(self.standard_opts.timeout_ms),
            TcpStream::connect(&addr),
        )
        .await
        {
            Ok(Ok(_)) => Some(port),
            _ => None,
        }
    }
}

impl OptionManager for PortScanner {
    fn validate(&self) -> Result<()> {
        self.standard_opts.validate_required(true)?;
        self.parse_ports()?; // Validate port format
        Ok(())
    }

    fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "RHOSTS" => {
                self.standard_opts.rhosts = Some(value.to_string());
                Ok(())
            }
            "RHOST" => {
                self.standard_opts.rhost = Some(value.to_string());
                Ok(())
            }
            "PORTS" => {
                self.ports = value.to_string();
                Ok(())
            }
            "TIMEOUT" => {
                self.standard_opts.timeout_ms = OptionParser::parse_timeout(value)?;
                Ok(())
            }
            "THREADS" => {
                self.standard_opts.threads = OptionParser::parse_threads(value)?;
                Ok(())
            }
            _ => Err(anyhow!("Unknown option: {}", key)),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        match key {
            "RHOSTS" => self.standard_opts.rhosts.clone(),
            "RHOST" => self.standard_opts.rhost.clone(),
            "PORTS" => Some(self.ports.clone()),
            "TIMEOUT" => Some(self.standard_opts.timeout_ms.to_string()),
            "THREADS" => Some(self.standard_opts.threads.to_string()),
            _ => None,
        }
    }

    fn list(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(
            "RHOSTS".to_string(),
            self.standard_opts.rhosts.clone().unwrap_or_default(),
        );
        map.insert("PORTS".to_string(), self.ports.clone());
        map.insert(
            "TIMEOUT".to_string(),
            self.standard_opts.timeout_ms.to_string(),
        );
        map.insert(
            "THREADS".to_string(),
            self.standard_opts.threads.to_string(),
        );
        map
    }
}

#[async_trait]
impl Module for PortScanner {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "port_scanner".to_string(),
            version: "2.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "High-performance async TCP port scanner with concurrent connections"
                .to_string(),
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
                current_value: self.get("RHOSTS"),
            },
            ModuleOption {
                name: "PORTS".to_string(),
                description: "Ports to scan (e.g., 80,443 or 1-1000)".to_string(),
                required: false,
                default_value: Some("1-1000".to_string()),
                current_value: self.get("PORTS"),
            },
            ModuleOption {
                name: "TIMEOUT".to_string(),
                description: "Connection timeout in milliseconds".to_string(),
                required: false,
                default_value: Some("5000".to_string()),
                current_value: self.get("TIMEOUT"),
            },
            ModuleOption {
                name: "THREADS".to_string(),
                description: "Number of concurrent connections".to_string(),
                required: false,
                default_value: Some("10".to_string()),
                current_value: self.get("THREADS"),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        self.set(name, value)
    }

    fn get_option(&self, name: &str) -> Option<String> {
        self.get(name)
    }

    fn validate(&self) -> Result<()> {
        OptionManager::validate(self)
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        Module::validate(self)?;

        let host = self
            .standard_opts
            .get_target()
            .ok_or_else(|| anyhow!("No target host specified"))?;
        let ports = self.parse_ports()?;
        let max_concurrent = self.standard_opts.threads;

        let mut open_ports = Vec::new();
        let total = ports.len();

        // Scan ports in chunks for better performance
        for chunk in ports.chunks(max_concurrent) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|&port| self.scan_port(&host, port))
                .collect();

            let results = futures::future::join_all(futures).await;

            for port in results.into_iter().flatten() {
                open_ports.push(port);
            }
        }

        // Sort results
        open_ports.sort_unstable();

        let mut result = ModuleResult::success(format!(
            "🎯 Found {} open ports out of {} scanned on {}",
            open_ports.len(),
            total,
            host
        ));

        result = result
            .with_data("host", serde_json::json!(host))
            .with_data("open_ports", serde_json::json!(open_ports))
            .with_data("total_scanned", serde_json::json!(total))
            .with_data("open_count", serde_json::json!(open_ports.len()));

        Ok(result)
    }
}

impl Default for PortScanner {
    fn default() -> Self {
        Self::new()
    }
}
