/**
 * ProcessViewer - Simulated Process List
 * For demo/training purposes only - no real process access
 */

import { useState, useCallback, useMemo, useEffect } from 'react';
import {
  Square,
  RefreshCw,
  Search,
  Cpu,
  HardDrive,
  Activity,
  Shield,
} from 'lucide-react';
import { clsx } from 'clsx';
import toast from 'react-hot-toast';
import { Spinner } from '../Loading';
import { simulateProcessList } from '../../lib/tauri';
import type { SimulatedProcess, ProcessListResult } from '../../types';

interface ProcessViewerProps {
  sessionId: string;
}

export function ProcessViewer({ sessionId }: ProcessViewerProps) {
  const [result, setResult] = useState<ProcessListResult | null>(null);
  const [processes, setProcesses] = useState<SimulatedProcess[]>([]);
  const [selectedPid, setSelectedPid] = useState<number | null>(null);
  const [loading, setLoading] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [sortField, setSortField] = useState<keyof SimulatedProcess>('cpu');
  const [sortDesc, setSortDesc] = useState(true);

  const loadProcesses = useCallback(async () => {
    setLoading(true);
    try {
      const data = await simulateProcessList(sessionId);
      setResult(data);
      setProcesses(data.processes);
    } catch (error) {
      console.error('Failed to load processes:', error);
      toast.error('Failed to enumerate processes');
    } finally {
      setLoading(false);
    }
  }, [sessionId]);

  useEffect(() => {
    loadProcesses();
  }, [loadProcesses]);

  const handleRefresh = useCallback(() => {
    loadProcesses();
  }, [loadProcesses]);

  const handleKillProcess = useCallback(() => {
    if (selectedPid) {
      setProcesses(prev => prev.filter(p => p.pid !== selectedPid));
      toast.success(`Process ${selectedPid} terminated (simulated)`);
      setSelectedPid(null);
    }
  }, [selectedPid]);

  const handleSort = useCallback((field: keyof SimulatedProcess) => {
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

  const totalCpu = result?.total_cpu ?? 0;
  const totalMemory = result?.total_memory ?? 0;

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-3 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center gap-2">
          <Activity className="text-success-text" size={18} />
          <h2 className="text-sm font-semibold text-text-primary">Process Viewer</h2>
          <span className="text-xs bg-success-soft text-success-text px-2 py-0.5 rounded">SIMULATION</span>
        </div>
      </div>

      {/* Toolbar */}
      <div className="flex items-center gap-2 p-2 bg-dark-800 border-b border-dark-600">
        <button
          onClick={handleRefresh}
          className="p-1.5 hover:bg-dark-600 rounded transition-colors"
          title="Refresh"
        >
          <RefreshCw size={16} className={loading ? 'animate-spin' : ''} />
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
            <Cpu size={12} className={totalCpu > 50 ? 'text-danger-text' : 'text-success-text'} />
            <span>{totalCpu.toFixed(1)}%</span>
          </div>
          <div className="flex items-center gap-1">
            <HardDrive size={12} className={totalMemory > 50 ? 'text-danger-text' : 'text-success-text'} />
            <span>{totalMemory.toFixed(1)}%</span>
          </div>
          <div className="flex items-center gap-1">
            <Activity size={12} />
            <span>{processes.length} procs</span>
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
                <th className="text-left p-2 font-medium w-20">Threads</th>
                <th className="text-left p-2 font-medium">Status</th>
              </tr>
            </thead>
            <tbody>
              {filteredProcesses.map((proc) => (
                <tr
                  key={proc.pid}
                  className={clsx(
                    'cursor-pointer hover:bg-dark-700 transition-colors',
                    selectedPid === proc.pid && 'bg-dark-600',
                    proc.is_implant && 'bg-ferox-green/5'
                  )}
                  onClick={() => setSelectedPid(proc.pid)}
                >
                  <td className="p-2 font-mono text-text-muted">{proc.pid}</td>
                  <td className="p-2">
                    <div className="flex items-center gap-2">
                      <span className="text-text-primary">{proc.name}</span>
                      {proc.is_implant && (
                        <span className="text-xs px-1.5 py-0.5 bg-ferox-green/20 text-ferox-green rounded flex items-center gap-1">
                          <Shield size={10} />
                          ours
                        </span>
                      )}
                    </div>
                    <div className="text-xs text-text-muted truncate max-w-xs" title={proc.command}>
                      {proc.command}
                    </div>
                  </td>
                  <td className="p-2 text-text-muted">{proc.user}</td>
                  <td className={clsx('p-2 text-right', proc.cpu > 2 && 'text-warning-text', proc.cpu > 5 && 'text-danger-text')}>
                    {proc.cpu.toFixed(1)}
                  </td>
                  <td className={clsx('p-2 text-right', proc.memory > 5 && 'text-warning-text', proc.memory > 10 && 'text-danger-text')}>
                    {proc.memory.toFixed(1)}
                  </td>
                  <td className="p-2 text-text-muted">{proc.threads}</td>
                  <td className="p-2">
                    <span
                      className={clsx(
                        'text-xs px-1.5 py-0.5 rounded',
                        proc.status === 'running' && 'bg-ferox-green/20 text-ferox-green',
                        proc.status === 'sleeping' && 'bg-info-soft text-info-text',
                        proc.status === 'stopped' && 'bg-danger-soft text-danger-text',
                        proc.status === 'zombie' && 'bg-purple-soft text-purple-text'
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

export default ProcessViewer;
