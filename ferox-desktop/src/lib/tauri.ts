// Tauri API bindings
import { invoke } from '@tauri-apps/api/core';
import type {
  Session,
  SessionTreeNode,
  HistoryEntry,
  CommandResult,
  PrivEscResult,
  CredentialHarvestResult,
  PayloadConfig,
  SimulatedPayload,
  PayloadTypeInfo,
  FormatInfo,
} from '../types';

// Session commands
export async function getSessions(): Promise<{
  sessions: Session[];
  total: number;
  active_count: number;
}> {
  return invoke('get_sessions');
}

export async function getSession(id: string): Promise<Session> {
  return invoke('get_session', { id });
}

export async function createSession(request: {
  hostname: string;
  ip_address: string;
  os: string;
  username: string;
  privileges: string;
  parent_id?: string;
}): Promise<Session> {
  return invoke('create_session', { request });
}

export async function terminateSession(id: string): Promise<void> {
  return invoke('terminate_session', { id });
}

export async function updateSessionNote(
  id: string,
  note: string | null
): Promise<void> {
  return invoke('update_session_note', { id, note });
}

export async function getSessionTree(): Promise<SessionTreeNode[]> {
  return invoke('get_session_tree');
}

// Terminal commands
export async function createTerminal(request: {
  session_id: string;
  rows?: number;
  cols?: number;
  shell?: string;
}): Promise<{ terminal_id: string; session_id: string }> {
  return invoke('create_terminal', { request });
}

export async function writeTerminal(
  terminal_id: string,
  data: string
): Promise<void> {
  return invoke('write_terminal', { request: { terminal_id, data } });
}

export async function resizeTerminal(
  terminal_id: string,
  rows: number,
  cols: number
): Promise<void> {
  return invoke('resize_terminal', { request: { terminal_id, rows, cols } });
}

export async function closeTerminal(terminal_id: string): Promise<void> {
  return invoke('close_terminal', { terminalId: terminal_id });
}

export async function getTerminalHistory(
  terminal_id: string
): Promise<HistoryEntry[]> {
  return invoke('get_terminal_history', { terminalId: terminal_id });
}

// Module commands
export async function executeCommand(
  session_id: string,
  command: string
): Promise<CommandResult> {
  return invoke('execute_command', { request: { session_id, command } });
}

export async function runPrivesc(request: {
  session_id: string;
  auto_escalate: boolean;
  safe_mode: boolean;
}): Promise<PrivEscResult> {
  return invoke('run_privesc', { request });
}

export async function harvestCredentials(request: {
  session_id: string;
  sources: string[];
  safe_mode: boolean;
}): Promise<CredentialHarvestResult> {
  return invoke('harvest_credentials', { request });
}

export async function installPersistence(request: {
  session_id: string;
  method: string;
  name: string;
  safe_mode: boolean;
}): Promise<{ success: boolean; output: string }> {
  return invoke('install_persistence', { request });
}

export async function lateralMove(request: {
  session_id: string;
  target_host: string;
  method: string;
  credential_id?: string;
  safe_mode: boolean;
}): Promise<{ success: boolean; new_session_id?: string; output: string }> {
  return invoke('lateral_move', { request });
}

export async function networkDiscovery(request: {
  session_id: string;
  subnet?: string;
  ports?: number[];
}): Promise<{ hosts: unknown[]; output: string }> {
  return invoke('network_discovery', { request });
}

// ============================================================================
// Simulated Payload Commands (for demo/training)
// ============================================================================

export async function generateSimulatedPayload(
  config: PayloadConfig
): Promise<SimulatedPayload> {
  return invoke('generate_simulated_payload', { config });
}

export async function getPayloadTypes(): Promise<PayloadTypeInfo[]> {
  return invoke('get_payload_types');
}

export async function getPayloadFormats(): Promise<FormatInfo[]> {
  return invoke('get_payload_formats');
}

// ============================================================================
// Simulation Telemetry Commands
// ============================================================================

import type {
  NetworkScanResult,
  CredentialDumpResult,
  SimulatedLogEntry,
  SimulatedTask,
  SimulatedNote,
  DirectoryListing,
  ProcessListResult,
} from '../types';

export async function simulateNetworkScan(
  subnet: string,
  sessionId: string
): Promise<NetworkScanResult> {
  return invoke('simulate_network_scan', { subnet, sessionId });
}

export async function simulateCredentialDump(
  sessionId: string,
  sources: string[] = []
): Promise<CredentialDumpResult> {
  return invoke('simulate_credential_dump', { sessionId, sources });
}

export async function simulateEventLog(
  count?: number
): Promise<SimulatedLogEntry[]> {
  return invoke('simulate_event_log', { count });
}

export async function simulateScheduledTasks(
  sessionId: string
): Promise<SimulatedTask[]> {
  return invoke('simulate_scheduled_tasks', { sessionId });
}

export async function simulateSessionNotes(
  sessionId: string
): Promise<SimulatedNote[]> {
  return invoke('simulate_session_notes', { sessionId });
}

export async function simulateDirectoryListing(
  path: string,
  sessionId: string
): Promise<DirectoryListing> {
  return invoke('simulate_directory_listing', { path, sessionId });
}

export async function simulateProcessList(
  sessionId: string
): Promise<ProcessListResult> {
  return invoke('simulate_process_list', { sessionId });
}
