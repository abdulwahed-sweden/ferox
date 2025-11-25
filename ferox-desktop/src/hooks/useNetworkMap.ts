// useNetworkMap Hook - Network topology and host discovery
import { useState, useCallback, useEffect } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import {
  getService,
  NetworkTopology,
  NetworkNode,
  NetworkConnection,
  isUsingRealBackend,
} from '../services';

interface UseNetworkMapState {
  loading: boolean;
  error: string | null;
  topology: NetworkTopology | null;
  selectedNode: NetworkNode | null;
  discoveredHosts: NetworkNode[];
}

interface UseNetworkMapReturn extends UseNetworkMapState {
  loadTopology: () => Promise<NetworkTopology | null>;
  discoverHosts: (cidr: string) => Promise<NetworkNode[] | null>;
  selectNode: (node: NetworkNode | null) => void;
  addNode: (node: NetworkNode) => void;
  removeNode: (nodeId: string) => void;
  addConnection: (connection: NetworkConnection) => void;
  removeConnection: (connectionId: string) => void;
  clearDiscoveredHosts: () => void;
  clearError: () => void;
  refreshTopology: () => Promise<void>;
}

export function useNetworkMap(): UseNetworkMapReturn {
  const [state, setState] = useState<UseNetworkMapState>({
    loading: false,
    error: null,
    topology: null,
    selectedNode: null,
    discoveredHosts: [],
  });

  // Listen for topology updates (only in Tauri)
  useEffect(() => {
    if (!isUsingRealBackend()) return;

    let unlisten: UnlistenFn | null = null;

    const setupListener = async () => {
      try {
        unlisten = await listen<NetworkTopology>('topology_update', (event) => {
          setState(prev => ({
            ...prev,
            topology: event.payload,
          }));
        });
      } catch (err) {
        console.warn('[useNetworkMap] Failed to setup topology listener:', err);
      }
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  const loadTopology = useCallback(async (): Promise<NetworkTopology | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.getTopology();

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          topology: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'Failed to load topology',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const discoverHosts = useCallback(async (cidr: string): Promise<NetworkNode[] | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.discoverHosts(cidr);

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          discoveredHosts: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'Host discovery failed',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const selectNode = useCallback((node: NetworkNode | null) => {
    setState(prev => ({ ...prev, selectedNode: node }));
  }, []);

  const addNode = useCallback((node: NetworkNode) => {
    setState(prev => {
      if (!prev.topology) return prev;
      return {
        ...prev,
        topology: {
          ...prev.topology,
          nodes: [...prev.topology.nodes, node],
          last_updated: new Date().toISOString(),
        },
      };
    });
  }, []);

  const removeNode = useCallback((nodeId: string) => {
    setState(prev => {
      if (!prev.topology) return prev;
      return {
        ...prev,
        topology: {
          ...prev.topology,
          nodes: prev.topology.nodes.filter(n => n.id !== nodeId),
          connections: prev.topology.connections.filter(
            c => c.source !== nodeId && c.target !== nodeId
          ),
          last_updated: new Date().toISOString(),
        },
        selectedNode: prev.selectedNode?.id === nodeId ? null : prev.selectedNode,
      };
    });
  }, []);

  const addConnection = useCallback((connection: NetworkConnection) => {
    setState(prev => {
      if (!prev.topology) return prev;
      return {
        ...prev,
        topology: {
          ...prev.topology,
          connections: [...prev.topology.connections, connection],
          last_updated: new Date().toISOString(),
        },
      };
    });
  }, []);

  const removeConnection = useCallback((connectionId: string) => {
    setState(prev => {
      if (!prev.topology) return prev;
      return {
        ...prev,
        topology: {
          ...prev.topology,
          connections: prev.topology.connections.filter(c => c.id !== connectionId),
          last_updated: new Date().toISOString(),
        },
      };
    });
  }, []);

  const clearDiscoveredHosts = useCallback(() => {
    setState(prev => ({ ...prev, discoveredHosts: [] }));
  }, []);

  const clearError = useCallback(() => {
    setState(prev => ({ ...prev, error: null }));
  }, []);

  const refreshTopology = useCallback(async (): Promise<void> => {
    await loadTopology();
  }, [loadTopology]);

  return {
    ...state,
    loadTopology,
    discoverHosts,
    selectNode,
    addNode,
    removeNode,
    addConnection,
    removeConnection,
    clearDiscoveredHosts,
    clearError,
    refreshTopology,
  };
}

export default useNetworkMap;
