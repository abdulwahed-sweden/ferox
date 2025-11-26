// ferox-desktop/src/components/modules/opsec/EdrDetectorPanel.tsx
// EDR Detection Panel

import { useState } from 'react';
import { Search, AlertTriangle, CheckCircle, Info } from 'lucide-react';
import { useOpsec } from '../../../hooks/useOpsec';
import type { EdrDetectionResult, EdrScanOptions } from '../../../types/opsec';

const SCAN_DEPTHS = [
  { id: 'quick', label: 'Quick Scan', description: 'Process names only (~1s)' },
  { id: 'standard', label: 'Standard Scan', description: 'Process + services (~3s)' },
  { id: 'deep', label: 'Deep Scan', description: 'Full system analysis (~10s)' },
] as const;

const EDR_INFO: Record<string, { description: string; mitigation: string }> = {
  WindowsDefender: {
    description: 'Microsoft built-in AV with cloud intelligence',
    mitigation: 'AMSI bypass + ETW patch recommended',
  },
  CrowdStrike: {
    description: 'Cloud-native EDR with kernel-level monitoring',
    mitigation: 'Direct syscalls + memory evasion required',
  },
  SentinelOne: {
    description: 'AI-powered EDR with behavioral analysis',
    mitigation: 'Ghost mode + delayed execution recommended',
  },
  CarbonBlack: {
    description: 'VMware EDR with process monitoring',
    mitigation: 'Process injection + LOLBins recommended',
  },
  Cylance: {
    description: 'AI-based AV with static analysis',
    mitigation: 'Code obfuscation + packing effective',
  },
};

export function EdrDetectorPanel() {
  const { scanEdr, loading } = useOpsec();
  const [results, setResults] = useState<EdrDetectionResult | null>(null);
  const [scanOptions, setScanOptions] = useState<EdrScanOptions>({
    depth: 'standard',
    safeMode: true,
  });
  const [expandedEdr, setExpandedEdr] = useState<string | null>(null);

  const handleScan = async () => {
    try {
      const result = await scanEdr(scanOptions);
      setResults(result);
    } catch (e) {
      console.error('EDR scan failed:', e);
    }
  };

  return (
    <div className="space-y-6">
      {/* Scan Configuration */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h3 className="text-lg font-semibold mb-4">EDR Detection Scan</h3>

        {/* Scan Depth Selection */}
        <div className="mb-4">
          <label className="text-sm text-text-secondary mb-2 block">
            Scan Depth
          </label>
          <div className="flex gap-2">
            {SCAN_DEPTHS.map(({ id, label, description }) => (
              <button
                key={id}
                onClick={() =>
                  setScanOptions((prev) => ({ ...prev, depth: id }))
                }
                className={`flex-1 p-3 rounded-lg border transition-colors ${
                  scanOptions.depth === id
                    ? 'border-cyan-400 bg-cyan-400/10 text-cyan-400'
                    : 'border-dark-600 hover:border-dark-500'
                }`}
              >
                <p className="font-medium">{label}</p>
                <p className="text-xs text-text-muted mt-1">{description}</p>
              </button>
            ))}
          </div>
        </div>

        {/* Safe Mode Toggle */}
        <div className="flex items-center justify-between mb-4 p-3 bg-dark-700/50 rounded-lg">
          <div>
            <p className="font-medium">Safe Mode</p>
            <p className="text-xs text-text-muted">
              Avoid detection during scan (slower but safer)
            </p>
          </div>
          <button
            onClick={() =>
              setScanOptions((prev) => ({ ...prev, safeMode: !prev.safeMode }))
            }
            className={`w-12 h-6 rounded-full transition-colors ${
              scanOptions.safeMode ? 'bg-cyan-600' : 'bg-dark-600'
            }`}
          >
            <div
              className={`w-5 h-5 bg-white rounded-full transition-transform ${
                scanOptions.safeMode ? 'translate-x-6' : 'translate-x-0.5'
              }`}
            />
          </button>
        </div>

        {/* Scan Button */}
        <button
          onClick={handleScan}
          disabled={loading}
          className="w-full flex items-center justify-center gap-2 px-4 py-3 bg-cyan-600 hover:bg-cyan-500
            rounded-lg font-medium disabled:opacity-50 transition-colors"
        >
          <Search className="w-4 h-4" />
          {loading ? 'Scanning...' : 'Scan for EDR/AV'}
        </button>
      </div>

      {/* Results */}
      {results && (
        <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold">Scan Results</h3>
            <span className="text-sm text-text-muted">
              Completed in {results.scanTimeMs}ms
            </span>
          </div>

          {/* Summary */}
          <div className="grid grid-cols-3 gap-4 mb-4">
            <div className="p-3 bg-dark-700/50 rounded-lg text-center">
              <p className="text-2xl font-bold">{results.detectedEdrs.length}</p>
              <p className="text-xs text-text-muted">Products Detected</p>
            </div>
            <div className="p-3 bg-dark-700/50 rounded-lg text-center">
              <p
                className={`text-2xl font-bold ${
                  results.totalThreatLevel > 7
                    ? 'text-red-400'
                    : results.totalThreatLevel > 4
                    ? 'text-yellow-400'
                    : 'text-green-400'
                }`}
              >
                {results.totalThreatLevel}/10
              </p>
              <p className="text-xs text-text-muted">Total Threat Level</p>
            </div>
            <div className="p-3 bg-dark-700/50 rounded-lg text-center">
              <p className="text-2xl font-bold text-cyan-400">
                {results.recommendedStealth}
              </p>
              <p className="text-xs text-text-muted">Recommended Stealth</p>
            </div>
          </div>

          {/* Detection List */}
          {results.detectedEdrs.length === 0 ? (
            <div className="flex items-center gap-3 p-4 bg-green-500/10 border border-green-500/30 rounded-lg">
              <CheckCircle className="w-6 h-6 text-green-400" />
              <div>
                <p className="font-medium text-green-400">No EDR Detected</p>
                <p className="text-sm text-text-muted">
                  Environment appears clean for operation
                </p>
              </div>
            </div>
          ) : (
            <div className="space-y-2">
              {results.detectedEdrs.map((edr, i) => (
                <div
                  key={i}
                  className="border border-dark-600 rounded-lg overflow-hidden"
                >
                  <button
                    onClick={() =>
                      setExpandedEdr(expandedEdr === edr.edrType ? null : edr.edrType)
                    }
                    className="w-full flex items-center justify-between p-3 hover:bg-dark-700/50 transition-colors"
                  >
                    <div className="flex items-center gap-3">
                      <AlertTriangle
                        className={`w-5 h-5 ${
                          edr.threatLevel > 7
                            ? 'text-red-400'
                            : edr.threatLevel > 4
                            ? 'text-yellow-400'
                            : 'text-green-400'
                        }`}
                      />
                      <div className="text-left">
                        <p className="font-medium">{edr.edrType}</p>
                        <p className="text-xs text-text-muted">
                          Confidence: {(edr.confidence * 100).toFixed(0)}%
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center gap-3">
                      <span
                        className={`px-2 py-1 rounded text-xs font-medium ${
                          edr.threatLevel > 7
                            ? 'bg-red-400/10 text-red-400'
                            : edr.threatLevel > 4
                            ? 'bg-yellow-400/10 text-yellow-400'
                            : 'bg-green-400/10 text-green-400'
                        }`}
                      >
                        Threat: {edr.threatLevel}/10
                      </span>
                      <Info className="w-4 h-4 text-text-muted" />
                    </div>
                  </button>

                  {expandedEdr === edr.edrType && (
                    <div className="p-3 bg-dark-700/30 border-t border-dark-600">
                      {EDR_INFO[edr.edrType] && (
                        <div className="space-y-2 mb-3">
                          <p className="text-sm text-text-secondary">
                            {EDR_INFO[edr.edrType].description}
                          </p>
                          <p className="text-sm">
                            <span className="text-cyan-400">Mitigation:</span>{' '}
                            {EDR_INFO[edr.edrType].mitigation}
                          </p>
                        </div>
                      )}
                      <div>
                        <p className="text-xs text-text-muted mb-1">Evidence:</p>
                        <ul className="text-xs text-text-secondary space-y-1">
                          {edr.evidence.map((e, j) => (
                            <li key={j} className="flex items-center gap-1">
                              <span className="w-1 h-1 bg-text-muted rounded-full" />
                              {e}
                            </li>
                          ))}
                        </ul>
                      </div>
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {/* Tips */}
      <div className="bg-dark-800/50 rounded-lg p-4 border border-dark-600">
        <h4 className="font-medium mb-2 flex items-center gap-2">
          <Info className="w-4 h-4 text-cyan-400" />
          Detection Tips
        </h4>
        <ul className="text-sm text-text-secondary space-y-1">
          <li>• Use Safe Mode in monitored environments</li>
          <li>• Deep scan may trigger some EDR alerts</li>
          <li>• Match stealth level to detected threat level</li>
          <li>• Re-scan after any system changes</li>
        </ul>
      </div>
    </div>
  );
}
