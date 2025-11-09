use crate::core::module::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::redirect::Policy;
use reqwest::{Client, Url};
use futures::{stream, StreamExt};
use std::collections::HashMap;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio_native_tls::native_tls::TlsConnector;
use tokio_native_tls::TlsConnector as AsyncTlsConnector;
use x509_parser::prelude::*;

/// Extract TLS certificate details from an HTTPS host
async fn extract_tls_info(host: &str, port: u16) -> Result<serde_json::Value> {
    let addr = format!("{}:{}", host, port);
    let tcp = TcpStream::connect(&addr).await?;
    
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true) // Accept self-signed for analysis
        .build()?;
    let async_connector = AsyncTlsConnector::from(connector);
    let tls_stream = async_connector.connect(host, tcp).await?;
    
    // Get peer certificate
    let tls_ref = tls_stream.get_ref();
    let cert_der = tls_ref
        .peer_certificate()?
        .ok_or_else(|| anyhow!("No peer certificate"))?
        .to_der()?;
    
    // Parse with x509-parser
    let (_, cert) = X509Certificate::from_der(&cert_der)
        .map_err(|e| anyhow!("Failed to parse certificate: {:?}", e))?;
    
    let subject = cert.subject().to_string();
    let issuer = cert.issuer().to_string();
    let not_before = cert.validity().not_before.to_datetime().to_string();
    let not_after = cert.validity().not_after.to_datetime().to_string();
    
    // Calculate days until expiry
    let now = chrono::Utc::now();
    let expiry = cert.validity().not_after.to_datetime();
    let days_to_expiry = (expiry.unix_timestamp() - now.timestamp()) / 86400;
    
    Ok(serde_json::json!({
        "subject": subject,
        "issuer": issuer,
        "not_before": not_before,
        "not_after": not_after,
        "days_to_expiry": days_to_expiry,
        "serial_number": format!("{:X}", cert.serial),
        "signature_algorithm": cert.signature_algorithm.algorithm.to_string(),
    }))
}

pub struct HttpScanner {
    options: HashMap<String, String>,
}

impl HttpScanner {
    pub fn new() -> Self {
        let mut options = HashMap::new();
        options.insert("RHOSTS".to_string(), String::new());
        options.insert("THREADS".to_string(), "10".to_string());
        options.insert("TIMEOUT".to_string(), "5000".to_string());
        options.insert("FOLLOW_REDIRECTS".to_string(), "true".to_string());
        options.insert("USER_AGENT".to_string(), "Ferox/2.0 (HTTP Scanner)".to_string());
        options.insert("RATE_LIMIT".to_string(), "0".to_string()); // req/sec, 0 = unlimited
        options.insert("PATHS".to_string(), "/".to_string()); // comma-separated paths
        Self { options }
    }

    fn build_client(&self) -> Result<Client> {
        let timeout_ms = self
            .get_option("TIMEOUT")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(5000);
        let follow = self
            .get_option("FOLLOW_REDIRECTS")
            .map(|v| v.to_lowercase())
            .unwrap_or_else(|| "true".to_string())
            == "true";
        let ua = self
            .get_option("USER_AGENT")
            .unwrap_or_else(|| "Ferox/2.0 (HTTP Scanner)".to_string());

        let mut builder = reqwest::Client::builder()
            .user_agent(ua)
            .timeout(std::time::Duration::from_millis(timeout_ms));
        if follow {
            builder = builder.redirect(Policy::limited(10));
        } else {
            builder = builder.redirect(Policy::none());
        }
        Ok(builder.build()?)
    }

    fn normalize_target(&self) -> Result<Url> {
        let target = self
            .get_option("RHOSTS")
            .unwrap_or_default()
            .trim()
            .to_string();
        if target.is_empty() {
            return Err(anyhow!("RHOSTS is required"));
        }
        // If no scheme, default to http
        let url = if target.starts_with("http://") || target.starts_with("https://") {
            target
        } else {
            format!("http://{}", target)
        };
        Ok(Url::parse(&url)?)
    }

    fn detect_technologies(&self, headers: &reqwest::header::HeaderMap, body: &str) -> Vec<String> {
        let mut techs = Vec::new();
        // Header-based hints
        if let Some(server) = headers.get(reqwest::header::SERVER).and_then(|v| v.to_str().ok()) {
            techs.push(format!("Server:{}", server));
        }
        if let Some(xpb) = headers
            .get("x-powered-by")
            .and_then(|v| v.to_str().ok())
        {
            techs.push(format!("PoweredBy:{}", xpb));
        }
        if headers.contains_key("x-akamai-transformed") {
            techs.push("Akamai".to_string());
        }
        if headers.contains_key("cf-ray") || headers.contains_key("cf-cache-status") {
            techs.push("Cloudflare".to_string());
        }
        if headers.contains_key("x-sucuri-id") {
            techs.push("Sucuri".to_string());
        }

        // Body heuristics
        let lower = body.to_lowercase();
        if lower.contains("wp-content") || lower.contains("wp-includes") {
            techs.push("WordPress".to_string());
        }
        if lower.contains("content=\"wordpress") {
            techs.push("WordPress".to_string());
        }
        if lower.contains("content=\"joomla") {
            techs.push("Joomla".to_string());
        }
        if lower.contains("drupal.settings") || lower.contains("/sites/all/") {
            techs.push("Drupal".to_string());
        }
        if lower.contains("<meta name=\"generator\"") {
            techs.push("CMS (generator meta)".to_string());
        }

        techs.sort();
        techs.dedup();
        techs
    }
}

#[async_trait]
impl Module for HttpScanner {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "http_scanner".to_string(),
            version: "0.1.0".to_string(),
            author: "Ferox Team".to_string(),
            description: "HTTP/HTTPS fingerprinting, headers, status, and simple tech detection".to_string(),
            module_type: ModuleType::Scanner,
            category: "scanner".to_string(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![
            ModuleOption {
                name: "RHOSTS".to_string(),
                description: "Target URL or hostname (e.g., https://example.com)".to_string(),
                required: true,
                default_value: None,
                current_value: self.get_option("RHOSTS"),
            },
            ModuleOption {
                name: "THREADS".to_string(),
                description: "Concurrency for path scanning (unused for single root)".to_string(),
                required: false,
                default_value: Some("10".to_string()),
                current_value: self.get_option("THREADS"),
            },
            ModuleOption {
                name: "TIMEOUT".to_string(),
                description: "Request timeout in ms".to_string(),
                required: false,
                default_value: Some("5000".to_string()),
                current_value: self.get_option("TIMEOUT"),
            },
            ModuleOption {
                name: "FOLLOW_REDIRECTS".to_string(),
                description: "Follow HTTP redirects (true/false)".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                current_value: self.get_option("FOLLOW_REDIRECTS"),
            },
            ModuleOption {
                name: "USER_AGENT".to_string(),
                description: "Custom User-Agent string".to_string(),
                required: false,
                default_value: Some("Ferox/2.0 (HTTP Scanner)".to_string()),
                current_value: self.get_option("USER_AGENT"),
            },
            ModuleOption {
                name: "RATE_LIMIT".to_string(),
                description: "Requests per second (0 = unlimited)".to_string(),
                required: false,
                default_value: Some("0".to_string()),
                current_value: self.get_option("RATE_LIMIT"),
            },
            ModuleOption {
                name: "PATHS".to_string(),
                description: "Comma-separated paths to probe (default: /)".to_string(),
                required: false,
                default_value: Some("/".to_string()),
                current_value: self.get_option("PATHS"),
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

    async fn check(&self) -> Result<CheckResult> {
        let base = self.normalize_target()?;
        let client = self.build_client()?;
        let start = Instant::now();
        let resp = client.head(base.clone()).send().await?;
        let elapsed_ms = start.elapsed().as_millis() as u64;
        let status = resp.status().as_u16();

        let mut fp = HashMap::new();
        fp.insert("url".to_string(), base.to_string());
        fp.insert("scheme".to_string(), base.scheme().to_string());
        fp.insert("status".to_string(), status.to_string());
        if let Some(server) = resp.headers().get(reqwest::header::SERVER) {
            if let Ok(s) = server.to_str() {
                fp.insert("server".to_string(), s.to_string());
            }
        }

        Ok(CheckResult {
            vulnerable: false,
            confidence: 1.0,
            details: format!("HEAD {} -> {} in {} ms", base, status, elapsed_ms),
            fingerprint: fp,
        })
    }

    async fn run(&mut self) -> Result<ModuleResult> {
        self.validate()?;
        let base = self.normalize_target()?;

        // Prepare paths and concurrency
        let paths_raw = self.get_option("PATHS").unwrap_or_else(|| "/".to_string());
        let mut paths: Vec<String> = paths_raw
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if paths.is_empty() {
            paths.push("/".to_string());
        }
        let threads = self
            .get_option("THREADS")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(10)
            .max(1);

        // Rate limit (simple sleep per request)
        let rate_per_sec = self
            .get_option("RATE_LIMIT")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);
        let rate_delay = if rate_per_sec > 0 {
            Some(std::time::Duration::from_millis(1000 / rate_per_sec))
        } else {
            None
        };

        // Always handle redirects manually to capture chain
        // Build a second client without automatic redirects (reqwest doesn't expose getters for UA/timeout)
        let client_no_redirect = reqwest::Client::builder()
            .user_agent(self.get_option("USER_AGENT").unwrap_or_else(|| "Ferox/2.0 (HTTP Scanner)".to_string()))
            .timeout(std::time::Duration::from_millis(
                self.get_option("TIMEOUT").and_then(|v| v.parse::<u64>().ok()).unwrap_or(5000)
            ))
            .redirect(Policy::none())
            .build()?;

        let scheme = base.scheme().to_string();
        let is_https = scheme.eq_ignore_ascii_case("https");
        let mut tls_info_json: Option<serde_json::Value> = None;
        if is_https {
            // Extract TLS certificate details
            let host = base.host_str().ok_or_else(|| anyhow!("No host in URL"))?;
            let port = base.port().unwrap_or(443);
            match extract_tls_info(host, port).await {
                Ok(info) => tls_info_json = Some(info),
                Err(e) => {
                    tls_info_json = Some(serde_json::json!({
                        "error": format!("TLS handshake failed: {}", e)
                    }));
                }
            }
        }

        let results = stream::iter(paths.into_iter())
            .map(|p| {
                let client = client_no_redirect.clone();
                let mut url = base.clone();
                if p != "/" {
                    url.set_path(&p);
                }
                let rate_delay = rate_delay.clone();
                async move {
                    if let Some(d) = rate_delay { tokio::time::sleep(d).await; }

                    let mut current = url.clone();
                    let mut chain: Vec<serde_json::Value> = Vec::new();
                    let start = Instant::now();
                    let final_resp = loop {
                        let resp = client.get(current.clone()).send().await;
                        match resp {
                            Ok(r) => {
                                let status = r.status();
                                let this_url = r.url().clone();
                                if status.is_redirection() {
                                    if let Some(loc) = r.headers().get(reqwest::header::LOCATION) {
                                        if let Ok(loc_str) = loc.to_str() {
                                            // Resolve relative locations
                                            match this_url.join(loc_str) {
                                                Ok(next_url) => {
                                                    chain.push(serde_json::json!({
                                                        "status": status.as_u16(),
                                                        "url": this_url.to_string(),
                                                        "location": next_url.to_string()
                                                    }));
                                                    current = next_url;
                                                    if chain.len() > 10 { break Ok(r); }
                                                    continue;
                                                }
                                                Err(_) => { break Ok(r); }
                                            }
                                        } else {
                                            break Ok(r);
                                        }
                                    } else {
                                        break Ok(r);
                                    }
                                } else {
                                    break Ok(r);
                                }
                            }
                            Err(e) => break Err(e),
                        }
                    };

                    let elapsed_ms = start.elapsed().as_millis() as u64;
                    match final_resp {
                        Ok(r) => {
                            let status = r.status().as_u16();
                            let final_url = r.url().clone();
                            let headers = r.headers().clone();
                            let body = r.text().await.unwrap_or_default();
                            let server = headers
                                .get(reqwest::header::SERVER)
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("")
                                .to_string();
                            let content_type = headers
                                .get(reqwest::header::CONTENT_TYPE)
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("")
                                .to_string();
                            let content_length = headers
                                .get(reqwest::header::CONTENT_LENGTH)
                                .and_then(|v| v.to_str().ok())
                                .and_then(|s| s.parse::<u64>().ok())
                                .unwrap_or(body.len() as u64);
                            let technologies = crate::modules::scanner::http_scanner::HttpScanner::default().detect_technologies(&headers, &body);

                            serde_json::json!({
                                "path": p,
                                "requested_url": url.to_string(),
                                "final_url": final_url.to_string(),
                                "status": status,
                                "response_time_ms": elapsed_ms,
                                "server": server,
                                "content_type": content_type,
                                "content_length": content_length,
                                "redirect_chain": chain,
                                "technologies": technologies,
                            })
                        }
                        Err(e) => {
                            serde_json::json!({
                                "path": p,
                                "requested_url": url.to_string(),
                                "error": e.to_string(),
                                "response_time_ms": elapsed_ms,
                                "redirect_chain": chain,
                            })
                        }
                    }
                }
            })
            .buffer_unordered(threads)
            .collect::<Vec<_>>()
            .await;

        let mut result = ModuleResult::success(format!(
            "{} HTTP scan on {} ({} paths)",
            if is_https { "🔒" } else { "🌐" },
            base.host_str().unwrap_or("unknown"),
            results.len()
        ));

        result = result
            .with_data("base_url", serde_json::json!(base.to_string()))
            .with_data("https", serde_json::json!(is_https))
            .with_data("tls", serde_json::json!(tls_info_json))
            .with_data("results", serde_json::json!(results));

        Ok(result)
    }
}

// (Detailed TLS parsing removed due to missing build tooling; placeholder retained.)

impl Default for HttpScanner {
    fn default() -> Self {
        Self::new()
    }
}
