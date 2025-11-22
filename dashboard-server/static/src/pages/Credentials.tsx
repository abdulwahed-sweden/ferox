import { useState } from 'react';
import { useDashboardStore } from '../store';
import {
  Key,
  Hash,
  Ticket,
  FileKey,
  Cloud,
  Search,
  Eye,
  EyeOff,
  Copy,
  Target,
  Trash2,
  ChevronRight,
  Shield,
  AlertTriangle,
  CheckCircle2,
  Clock,
  StickyNote,
} from 'lucide-react';
import { clsx } from 'clsx';
import type { Credential, CredentialType, Sensitivity } from '../types';

const credTypeIcons: Record<CredentialType, typeof Key> = {
  plain_text: Key,
  ntlm_hash: Hash,
  kerberos_ticket: Ticket,
  ssh_key: FileKey,
  cloud_credential: Cloud,
  token: Key,
  certificate: FileKey,
};

const credTypeLabels: Record<CredentialType, string> = {
  plain_text: 'Plain Text Password',
  ntlm_hash: 'NTLM Hash',
  kerberos_ticket: 'Kerberos Ticket',
  ssh_key: 'SSH Key',
  cloud_credential: 'Cloud Credential',
  token: 'Token',
  certificate: 'Certificate',
};

const credTypeEmoji: Record<CredentialType, string> = {
  plain_text: '🔑',
  ntlm_hash: '#️⃣',
  kerberos_ticket: '🎫',
  ssh_key: '🔐',
  cloud_credential: '🌐',
  token: '🎟️',
  certificate: '📜',
};

interface CredentialListItemProps {
  credential: Credential;
  isSelected: boolean;
  onClick: () => void;
}

function CredentialListItem({ credential, isSelected, onClick }: CredentialListItemProps) {
  const Icon = credTypeIcons[credential.cred_type];

  return (
    <button
      onClick={onClick}
      className={clsx(
        'w-full text-left p-3 rounded-lg border transition-all',
        isSelected
          ? 'bg-dark-600 border-ferox-green'
          : 'bg-dark-800 border-dark-600 hover:border-dark-400'
      )}
    >
      <div className="flex items-start gap-3">
        <div
          className={clsx(
            'p-2 rounded-lg',
            isSelected ? 'bg-ferox-green/20' : 'bg-dark-700'
          )}
        >
          <Icon size={16} className={isSelected ? 'text-ferox-green' : 'text-text-secondary'} />
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center justify-between gap-2">
            <p className="font-medium text-text-primary truncate">{credential.username}</p>
            <span
              className={clsx(
                'badge text-xs',
                credential.sensitivity === 'critical' && 'badge-danger',
                credential.sensitivity === 'high' && 'badge-warning',
                credential.sensitivity === 'medium' && 'badge-info',
                credential.sensitivity === 'low' && 'badge-gray'
              )}
            >
              {credential.sensitivity}
            </span>
          </div>
          {credential.domain && (
            <p className="text-xs text-text-muted truncate">{credential.domain}</p>
          )}
          <div className="flex items-center gap-2 mt-1">
            <span className="text-xs text-text-muted">{credential.source_hostname}</span>
            {credential.is_reusable && (
              <span className="text-xs text-ferox-green">• Reusable</span>
            )}
          </div>
        </div>
        <ChevronRight
          size={16}
          className={clsx('text-text-muted flex-shrink-0', isSelected && 'text-ferox-green')}
        />
      </div>
    </button>
  );
}

interface CredentialDetailsPanelProps {
  credential: Credential;
  showSecret: boolean;
  onToggleSecret: () => void;
}

function CredentialDetailsPanel({
  credential,
  showSecret,
  onToggleSecret,
}: CredentialDetailsPanelProps) {
  const Icon = credTypeIcons[credential.cred_type];
  const [note, setNote] = useState(credential.notes || '');
  const [showNoteInput, setShowNoteInput] = useState(false);

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-dark-600">
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-dark-700 rounded-lg">
              <Icon size={24} className="text-info" />
            </div>
            <div>
              <h3 className="text-lg font-semibold text-text-primary">{credential.username}</h3>
              {credential.domain && (
                <p className="text-sm text-text-secondary">{credential.domain}</p>
              )}
            </div>
          </div>
          <span
            className={clsx(
              'badge',
              credential.sensitivity === 'critical' && 'badge-danger',
              credential.sensitivity === 'high' && 'badge-warning',
              credential.sensitivity === 'medium' && 'badge-info',
              credential.sensitivity === 'low' && 'badge-gray'
            )}
          >
            {credential.sensitivity}
          </span>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-auto p-4 space-y-4">
        {/* Credential Type */}
        <div className="p-3 bg-dark-800 rounded-lg">
          <p className="text-xs text-text-muted mb-1">Credential Type</p>
          <div className="flex items-center gap-2">
            <span className="text-lg">{credTypeEmoji[credential.cred_type]}</span>
            <span className="text-text-primary font-medium">
              {credTypeLabels[credential.cred_type]}
            </span>
          </div>
        </div>

        {/* Secret */}
        <div className="p-3 bg-dark-800 rounded-lg">
          <div className="flex items-center justify-between mb-2">
            <p className="text-xs text-text-muted">Secret</p>
            <div className="flex items-center gap-1">
              <button
                onClick={onToggleSecret}
                className="p-1.5 rounded hover:bg-dark-600 text-text-muted hover:text-text-primary transition-colors"
                title={showSecret ? 'Hide secret' : 'Show secret'}
              >
                {showSecret ? <EyeOff size={14} /> : <Eye size={14} />}
              </button>
              <button
                onClick={() => copyToClipboard(credential.secret)}
                className="p-1.5 rounded hover:bg-dark-600 text-text-muted hover:text-text-primary transition-colors"
                title="Copy to clipboard"
              >
                <Copy size={14} />
              </button>
            </div>
          </div>
          <code className="text-sm text-ferox-green break-all font-mono block bg-dark-900 p-2 rounded">
            {showSecret ? credential.secret : '•'.repeat(Math.min(credential.secret.length, 24))}
          </code>
        </div>

        {/* Source Information */}
        <div className="space-y-3">
          <h4 className="text-sm font-medium text-text-primary">Source Information</h4>
          <div className="grid grid-cols-2 gap-3">
            <div className="p-3 bg-dark-800 rounded-lg">
              <p className="text-xs text-text-muted mb-1">Hostname</p>
              <p className="text-text-primary text-sm">{credential.source_hostname}</p>
            </div>
            <div className="p-3 bg-dark-800 rounded-lg">
              <p className="text-xs text-text-muted mb-1">Collected</p>
              <p className="text-text-primary text-sm flex items-center gap-1">
                <Clock size={12} />
                {new Date(credential.collected_at).toLocaleString()}
              </p>
            </div>
          </div>
        </div>

        {/* Status */}
        <div className="space-y-3">
          <h4 className="text-sm font-medium text-text-primary">Status</h4>
          <div className="flex items-center gap-4">
            <div
              className={clsx(
                'flex items-center gap-2 px-3 py-2 rounded-lg',
                credential.is_reusable ? 'bg-ferox-green/10' : 'bg-dark-800'
              )}
            >
              {credential.is_reusable ? (
                <CheckCircle2 size={16} className="text-ferox-green" />
              ) : (
                <AlertTriangle size={16} className="text-text-muted" />
              )}
              <span className={credential.is_reusable ? 'text-ferox-green' : 'text-text-muted'}>
                {credential.is_reusable ? 'Reusable' : 'Not Reusable'}
              </span>
            </div>
          </div>
        </div>

        {/* MITRE Technique */}
        <div className="space-y-3">
          <h4 className="text-sm font-medium text-text-primary">MITRE ATT&CK</h4>
          <div className="flex flex-wrap gap-2">
            <span className="badge bg-purple-400/20 text-purple-400">T1555</span>
            <span className="badge bg-purple-400/20 text-purple-400">T1552</span>
            <span className="badge bg-purple-400/20 text-purple-400">T1003</span>
          </div>
        </div>

        {/* Notes */}
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <h4 className="text-sm font-medium text-text-primary">Notes</h4>
            <button
              onClick={() => setShowNoteInput(!showNoteInput)}
              className="text-xs text-ferox-green hover:underline"
            >
              {showNoteInput ? 'Cancel' : credential.notes ? 'Edit' : 'Add Note'}
            </button>
          </div>
          {showNoteInput ? (
            <div className="space-y-2">
              <textarea
                value={note}
                onChange={(e) => setNote(e.target.value)}
                className="input w-full h-24 resize-none"
                placeholder="Add a note about this credential..."
              />
              <button className="btn-primary text-sm py-1.5">
                <StickyNote size={14} />
                Save Note
              </button>
            </div>
          ) : credential.notes ? (
            <p className="text-sm text-text-secondary p-3 bg-dark-800 rounded-lg">
              {credential.notes}
            </p>
          ) : (
            <p className="text-sm text-text-muted italic">No notes added</p>
          )}
        </div>
      </div>

      {/* Actions */}
      <div className="p-4 border-t border-dark-600 space-y-2">
        <button className="btn-primary w-full justify-center">
          <Target size={16} />
          Test Against Targets
        </button>
        <div className="flex gap-2">
          <button className="btn-outline flex-1 justify-center">
            <Shield size={16} />
            Use for Lateral Movement
          </button>
          <button className="btn-ghost text-danger p-2">
            <Trash2 size={18} />
          </button>
        </div>
      </div>
    </div>
  );
}

export function CredentialsPage() {
  const { credentials } = useDashboardStore();
  const [search, setSearch] = useState('');
  const [typeFilter, setTypeFilter] = useState<CredentialType | 'all'>('all');
  const [sensitivityFilter, setSensitivityFilter] = useState<Sensitivity | 'all'>('all');
  const [showReusableOnly, setShowReusableOnly] = useState(false);
  const [selectedCredential, setSelectedCredential] = useState<Credential | null>(null);
  const [visibleSecrets, setVisibleSecrets] = useState<Set<string>>(new Set());

  const toggleSecret = (id: string) => {
    setVisibleSecrets((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const filteredCredentials = credentials.filter((cred) => {
    const matchesSearch =
      search === '' ||
      cred.username.toLowerCase().includes(search.toLowerCase()) ||
      cred.source_hostname.toLowerCase().includes(search.toLowerCase()) ||
      (cred.domain?.toLowerCase().includes(search.toLowerCase()) ?? false);

    const matchesType = typeFilter === 'all' || cred.cred_type === typeFilter;
    const matchesSensitivity =
      sensitivityFilter === 'all' || cred.sensitivity === sensitivityFilter;
    const matchesReusable = !showReusableOnly || cred.is_reusable;

    return matchesSearch && matchesType && matchesSensitivity && matchesReusable;
  });

  // Group by type for the sidebar
  const groupedByType = filteredCredentials.reduce((acc, cred) => {
    const type = cred.cred_type;
    if (!acc[type]) acc[type] = [];
    acc[type].push(cred);
    return acc;
  }, {} as Record<CredentialType, Credential[]>);

  return (
    <div className="h-[calc(100vh-8rem)] flex flex-col gap-4 animate-fade-in">
      {/* Top Stats Bar */}
      <div className="grid grid-cols-4 gap-4">
        <div className="card text-center py-3">
          <p className="text-2xl font-bold text-text-primary">{credentials.length}</p>
          <p className="text-xs text-text-secondary">Total</p>
        </div>
        <div className="card text-center py-3">
          <p className="text-2xl font-bold text-danger">
            {credentials.filter((c) => c.sensitivity === 'critical').length}
          </p>
          <p className="text-xs text-text-secondary">Critical</p>
        </div>
        <div className="card text-center py-3">
          <p className="text-2xl font-bold text-ferox-green">
            {credentials.filter((c) => c.is_reusable).length}
          </p>
          <p className="text-xs text-text-secondary">Reusable</p>
        </div>
        <div className="card text-center py-3">
          <p className="text-2xl font-bold text-info">
            {new Set(credentials.map((c) => c.source_hostname)).size}
          </p>
          <p className="text-xs text-text-secondary">Sources</p>
        </div>
      </div>

      {/* Search and Filters */}
      <div className="flex flex-wrap gap-3 items-center">
        <div className="flex-1 min-w-[200px] relative">
          <Search
            size={18}
            className="absolute left-3 top-1/2 -translate-y-1/2 text-text-muted"
          />
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search by username, domain, or hostname..."
            className="input w-full pl-10"
          />
        </div>

        <select
          value={typeFilter}
          onChange={(e) => setTypeFilter(e.target.value as CredentialType | 'all')}
          className="input"
        >
          <option value="all">All Types</option>
          {Object.entries(credTypeLabels).map(([value, label]) => (
            <option key={value} value={value}>
              {credTypeEmoji[value as CredentialType]} {label}
            </option>
          ))}
        </select>

        <select
          value={sensitivityFilter}
          onChange={(e) => setSensitivityFilter(e.target.value as Sensitivity | 'all')}
          className="input"
        >
          <option value="all">All Sensitivity</option>
          <option value="critical">Critical</option>
          <option value="high">High</option>
          <option value="medium">Medium</option>
          <option value="low">Low</option>
        </select>

        <label className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer">
          <input
            type="checkbox"
            checked={showReusableOnly}
            onChange={(e) => setShowReusableOnly(e.target.checked)}
            className="rounded border-dark-500"
          />
          Reusable only
        </label>
      </div>

      {/* Two Column Layout */}
      <div className="flex-1 flex gap-4 min-h-0">
        {/* Left: Credential List */}
        <div className="w-96 flex flex-col bg-dark-700 rounded-lg border border-dark-600 overflow-hidden">
          <div className="p-3 border-b border-dark-600 bg-dark-800">
            <h3 className="font-medium text-text-primary">
              Credentials ({filteredCredentials.length})
            </h3>
          </div>
          <div className="flex-1 overflow-auto p-2 space-y-1">
            {Object.entries(groupedByType).length > 0 ? (
              Object.entries(groupedByType).map(([type, creds]) => (
                <div key={type} className="mb-3">
                  <div className="flex items-center gap-2 px-2 py-1.5 text-sm text-text-secondary">
                    <span>{credTypeEmoji[type as CredentialType]}</span>
                    <span>{credTypeLabels[type as CredentialType]}</span>
                    <span className="badge badge-gray ml-auto">{creds.length}</span>
                  </div>
                  <div className="space-y-1">
                    {creds.map((cred) => (
                      <CredentialListItem
                        key={cred.id}
                        credential={cred}
                        isSelected={selectedCredential?.id === cred.id}
                        onClick={() => setSelectedCredential(cred)}
                      />
                    ))}
                  </div>
                </div>
              ))
            ) : (
              <div className="text-center py-8 text-text-muted">
                <Key size={32} className="mx-auto mb-2 opacity-50" />
                <p>No credentials found</p>
              </div>
            )}
          </div>
        </div>

        {/* Right: Credential Details */}
        <div className="flex-1 bg-dark-700 rounded-lg border border-dark-600 overflow-hidden">
          {selectedCredential ? (
            <CredentialDetailsPanel
              credential={selectedCredential}
              showSecret={visibleSecrets.has(selectedCredential.id)}
              onToggleSecret={() => toggleSecret(selectedCredential.id)}
            />
          ) : (
            <div className="h-full flex flex-col items-center justify-center text-text-muted">
              <Key size={48} className="mb-4 opacity-50" />
              <p className="text-lg">Select a credential to view details</p>
              <p className="text-sm mt-2">
                Click on any credential in the list to see more information
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Intelligence Panel - Bottom */}
      {credentials.length > 0 && (
        <div className="card">
          <h3 className="text-sm font-medium text-text-primary mb-3">Intelligence</h3>
          <div className="flex flex-wrap gap-4 text-sm">
            {credentials.filter((c) => c.sensitivity === 'critical').length > 0 && (
              <div className="flex items-center gap-2 px-3 py-1.5 bg-danger/10 rounded-lg text-danger">
                <AlertTriangle size={14} />
                <span>
                  {credentials.filter((c) => c.sensitivity === 'critical').length} domain admin
                  credential(s) found
                </span>
              </div>
            )}
            {(() => {
              const usernames = credentials.map((c) => c.username);
              const duplicates = usernames.filter(
                (u, i) => usernames.indexOf(u) !== i
              ).length;
              return duplicates > 0 ? (
                <div className="flex items-center gap-2 px-3 py-1.5 bg-warning/10 rounded-lg text-warning">
                  <AlertTriangle size={14} />
                  <span>Password reuse detected ({duplicates} users)</span>
                </div>
              ) : null;
            })()}
            {credentials.filter((c) => c.cred_type === 'cloud_credential').length > 0 && (
              <div className="flex items-center gap-2 px-3 py-1.5 bg-info/10 rounded-lg text-info">
                <Cloud size={14} />
                <span>
                  {credentials.filter((c) => c.cred_type === 'cloud_credential').length} cloud
                  credential(s) available
                </span>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
