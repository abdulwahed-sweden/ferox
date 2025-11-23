/**
 * CredentialsViewer - Simulated Credential Dump Results
 * For demo/training purposes only - all data is fake
 */

import { useState, useEffect } from 'react';
import { KeyRound, Eye, EyeOff, Copy, Shield, User, Hash, Key, Database, RefreshCw, Clock, CheckCircle } from 'lucide-react';
import { clsx } from 'clsx';
import toast from 'react-hot-toast';
import { simulateCredentialDump } from '../../lib/tauri';
import type { SimulatedCredential, CredentialDumpResult } from '../../types';

interface CredentialsViewerProps {
  sessionId: string;
}

export function CredentialsViewer({ sessionId }: CredentialsViewerProps) {
  const [result, setResult] = useState<CredentialDumpResult | null>(null);
  const [credentials, setCredentials] = useState<SimulatedCredential[]>([]);
  const [selectedType, setSelectedType] = useState<string | null>(null);
  const [showValues, setShowValues] = useState<Record<string, boolean>>({});
  const [searchQuery, setSearchQuery] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const loadCredentials = async () => {
    setIsLoading(true);
    try {
      toast.loading('Harvesting credentials...', { id: 'creds' });
      const data = await simulateCredentialDump(sessionId, []);
      setResult(data);
      setCredentials(data.credentials);
      toast.success(`Found ${data.total_found} credentials`, { id: 'creds' });
    } catch (error) {
      console.error('Failed to load credentials:', error);
      toast.error('Failed to harvest credentials', { id: 'creds' });
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadCredentials();
  }, [sessionId]);

  const filteredCredentials = credentials.filter(cred => {
    const matchesType = !selectedType || cred.cred_type === selectedType;
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
      case 'ticket': return <KeyRound size={14} className="text-orange-400" />;
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
    { id: 'password', label: 'Passwords', count: credentials.filter(c => c.cred_type === 'password').length },
    { id: 'hash', label: 'Hashes', count: credentials.filter(c => c.cred_type === 'hash').length },
    { id: 'token', label: 'Tokens', count: credentials.filter(c => c.cred_type === 'token').length },
    { id: 'certificate', label: 'Certs', count: credentials.filter(c => c.cred_type === 'certificate').length },
    { id: 'ticket', label: 'Tickets', count: credentials.filter(c => c.cred_type === 'ticket').length },
  ];

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <KeyRound className="text-yellow-400" size={20} />
            <h2 className="text-lg font-semibold text-text-primary">Credentials Viewer</h2>
            <span className="text-xs bg-yellow-500/20 text-yellow-400 px-2 py-0.5 rounded">SIMULATION</span>
          </div>
          <button
            onClick={loadCredentials}
            disabled={isLoading}
            className="px-3 py-1.5 bg-yellow-500/20 text-yellow-400 rounded text-xs font-medium flex items-center gap-1.5 hover:bg-yellow-500/30 transition-colors disabled:opacity-50"
          >
            <RefreshCw size={12} className={isLoading ? 'animate-spin' : ''} />
            Refresh
          </button>
        </div>
        <p className="text-xs text-text-muted mt-1">Simulated credential dump for demo/training (all data is fake)</p>
      </div>

      {/* Stats */}
      {result && (
        <div className="p-3 border-b border-dark-600 grid grid-cols-4 gap-3">
          {Object.entries(result.by_sensitivity).map(([level, count]) => (
            <div key={level} className={clsx('rounded p-2 text-center', getSensitivityColor(level))}>
              <div className="text-lg font-bold">{count}</div>
              <div className="text-xs capitalize">{level}</div>
            </div>
          ))}
        </div>
      )}

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
                  {getTypeIcon(cred.cred_type)}
                  <div>
                    <div className="flex items-center gap-2">
                      <User size={12} className="text-text-muted" />
                      <span className="text-sm font-medium text-text-primary">
                        {cred.domain ? `${cred.domain}\\${cred.username}` : cred.username}
                      </span>
                      {cred.cracked && (
                        <span title="Cracked"><CheckCircle size={12} className="text-green-400" /></span>
                      )}
                    </div>
                    <div className="text-xs text-text-muted mt-1">Source: {cred.source}</div>
                    {cred.last_used && (
                      <div className="text-xs text-text-muted flex items-center gap-1 mt-1">
                        <Clock size={10} />
                        Last used: {new Date(cred.last_used).toLocaleDateString()}
                      </div>
                    )}
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

              {cred.cracked && cred.cracked_value && (
                <div className="mt-2 flex items-center gap-2 text-xs">
                  <span className="text-green-400">Cracked:</span>
                  <code className="px-2 py-1 bg-green-500/10 rounded text-green-400 font-mono">
                    {cred.cracked_value}
                  </code>
                </div>
              )}

              {cred.expires_at && (
                <div className="mt-2 text-xs text-text-muted">
                  Expires: {new Date(cred.expires_at).toLocaleString()}
                </div>
              )}
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
