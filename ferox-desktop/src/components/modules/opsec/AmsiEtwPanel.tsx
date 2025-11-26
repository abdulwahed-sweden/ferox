// ferox-desktop/src/components/modules/opsec/AmsiEtwPanel.tsx
// AMSI/ETW Bypass Panel

import { useState } from 'react';
import { Shield, CheckCircle, XCircle, Play, Info, AlertTriangle } from 'lucide-react';
import { useOpsec } from '../../../hooks/useOpsec';
import type { AmsiBypassResult, EtwPatchResult } from '../../../types/opsec';

const AMSI_TECHNIQUES = [
  {
    id: 'PatchScanBuffer',
    name: 'Patch AmsiScanBuffer',
    description: 'Patches the AmsiScanBuffer function to always return clean',
    risk: 'low',
    mitre: 'T1562.001',
  },
  {
    id: 'MemoryPatch',
    name: 'Memory Patch',
    description: 'Patches AMSI.dll in memory to disable scanning',
    risk: 'medium',
    mitre: 'T1562.001',
  },
  {
    id: 'ReflectiveUnhook',
    name: 'Reflective Unhook',
    description: 'Reloads clean AMSI from disk to remove hooks',
    risk: 'high',
    mitre: 'T1562.001',
  },
  {
    id: 'Amsi2',
    name: 'AMSI v2 Bypass',
    description: 'Targets newer AMSI implementation in .NET',
    risk: 'medium',
    mitre: 'T1562.001',
  },
];

const ETW_PROVIDERS = [
  {
    id: 'PowerShell',
    name: 'PowerShell',
    description: 'Microsoft-Windows-PowerShell provider',
    critical: true,
  },
  {
    id: 'DotNet',
    name: '.NET Runtime',
    description: 'Microsoft-Windows-DotNETRuntime provider',
    critical: true,
  },
  {
    id: 'SecurityAuditing',
    name: 'Security Auditing',
    description: 'Microsoft-Windows-Security-Auditing',
    critical: false,
  },
  {
    id: 'ThreatIntel',
    name: 'Threat Intelligence',
    description: 'Microsoft-Windows-Threat-Intelligence',
    critical: true,
  },
];

export function AmsiEtwPanel() {
  const { bypassAmsi, patchEtw, loading, status } = useOpsec();
  const [amsiTechnique, setAmsiTechnique] = useState('PatchScanBuffer');
  const [amsiResult, setAmsiResult] = useState<AmsiBypassResult | null>(null);
  const [etwProviders, setEtwProviders] = useState<string[]>(['PowerShell', 'DotNet']);
  const [etwResult, setEtwResult] = useState<EtwPatchResult | null>(null);

  const handleAmsiBypass = async () => {
    try {
      const result = await bypassAmsi({ technique: amsiTechnique as any });
      setAmsiResult(result);
    } catch (e) {
      console.error('AMSI bypass failed:', e);
    }
  };

  const handleEtwPatch = async () => {
    try {
      const result = await patchEtw({ providers: etwProviders as any[] });
      setEtwResult(result);
    } catch (e) {
      console.error('ETW patch failed:', e);
    }
  };

  const toggleEtwProvider = (providerId: string) => {
    setEtwProviders((prev) =>
      prev.includes(providerId)
        ? prev.filter((p) => p !== providerId)
        : [...prev, providerId]
    );
  };

  return (
    <div className="space-y-6">
      {/* AMSI Bypass Section */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h3 className="text-lg font-semibold">AMSI Bypass</h3>
            <p className="text-sm text-text-muted">
              Antimalware Scan Interface bypass (T1562.001)
            </p>
          </div>
          <div
            className={`px-3 py-1 rounded-lg text-sm font-medium ${
              status?.amsiBypass
                ? 'bg-green-400/10 text-green-400'
                : 'bg-dark-700 text-text-muted'
            }`}
          >
            {status?.amsiBypass ? 'Active' : 'Inactive'}
          </div>
        </div>

        {/* Technique Selection */}
        <div className="space-y-2 mb-4">
          <label className="text-sm text-text-secondary">Select Technique</label>
          {AMSI_TECHNIQUES.map((technique) => (
            <button
              key={technique.id}
              onClick={() => setAmsiTechnique(technique.id)}
              className={`w-full p-3 rounded-lg border text-left transition-colors ${
                amsiTechnique === technique.id
                  ? 'border-cyan-400 bg-cyan-400/10'
                  : 'border-dark-600 hover:border-dark-500'
              }`}
            >
              <div className="flex items-center justify-between">
                <span className="font-medium">{technique.name}</span>
                <div className="flex items-center gap-2">
                  <span
                    className={`px-2 py-0.5 rounded text-xs ${
                      technique.risk === 'low'
                        ? 'bg-green-400/10 text-green-400'
                        : technique.risk === 'medium'
                        ? 'bg-yellow-400/10 text-yellow-400'
                        : 'bg-red-400/10 text-red-400'
                    }`}
                  >
                    {technique.risk} risk
                  </span>
                  <span className="text-xs text-text-muted">{technique.mitre}</span>
                </div>
              </div>
              <p className="text-xs text-text-muted mt-1">{technique.description}</p>
            </button>
          ))}
        </div>

        {/* Execute Button */}
        <button
          onClick={handleAmsiBypass}
          disabled={loading || status?.amsiBypass}
          className="w-full flex items-center justify-center gap-2 px-4 py-3 bg-cyan-600 hover:bg-cyan-500
            rounded-lg font-medium disabled:opacity-50 transition-colors"
        >
          <Play className="w-4 h-4" />
          {loading ? 'Executing...' : 'Execute Bypass'}
        </button>

        {/* Result */}
        {amsiResult && (
          <div
            className={`mt-4 p-3 rounded-lg border ${
              amsiResult.success
                ? 'bg-green-400/10 border-green-400/30'
                : 'bg-red-400/10 border-red-400/30'
            }`}
          >
            <div className="flex items-center gap-2">
              {amsiResult.success ? (
                <CheckCircle className="w-5 h-5 text-green-400" />
              ) : (
                <XCircle className="w-5 h-5 text-red-400" />
              )}
              <span className="font-medium">
                {amsiResult.success ? 'Bypass Successful' : 'Bypass Failed'}
              </span>
            </div>
            <p className="text-sm text-text-secondary mt-1">{amsiResult.message}</p>
            {amsiResult.patchedAddress && (
              <p className="text-xs text-text-muted mt-1">
                Patched at: {amsiResult.patchedAddress}
              </p>
            )}
          </div>
        )}
      </div>

      {/* ETW Patch Section */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h3 className="text-lg font-semibold">ETW Patch</h3>
            <p className="text-sm text-text-muted">
              Event Tracing for Windows bypass (T1562.006)
            </p>
          </div>
          <div
            className={`px-3 py-1 rounded-lg text-sm font-medium ${
              status?.etwPatched
                ? 'bg-green-400/10 text-green-400'
                : 'bg-dark-700 text-text-muted'
            }`}
          >
            {status?.etwPatched ? 'Patched' : 'Not Patched'}
          </div>
        </div>

        {/* Provider Selection */}
        <div className="space-y-2 mb-4">
          <label className="text-sm text-text-secondary">Select Providers to Patch</label>
          <div className="grid grid-cols-2 gap-2">
            {ETW_PROVIDERS.map((provider) => (
              <button
                key={provider.id}
                onClick={() => toggleEtwProvider(provider.id)}
                className={`p-3 rounded-lg border text-left transition-colors ${
                  etwProviders.includes(provider.id)
                    ? 'border-cyan-400 bg-cyan-400/10'
                    : 'border-dark-600 hover:border-dark-500'
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="font-medium text-sm">{provider.name}</span>
                  {provider.critical && (
                    <AlertTriangle className="w-3 h-3 text-yellow-400" />
                  )}
                </div>
                <p className="text-xs text-text-muted mt-1">{provider.description}</p>
              </button>
            ))}
          </div>
        </div>

        {/* Execute Button */}
        <button
          onClick={handleEtwPatch}
          disabled={loading || etwProviders.length === 0}
          className="w-full flex items-center justify-center gap-2 px-4 py-3 bg-cyan-600 hover:bg-cyan-500
            rounded-lg font-medium disabled:opacity-50 transition-colors"
        >
          <Shield className="w-4 h-4" />
          {loading ? 'Patching...' : `Patch ${etwProviders.length} Provider(s)`}
        </button>

        {/* Result */}
        {etwResult && (
          <div
            className={`mt-4 p-3 rounded-lg border ${
              etwResult.success
                ? 'bg-green-400/10 border-green-400/30'
                : 'bg-red-400/10 border-red-400/30'
            }`}
          >
            <div className="flex items-center gap-2">
              {etwResult.success ? (
                <CheckCircle className="w-5 h-5 text-green-400" />
              ) : (
                <XCircle className="w-5 h-5 text-red-400" />
              )}
              <span className="font-medium">
                {etwResult.success ? 'Patch Successful' : 'Patch Failed'}
              </span>
            </div>
            <p className="text-sm text-text-secondary mt-1">{etwResult.message}</p>
            {etwResult.providersPatched.length > 0 && (
              <div className="mt-2">
                <p className="text-xs text-text-muted">Patched providers:</p>
                <div className="flex flex-wrap gap-1 mt-1">
                  {etwResult.providersPatched.map((p) => (
                    <span key={p} className="px-2 py-0.5 bg-dark-700 rounded text-xs">
                      {p}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}
      </div>

      {/* Warning */}
      <div className="bg-yellow-400/10 border border-yellow-400/30 rounded-lg p-4">
        <div className="flex items-start gap-3">
          <Info className="w-5 h-5 text-yellow-400 mt-0.5" />
          <div>
            <p className="font-medium text-yellow-400">Security Notice</p>
            <p className="text-sm text-text-secondary mt-1">
              These bypasses modify system behavior and may be detected by advanced
              EDR solutions. Use appropriate stealth level and verify environment
              before execution.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
