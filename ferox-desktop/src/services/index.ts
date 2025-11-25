// Service Layer Factory
// Provides abstraction between UI and data source (mock vs real)

import { IFeroxService } from './types';
import { mockService } from './mockService';
import { realService } from './realService';

// =============================================================================
// Configuration
// =============================================================================

export type ServiceMode = 'mock' | 'real' | 'auto';

interface ServiceConfig {
  mode: ServiceMode;
  enableLogging: boolean;
}

// Default config - can be overridden
let config: ServiceConfig = {
  mode: 'auto',  // Auto-detect based on Tauri availability
  enableLogging: process.env.NODE_ENV === 'development',
};

// =============================================================================
// Service Detection
// =============================================================================

function isTauriAvailable(): boolean {
  try {
    // Check if we're running in Tauri environment
    return typeof window !== 'undefined' && '__TAURI__' in window;
  } catch {
    return false;
  }
}

function getEffectiveMode(): 'mock' | 'real' {
  if (config.mode === 'auto') {
    return isTauriAvailable() ? 'real' : 'mock';
  }
  return config.mode;
}

// =============================================================================
// Logging Wrapper
// =============================================================================

function createLoggingProxy(service: IFeroxService, mode: string): IFeroxService {
  if (!config.enableLogging) {
    return service;
  }

  return new Proxy(service, {
    get(target, prop, receiver) {
      const value = Reflect.get(target, prop, receiver);
      if (typeof value === 'function') {
        return async (...args: unknown[]) => {
          const start = performance.now();
          console.log(`[Service:${mode}] ${String(prop)} called with:`, args);
          try {
            const result = await value.apply(target, args);
            const duration = (performance.now() - start).toFixed(2);
            console.log(`[Service:${mode}] ${String(prop)} completed in ${duration}ms:`, result);
            return result;
          } catch (error) {
            console.error(`[Service:${mode}] ${String(prop)} failed:`, error);
            throw error;
          }
        };
      }
      return value;
    },
  });
}

// =============================================================================
// Service Factory
// =============================================================================

let cachedService: IFeroxService | null = null;
let cachedMode: string | null = null;

/**
 * Get the current service instance
 * Returns mock service in browser, real service in Tauri
 */
export function getService(): IFeroxService {
  const mode = getEffectiveMode();

  // Return cached if mode hasn't changed
  if (cachedService && cachedMode === mode) {
    return cachedService;
  }

  const baseService = mode === 'real' ? realService : mockService;
  cachedService = createLoggingProxy(baseService, mode);
  cachedMode = mode;

  if (config.enableLogging) {
    console.log(`[ServiceFactory] Using ${mode} service (Tauri: ${isTauriAvailable()})`);
  }

  return cachedService;
}

/**
 * Force use of a specific service mode
 */
export function setServiceMode(mode: ServiceMode): void {
  config.mode = mode;
  cachedService = null;  // Clear cache to force recreation
  cachedMode = null;
  console.log(`[ServiceFactory] Service mode set to: ${mode}`);
}

/**
 * Enable or disable logging
 */
export function setLogging(enabled: boolean): void {
  config.enableLogging = enabled;
  cachedService = null;
  cachedMode = null;
}

/**
 * Get current service mode
 */
export function getServiceMode(): { configured: ServiceMode; effective: 'mock' | 'real' } {
  return {
    configured: config.mode,
    effective: getEffectiveMode(),
  };
}

/**
 * Check if using real backend
 */
export function isUsingRealBackend(): boolean {
  return getEffectiveMode() === 'real';
}

// =============================================================================
// Re-exports
// =============================================================================

export * from './types';
export { mockService } from './mockService';
export { realService } from './realService';

// Default export
export default getService;
