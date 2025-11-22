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
