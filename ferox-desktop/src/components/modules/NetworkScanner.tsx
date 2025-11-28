/**
 * NetworkScanner - Simulated Network Scan Results
 * For demo/training purposes only - no real network access
 */

import { useState } from 'react';
import { Radar, Play, RefreshCw, Server, Wifi, Shield, Clock } from 'lucide-react';
import { clsx } from 'clsx';
import toast from 'react-hot-toast';
import { simulateNetworkScan } from '../../lib/tauri';
import type { SimulatedHost, NetworkScanResult } from '../../types';

interface NetworkScannerProps {
  sessionId: string;
}

export function NetworkScanner({ sessionId }: NetworkScannerProps) {
  const [isScanning, setIsScanning] = useState(false);
  const [scanResult, setScanResult] = useState<NetworkScanResult | null>(null);
  const [hosts, setHosts] = useState<SimulatedHost[]>([]);
  const [selectedHost, setSelectedHost] = useState<SimulatedHost | null>(null);
  const [scanRange, setScanRange] = useState('192.168.1.0/24');

  const handleScan = async () => {
    setIsScanning(true);
    setHosts([]);
    setSelectedHost(null);
    setScanResult(null);

    try {
      toast.loading('Scanning network...', { id: 'scan' });
      const result = await simulateNetworkScan(scanRange, sessionId);

      // Simulate progressive discovery for better UX
      for (let i = 0; i < result.hosts.length; i++) {
        await new Promise(r => setTimeout(r, 200 + Math.random() * 300));
        setHosts(prev => [...prev, result.hosts[i]]);
      }

      setScanResult(result);
      toast.success(`Scan complete: ${result.hosts_up} hosts up`, { id: 'scan' });
    } catch (error) {
      console.error('Scan failed:', error);
      toast.error('Scan failed', { id: 'scan' });
    } finally {
      setIsScanning(false);
    }
  };

  const getPortStateColor = (state: string) => {
    switch (state) {
      case 'open': return 'text-success-text';
      case 'closed': return 'text-danger-text';
      case 'filtered': return 'text-warning-text';
      default: return 'text-text-muted';
    }
  };

  const getLatencyColor = (ms: number) => {
    if (ms < 10) return 'text-success-text';
    if (ms < 50) return 'text-warning-text';
    return 'text-danger-text';
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center gap-2">
          <Radar className="text-info-text" size={20} />
          <h2 className="text-lg font-semibold text-text-primary">Network Scanner</h2>
          <span className="text-xs bg-info-soft text-info-text px-2 py-0.5 rounded">SIMULATION</span>
        </div>
        <p className="text-xs text-text-muted mt-1">Simulated network discovery for demo/training</p>
      </div>

      {/* Controls */}
      <div className="p-4 border-b border-dark-600 flex items-center gap-4">
        <div className="flex-1">
          <input
            type="text"
            value={scanRange}
            onChange={e => setScanRange(e.target.value)}
            placeholder="192.168.1.0/24"
            className="w-full max-w-xs px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary focus:border-blue-400/50 focus:outline-none"
          />
        </div>
        <button
          onClick={handleScan}
          disabled={isScanning}
          className={clsx(
            'px-4 py-2 rounded font-medium text-sm flex items-center gap-2 transition-colors',
            isScanning
              ? 'bg-dark-600 text-text-muted cursor-not-allowed'
              : 'bg-blue-500 text-white hover:bg-blue-600'
          )}
        >
          {isScanning ? (
            <>
              <RefreshCw size={16} className="animate-spin" />
              Scanning...
            </>
          ) : (
            <>
              <Play size={16} />
              Start Scan
            </>
          )}
        </button>
        {scanResult && (
          <div className="flex items-center gap-4 text-xs text-text-muted">
            <span className="flex items-center gap-1">
              <Clock size={12} />
              {(scanResult.scan_duration_ms / 1000).toFixed(1)}s
            </span>
            <span className="text-success-text">{scanResult.hosts_up} up</span>
            <span className="text-danger-text">{scanResult.hosts_down} down</span>
          </div>
        )}
      </div>

      {/* Results */}
      <div className="flex-1 flex overflow-hidden">
        {/* Host List */}
        <div className="w-80 border-r border-dark-600 overflow-y-auto">
          {hosts.length === 0 ? (
            <div className="p-4 text-center text-text-muted">
              <Wifi size={32} className="mx-auto mb-2 opacity-30" />
              <p className="text-sm">No hosts discovered</p>
              <p className="text-xs mt-1">Click "Start Scan" to begin</p>
            </div>
          ) : (
            <div className="divide-y divide-dark-600">
              {hosts.map(host => (
                <button
                  key={host.id}
                  onClick={() => setSelectedHost(host)}
                  className={clsx(
                    'w-full p-3 text-left hover:bg-dark-700 transition-colors',
                    selectedHost?.id === host.id && 'bg-dark-700 border-l-2 border-l-blue-400'
                  )}
                >
                  <div className="flex items-center gap-2">
                    <Server size={14} className={host.status === 'up' ? 'text-success-text' : 'text-danger-text'} />
                    <span className="text-sm font-medium text-text-primary">{host.ip}</span>
                    {host.status === 'up' && (
                      <span className={clsx('text-xs', getLatencyColor(host.latency_ms))}>
                        {host.latency_ms.toFixed(1)}ms
                      </span>
                    )}
                  </div>
                  <div className="text-xs text-text-muted mt-1">{host.hostname}</div>
                  <div className="text-xs text-text-muted">{host.os}</div>
                  {host.status === 'up' && host.ports.length > 0 && (
                    <div className="flex items-center gap-1 mt-1">
                      <span className="text-xs text-success-text">{host.ports.filter(p => p.state === 'open').length} open</span>
                    </div>
                  )}
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Host Details */}
        <div className="flex-1 p-4 overflow-y-auto">
          {selectedHost ? (
            <div className="space-y-4">
              <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
                <h3 className="text-lg font-semibold text-text-primary flex items-center gap-2">
                  <Server size={18} className={selectedHost.status === 'up' ? 'text-success-text' : 'text-danger-text'} />
                  {selectedHost.ip}
                </h3>
                <div className="grid grid-cols-2 gap-4 mt-4">
                  <div>
                    <div className="text-xs text-text-muted">Hostname</div>
                    <div className="text-sm text-text-primary">{selectedHost.hostname}</div>
                  </div>
                  <div>
                    <div className="text-xs text-text-muted">MAC Address</div>
                    <div className="text-sm text-text-primary font-mono">{selectedHost.mac}</div>
                  </div>
                  <div>
                    <div className="text-xs text-text-muted">Operating System</div>
                    <div className="text-sm text-text-primary">{selectedHost.os}</div>
                  </div>
                  <div>
                    <div className="text-xs text-text-muted">OS Version</div>
                    <div className="text-sm text-text-primary">{selectedHost.os_version}</div>
                  </div>
                  <div>
                    <div className="text-xs text-text-muted">Vendor</div>
                    <div className="text-sm text-text-primary">{selectedHost.vendor}</div>
                  </div>
                  <div>
                    <div className="text-xs text-text-muted">Status</div>
                    <div className={clsx('text-sm', selectedHost.status === 'up' ? 'text-success-text' : 'text-danger-text')}>
                      {selectedHost.status.toUpperCase()}
                    </div>
                  </div>
                  {selectedHost.status === 'up' && (
                    <>
                      <div>
                        <div className="text-xs text-text-muted">Latency</div>
                        <div className={clsx('text-sm', getLatencyColor(selectedHost.latency_ms))}>
                          {selectedHost.latency_ms.toFixed(2)} ms
                        </div>
                      </div>
                      <div>
                        <div className="text-xs text-text-muted">TTL</div>
                        <div className="text-sm text-text-primary">{selectedHost.ttl}</div>
                      </div>
                    </>
                  )}
                </div>
              </div>

              <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
                <h4 className="text-sm font-semibold text-text-primary mb-3 flex items-center gap-2">
                  <Shield size={14} />
                  Open Ports ({selectedHost.ports.filter(p => p.state === 'open').length})
                </h4>
                {selectedHost.ports.length > 0 ? (
                  <div className="space-y-2">
                    {selectedHost.ports.map((port, i) => (
                      <div key={i} className="bg-dark-900 rounded p-3">
                        <div className="flex items-center gap-4">
                          <span className="text-sm font-mono text-text-primary w-16">{port.port}/{port.protocol}</span>
                          <span className="text-sm text-text-secondary flex-1">{port.service}</span>
                          <span className={clsx('text-xs px-2 py-0.5 rounded', getPortStateColor(port.state))}>
                            {port.state}
                          </span>
                        </div>
                        <div className="text-xs text-text-muted mt-1">{port.version}</div>
                        {port.banner && (
                          <div className="text-xs text-purple-text mt-1 font-mono">Banner: {port.banner}</div>
                        )}
                      </div>
                    ))}
                  </div>
                ) : (
                  <p className="text-sm text-text-muted">No open ports detected</p>
                )}
              </div>
            </div>
          ) : (
            <div className="h-full flex items-center justify-center text-text-muted">
              <div className="text-center">
                <Server size={48} className="mx-auto mb-4 opacity-20" />
                <p>Select a host to view details</p>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default NetworkScanner;
