//! Scanner Commands - Port scanning and HTTP analysis
//!
//! Provides real scanning functionality through the Ferox core library.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use tauri::command;

// =============================================================================
// Types
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortResult {
    pub port: u16,
    pub state: String,      // "open", "closed", "filtered"
    pub service: String,
    pub version: Option<String>,
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostScanResult {
    pub ip: String,
    pub hostname: Option<String>,
    pub status: String,     // "up", "down"
    pub ports: Vec<PortResult>,
    pub os_guess: Option<String>,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub id: String,
    pub target: String,
    pub hosts: Vec<HostScanResult>,
    pub start_time: String,
    pub end_time: String,
    pub duration_ms: u64,
    pub total_hosts: usize,
    pub hosts_up: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpScanResult {
    pub url: String,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub server: Option<String>,
    pub technologies: Vec<String>,
    pub title: Option<String>,
    pub response_time_ms: u64,
}

// =============================================================================
// Port Scanning
// =============================================================================

/// Common service names for well-known ports
fn get_service_name(port: u16) -> &'static str {
    match port {
        20 => "ftp-data",
        21 => "ftp",
        22 => "ssh",
        23 => "telnet",
        25 => "smtp",
        53 => "dns",
        80 => "http",
        110 => "pop3",
        111 => "rpcbind",
        135 => "msrpc",
        139 => "netbios-ssn",
        143 => "imap",
        443 => "https",
        445 => "microsoft-ds",
        993 => "imaps",
        995 => "pop3s",
        1433 => "ms-sql-s",
        1521 => "oracle",
        3306 => "mysql",
        3389 => "ms-wbt-server",
        5432 => "postgresql",
        5900 => "vnc",
        6379 => "redis",
        8080 => "http-proxy",
        8443 => "https-alt",
        27017 => "mongodb",
        _ => "unknown",
    }
}

/// Parse port range string (e.g., "22,80,443" or "1-1000")
fn parse_ports(ports_str: &str) -> Vec<u16> {
    let mut ports = Vec::new();

    for part in ports_str.split(',') {
        let part = part.trim();
        if part.contains('-') {
            // Range: "1-100"
            let parts: Vec<&str> = part.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (parts[0].parse::<u16>(), parts[1].parse::<u16>()) {
                    for port in start..=end {
                        ports.push(port);
                    }
                }
            }
        } else {
            // Single port
            if let Ok(port) = part.parse::<u16>() {
                ports.push(port);
            }
        }
    }

    ports
}

/// Parse host/CIDR string
fn parse_hosts(hosts_str: &str) -> Vec<String> {
    let mut hosts = Vec::new();

    for part in hosts_str.split(',') {
        let part = part.trim();
        if part.contains('/') {
            // CIDR notation - expand (simplified, only /24 for now)
            let parts: Vec<&str> = part.split('/').collect();
            if parts.len() == 2 {
                let base_ip = parts[0];
                let prefix: u8 = parts[1].parse().unwrap_or(24);

                if prefix == 24 {
                    // Expand /24 to 254 hosts
                    let octets: Vec<&str> = base_ip.split('.').collect();
                    if octets.len() == 4 {
                        let base = format!("{}.{}.{}.", octets[0], octets[1], octets[2]);
                        for i in 1..=254 {
                            hosts.push(format!("{}{}", base, i));
                        }
                    }
                } else {
                    hosts.push(part.to_string());
                }
            }
        } else {
            hosts.push(part.to_string());
        }
    }

    hosts
}

/// Scan a single port on a host
fn scan_port(ip: &str, port: u16, timeout_ms: u64) -> PortResult {
    let addr = format!("{}:{}", ip, port);
    let start = Instant::now();

    let state = match addr.parse::<SocketAddr>() {
        Ok(socket_addr) => {
            match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(timeout_ms)) {
                Ok(_stream) => "open",
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::ConnectionRefused {
                        "closed"
                    } else {
                        "filtered"
                    }
                }
            }
        }
        Err(_) => "filtered",
    };

    let _latency = start.elapsed().as_millis() as u64;

    PortResult {
        port,
        state: state.to_string(),
        service: get_service_name(port).to_string(),
        version: None,  // Would require service detection
        banner: None,   // Would require banner grabbing
    }
}

/// Scan ports on a target
#[command]
pub async fn scan_ports(
    hosts: String,
    ports: String,
    threads: Option<u32>,
    timeout: Option<u64>,
) -> Result<ScanResult, String> {
    let start_time = Instant::now();
    let start_ts = chrono::Utc::now().to_rfc3339();

    let host_list = parse_hosts(&hosts);
    let port_list = parse_ports(&ports);
    let timeout_ms = timeout.unwrap_or(3000);
    let _thread_count = threads.unwrap_or(10);  // For future parallel implementation

    let mut results: Vec<HostScanResult> = Vec::new();

    for ip in &host_list {
        let host_start = Instant::now();
        let mut open_ports = Vec::new();

        for port in &port_list {
            let result = scan_port(ip, *port, timeout_ms);
            if result.state == "open" {
                open_ports.push(result);
            }
        }

        let latency = host_start.elapsed().as_millis() as u64;
        let status = if open_ports.is_empty() { "down" } else { "up" };

        results.push(HostScanResult {
            ip: ip.clone(),
            hostname: None,  // Would require reverse DNS
            status: status.to_string(),
            ports: open_ports,
            os_guess: None,  // Would require OS detection
            latency_ms: latency,
        });
    }

    let duration = start_time.elapsed().as_millis() as u64;
    let end_ts = chrono::Utc::now().to_rfc3339();
    let hosts_up = results.iter().filter(|h| h.status == "up").count();

    Ok(ScanResult {
        id: uuid::Uuid::new_v4().to_string(),
        target: hosts,
        hosts: results.clone(),
        start_time: start_ts,
        end_time: end_ts,
        duration_ms: duration,
        total_hosts: results.len(),
        hosts_up,
    })
}

// =============================================================================
// HTTP Scanning
// =============================================================================

/// Scan HTTP/HTTPS URLs
#[command]
pub async fn scan_http(
    urls: Vec<String>,
    follow_redirects: Option<bool>,
    timeout: Option<u64>,
) -> Result<Vec<HttpScanResult>, String> {
    let timeout_ms = timeout.unwrap_or(10000);
    let _follow = follow_redirects.unwrap_or(true);

    let mut results = Vec::new();

    for url in urls {
        let start = Instant::now();

        // Use reqwest for HTTP requests
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .danger_accept_invalid_certs(true)  // For testing
            .build()
            .map_err(|e| e.to_string())?;

        match client.get(&url).send().await {
            Ok(response) => {
                let status_code = response.status().as_u16();
                let mut headers_map = HashMap::new();

                for (key, value) in response.headers() {
                    if let Ok(v) = value.to_str() {
                        headers_map.insert(key.to_string(), v.to_string());
                    }
                }

                let server = headers_map.get("server").cloned();
                let response_time = start.elapsed().as_millis() as u64;

                // Detect technologies from headers
                let mut technologies = Vec::new();
                if let Some(powered_by) = headers_map.get("x-powered-by") {
                    technologies.push(powered_by.clone());
                }
                if let Some(srv) = &server {
                    if srv.to_lowercase().contains("nginx") {
                        technologies.push("nginx".to_string());
                    } else if srv.to_lowercase().contains("apache") {
                        technologies.push("Apache".to_string());
                    }
                }

                // Try to get title from HTML (simplified)
                let title = if let Ok(body) = response.text().await {
                    extract_title(&body)
                } else {
                    None
                };

                results.push(HttpScanResult {
                    url: url.clone(),
                    status_code,
                    headers: headers_map,
                    server,
                    technologies,
                    title,
                    response_time_ms: response_time,
                });
            }
            Err(e) => {
                results.push(HttpScanResult {
                    url: url.clone(),
                    status_code: 0,
                    headers: HashMap::new(),
                    server: None,
                    technologies: Vec::new(),
                    title: Some(format!("Error: {}", e)),
                    response_time_ms: start.elapsed().as_millis() as u64,
                });
            }
        }
    }

    Ok(results)
}

/// Extract title from HTML
fn extract_title(html: &str) -> Option<String> {
    let lower = html.to_lowercase();
    if let Some(start) = lower.find("<title>") {
        if let Some(end) = lower[start..].find("</title>") {
            let title_start = start + 7;
            let title_end = start + end;
            if title_end > title_start && title_end <= html.len() {
                return Some(html[title_start..title_end].trim().to_string());
            }
        }
    }
    None
}
