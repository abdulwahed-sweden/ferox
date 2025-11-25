// Real Service - Connects to Tauri backend via invoke
// This provides actual functionality through the Rust backend

import { invoke } from '@tauri-apps/api/core';
import {
  IFeroxService,
  ServiceResponse,
  ScanTarget,
  ScanResult,
  HttpScanTarget,
  HttpScanResult,
  DnsEnumResult,
  WhoisResult,
  SubdomainEnumResult,
  AsnResult,
  OpsecStatus,
  Countermeasure,
  EnvironmentAnalysis,
  TrafficAnalysis,
  NetworkTopology,
  NetworkNode,
  ReportOptions,
  ReportResult,
} from './types';

// Helper to wrap invoke calls with error handling
async function invokeCommand<T>(command: string, args?: Record<string, unknown>): Promise<ServiceResponse<T>> {
  try {
    const data = await invoke<T>(command, args);
    return {
      success: true,
      data,
      timestamp: new Date().toISOString(),
    };
  } catch (error) {
    console.error(`[RealService] Command ${command} failed:`, error);
    return {
      success: false,
      error: error instanceof Error ? error.message : String(error),
      timestamp: new Date().toISOString(),
    };
  }
}

// =============================================================================
// Real Service Implementation
// =============================================================================

export class RealService implements IFeroxService {
  // Scanner
  async scanPorts(target: ScanTarget): Promise<ServiceResponse<ScanResult>> {
    return invokeCommand<ScanResult>('scan_ports', {
      hosts: target.hosts,
      ports: target.ports,
      threads: target.threads ?? 10,
      timeout: target.timeout ?? 3000,
    });
  }

  async scanHttp(target: HttpScanTarget): Promise<ServiceResponse<HttpScanResult[]>> {
    return invokeCommand<HttpScanResult[]>('scan_http', {
      urls: target.urls,
      followRedirects: target.follow_redirects ?? true,
      timeout: target.timeout ?? 10000,
    });
  }

  // Recon
  async dnsEnum(domain: string): Promise<ServiceResponse<DnsEnumResult>> {
    return invokeCommand<DnsEnumResult>('dns_enum', { domain });
  }

  async whoisLookup(domain: string): Promise<ServiceResponse<WhoisResult>> {
    return invokeCommand<WhoisResult>('whois_lookup', { domain });
  }

  async subdomainEnum(domain: string, methods?: string[]): Promise<ServiceResponse<SubdomainEnumResult>> {
    return invokeCommand<SubdomainEnumResult>('subdomain_enum', {
      domain,
      methods: methods ?? ['dns', 'ct', 'brute'],
    });
  }

  async asnLookup(ip: string): Promise<ServiceResponse<AsnResult>> {
    return invokeCommand<AsnResult>('asn_lookup', { ip });
  }

  // OPSEC
  async checkOpsec(): Promise<ServiceResponse<OpsecStatus>> {
    return invokeCommand<OpsecStatus>('check_opsec');
  }

  async getCountermeasures(): Promise<ServiceResponse<Countermeasure[]>> {
    return invokeCommand<Countermeasure[]>('get_countermeasures');
  }

  async activateCountermeasure(id: string): Promise<ServiceResponse<boolean>> {
    return invokeCommand<boolean>('activate_countermeasure', { id });
  }

  async deactivateCountermeasure(id: string): Promise<ServiceResponse<boolean>> {
    return invokeCommand<boolean>('deactivate_countermeasure', { id });
  }

  async analyzeEnvironment(): Promise<ServiceResponse<EnvironmentAnalysis>> {
    return invokeCommand<EnvironmentAnalysis>('analyze_environment');
  }

  async analyzeTraffic(): Promise<ServiceResponse<TrafficAnalysis>> {
    return invokeCommand<TrafficAnalysis>('analyze_traffic');
  }

  // Network Map
  async getTopology(): Promise<ServiceResponse<NetworkTopology>> {
    return invokeCommand<NetworkTopology>('get_topology');
  }

  async discoverHosts(cidr: string): Promise<ServiceResponse<NetworkNode[]>> {
    return invokeCommand<NetworkNode[]>('discover_hosts', { cidr });
  }

  // Reports
  async exportReport(options: ReportOptions): Promise<ServiceResponse<ReportResult>> {
    return invokeCommand<ReportResult>('export_report', {
      format: options.format,
      includeRaw: options.include_raw,
      includeScreenshots: options.include_screenshots,
      classification: options.classification,
    });
  }
}

// Export singleton instance
export const realService = new RealService();
