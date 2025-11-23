/**
 * NetworkScanner - Simulated Network Scan Results
 * For demo/training purposes only - no real network access
 */

import { useState } from 'react';
import { Radar, Play, RefreshCw, Server, Wifi, Shield } from 'lucide-react';
import { clsx } from 'clsx';

interface SimulatedHost {
  id: string;
  ip: string;
  hostname: string;
  mac: string;
  os: string;
  ports: { port: number; service: string; state: string }[];
  status: 'up' | 'down';
}

const SIMULATED_HOSTS: SimulatedHost[] = [
  {
    id: '1',
    ip: '192.168.1.1',
    hostname: 'gateway.local',
    mac: 'AA:BB:CC:DD:EE:01',
    os: 'Linux 4.x (Router)',
    ports: [
      { port: 22, service: 'SSH', state: 'open' },
      { port: 80, service: 'HTTP', state: 'open' },
      { port: 443, service: 'HTTPS', state: 'open' },
    ],
    status: 'up',
  },
  {
    id: '2',
    ip: '192.168.1.10',
    hostname: 'dc01.corp.local',
    mac: 'AA:BB:CC:DD:EE:02',
    os: 'Windows Server 2019',
    ports: [
      { port: 53, service: 'DNS', state: 'open' },
      { port: 88, service: 'Kerberos', state: 'open' },
      { port: 135, service: 'MSRPC', state: 'open' },
      { port: 389, service: 'LDAP', state: 'open' },
      { port: 445, service: 'SMB', state: 'open' },
      { port: 3389, service: 'RDP', state: 'open' },
    ],
    status: 'up',
  },
  {
    id: '3',
    ip: '192.168.1.20',
    hostname: 'web01.corp.local',
    mac: 'AA:BB:CC:DD:EE:03',
    os: 'Ubuntu 22.04 LTS',
    ports: [
      { port: 22, service: 'SSH', state: 'open' },
      { port: 80, service: 'HTTP', state: 'open' },
      { port: 443, service: 'HTTPS', state: 'open' },
      { port: 3306, service: 'MySQL', state: 'filtered' },
    ],
    status: 'up',
  },
  {
    id: '4',
    ip: '192.168.1.50',
    hostname: 'workstation01',
    mac: 'AA:BB:CC:DD:EE:04',
    os: 'Windows 10 Pro',
    ports: [
      { port: 135, service: 'MSRPC', state: 'open' },
      { port: 445, service: 'SMB', state: 'open' },
      { port: 3389, service: 'RDP', state: 'closed' },
    ],
    status: 'up',
  },
  {
    id: '5',
    ip: '192.168.1.100',
    hostname: 'unknown',
    mac: 'AA:BB:CC:DD:EE:05',
    os: 'Unknown',
    ports: [],
    status: 'down',
  },
];

interface NetworkScannerProps {
  sessionId: string;
}

export function NetworkScanner({ sessionId: _sessionId }: NetworkScannerProps) {
  const [isScanning, setIsScanning] = useState(false);
  const [hosts, setHosts] = useState<SimulatedHost[]>([]);
  const [selectedHost, setSelectedHost] = useState<SimulatedHost | null>(null);
  const [scanRange, setScanRange] = useState('192.168.1.0/24');

  const handleScan = async () => {
    setIsScanning(true);
    setHosts([]);
    setSelectedHost(null);

    // Simulate progressive discovery
    for (let i = 0; i < SIMULATED_HOSTS.length; i++) {
      await new Promise(r => setTimeout(r, 500 + Math.random() * 500));
      setHosts(prev => [...prev, SIMULATED_HOSTS[i]]);
    }

    setIsScanning(false);
  };

  const getPortStateColor = (state: string) => {
    switch (state) {
      case 'open': return 'text-green-400';
      case 'closed': return 'text-red-400';
      case 'filtered': return 'text-yellow-400';
      default: return 'text-text-muted';
    }
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center gap-2">
          <Radar className="text-blue-400" size={20} />
          <h2 className="text-lg font-semibold text-text-primary">Network Scanner</h2>
          <span className="text-xs bg-blue-500/20 text-blue-400 px-2 py-0.5 rounded">SIMULATION</span>
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
                    <Server size={14} className={host.status === 'up' ? 'text-green-400' : 'text-red-400'} />
                    <span className="text-sm font-medium text-text-primary">{host.ip}</span>
                  </div>
                  <div className="text-xs text-text-muted mt-1">{host.hostname}</div>
                  <div className="text-xs text-text-muted">{host.os}</div>
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
                  <Server size={18} className={selectedHost.status === 'up' ? 'text-green-400' : 'text-red-400'} />
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
                    <div className="text-xs text-text-muted">Status</div>
                    <div className={clsx('text-sm', selectedHost.status === 'up' ? 'text-green-400' : 'text-red-400')}>
                      {selectedHost.status.toUpperCase()}
                    </div>
                  </div>
                </div>
              </div>

              <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
                <h4 className="text-sm font-semibold text-text-primary mb-3 flex items-center gap-2">
                  <Shield size={14} />
                  Open Ports ({selectedHost.ports.length})
                </h4>
                {selectedHost.ports.length > 0 ? (
                  <div className="space-y-2">
                    {selectedHost.ports.map((port, i) => (
                      <div key={i} className="flex items-center gap-4 bg-dark-900 rounded p-2">
                        <span className="text-sm font-mono text-text-primary w-16">{port.port}</span>
                        <span className="text-sm text-text-secondary flex-1">{port.service}</span>
                        <span className={clsx('text-xs', getPortStateColor(port.state))}>
                          {port.state}
                        </span>
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
