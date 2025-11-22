import { useState } from 'react';
import { useDashboardStore } from '../store';
import {
  Monitor,
  MoreVertical,
  Terminal,
  Key,
  Shield,
  ArrowRight,
  Trash2,
  Eye,
  Clock,
} from 'lucide-react';
import { clsx } from 'clsx';
import type { Session } from '../types';

function formatRelativeTime(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (seconds < 60) return `${seconds}s ago`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
  return `${Math.floor(seconds / 86400)}d ago`;
}

interface SessionRowProps {
  session: Session;
  onSelect: (session: Session) => void;
}

function SessionRow({ session, onSelect }: SessionRowProps) {
  const [menuOpen, setMenuOpen] = useState(false);
  const { setActiveTab, selectSession } = useDashboardStore();

  const handleTerminal = () => {
    selectSession(session.id);
    setActiveTab('terminal');
    setMenuOpen(false);
  };

  return (
    <tr className="table-row">
      <td className="table-cell">
        <div className="flex items-center gap-3">
          <div
            className={clsx(
              'status-dot',
              session.status === 'active' && 'status-active',
              session.status === 'sleeping' && 'status-sleeping',
              session.status === 'dead' && 'status-dead'
            )}
          />
          <div>
            <p className="font-medium text-text-primary">{session.hostname}</p>
            <p className="text-xs text-text-muted">{session.ip_address}</p>
          </div>
        </div>
      </td>
      <td className="table-cell">
        <div className="flex items-center gap-2">
          <span className="capitalize">{session.os}</span>
          <span className="text-text-muted text-xs">({session.architecture})</span>
        </div>
      </td>
      <td className="table-cell">
        <div>
          <p className="text-text-primary">{session.username}</p>
          <span
            className={clsx(
              'badge mt-1',
              session.privileges === 'system' || session.privileges === 'root'
                ? 'badge-danger'
                : session.privileges === 'administrator'
                ? 'badge-warning'
                : 'badge-gray'
            )}
          >
            {session.privileges}
          </span>
        </div>
      </td>
      <td className="table-cell">
        {session.metrics.credentials_count > 0 ? (
          <span className="badge badge-info">
            {session.metrics.credentials_count} creds
          </span>
        ) : (
          <span className="text-text-muted">-</span>
        )}
      </td>
      <td className="table-cell">
        <div className="flex items-center gap-1 text-text-secondary">
          <Clock size={14} />
          <span>{formatRelativeTime(session.last_seen)}</span>
        </div>
      </td>
      <td className="table-cell">
        <div className="relative">
          <button
            onClick={() => setMenuOpen(!menuOpen)}
            className="p-1.5 rounded hover:bg-dark-600 text-text-secondary hover:text-text-primary"
          >
            <MoreVertical size={18} />
          </button>

          {menuOpen && (
            <>
              <div
                className="fixed inset-0 z-10"
                onClick={() => setMenuOpen(false)}
              />
              <div className="absolute right-0 top-8 z-20 bg-dark-700 border border-dark-500 rounded-lg shadow-lg py-1 min-w-[180px] animate-fade-in">
                <button
                  onClick={handleTerminal}
                  className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary"
                >
                  <Terminal size={16} />
                  Execute Command
                </button>
                <button className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary">
                  <Shield size={16} />
                  Escalate Privileges
                </button>
                <button className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary">
                  <Key size={16} />
                  Harvest Credentials
                </button>
                <button className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary">
                  <ArrowRight size={16} />
                  Lateral Movement
                </button>
                <hr className="my-1 border-dark-500" />
                <button
                  onClick={() => onSelect(session)}
                  className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary"
                >
                  <Eye size={16} />
                  View Details
                </button>
                <button className="w-full flex items-center gap-2 px-3 py-2 text-sm text-danger hover:bg-dark-600">
                  <Trash2 size={16} />
                  Terminate Session
                </button>
              </div>
            </>
          )}
        </div>
      </td>
    </tr>
  );
}

interface SessionModalProps {
  session: Session;
  onClose: () => void;
}

function SessionModal({ session, onClose }: SessionModalProps) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div className="absolute inset-0 bg-black/60" onClick={onClose} />
      <div className="relative bg-dark-700 border border-dark-500 rounded-lg w-full max-w-2xl max-h-[80vh] overflow-auto animate-fade-in">
        <div className="sticky top-0 bg-dark-700 border-b border-dark-500 p-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Monitor className="text-ferox-green" size={24} />
            <div>
              <h2 className="text-lg font-semibold text-text-primary">
                {session.hostname}
              </h2>
              <p className="text-sm text-text-secondary">{session.ip_address}</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="text-text-secondary hover:text-text-primary"
          >
            ×
          </button>
        </div>

        <div className="p-4 space-y-4">
          {/* System Info */}
          <div className="grid grid-cols-2 gap-4">
            <div className="bg-dark-800 rounded-lg p-3">
              <p className="text-xs text-text-muted mb-1">Operating System</p>
              <p className="text-text-primary capitalize">
                {session.os} {session.os_version}
              </p>
            </div>
            <div className="bg-dark-800 rounded-lg p-3">
              <p className="text-xs text-text-muted mb-1">Architecture</p>
              <p className="text-text-primary uppercase">{session.architecture}</p>
            </div>
            <div className="bg-dark-800 rounded-lg p-3">
              <p className="text-xs text-text-muted mb-1">User</p>
              <p className="text-text-primary">{session.username}</p>
            </div>
            <div className="bg-dark-800 rounded-lg p-3">
              <p className="text-xs text-text-muted mb-1">Privileges</p>
              <span
                className={clsx(
                  'badge',
                  session.privileges === 'system' || session.privileges === 'root'
                    ? 'badge-danger'
                    : session.privileges === 'administrator'
                    ? 'badge-warning'
                    : 'badge-gray'
                )}
              >
                {session.privileges}
              </span>
            </div>
          </div>

          {/* Intelligence */}
          <div className="bg-dark-800 rounded-lg p-4">
            <h3 className="text-sm font-medium text-text-primary mb-3">Intelligence</h3>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-text-secondary">Domain</span>
                <span className="text-text-primary">
                  {session.intelligence.domain || 'N/A'}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-text-secondary">Domain Joined</span>
                <span className="text-text-primary">
                  {session.intelligence.is_domain_joined ? 'Yes' : 'No'}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-text-secondary">Detected AV</span>
                <span className="text-text-primary">
                  {session.intelligence.detected_av.join(', ') || 'None'}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-text-secondary">Stealth Mode</span>
                <span className="text-ferox-green capitalize">
                  {session.intelligence.stealth_mode}
                </span>
              </div>
            </div>
          </div>

          {/* Metrics */}
          <div className="grid grid-cols-4 gap-3">
            <div className="bg-dark-800 rounded-lg p-3 text-center">
              <p className="text-2xl font-bold text-ferox-green">
                {session.metrics.commands_executed}
              </p>
              <p className="text-xs text-text-muted">Commands</p>
            </div>
            <div className="bg-dark-800 rounded-lg p-3 text-center">
              <p className="text-2xl font-bold text-info">
                {session.metrics.credentials_count}
              </p>
              <p className="text-xs text-text-muted">Credentials</p>
            </div>
            <div className="bg-dark-800 rounded-lg p-3 text-center">
              <p className="text-2xl font-bold text-warning">
                {session.metrics.files_transferred}
              </p>
              <p className="text-xs text-text-muted">Files</p>
            </div>
            <div className="bg-dark-800 rounded-lg p-3 text-center">
              <p className="text-2xl font-bold text-purple-400">
                {session.metrics.persistence_methods}
              </p>
              <p className="text-xs text-text-muted">Persistence</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export function SessionsPage() {
  const { sessions } = useDashboardStore();
  const [selectedSession, setSelectedSession] = useState<Session | null>(null);
  const [filter, setFilter] = useState<'all' | 'active' | 'sleeping' | 'dead'>('all');

  const filteredSessions = sessions.filter(
    (s) => filter === 'all' || s.status === filter
  );

  return (
    <div className="space-y-4 animate-fade-in">
      {/* Filter tabs */}
      <div className="flex gap-2">
        {(['all', 'active', 'sleeping', 'dead'] as const).map((status) => (
          <button
            key={status}
            onClick={() => setFilter(status)}
            className={clsx(
              'px-4 py-2 rounded-lg text-sm font-medium transition-colors',
              filter === status
                ? 'bg-dark-600 text-text-primary'
                : 'text-text-secondary hover:text-text-primary hover:bg-dark-700'
            )}
          >
            {status.charAt(0).toUpperCase() + status.slice(1)}
            <span className="ml-2 text-text-muted">
              ({status === 'all' ? sessions.length : sessions.filter((s) => s.status === status).length})
            </span>
          </button>
        ))}
      </div>

      {/* Sessions table */}
      <div className="card overflow-hidden">
        <table className="w-full">
          <thead className="bg-dark-800">
            <tr>
              <th className="table-header">Host</th>
              <th className="table-header">OS</th>
              <th className="table-header">User</th>
              <th className="table-header">Credentials</th>
              <th className="table-header">Last Seen</th>
              <th className="table-header w-12"></th>
            </tr>
          </thead>
          <tbody>
            {filteredSessions.map((session) => (
              <SessionRow
                key={session.id}
                session={session}
                onSelect={setSelectedSession}
              />
            ))}
          </tbody>
        </table>

        {filteredSessions.length === 0 && (
          <div className="p-8 text-center text-text-muted">
            No sessions found
          </div>
        )}
      </div>

      {/* Session detail modal */}
      {selectedSession && (
        <SessionModal
          session={selectedSession}
          onClose={() => setSelectedSession(null)}
        />
      )}
    </div>
  );
}
