use crate::core::module::*;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{Duration, timeout};
use hickory_resolver::TokioResolver;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubdomainRecord {
    pub subdomain: String,
    pub ips: Vec<String>,
    pub http_status: Option<u16>,
    pub title: Option<String>,
    pub resolved: bool,
}

impl SubdomainRecord {
    fn new(subdomain: String) -> Self {
        Self {
            subdomain,
            ips: Vec::new(),
            http_status: None,
            title: None,
            resolved: false,
        }
    }
}

pub struct SubdomainEnum {
    options: HashMap<String, String>,
}

impl SubdomainEnum {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("RHOSTS".to_string(), String::new());
        options.insert("WORDLIST".to_string(), "./wordlist.txt".to_string());
        options.insert("THREADS".to_string(), "50".to_string());
        options.insert("TIMEOUT".to_string(), "2000".to_string());
        options.insert("PROBE_HTTP".to_string(), "true".to_string());
        options.insert("OUTPUT".to_string(), "human".to_string());

        Self { options }
    }

    fn read_wordlist<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut words = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            words.push(line.to_string());
        }

        if words.is_empty() {
            return Err(anyhow!("Wordlist is empty"));
        }

        Ok(words)
    }

    async fn resolve_subdomain(
        resolver: Arc<TokioResolver>,
        subdomain: String,
        timeout_ms: u64,
    ) -> Result<SubdomainRecord> {
        let mut record = SubdomainRecord::new(subdomain.clone());

        match timeout(
            Duration::from_millis(timeout_ms),
            resolver.lookup_ip(subdomain.as_str()),
        )
        .await
        {
            Ok(Ok(response)) => {
                record.ips = response.iter().map(|ip| ip.to_string()).collect();
                record.resolved = !record.ips.is_empty();
            }
            _ => {
                record.resolved = false;
            }
        }

        Ok(record)
    }

    async fn probe_http(
        client: &reqwest::Client,
        subdomain: &str,
        timeout_ms: u64,
    ) -> (Option<u16>, Option<String>) {
        let url = format!("http://{}", subdomain);

        // Try HEAD request first
        match timeout(Duration::from_millis(timeout_ms), client.head(&url).send()).await {
            Ok(Ok(resp)) => {
                let status = resp.status().as_u16();

                // If successful, try GET for title
                if resp.status().is_success()
                    && let Ok(Ok(get_resp)) =
                        timeout(Duration::from_millis(timeout_ms), client.get(&url).send()).await
                    && let Ok(text) = get_resp.text().await
                {
                    let title = Self::extract_title(&text);
                    return (Some(status), title);
                }

                (Some(status), None)
            }
            _ => (None, None),
        }
    }

    fn extract_title(html: &str) -> Option<String> {
        if let Some(start) = html.find("<title>")
            && let Some(end) = html[start..].find("</title>")
        {
            let title = html[start + 7..start + end].trim().to_string();
            if !title.is_empty() {
                return Some(title);
            }
        }
        None
    }
}

#[async_trait]
impl Module for SubdomainEnum {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "subdomain_enum".to_string(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "Non-destructive subdomain enumeration via DNS resolution with optional HTTP probing".to_string(),
            module_type: ModuleType::Scanner,
            category: "recon".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "RHOSTS".to_string(),
                description: "Target domain (e.g., example.com)".to_string(),
                required: true,
                default_value: None,
                current_value: self.get_option("RHOSTS"),
            },
            ModuleOption {
                name: "WORDLIST".to_string(),
                description: "Path to subdomain wordlist file".to_string(),
                required: true,
                default_value: Some("./wordlist.txt".to_string()),
                current_value: self.get_option("WORDLIST"),
            },
            ModuleOption {
                name: "THREADS".to_string(),
                description: "Number of concurrent threads".to_string(),
                required: false,
                default_value: Some("50".to_string()),
                current_value: self.get_option("THREADS"),
            },
            ModuleOption {
                name: "TIMEOUT".to_string(),
                description: "Timeout per request in milliseconds".to_string(),
                required: false,
                default_value: Some("2000".to_string()),
                current_value: self.get_option("TIMEOUT"),
            },
            ModuleOption {
                name: "PROBE_HTTP".to_string(),
                description: "Probe HTTP/HTTPS after DNS resolution (true/false)".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.get_option("PROBE_HTTP"),
            },
            ModuleOption {
                name: "OUTPUT".to_string(),
                description: "Output format: human or json".to_string(),
                required: false,
                default_value: Some("human".to_string()),
                current_value: self.get_option("OUTPUT"),
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

        let wordlist_path = self.get_option("WORDLIST").unwrap_or_default();
        if !Path::new(&wordlist_path).exists() {
            return Err(anyhow!("Wordlist file not found: {}", wordlist_path));
        }

        Ok(())
    }

    async fn check(&self) -> Result<CheckResult> {
        self.validate()?;

        let domain = self.get_option("RHOSTS").unwrap();

        // Create resolver
        let resolver = TokioResolver::builder_tokio()?.build();

        // Try to resolve base domain
        match timeout(Duration::from_secs(5), resolver.lookup_ip(domain.as_str())).await {
            Ok(Ok(response)) => {
                let ips: Vec<String> = response.iter().map(|ip| ip.to_string()).collect();
                let mut fingerprint = HashMap::new();
                fingerprint.insert("domain".to_string(), domain.clone());
                fingerprint.insert("resolved_ips".to_string(), ips.join(", "));

                Ok(CheckResult {
                    vulnerable: false,
                    confidence: 1.0,
                    details: format!("Base domain {} resolves to {} IP(s)", domain, ips.len()),
                    fingerprint,
                })
            }
            _ => Ok(CheckResult {
                vulnerable: false,
                confidence: 0.0,
                details: format!("Base domain {} does not resolve", domain),
                fingerprint: HashMap::new(),
            }),
        }
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        self.validate()?;

        let domain = self.get_option("RHOSTS").unwrap();
        let wordlist_path = self.get_option("WORDLIST").unwrap();
        let threads: usize = self
            .get_option("THREADS")
            .and_then(|s| s.parse().ok())
            .unwrap_or(50);
        let timeout_ms: u64 = self
            .get_option("TIMEOUT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(2000);
        let probe_http = self
            .get_option("PROBE_HTTP")
            .map(|s| s.to_lowercase() == "true")
            .unwrap_or(true);
        let output = self
            .get_option("OUTPUT")
            .unwrap_or_else(|| "human".to_string());

        // Read wordlist
        let words = Self::read_wordlist(&wordlist_path)?;

        // Create resolver and HTTP client
        let resolver = Arc::new(TokioResolver::builder_tokio()?.build());
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .danger_accept_invalid_certs(true)
            .build()?;

        let results: Arc<Mutex<Vec<SubdomainRecord>>> = Arc::new(Mutex::new(Vec::new()));
        let semaphore = Arc::new(Semaphore::new(threads));

        // Spawn tasks
        let mut handles = Vec::new();

        for word in words {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let subdomain = format!("{}.{}", word, domain);
            let resolver = resolver.clone();
            let client = client.clone();
            let results = results.clone();

            let handle = tokio::spawn(async move {
                let _permit = permit;

                // DNS resolution
                let mut record =
                    match Self::resolve_subdomain(resolver, subdomain.clone(), timeout_ms).await {
                        Ok(r) => r,
                        Err(_) => return,
                    };

                // HTTP probing if enabled and resolved
                if probe_http && record.resolved {
                    let (status, title) = Self::probe_http(&client, &subdomain, timeout_ms).await;
                    record.http_status = status;
                    record.title = title;
                }

                // Store only if resolved
                if record.resolved {
                    let mut guard = results.lock().await;
                    guard.push(record);
                }
            });

            handles.push(handle);
        }

        // Wait for all tasks
        for handle in handles {
            let _ = handle.await;
        }

        // Collect results
        let guard = results.lock().await;
        let found: Vec<SubdomainRecord> = guard.clone();
        drop(guard);

        // Prepare result
        let mut result =
            ModuleResult::success(format!("Found {} subdomains for {}", found.len(), domain));

        result = result.with_data("subdomains", serde_json::to_value(&found)?);
        result = result.with_data("total_found", serde_json::json!(found.len()));
        result = result.with_data("domain", serde_json::json!(domain));

        // Output based on format
        if output != "json" {
            println!();
            println!("  {}", "Results:".bright_cyan().bold());
            println!("  {}", "─".repeat(85).bright_blue());

            for record in &found {
                let ips = record.ips.join(", ");
                let http_info = if let Some(status) = record.http_status {
                    format!(" [HTTP: {}]", status)
                } else {
                    String::new()
                };

                let title_info = if let Some(title) = &record.title {
                    format!(" - {}", title)
                } else {
                    String::new()
                };

                println!(
                    "  {} → {}{}{}",
                    record.subdomain.bright_green(),
                    ips.bright_yellow(),
                    http_info.bright_blue(),
                    title_info.bright_white()
                );
            }
            println!();
        }

        Ok(result)
    }
}

impl Default for SubdomainEnum {
    fn default() -> Self {
        Self::new()
    }
}

use colored::Colorize;
