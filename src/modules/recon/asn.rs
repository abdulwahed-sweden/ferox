use crate::core::module::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct AsnDiscovery {
    options: HashMap<String, String>,
}

impl AsnDiscovery {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("TARGET".to_string(), String::new());
        options.insert("WHOIS_SERVER".to_string(), "whois.cymru.com".to_string());
        options.insert("PORT".to_string(), "43".to_string());
        options.insert("TIMEOUT".to_string(), "10".to_string());
        options.insert("LOOKUP_PEERS".to_string(), "false".to_string());
        options.insert("LOOKUP_PREFIXES".to_string(), "true".to_string());
        Self { options }
    }

    async fn query_whois(&self, query: &str, server: &str) -> Result<String> {
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

        // Send query with newline
        let query_str = format!("{}\r\n", query);
        writer.write_all(query_str.as_bytes()).await?;

        // Read response
        let mut response = String::new();
        let mut line = String::new();
        
        while reader.read_line(&mut line).await? > 0 {
            response.push_str(&line);
            line.clear();
        }

        Ok(response)
    }

    async fn lookup_asn_for_ip(&self, ip: &str) -> Result<HashMap<String, String>> {
        let server = self
            .get_option("WHOIS_SERVER")
            .unwrap_or_else(|| "whois.cymru.com".to_string());

        // Team Cymru format: begin + origin + IP + end
        let query = format!("begin\norigin\n{}\nend", ip);
        let response = self.query_whois(&query, &server).await?;

        let mut result = HashMap::new();
        
        // Parse Team Cymru response format
        // Format: AS | IP | BGP Prefix | CC | Registry | Allocated | AS Name
        for line in response.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("Bulk") || line.starts_with("AS") {
                continue;
            }

            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 5 {
                if let Some(asn) = parts.first() {
                    result.insert("asn".to_string(), asn.to_string());
                }
                if parts.len() > 1 {
                    result.insert("ip".to_string(), parts[1].to_string());
                }
                if parts.len() > 2 {
                    result.insert("bgp_prefix".to_string(), parts[2].to_string());
                }
                if parts.len() > 3 {
                    result.insert("country_code".to_string(), parts[3].to_string());
                }
                if parts.len() > 4 {
                    result.insert("registry".to_string(), parts[4].to_string());
                }
                if parts.len() > 5 {
                    result.insert("allocated_date".to_string(), parts[5].to_string());
                }
                if parts.len() > 6 {
                    result.insert("as_name".to_string(), parts[6].to_string());
                }
                break; // First valid line
            }
        }

        Ok(result)
    }

    async fn lookup_asn_details(&self, asn: &str) -> Result<HashMap<String, String>> {
        let server = self
            .get_option("WHOIS_SERVER")
            .unwrap_or_else(|| "whois.cymru.com".to_string());

        // Query for AS details
        let clean_asn = asn.trim_start_matches("AS").trim();
        let query = format!("begin\nasn\n{}\nend", clean_asn);
        let response = self.query_whois(&query, &server).await?;

        let mut result = HashMap::new();
        
        // Parse response
        for line in response.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("Bulk") || line.starts_with("AS") {
                continue;
            }

            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                if let Some(asn_num) = parts.first() {
                    result.insert("asn".to_string(), format!("AS{}", asn_num));
                }
                if parts.len() > 1 {
                    result.insert("country_code".to_string(), parts[1].to_string());
                }
                if parts.len() > 2 {
                    result.insert("registry".to_string(), parts[2].to_string());
                }
                if parts.len() > 3 {
                    result.insert("allocated_date".to_string(), parts[3].to_string());
                }
                if parts.len() > 4 {
                    result.insert("as_name".to_string(), parts[4].to_string());
                }
                break;
            }
        }

        Ok(result)
    }

    async fn lookup_prefixes(&self, asn: &str) -> Result<Vec<String>> {
        let server = self
            .get_option("WHOIS_SERVER")
            .unwrap_or_else(|| "whois.cymru.com".to_string());

        let clean_asn = asn.trim_start_matches("AS").trim();
        let query = format!("begin\nprefix\n{}\nend", clean_asn);
        let response = self.query_whois(&query, &server).await?;

        let mut prefixes = Vec::new();
        
        for line in response.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("Bulk") || line.starts_with("Prefix") {
                continue;
            }

            // Each line is a prefix
            if line.contains('/') {
                prefixes.push(line.to_string());
            }
        }

        Ok(prefixes)
    }

    async fn resolve_domain_to_ip(&self, domain: &str) -> Result<String> {
        use trust_dns_resolver::TokioAsyncResolver;
        
        let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
        let lookup = resolver.lookup_ip(domain).await?;
        
        lookup
            .iter()
            .next()
            .map(|ip| ip.to_string())
            .ok_or_else(|| anyhow!("No IP addresses found for domain"))
    }
}

#[async_trait]
impl Module for AsnDiscovery {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "asn_discovery".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "ASN/BGP reconnaissance with AS number lookup, prefix enumeration, and organization details".to_string(),
            module_type: ModuleType::Auxiliary,
            category: "recon".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "TARGET".to_string(),
                description: "Target IP, domain, or ASN (e.g., 8.8.8.8, google.com, AS15169)".to_string(),
                required: true,
                default_value: None,
                current_value: self.get_option("TARGET"),
            },
            ModuleOption {
                name: "WHOIS_SERVER".to_string(),
                description: "WHOIS server for ASN lookups".to_string(),
                required: false,
                default_value: Some("whois.cymru.com".to_string()),
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
                name: "LOOKUP_PEERS".to_string(),
                description: "Lookup BGP peers (not yet implemented)".to_string(),
                required: false,
                default_value: Some("false".to_string()),
                current_value: self.get_option("LOOKUP_PEERS"),
            },
            ModuleOption {
                name: "LOOKUP_PREFIXES".to_string(),
                description: "Enumerate BGP prefixes announced by ASN".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.get_option("LOOKUP_PREFIXES"),
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
        let start = Instant::now();

        let mut fp = HashMap::new();
        fp.insert("target".to_string(), target.clone());

        // Determine target type
        let ip = if target.parse::<IpAddr>().is_ok() {
            target.clone()
        } else if target.to_uppercase().starts_with("AS") {
            // It's an ASN
            match self.lookup_asn_details(&target).await {
                Ok(details) => {
                    let elapsed_ms = start.elapsed().as_millis() as u64;
                    fp.insert("response_time_ms".to_string(), elapsed_ms.to_string());
                    
                    return Ok(CheckResult {
                        vulnerable: false,
                        confidence: 1.0,
                        details: format!("ASN {} details available", target),
                        fingerprint: fp,
                    });
                }
                Err(e) => {
                    return Ok(CheckResult {
                        vulnerable: false,
                        confidence: 0.0,
                        details: format!("ASN lookup failed: {}", e),
                        fingerprint: fp,
                    });
                }
            }
        } else {
            // It's a domain, resolve to IP
            match self.resolve_domain_to_ip(&target).await {
                Ok(resolved_ip) => resolved_ip,
                Err(e) => {
                    return Ok(CheckResult {
                        vulnerable: false,
                        confidence: 0.0,
                        details: format!("Domain resolution failed: {}", e),
                        fingerprint: fp,
                    });
                }
            }
        };

        // Lookup ASN for IP
        match self.lookup_asn_for_ip(&ip).await {
            Ok(asn_data) => {
                let elapsed_ms = start.elapsed().as_millis() as u64;
                fp.insert("response_time_ms".to_string(), elapsed_ms.to_string());
                
                let asn = asn_data.get("asn").cloned().unwrap_or_else(|| "Unknown".to_string());
                Ok(CheckResult {
                    vulnerable: false,
                    confidence: 1.0,
                    details: format!("IP {} belongs to ASN {}", ip, asn),
                    fingerprint: fp,
                })
            }
            Err(e) => Ok(CheckResult {
                vulnerable: false,
                confidence: 0.0,
                details: format!("ASN lookup failed: {}", e),
                fingerprint: fp,
            }),
        }
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        self.validate()?;
        let target = self.get_option("TARGET").unwrap_or_default();
        let start = Instant::now();

        let mut result_data = HashMap::new();
        result_data.insert("target".to_string(), serde_json::json!(target));

        // Determine target type and process accordingly
        if target.to_uppercase().starts_with("AS") {
            // Direct ASN lookup
            let asn_details = self.lookup_asn_details(&target).await?;
            result_data.insert("asn_details".to_string(), serde_json::json!(asn_details));

            // Lookup prefixes if enabled
            if self.get_option("LOOKUP_PREFIXES").unwrap_or_default() == "true" {
                match self.lookup_prefixes(&target).await {
                    Ok(prefixes) => {
                        result_data.insert("bgp_prefixes".to_string(), serde_json::json!(prefixes));
                        result_data.insert("prefix_count".to_string(), serde_json::json!(prefixes.len()));
                    }
                    Err(_) => {
                        result_data.insert("bgp_prefixes".to_string(), serde_json::json!(Vec::<String>::new()));
                    }
                }
            }
        } else {
            // IP or domain lookup
            let ip = if target.parse::<IpAddr>().is_ok() {
                target.clone()
            } else {
                let resolved = self.resolve_domain_to_ip(&target).await?;
                result_data.insert("resolved_ip".to_string(), serde_json::json!(resolved));
                resolved
            };

            // Lookup ASN for IP
            let asn_data = self.lookup_asn_for_ip(&ip).await?;
            result_data.insert("asn_info".to_string(), serde_json::json!(asn_data));

            // Get detailed ASN info
            if let Some(asn) = asn_data.get("asn") {
                let asn_details = self.lookup_asn_details(asn).await?;
                result_data.insert("asn_details".to_string(), serde_json::json!(asn_details));

                // Lookup prefixes if enabled
                if self.get_option("LOOKUP_PREFIXES").unwrap_or_default() == "true" {
                    match self.lookup_prefixes(asn).await {
                        Ok(prefixes) => {
                            result_data.insert("bgp_prefixes".to_string(), serde_json::json!(prefixes));
                            result_data.insert("prefix_count".to_string(), serde_json::json!(prefixes.len()));
                        }
                        Err(_) => {
                            result_data.insert("bgp_prefixes".to_string(), serde_json::json!(Vec::<String>::new()));
                        }
                    }
                }
            }
        }

        let elapsed = start.elapsed();
        result_data.insert("scan_time_ms".to_string(), serde_json::json!(elapsed.as_millis()));

        let mut result = ModuleResult::success(format!(
            "🌐 ASN discovery completed for {} in {:.2}s",
            target,
            elapsed.as_secs_f64()
        ));

        for (key, value) in result_data {
            result = result.with_data(key, value);
        }

        Ok(result)
    }
}

impl Default for AsnDiscovery {
    fn default() -> Self {
        Self::new()
    }
}
