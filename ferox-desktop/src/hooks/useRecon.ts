// useRecon Hook - Reconnaissance functionality (DNS, WHOIS, Subdomain, ASN)
import { useState, useCallback } from 'react';
import {
  getService,
  DnsEnumResult,
  WhoisResult,
  SubdomainEnumResult,
  AsnResult,
} from '../services';

interface UseReconState {
  loading: boolean;
  error: string | null;
  dnsResult: DnsEnumResult | null;
  whoisResult: WhoisResult | null;
  subdomainResult: SubdomainEnumResult | null;
  asnResult: AsnResult | null;
}

interface UseReconReturn extends UseReconState {
  dnsEnum: (domain: string) => Promise<DnsEnumResult | null>;
  whoisLookup: (domain: string) => Promise<WhoisResult | null>;
  subdomainEnum: (domain: string, methods?: string[]) => Promise<SubdomainEnumResult | null>;
  asnLookup: (ip: string) => Promise<AsnResult | null>;
  clearResults: () => void;
  clearError: () => void;
}

export function useRecon(): UseReconReturn {
  const [state, setState] = useState<UseReconState>({
    loading: false,
    error: null,
    dnsResult: null,
    whoisResult: null,
    subdomainResult: null,
    asnResult: null,
  });

  const dnsEnum = useCallback(async (domain: string): Promise<DnsEnumResult | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.dnsEnum(domain);

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          dnsResult: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'DNS enumeration failed',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const whoisLookup = useCallback(async (domain: string): Promise<WhoisResult | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.whoisLookup(domain);

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          whoisResult: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'WHOIS lookup failed',
        }));
        return null;
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Unknown error';
      setState(prev => ({ ...prev, loading: false, error: errorMsg }));
      return null;
    }
  }, []);

  const subdomainEnum = useCallback(
    async (domain: string, methods?: string[]): Promise<SubdomainEnumResult | null> => {
      setState(prev => ({ ...prev, loading: true, error: null }));

      try {
        const service = getService();
        const response = await service.subdomainEnum(domain, methods);

        if (response.success && response.data) {
          setState(prev => ({
            ...prev,
            loading: false,
            subdomainResult: response.data!,
          }));
          return response.data;
        } else {
          setState(prev => ({
            ...prev,
            loading: false,
            error: response.error || 'Subdomain enumeration failed',
          }));
          return null;
        }
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : 'Unknown error';
        setState(prev => ({ ...prev, loading: false, error: errorMsg }));
        return null;
      }
    },
    []
  );

  const asnLookup = useCallback(async (ip: string): Promise<AsnResult | null> => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const service = getService();
      const response = await service.asnLookup(ip);

      if (response.success && response.data) {
        setState(prev => ({
          ...prev,
          loading: false,
          asnResult: response.data!,
        }));
        return response.data;
      } else {
        setState(prev => ({
          ...prev,
          loading: false,
          error: response.error || 'ASN lookup failed',
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
    setState(prev => ({
      ...prev,
      dnsResult: null,
      whoisResult: null,
      subdomainResult: null,
      asnResult: null,
    }));
  }, []);

  const clearError = useCallback(() => {
    setState(prev => ({ ...prev, error: null }));
  }, []);

  return {
    ...state,
    dnsEnum,
    whoisLookup,
    subdomainEnum,
    asnLookup,
    clearResults,
    clearError,
  };
}

export default useRecon;
