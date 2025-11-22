import { useState } from 'react';
import { useDashboardStore } from '../store';
import { useApi } from '../hooks/useApi';
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
  Loader2,
  X,
  CheckCircle,
  AlertCircle,
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
  onAction: (action: string, session: Session) => void;
  loadingAction: string | null;
}

function SessionRow({ session, onSelect, onAction, loadingAction }: SessionRowProps) {
  const [menuOpen, setMenuOpen] = useState(false);
  const { setActiveTab, selectSession } = useDashboardStore();

  const handleTerminal = () => {
    selectSession(session.id);
    setActiveTab('terminal');
    setMenuOpen(false);
  };

  const handleAction = (action: string) => {
    setMenuOpen(false);
    onAction(action, session);
  };

  const isLoading = loadingAction !== null;

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
                  disabled={isLoading}
                  className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary disabled:opacity-50"
                >
                  <Terminal size={16} />
                  Execute Command
                </button>
                <button
                  onClick={() => handleAction('privesc')}
                  disabled={isLoading}
                  className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary disabled:opacity-50"
                >
                  {loadingAction === 'privesc' ? <Loader2 size={16} className="animate-spin" /> : <Shield size={16} />}
                  Escalate Privileges
                </button>
                <button
                  onClick={() => handleAction('credentials')}
                  disabled={isLoading}
                  className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary disabled:opacity-50"
                >
                  {loadingAction === 'credentials' ? <Loader2 size={16} className="animate-spin" /> : <Key size={16} />}
                  Harvest Credentials
                </button>
                <button
                  onClick={() => handleAction('lateral')}
                  disabled={isLoading}
                  className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary disabled:opacity-50"
                >
                  {loadingAction === 'lateral' ? <Loader2 size={16} className="animate-spin" /> : <ArrowRight size={16} />}
                  Lateral Movement
                </button>
                <hr className="my-1 border-dark-500" />
                <button
                  onClick={() => onSelect(session)}
                  disabled={isLoading}
                  className="w-full flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:bg-dark-600 hover:text-text-primary disabled:opacity-50"
                >
                  <Eye size={16} />
                  View Details
                </button>
                <button
                  onClick={() => handleAction('terminate')}
                  disabled={isLoading}
                  className="w-full flex items-center gap-2 px-3 py-2 text-sm text-danger hover:bg-dark-600 disabled:opacity-50"
                >
                  {loadingAction === 'terminate' ? <Loader2 size={16} className="animate-spin" /> : <Trash2 size={16} />}
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

// Result modal for showing action output
interface ActionResultModalProps {
  title: string;
  success: boolean;
  output: string;
  onClose: () => void;
}

function ActionResultModal({ title, success, output, onClose }: ActionResultModalProps) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div className="absolute inset-0 bg-black/60" onClick={onClose} />
      <div className="relative bg-dark-700 border border-dark-500 rounded-lg w-full max-w-2xl max-h-[80vh] overflow-auto animate-fade-in">
        <div className="sticky top-0 bg-dark-700 border-b border-dark-500 p-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            {success ? (
              <CheckCircle className="text-ferox-green" size={24} />
            ) : (
              <AlertCircle className="text-danger" size={24} />
            )}
            <h2 className="text-lg font-semibold text-text-primary">{title}</h2>
          </div>
          <button
            onClick={onClose}
            className="text-text-secondary hover:text-text-primary"
          >
            <X size={20} />
          </button>
        </div>
        <div className="p-4">
          <pre className="bg-dark-900 rounded-lg p-4 text-sm text-ferox-green font-mono whitespace-pre-wrap overflow-x-auto">
            {output}
          </pre>
        </div>
      </div>
    </div>
  );
}

// Lateral movement input modal
interface LateralMoveModalProps {
  session: Session;
  onClose: () => void;
  onSubmit: (targetHost: string) => void;
  isLoading: boolean;
}

function LateralMoveModal({ session, onClose, onSubmit, isLoading }: LateralMoveModalProps) {
  const [targetHost, setTargetHost] = useState('');

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div className="absolute inset-0 bg-black/60" onClick={onClose} />
      <div className="relative bg-dark-700 border border-dark-500 rounded-lg w-full max-w-md animate-fade-in">
        <div className="border-b border-dark-500 p-4">
          <h2 className="text-lg font-semibold text-text-primary">Lateral Movement</h2>
          <p className="text-sm text-text-secondary mt-1">
            From: {session.hostname} ({session.ip_address})
          </p>
        </div>
        <div className="p-4 space-y-4">
          <div>
            <label className="block text-sm text-text-secondary mb-2">Target Host</label>
            <input
              type="text"
              value={targetHost}
              onChange={(e) => setTargetHost(e.target.value)}
              placeholder="192.168.1.50 or WS-DEV01"
              className="w-full bg-dark-800 border border-dark-500 rounded-lg px-3 py-2 text-text-primary placeholder:text-text-muted focus:outline-none focus:border-ferox-green"
            />
          </div>
          <div className="flex gap-3 justify-end">
            <button
              onClick={onClose}
              disabled={isLoading}
              className="px-4 py-2 text-sm text-text-secondary hover:text-text-primary disabled:opacity-50"
            >
              Cancel
            </button>
            <button
              onClick={() => onSubmit(targetHost)}
              disabled={!targetHost || isLoading}
              className="px-4 py-2 text-sm bg-ferox-green text-dark-900 rounded-lg hover:bg-ferox-green/90 disabled:opacity-50 flex items-center gap-2"
            >
              {isLoading && <Loader2 size={16} className="animate-spin" />}
              Execute
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

export function SessionsPage() {
  const { sessions, removeSession } = useDashboardStore();
  const api = useApi();
  const [selectedSession, setSelectedSession] = useState<Session | null>(null);
  const [filter, setFilter] = useState<'all' | 'active' | 'sleeping' | 'dead'>('all');

  // Action states
  const [loadingAction, setLoadingAction] = useState<string | null>(null);
  const [actionResult, setActionResult] = useState<{ title: string; success: boolean; output: string } | null>(null);
  const [lateralMoveSession, setLateralMoveSession] = useState<Session | null>(null);

  const filteredSessions = sessions.filter(
    (s) => filter === 'all' || s.status === filter
  );

  // Handle quick actions
  const handleAction = async (action: string, session: Session) => {
    if (action === 'lateral') {
      setLateralMoveSession(session);
      return;
    }

    setLoadingAction(action);

    try {
      switch (action) {
        case 'privesc': {
          const result = await api.runPrivEsc({
            session_id: session.id,
            auto_escalate: true,
            safe_mode: false,
          });
          setActionResult({
            title: 'Privilege Escalation',
            success: result.escalation_success,
            output: result.output,
          });
          break;
        }
        case 'credentials': {
          const result = await api.harvestCredentials({
            session_id: session.id,
            sources: ['all'],
            safe_mode: false,
          });
          setActionResult({
            title: 'Credential Harvesting',
            success: result.total_found > 0,
            output: result.output,
          });
          break;
        }
        case 'terminate': {
          await api.terminateSession(session.id);
          // Update local state immediately
          removeSession(session.id);
          setActionResult({
            title: 'Session Terminated',
            success: true,
            output: `Session ${session.hostname} (${session.id}) has been terminated.`,
          });
          break;
        }
      }
    } catch (error) {
      setActionResult({
        title: 'Error',
        success: false,
        output: error instanceof Error ? error.message : 'An unknown error occurred',
      });
    } finally {
      setLoadingAction(null);
    }
  };

  // Handle lateral movement submission
  const handleLateralMove = async (targetHost: string) => {
    if (!lateralMoveSession) return;

    setLoadingAction('lateral');

    try {
      const result = await api.lateralMove({
        session_id: lateralMoveSession.id,
        target_host: targetHost,
        method: 'auto',
        safe_mode: false,
      });
      setActionResult({
        title: 'Lateral Movement',
        success: result.success,
        output: result.output,
      });
    } catch (error) {
      setActionResult({
        title: 'Error',
        success: false,
        output: error instanceof Error ? error.message : 'An unknown error occurred',
      });
    } finally {
      setLoadingAction(null);
      setLateralMoveSession(null);
    }
  };

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
                onAction={handleAction}
                loadingAction={loadingAction}
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

      {/* Action result modal */}
      {actionResult && (
        <ActionResultModal
          title={actionResult.title}
          success={actionResult.success}
          output={actionResult.output}
          onClose={() => setActionResult(null)}
        />
      )}

      {/* Lateral movement input modal */}
      {lateralMoveSession && (
        <LateralMoveModal
          session={lateralMoveSession}
          onClose={() => setLateralMoveSession(null)}
          onSubmit={handleLateralMove}
          isLoading={loadingAction === 'lateral'}
        />
      )}
    </div>
  );
}
