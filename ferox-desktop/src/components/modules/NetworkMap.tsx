/**
 * NetworkMap - Network Topology Visualization (Placeholder)
 * For demo/training purposes only
 */

import { Globe, Monitor, Server, Router, Wifi, RefreshCw } from 'lucide-react';

interface NetworkMapProps {
  sessionId?: string;
}

export function NetworkMap({ sessionId: _sessionId }: NetworkMapProps) {
  // Mock network stats
  const mockStats = {
    totalNodes: 24,
    activeConnections: 18,
    subnets: 3,
    compromised: 5,
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Globe className="text-cyan-400" size={22} />
            <h2 className="text-lg font-semibold text-text-primary">Network Topology</h2>
            <span className="text-xs bg-cyan-500/20 text-cyan-400 px-2 py-0.5 rounded">PLACEHOLDER</span>
          </div>
          <button
            disabled
            className="px-3 py-1.5 bg-dark-700 border border-dark-600 text-text-muted rounded text-xs font-medium flex items-center gap-1.5 cursor-not-allowed opacity-50"
          >
            <RefreshCw size={12} />
            Refresh
          </button>
        </div>
        <p className="text-xs text-text-muted mt-2">
          Interactive network visualization coming soon
        </p>
      </div>

      {/* Stats Summary */}
      <div className="p-4 border-b border-dark-600 grid grid-cols-4 gap-3">
        <div className="bg-cyan-500/10 border border-cyan-500/20 rounded-lg p-3 text-center">
          <div className="text-2xl font-bold text-cyan-400">{mockStats.totalNodes}</div>
          <div className="text-xs text-text-muted">Total Nodes</div>
        </div>
        <div className="bg-green-500/10 border border-green-500/20 rounded-lg p-3 text-center">
          <div className="text-2xl font-bold text-green-400">{mockStats.activeConnections}</div>
          <div className="text-xs text-text-muted">Active Connections</div>
        </div>
        <div className="bg-blue-500/10 border border-blue-500/20 rounded-lg p-3 text-center">
          <div className="text-2xl font-bold text-blue-400">{mockStats.subnets}</div>
          <div className="text-xs text-text-muted">Subnets</div>
        </div>
        <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-3 text-center">
          <div className="text-2xl font-bold text-red-400">{mockStats.compromised}</div>
          <div className="text-xs text-text-muted">Compromised</div>
        </div>
      </div>

      {/* Placeholder Content */}
      <div className="flex-1 flex items-center justify-center p-8">
        <div className="text-center max-w-md">
          <div className="relative mb-8">
            {/* Mock network diagram */}
            <div className="flex justify-center items-center gap-8">
              <div className="flex flex-col items-center gap-2">
                <div className="w-16 h-16 rounded-full bg-dark-700 border-2 border-cyan-500/30 flex items-center justify-center">
                  <Router size={24} className="text-cyan-400" />
                </div>
                <span className="text-xs text-text-muted">Gateway</span>
              </div>
              <div className="w-16 h-px bg-dark-600" />
              <div className="flex flex-col items-center gap-2">
                <div className="w-16 h-16 rounded-full bg-dark-700 border-2 border-green-500/30 flex items-center justify-center">
                  <Server size={24} className="text-green-400" />
                </div>
                <span className="text-xs text-text-muted">Server</span>
              </div>
              <div className="w-16 h-px bg-dark-600" />
              <div className="flex flex-col items-center gap-2">
                <div className="w-16 h-16 rounded-full bg-dark-700 border-2 border-red-500/30 flex items-center justify-center">
                  <Monitor size={24} className="text-red-400" />
                </div>
                <span className="text-xs text-text-muted">Target</span>
              </div>
            </div>
          </div>

          <Globe size={48} className="mx-auto mb-4 text-cyan-400/30" />
          <h3 className="text-lg font-medium text-text-primary mb-2">Network Visualization Coming Soon</h3>
          <p className="text-sm text-text-muted">
            This module will provide interactive network topology mapping with real-time
            visualization of discovered hosts, connections, and attack paths.
          </p>

          {/* Feature preview */}
          <div className="mt-6 grid grid-cols-2 gap-3 text-left">
            <div className="bg-dark-800 rounded-lg p-3 border border-dark-600">
              <Wifi size={16} className="text-cyan-400 mb-2" />
              <div className="text-xs font-medium text-text-primary">Auto-Discovery</div>
              <div className="text-xs text-text-muted">Automatic network scanning</div>
            </div>
            <div className="bg-dark-800 rounded-lg p-3 border border-dark-600">
              <Server size={16} className="text-green-400 mb-2" />
              <div className="text-xs font-medium text-text-primary">Service Detection</div>
              <div className="text-xs text-text-muted">Identify running services</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default NetworkMap;
