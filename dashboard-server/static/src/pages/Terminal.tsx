import { useState, useRef, useEffect, KeyboardEvent } from 'react';
import { useDashboardStore } from '../store';
import { useWebSocket } from '../hooks/useWebSocket';
import {
  Send,
  Trash2,
  Copy,
  CheckCircle2,
  XCircle,
  Loader2,
  ChevronDown,
} from 'lucide-react';
import { clsx } from 'clsx';

const quickCommands = [
  { label: 'whoami', cmd: 'whoami' },
  { label: 'hostname', cmd: 'hostname' },
  { label: 'ipconfig', cmd: 'ipconfig /all' },
  { label: 'net user', cmd: 'net user' },
  { label: 'systeminfo', cmd: 'systeminfo' },
  { label: 'dir', cmd: 'dir' },
];

const feroxCommands = [
  { label: 'Auto PrivEsc', cmd: 'ferox privesc --auto' },
  { label: 'Harvest Creds', cmd: 'ferox creds harvest --all' },
  { label: 'Install Persist', cmd: 'ferox persist install --stealth' },
  { label: 'Network Scan', cmd: 'ferox lateral discover' },
];

interface TerminalEntry {
  id: string;
  timestamp: Date;
  command: string;
  output: string;
  isComplete: boolean;
  success: boolean | null;
}

export function TerminalPage() {
  const { sessions, selectedSessionId, selectSession, commands } = useDashboardStore();
  const [input, setInput] = useState('');
  const [entries, setEntries] = useState<TerminalEntry[]>([]);
  const [commandHistory, setCommandHistory] = useState<string[]>([]);
  const [historyIndex, setHistoryIndex] = useState(-1);

  const terminalRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const selectedSession = sessions.find((s) => s.id === selectedSessionId);

  // WebSocket for sending commands
  const { send, isConnected } = useWebSocket(`ws://${window.location.host}/ws`);

  // Auto-scroll to bottom
  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [entries]);

  // Focus input on mount
  useEffect(() => {
    inputRef.current?.focus();
  }, [selectedSessionId]);

  // Load existing commands for session
  useEffect(() => {
    if (selectedSessionId && commands[selectedSessionId]) {
      const existingEntries = commands[selectedSessionId].map((cmd) => ({
        id: cmd.id,
        timestamp: new Date(cmd.timestamp),
        command: cmd.command,
        output: cmd.output,
        isComplete: cmd.completed_at !== null,
        success: cmd.success,
      }));
      setEntries(existingEntries);
    } else {
      setEntries([]);
    }
  }, [selectedSessionId, commands]);

  const handleSubmit = () => {
    if (!input.trim() || !selectedSessionId || !isConnected) return;

    const newEntry: TerminalEntry = {
      id: crypto.randomUUID(),
      timestamp: new Date(),
      command: input,
      output: '',
      isComplete: false,
      success: null,
    };

    setEntries((prev) => [...prev, newEntry]);
    setCommandHistory((prev) => [input, ...prev].slice(0, 50));
    setHistoryIndex(-1);

    // Send command via WebSocket
    send({
      type: 'ExecuteCommand',
      data: {
        session_id: selectedSessionId,
        command: input,
      },
    });

    setInput('');
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSubmit();
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (historyIndex < commandHistory.length - 1) {
        const newIndex = historyIndex + 1;
        setHistoryIndex(newIndex);
        setInput(commandHistory[newIndex]);
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (historyIndex > 0) {
        const newIndex = historyIndex - 1;
        setHistoryIndex(newIndex);
        setInput(commandHistory[newIndex]);
      } else if (historyIndex === 0) {
        setHistoryIndex(-1);
        setInput('');
      }
    }
  };

  const clearTerminal = () => {
    setEntries([]);
  };

  const copyOutput = (output: string) => {
    navigator.clipboard.writeText(output);
  };

  if (!selectedSession) {
    return (
      <div className="h-full flex flex-col items-center justify-center text-text-secondary">
        <p className="mb-4">Select a session to open terminal</p>
        <div className="flex flex-wrap gap-2 justify-center max-w-lg">
          {sessions.slice(0, 6).map((session) => (
            <button
              key={session.id}
              onClick={() => selectSession(session.id)}
              className="btn-outline"
            >
              <div
                className={clsx(
                  'w-2 h-2 rounded-full',
                  session.status === 'active' && 'bg-ferox-green',
                  session.status === 'sleeping' && 'bg-warning',
                  session.status === 'dead' && 'bg-danger'
                )}
              />
              {session.hostname}
            </button>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="h-[calc(100vh-8rem)] flex flex-col gap-4 animate-fade-in">
      {/* Session selector */}
      <div className="flex items-center justify-between">
        <div className="relative">
          <select
            value={selectedSessionId || ''}
            onChange={(e) => selectSession(e.target.value || null)}
            className="input pr-10 appearance-none cursor-pointer"
          >
            {sessions.map((session) => (
              <option key={session.id} value={session.id}>
                {session.hostname} ({session.ip_address}) - {session.username}
              </option>
            ))}
          </select>
          <ChevronDown
            size={16}
            className="absolute right-3 top-1/2 -translate-y-1/2 text-text-muted pointer-events-none"
          />
        </div>

        <div className="flex items-center gap-2">
          <span
            className={clsx(
              'badge',
              selectedSession.status === 'active' && 'badge-success',
              selectedSession.status === 'sleeping' && 'badge-warning',
              selectedSession.status === 'dead' && 'badge-danger'
            )}
          >
            {selectedSession.status}
          </span>
          <button onClick={clearTerminal} className="btn-ghost" title="Clear terminal">
            <Trash2 size={18} />
          </button>
        </div>
      </div>

      {/* Quick commands */}
      <div className="flex flex-wrap gap-2">
        {quickCommands.map((cmd) => (
          <button
            key={cmd.label}
            onClick={() => setInput(cmd.cmd)}
            className="px-2 py-1 text-xs bg-dark-700 text-text-secondary rounded hover:bg-dark-600 hover:text-text-primary transition-colors"
          >
            {cmd.label}
          </button>
        ))}
        <span className="text-text-muted mx-2">|</span>
        {feroxCommands.map((cmd) => (
          <button
            key={cmd.label}
            onClick={() => setInput(cmd.cmd)}
            className="px-2 py-1 text-xs bg-ferox-green/10 text-ferox-green rounded hover:bg-ferox-green/20 transition-colors"
          >
            {cmd.label}
          </button>
        ))}
      </div>

      {/* Terminal output */}
      <div
        ref={terminalRef}
        className="flex-1 terminal overflow-auto"
        onClick={() => inputRef.current?.focus()}
      >
        {entries.length === 0 && (
          <div className="text-text-muted">
            <p>Connected to {selectedSession.hostname}</p>
            <p>Type a command or click a quick action above</p>
          </div>
        )}

        {entries.map((entry) => (
          <div key={entry.id} className="mb-4">
            {/* Command line */}
            <div className="flex items-center gap-2">
              <span className="text-text-muted text-xs">
                [{entry.timestamp.toLocaleTimeString()}]
              </span>
              <span className="terminal-prompt">
                [{selectedSession.hostname}]$
              </span>
              <span className="text-ferox-green">{entry.command}</span>
            </div>

            {/* Output */}
            {entry.output && (
              <div className="mt-1 ml-4 relative group">
                <pre className="terminal-output">{entry.output}</pre>
                <button
                  onClick={() => copyOutput(entry.output)}
                  className="absolute top-0 right-0 p-1 opacity-0 group-hover:opacity-100 transition-opacity text-text-muted hover:text-text-primary"
                  title="Copy output"
                >
                  <Copy size={14} />
                </button>
              </div>
            )}

            {/* Status */}
            <div className="mt-1 ml-4 flex items-center gap-2 text-xs">
              {entry.isComplete ? (
                entry.success ? (
                  <span className="flex items-center gap-1 text-ferox-green">
                    <CheckCircle2 size={12} />
                    Completed
                  </span>
                ) : (
                  <span className="flex items-center gap-1 text-danger">
                    <XCircle size={12} />
                    Failed
                  </span>
                )
              ) : (
                <span className="flex items-center gap-1 text-warning">
                  <Loader2 size={12} className="animate-spin" />
                  Running...
                </span>
              )}
            </div>
          </div>
        ))}
      </div>

      {/* Input */}
      <div className="flex items-center gap-2 bg-dark-800 rounded-lg p-2 border border-dark-600">
        <span className="terminal-prompt pl-2">
          [{selectedSession.hostname}]$
        </span>
        <input
          ref={inputRef}
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Enter command..."
          className="flex-1 bg-transparent text-text-primary font-mono outline-none"
          disabled={!isConnected}
        />
        <button
          onClick={handleSubmit}
          disabled={!input.trim() || !isConnected}
          className="btn-primary py-1.5"
        >
          <Send size={16} />
        </button>
      </div>
    </div>
  );
}
