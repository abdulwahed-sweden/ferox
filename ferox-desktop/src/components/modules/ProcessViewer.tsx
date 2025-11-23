import { useState, useCallback, useMemo } from 'react';
import {
  Square,
  RefreshCw,
  Search,
  Cpu,
  HardDrive,
} from 'lucide-react';
import { clsx } from 'clsx';
import { Spinner } from '../Loading';

interface ProcessInfo {
  pid: number;
  name: string;
  user: string;
  cpu: number;
  memory: number;
  status: 'running' | 'sleeping' | 'stopped';
  command: string;
}

interface ProcessViewerProps {
  sessionId: string;
}

// Mock data for demonstration
const mockProcesses: ProcessInfo[] = [
  { pid: 1, name: 'systemd', user: 'root', cpu: 0.1, memory: 0.5, status: 'running', command: '/sbin/init' },
  { pid: 234, name: 'sshd', user: 'root', cpu: 0.0, memory: 0.3, status: 'running', command: '/usr/sbin/sshd -D' },
  { pid: 456, name: 'bash', user: 'user', cpu: 0.0, memory: 0.2, status: 'sleeping', command: '-bash' },
  { pid: 789, name: 'nginx', user: 'www-data', cpu: 0.5, memory: 1.2, status: 'running', command: 'nginx: worker process' },
  { pid: 1024, name: 'python3', user: 'user', cpu: 2.3, memory: 4.5, status: 'running', command: 'python3 app.py' },
  { pid: 1337, name: 'implant', user: 'user', cpu: 0.1, memory: 0.8, status: 'running', command: './update_service' },
  { pid: 2048, name: 'mysql', user: 'mysql', cpu: 1.5, memory: 8.2, status: 'running', command: '/usr/sbin/mysqld' },
  { pid: 3000, name: 'node', user: 'user', cpu: 3.2, memory: 5.6, status: 'running', command: 'node server.js' },
];

export function ProcessViewer({ sessionId }: ProcessViewerProps) {
  const [processes, setProcesses] = useState<ProcessInfo[]>(mockProcesses);
  const [selectedPid, setSelectedPid] = useState<number | null>(null);
  const [loading, setLoading] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [sortField, setSortField] = useState<keyof ProcessInfo>('cpu');
  const [sortDesc, setSortDesc] = useState(true);

  const handleRefresh = useCallback(() => {
    setLoading(true);
    // Simulate API call
    setTimeout(() => {
      setProcesses(mockProcesses); // Would fetch from backend
      setLoading(false);
    }, 500);
  }, []);

  const handleKillProcess = useCallback(() => {
    if (selectedPid) {
      console.log('Kill process:', selectedPid);
      // Would send kill signal via Tauri
    }
  }, [selectedPid]);

  const handleSort = useCallback((field: keyof ProcessInfo) => {
    if (sortField === field) {
      setSortDesc(!sortDesc);
    } else {
      setSortField(field);
      setSortDesc(true);
    }
  }, [sortField, sortDesc]);

  const filteredProcesses = useMemo(() => {
    let result = processes;

    // Filter by search
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      result = result.filter(
        (p) =>
          p.name.toLowerCase().includes(query) ||
          p.user.toLowerCase().includes(query) ||
          p.command.toLowerCase().includes(query) ||
          p.pid.toString().includes(query)
      );
    }

    // Sort
    result = [...result].sort((a, b) => {
      const aVal = a[sortField];
      const bVal = b[sortField];
      if (typeof aVal === 'number' && typeof bVal === 'number') {
        return sortDesc ? bVal - aVal : aVal - bVal;
      }
      return sortDesc
        ? String(bVal).localeCompare(String(aVal))
        : String(aVal).localeCompare(String(bVal));
    });

    return result;
  }, [processes, searchQuery, sortField, sortDesc]);

  const totalCpu = useMemo(() => processes.reduce((sum, p) => sum + p.cpu, 0), [processes]);
  const totalMemory = useMemo(() => processes.reduce((sum, p) => sum + p.memory, 0), [processes]);

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Toolbar */}
      <div className="flex items-center gap-2 p-2 bg-dark-800 border-b border-dark-600">
        <button
          onClick={handleRefresh}
          className="p-1.5 hover:bg-dark-600 rounded transition-colors"
          title="Refresh"
        >
          <RefreshCw size={16} />
        </button>
        <button
          onClick={handleKillProcess}
          disabled={!selectedPid}
          className="p-1.5 hover:bg-dark-600 rounded transition-colors text-danger disabled:opacity-50"
          title="Kill Process"
        >
          <Square size={16} />
        </button>

        <div className="flex-1" />

        <div className="relative">
          <Search size={14} className="absolute left-2 top-1/2 -translate-y-1/2 text-text-muted" />
          <input
            type="text"
            placeholder="Search processes..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-64 pl-8 pr-3 py-1 text-sm bg-dark-700 border border-dark-600 rounded text-text-primary"
          />
        </div>

        {/* Stats */}
        <div className="flex items-center gap-3 pl-3 border-l border-dark-600 text-xs text-text-muted">
          <div className="flex items-center gap-1">
            <Cpu size={12} />
            <span>{totalCpu.toFixed(1)}%</span>
          </div>
          <div className="flex items-center gap-1">
            <HardDrive size={12} />
            <span>{totalMemory.toFixed(1)}%</span>
          </div>
        </div>
      </div>

      {/* Process list */}
      <div className="flex-1 overflow-auto">
        {loading ? (
          <div className="flex items-center justify-center h-full">
            <Spinner size="lg" />
          </div>
        ) : (
          <table className="w-full text-sm">
            <thead className="sticky top-0 bg-dark-800 text-text-muted">
              <tr>
                <th
                  className="text-left p-2 font-medium cursor-pointer hover:text-text-primary w-20"
                  onClick={() => handleSort('pid')}
                >
                  PID {sortField === 'pid' && (sortDesc ? '↓' : '↑')}
                </th>
                <th
                  className="text-left p-2 font-medium cursor-pointer hover:text-text-primary"
                  onClick={() => handleSort('name')}
                >
                  Name {sortField === 'name' && (sortDesc ? '↓' : '↑')}
                </th>
                <th
                  className="text-left p-2 font-medium cursor-pointer hover:text-text-primary w-24"
                  onClick={() => handleSort('user')}
                >
                  User {sortField === 'user' && (sortDesc ? '↓' : '↑')}
                </th>
                <th
                  className="text-right p-2 font-medium cursor-pointer hover:text-text-primary w-20"
                  onClick={() => handleSort('cpu')}
                >
                  CPU% {sortField === 'cpu' && (sortDesc ? '↓' : '↑')}
                </th>
                <th
                  className="text-right p-2 font-medium cursor-pointer hover:text-text-primary w-20"
                  onClick={() => handleSort('memory')}
                >
                  MEM% {sortField === 'memory' && (sortDesc ? '↓' : '↑')}
                </th>
                <th className="text-left p-2 font-medium">Status</th>
              </tr>
            </thead>
            <tbody>
              {filteredProcesses.map((proc) => (
                <tr
                  key={proc.pid}
                  className={clsx(
                    'cursor-pointer hover:bg-dark-700 transition-colors',
                    selectedPid === proc.pid && 'bg-dark-600'
                  )}
                  onClick={() => setSelectedPid(proc.pid)}
                >
                  <td className="p-2 font-mono text-text-muted">{proc.pid}</td>
                  <td className="p-2">
                    <div className="flex items-center gap-2">
                      <span className="text-text-primary">{proc.name}</span>
                      {proc.name === 'implant' && (
                        <span className="text-xs px-1 bg-ferox-green/20 text-ferox-green rounded">
                          ours
                        </span>
                      )}
                    </div>
                  </td>
                  <td className="p-2 text-text-muted">{proc.user}</td>
                  <td className={clsx('p-2 text-right', proc.cpu > 2 && 'text-warning')}>
                    {proc.cpu.toFixed(1)}
                  </td>
                  <td className={clsx('p-2 text-right', proc.memory > 5 && 'text-warning')}>
                    {proc.memory.toFixed(1)}
                  </td>
                  <td className="p-2">
                    <span
                      className={clsx(
                        'text-xs px-1.5 py-0.5 rounded',
                        proc.status === 'running' && 'bg-ferox-green/20 text-ferox-green',
                        proc.status === 'sleeping' && 'bg-info/20 text-info',
                        proc.status === 'stopped' && 'bg-danger/20 text-danger'
                      )}
                    >
                      {proc.status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {/* Status bar */}
      <div className="px-3 py-1.5 bg-dark-800 border-t border-dark-600 text-xs text-text-muted">
        {filteredProcesses.length} processes | Session: {sessionId.slice(0, 8)}...
      </div>
    </div>
  );
}
