// Service Layer Types
// These types define the interface between UI and backend

// =============================================================================
// Scanner Types
// =============================================================================

export interface ScanTarget {
  hosts: string;      // CIDR or comma-separated IPs
  ports: string;      // Port range or comma-separated
  threads?: number;
  timeout?: number;
}

export interface PortResult {
  port: number;
  state: 'open' | 'closed' | 'filtered';
  service: string;
  version: string | null;
  banner: string | null;
}

export interface HostScanResult {
  ip: string;
  hostname: string | null;
  status: 'up' | 'down';
  ports: PortResult[];
  os_guess: string | null;
  latency_ms: number;
}

export interface ScanResult {
  id: string;
  target: string;
  hosts: HostScanResult[];
  start_time: string;
  end_time: string;
  duration_ms: number;
  total_hosts: number;
  hosts_up: number;
}

export interface HttpScanTarget {
  urls: string[];
  follow_redirects?: boolean;
  timeout?: number;
}

export interface HttpScanResult {
  url: string;
  status_code: number;
  headers: Record<string, string>;
  server: string | null;
  technologies: string[];
  title: string | null;
  response_time_ms: number;
}

// =============================================================================
// Recon Types
// =============================================================================

export interface DnsRecord {
  record_type: string;  // A, AAAA, MX, NS, TXT, CNAME, SOA
  name: string;
  value: string;
  ttl: number;
}

export interface DnsEnumResult {
  domain: string;
  records: DnsRecord[];
  nameservers: string[];
  mx_records: string[];
}

export interface WhoisResult {
  domain: string;
  registrar: string | null;
  creation_date: string | null;
  expiration_date: string | null;
  updated_date: string | null;
  nameservers: string[];
  status: string[];
  registrant: Record<string, string>;
  raw: string;
}

export interface SubdomainResult {
  subdomain: string;
  ip: string | null;
  status: 'active' | 'inactive';
  source: string;  // dns, ct, brute
}

export interface SubdomainEnumResult {
  domain: string;
  subdomains: SubdomainResult[];
  total_found: number;
  sources_used: string[];
}

export interface AsnResult {
  ip: string;
  asn: string;
  org: string;
  country: string;
  registry: string;
  cidr: string;
  ranges: string[];
}

// =============================================================================
// OPSEC Types
// =============================================================================

export type ThreatLevel = 'low' | 'medium' | 'high' | 'critical';
export type ThreatCategory = 'network' | 'process' | 'behavioral' | 'forensic' | 'detection';

export interface Threat {
  id: string;
  category: ThreatCategory;
  severity: ThreatLevel;
  title: string;
  description: string;
  mitigation: string;
  timestamp: string;
  source: string;
  indicators: string[];
}

export interface Countermeasure {
  id: string;
  name: string;
  description: string;
  category: string;
  enabled: boolean;
  risk_reduction: number;  // 0-100
  performance_impact: 'none' | 'low' | 'medium' | 'high';
}

export interface EnvironmentAnalysis {
  hostname: string;
  os: string;
  os_version: string;
  is_vm: boolean;
  vm_type: string | null;
  is_sandbox: boolean;
  sandbox_indicators: string[];
  av_detected: string[];
  edr_detected: string[];
  monitoring_tools: string[];
  network_monitoring: boolean;
  debug_mode: boolean;
}

export interface OpsecStatus {
  score: number;              // 0-100
  threat_level: ThreatLevel;
  active_countermeasures: Countermeasure[];
  detected_threats: Threat[];
  recommendations: string[];
  last_check: string;
  environment: EnvironmentAnalysis;
}

export interface TrafficAnalysis {
  total_bytes_sent: number;
  total_bytes_received: number;
  connections_active: number;
  suspicious_patterns: string[];
  beacon_detected: boolean;
  exfil_risk: ThreatLevel;
}

// =============================================================================
// Network Map Types
// =============================================================================

export type NodeType =
  | 'attacker'
  | 'c2'
  | 'target'
  | 'compromised'
  | 'pivot'
  | 'exfil'
  | 'router'
  | 'firewall';

export type ConnectionType = 'control' | 'data' | 'exfil' | 'lateral' | 'recon';

export interface NetworkNode {
  id: string;
  label: string;
  type: NodeType;
  ip: string;
  hostname: string | null;
  os: string | null;
  status: 'active' | 'inactive' | 'unknown';
  sessions: number;
  x?: number;
  y?: number;
}

export interface NetworkConnection {
  id: string;
  source: string;
  target: string;
  type: ConnectionType;
  protocol: string;
  port: number;
  encrypted: boolean;
  active: boolean;
  bandwidth: number;  // bytes/sec
}

export interface NetworkTopology {
  nodes: NetworkNode[];
  connections: NetworkConnection[];
  last_updated: string;
}

// =============================================================================
// Report Types
// =============================================================================

export type ReportFormat = 'json' | 'html' | 'pdf';

export interface ReportOptions {
  format: ReportFormat;
  include_raw: boolean;
  include_screenshots: boolean;
  classification: string;
}

export interface ReportResult {
  path: string;
  format: ReportFormat;
  size_bytes: number;
  generated_at: string;
}

// =============================================================================
// Service Response Wrapper
// =============================================================================

export interface ServiceResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  timestamp: string;
}

// =============================================================================
// Service Interface
// =============================================================================

export interface IFeroxService {
  // Scanner
  scanPorts(target: ScanTarget): Promise<ServiceResponse<ScanResult>>;
  scanHttp(target: HttpScanTarget): Promise<ServiceResponse<HttpScanResult[]>>;

  // Recon
  dnsEnum(domain: string): Promise<ServiceResponse<DnsEnumResult>>;
  whoisLookup(domain: string): Promise<ServiceResponse<WhoisResult>>;
  subdomainEnum(domain: string, methods?: string[]): Promise<ServiceResponse<SubdomainEnumResult>>;
  asnLookup(ip: string): Promise<ServiceResponse<AsnResult>>;

  // OPSEC
  checkOpsec(): Promise<ServiceResponse<OpsecStatus>>;
  getCountermeasures(): Promise<ServiceResponse<Countermeasure[]>>;
  activateCountermeasure(id: string): Promise<ServiceResponse<boolean>>;
  deactivateCountermeasure(id: string): Promise<ServiceResponse<boolean>>;
  analyzeEnvironment(): Promise<ServiceResponse<EnvironmentAnalysis>>;
  analyzeTraffic(): Promise<ServiceResponse<TrafficAnalysis>>;

  // Network Map
  getTopology(): Promise<ServiceResponse<NetworkTopology>>;
  discoverHosts(cidr: string): Promise<ServiceResponse<NetworkNode[]>>;

  // Reports
  exportReport(options: ReportOptions): Promise<ServiceResponse<ReportResult>>;
}
