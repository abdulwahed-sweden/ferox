use crate::core::module::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct WhoisLookup {
    options: HashMap<String, String>,
}

impl WhoisLookup {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("TARGET".to_string(), String::new());
        options.insert("WHOIS_SERVER".to_string(), String::new()); // Auto-detect if empty
        options.insert("PORT".to_string(), "43".to_string());
        options.insert("TIMEOUT".to_string(), "10".to_string());
        options.insert("FOLLOW_REFERRAL".to_string(), "true".to_string());
        Self { options }
    }

    fn get_whois_server_for_tld(&self, target: &str) -> String {
        // Determine WHOIS server based on TLD or if it's an IP
        if target.parse::<std::net::IpAddr>().is_ok() {
            return "whois.arin.net".to_string(); // ARIN for IPs
        }

        // Extract TLD
        let parts: Vec<&str> = target.split('.').collect();
        if parts.len() < 2 {
            return "whois.internic.net".to_string();
        }

        let tld = parts.last().unwrap();
        match *tld {
            "com" | "net" | "edu" => "whois.verisign-grs.com",
            "org" => "whois.pir.org",
            "io" => "whois.nic.io",
            "uk" => "whois.nic.uk",
            "de" => "whois.denic.de",
            "fr" => "whois.nic.fr",
            "ca" => "whois.cira.ca",
            "au" => "whois.auda.org.au",
            "jp" => "whois.jprs.jp",
            "cn" => "whois.cnnic.cn",
            "ru" => "whois.tcinet.ru",
            "br" => "whois.registro.br",
            "in" => "whois.registry.in",
            "info" => "whois.afilias.net",
            "biz" => "whois.neulevel.biz",
            "us" => "whois.nic.us",
            "co" => "whois.nic.co",
            _ => "whois.internic.net", // Default fallback
        }
        .to_string()
    }

    async fn query_whois(&self, target: &str, server: &str) -> Result<String> {
        let port = self
            .get_option("PORT")
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(43);

        let timeout = self
            .get_option("TIMEOUT")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(10);

        let addr = format!("{}:{}", server, port);
        
        let stream = tokio::time::timeout(
            std::time::Duration::from_secs(timeout),
            TcpStream::connect(&addr),
        )
        .await??;

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // Send query
        let query = format!("{}\r\n", target);
        writer.write_all(query.as_bytes()).await?;

        // Read response
        let mut response = String::new();
        let mut line = String::new();
        
        while reader.read_line(&mut line).await? > 0 {
            response.push_str(&line);
            line.clear();
        }

        Ok(response)
    }

    fn parse_whois_response(&self, raw: &str) -> HashMap<String, String> {
        let mut parsed = HashMap::new();
        
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('%') || line.starts_with('#') {
                continue;
            }

            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_lowercase().replace(' ', "_");
                let value = line[pos + 1..].trim().to_string();
                
                if !value.is_empty() {
                    // Collect interesting fields
                    if key.contains("domain")
                        || key.contains("registrar")
                        || key.contains("registrant")
                        || key.contains("admin")
                        || key.contains("tech")
                        || key.contains("name_server")
                        || key.contains("created")
                        || key.contains("updated")
                        || key.contains("expir")
                        || key.contains("status")
                        || key.contains("organization")
                        || key.contains("email")
                        || key.contains("phone")
                        || key.contains("address")
                        || key.contains("country")
                        || key.contains("netname")
                        || key.contains("inetnum")
                        || key.contains("cidr")
                    {
                        parsed.insert(key, value);
                    }
                }
            }
        }

        parsed
    }

    fn extract_referral_server(&self, response: &str) -> Option<String> {
        // Look for referral WHOIS server in response
        for line in response.lines() {
            let lower = line.to_lowercase();
            if lower.contains("whois server:") || lower.contains("referral url:") {
                if let Some(server) = line.split(':').nth(1) {
                    let server = server.trim().replace("http://", "").replace("https://", "");
                    if server.contains("whois") {
                        return Some(server);
                    }
                }
            }
        }
        None
    }
}

#[async_trait]
impl Module for WhoisLookup {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "whois_lookup".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "WHOIS domain and IP lookup with registrar info, nameservers, dates, and contact details".to_string(),
            module_type: ModuleType::Auxiliary,
            category: "recon".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "TARGET".to_string(),
                description: "Target domain or IP address".to_string(),
                required: true,
                default_value: None,
                current_value: self.get_option("TARGET"),
            },
            ModuleOption {
                name: "WHOIS_SERVER".to_string(),
                description: "Custom WHOIS server (empty = auto-detect from TLD)".to_string(),
                required: false,
                default_value: Some(String::new()),
                current_value: self.get_option("WHOIS_SERVER"),
            },
            ModuleOption {
                name: "PORT".to_string(),
                description: "WHOIS server port".to_string(),
                required: false,
                default_value: Some("43".to_string()),
                current_value: self.get_option("PORT"),
            },
            ModuleOption {
                name: "TIMEOUT".to_string(),
                description: "Connection timeout in seconds".to_string(),
                required: false,
                default_value: Some("10".to_string()),
                current_value: self.get_option("TIMEOUT"),
            },
            ModuleOption {
                name: "FOLLOW_REFERRAL".to_string(),
                description: "Follow referral to authoritative WHOIS server".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.get_option("FOLLOW_REFERRAL"),
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
        if self.get_option("TARGET").unwrap_or_default().is_empty() {
            return Err(anyhow!("TARGET is required"));
        }
        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        let target = self.get_option("TARGET").unwrap_or_default();
        let server = if let Some(custom) = self.get_option("WHOIS_SERVER").filter(|s| !s.is_empty()) {
            custom
        } else {
            self.get_whois_server_for_tld(&target)
        };

        let mut fp = HashMap::new();
        fp.insert("target".to_string(), target.clone());
        fp.insert("whois_server".to_string(), server.clone());

        let start = Instant::now();
        match self.query_whois(&target, &server).await {
            Ok(response) => {
                let elapsed_ms = start.elapsed().as_millis() as u64;
                fp.insert("response_time_ms".to_string(), elapsed_ms.to_string());
                fp.insert("response_length".to_string(), response.len().to_string());

                let has_data = !response.is_empty() && !response.to_lowercase().contains("no match");
                
                Ok(CheckResult {
                    vulnerable: false,
                    confidence: if has_data { 1.0 } else { 0.0 },
                    details: if has_data {
                        format!("WHOIS data available for {} ({} bytes)", target, response.len())
                    } else {
                        format!("No WHOIS data found for {}", target)
                    },
                    fingerprint: fp,
                })
            }
            Err(e) => Ok(CheckResult {
                vulnerable: false,
                confidence: 0.0,
                details: format!("WHOIS query failed: {}", e),
                fingerprint: fp,
            }),
        }
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        self.validate()?;
        let target = self.get_option("TARGET").unwrap_or_default();
        let start = Instant::now();

        let server = if let Some(custom) = self.get_option("WHOIS_SERVER").filter(|s| !s.is_empty()) {
            custom
        } else {
            self.get_whois_server_for_tld(&target)
        };

        // Initial query
        let raw_response = self.query_whois(&target, &server).await?;
        let parsed = self.parse_whois_response(&raw_response);

        let mut final_parsed = parsed.clone();
        let mut servers_queried = vec![server.clone()];

        // Follow referral if enabled
        if self.get_option("FOLLOW_REFERRAL").unwrap_or_default() == "true" {
            if let Some(referral) = self.extract_referral_server(&raw_response) {
                if referral != server {
                    match self.query_whois(&target, &referral).await {
                        Ok(ref_response) => {
                            let ref_parsed = self.parse_whois_response(&ref_response);
                            // Merge results, preferring referral data
                            for (k, v) in ref_parsed {
                                final_parsed.insert(k, v);
                            }
                            servers_queried.push(referral);
                        }
                        Err(_) => {
                            // Referral failed, continue with initial results
                        }
                    }
                }
            }
        }

        let elapsed = start.elapsed();

        let mut result = ModuleResult::success(format!(
            "📋 WHOIS lookup completed for {} in {:.2}s",
            target,
            elapsed.as_secs_f64()
        ));

        result = result
            .with_data("target", serde_json::json!(target))
            .with_data("whois_servers", serde_json::json!(servers_queried))
            .with_data("whois_data", serde_json::json!(final_parsed))
            .with_data("raw_response", serde_json::json!(raw_response))
            .with_data("scan_time_ms", serde_json::json!(elapsed.as_millis()));

        Ok(result)
    }
}

impl Default for WhoisLookup {
    fn default() -> Self {
        Self::new()
    }
}
