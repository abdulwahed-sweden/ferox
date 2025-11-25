//! Network Commands - Network topology and host discovery
//!
//! Provides network mapping and topology visualization data.

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, TcpStream};
use std::time::Duration;
use tauri::command;

// =============================================================================
// Types
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub id: String,
    pub label: String,
    pub node_type: String,  // attacker, c2, target, compromised, pivot, exfil, router, firewall
    pub ip: String,
    pub hostname: Option<String>,
    pub os: Option<String>,
    pub status: String,     // active, inactive, unknown
    pub sessions: u32,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub id: String,
    pub source: String,
    pub target: String,
    pub connection_type: String,  // control, data, exfil, lateral, recon
    pub protocol: String,
    pub port: u16,
    pub encrypted: bool,
    pub active: bool,
    pub bandwidth: u64,     // bytes/sec
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    pub nodes: Vec<NetworkNode>,
    pub connections: Vec<NetworkConnection>,
    pub last_updated: String,
}

// =============================================================================
// Global Topology State
// =============================================================================

lazy_static::lazy_static! {
    static ref TOPOLOGY: std::sync::RwLock<NetworkTopology> = {
        std::sync::RwLock::new(NetworkTopology {
            nodes: vec![
                NetworkNode {
                    id: "attacker".to_string(),
                    label: "Attacker".to_string(),
                    node_type: "attacker".to_string(),
                    ip: get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string()),
                    hostname: Some(hostname::get().map(|h| h.to_string_lossy().to_string()).unwrap_or_else(|_| "localhost".to_string())),
                    os: Some(std::env::consts::OS.to_string()),
                    status: "active".to_string(),
                    sessions: 0,
                    x: Some(100.0),
                    y: Some(300.0),
                },
            ],
            connections: Vec::new(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    };
}

/// Get local IP address
fn get_local_ip() -> Option<String> {
    // Try to get local IP by connecting to an external address
    if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                return Some(addr.ip().to_string());
            }
        }
    }
    None
}

// =============================================================================
// Commands
// =============================================================================

/// Get current network topology
#[command]
pub async fn get_topology() -> Result<NetworkTopology, String> {
    let topology = TOPOLOGY.read().unwrap();
    Ok(topology.clone())
}

/// Discover hosts on a network
#[command]
pub async fn discover_hosts(cidr: String) -> Result<Vec<NetworkNode>, String> {
    let mut discovered = Vec::new();

    // Parse CIDR to get IP range
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.is_empty() {
        return Err("Invalid CIDR format".to_string());
    }

    let base_ip = parts[0];
    let prefix: u8 = parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(24);

    // Only support /24 for now
    if prefix != 24 {
        return Err("Only /24 networks are currently supported".to_string());
    }

    let octets: Vec<&str> = base_ip.split('.').collect();
    if octets.len() != 4 {
        return Err("Invalid IP address format".to_string());
    }

    let base = format!("{}.{}.{}.", octets[0], octets[1], octets[2]);

    // Scan first 20 hosts for demo (would scan all 254 in production)
    for i in 1..=20 {
        let ip = format!("{}{}", base, i);

        // Quick TCP connect check on common ports
        let is_up = check_host_alive(&ip);

        if is_up {
            let node = NetworkNode {
                id: format!("host-{}", i),
                label: format!("Host {}", i),
                node_type: "target".to_string(),
                ip: ip.clone(),
                hostname: resolve_hostname(&ip),
                os: guess_os_from_ports(&ip),
                status: "active".to_string(),
                sessions: 0,
                x: None,
                y: None,
            };
            discovered.push(node);
        }
    }

    // Update global topology with discovered nodes
    {
        let mut topology = TOPOLOGY.write().unwrap();
        for node in &discovered {
            if !topology.nodes.iter().any(|n| n.ip == node.ip) {
                topology.nodes.push(node.clone());
            }
        }
        topology.last_updated = chrono::Utc::now().to_rfc3339();
    }

    Ok(discovered)
}

/// Check if a host is alive using TCP connect
fn check_host_alive(ip: &str) -> bool {
    let common_ports = [22, 80, 443, 445, 3389, 8080];

    for port in common_ports {
        let addr = format!("{}:{}", ip, port);
        if let Ok(addr) = addr.parse() {
            if TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok() {
                return true;
            }
        }
    }

    // Also try ICMP-like check (TCP SYN to port 7)
    let addr = format!("{}:7", ip);
    if let Ok(addr) = addr.parse() {
        // Connection refused also means host is up
        match TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
            Ok(_) => return true,
            Err(e) if e.kind() == std::io::ErrorKind::ConnectionRefused => return true,
            _ => {}
        }
    }

    false
}

/// Try to resolve hostname from IP
fn resolve_hostname(ip: &str) -> Option<String> {
    // Reverse DNS lookup
    if ip.parse::<IpAddr>().is_ok() {
        // Would use DNS PTR lookup in real implementation
        // For now, return None
        None
    } else {
        None
    }
}

/// Guess OS from open ports (very simplified)
fn guess_os_from_ports(ip: &str) -> Option<String> {
    let windows_ports = [135, 139, 445, 3389];
    let linux_ports = [22];

    for port in windows_ports {
        let addr = format!("{}:{}", ip, port);
        if let Ok(addr) = addr.parse() {
            if TcpStream::connect_timeout(&addr, Duration::from_millis(100)).is_ok() {
                return Some("Windows".to_string());
            }
        }
    }

    for port in linux_ports {
        let addr = format!("{}:{}", ip, port);
        if let Ok(addr) = addr.parse() {
            if TcpStream::connect_timeout(&addr, Duration::from_millis(100)).is_ok() {
                return Some("Linux".to_string());
            }
        }
    }

    None
}

/// Add a node to the topology
#[command]
pub async fn add_topology_node(node: NetworkNode) -> Result<bool, String> {
    let mut topology = TOPOLOGY.write().unwrap();

    if topology.nodes.iter().any(|n| n.id == node.id) {
        return Err(format!("Node {} already exists", node.id));
    }

    topology.nodes.push(node);
    topology.last_updated = chrono::Utc::now().to_rfc3339();

    Ok(true)
}

/// Add a connection to the topology
#[command]
pub async fn add_topology_connection(connection: NetworkConnection) -> Result<bool, String> {
    let mut topology = TOPOLOGY.write().unwrap();

    // Verify source and target exist
    if !topology.nodes.iter().any(|n| n.id == connection.source) {
        return Err(format!("Source node {} not found", connection.source));
    }
    if !topology.nodes.iter().any(|n| n.id == connection.target) {
        return Err(format!("Target node {} not found", connection.target));
    }

    topology.connections.push(connection);
    topology.last_updated = chrono::Utc::now().to_rfc3339();

    Ok(true)
}

/// Remove a node from the topology
#[command]
pub async fn remove_topology_node(node_id: String) -> Result<bool, String> {
    let mut topology = TOPOLOGY.write().unwrap();

    let initial_len = topology.nodes.len();
    topology.nodes.retain(|n| n.id != node_id);

    // Also remove connections involving this node
    topology.connections.retain(|c| c.source != node_id && c.target != node_id);

    if topology.nodes.len() < initial_len {
        topology.last_updated = chrono::Utc::now().to_rfc3339();
        Ok(true)
    } else {
        Err(format!("Node {} not found", node_id))
    }
}

/// Update node status
#[command]
pub async fn update_node_status(node_id: String, status: String, sessions: Option<u32>) -> Result<bool, String> {
    let mut topology = TOPOLOGY.write().unwrap();

    if let Some(node) = topology.nodes.iter_mut().find(|n| n.id == node_id) {
        node.status = status;
        if let Some(s) = sessions {
            node.sessions = s;
        }
        topology.last_updated = chrono::Utc::now().to_rfc3339();
        Ok(true)
    } else {
        Err(format!("Node {} not found", node_id))
    }
}

/// Mark a host as compromised
#[command]
pub async fn mark_compromised(node_id: String) -> Result<bool, String> {
    let mut topology = TOPOLOGY.write().unwrap();

    if let Some(node) = topology.nodes.iter_mut().find(|n| n.id == node_id) {
        node.node_type = "compromised".to_string();
        node.sessions = node.sessions.saturating_add(1);
        topology.last_updated = chrono::Utc::now().to_rfc3339();
        Ok(true)
    } else {
        Err(format!("Node {} not found", node_id))
    }
}
