import type {
  ApiResponse,
  Session,
  SessionListResponse,
  Command,
  Credential,
  DashboardStats,
  MitreCoverage,
  NetworkHost,
  NetworkEdge,
  // Module types
  PrivEscRequest,
  PrivEscResult,
  CredentialHarvestRequest,
  CredentialHarvestResult,
  PersistenceRequest,
  PersistenceResult,
  LateralMoveRequest,
  LateralMoveResult,
  DiscoveryResult,
} from '../types';

const API_BASE = '/api';

async function fetchApi<T>(endpoint: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });

  if (!response.ok) {
    throw new Error(`API error: ${response.status} ${response.statusText}`);
  }

  const data: ApiResponse<T> = await response.json();

  if (!data.success || data.data === null) {
    throw new Error(data.error || 'Unknown API error');
  }

  return data.data;
}

export function useApi() {
  return {
    // Sessions
    getSessions: () => fetchApi<SessionListResponse>('/sessions'),

    getSession: (id: string) => fetchApi<Session>(`/sessions/${id}`),

    executeCommand: (sessionId: string, command: string) =>
      fetchApi<Command>(`/sessions/${sessionId}/execute`, {
        method: 'POST',
        body: JSON.stringify({ command }),
      }),

    getSessionCommands: (sessionId: string) =>
      fetchApi<Command[]>(`/sessions/${sessionId}/commands`),

    terminateSession: (sessionId: string) =>
      fetchApi<void>(`/sessions/${sessionId}`, {
        method: 'DELETE',
      }),

    // Credentials
    getCredentials: () => fetchApi<Credential[]>('/credentials'),

    // Stats
    getStats: () => fetchApi<DashboardStats>('/stats'),

    // MITRE
    getMitreCoverage: () => fetchApi<MitreCoverage>('/mitre/coverage'),

    // Network
    getNetworkHosts: () => fetchApi<NetworkHost[]>('/network/hosts'),
    getNetworkEdges: () => fetchApi<NetworkEdge[]>('/network/edges'),

    // Health
    healthCheck: () => fetchApi<string>('/health'),

    // ================================================================
    // Post-Exploitation Modules
    // ================================================================

    // Privilege Escalation
    runPrivEsc: (request: PrivEscRequest) =>
      fetchApi<PrivEscResult>('/modules/privesc', {
        method: 'POST',
        body: JSON.stringify(request),
      }),

    // Credential Harvesting
    harvestCredentials: (request: CredentialHarvestRequest) =>
      fetchApi<CredentialHarvestResult>('/modules/credentials', {
        method: 'POST',
        body: JSON.stringify(request),
      }),

    // Persistence
    installPersistence: (request: PersistenceRequest) =>
      fetchApi<PersistenceResult>('/modules/persistence', {
        method: 'POST',
        body: JSON.stringify(request),
      }),

    // Lateral Movement
    lateralMove: (request: LateralMoveRequest) =>
      fetchApi<LateralMoveResult>('/modules/lateral', {
        method: 'POST',
        body: JSON.stringify(request),
      }),

    // Network Discovery
    discoverNetwork: (sessionId: string) =>
      fetchApi<DiscoveryResult>(`/modules/discovery/${sessionId}`),
  };
}

// React Query keys
export const queryKeys = {
  sessions: ['sessions'] as const,
  session: (id: string) => ['session', id] as const,
  sessionCommands: (id: string) => ['sessionCommands', id] as const,
  credentials: ['credentials'] as const,
  stats: ['stats'] as const,
  mitre: ['mitre'] as const,
  networkHosts: ['networkHosts'] as const,
  networkEdges: ['networkEdges'] as const,
};
