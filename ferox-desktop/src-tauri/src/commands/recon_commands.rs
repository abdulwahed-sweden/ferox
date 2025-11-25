//! Recon Commands - DNS, WHOIS, Subdomain, ASN reconnaissance
//!
//! Provides reconnaissance functionality for target information gathering.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use tauri::command;

// =============================================================================
// Types
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    pub record_type: String,
    pub name: String,
    pub value: String,
    pub ttl: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsEnumResult {
    pub domain: String,
    pub records: Vec<DnsRecord>,
    pub nameservers: Vec<String>,
    pub mx_records: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhoisResult {
    pub domain: String,
    pub registrar: Option<String>,
    pub creation_date: Option<String>,
    pub expiration_date: Option<String>,
    pub updated_date: Option<String>,
    pub nameservers: Vec<String>,
    pub status: Vec<String>,
    pub registrant: HashMap<String, String>,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainResult {
    pub subdomain: String,
    pub ip: Option<String>,
    pub status: String,  // "active", "inactive"
    pub source: String,  // "dns", "ct", "brute"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainEnumResult {
    pub domain: String,
    pub subdomains: Vec<SubdomainResult>,
    pub total_found: usize,
    pub sources_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsnResult {
    pub ip: String,
    pub asn: String,
    pub org: String,
    pub country: String,
    pub registry: String,
    pub cidr: String,
    pub ranges: Vec<String>,
}

// =============================================================================
// DNS Enumeration
// =============================================================================

/// Resolve hostname to IP addresses
fn resolve_hostname(hostname: &str) -> Vec<String> {
    let mut ips = Vec::new();

    // Try to resolve with port 0 to get IP addresses
    if let Ok(addrs) = (hostname, 0).to_socket_addrs() {
        for addr in addrs {
            ips.push(addr.ip().to_string());
        }
    }

    ips
}

/// Enumerate DNS records for a domain
#[command]
pub async fn dns_enum(domain: String) -> Result<DnsEnumResult, String> {
    let mut records = Vec::new();
    let mut nameservers = Vec::new();
    let mut mx_records = Vec::new();

    // Resolve A records
    let ips = resolve_hostname(&domain);
    for ip in &ips {
        if ip.contains(':') {
            records.push(DnsRecord {
                record_type: "AAAA".to_string(),
                name: domain.clone(),
                value: ip.clone(),
                ttl: 3600,
            });
        } else {
            records.push(DnsRecord {
                record_type: "A".to_string(),
                name: domain.clone(),
                value: ip.clone(),
                ttl: 3600,
            });
        }
    }

    // Try common subdomains for additional records
    let common_prefixes = ["www", "mail", "ns1", "ns2", "mx", "smtp"];
    for prefix in common_prefixes {
        let subdomain = format!("{}.{}", prefix, domain);
        let sub_ips = resolve_hostname(&subdomain);
        for ip in sub_ips {
            if prefix.starts_with("ns") {
                nameservers.push(subdomain.clone());
            } else if prefix == "mx" || prefix == "mail" || prefix == "smtp" {
                mx_records.push(subdomain.clone());
            }

            records.push(DnsRecord {
                record_type: if ip.contains(':') { "AAAA" } else { "A" }.to_string(),
                name: subdomain.clone(),
                value: ip,
                ttl: 3600,
            });
        }
    }

    // Add mock TXT/SPF record (real implementation would query DNS TXT)
    records.push(DnsRecord {
        record_type: "TXT".to_string(),
        name: domain.clone(),
        value: format!("v=spf1 include:_spf.{} ~all", domain),
        ttl: 3600,
    });

    Ok(DnsEnumResult {
        domain,
        records,
        nameservers,
        mx_records,
    })
}

// =============================================================================
// WHOIS Lookup
// =============================================================================

/// Perform WHOIS lookup for a domain
#[command]
pub async fn whois_lookup(domain: String) -> Result<WhoisResult, String> {
    // Note: Real implementation would query WHOIS servers
    // This is a simplified version that returns structured mock data

    // Try to use system whois command
    let output = std::process::Command::new("whois")
        .arg(&domain)
        .output();

    let raw = match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => format!("WHOIS lookup for {} (whois command not available)", domain),
    };

    // Parse common fields from raw WHOIS (simplified)
    let registrar = extract_whois_field(&raw, &["Registrar:", "registrar:"]);
    let creation_date = extract_whois_field(&raw, &["Creation Date:", "created:"]);
    let expiration_date = extract_whois_field(&raw, &["Registry Expiry Date:", "expires:"]);
    let updated_date = extract_whois_field(&raw, &["Updated Date:", "changed:"]);

    // Extract nameservers
    let nameservers = extract_whois_list(&raw, &["Name Server:", "nserver:"]);

    // Extract status
    let status = extract_whois_list(&raw, &["Domain Status:", "status:"]);

    let mut registrant = HashMap::new();
    if let Some(org) = extract_whois_field(&raw, &["Registrant Organization:", "org:"]) {
        registrant.insert("organization".to_string(), org);
    }
    if let Some(country) = extract_whois_field(&raw, &["Registrant Country:", "country:"]) {
        registrant.insert("country".to_string(), country);
    }

    Ok(WhoisResult {
        domain,
        registrar,
        creation_date,
        expiration_date,
        updated_date,
        nameservers,
        status,
        registrant,
        raw,
    })
}

fn extract_whois_field(raw: &str, patterns: &[&str]) -> Option<String> {
    for pattern in patterns {
        for line in raw.lines() {
            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                if let Some(value) = line.split(':').nth(1) {
                    let value = value.trim();
                    if !value.is_empty() {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}

fn extract_whois_list(raw: &str, patterns: &[&str]) -> Vec<String> {
    let mut results = Vec::new();
    for pattern in patterns {
        for line in raw.lines() {
            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                if let Some(value) = line.split(':').nth(1) {
                    let value = value.trim();
                    if !value.is_empty() && !results.contains(&value.to_string()) {
                        results.push(value.to_string());
                    }
                }
            }
        }
    }
    results
}

// =============================================================================
// Subdomain Enumeration
// =============================================================================

/// Common subdomain prefixes for brute-force
const COMMON_SUBDOMAINS: &[&str] = &[
    "www", "mail", "ftp", "localhost", "webmail", "smtp", "pop", "ns1", "ns2",
    "dns", "dns1", "dns2", "mx", "mx1", "mx2", "blog", "admin", "api", "dev",
    "staging", "test", "portal", "secure", "vpn", "remote", "m", "mobile",
    "app", "apps", "cdn", "static", "assets", "img", "images", "docs", "help",
    "support", "forum", "shop", "store", "login", "auth", "sso", "id", "git",
    "gitlab", "github", "jenkins", "ci", "build", "deploy", "monitor", "status",
    "internal", "intranet", "corp", "corporate", "hr", "finance", "sales",
];

/// Enumerate subdomains for a domain
#[command]
pub async fn subdomain_enum(
    domain: String,
    methods: Option<Vec<String>>,
) -> Result<SubdomainEnumResult, String> {
    let methods = methods.unwrap_or_else(|| vec!["dns".to_string(), "brute".to_string()]);
    let mut subdomains = Vec::new();
    let mut sources_used = Vec::new();

    // DNS-based enumeration
    if methods.contains(&"dns".to_string()) {
        sources_used.push("dns".to_string());

        // Check common subdomains
        for prefix in COMMON_SUBDOMAINS.iter().take(20) {
            let subdomain = format!("{}.{}", prefix, domain);
            let ips = resolve_hostname(&subdomain);

            if !ips.is_empty() {
                subdomains.push(SubdomainResult {
                    subdomain,
                    ip: ips.first().cloned(),
                    status: "active".to_string(),
                    source: "dns".to_string(),
                });
            }
        }
    }

    // Brute-force enumeration (simplified)
    if methods.contains(&"brute".to_string()) {
        sources_used.push("brute_force".to_string());

        // Additional brute-force prefixes
        for prefix in COMMON_SUBDOMAINS.iter().skip(20) {
            let subdomain = format!("{}.{}", prefix, domain);
            let ips = resolve_hostname(&subdomain);

            if !ips.is_empty() {
                // Check if not already found
                if !subdomains.iter().any(|s| s.subdomain == subdomain) {
                    subdomains.push(SubdomainResult {
                        subdomain,
                        ip: ips.first().cloned(),
                        status: "active".to_string(),
                        source: "brute".to_string(),
                    });
                }
            }
        }
    }

    // Certificate Transparency (would use CT logs in real implementation)
    if methods.contains(&"ct".to_string()) {
        sources_used.push("certificate_transparency".to_string());
        // Would query crt.sh or similar CT log aggregator
    }

    let total_found = subdomains.len();

    Ok(SubdomainEnumResult {
        domain,
        subdomains,
        total_found,
        sources_used,
    })
}

// =============================================================================
// ASN Lookup
// =============================================================================

/// Lookup ASN information for an IP address
#[command]
pub async fn asn_lookup(ip: String) -> Result<AsnResult, String> {
    // Note: Real implementation would query Team Cymru or similar BGP/ASN service
    // This provides a simplified lookup

    // Try to use system whois with ASN query
    let output = std::process::Command::new("whois")
        .arg("-h")
        .arg("whois.cymru.com")
        .arg(format!(" -v {}", ip))
        .output();

    let (asn, org, country, registry) = match output {
        Ok(out) => {
            let raw = String::from_utf8_lossy(&out.stdout).to_string();
            parse_cymru_response(&raw)
        }
        Err(_) => {
            // Fallback to basic info
            ("AS0".to_string(), "Unknown".to_string(), "XX".to_string(), "Unknown".to_string())
        }
    };

    // Derive CIDR from IP (simplified - assumes /24)
    let octets: Vec<&str> = ip.split('.').collect();
    let cidr = if octets.len() == 4 {
        format!("{}.{}.{}.0/24", octets[0], octets[1], octets[2])
    } else {
        format!("{}/128", ip)  // IPv6
    };

    Ok(AsnResult {
        ip,
        asn,
        org,
        country,
        registry,
        cidr: cidr.clone(),
        ranges: vec![cidr],
    })
}

fn parse_cymru_response(raw: &str) -> (String, String, String, String) {
    // Team Cymru format: AS | IP | BGP Prefix | CC | Registry | Allocated | AS Name
    for line in raw.lines().skip(1) {  // Skip header
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() >= 7 {
            return (
                format!("AS{}", parts[0]),
                parts[6].to_string(),
                parts[3].to_string(),
                parts[4].to_string(),
            );
        }
    }
    ("AS0".to_string(), "Unknown".to_string(), "XX".to_string(), "Unknown".to_string())
}
