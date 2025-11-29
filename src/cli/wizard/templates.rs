//! Attack Templates
//!
//! Pre-built attack templates for common assessment scenarios

use super::types::*;
use std::collections::HashMap;

/// Template type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemplateType {
    /// Web application assessment
    WebApp,
    /// Network infrastructure assessment
    Network,
    /// Domain reconnaissance
    Domain,
    /// Quick port and HTTP scan
    QuickScan,
}

impl TemplateType {
    pub fn name(&self) -> &str {
        match self {
            TemplateType::WebApp => "web-app",
            TemplateType::Network => "network",
            TemplateType::Domain => "domain",
            TemplateType::QuickScan => "quick-scan",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            TemplateType::WebApp => "Web application assessment - HTTP fingerprinting, tech detection",
            TemplateType::Network => "Network infrastructure - port scanning, service enumeration",
            TemplateType::Domain => "Domain reconnaissance - DNS, WHOIS, subdomains, ASN",
            TemplateType::QuickScan => "Fast port scan + HTTP fingerprint only",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "web-app" | "webapp" | "web" => Some(TemplateType::WebApp),
            "network" | "net" | "infra" => Some(TemplateType::Network),
            "domain" | "recon" | "dns" => Some(TemplateType::Domain),
            "quick-scan" | "quick" | "fast" => Some(TemplateType::QuickScan),
            _ => None,
        }
    }

    pub fn all() -> Vec<TemplateType> {
        vec![
            TemplateType::WebApp,
            TemplateType::Network,
            TemplateType::Domain,
            TemplateType::QuickScan,
        ]
    }
}

/// Attack template
#[derive(Debug, Clone)]
pub struct AttackTemplate {
    pub template_type: TemplateType,
    pub name: String,
    pub description: String,
    pub phases: Vec<TemplatePhase>,
    pub recommended_target_type: TargetType,
    pub default_intensity: IntensityLevel,
}

/// Template phase
#[derive(Debug, Clone)]
pub struct TemplatePhase {
    pub name: String,
    pub phase_number: usize,
    pub modules: Vec<TemplateModule>,
}

/// Template module
#[derive(Debug, Clone)]
pub struct TemplateModule {
    pub path: String,
    pub name: String,
    pub purpose: String,
    pub default_options: HashMap<String, String>,
    pub required: bool,
}

impl AttackTemplate {
    /// Create web application assessment template
    pub fn web_app() -> Self {
        Self {
            template_type: TemplateType::WebApp,
            name: "Web Application Assessment".to_string(),
            description: "Comprehensive web application security assessment".to_string(),
            recommended_target_type: TargetType::Domain,
            default_intensity: IntensityLevel::Normal,
            phases: vec![
                TemplatePhase {
                    name: "Reconnaissance".to_string(),
                    phase_number: 1,
                    modules: vec![
                        TemplateModule {
                            path: "recon/dns_enum".to_string(),
                            name: "DNS Enumeration".to_string(),
                            purpose: "DNS records & zone info".to_string(),
                            default_options: HashMap::from([
                                ("RECORD_TYPES".to_string(), "A,AAAA,MX,NS,TXT,CNAME".to_string()),
                            ]),
                            required: true,
                        },
                        TemplateModule {
                            path: "recon/subdomain_enum".to_string(),
                            name: "Subdomain Enumeration".to_string(),
                            purpose: "Discover subdomains".to_string(),
                            default_options: HashMap::from([
                                ("PROBE_HTTP".to_string(), "true".to_string()),
                            ]),
                            required: true,
                        },
                    ],
                },
                TemplatePhase {
                    name: "Enumeration".to_string(),
                    phase_number: 2,
                    modules: vec![
                        TemplateModule {
                            path: "scanner/port_scanner".to_string(),
                            name: "Port Scanner".to_string(),
                            purpose: "Common web ports".to_string(),
                            default_options: HashMap::from([
                                ("PORTS".to_string(), "80,443,8080,8443,8000,3000,5000".to_string()),
                            ]),
                            required: true,
                        },
                        TemplateModule {
                            path: "scanner/http_scanner".to_string(),
                            name: "HTTP Scanner".to_string(),
                            purpose: "Web technology fingerprinting".to_string(),
                            default_options: HashMap::from([
                                ("FOLLOW_REDIRECTS".to_string(), "true".to_string()),
                                ("PATHS".to_string(), "/,/robots.txt,/sitemap.xml".to_string()),
                            ]),
                            required: true,
                        },
                    ],
                },
                TemplatePhase {
                    name: "Post-Exploitation".to_string(),
                    phase_number: 3,
                    modules: vec![
                        TemplateModule {
                            path: "post/browser/deep_session_hijack".to_string(),
                            name: "Session Analysis".to_string(),
                            purpose: "Browser session analysis (mock mode)".to_string(),
                            default_options: HashMap::from([
                                ("mock_mode".to_string(), "true".to_string()),
                            ]),
                            required: false,
                        },
                    ],
                },
            ],
        }
    }

    /// Create network infrastructure template
    pub fn network() -> Self {
        Self {
            template_type: TemplateType::Network,
            name: "Network Infrastructure Assessment".to_string(),
            description: "Network infrastructure security assessment".to_string(),
            recommended_target_type: TargetType::SingleHost,
            default_intensity: IntensityLevel::Normal,
            phases: vec![
                TemplatePhase {
                    name: "Reconnaissance".to_string(),
                    phase_number: 1,
                    modules: vec![
                        TemplateModule {
                            path: "recon/whois_lookup".to_string(),
                            name: "WHOIS Lookup".to_string(),
                            purpose: "Registration & ownership".to_string(),
                            default_options: HashMap::new(),
                            required: true,
                        },
                        TemplateModule {
                            path: "recon/asn_discovery".to_string(),
                            name: "ASN Discovery".to_string(),
                            purpose: "Network range identification".to_string(),
                            default_options: HashMap::from([
                                ("LOOKUP_PREFIXES".to_string(), "true".to_string()),
                            ]),
                            required: true,
                        },
                    ],
                },
                TemplatePhase {
                    name: "Enumeration".to_string(),
                    phase_number: 2,
                    modules: vec![
                        TemplateModule {
                            path: "scanner/port_scanner".to_string(),
                            name: "Port Scanner".to_string(),
                            purpose: "Full port scan".to_string(),
                            default_options: HashMap::from([
                                ("PORTS".to_string(), "1-1000".to_string()),
                            ]),
                            required: true,
                        },
                        TemplateModule {
                            path: "scanner/http_scanner".to_string(),
                            name: "HTTP Scanner".to_string(),
                            purpose: "HTTP service detection".to_string(),
                            default_options: HashMap::new(),
                            required: false,
                        },
                    ],
                },
                TemplatePhase {
                    name: "Post-Exploitation".to_string(),
                    phase_number: 3,
                    modules: vec![
                        TemplateModule {
                            path: "evasion/edr/silent_shadow".to_string(),
                            name: "EDR Detection".to_string(),
                            purpose: "Security product detection".to_string(),
                            default_options: HashMap::from([
                                ("technique".to_string(), "detection_only".to_string()),
                                ("mock_mode".to_string(), "true".to_string()),
                            ]),
                            required: false,
                        },
                    ],
                },
            ],
        }
    }

    /// Create domain reconnaissance template
    pub fn domain() -> Self {
        Self {
            template_type: TemplateType::Domain,
            name: "Domain Reconnaissance".to_string(),
            description: "Comprehensive domain information gathering".to_string(),
            recommended_target_type: TargetType::Domain,
            default_intensity: IntensityLevel::Stealth,
            phases: vec![
                TemplatePhase {
                    name: "Reconnaissance".to_string(),
                    phase_number: 1,
                    modules: vec![
                        TemplateModule {
                            path: "recon/dns_enum".to_string(),
                            name: "DNS Enumeration".to_string(),
                            purpose: "DNS records & zone info".to_string(),
                            default_options: HashMap::from([
                                ("RECORD_TYPES".to_string(), "A,AAAA,MX,NS,TXT,SOA,CNAME".to_string()),
                                ("ZONE_TRANSFER".to_string(), "true".to_string()),
                            ]),
                            required: true,
                        },
                        TemplateModule {
                            path: "recon/whois_lookup".to_string(),
                            name: "WHOIS Lookup".to_string(),
                            purpose: "Registration & ownership".to_string(),
                            default_options: HashMap::from([
                                ("FOLLOW_REFERRAL".to_string(), "true".to_string()),
                            ]),
                            required: true,
                        },
                        TemplateModule {
                            path: "recon/asn_discovery".to_string(),
                            name: "ASN Discovery".to_string(),
                            purpose: "Network range identification".to_string(),
                            default_options: HashMap::from([
                                ("LOOKUP_PREFIXES".to_string(), "true".to_string()),
                            ]),
                            required: true,
                        },
                        TemplateModule {
                            path: "recon/subdomain_enum".to_string(),
                            name: "Subdomain Enumeration".to_string(),
                            purpose: "Subdomain discovery".to_string(),
                            default_options: HashMap::from([
                                ("PROBE_HTTP".to_string(), "true".to_string()),
                            ]),
                            required: true,
                        },
                    ],
                },
            ],
        }
    }

    /// Create quick scan template
    pub fn quick_scan() -> Self {
        Self {
            template_type: TemplateType::QuickScan,
            name: "Quick Scan".to_string(),
            description: "Fast port and HTTP scan".to_string(),
            recommended_target_type: TargetType::SingleHost,
            default_intensity: IntensityLevel::Aggressive,
            phases: vec![
                TemplatePhase {
                    name: "Quick Enumeration".to_string(),
                    phase_number: 1,
                    modules: vec![
                        TemplateModule {
                            path: "scanner/port_scanner".to_string(),
                            name: "Port Scanner".to_string(),
                            purpose: "Common ports".to_string(),
                            default_options: HashMap::from([
                                ("PORTS".to_string(), "21,22,23,25,53,80,110,143,443,445,993,995,3306,3389,5432,8080".to_string()),
                            ]),
                            required: true,
                        },
                        TemplateModule {
                            path: "scanner/http_scanner".to_string(),
                            name: "HTTP Scanner".to_string(),
                            purpose: "HTTP fingerprinting".to_string(),
                            default_options: HashMap::new(),
                            required: true,
                        },
                    ],
                },
            ],
        }
    }

    /// Get template by type
    pub fn from_type(template_type: TemplateType) -> Self {
        match template_type {
            TemplateType::WebApp => Self::web_app(),
            TemplateType::Network => Self::network(),
            TemplateType::Domain => Self::domain(),
            TemplateType::QuickScan => Self::quick_scan(),
        }
    }

    /// Convert template to selected modules
    pub fn to_selected_modules(&self, target: &str, intensity: IntensityLevel) -> Vec<SelectedModule> {
        let mut modules = Vec::new();

        for phase in &self.phases {
            for template_mod in &phase.modules {
                let mut selected = SelectedModule::new(
                    &template_mod.path,
                    &template_mod.name,
                    phase.phase_number,
                );

                // Apply default options
                for (key, value) in &template_mod.default_options {
                    selected.options.insert(key.clone(), value.clone());
                }

                // Apply target
                if template_mod.path.starts_with("scanner/") {
                    selected.options.insert("RHOSTS".to_string(), target.to_string());
                } else if template_mod.path.starts_with("recon/") {
                    if template_mod.path.contains("subdomain") || template_mod.path.contains("dns") {
                        // These use RHOSTS or TARGET depending on module
                        if template_mod.path.contains("subdomain") {
                            selected.options.insert("RHOSTS".to_string(), target.to_string());
                        } else {
                            selected.options.insert("TARGET".to_string(), target.to_string());
                        }
                    } else {
                        selected.options.insert("TARGET".to_string(), target.to_string());
                    }
                }

                // Apply intensity settings
                selected.options.insert("THREADS".to_string(), intensity.threads().to_string());
                selected.options.insert("TIMEOUT".to_string(), intensity.timeout_ms().to_string());

                selected.enabled = template_mod.required;
                modules.push(selected);
            }
        }

        modules
    }

    /// Get all available templates
    pub fn all() -> Vec<Self> {
        vec![
            Self::web_app(),
            Self::network(),
            Self::domain(),
            Self::quick_scan(),
        ]
    }
}
