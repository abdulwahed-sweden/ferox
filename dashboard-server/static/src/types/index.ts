// Session types
export type PrivilegeLevel = 'user' | 'administrator' | 'system' | 'root';
export type SessionStatus = 'active' | 'sleeping' | 'dead';
export type OsType = 'windows' | 'linux' | 'macos' | 'unknown';
export type Architecture = 'x64' | 'x86' | 'arm64' | 'unknown';

export interface SessionIntelligence {
  domain: string | null;
  is_domain_joined: boolean;
  detected_av: string[];
  stealth_mode: string;
  network_segment: string | null;
}

export interface SessionMetrics {
  credentials_count: number;
  commands_executed: number;
  files_transferred: number;
  persistence_methods: number;
}

export interface Session {
  id: string;
  hostname: string;
  ip_address: string;
  os: OsType;
  os_version: string | null;
  architecture: Architecture;
  username: string;
  privileges: PrivilegeLevel;
  status: SessionStatus;
  established_at: string;
  last_seen: string;
  intelligence: SessionIntelligence;
  metrics: SessionMetrics;
  tags: string[];
}

// Command types
export interface Command {
  id: string;
  session_id: string;
  command: string;
  output: string;
  timestamp: string;
  completed_at: string | null;
  success: boolean;
  execution_time_ms: number | null;
}

// Credential types
export type CredentialType =
  | 'plain_text'
  | 'ntlm_hash'
  | 'kerberos_ticket'
  | 'ssh_key'
  | 'cloud_credential'
  | 'token'
  | 'certificate';

export type Sensitivity = 'low' | 'medium' | 'high' | 'critical';

export interface Credential {
  id: string;
  cred_type: CredentialType;
  username: string;
  domain: string | null;
  secret: string;
  source_hostname: string;
  source_session_id: string;
  sensitivity: Sensitivity;
  collected_at: string;
  is_reusable: boolean;
  notes: string | null;
}

// Network types
export interface NetworkHost {
  id: string;
  hostname: string | null;
  ip_address: string;
  os: OsType | null;
  services: string[];
  ports: number[];
  is_compromised: boolean;
  session_id: string | null;
  credentials_available: number;
  is_domain_controller: boolean;
  is_high_value: boolean;
  discovered_at: string;
}

export interface NetworkEdge {
  source_id: string;
  target_id: string;
  protocol: string;
  port: number;
  can_pivot: boolean;
}

// MITRE types
export interface MitreTechniqueUsage {
  technique_id: string;
  technique_name: string;
  tactic: string;
  times_used: number;
  success_rate: number;
  detection_risk: string;
  last_used: string | null;
  sessions_used: string[];
}

export interface MitreCoverage {
  total_techniques: number;
  techniques_used: number;
  coverage_percentage: number;
  tactics_covered: string[];
  techniques: MitreTechniqueUsage[];
}

// Dashboard stats
export interface DashboardStats {
  active_sessions: number;
  total_sessions: number;
  credentials_collected: number;
  targets_discovered: number;
  mitre_coverage: number;
}

// API response wrapper
export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}

export interface SessionListResponse {
  sessions: Session[];
  total: number;
  active_count: number;
}

// WebSocket events
export type ServerEventType =
  | 'SessionCreated'
  | 'SessionUpdated'
  | 'SessionClosed'
  | 'CommandOutput'
  | 'CredentialsFound'
  | 'OpsecAlert'
  | 'Connected'
  | 'Pong'
  | 'Error';

export interface ServerEvent {
  type: ServerEventType;
  data: unknown;
}

export interface CommandOutputEvent {
  command_id: string;
  session_id: string;
  output: string;
  is_complete: boolean;
  success: boolean | null;
}

export interface OpsecAlertEvent {
  session_id: string;
  level: string;
  message: string;
  recommendation: string;
}

// Client events
export interface ExecuteCommandEvent {
  type: 'ExecuteCommand';
  data: {
    session_id: string;
    command: string;
  };
}

export interface SubscribeEvent {
  type: 'SubscribeToSession';
  data: {
    session_id: string;
  };
}

// ============================================================================
// Module API Types
// ============================================================================

// Privilege Escalation
export interface PrivEscRequest {
  session_id: string;
  auto_escalate: boolean;
  safe_mode: boolean;
}

export interface PrivEscVector {
  id: string;
  name: string;
  description: string;
  category: string;
  severity: string;
  confidence: number;
  mitre_id: string;
  exploitable: boolean;
}

export interface PrivEscResult {
  session_id: string;
  current_privilege: string;
  vectors_found: PrivEscVector[];
  escalation_attempted: boolean;
  escalation_success: boolean;
  new_privilege: string | null;
  output: string;
}

// Credential Harvesting
export interface CredentialHarvestRequest {
  session_id: string;
  sources: string[];
  safe_mode: boolean;
}

export interface HarvestedCred {
  id: string;
  cred_type: string;
  username: string;
  domain: string | null;
  secret: string;
  source: string;
  sensitivity: string;
  is_reusable: boolean;
}

export interface CredentialHarvestResult {
  session_id: string;
  credentials: HarvestedCred[];
  total_found: number;
  by_type: Record<string, number>;
  output: string;
}

// Persistence
export interface PersistenceRequest {
  session_id: string;
  method: string;
  name: string;
  safe_mode: boolean;
}

export interface PersistenceHandle {
  id: string;
  method: string;
  name: string;
  location: string;
  status: string;
  mitre_id: string;
}

export interface PersistenceResult {
  session_id: string;
  success: boolean;
  handles: PersistenceHandle[];
  output: string;
}

// Lateral Movement
export interface LateralMoveRequest {
  session_id: string;
  target_host: string;
  method: string;
  credential_id?: string;
  safe_mode: boolean;
}

export interface LateralMoveResult {
  session_id: string;
  target_host: string;
  success: boolean;
  new_session_id: string | null;
  method_used: string;
  output: string;
}

// Network Discovery
export interface DiscoveredHost {
  ip: string;
  hostname: string | null;
  os: string | null;
  open_ports: number[];
  services: string[];
}

export interface DiscoveryResult {
  session_id: string;
  hosts: DiscoveredHost[];
  subnets_scanned: string[];
  output: string;
}
