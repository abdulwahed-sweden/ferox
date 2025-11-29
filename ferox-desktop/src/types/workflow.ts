// Security Assessment Workflow Types

// Target type for assessment
export type AssessmentTargetType =
  | "ip_address"
  | "domain"
  | "url"
  | "cidr_range"
  | "multi_target"
  | "mobile_app";

// Mobile platform type
export type MobilePlatform = "android" | "ios" | "unknown";

// Assessment scope
export type AssessmentScope =
  | "passive_recon"
  | "active_recon"
  | "discovery"
  | "comprehensive"
  | "mobile_analysis";

// Scan intensity
export type ScanIntensity = "quiet" | "normal" | "aggressive";

// Workflow status
export type WorkflowStatus =
  | "idle"
  | "running"
  | "paused"
  | "completed"
  | "failed"
  | "cancelled";

// Phase status
export type PhaseStatus =
  | "pending"
  | "running"
  | "completed"
  | "failed"
  | "skipped";

// Module status
export type ModuleStatus =
  | "pending"
  | "running"
  | "completed"
  | "failed"
  | "skipped";

// Discovery type
export type DiscoveryType =
  | "open_port"
  | "http_service"
  | "dns_record"
  | "subdomain"
  | "technology"
  | "certificate"
  | "whois_info"
  | "asn_info"
  | "vulnerability"
  | "misconfiguration";

// Target configuration
export interface TargetConfig {
  target_type: AssessmentTargetType;
  target: string;
  resolved_targets: string[];
  authorized: boolean;
  authorization_ref: string;
  notes: string;
  mobile_platform?: MobilePlatform;
}

// Workflow module
export interface WorkflowModule {
  id: string;
  path: string;
  name: string;
  description: string;
  options: Record<string, string>;
  enabled: boolean;
  phase: number;
  estimated_duration_secs: number;
}

// Workflow configuration
export interface WorkflowConfig {
  id: string;
  name: string;
  target: TargetConfig;
  scope: AssessmentScope;
  intensity: ScanIntensity;
  modules: WorkflowModule[];
  created_at: string;
  modified_at: string;
}

// Discovery
export interface Discovery {
  discovery_type: DiscoveryType;
  value: string;
  details: Record<string, string>;
  importance: number;
}

// Module execution result
export interface ModuleExecutionResult {
  module_id: string;
  module_path: string;
  module_name: string;
  status: ModuleStatus;
  success: boolean;
  message: string;
  data: Record<string, unknown>;
  discoveries: Discovery[];
  started_at: string;
  completed_at: string | null;
  duration_ms: number;
}

// Phase result
export interface PhaseResult {
  phase: number;
  name: string;
  status: PhaseStatus;
  modules: ModuleExecutionResult[];
  total_modules: number;
  completed_modules: number;
  started_at: string | null;
  completed_at: string | null;
}

// Workflow progress
export interface WorkflowProgress {
  workflow_id: string;
  status: WorkflowStatus;
  current_phase: number;
  current_module_index: number;
  total_modules: number;
  completed_modules: number;
  progress_percent: number;
  phase1_result: PhaseResult | null;
  phase2_result: PhaseResult | null;
  all_discoveries: Discovery[];
  started_at: string | null;
  estimated_completion: string | null;
  error: string | null;
}

// Workflow template
export interface WorkflowTemplate {
  id: string;
  name: string;
  description: string;
  recommended_target_type: AssessmentTargetType;
  default_scope: AssessmentScope;
  default_intensity: ScanIntensity;
  modules: WorkflowModule[];
  icon: string;
  tags: string[];
}

// Report summary
export interface ReportSummary {
  total_duration_secs: number;
  modules_executed: number;
  modules_succeeded: number;
  modules_failed: number;
  total_discoveries: number;
  open_ports: number;
  http_services: number;
  subdomains: number;
  technologies: number;
}

// Assessment report
export interface AssessmentReport {
  id: string;
  title: string;
  target: string;
  scope: AssessmentScope;
  generated_at: string;
  started_at: string;
  completed_at: string;
  summary: ReportSummary;
  discoveries_by_type: Record<string, Discovery[]>;
  phases: PhaseResult[];
  authorization_ref: string;
  notes: string;
}

// Workflow event types
export type WorkflowEventType =
  | "started"
  | "phase_started"
  | "module_started"
  | "module_progress"
  | "module_completed"
  | "phase_completed"
  | "discovery_made"
  | "progress_update"
  | "paused"
  | "resumed"
  | "completed"
  | "error";

// Workflow event
export interface WorkflowEvent {
  type: WorkflowEventType;
  workflow_id: string;
  timestamp?: string;
  // Event-specific fields
  phase?: number;
  phase_name?: string;
  module_id?: string;
  module_name?: string;
  success?: boolean;
  message?: string;
  progress_percent?: number;
  discoveries_count?: number;
  discovery?: Discovery;
  progress?: WorkflowProgress;
  error?: string;
  total_modules?: number;
  total_discoveries?: number;
}

// Wizard step
export type WizardStep =
  | "target"
  | "scope"
  | "modules"
  | "review"
  | "execute";

// Helper functions for display
export const targetTypeLabels: Record<AssessmentTargetType, string> = {
  ip_address: "IP Address",
  domain: "Domain",
  url: "URL",
  cidr_range: "CIDR Range",
  multi_target: "Multiple Targets",
  mobile_app: "Mobile Application (APK/IPA)",
};

export const mobilePlatformLabels: Record<MobilePlatform, string> = {
  android: "Android",
  ios: "iOS",
  unknown: "Unknown",
};

export const scopeLabels: Record<AssessmentScope, string> = {
  passive_recon: "Passive Reconnaissance",
  active_recon: "Active Reconnaissance",
  discovery: "Discovery Scanning",
  comprehensive: "Comprehensive Assessment",
  mobile_analysis: "Mobile App Analysis",
};

export const scopeDescriptions: Record<AssessmentScope, string> = {
  passive_recon: "Gather information without directly contacting the target",
  active_recon: "DNS enumeration, WHOIS lookup, subdomain discovery",
  discovery: "Port scanning, service detection, HTTP fingerprinting",
  comprehensive: "Full reconnaissance and discovery workflow",
  mobile_analysis: "Static analysis of mobile applications (APK/IPA)",
};

// Helper to detect mobile platform from file path
export function detectMobilePlatform(filePath: string): MobilePlatform {
  const lower = filePath.toLowerCase();
  if (lower.endsWith(".apk")) return "android";
  if (lower.endsWith(".ipa")) return "ios";
  return "unknown";
}

export const intensityLabels: Record<ScanIntensity, string> = {
  quiet: "Quiet",
  normal: "Normal",
  aggressive: "Aggressive",
};

export const intensityDescriptions: Record<ScanIntensity, string> = {
  quiet: "Slow, minimal network footprint",
  normal: "Balanced speed and resource usage",
  aggressive: "Fast scanning, higher detection risk",
};

export const discoveryTypeLabels: Record<DiscoveryType, string> = {
  open_port: "Open Port",
  http_service: "HTTP Service",
  dns_record: "DNS Record",
  subdomain: "Subdomain",
  technology: "Technology",
  certificate: "Certificate",
  whois_info: "WHOIS Info",
  asn_info: "ASN Info",
  vulnerability: "Vulnerability",
  misconfiguration: "Misconfiguration",
};

export const discoveryTypeColors: Record<DiscoveryType, string> = {
  open_port: "text-blue-400",
  http_service: "text-cyan-400",
  dns_record: "text-emerald-400",
  subdomain: "text-purple-400",
  technology: "text-yellow-400",
  certificate: "text-orange-400",
  whois_info: "text-gray-400",
  asn_info: "text-pink-400",
  vulnerability: "text-red-400",
  misconfiguration: "text-amber-400",
};
