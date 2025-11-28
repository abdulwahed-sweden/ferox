// ferox-desktop/src/types/opsec.ts
// OPSEC Dashboard TypeScript Types

export type StealthLevel = "Normal" | "Quiet" | "Silent" | "Ghost";

export type EdrType =
  | "WindowsDefender"
  | "DefenderATP"
  | "CrowdStrike"
  | "SentinelOne"
  | "CarbonBlack"
  | "Cylance"
  | "Sophos"
  | "Kaspersky"
  | "ESET"
  | "McAfee"
  | "Bitdefender"
  | "Elastic"
  | "Unknown";

export interface DetectedEdr {
  edrType: EdrType;
  confidence: number;
  threatLevel: number;
  evidence: string[];
}

export interface EdrDetectionResult {
  detectedEdrs: DetectedEdr[];
  totalThreatLevel: number;
  scanTimeMs: number;
  recommendedStealth: StealthLevel;
}

export interface AmsiBypassResult {
  success: boolean;
  technique: string;
  message: string;
  patchedAddress?: string;
}

export interface EtwPatchResult {
  success: boolean;
  providersPatched: string[];
  message: string;
}

export type VmType =
  | "VMware"
  | "VirtualBox"
  | "HyperV"
  | "QEMU"
  | "KVM"
  | "Xen"
  | "AmazonEC2"
  | "Azure"
  | "GoogleCloud"
  | "Docker";

export type SandboxType =
  | "CuckooSandbox"
  | "JoeSandbox"
  | "AnyRun"
  | "VirusTotal"
  | "HybridAnalysis"
  | "FireEye"
  | "CAPEv2"
  | "WindowsSandbox";

export interface EnvironmentReport {
  detectedVm?: VmType;
  detectedSandbox?: SandboxType;
  analysisTools: string[];
  suspicionScore: number;
  isSafeToExecute: boolean;
  recommendations: string[];
  timingAnomalies: string[];
}

export type MemoryEvasionTechnique =
  | "HeapEncrypt"
  | "StackObfuscate"
  | "ModuleHide"
  | "PeHeader";

export interface MemoryEvasionResult {
  success: boolean;
  technique: MemoryEvasionTechnique;
  message: string;
  regionsProtected: number;
}

export type InjectionTechnique =
  | "ClassicRemoteThread"
  | "NtCreateThreadEx"
  | "QueueUserApc"
  | "EarlyBird"
  | "ThreadHijack"
  | "ProcessHollowing"
  | "ModuleStomping"
  | "DirectSyscall";

export interface TargetProcess {
  pid: number;
  name: string;
  path?: string;
  is64bit: boolean;
  integrityLevel: string;
  suitability: number;
}

export interface InjectionResult {
  success: boolean;
  technique: InjectionTechnique;
  targetPid: number;
  threadId?: number;
  message: string;
}

export type ExfilChannel =
  | "Dns"
  | "HttpsPost"
  | "HttpsGet"
  | "Icmp"
  | "CloudStorage"
  | "Webhook"
  | "Steganography"
  | "Pastebin"
  | "WebSocket";

export interface ExfilChannelInfo {
  channel: ExfilChannel;
  stealthRating: number;
  bandwidthRating: number;
  maxChunkSize: number;
  mitreId: string;
}

export interface ExfilSession {
  sessionId: string;
  channel: ExfilChannel;
  totalBytes: number;
  bytesSent: number;
  chunksTotal: number;
  chunksSent: number;
  status: "Pending" | "InProgress" | "Completed" | "Failed" | "Paused";
  startedAt: string;
}

export interface OpsecStatus {
  stealthLevel: StealthLevel;
  amsiBypass: boolean;
  etwPatched: boolean;
  edrDetected: DetectedEdr[];
  vmDetected?: VmType;
  sandboxDetected?: SandboxType;
  memoryProtected: boolean;
  isSafe: boolean;
  lastScan: string;
}

// Panel state types
export interface EdrScanOptions {
  depth: "quick" | "standard" | "deep";
  safeMode: boolean;
}

export interface AmsiBypassOptions {
  technique: "PatchScanBuffer" | "MemoryPatch" | "ReflectiveUnhook" | "Amsi2";
}

export interface EtwPatchOptions {
  providers: ("PowerShell" | "DotNet" | "SecurityAuditing" | "ThreatIntel")[];
}

export interface InjectionOptions {
  technique: InjectionTechnique;
  targetPid?: number;
  targetName?: string;
  shellcode?: string;
}

export interface ExfilOptions {
  channel: ExfilChannel;
  endpoint: string;
  chunkSize: number;
  delayMs: number;
  jitterPercent: number;
  encryption: boolean;
}
