// Tauri API bindings with timeout, retry, and validation support
import { invoke } from "@tauri-apps/api/core";
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
} from "../types";

// ============================================================================
// Timeout & Retry Configuration
// ============================================================================

export interface InvokeOptions {
  timeout?: number; // Timeout in milliseconds (default: 30000)
  retries?: number; // Number of retries (default: 0)
  retryDelay?: number; // Delay between retries in ms (default: 1000)
  onTimeout?: () => void; // Callback when timeout occurs
  onRetry?: (attempt: number, error: Error) => void; // Callback on retry
}

// Default timeouts by operation type
export const TIMEOUTS = {
  QUICK: 5000, // Quick operations (get session, etc.)
  STANDARD: 30000, // Standard operations
  LONG: 60000, // Long operations (scans, discovery)
  VERY_LONG: 120000, // Very long operations
} as const;

export class TauriTimeoutError extends Error {
  constructor(command: string, timeout: number) {
    super(`Command "${command}" timed out after ${timeout}ms`);
    this.name = "TauriTimeoutError";
  }
}

export class TauriInvokeError extends Error {
  public readonly command: string;
  public readonly originalError: unknown;

  constructor(command: string, error: unknown) {
    const message =
      error instanceof Error ? error.message : String(error);
    super(`Command "${command}" failed: ${message}`);
    this.name = "TauriInvokeError";
    this.command = command;
    this.originalError = error;
  }
}

/**
 * Enhanced invoke with timeout and retry support
 */
export async function invokeWithTimeout<T>(
  cmd: string,
  args?: Record<string, unknown>,
  options: InvokeOptions = {}
): Promise<T> {
  const {
    timeout = TIMEOUTS.STANDARD,
    retries = 0,
    retryDelay = 1000,
    onTimeout,
    onRetry,
  } = options;

  let lastError: Error | null = null;

  for (let attempt = 0; attempt <= retries; attempt++) {
    try {
      const result = await Promise.race<T>([
        invoke<T>(cmd, args),
        new Promise<never>((_, reject) => {
          setTimeout(() => {
            onTimeout?.();
            reject(new TauriTimeoutError(cmd, timeout));
          }, timeout);
        }),
      ]);
      return result;
    } catch (error) {
      lastError =
        error instanceof Error ? error : new TauriInvokeError(cmd, error);

      // Don't retry on timeout errors
      if (error instanceof TauriTimeoutError) {
        throw error;
      }

      // If we have retries left, wait and try again
      if (attempt < retries) {
        onRetry?.(attempt + 1, lastError);
        await new Promise((resolve) => setTimeout(resolve, retryDelay));
      }
    }
  }

  throw lastError || new TauriInvokeError(cmd, "Unknown error");
}

// ============================================================================
// Connection State Management
// ============================================================================

let connectionState: "connected" | "disconnected" | "connecting" = "connected";
const connectionListeners: Set<(state: typeof connectionState) => void> =
  new Set();

export function getConnectionState() {
  return connectionState;
}

export function onConnectionStateChange(
  listener: (state: typeof connectionState) => void
): () => void {
  connectionListeners.add(listener);
  return () => {
    connectionListeners.delete(listener);
  };
}

function setConnectionState(state: typeof connectionState) {
  if (connectionState !== state) {
    connectionState = state;
    connectionListeners.forEach((listener) => listener(state));
  }
}

// Wrapper that tracks connection state
async function trackedInvoke<T>(
  cmd: string,
  args?: Record<string, unknown>,
  options?: InvokeOptions
): Promise<T> {
  try {
    const result = await invokeWithTimeout<T>(cmd, args, options);
    setConnectionState("connected");
    return result;
  } catch (error) {
    if (error instanceof TauriTimeoutError) {
      setConnectionState("disconnected");
    }
    throw error;
  }
}

// Session commands
export async function getSessions(): Promise<{
  sessions: Session[];
  total: number;
  active_count: number;
}> {
  return trackedInvoke("get_sessions", undefined, { timeout: TIMEOUTS.QUICK });
}

export async function getSession(id: string): Promise<Session> {
  return trackedInvoke("get_session", { id }, { timeout: TIMEOUTS.QUICK });
}

export async function createSession(request: {
  hostname: string;
  ip_address: string;
  os: string;
  username: string;
  privileges: string;
  parent_id?: string;
}): Promise<Session> {
  return trackedInvoke("create_session", { request }, { timeout: TIMEOUTS.STANDARD });
}

export async function terminateSession(id: string): Promise<void> {
  return trackedInvoke("terminate_session", { id }, { timeout: TIMEOUTS.STANDARD });
}

export async function updateSessionNote(
  id: string,
  note: string | null,
): Promise<void> {
  return trackedInvoke("update_session_note", { id, note }, { timeout: TIMEOUTS.QUICK });
}

export async function getSessionTree(): Promise<SessionTreeNode[]> {
  return trackedInvoke("get_session_tree", undefined, { timeout: TIMEOUTS.QUICK });
}

// Terminal commands
export async function createTerminal(request: {
  session_id: string;
  rows?: number;
  cols?: number;
  shell?: string;
}): Promise<{ terminal_id: string; session_id: string }> {
  return trackedInvoke("create_terminal", { request }, { timeout: TIMEOUTS.STANDARD });
}

export async function writeTerminal(
  terminal_id: string,
  data: string,
): Promise<void> {
  return trackedInvoke("write_terminal", { request: { terminal_id, data } }, { timeout: TIMEOUTS.QUICK });
}

export async function resizeTerminal(
  terminal_id: string,
  rows: number,
  cols: number,
): Promise<void> {
  return trackedInvoke("resize_terminal", { request: { terminal_id, rows, cols } }, { timeout: TIMEOUTS.QUICK });
}

export async function closeTerminal(terminal_id: string): Promise<void> {
  return trackedInvoke("close_terminal", { terminalId: terminal_id }, { timeout: TIMEOUTS.STANDARD });
}

export async function getTerminalHistory(
  terminal_id: string,
): Promise<HistoryEntry[]> {
  return trackedInvoke("get_terminal_history", { terminalId: terminal_id }, { timeout: TIMEOUTS.QUICK });
}

// Module commands
export async function executeCommand(
  session_id: string,
  command: string,
): Promise<CommandResult> {
  return trackedInvoke("execute_command", { request: { session_id, command } }, { timeout: TIMEOUTS.LONG });
}

export async function runPrivesc(request: {
  session_id: string;
  auto_escalate: boolean;
  safe_mode: boolean;
}): Promise<PrivEscResult> {
  return trackedInvoke("run_privesc", { request }, { timeout: TIMEOUTS.LONG });
}

export async function harvestCredentials(request: {
  session_id: string;
  sources: string[];
  safe_mode: boolean;
}): Promise<CredentialHarvestResult> {
  return trackedInvoke("harvest_credentials", { request }, { timeout: TIMEOUTS.LONG });
}

export async function installPersistence(request: {
  session_id: string;
  method: string;
  name: string;
  safe_mode: boolean;
}): Promise<{ success: boolean; output: string }> {
  return trackedInvoke("install_persistence", { request }, { timeout: TIMEOUTS.STANDARD });
}

export async function lateralMove(request: {
  session_id: string;
  target_host: string;
  method: string;
  credential_id?: string;
  safe_mode: boolean;
}): Promise<{ success: boolean; new_session_id?: string; output: string }> {
  return trackedInvoke("lateral_move", { request }, { timeout: TIMEOUTS.LONG });
}

export async function networkDiscovery(request: {
  session_id: string;
  subnet?: string;
  ports?: number[];
}): Promise<{ hosts: unknown[]; output: string }> {
  return trackedInvoke("network_discovery", { request }, { timeout: TIMEOUTS.VERY_LONG });
}

// ============================================================================
// Simulated Payload Commands (for demo/training)
// ============================================================================

export async function generateSimulatedPayload(
  config: PayloadConfig,
): Promise<SimulatedPayload> {
  return trackedInvoke("generate_simulated_payload", { config }, { timeout: TIMEOUTS.STANDARD });
}

export async function getPayloadTypes(): Promise<PayloadTypeInfo[]> {
  return trackedInvoke("get_payload_types", undefined, { timeout: TIMEOUTS.QUICK });
}

export async function getPayloadFormats(): Promise<FormatInfo[]> {
  return trackedInvoke("get_payload_formats", undefined, { timeout: TIMEOUTS.QUICK });
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
} from "../types";

export async function simulateNetworkScan(
  subnet: string,
  sessionId: string,
): Promise<NetworkScanResult> {
  return trackedInvoke("simulate_network_scan", { subnet, sessionId }, { timeout: TIMEOUTS.LONG });
}

export async function simulateCredentialDump(
  sessionId: string,
  sources: string[] = [],
): Promise<CredentialDumpResult> {
  return trackedInvoke("simulate_credential_dump", { sessionId, sources }, { timeout: TIMEOUTS.STANDARD });
}

export async function simulateEventLog(
  count?: number,
): Promise<SimulatedLogEntry[]> {
  return trackedInvoke("simulate_event_log", { count }, { timeout: TIMEOUTS.QUICK });
}

export async function simulateScheduledTasks(
  sessionId: string,
): Promise<SimulatedTask[]> {
  return trackedInvoke("simulate_scheduled_tasks", { sessionId }, { timeout: TIMEOUTS.QUICK });
}

export async function simulateSessionNotes(
  sessionId: string,
): Promise<SimulatedNote[]> {
  return trackedInvoke("simulate_session_notes", { sessionId }, { timeout: TIMEOUTS.QUICK });
}

export async function simulateDirectoryListing(
  path: string,
  sessionId: string,
): Promise<DirectoryListing> {
  return trackedInvoke("simulate_directory_listing", { path, sessionId }, { timeout: TIMEOUTS.QUICK });
}

export async function simulateProcessList(
  sessionId: string,
): Promise<ProcessListResult> {
  return trackedInvoke("simulate_process_list", { sessionId }, { timeout: TIMEOUTS.QUICK });
}
