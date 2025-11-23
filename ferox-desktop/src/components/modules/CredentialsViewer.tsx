/**
 * CredentialsViewer - Simulated Credential Dump Results
 * For demo/training purposes only - all data is fake
 */

import { useState } from 'react';
import { KeyRound, Eye, EyeOff, Copy, Shield, User, Hash, Key, Database } from 'lucide-react';
import { clsx } from 'clsx';
import toast from 'react-hot-toast';

interface SimulatedCredential {
  id: string;
  type: 'password' | 'hash' | 'token' | 'certificate';
  username: string;
  domain: string | null;
  value: string;
  source: string;
  sensitivity: 'low' | 'medium' | 'high' | 'critical';
}

const SIMULATED_CREDENTIALS: SimulatedCredential[] = [
  {
    id: '1',
    type: 'hash',
    username: 'Administrator',
    domain: 'CORP',
    value: 'aad3b435b51404eeaad3b435b51404ee:31d6cfe0d16ae931b73c59d7e0c089c0',
    source: 'LSASS Memory',
    sensitivity: 'critical',
  },
  {
    id: '2',
    type: 'password',
    username: 'svc_backup',
    domain: 'CORP',
    value: 'B@ckup2024!',
    source: 'Credential Manager',
    sensitivity: 'high',
  },
  {
    id: '3',
    type: 'hash',
    username: 'john.doe',
    domain: 'CORP',
    value: 'aad3b435b51404eeaad3b435b51404ee:e19ccf75ee54e06b06a5907af13cef42',
    source: 'SAM Database',
    sensitivity: 'medium',
  },
  {
    id: '4',
    type: 'token',
    username: 'api-service',
    domain: null,
    value: 'ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
    source: 'Environment Variables',
    sensitivity: 'high',
  },
  {
    id: '5',
    type: 'password',
    username: 'admin',
    domain: null,
    value: 'admin123',
    source: 'Browser (Chrome)',
    sensitivity: 'low',
  },
  {
    id: '6',
    type: 'certificate',
    username: 'web-server',
    domain: 'corp.local',
    value: '-----BEGIN CERTIFICATE-----\nMIIBkTCB+wIJAKHBfp...',
    source: 'Certificate Store',
    sensitivity: 'high',
  },
];

interface CredentialsViewerProps {
  sessionId: string;
}

export function CredentialsViewer({ sessionId: _sessionId }: CredentialsViewerProps) {
  const [credentials] = useState<SimulatedCredential[]>(SIMULATED_CREDENTIALS);
  const [selectedType, setSelectedType] = useState<string | null>(null);
  const [showValues, setShowValues] = useState<Record<string, boolean>>({});
  const [searchQuery, setSearchQuery] = useState('');

  const filteredCredentials = credentials.filter(cred => {
    const matchesType = !selectedType || cred.type === selectedType;
    const matchesSearch = !searchQuery ||
      cred.username.toLowerCase().includes(searchQuery.toLowerCase()) ||
      cred.domain?.toLowerCase().includes(searchQuery.toLowerCase()) ||
      cred.source.toLowerCase().includes(searchQuery.toLowerCase());
    return matchesType && matchesSearch;
  });

  const toggleValue = (id: string) => {
    setShowValues(prev => ({ ...prev, [id]: !prev[id] }));
  };

  const copyValue = (value: string) => {
    navigator.clipboard.writeText(value);
    toast.success('Copied to clipboard');
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'password': return <Key size={14} className="text-green-400" />;
      case 'hash': return <Hash size={14} className="text-purple-400" />;
      case 'token': return <Shield size={14} className="text-blue-400" />;
      case 'certificate': return <Database size={14} className="text-yellow-400" />;
      default: return <Key size={14} />;
    }
  };

  const getSensitivityColor = (sensitivity: string) => {
    switch (sensitivity) {
      case 'critical': return 'bg-red-500/20 text-red-400';
      case 'high': return 'bg-orange-500/20 text-orange-400';
      case 'medium': return 'bg-yellow-500/20 text-yellow-400';
      case 'low': return 'bg-green-500/20 text-green-400';
      default: return 'bg-dark-600 text-text-muted';
    }
  };

  const credTypes = [
    { id: 'password', label: 'Passwords', count: credentials.filter(c => c.type === 'password').length },
    { id: 'hash', label: 'Hashes', count: credentials.filter(c => c.type === 'hash').length },
    { id: 'token', label: 'Tokens', count: credentials.filter(c => c.type === 'token').length },
    { id: 'certificate', label: 'Certs', count: credentials.filter(c => c.type === 'certificate').length },
  ];

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center gap-2">
          <KeyRound className="text-yellow-400" size={20} />
          <h2 className="text-lg font-semibold text-text-primary">Credentials Viewer</h2>
          <span className="text-xs bg-yellow-500/20 text-yellow-400 px-2 py-0.5 rounded">SIMULATION</span>
        </div>
        <p className="text-xs text-text-muted mt-1">Simulated credential dump for demo/training (all data is fake)</p>
      </div>

      {/* Filters */}
      <div className="p-4 border-b border-dark-600 flex items-center gap-4">
        <input
          type="text"
          value={searchQuery}
          onChange={e => setSearchQuery(e.target.value)}
          placeholder="Search credentials..."
          className="flex-1 max-w-xs px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary focus:border-yellow-400/50 focus:outline-none"
        />
        <div className="flex items-center gap-2">
          {credTypes.map(type => (
            <button
              key={type.id}
              onClick={() => setSelectedType(selectedType === type.id ? null : type.id)}
              className={clsx(
                'px-3 py-1.5 rounded text-xs font-medium transition-colors',
                selectedType === type.id
                  ? 'bg-yellow-500/20 text-yellow-400 border border-yellow-500/50'
                  : 'bg-dark-700 text-text-secondary border border-dark-600 hover:border-dark-500'
              )}
            >
              {type.label} ({type.count})
            </button>
          ))}
        </div>
      </div>

      {/* Credentials List */}
      <div className="flex-1 overflow-y-auto p-4">
        <div className="space-y-3">
          {filteredCredentials.map(cred => (
            <div key={cred.id} className="bg-dark-800 rounded-lg p-4 border border-dark-600">
              <div className="flex items-start justify-between">
                <div className="flex items-center gap-3">
                  {getTypeIcon(cred.type)}
                  <div>
                    <div className="flex items-center gap-2">
                      <User size={12} className="text-text-muted" />
                      <span className="text-sm font-medium text-text-primary">
                        {cred.domain ? `${cred.domain}\\${cred.username}` : cred.username}
                      </span>
                    </div>
                    <div className="text-xs text-text-muted mt-1">Source: {cred.source}</div>
                  </div>
                </div>
                <span className={clsx('text-xs px-2 py-0.5 rounded', getSensitivityColor(cred.sensitivity))}>
                  {cred.sensitivity}
                </span>
              </div>

              <div className="mt-3 flex items-center gap-2">
                <code className="flex-1 px-3 py-2 bg-dark-900 rounded text-xs font-mono text-ferox-green overflow-x-auto">
                  {showValues[cred.id] ? cred.value : '••••••••••••••••••••'}
                </code>
                <button
                  onClick={() => toggleValue(cred.id)}
                  className="p-2 hover:bg-dark-700 rounded transition-colors"
                  title={showValues[cred.id] ? 'Hide value' : 'Show value'}
                >
                  {showValues[cred.id] ? (
                    <EyeOff size={14} className="text-text-muted" />
                  ) : (
                    <Eye size={14} className="text-text-muted" />
                  )}
                </button>
                <button
                  onClick={() => copyValue(cred.value)}
                  className="p-2 hover:bg-dark-700 rounded transition-colors"
                  title="Copy to clipboard"
                >
                  <Copy size={14} className="text-text-muted" />
                </button>
              </div>
            </div>
          ))}
        </div>

        {filteredCredentials.length === 0 && (
          <div className="h-full flex items-center justify-center text-text-muted">
            <div className="text-center">
              <KeyRound size={48} className="mx-auto mb-4 opacity-20" />
              <p>No credentials match your filters</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default CredentialsViewer;
