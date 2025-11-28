// ferox-desktop/src/components/modules/opsec/OpsecDashboard.tsx
// Main OPSEC Dashboard Component

import { useState, useEffect } from 'react';
import {
  Shield,
  Eye,
  Cpu,
  Syringe,
  Upload,
  AlertTriangle,
  CheckCircle,
  XCircle,
  Monitor,
  RefreshCw,
} from 'lucide-react';
import { useOpsec } from '../../../hooks/useOpsec';
import { EdrDetectorPanel } from './EdrDetectorPanel';
import { AmsiEtwPanel } from './AmsiEtwPanel';
import { MemoryEvasionPanel } from './MemoryEvasionPanel';
import { EnvDetectorPanel } from './EnvDetectorPanel';
import { InjectionPanel } from './InjectionPanel';
import { ExfilPanel } from './ExfilPanel';
import type { StealthLevel, OpsecStatus } from '../../../types/opsec';

const STEALTH_COLORS: Record<StealthLevel, string> = {
  Normal: 'text-success-text bg-success-soft border-success-border',
  Quiet: 'text-warning-text bg-warning-soft border-warning-border',
  Silent: 'text-[var(--warning-text)] bg-[var(--warning-soft)] border-[var(--warning-border)]',
  Ghost: 'text-danger-text bg-danger-soft border-danger-border',
};

const STEALTH_DESCRIPTIONS: Record<StealthLevel, string> = {
  Normal: 'Standard operation - minimal evasion',
  Quiet: 'Reduced noise - basic evasion enabled',
  Silent: 'Low profile - advanced evasion active',
  Ghost: 'Maximum stealth - all evasion enabled',
};

interface StatusIndicatorProps {
  label: string;
  status: 'success' | 'warning' | 'error' | 'inactive';
  count?: number;
}

function StatusIndicator({ label, status, count }: StatusIndicatorProps) {
  const colors = {
    success: 'text-success-text',
    warning: 'text-warning-text',
    error: 'text-danger-text',
    inactive: 'text-content-tertiary',
  };

  const icons = {
    success: CheckCircle,
    warning: AlertTriangle,
    error: XCircle,
    inactive: Shield,
  };

  const Icon = icons[status];

  return (
    <div className={`flex items-center gap-1.5 ${colors[status]}`}>
      <Icon className="w-4 h-4" />
      <span className="text-sm font-medium">{label}</span>
      {count !== undefined && count > 0 && (
        <span className="px-1.5 py-0.5 text-xs bg-dark-700 rounded">{count}</span>
      )}
    </div>
  );
}

interface OverviewPanelProps {
  status: OpsecStatus | null;
  onRefresh: () => void;
  loading: boolean;
}

function OverviewPanel({ status, onRefresh, loading }: OverviewPanelProps) {
  return (
    <div className="space-y-4">
      {/* Quick Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {/* Security Status Card */}
        <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-lg font-semibold">Security Status</h3>
            <button
              onClick={onRefresh}
              disabled={loading}
              className="p-1 hover:bg-dark-700 rounded transition-colors"
            >
              <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
            </button>
          </div>
          <div className="space-y-3">
            <div className="flex justify-between items-center">
              <span className="text-text-secondary">Stealth Level</span>
              <span
                className={`px-2 py-1 rounded text-sm font-medium ${
                  status ? STEALTH_COLORS[status.stealthLevel] : 'text-content-tertiary'
                }`}
              >
                {status?.stealthLevel || 'Unknown'}
              </span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-text-secondary">Safe to Execute</span>
              <span className={status?.isSafe ? 'text-success-text' : 'text-danger-text'}>
                {status?.isSafe ? 'Yes' : 'No'}
              </span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-text-secondary">Last Scan</span>
              <span className="text-text-muted text-sm">
                {status?.lastScan
                  ? new Date(status.lastScan).toLocaleTimeString()
                  : 'Never'}
              </span>
            </div>
          </div>
        </div>

        {/* Bypass Status Card */}
        <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
          <h3 className="text-lg font-semibold mb-3">Bypass Status</h3>
          <div className="space-y-3">
            <div className="flex justify-between items-center">
              <span className="text-text-secondary">AMSI Bypass</span>
              <span
                className={`px-2 py-1 rounded text-sm ${
                  status?.amsiBypass
                    ? 'text-success-text bg-success-soft'
                    : 'text-content-tertiary bg-dark-700'
                }`}
              >
                {status?.amsiBypass ? 'Active' : 'Inactive'}
              </span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-text-secondary">ETW Patched</span>
              <span
                className={`px-2 py-1 rounded text-sm ${
                  status?.etwPatched
                    ? 'text-success-text bg-success-soft'
                    : 'text-content-tertiary bg-dark-700'
                }`}
              >
                {status?.etwPatched ? 'Yes' : 'No'}
              </span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-text-secondary">Memory Protected</span>
              <span
                className={`px-2 py-1 rounded text-sm ${
                  status?.memoryProtected
                    ? 'text-success-text bg-success-soft'
                    : 'text-content-tertiary bg-dark-700'
                }`}
              >
                {status?.memoryProtected ? 'Yes' : 'No'}
              </span>
            </div>
          </div>
        </div>

        {/* Environment Card */}
        <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
          <h3 className="text-lg font-semibold mb-3">Environment</h3>
          <div className="space-y-3">
            <div className="flex justify-between items-center">
              <span className="text-text-secondary">VM Detected</span>
              <span
                className={
                  status?.vmDetected ? 'text-warning-text' : 'text-success-text'
                }
              >
                {status?.vmDetected || 'None'}
              </span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-text-secondary">Sandbox</span>
              <span
                className={
                  status?.sandboxDetected ? 'text-danger-text' : 'text-success-text'
                }
              >
                {status?.sandboxDetected || 'None'}
              </span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-text-secondary">EDR Products</span>
              <span
                className={
                  status?.edrDetected?.length ? 'text-warning-text' : 'text-success-text'
                }
              >
                {status?.edrDetected?.length || 0} detected
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* EDR Detections */}
      {status?.edrDetected && status.edrDetected.length > 0 && (
        <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
          <h3 className="text-lg font-semibold mb-3 flex items-center gap-2">
            <AlertTriangle className="w-5 h-5 text-warning-text" />
            Detected Security Products
          </h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
            {status.edrDetected.map((edr, i) => (
              <div
                key={i}
                className="flex items-center justify-between p-2 bg-dark-700/50 rounded"
              >
                <div>
                  <p className="font-medium">{edr.edrType}</p>
                  <p className="text-xs text-text-muted">
                    Confidence: {(edr.confidence * 100).toFixed(0)}%
                  </p>
                </div>
                <div
                  className={`px-2 py-1 rounded text-xs ${
                    edr.threatLevel > 7
                      ? 'bg-danger-soft text-danger-text'
                      : edr.threatLevel > 4
                      ? 'bg-warning-soft text-warning-text'
                      : 'bg-success-soft text-success-text'
                  }`}
                >
                  Threat: {edr.threatLevel}/10
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Quick Actions */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h3 className="text-lg font-semibold mb-3">Quick Actions</h3>
        <div className="flex flex-wrap gap-2">
          <button className="px-4 py-2 bg-cyan-600 hover:bg-cyan-500 rounded-lg font-medium transition-colors flex items-center gap-2">
            <Eye className="w-4 h-4" />
            Full Scan
          </button>
          <button className="px-4 py-2 bg-dark-700 hover:bg-dark-600 rounded-lg font-medium transition-colors flex items-center gap-2 border border-dark-600">
            <Shield className="w-4 h-4" />
            Enable All Bypasses
          </button>
          <button className="px-4 py-2 bg-dark-700 hover:bg-dark-600 rounded-lg font-medium transition-colors flex items-center gap-2 border border-dark-600">
            <Monitor className="w-4 h-4" />
            Check Environment
          </button>
        </div>
      </div>
    </div>
  );
}

export function OpsecDashboard() {
  const { status, loading, error, getStatus, setStealthLevel } = useOpsec();
  const [activePanel, setActivePanel] = useState<string>('overview');

  useEffect(() => {
    getStatus().catch(() => {
      // Silently handle initial load errors
    });
  }, [getStatus]);

  const panels = [
    { id: 'overview', label: 'Overview', icon: Shield },
    { id: 'edr', label: 'EDR Detection', icon: Eye },
    { id: 'amsi-etw', label: 'AMSI/ETW', icon: Shield },
    { id: 'memory', label: 'Memory Evasion', icon: Cpu },
    { id: 'environment', label: 'VM/Sandbox', icon: Monitor },
    { id: 'injection', label: 'Injection', icon: Syringe },
    { id: 'exfil', label: 'Exfiltration', icon: Upload },
  ];

  return (
    <div className="flex flex-col h-full bg-dark-900 text-text-primary">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center gap-3">
          <Shield className="w-6 h-6 text-cyan-400" />
          <h1 className="text-xl font-bold">OPSEC Dashboard</h1>
        </div>

        {/* Stealth Level Selector */}
        <div className="flex items-center gap-3">
          <span className="text-sm text-text-secondary">Stealth Level:</span>
          <div className="relative group">
            <select
              value={status?.stealthLevel || 'Silent'}
              onChange={(e) => setStealthLevel(e.target.value as StealthLevel)}
              className={`px-3 py-1.5 rounded-lg border bg-dark-700 cursor-pointer
                appearance-none pr-8 font-medium
                ${status ? STEALTH_COLORS[status.stealthLevel] : 'border-dark-600'}`}
            >
              <option value="Normal">Normal</option>
              <option value="Quiet">Quiet</option>
              <option value="Silent">Silent</option>
              <option value="Ghost">Ghost</option>
            </select>
            <div className="absolute hidden group-hover:block top-full right-0 mt-2 p-2 bg-dark-700 border border-dark-600 rounded-lg text-xs text-text-secondary whitespace-nowrap z-10">
              {status && STEALTH_DESCRIPTIONS[status.stealthLevel]}
            </div>
          </div>
        </div>
      </div>

      {/* Status Bar */}
      <div className="flex items-center gap-4 px-4 py-2 bg-dark-800/50 border-b border-dark-600">
        <StatusIndicator
          label="AMSI"
          status={status?.amsiBypass ? 'success' : 'inactive'}
        />
        <StatusIndicator
          label="ETW"
          status={status?.etwPatched ? 'success' : 'inactive'}
        />
        <StatusIndicator
          label="EDR"
          status={
            status?.edrDetected?.length
              ? status.edrDetected.length > 2
                ? 'error'
                : 'warning'
              : 'success'
          }
          count={status?.edrDetected?.length}
        />
        <StatusIndicator
          label="Environment"
          status={status?.isSafe ? 'success' : 'error'}
        />
        <StatusIndicator
          label="Memory"
          status={status?.memoryProtected ? 'success' : 'inactive'}
        />
      </div>

      {/* Navigation */}
      <div className="flex border-b border-dark-600 overflow-x-auto bg-dark-800/30">
        {panels.map(({ id, label, icon: Icon }) => (
          <button
            key={id}
            onClick={() => setActivePanel(id)}
            className={`flex items-center gap-2 px-4 py-3 text-sm font-medium
              transition-colors whitespace-nowrap
              ${
                activePanel === id
                  ? 'text-cyan-400 border-b-2 border-cyan-400 bg-dark-800/50'
                  : 'text-text-secondary hover:text-text-primary hover:bg-dark-800/30'
              }`}
          >
            <Icon className="w-4 h-4" />
            {label}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-auto p-4">
        {loading && (
          <div className="flex items-center justify-center h-32">
            <div className="animate-spin rounded-full h-8 w-8 border-2 border-info border-t-transparent" />
          </div>
        )}

        {error && (
          <div className="bg-danger-soft border border-danger-border rounded-lg p-4 mb-4">
            <p className="text-danger-text">{error}</p>
          </div>
        )}

        {!loading && (
          <>
            {activePanel === 'overview' && (
              <OverviewPanel
                status={status}
                onRefresh={() => getStatus()}
                loading={loading}
              />
            )}
            {activePanel === 'edr' && <EdrDetectorPanel />}
            {activePanel === 'amsi-etw' && <AmsiEtwPanel />}
            {activePanel === 'memory' && <MemoryEvasionPanel />}
            {activePanel === 'environment' && <EnvDetectorPanel />}
            {activePanel === 'injection' && <InjectionPanel />}
            {activePanel === 'exfil' && <ExfilPanel />}
          </>
        )}
      </div>
    </div>
  );
}

export default OpsecDashboard;
