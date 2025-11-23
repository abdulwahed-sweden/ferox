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
  parent_id: string | null;
  intelligence: SessionIntelligence;
  metrics: SessionMetrics;
  tags: string[];
  note: string | null;
}

export interface SessionTreeNode {
  session: Session;
  children: SessionTreeNode[];
}

// Terminal types
export interface TerminalConfig {
  rows: number;
  cols: number;
  shell?: string;
}

export interface HistoryEntry {
  command: string;
  output: string;
  timestamp: number;
  success: boolean;
}

// Tab types
export type TabType =
  | 'terminal'
  | 'filebrowser'
  | 'processes'
  | 'network'
  | 'payloads'
  | 'scanner'
  | 'credentials'
  | 'eventlog'
  | 'scheduler'
  | 'notes';

export interface Tab {
  id: string;
  type: TabType;
  sessionId: string;
  terminalId?: string;
  title: string;
  icon: string;
}

// Context menu
export interface ContextMenuItem {
  id: string;
  label: string;
  icon?: string;
  shortcut?: string;
  disabled?: boolean;
  danger?: boolean;
  onClick?: () => void;
}

export interface ContextMenuSection {
  label: string;
  items: ContextMenuItem[];
}

// Command result
export interface CommandResult {
  session_id: string;
  command: string;
  output: string;
  success: boolean;
  execution_time_ms: number;
}

// Module results
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

export interface HarvestedCredential {
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
  credentials: HarvestedCredential[];
  total_found: number;
  by_type: Record<string, number>;
  output: string;
}

// ============================================================================
// Simulated Payload Types (for demo/training)
// ============================================================================

export interface PayloadConfig {
  payload_type: string;
  lhost: string;
  lport: number;
  target_os: string;
  format: string;
  architecture: string;
  obfuscation: boolean;
  signing: boolean;
  staged: boolean;
  name?: string;
}

export interface BuildLogEntry {
  timestamp: string;
  level: 'info' | 'warn' | 'success';
  message: string;
}

export interface RiskFactor {
  name: string;
  score: number;
  description: string;
}

export interface RiskAnalysis {
  risk_score: number;
  risk_level: 'low' | 'medium' | 'high' | 'critical';
  factors: RiskFactor[];
  recommendations: string[];
}

export interface DetectionAnalysis {
  estimated_detection_rate: number;
  likely_detectors: string[];
  behavioral_indicators: string[];
  network_indicators: string[];
  evasion_notes: string[];
}

export interface MitreMapping {
  technique_id: string;
  technique_name: string;
  tactic: string;
  description: string;
}

export interface ExecutionHint {
  name: string;
  command: string;
  description: string;
  os: string;
}

export interface SimulatedPayload {
  id: string;
  name: string;
  config: PayloadConfig;
  simulated_path: string;
  simulated_size_bytes: number;
  simulated_hash: string;
  created_at: string;
  build_log: BuildLogEntry[];
  risk_analysis: RiskAnalysis;
  detection_analysis: DetectionAnalysis;
  mitre_mapping: MitreMapping[];
  execution_hints: ExecutionHint[];
}

export interface PayloadTypeInfo {
  id: string;
  name: string;
  description: string;
  category: string;
  risk_level: string;
}

export interface FormatInfo {
  id: string;
  name: string;
  extension: string;
  os: string[];
  description: string;
}
