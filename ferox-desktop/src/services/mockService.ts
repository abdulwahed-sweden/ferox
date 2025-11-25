// Mock Service - Provides simulated data for demo/testing
// This keeps the UI working without backend connection

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
  HostScanResult,
  PortResult,
} from './types';

// Helper to create response
function success<T>(data: T): ServiceResponse<T> {
  return { success: true, data, timestamp: new Date().toISOString() };
}

function createError<T>(message: string): ServiceResponse<T> {
  return { success: false, error: message, timestamp: new Date().toISOString() };
}

// Export for potential use
export { createError };

// Simulate network delay
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

// =============================================================================
// Mock Data Generators
// =============================================================================

function generateMockPorts(): PortResult[] {
  const services = [
    { port: 22, service: 'ssh', version: 'OpenSSH 8.9' },
    { port: 80, service: 'http', version: 'nginx/1.18.0' },
    { port: 443, service: 'https', version: 'nginx/1.18.0' },
    { port: 3306, service: 'mysql', version: 'MySQL 8.0.32' },
    { port: 5432, service: 'postgresql', version: 'PostgreSQL 14.7' },
    { port: 8080, service: 'http-proxy', version: 'Apache Tomcat/9.0' },
    { port: 3389, service: 'ms-wbt-server', version: 'Microsoft Terminal Services' },
    { port: 445, service: 'microsoft-ds', version: 'Windows Server 2019' },
  ];

  return services
    .filter(() => Math.random() > 0.4)
    .map(s => ({
      port: s.port,
      state: 'open' as const,
      service: s.service,
      version: s.version,
      banner: null,
    }));
}

function generateMockHost(ip: string): HostScanResult {
  const hostnames = ['web-server', 'db-server', 'app-server', 'mail-server', 'fw-01', null];
  const oses = ['Linux 5.x', 'Windows Server 2019', 'Ubuntu 22.04', 'CentOS 8', null];

  return {
    ip,
    hostname: hostnames[Math.floor(Math.random() * hostnames.length)],
    status: Math.random() > 0.2 ? 'up' : 'down',
    ports: generateMockPorts(),
    os_guess: oses[Math.floor(Math.random() * oses.length)],
    latency_ms: Math.floor(Math.random() * 50) + 5,
  };
}

// =============================================================================
// Mock Service Implementation
// =============================================================================

export class MockService implements IFeroxService {
  // Scanner
  async scanPorts(target: ScanTarget): Promise<ServiceResponse<ScanResult>> {
    await delay(1500 + Math.random() * 1000);

    const baseIp = target.hosts.split('/')[0].split('.').slice(0, 3).join('.');
    const hosts: HostScanResult[] = [];

    for (let i = 1; i <= 5; i++) {
      hosts.push(generateMockHost(`${baseIp}.${i}`));
    }

    const result: ScanResult = {
      id: crypto.randomUUID(),
      target: target.hosts,
      hosts,
      start_time: new Date(Date.now() - 2000).toISOString(),
      end_time: new Date().toISOString(),
      duration_ms: 2000,
      total_hosts: hosts.length,
      hosts_up: hosts.filter(h => h.status === 'up').length,
    };

    return success(result);
  }

  async scanHttp(target: HttpScanTarget): Promise<ServiceResponse<HttpScanResult[]>> {
    await delay(1000 + Math.random() * 500);

    const results: HttpScanResult[] = target.urls.map(url => ({
      url,
      status_code: [200, 301, 403, 404, 500][Math.floor(Math.random() * 5)],
      headers: {
        'Server': 'nginx/1.18.0',
        'Content-Type': 'text/html',
        'X-Powered-By': 'Express',
      },
      server: 'nginx/1.18.0',
      technologies: ['nginx', 'Node.js', 'React'].slice(0, Math.floor(Math.random() * 3) + 1),
      title: 'Welcome Page',
      response_time_ms: Math.floor(Math.random() * 200) + 50,
    }));

    return success(results);
  }

  // Recon
  async dnsEnum(domain: string): Promise<ServiceResponse<DnsEnumResult>> {
    await delay(800);

    return success({
      domain,
      records: [
        { record_type: 'A', name: domain, value: '93.184.216.34', ttl: 3600 },
        { record_type: 'AAAA', name: domain, value: '2606:2800:220:1:248:1893:25c8:1946', ttl: 3600 },
        { record_type: 'MX', name: domain, value: 'mail.example.com', ttl: 3600 },
        { record_type: 'NS', name: domain, value: 'ns1.example.com', ttl: 86400 },
        { record_type: 'TXT', name: domain, value: 'v=spf1 include:_spf.example.com ~all', ttl: 3600 },
      ],
      nameservers: ['ns1.example.com', 'ns2.example.com'],
      mx_records: ['mail.example.com'],
    });
  }

  async whoisLookup(domain: string): Promise<ServiceResponse<WhoisResult>> {
    await delay(600);

    return success({
      domain,
      registrar: 'MarkMonitor Inc.',
      creation_date: '1995-08-14',
      expiration_date: '2025-08-13',
      updated_date: '2024-08-14',
      nameservers: ['ns1.example.com', 'ns2.example.com'],
      status: ['clientTransferProhibited', 'clientUpdateProhibited'],
      registrant: {
        name: 'Domain Administrator',
        organization: 'Example Inc.',
        country: 'US',
      },
      raw: 'Domain Name: EXAMPLE.COM\nRegistry Domain ID: ...',
    });
  }

  async subdomainEnum(domain: string): Promise<ServiceResponse<SubdomainEnumResult>> {
    await delay(2000);

    const subdomains = ['www', 'mail', 'api', 'dev', 'staging', 'admin', 'portal', 'cdn'];

    return success({
      domain,
      subdomains: subdomains.map(sub => ({
        subdomain: `${sub}.${domain}`,
        ip: `${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}`,
        status: Math.random() > 0.2 ? 'active' as const : 'inactive' as const,
        source: ['dns', 'ct', 'brute'][Math.floor(Math.random() * 3)],
      })),
      total_found: subdomains.length,
      sources_used: ['dns', 'certificate_transparency', 'brute_force'],
    });
  }

  async asnLookup(ip: string): Promise<ServiceResponse<AsnResult>> {
    await delay(500);

    return success({
      ip,
      asn: 'AS15169',
      org: 'Google LLC',
      country: 'US',
      registry: 'ARIN',
      cidr: '8.8.8.0/24',
      ranges: ['8.8.8.0/24', '8.8.4.0/24'],
    });
  }

  // OPSEC
  async checkOpsec(): Promise<ServiceResponse<OpsecStatus>> {
    await delay(1000);

    return success({
      score: 78,
      threat_level: 'medium',
      active_countermeasures: [
        {
          id: 'cm-001',
          name: 'Traffic Encryption',
          description: 'All C2 traffic is encrypted with AES-256-GCM',
          category: 'network',
          enabled: true,
          risk_reduction: 25,
          performance_impact: 'low',
        },
        {
          id: 'cm-002',
          name: 'Process Injection',
          description: 'Implant runs inside legitimate process',
          category: 'evasion',
          enabled: true,
          risk_reduction: 30,
          performance_impact: 'medium',
        },
      ],
      detected_threats: [
        {
          id: 'th-001',
          category: 'network',
          severity: 'medium',
          title: 'Unusual Outbound Traffic',
          description: 'Detected periodic beacon pattern to external host',
          mitigation: 'Enable jitter on beacon interval',
          timestamp: new Date().toISOString(),
          source: 'traffic_analyzer',
          indicators: ['periodic_timing', 'fixed_interval'],
        },
      ],
      recommendations: [
        'Enable traffic jitter to randomize beacon intervals',
        'Consider using domain fronting for C2 communication',
        'Rotate encryption keys more frequently',
      ],
      last_check: new Date().toISOString(),
      environment: {
        hostname: 'WORKSTATION-01',
        os: 'Windows',
        os_version: '10.0.19044',
        is_vm: false,
        vm_type: null,
        is_sandbox: false,
        sandbox_indicators: [],
        av_detected: ['Windows Defender'],
        edr_detected: [],
        monitoring_tools: ['Sysmon'],
        network_monitoring: false,
        debug_mode: false,
      },
    });
  }

  async getCountermeasures(): Promise<ServiceResponse<Countermeasure[]>> {
    await delay(300);

    return success([
      {
        id: 'cm-001',
        name: 'Traffic Encryption',
        description: 'Encrypt all C2 traffic',
        category: 'network',
        enabled: true,
        risk_reduction: 25,
        performance_impact: 'low',
      },
      {
        id: 'cm-002',
        name: 'Process Hollowing',
        description: 'Hide implant in legitimate process',
        category: 'evasion',
        enabled: true,
        risk_reduction: 30,
        performance_impact: 'medium',
      },
      {
        id: 'cm-003',
        name: 'AMSI Bypass',
        description: 'Bypass Windows AMSI scanning',
        category: 'evasion',
        enabled: false,
        risk_reduction: 20,
        performance_impact: 'low',
      },
      {
        id: 'cm-004',
        name: 'ETW Patching',
        description: 'Disable Event Tracing for Windows',
        category: 'forensic',
        enabled: false,
        risk_reduction: 15,
        performance_impact: 'none',
      },
      {
        id: 'cm-005',
        name: 'Domain Fronting',
        description: 'Use CDN for C2 masquerading',
        category: 'network',
        enabled: false,
        risk_reduction: 35,
        performance_impact: 'medium',
      },
    ]);
  }

  async activateCountermeasure(id: string): Promise<ServiceResponse<boolean>> {
    await delay(500);
    console.log(`[Mock] Activating countermeasure: ${id}`);
    return success(true);
  }

  async deactivateCountermeasure(id: string): Promise<ServiceResponse<boolean>> {
    await delay(500);
    console.log(`[Mock] Deactivating countermeasure: ${id}`);
    return success(true);
  }

  async analyzeEnvironment(): Promise<ServiceResponse<EnvironmentAnalysis>> {
    await delay(800);

    return success({
      hostname: 'WORKSTATION-01',
      os: 'Windows',
      os_version: '10.0.19044',
      is_vm: false,
      vm_type: null,
      is_sandbox: false,
      sandbox_indicators: [],
      av_detected: ['Windows Defender'],
      edr_detected: [],
      monitoring_tools: ['Sysmon'],
      network_monitoring: false,
      debug_mode: false,
    });
  }

  async analyzeTraffic(): Promise<ServiceResponse<TrafficAnalysis>> {
    await delay(600);

    return success({
      total_bytes_sent: 1024 * 1024 * 15,
      total_bytes_received: 1024 * 1024 * 42,
      connections_active: 3,
      suspicious_patterns: ['periodic_beacon'],
      beacon_detected: true,
      exfil_risk: 'low',
    });
  }

  // Network Map
  async getTopology(): Promise<ServiceResponse<NetworkTopology>> {
    await delay(500);

    return success({
      nodes: [
        { id: 'attacker', label: 'Attacker', type: 'attacker', ip: '10.0.0.1', hostname: 'kali', os: 'Linux', status: 'active', sessions: 0 },
        { id: 'c2', label: 'C2 Server', type: 'c2', ip: '185.x.x.x', hostname: 'c2.example.com', os: 'Linux', status: 'active', sessions: 5 },
        { id: 'target1', label: 'Web Server', type: 'compromised', ip: '192.168.1.10', hostname: 'web-01', os: 'Linux', status: 'active', sessions: 1 },
        { id: 'target2', label: 'DB Server', type: 'target', ip: '192.168.1.20', hostname: 'db-01', os: 'Linux', status: 'active', sessions: 0 },
        { id: 'pivot', label: 'Jump Host', type: 'pivot', ip: '192.168.1.5', hostname: 'jump-01', os: 'Windows', status: 'active', sessions: 2 },
        { id: 'fw', label: 'Firewall', type: 'firewall', ip: '192.168.1.1', hostname: 'fw-01', os: null, status: 'active', sessions: 0 },
      ],
      connections: [
        { id: 'c1', source: 'attacker', target: 'c2', type: 'control', protocol: 'HTTPS', port: 443, encrypted: true, active: true, bandwidth: 1024 },
        { id: 'c2', source: 'c2', target: 'target1', type: 'control', protocol: 'HTTPS', port: 443, encrypted: true, active: true, bandwidth: 512 },
        { id: 'c3', source: 'target1', target: 'pivot', type: 'lateral', protocol: 'SMB', port: 445, encrypted: false, active: true, bandwidth: 256 },
        { id: 'c4', source: 'pivot', target: 'target2', type: 'recon', protocol: 'TCP', port: 3306, encrypted: false, active: false, bandwidth: 0 },
      ],
      last_updated: new Date().toISOString(),
    });
  }

  async discoverHosts(cidr: string): Promise<ServiceResponse<NetworkNode[]>> {
    await delay(2000);

    const baseIp = cidr.split('/')[0].split('.').slice(0, 3).join('.');
    const nodes: NetworkNode[] = [];

    for (let i = 1; i <= 10; i++) {
      if (Math.random() > 0.3) {
        nodes.push({
          id: `host-${i}`,
          label: `Host ${i}`,
          type: 'target',
          ip: `${baseIp}.${i}`,
          hostname: Math.random() > 0.5 ? `host-${i}.local` : null,
          os: ['Windows', 'Linux', 'macOS'][Math.floor(Math.random() * 3)],
          status: 'active',
          sessions: 0,
        });
      }
    }

    return success(nodes);
  }

  // Reports
  async exportReport(options: ReportOptions): Promise<ServiceResponse<ReportResult>> {
    await delay(1500);

    return success({
      path: `/tmp/ferox-report-${Date.now()}.${options.format}`,
      format: options.format,
      size_bytes: Math.floor(Math.random() * 50000) + 10000,
      generated_at: new Date().toISOString(),
    });
  }
}

// Export singleton instance
export const mockService = new MockService();
