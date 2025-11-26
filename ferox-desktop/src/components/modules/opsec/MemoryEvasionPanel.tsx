// ferox-desktop/src/components/modules/opsec/MemoryEvasionPanel.tsx
// Memory Evasion Panel

import { useState } from 'react';
import { Cpu, Shield, Lock, Eye, Play, CheckCircle, XCircle, Info } from 'lucide-react';
import { useOpsec } from '../../../hooks/useOpsec';
import type { MemoryEvasionResult } from '../../../types/opsec';

const MEMORY_TECHNIQUES = [
  {
    id: 'HeapEncrypt',
    name: 'Heap Encryption',
    description: 'Encrypts sensitive data on the heap when not in use',
    icon: Lock,
    mitre: 'T1027',
    effectiveness: 8,
  },
  {
    id: 'StackObfuscate',
    name: 'Stack Obfuscation',
    description: 'Obfuscates return addresses and stack frames',
    icon: Shield,
    mitre: 'T1027',
    effectiveness: 7,
  },
  {
    id: 'ModuleHide',
    name: 'Module Hiding',
    description: 'Unlinks modules from PEB to hide from memory scanners',
    icon: Eye,
    mitre: 'T1055.012',
    effectiveness: 9,
  },
  {
    id: 'PeHeader',
    name: 'PE Header Wipe',
    description: 'Zeros PE headers in memory to prevent identification',
    icon: Cpu,
    mitre: 'T1070',
    effectiveness: 8,
  },
];

export function MemoryEvasionPanel() {
  const { enableMemoryEvasion, loading, status } = useOpsec();
  const [selectedTechnique] = useState<string | null>(null);
  const [results, setResults] = useState<Map<string, MemoryEvasionResult>>(new Map());

  const handleEnable = async (techniqueId: string) => {
    try {
      const result = await enableMemoryEvasion(techniqueId);
      setResults((prev) => new Map(prev).set(techniqueId, result));
    } catch (e) {
      console.error('Memory evasion failed:', e);
    }
  };

  const handleEnableAll = async () => {
    for (const technique of MEMORY_TECHNIQUES) {
      await handleEnable(technique.id);
    }
  };

  return (
    <div className="space-y-6">
      {/* Status Overview */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h3 className="text-lg font-semibold">Memory Evasion</h3>
            <p className="text-sm text-text-muted">
              Protect against memory forensics (T1070, T1027)
            </p>
          </div>
          <div
            className={`px-3 py-1 rounded-lg text-sm font-medium ${
              status?.memoryProtected
                ? 'bg-green-400/10 text-green-400'
                : 'bg-dark-700 text-text-muted'
            }`}
          >
            {status?.memoryProtected ? 'Protected' : 'Not Protected'}
          </div>
        </div>

        {/* Quick Enable All */}
        <button
          onClick={handleEnableAll}
          disabled={loading}
          className="w-full flex items-center justify-center gap-2 px-4 py-3 bg-cyan-600 hover:bg-cyan-500
            rounded-lg font-medium disabled:opacity-50 transition-colors"
        >
          <Shield className="w-4 h-4" />
          {loading ? 'Enabling...' : 'Enable All Protections'}
        </button>
      </div>

      {/* Technique Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {MEMORY_TECHNIQUES.map((technique) => {
          const Icon = technique.icon;
          const result = results.get(technique.id);
          const isEnabled = result?.success;

          return (
            <div
              key={technique.id}
              className={`bg-dark-800 rounded-lg p-4 border transition-colors ${
                isEnabled
                  ? 'border-green-400/50'
                  : selectedTechnique === technique.id
                  ? 'border-cyan-400'
                  : 'border-dark-600'
              }`}
            >
              <div className="flex items-start justify-between mb-3">
                <div className="flex items-center gap-3">
                  <div
                    className={`p-2 rounded-lg ${
                      isEnabled ? 'bg-green-400/10' : 'bg-dark-700'
                    }`}
                  >
                    <Icon
                      className={`w-5 h-5 ${
                        isEnabled ? 'text-green-400' : 'text-text-secondary'
                      }`}
                    />
                  </div>
                  <div>
                    <h4 className="font-medium">{technique.name}</h4>
                    <p className="text-xs text-text-muted">{technique.mitre}</p>
                  </div>
                </div>
                {isEnabled && <CheckCircle className="w-5 h-5 text-green-400" />}
              </div>

              <p className="text-sm text-text-secondary mb-3">
                {technique.description}
              </p>

              {/* Effectiveness Bar */}
              <div className="mb-3">
                <div className="flex items-center justify-between text-xs mb-1">
                  <span className="text-text-muted">Effectiveness</span>
                  <span className="text-text-secondary">
                    {technique.effectiveness}/10
                  </span>
                </div>
                <div className="h-1.5 bg-dark-700 rounded-full overflow-hidden">
                  <div
                    className="h-full bg-cyan-400 rounded-full transition-all"
                    style={{ width: `${technique.effectiveness * 10}%` }}
                  />
                </div>
              </div>

              {/* Result or Enable Button */}
              {result ? (
                <div
                  className={`p-2 rounded-lg text-sm ${
                    result.success
                      ? 'bg-green-400/10 text-green-400'
                      : 'bg-red-400/10 text-red-400'
                  }`}
                >
                  <div className="flex items-center gap-2">
                    {result.success ? (
                      <CheckCircle className="w-4 h-4" />
                    ) : (
                      <XCircle className="w-4 h-4" />
                    )}
                    {result.message}
                  </div>
                  {result.regionsProtected > 0 && (
                    <p className="text-xs mt-1 text-text-muted">
                      {result.regionsProtected} memory regions protected
                    </p>
                  )}
                </div>
              ) : (
                <button
                  onClick={() => handleEnable(technique.id)}
                  disabled={loading}
                  className="w-full flex items-center justify-center gap-2 px-3 py-2 bg-dark-700 hover:bg-dark-600
                    rounded-lg text-sm font-medium disabled:opacity-50 transition-colors border border-dark-600"
                >
                  <Play className="w-3 h-3" />
                  Enable
                </button>
              )}
            </div>
          );
        })}
      </div>

      {/* Memory Regions Info */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h4 className="font-medium mb-3 flex items-center gap-2">
          <Info className="w-4 h-4 text-cyan-400" />
          Memory Protection Details
        </h4>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
          <div className="p-3 bg-dark-700/50 rounded-lg">
            <p className="text-xl font-bold text-cyan-400">
              {Array.from(results.values()).filter((r) => r.success).length}
            </p>
            <p className="text-xs text-text-muted">Techniques Active</p>
          </div>
          <div className="p-3 bg-dark-700/50 rounded-lg">
            <p className="text-xl font-bold">
              {Array.from(results.values()).reduce(
                (acc, r) => acc + (r.regionsProtected || 0),
                0
              )}
            </p>
            <p className="text-xs text-text-muted">Regions Protected</p>
          </div>
          <div className="p-3 bg-dark-700/50 rounded-lg">
            <p className="text-xl font-bold">0</p>
            <p className="text-xs text-text-muted">Hooks Detected</p>
          </div>
          <div className="p-3 bg-dark-700/50 rounded-lg">
            <p className="text-xl font-bold text-green-400">Clean</p>
            <p className="text-xs text-text-muted">Memory Status</p>
          </div>
        </div>
      </div>

      {/* Tips */}
      <div className="bg-dark-800/50 rounded-lg p-4 border border-dark-600">
        <h4 className="font-medium mb-2 flex items-center gap-2">
          <Info className="w-4 h-4 text-cyan-400" />
          Memory Evasion Tips
        </h4>
        <ul className="text-sm text-text-secondary space-y-1">
          <li>• Enable Heap Encryption for sensitive string storage</li>
          <li>• Module Hiding prevents detection by memory scanners</li>
          <li>• PE Header Wipe removes identifying signatures</li>
          <li>• Combine multiple techniques for maximum protection</li>
        </ul>
      </div>
    </div>
  );
}
