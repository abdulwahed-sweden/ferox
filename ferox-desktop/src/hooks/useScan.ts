// useScan Hook - Port and HTTP scanning functionality
import { useState, useCallback } from 'react';
import { getService, ScanTarget, ScanResult, HttpScanTarget, HttpScanResult } from '../services';

interface UseScanState {
  loading: boolean;
  error: string | null;
  portResult: ScanResult | null;
  httpResults: HttpScanResult[] | null;
}

interface UseScanReturn extends UseScanState {
  scanPorts: (target: ScanTarget) => Promise<ScanResult | null>;
  scanHttp: (target: HttpScanTarget) => Promise<HttpScanResult[] | null>;
  clearResults: () => void;
  clearError: () => void;
}

export function useScan(): UseScanReturn {
  const [state, setState] = useState<UseScanState>({
    loading: false,
    error: null,
    portResult: null,
    httpResults: null,
  });

  const scanPorts = useCallback(async (target: ScanTarget): Promise<ScanResult | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.scanPorts(target);

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          portResult: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'Port scan failed',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const scanHttp = useCallback(async (target: HttpScanTarget): Promise<HttpScanResult[] | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.scanHttp(target);

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          httpResults: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'HTTP scan failed',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const clearResults = useCallback(() => {
    setState(prev => ({ ...prev, portResult: null, httpResults: null }));
  }, []);

  const clearError = useCallback(() => {
    setState(prev => ({ ...prev, error: null }));
  }, []);

  return {
    ...state,
    scanPorts,
    scanHttp,
    clearResults,
    clearError,
  };
}

export default useScan;
