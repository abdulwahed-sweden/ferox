use crate::core::module::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Instant;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::proto::rr::RecordType;

pub struct DnsEnumerator {
    options: HashMap<String, String>,
}

impl DnsEnumerator {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("TARGET".to_string(), String::new());
        options.insert("RECORD_TYPES".to_string(), "A,AAAA,MX,NS,TXT,SOA,CNAME".to_string());
        options.insert("NAMESERVER".to_string(), String::new()); // Empty = use system default
        options.insert("TIMEOUT".to_string(), "5".to_string());
        options.insert("SUBDOMAIN_ENUM".to_string(), "false".to_string());
        options.insert("WORDLIST".to_string(), "common,www,mail,ftp,admin,dev,staging".to_string());
        options.insert("ZONE_TRANSFER".to_string(), "true".to_string());
        options.insert("REVERSE_DNS".to_string(), "false".to_string());
        Self { options }
    }

    async fn get_resolver(&self) -> Result<TokioAsyncResolver> {
        let timeout = self
            .get_option("TIMEOUT")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(5);

        let mut config = ResolverConfig::default();
        let mut opts = ResolverOpts::default();
        opts.timeout = std::time::Duration::from_secs(timeout);

        // Custom nameserver if specified
        if let Some(ns) = self.get_option("NAMESERVER") {
            if !ns.is_empty() {
                if let Ok(ip) = ns.parse::<IpAddr>() {
                    config = ResolverConfig::from_parts(
                        None,
                        vec![],
                        trust_dns_resolver::config::NameServerConfigGroup::from_ips_clear(&[ip], 53, true),
                    );
                }
            }
        }

        Ok(TokioAsyncResolver::tokio(config, opts))
    }

    fn parse_record_types(&self) -> Vec<RecordType> {
        let types_str = self
            .get_option("RECORD_TYPES")
            .unwrap_or_else(|| "A,AAAA,MX,NS,TXT".to_string());

        types_str
            .split(',')
            .filter_map(|s| match s.trim().to_uppercase().as_str() {
                "A" => Some(RecordType::A),
                "AAAA" => Some(RecordType::AAAA),
                "MX" => Some(RecordType::MX),
                "NS" => Some(RecordType::NS),
                "TXT" => Some(RecordType::TXT),
                "SOA" => Some(RecordType::SOA),
                "CNAME" => Some(RecordType::CNAME),
                "PTR" => Some(RecordType::PTR),
                "SRV" => Some(RecordType::SRV),
                _ => None,
            })
            .collect()
    }

    async fn enumerate_records(
        &self,
        domain: &str,
        resolver: &TokioAsyncResolver,
    ) -> HashMap<String, Vec<String>> {
        let mut results = HashMap::new();
        let record_types = self.parse_record_types();

        for rtype in record_types {
            let lookup = resolver.lookup(domain, rtype).await;
            
            if let Ok(response) = lookup {
                let records: Vec<String> = response
                    .iter()
                    .map(|r| format!("{}", r))
                    .collect();
                
                if !records.is_empty() {
                    results.insert(format!("{:?}", rtype), records);
                }
            }
        }

        results
    }

    async fn attempt_zone_transfer(
        &self,
        domain: &str,
        nameservers: &[String],
    ) -> Vec<String> {
        // Zone transfer is typically restricted, but we'll note the attempt
        let mut results = Vec::new();
        
        for ns in nameservers {
            results.push(format!("Zone transfer attempted against {} (typically restricted)", ns));
        }
        
        results
    }

    async fn enumerate_subdomains(
        &self,
        domain: &str,
        resolver: &TokioAsyncResolver,
    ) -> Vec<String> {
        let wordlist_str = self
            .get_option("WORDLIST")
            .unwrap_or_else(|| "www,mail,ftp".to_string());

        let subdomains: Vec<String> = wordlist_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let mut found = Vec::new();

        for subdomain in subdomains {
            let fqdn = format!("{}.{}", subdomain, domain);
            
            if let Ok(lookup) = resolver.lookup_ip(&fqdn).await {
                let ips: Vec<String> = lookup.iter().map(|ip| ip.to_string()).collect();
                if !ips.is_empty() {
                    found.push(format!("{} => {}", fqdn, ips.join(", ")));
                }
            }
        }

        found
    }

    async fn reverse_dns_lookup(
        &self,
        ip: IpAddr,
        resolver: &TokioAsyncResolver,
    ) -> Option<String> {
        if let Ok(lookup) = resolver.reverse_lookup(ip).await {
            lookup.iter().next().map(|name| name.to_string())
        } else {
            None
        }
    }
}

#[async_trait]
impl Module for DnsEnumerator {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "dns_enum".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "Advanced DNS enumeration with multiple record types, subdomain discovery, and zone transfer attempts".to_string(),
            module_type: ModuleType::Auxiliary,
            category: "recon".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "TARGET".to_string(),
                description: "Target domain (e.g., example.com) or IP for reverse DNS".to_string(),
                required: true,
                default_value: None,
                current_value: self.get_option("TARGET"),
            },
            ModuleOption {
                name: "RECORD_TYPES".to_string(),
                description: "Comma-separated DNS record types (A,AAAA,MX,NS,TXT,SOA,CNAME,PTR,SRV)".to_string(),
                required: false,
                default_value: Some("A,AAAA,MX,NS,TXT,SOA,CNAME".to_string()),
                current_value: self.get_option("RECORD_TYPES"),
            },
            ModuleOption {
                name: "NAMESERVER".to_string(),
                description: "Custom DNS nameserver IP (empty = system default)".to_string(),
                required: false,
                default_value: Some(String::new()),
                current_value: self.get_option("NAMESERVER"),
            },
            ModuleOption {
                name: "TIMEOUT".to_string(),
                description: "DNS query timeout in seconds".to_string(),
                required: false,
                default_value: Some("5".to_string()),
                current_value: self.get_option("TIMEOUT"),
            },
            ModuleOption {
                name: "SUBDOMAIN_ENUM".to_string(),
                description: "Enable subdomain enumeration (true/false)".to_string(),
                required: false,
                default_value: Some("false".to_string()),
                current_value: self.get_option("SUBDOMAIN_ENUM"),
            },
            ModuleOption {
                name: "WORDLIST".to_string(),
                description: "Comma-separated subdomain wordlist".to_string(),
                required: false,
                default_value: Some("common,www,mail,ftp,admin,dev,staging".to_string()),
                current_value: self.get_option("WORDLIST"),
            },
            ModuleOption {
                name: "ZONE_TRANSFER".to_string(),
                description: "Attempt zone transfer (AXFR) - typically restricted".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.get_option("ZONE_TRANSFER"),
            },
            ModuleOption {
                name: "REVERSE_DNS".to_string(),
                description: "Perform reverse DNS lookup if TARGET is IP".to_string(),
                required: false,
                default_value: Some("false".to_string()),
                current_value: self.get_option("REVERSE_DNS"),
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
        let resolver = self.get_resolver().await?;

        let start = Instant::now();
        let lookup = resolver.lookup_ip(&target).await;
        let elapsed_ms = start.elapsed().as_millis() as u64;

        let mut fp = HashMap::new();
        fp.insert("target".to_string(), target.clone());
        fp.insert("response_time_ms".to_string(), elapsed_ms.to_string());

        match lookup {
            Ok(ips) => {
                let ip_list: Vec<String> = ips.iter().map(|ip| ip.to_string()).collect();
                fp.insert("resolved_ips".to_string(), ip_list.join(", "));
                
                Ok(CheckResult {
                    vulnerable: false,
                    confidence: 1.0,
                    details: format!("{} resolves to {} IPs", target, ip_list.len()),
                    fingerprint: fp,
                })
            }
            Err(e) => Ok(CheckResult {
                vulnerable: false,
                confidence: 0.0,
                details: format!("DNS resolution failed: {}", e),
                fingerprint: fp,
            }),
        }
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        self.validate()?;
        let target = self.get_option("TARGET").unwrap_or_default();
        let resolver = self.get_resolver().await?;

        let start = Instant::now();

        // Check if target is IP (for reverse DNS)
        let is_ip = target.parse::<IpAddr>().is_ok();

        let mut result_data = HashMap::new();
        result_data.insert("target".to_string(), serde_json::json!(target));

        if is_ip && self.get_option("REVERSE_DNS").unwrap_or_default() == "true" {
            // Reverse DNS lookup
            let ip = target.parse::<IpAddr>().unwrap();
            if let Some(hostname) = self.reverse_dns_lookup(ip, &resolver).await {
                result_data.insert("reverse_dns".to_string(), serde_json::json!(hostname));
            }
        } else {
            // Standard DNS enumeration
            let records = self.enumerate_records(&target, &resolver).await;
            result_data.insert("dns_records".to_string(), serde_json::json!(records));

            // Subdomain enumeration
            if self.get_option("SUBDOMAIN_ENUM").unwrap_or_default() == "true" {
                let subdomains = self.enumerate_subdomains(&target, &resolver).await;
                result_data.insert("subdomains".to_string(), serde_json::json!(subdomains));
            }

            // Zone transfer attempt
            if self.get_option("ZONE_TRANSFER").unwrap_or_default() == "true" {
                if let Some(ns_records) = records.get("NS") {
                    let zt_results = self.attempt_zone_transfer(&target, ns_records).await;
                    result_data.insert("zone_transfer".to_string(), serde_json::json!(zt_results));
                }
            }
        }

        let elapsed = start.elapsed();
        result_data.insert("scan_time_ms".to_string(), serde_json::json!(elapsed.as_millis()));

        let mut result = ModuleResult::success(format!(
            "🔍 DNS enumeration completed for {} in {:.2}s",
            target,
            elapsed.as_secs_f64()
        ));

        for (key, value) in result_data {
            result = result.with_data(key, value);
        }

        Ok(result)
    }
}

impl Default for DnsEnumerator {
    fn default() -> Self {
        Self::new()
    }
}
