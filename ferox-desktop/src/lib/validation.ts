// src/lib/validation.ts
// Zod schemas for Tauri command validation

import { z } from 'zod';

// ============================================================================
// Common Schemas
// ============================================================================

export const SessionIdSchema = z.string().uuid().or(z.string().min(1));

export const IpAddressSchema = z.string().regex(
  /^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/,
  'Invalid IP address format'
);

export const CidrSchema = z.string().regex(
  /^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\/(3[0-2]|[12]?[0-9])$/,
  'Invalid CIDR notation'
);

export const HostnameSchema = z.string().min(1).max(255).regex(
  /^[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?)*$/,
  'Invalid hostname format'
);

export const PortSchema = z.number().int().min(1).max(65535);

export const PortRangeSchema = z.string().regex(
  /^[0-9]+(-[0-9]+)?(,[0-9]+(-[0-9]+)?)*$/,
  'Invalid port range format (e.g., "80,443,8080-8090")'
);

// ============================================================================
// Session Schemas
// ============================================================================

export const CreateSessionRequestSchema = z.object({
  hostname: HostnameSchema,
  ip_address: IpAddressSchema,
  os: z.enum(['windows', 'linux', 'macos', 'unknown']),
  username: z.string().min(1).max(256),
  privileges: z.enum(['user', 'administrator', 'system', 'root']),
  parent_id: SessionIdSchema.optional(),
});

export const UpdateSessionNoteSchema = z.object({
  id: SessionIdSchema,
  note: z.string().max(10000).nullable(),
});

// ============================================================================
// Terminal Schemas
// ============================================================================

export const CreateTerminalRequestSchema = z.object({
  session_id: SessionIdSchema,
  rows: z.number().int().min(1).max(500).optional(),
  cols: z.number().int().min(1).max(500).optional(),
  shell: z.string().max(256).optional(),
});

export const WriteTerminalRequestSchema = z.object({
  terminal_id: z.string().min(1),
  data: z.string().max(1024 * 1024), // 1MB max
});

export const ResizeTerminalRequestSchema = z.object({
  terminal_id: z.string().min(1),
  rows: z.number().int().min(1).max(500),
  cols: z.number().int().min(1).max(500),
});

// ============================================================================
// Module Schemas
// ============================================================================

export const ExecuteCommandRequestSchema = z.object({
  session_id: SessionIdSchema,
  command: z.string().min(1).max(10000),
});

export const RunPrivescRequestSchema = z.object({
  session_id: SessionIdSchema,
  auto_escalate: z.boolean(),
  safe_mode: z.boolean(),
});

export const HarvestCredentialsRequestSchema = z.object({
  session_id: SessionIdSchema,
  sources: z.array(z.string()).max(50),
  safe_mode: z.boolean(),
});

export const InstallPersistenceRequestSchema = z.object({
  session_id: SessionIdSchema,
  method: z.string().min(1),
  name: z.string().min(1).max(256),
  safe_mode: z.boolean(),
});

export const LateralMoveRequestSchema = z.object({
  session_id: SessionIdSchema,
  target_host: IpAddressSchema.or(HostnameSchema),
  method: z.string().min(1),
  credential_id: z.string().optional(),
  safe_mode: z.boolean(),
});

export const NetworkDiscoveryRequestSchema = z.object({
  session_id: SessionIdSchema,
  subnet: CidrSchema.optional(),
  ports: z.array(PortSchema).max(1000).optional(),
});

// ============================================================================
// Payload Schemas
// ============================================================================

export const PayloadConfigSchema = z.object({
  payload_type: z.string().min(1),
  format: z.string().min(1),
  lhost: IpAddressSchema.or(HostnameSchema),
  lport: PortSchema,
  staged: z.boolean().optional(),
  obfuscation: z.boolean().optional(),
  encryption: z.string().optional(),
  sleep_time: z.number().int().min(0).optional(),
  jitter: z.number().min(0).max(100).optional(),
  signing: z.boolean().optional(),
});

// ============================================================================
// Simulation Schemas
// ============================================================================

export const SimulateNetworkScanSchema = z.object({
  subnet: CidrSchema,
  sessionId: SessionIdSchema,
});

export const SimulateCredentialDumpSchema = z.object({
  sessionId: SessionIdSchema,
  sources: z.array(z.string()).optional(),
});

export const SimulateDirectoryListingSchema = z.object({
  path: z.string().min(1).max(4096),
  sessionId: SessionIdSchema,
});

// ============================================================================
// Validation Helpers
// ============================================================================

export class ValidationError extends Error {
  public readonly issues: z.ZodIssue[];
  
  constructor(issues: z.ZodIssue[]) {
    const message = issues.map(i => `${i.path.join('.')}: ${i.message}`).join(', ');
    super(`Validation failed: ${message}`);
    this.name = 'ValidationError';
    this.issues = issues;
  }
}

export function validate<T>(schema: z.ZodSchema<T>, data: unknown): T {
  const result = schema.safeParse(data);
  if (!result.success) {
    throw new ValidationError(result.error.issues);
  }
  return result.data;
}

export function validateOrNull<T>(schema: z.ZodSchema<T>, data: unknown): T | null {
  const result = schema.safeParse(data);
  return result.success ? result.data : null;
}

// Type exports for use in components
export type CreateSessionRequest = z.infer<typeof CreateSessionRequestSchema>;
export type CreateTerminalRequest = z.infer<typeof CreateTerminalRequestSchema>;
export type ExecuteCommandRequest = z.infer<typeof ExecuteCommandRequestSchema>;
export type PayloadConfig = z.infer<typeof PayloadConfigSchema>;
