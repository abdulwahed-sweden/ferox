/**
 * EventLog - Simulated Event Log Viewer
 * For demo/training purposes only
 */

import { useState, useEffect, useRef } from 'react';
import { FileText, Pause, Play, Trash2, Download } from 'lucide-react';
import { clsx } from 'clsx';

interface LogEntry {
  id: string;
  timestamp: Date;
  level: 'info' | 'warn' | 'error' | 'success' | 'debug';
  module: string;
  message: string;
}

const MODULES = ['Scanner', 'Payload', 'Session', 'C2', 'PrivEsc', 'Creds', 'System'];
const MESSAGES = {
  Scanner: ['Port scan completed', 'Host discovered: 192.168.1.x', 'Service identified: SSH', 'Scan timeout on host'],
  Payload: ['Payload generated successfully', 'Obfuscation applied', 'Signature check passed', 'Build completed'],
  Session: ['New session established', 'Session heartbeat received', 'Session migrated', 'Session terminated'],
  C2: ['Beacon received', 'Command queued', 'Response encrypted', 'Channel established'],
  PrivEsc: ['UAC bypass attempted', 'Token impersonation success', 'Privilege escalation to SYSTEM', 'Enumeration complete'],
  Creds: ['Credential dump started', 'Hash extracted', 'Token captured', 'Browser passwords retrieved'],
  System: ['Module loaded', 'Configuration updated', 'Database synced', 'Memory optimized'],
};

interface EventLogProps {
  sessionId: string;
}

export function EventLog({ sessionId: _sessionId }: EventLogProps) {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [isPaused, setIsPaused] = useState(false);
  const [filter, setFilter] = useState<string | null>(null);
  const [levelFilter, setLevelFilter] = useState<string | null>(null);
  const logEndRef = useRef<HTMLDivElement>(null);

  // Generate random logs
  useEffect(() => {
    if (isPaused) return;

    const interval = setInterval(() => {
      const module = MODULES[Math.floor(Math.random() * MODULES.length)];
      const messages = MESSAGES[module as keyof typeof MESSAGES];
      const message = messages[Math.floor(Math.random() * messages.length)];
      const levels: LogEntry['level'][] = ['info', 'info', 'info', 'success', 'warn', 'debug'];
      const level = levels[Math.floor(Math.random() * levels.length)];

      const newLog: LogEntry = {
        id: `${Date.now()}-${Math.random()}`,
        timestamp: new Date(),
        level,
        module,
        message: message.replace('192.168.1.x', `192.168.1.${Math.floor(Math.random() * 255)}`),
      };

      setLogs(prev => [...prev.slice(-199), newLog]);
    }, 1000 + Math.random() * 2000);

    return () => clearInterval(interval);
  }, [isPaused]);

  // Auto-scroll
  useEffect(() => {
    if (!isPaused) {
      logEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }
  }, [logs, isPaused]);

  const filteredLogs = logs.filter(log => {
    const matchesModule = !filter || log.module === filter;
    const matchesLevel = !levelFilter || log.level === levelFilter;
    return matchesModule && matchesLevel;
  });

  const getLevelColor = (level: string) => {
    switch (level) {
      case 'info': return 'text-blue-400';
      case 'warn': return 'text-yellow-400';
      case 'error': return 'text-red-400';
      case 'success': return 'text-green-400';
      case 'debug': return 'text-purple-400';
      default: return 'text-text-muted';
    }
  };

  const getLevelBg = (level: string) => {
    switch (level) {
      case 'info': return 'bg-blue-500/10';
      case 'warn': return 'bg-yellow-500/10';
      case 'error': return 'bg-red-500/10';
      case 'success': return 'bg-green-500/10';
      case 'debug': return 'bg-purple-500/10';
      default: return 'bg-dark-700';
    }
  };

  const clearLogs = () => setLogs([]);

  const exportLogs = () => {
    const content = filteredLogs
      .map(log => `[${log.timestamp.toISOString()}] [${log.level.toUpperCase()}] [${log.module}] ${log.message}`)
      .join('\n');
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `ferox-logs-${Date.now()}.txt`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center gap-2">
          <FileText className="text-cyan-400" size={20} />
          <h2 className="text-lg font-semibold text-text-primary">Event Log</h2>
          <span className="text-xs bg-cyan-500/20 text-cyan-400 px-2 py-0.5 rounded">SIMULATION</span>
          <span className="text-xs text-text-muted ml-2">{logs.length} entries</span>
        </div>
      </div>

      {/* Controls */}
      <div className="p-3 border-b border-dark-600 flex items-center gap-3">
        <button
          onClick={() => setIsPaused(!isPaused)}
          className={clsx(
            'px-3 py-1.5 rounded text-xs font-medium flex items-center gap-1.5 transition-colors',
            isPaused
              ? 'bg-green-500/20 text-green-400 hover:bg-green-500/30'
              : 'bg-yellow-500/20 text-yellow-400 hover:bg-yellow-500/30'
          )}
        >
          {isPaused ? <Play size={12} /> : <Pause size={12} />}
          {isPaused ? 'Resume' : 'Pause'}
        </button>

        <div className="h-4 w-px bg-dark-600" />

        <select
          value={filter || ''}
          onChange={e => setFilter(e.target.value || null)}
          className="px-2 py-1.5 bg-dark-700 border border-dark-600 rounded text-xs text-text-primary focus:outline-none"
        >
          <option value="">All Modules</option>
          {MODULES.map(m => (
            <option key={m} value={m}>{m}</option>
          ))}
        </select>

        <select
          value={levelFilter || ''}
          onChange={e => setLevelFilter(e.target.value || null)}
          className="px-2 py-1.5 bg-dark-700 border border-dark-600 rounded text-xs text-text-primary focus:outline-none"
        >
          <option value="">All Levels</option>
          <option value="info">Info</option>
          <option value="success">Success</option>
          <option value="warn">Warning</option>
          <option value="error">Error</option>
          <option value="debug">Debug</option>
        </select>

        <div className="flex-1" />

        <button
          onClick={exportLogs}
          className="px-3 py-1.5 rounded text-xs font-medium flex items-center gap-1.5 bg-dark-700 text-text-secondary hover:text-text-primary transition-colors"
        >
          <Download size={12} />
          Export
        </button>

        <button
          onClick={clearLogs}
          className="px-3 py-1.5 rounded text-xs font-medium flex items-center gap-1.5 bg-dark-700 text-text-secondary hover:text-red-400 transition-colors"
        >
          <Trash2 size={12} />
          Clear
        </button>
      </div>

      {/* Log Entries */}
      <div className="flex-1 overflow-y-auto font-mono text-xs">
        {filteredLogs.length === 0 ? (
          <div className="h-full flex items-center justify-center text-text-muted">
            <div className="text-center">
              <FileText size={48} className="mx-auto mb-4 opacity-20" />
              <p>No log entries</p>
              <p className="text-xs mt-1">Events will appear here</p>
            </div>
          </div>
        ) : (
          <div className="divide-y divide-dark-700/50">
            {filteredLogs.map(log => (
              <div key={log.id} className={clsx('px-4 py-2 hover:bg-dark-800/50', getLevelBg(log.level))}>
                <span className="text-text-muted">
                  {log.timestamp.toLocaleTimeString()}
                </span>
                <span className={clsx('mx-2 px-1.5 py-0.5 rounded text-[10px] uppercase', getLevelColor(log.level))}>
                  {log.level}
                </span>
                <span className="text-purple-400">[{log.module}]</span>
                <span className="text-text-primary ml-2">{log.message}</span>
              </div>
            ))}
            <div ref={logEndRef} />
          </div>
        )}
      </div>
    </div>
  );
}

export default EventLog;
