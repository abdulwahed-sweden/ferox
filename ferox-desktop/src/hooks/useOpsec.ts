// useOpsec Hook - Operational Security monitoring and countermeasures
import { useState, useCallback, useEffect } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import {
  getService,
  OpsecStatus,
  Countermeasure,
  EnvironmentAnalysis,
  TrafficAnalysis,
  Threat,
  isUsingRealBackend,
} from '../services';

interface UseOpsecState {
  loading: boolean;
  error: string | null;
  status: OpsecStatus | null;
  countermeasures: Countermeasure[];
  environment: EnvironmentAnalysis | null;
  traffic: TrafficAnalysis | null;
  realtimeThreats: Threat[];
}

interface UseOpsecReturn extends UseOpsecState {
  checkOpsec: () => Promise<OpsecStatus | null>;
  loadCountermeasures: () => Promise<Countermeasure[] | null>;
  activateCountermeasure: (id: string) => Promise<boolean>;
  deactivateCountermeasure: (id: string) => Promise<boolean>;
  analyzeEnvironment: () => Promise<EnvironmentAnalysis | null>;
  analyzeTraffic: () => Promise<TrafficAnalysis | null>;
  clearThreats: () => void;
  clearError: () => void;
  goDark: () => Promise<void>;  // Enable all countermeasures
}

export function useOpsec(): UseOpsecReturn {
  const [state, setState] = useState<UseOpsecState>({
    loading: false,
    error: null,
    status: null,
    countermeasures: [],
    environment: null,
    traffic: null,
    realtimeThreats: [],
  });

  // Listen for real-time threat events (only in Tauri)
  useEffect(() => {
    if (!isUsingRealBackend()) return;

    let unlisten: UnlistenFn | null = null;

    const setupListener = async () => {
      try {
        unlisten = await listen<Threat>('opsec_threat', (event) => {
          setState(prev => ({
            ...prev,
            realtimeThreats: [...prev.realtimeThreats, event.payload],
          }));
        });
      } catch (err) {
        console.warn('[useOpsec] Failed to setup threat listener:', err);
      }
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  const checkOpsec = useCallback(async (): Promise<OpsecStatus | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.checkOpsec();

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          status: response.data!,
          environment: response.data!.environment,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'OPSEC check failed',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const loadCountermeasures = useCallback(async (): Promise<Countermeasure[] | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.getCountermeasures();

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          countermeasures: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'Failed to load countermeasures',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const activateCountermeasure = useCallback(async (id: string): Promise<boolean> => {
    try {
      const service = getService();
      const response = await service.activateCountermeasure(id);

      if (response.success) {
        // Update local state
        setState(prev => ({
          ...prev,
          countermeasures: prev.countermeasures.map(cm =>
            cm.id === id ? { ...cm, enabled: true } : cm
          ),
        }));
        // Refresh OPSEC status
        await checkOpsec();
        return true;
      }
      return false;
    } catch (err) {
      console.error('[useOpsec] Failed to activate countermeasure:', err);
      return false;
    }
  }, [checkOpsec]);

  const deactivateCountermeasure = useCallback(async (id: string): Promise<boolean> => {
    try {
      const service = getService();
      const response = await service.deactivateCountermeasure(id);

      if (response.success) {
        setState(prev => ({
          ...prev,
          countermeasures: prev.countermeasures.map(cm =>
            cm.id === id ? { ...cm, enabled: false } : cm
          ),
        }));
        await checkOpsec();
        return true;
      }
      return false;
    } catch (err) {
      console.error('[useOpsec] Failed to deactivate countermeasure:', err);
      return false;
    }
  }, [checkOpsec]);

  const analyzeEnvironment = useCallback(async (): Promise<EnvironmentAnalysis | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.analyzeEnvironment();

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          environment: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'Environment analysis failed',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const analyzeTraffic = useCallback(async (): Promise<TrafficAnalysis | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.analyzeTraffic();

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          traffic: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'Traffic analysis failed',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const clearThreats = useCallback(() => {
    setState(prev => ({ ...prev, realtimeThreats: [] }));
  }, []);

  const clearError = useCallback(() => {
    setState(prev => ({ ...prev, error: null }));
  }, []);

  // "Go Dark" - Enable all countermeasures
  const goDark = useCallback(async (): Promise<void> => {
    setState(prev => ({ ...prev, loading: true }));

    const service = getService();
    const cmResponse = await service.getCountermeasures();

    if (cmResponse.success && cmResponse.data) {
      for (const cm of cmResponse.data) {
        if (!cm.enabled) {
          await service.activateCountermeasure(cm.id);
        }
      }
    }

    await checkOpsec();
    await loadCountermeasures();

    setState(prev => ({ ...prev, loading: false }));
  }, [checkOpsec, loadCountermeasures]);

  return {
    ...state,
    checkOpsec,
    loadCountermeasures,
    activateCountermeasure,
    deactivateCountermeasure,
    analyzeEnvironment,
    analyzeTraffic,
    clearThreats,
    clearError,
    goDark,
  };
}

export default useOpsec;
