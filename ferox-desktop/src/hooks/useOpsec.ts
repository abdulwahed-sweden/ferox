// ferox-desktop/src/hooks/useOpsec.ts
// OPSEC state management hook

import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  OpsecStatus,
  EdrDetectionResult,
  EnvironmentReport,
  AmsiBypassResult,
  EtwPatchResult,
  MemoryEvasionResult,
  InjectionResult,
  ExfilSession,
  TargetProcess,
  ExfilChannelInfo,
  StealthLevel,
  EdrScanOptions,
  AmsiBypassOptions,
  EtwPatchOptions,
  InjectionOptions,
  ExfilOptions,
} from "../types/opsec";

export function useOpsec() {
  const [status, setStatus] = useState<OpsecStatus | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // EDR Detection
  const scanEdr = useCallback(async (options: EdrScanOptions) => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<EdrDetectionResult>("opsec_scan_edr", {
        depth: options.depth,
        safeMode: options.safeMode,
      });
      setStatus((prev) =>
        prev
          ? {
              ...prev,
              edrDetected: result.detectedEdrs,
              stealthLevel: result.recommendedStealth,
              lastScan: new Date().toISOString(),
            }
          : null,
      );
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  // Environment Detection (VM/Sandbox)
  const scanEnvironment = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<EnvironmentReport>("opsec_scan_environment");
      setStatus((prev) =>
        prev
          ? {
              ...prev,
              vmDetected: result.detectedVm,
              sandboxDetected: result.detectedSandbox,
              isSafe: result.isSafeToExecute,
            }
          : null,
      );
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  // AMSI Bypass
  const bypassAmsi = useCallback(async (options?: AmsiBypassOptions) => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<AmsiBypassResult>("opsec_bypass_amsi", {
        technique: options?.technique || "PatchScanBuffer",
      });
      setStatus((prev) =>
        prev ? { ...prev, amsiBypass: result.success } : null,
      );
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  // ETW Patch
  const patchEtw = useCallback(async (options?: EtwPatchOptions) => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<EtwPatchResult>("opsec_patch_etw", {
        providers: options?.providers || ["PowerShell", "DotNet"],
      });
      setStatus((prev) =>
        prev ? { ...prev, etwPatched: result.success } : null,
      );
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  // Memory Evasion
  const enableMemoryEvasion = useCallback(async (technique: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<MemoryEvasionResult>("opsec_memory_evasion", {
        technique,
      });
      setStatus((prev) =>
        prev ? { ...prev, memoryProtected: result.success } : null,
      );
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  // Process Injection
  const findTargets = useCallback(async (criteria: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<TargetProcess[]>("opsec_find_targets", {
        criteria,
      });
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  const inject = useCallback(async (options: InjectionOptions) => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<InjectionResult>("opsec_inject", {
        technique: options.technique,
        targetPid: options.targetPid,
        shellcode: options.shellcode,
      });
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  // Exfiltration
  const listExfilChannels = useCallback(async () => {
    try {
      const result = await invoke<ExfilChannelInfo[]>(
        "opsec_list_exfil_channels",
      );
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    }
  }, []);

  const startExfil = useCallback(
    async (options: ExfilOptions, data: string) => {
      setLoading(true);
      setError(null);
      try {
        const result = await invoke<ExfilSession>("opsec_start_exfil", {
          ...options,
          data,
        });
        return result;
      } catch (e) {
        setError(String(e));
        throw e;
      } finally {
        setLoading(false);
      }
    },
    [],
  );

  const getExfilSessions = useCallback(async () => {
    try {
      const result = await invoke<ExfilSession[]>("opsec_get_exfil_sessions");
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    }
  }, []);

  // Stealth Level
  const setStealthLevel = useCallback(async (level: StealthLevel) => {
    try {
      await invoke("opsec_set_stealth_level", { level });
      setStatus((prev) => (prev ? { ...prev, stealthLevel: level } : null));
    } catch (e) {
      setError(String(e));
    }
  }, []);

  // Get current status
  const getStatus = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<OpsecStatus>("opsec_get_status");
      setStatus(result);
      return result;
    } catch (e) {
      setError(String(e));
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  // Full OPSEC setup
  const setupOpsec = useCallback(
    async (stealthLevel: StealthLevel) => {
      setLoading(true);
      setError(null);
      try {
        await setStealthLevel(stealthLevel);

        // Run EDR scan
        const edrResult = await scanEdr({ depth: "standard", safeMode: true });

        // If high threat, enable bypasses
        if (edrResult.totalThreatLevel > 5) {
          await bypassAmsi({ technique: "PatchScanBuffer" });
          await patchEtw({ providers: ["PowerShell", "DotNet"] });
        }

        // Scan environment
        await scanEnvironment();

        return await getStatus();
      } catch (e) {
        setError(String(e));
        throw e;
      } finally {
        setLoading(false);
      }
    },
    [
      setStealthLevel,
      scanEdr,
      bypassAmsi,
      patchEtw,
      scanEnvironment,
      getStatus,
    ],
  );

  return {
    // State
    status,
    loading,
    error,

    // EDR
    scanEdr,

    // Environment
    scanEnvironment,

    // Bypasses
    bypassAmsi,
    patchEtw,

    // Memory
    enableMemoryEvasion,

    // Injection
    findTargets,
    inject,

    // Exfiltration
    listExfilChannels,
    startExfil,
    getExfilSessions,

    // General
    setStealthLevel,
    getStatus,
    setupOpsec,
  };
}
