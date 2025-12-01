/**
 * EventLog - Simulated Event Log Viewer
 * For demo/training purposes only
 */

import { useState, useEffect, useRef, useCallback } from "react";
import {
  FileText,
  Pause,
  Play,
  Trash2,
  Download,
  RefreshCw,
} from "lucide-react";
import { clsx } from "clsx";
import toast from "react-hot-toast";
import { simulateEventLog } from "../../lib/tauri";
import { useAsyncCommand } from "../../hooks";
import type { SimulatedLogEntry } from "../../types";

const MODULES = [
  "Scanner",
  "Payload",
  "Session",
  "C2",
  "PrivEsc",
  "Creds",
  "Lateral",
  "Persist",
  "System",
  "Network",
];

interface EventLogProps {
  sessionId: string;
}

export function EventLog({ sessionId: _sessionId }: EventLogProps) {
  const [logs, setLogs] = useState<SimulatedLogEntry[]>([]);
  const [isPaused, setIsPaused] = useState(false);
  const [filter, setFilter] = useState<string | null>(null);
  const [levelFilter, setLevelFilter] = useState<string | null>(null);
  const logEndRef = useRef<HTMLDivElement>(null);

  // Use the new async command hook for event log loading
  const { loading: isLoading, execute: fetchLogs } = useAsyncCommand<
    SimulatedLogEntry[],
    [number | undefined]
  >((count?: number) => simulateEventLog(count), {
    onError: (error) => {
      console.error("Failed to load logs:", error);
    },
  });

  const loadLogs = useCallback(
    async (append = false) => {
      if (isPaused && append) return;

      const newLogs = await fetchLogs(append ? 5 : 50);
      if (!newLogs) return;

      if (append) {
        setLogs((prev) => {
          const combined = [...newLogs.slice(0, 5), ...prev];
          return combined.slice(0, 200); // Keep max 200 logs
        });
      } else {
        setLogs(newLogs);
      }
    },
    [isPaused, fetchLogs]
  );

  // Initial load
  useEffect(() => {
    loadLogs(false);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Auto-refresh with simulated new logs
  useEffect(() => {
    if (isPaused) return;

    const interval = setInterval(
      () => {
        loadLogs(true);
      },
      2000 + Math.random() * 2000,
    );

    return () => clearInterval(interval);
  }, [isPaused, loadLogs]);

  // Auto-scroll
  useEffect(() => {
    if (!isPaused) {
      logEndRef.current?.scrollIntoView({ behavior: "smooth" });
    }
  }, [logs, isPaused]);

  const filteredLogs = logs.filter((log) => {
    const matchesModule = !filter || log.module === filter;
    const matchesLevel = !levelFilter || log.level === levelFilter;
    return matchesModule && matchesLevel;
  });

  const getLevelColor = (level: string) => {
    switch (level) {
      case "info":
        return "text-info-text";
      case "warn":
        return "text-warning-text";
      case "error":
        return "text-danger-text";
      case "success":
        return "text-success-text";
      case "debug":
        return "text-purple-text";
      default:
        return "text-text-muted";
    }
  };

  const getLevelBg = (level: string) => {
    switch (level) {
      case "info":
        return "bg-info-soft";
      case "warn":
        return "bg-warning-soft";
      case "error":
        return "bg-danger-soft";
      case "success":
        return "bg-success-soft";
      case "debug":
        return "bg-purple-soft";
      default:
        return "bg-dark-700";
    }
  };

  const clearLogs = () => {
    setLogs([]);
    toast.success("Logs cleared");
  };

  const exportLogs = () => {
    const content = filteredLogs
      .map(
        (log) =>
          `[${new Date(log.timestamp).toISOString()}] [${log.level.toUpperCase()}] [${log.module}] ${log.message}`,
      )
      .join("\n");
    const blob = new Blob([content], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `ferox-logs-${Date.now()}.txt`;
    a.click();
    URL.revokeObjectURL(url);
    toast.success("Logs exported");
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center gap-2">
          <FileText className="text-info-text" size={20} />
          <h2 className="text-lg font-semibold text-text-primary">Event Log</h2>
          <span className="text-xs bg-info-soft text-info-text px-2 py-0.5 rounded">
            SIMULATION
          </span>
          <span className="text-xs text-text-muted ml-2">
            {logs.length} entries
          </span>
          {isLoading && (
            <RefreshCw size={12} className="text-info-text animate-spin ml-2" />
          )}
        </div>
      </div>

      {/* Controls */}
      <div className="p-3 border-b border-dark-600 flex items-center gap-3">
        <button
          onClick={() => setIsPaused(!isPaused)}
          className={clsx(
            "px-3 py-1.5 rounded text-xs font-medium flex items-center gap-1.5 transition-colors",
            isPaused
              ? "bg-success-soft text-success-text hover:bg-success-soft"
              : "bg-warning-soft text-warning-text hover:bg-warning-soft",
          )}
        >
          {isPaused ? <Play size={12} /> : <Pause size={12} />}
          {isPaused ? "Resume" : "Pause"}
        </button>

        <div className="h-4 w-px bg-dark-600" />

        <select
          value={filter || ""}
          onChange={(e) => setFilter(e.target.value || null)}
          className="px-2 py-1.5 bg-dark-700 border border-dark-600 rounded text-xs text-text-primary focus:outline-none"
        >
          <option value="">All Modules</option>
          {MODULES.map((m) => (
            <option key={m} value={m}>
              {m}
            </option>
          ))}
        </select>

        <select
          value={levelFilter || ""}
          onChange={(e) => setLevelFilter(e.target.value || null)}
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
          className="px-3 py-1.5 rounded text-xs font-medium flex items-center gap-1.5 bg-dark-700 text-text-secondary hover:text-danger-text transition-colors"
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
            {filteredLogs.map((log) => (
              <div
                key={log.id}
                className={clsx(
                  "px-4 py-2 hover:bg-dark-800/50",
                  getLevelBg(log.level),
                )}
              >
                <span className="text-text-muted">
                  {new Date(log.timestamp).toLocaleTimeString()}
                </span>
                <span
                  className={clsx(
                    "mx-2 px-1.5 py-0.5 rounded text-[10px] uppercase",
                    getLevelColor(log.level),
                  )}
                >
                  {log.level}
                </span>
                <span className="text-purple-text">[{log.module}]</span>
                <span className="text-text-primary ml-2">{log.message}</span>
                {log.session_id && (
                  <span className="text-text-muted ml-2">
                    ({log.session_id})
                  </span>
                )}
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
