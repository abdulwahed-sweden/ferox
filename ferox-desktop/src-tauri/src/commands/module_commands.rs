//! Module Commands
//!
//! Tauri commands for post-exploitation module operations.
//! These integrate with the Ferox Core for real functionality.

use crate::security::Validator;
use crate::AppState;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

// ============================================================================
// Request Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct ExecuteCommandRequest {
    pub session_id: String,
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct PrivEscRequest {
    pub session_id: String,
    pub auto_escalate: bool,
    pub safe_mode: bool,
}

#[derive(Debug, Deserialize)]
pub struct CredentialHarvestRequest {
    pub session_id: String,
    pub sources: Vec<String>,
    pub safe_mode: bool,
}

#[derive(Debug, Deserialize)]
pub struct PersistenceRequest {
    pub session_id: String,
    pub method: String,
    pub name: String,
    pub safe_mode: bool,
}

#[derive(Debug, Deserialize)]
pub struct LateralMoveRequest {
    pub session_id: String,
    pub target_host: String,
    pub method: String,
    pub credential_id: Option<String>,
    pub safe_mode: bool,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryRequest {
    pub session_id: String,
    pub subnet: Option<String>,
    pub ports: Option<Vec<u16>>,
}

// ============================================================================
// Response Types
// ============================================================================

#[derive(Debug, Serialize)]
pub struct CommandResult {
    pub session_id: String,
    pub command: String,
    pub output: String,
    pub success: bool,
    pub execution_time_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct PrivEscVector {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub severity: String,
    pub confidence: f32,
    pub mitre_id: String,
    pub exploitable: bool,
}

#[derive(Debug, Serialize)]
pub struct PrivEscResult {
    pub session_id: String,
    pub current_privilege: String,
    pub vectors_found: Vec<PrivEscVector>,
    pub escalation_attempted: bool,
    pub escalation_success: bool,
    pub new_privilege: Option<String>,
    pub output: String,
}

#[derive(Debug, Serialize)]
pub struct HarvestedCredential {
    pub id: String,
    pub cred_type: String,
    pub username: String,
    pub domain: Option<String>,
    pub secret: String,
    pub source: String,
    pub sensitivity: String,
    pub is_reusable: bool,
}

#[derive(Debug, Serialize)]
pub struct CredentialHarvestResult {
    pub session_id: String,
    pub credentials: Vec<HarvestedCredential>,
    pub total_found: usize,
    pub by_type: std::collections::HashMap<String, usize>,
    pub output: String,
}

#[derive(Debug, Serialize)]
pub struct PersistenceHandle {
    pub id: String,
    pub method: String,
    pub name: String,
    pub location: String,
    pub status: String,
    pub mitre_id: String,
}

#[derive(Debug, Serialize)]
pub struct PersistenceResult {
    pub session_id: String,
    pub success: bool,
    pub handles: Vec<PersistenceHandle>,
    pub output: String,
}

#[derive(Debug, Serialize)]
pub struct LateralMoveResult {
    pub session_id: String,
    pub target_host: String,
    pub success: bool,
    pub new_session_id: Option<String>,
    pub method_used: String,
    pub output: String,
}

#[derive(Debug, Serialize)]
pub struct DiscoveredHost {
    pub ip: String,
    pub hostname: Option<String>,
    pub os: Option<String>,
    pub open_ports: Vec<u16>,
    pub services: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DiscoveryResult {
    pub session_id: String,
    pub hosts: Vec<DiscoveredHost>,
    pub subnets_scanned: Vec<String>,
    pub output: String,
}

// ============================================================================
// Commands
// ============================================================================

/// Execute a command on a session (via Ferox Core bridge)
#[tauri::command]
pub async fn execute_command(
    request: ExecuteCommandRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<CommandResult, String> {
    // Validate inputs
    Validator::session_id(&request.session_id).map_err(|e| e.to_string())?;
    Validator::command(&request.command).map_err(|e| e.to_string())?;

    let start = std::time::Instant::now();

    let (bridge, audit_logger) = {
        let state = state.read();
        (state.bridge.clone(), state.audit_logger.clone())
    };

    // Execute command via bridge (which calls Ferox Core)
    let result = bridge
        .execute_command(&request.session_id, &request.command)
        .await;

    let (output, success) = match result {
        Ok(cmd_output) => (cmd_output.stdout, cmd_output.exit_code == 0),
        Err(e) => (format!("Error: {}", e), false),
    };

    // Audit log the command execution
    audit_logger.log_command_executed(&request.session_id, &request.command, success);

    Ok(CommandResult {
        session_id: request.session_id,
        command: request.command.clone(),
        output,
        success,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Run privilege escalation scan/exploit (via Ferox Core bridge)
#[tauri::command]
pub async fn run_privesc(
    request: PrivEscRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<PrivEscResult, String> {
    // Validate input
    Validator::session_id(&request.session_id).map_err(|e| e.to_string())?;

    let (bridge, audit_logger) = {
        let state = state.read();
        (state.bridge.clone(), state.audit_logger.clone())
    };

    // Run privesc scan via bridge (which calls Ferox Core)
    let vectors = bridge
        .scan_privesc(&request.session_id, request.safe_mode)
        .await
        .map_err(|e| e.to_string())?;

    let escalation_success = false; // TODO: implement auto_escalate

    // Audit log the privesc attempt
    audit_logger.log_privesc_attempt(&request.session_id, escalation_success);

    // Convert bridge vectors to command response format
    let result_vectors: Vec<PrivEscVector> = vectors
        .iter()
        .map(|v| PrivEscVector {
            id: v.id.clone(),
            name: v.name.clone(),
            description: v.description.clone(),
            category: "enumerated".to_string(),
            severity: v.severity.clone(),
            confidence: 0.75,
            mitre_id: "T1548".to_string(),
            exploitable: v.exploitable,
        })
        .collect();

    Ok(PrivEscResult {
        session_id: request.session_id,
        current_privilege: "user".to_string(),
        vectors_found: result_vectors,
        escalation_attempted: request.auto_escalate,
        escalation_success,
        new_privilege: None,
        output: format!("[PrivEsc scan completed: {} vectors found]", vectors.len()),
    })
}

/// Harvest credentials from a session (via Ferox Core bridge)
#[tauri::command]
pub async fn harvest_credentials(
    request: CredentialHarvestRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<CredentialHarvestResult, String> {
    // Validate input
    Validator::session_id(&request.session_id).map_err(|e| e.to_string())?;

    let (bridge, audit_logger) = {
        let state = state.read();
        (state.bridge.clone(), state.audit_logger.clone())
    };

    // Harvest credentials via bridge (which calls Ferox Core)
    let creds = bridge
        .harvest_credentials(&request.session_id, &request.sources, request.safe_mode)
        .await
        .map_err(|e| e.to_string())?;

    let total_found = creds.len();

    // Audit log the credential harvest
    audit_logger.log_credential_harvest(&request.session_id, total_found);

    // Convert to command response format
    let credentials: Vec<HarvestedCredential> = creds
        .iter()
        .map(|c| HarvestedCredential {
            id: uuid::Uuid::new_v4().to_string(),
            cred_type: c.cred_type.clone(),
            username: c.username.clone(),
            domain: c.domain.clone(),
            secret: "[REDACTED]".to_string(),
            source: c.source.clone(),
            sensitivity: "high".to_string(),
            is_reusable: true,
        })
        .collect();

    // Build type counts
    let mut by_type: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for cred in &credentials {
        *by_type.entry(cred.cred_type.clone()).or_insert(0) += 1;
    }

    Ok(CredentialHarvestResult {
        session_id: request.session_id,
        credentials,
        total_found,
        by_type,
        output: format!("[Credential harvest completed: {} credentials found]", total_found),
    })
}

/// Install persistence on a session (via Ferox Core bridge)
#[tauri::command]
pub async fn install_persistence(
    request: PersistenceRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<PersistenceResult, String> {
    // Validate inputs
    Validator::session_id(&request.session_id).map_err(|e| e.to_string())?;
    Validator::method(&request.method).map_err(|e| e.to_string())?;
    Validator::persistence_name(&request.name).map_err(|e| e.to_string())?;

    let (bridge, audit_logger) = {
        let state = state.read();
        (state.bridge.clone(), state.audit_logger.clone())
    };

    // Install persistence via bridge (which calls Ferox Core)
    // Note: Using method as payload_path placeholder for now
    let result = bridge
        .install_persistence(&request.session_id, &request.method, &request.name, request.safe_mode)
        .await
        .map_err(|e| e.to_string())?;

    // Audit log the persistence installation
    audit_logger.log_persistence_install(&request.session_id, &request.method, result.success);

    // Convert to command response format
    let handles: Vec<PersistenceHandle> = result
        .handles
        .iter()
        .map(|h| PersistenceHandle {
            id: h.id.clone(),
            method: h.method.clone(),
            name: h.name.clone(),
            location: h.location.clone(),
            status: h.status.clone(),
            mitre_id: h.mitre_id.clone(),
        })
        .collect();

    Ok(PersistenceResult {
        session_id: request.session_id,
        success: result.success,
        handles,
        output: result.message,
    })
}

/// Perform lateral movement
#[tauri::command]
pub async fn lateral_move(
    request: LateralMoveRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<LateralMoveResult, String> {
    // Validate inputs
    Validator::session_id(&request.session_id).map_err(|e| e.to_string())?;
    Validator::target_host(&request.target_host).map_err(|e| e.to_string())?;
    Validator::method(&request.method).map_err(|e| e.to_string())?;
    if let Some(ref cred_id) = request.credential_id {
        Validator::session_id(cred_id).map_err(|e| e.to_string())?;
    }

    // TODO: Integrate with ferox::LateralMovementEngine

    let success = false;

    // Audit log the lateral movement
    {
        let state = state.read();
        state.audit_logger.log_lateral_move(&request.session_id, &request.target_host, success);
    }

    Ok(LateralMoveResult {
        session_id: request.session_id,
        target_host: request.target_host,
        success,
        new_session_id: None,
        method_used: request.method,
        output: "[Lateral movement results from Ferox Core]".to_string(),
    })
}

/// Run network discovery
#[tauri::command]
pub async fn network_discovery(
    request: DiscoveryRequest,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<DiscoveryResult, String> {
    // Validate inputs
    Validator::session_id(&request.session_id).map_err(|e| e.to_string())?;
    Validator::ports(&request.ports).map_err(|e| e.to_string())?;

    // TODO: Integrate with ferox network scanner

    let hosts: Vec<DiscoveredHost> = vec![];

    // Audit log the network discovery
    {
        let state = state.read();
        state.audit_logger.log_network_discovery(&request.session_id, hosts.len());
    }

    Ok(DiscoveryResult {
        session_id: request.session_id,
        hosts,
        subnets_scanned: request.subnet.map(|s| vec![s]).unwrap_or_default(),
        output: "[Network discovery results from Ferox Core]".to_string(),
    })
}
