/**
 * PayloadBuilder - Simulated Payload Generation UI
 *
 * This component provides a UI for generating SIMULATED payloads.
 * No actual malware is created - only JSON metadata for demo/training purposes.
 */

import { useState, useEffect } from 'react';
import {
  Package,
  Shield,
  AlertTriangle,
  CheckCircle,
  Info,
  Terminal,
  Copy,
  ChevronDown,
  ChevronUp,
  Loader2,
  FileCode,
  Target,
  Cpu,
  Lock,
  Eye,
  EyeOff,
} from 'lucide-react';
import { clsx } from 'clsx';
import toast from 'react-hot-toast';
import {
  generateSimulatedPayload,
  getPayloadTypes,
  getPayloadFormats
} from '../lib/tauri';
import type {
  PayloadConfig,
  SimulatedPayload,
  PayloadTypeInfo,
  FormatInfo,
  BuildLogEntry,
} from '../types';

export function PayloadBuilder() {
  // Form state
  const [config, setConfig] = useState<PayloadConfig>({
    payload_type: 'reverse_tcp',
    lhost: '192.168.1.100',
    lport: 4444,
    target_os: 'windows',
    format: 'exe',
    architecture: 'x64',
    obfuscation: true,
    signing: false,
    staged: false,
    name: '',
  });

  // UI state
  const [payloadTypes, setPayloadTypes] = useState<PayloadTypeInfo[]>([]);
  const [formats, setFormats] = useState<FormatInfo[]>([]);
  const [isGenerating, setIsGenerating] = useState(false);
  const [result, setResult] = useState<SimulatedPayload | null>(null);
  const [expandedSections, setExpandedSections] = useState<Record<string, boolean>>({
    buildLog: true,
    riskAnalysis: true,
    detection: false,
    mitre: false,
    execution: true,
  });

  // Load payload types and formats on mount
  useEffect(() => {
    const loadOptions = async () => {
      try {
        const [types, fmts] = await Promise.all([
          getPayloadTypes(),
          getPayloadFormats(),
        ]);
        setPayloadTypes(types);
        setFormats(fmts);
      } catch (error) {
        console.error('Failed to load payload options:', error);
        toast.error('Failed to load payload options');
      }
    };
    loadOptions();
  }, []);

  // Filter formats based on selected OS
  const filteredFormats = formats.filter(f =>
    f.os.includes(config.target_os) || f.os.includes('universal')
  );

  // Handle form changes
  const handleChange = (field: keyof PayloadConfig, value: unknown) => {
    setConfig(prev => ({ ...prev, [field]: value }));
  };

  // Generate simulated payload
  const handleGenerate = async () => {
    setIsGenerating(true);
    setResult(null);

    try {
      const payload = await generateSimulatedPayload(config);
      setResult(payload);
      toast.success('Simulated payload generated!');
    } catch (error) {
      console.error('Generation failed:', error);
      toast.error('Failed to generate simulated payload');
    } finally {
      setIsGenerating(false);
    }
  };

  // Copy to clipboard
  const copyToClipboard = (text: string, label: string) => {
    navigator.clipboard.writeText(text);
    toast.success(`${label} copied to clipboard`);
  };

  // Toggle section
  const toggleSection = (section: string) => {
    setExpandedSections(prev => ({ ...prev, [section]: !prev[section] }));
  };

  // Get risk level color
  const getRiskColor = (level: string) => {
    switch (level) {
      case 'low': return 'text-green-400';
      case 'medium': return 'text-yellow-400';
      case 'high': return 'text-orange-400';
      case 'critical': return 'text-red-400';
      default: return 'text-text-muted';
    }
  };

  // Get log level icon
  const getLogIcon = (level: string) => {
    switch (level) {
      case 'success': return <CheckCircle size={14} className="text-green-400" />;
      case 'warn': return <AlertTriangle size={14} className="text-yellow-400" />;
      default: return <Info size={14} className="text-blue-400" />;
    }
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center gap-2">
          <Package className="text-ferox-green" size={20} />
          <h2 className="text-lg font-semibold text-text-primary">
            Payload Builder
          </h2>
          <span className="text-xs bg-yellow-500/20 text-yellow-400 px-2 py-0.5 rounded">
            SIMULATION MODE
          </span>
        </div>
        <p className="text-xs text-text-muted mt-1">
          Generate simulated payload metadata for demo/training. No actual binaries are created.
        </p>
      </div>

      <div className="flex-1 flex overflow-hidden">
        {/* Configuration Panel */}
        <div className="w-80 border-r border-dark-600 overflow-y-auto p-4 space-y-4">
          {/* Payload Type */}
          <div>
            <label className="block text-xs text-text-secondary mb-1">Payload Type</label>
            <select
              value={config.payload_type}
              onChange={e => handleChange('payload_type', e.target.value)}
              className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary focus:border-ferox-green/50 focus:outline-none"
            >
              {payloadTypes.map(type => (
                <option key={type.id} value={type.id}>
                  {type.name} ({type.category})
                </option>
              ))}
            </select>
          </div>

          {/* Target OS */}
          <div>
            <label className="block text-xs text-text-secondary mb-1">Target OS</label>
            <div className="grid grid-cols-3 gap-2">
              {['windows', 'linux', 'macos'].map(os => (
                <button
                  key={os}
                  onClick={() => handleChange('target_os', os)}
                  className={clsx(
                    'px-3 py-2 rounded text-xs font-medium transition-colors',
                    config.target_os === os
                      ? 'bg-ferox-green/20 text-ferox-green border border-ferox-green/50'
                      : 'bg-dark-700 text-text-secondary border border-dark-600 hover:border-dark-500'
                  )}
                >
                  {os.charAt(0).toUpperCase() + os.slice(1)}
                </button>
              ))}
            </div>
          </div>

          {/* Format */}
          <div>
            <label className="block text-xs text-text-secondary mb-1">Output Format</label>
            <select
              value={config.format}
              onChange={e => handleChange('format', e.target.value)}
              className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary focus:border-ferox-green/50 focus:outline-none"
            >
              {filteredFormats.map(fmt => (
                <option key={fmt.id} value={fmt.id}>
                  {fmt.name} ({fmt.extension || 'no ext'})
                </option>
              ))}
            </select>
          </div>

          {/* Architecture */}
          <div>
            <label className="block text-xs text-text-secondary mb-1">Architecture</label>
            <div className="grid grid-cols-3 gap-2">
              {['x64', 'x86', 'arm64'].map(arch => (
                <button
                  key={arch}
                  onClick={() => handleChange('architecture', arch)}
                  className={clsx(
                    'px-3 py-2 rounded text-xs font-medium transition-colors',
                    config.architecture === arch
                      ? 'bg-ferox-green/20 text-ferox-green border border-ferox-green/50'
                      : 'bg-dark-700 text-text-secondary border border-dark-600 hover:border-dark-500'
                  )}
                >
                  {arch}
                </button>
              ))}
            </div>
          </div>

          {/* Connection Settings */}
          <div className="grid grid-cols-2 gap-2">
            <div>
              <label className="block text-xs text-text-secondary mb-1">LHOST</label>
              <input
                type="text"
                value={config.lhost}
                onChange={e => handleChange('lhost', e.target.value)}
                placeholder="192.168.1.100"
                className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary placeholder:text-text-muted focus:border-ferox-green/50 focus:outline-none"
              />
            </div>
            <div>
              <label className="block text-xs text-text-secondary mb-1">LPORT</label>
              <input
                type="number"
                value={config.lport}
                onChange={e => handleChange('lport', parseInt(e.target.value) || 4444)}
                placeholder="4444"
                className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary placeholder:text-text-muted focus:border-ferox-green/50 focus:outline-none"
              />
            </div>
          </div>

          {/* Options */}
          <div className="space-y-2">
            <label className="block text-xs text-text-secondary mb-1">Options</label>

            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={config.obfuscation}
                onChange={e => handleChange('obfuscation', e.target.checked)}
                className="w-4 h-4 rounded bg-dark-700 border-dark-600 text-ferox-green focus:ring-ferox-green/50"
              />
              <Lock size={14} className="text-text-muted" />
              <span className="text-sm text-text-primary">Obfuscation</span>
            </label>

            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={config.signing}
                onChange={e => handleChange('signing', e.target.checked)}
                className="w-4 h-4 rounded bg-dark-700 border-dark-600 text-ferox-green focus:ring-ferox-green/50"
              />
              <Shield size={14} className="text-text-muted" />
              <span className="text-sm text-text-primary">Code Signing</span>
            </label>

            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={config.staged}
                onChange={e => handleChange('staged', e.target.checked)}
                className="w-4 h-4 rounded bg-dark-700 border-dark-600 text-ferox-green focus:ring-ferox-green/50"
              />
              <FileCode size={14} className="text-text-muted" />
              <span className="text-sm text-text-primary">Staged Delivery</span>
            </label>
          </div>

          {/* Custom Name */}
          <div>
            <label className="block text-xs text-text-secondary mb-1">Custom Name (optional)</label>
            <input
              type="text"
              value={config.name || ''}
              onChange={e => handleChange('name', e.target.value || undefined)}
              placeholder="my_payload"
              className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary placeholder:text-text-muted focus:border-ferox-green/50 focus:outline-none"
            />
          </div>

          {/* Generate Button */}
          <button
            onClick={handleGenerate}
            disabled={isGenerating}
            className={clsx(
              'w-full py-3 rounded font-medium text-sm flex items-center justify-center gap-2 transition-colors',
              isGenerating
                ? 'bg-dark-600 text-text-muted cursor-not-allowed'
                : 'bg-ferox-green text-dark-900 hover:bg-ferox-green/90'
            )}
          >
            {isGenerating ? (
              <>
                <Loader2 size={16} className="animate-spin" />
                Generating...
              </>
            ) : (
              <>
                <Package size={16} />
                Generate Simulated Payload
              </>
            )}
          </button>
        </div>

        {/* Results Panel */}
        <div className="flex-1 overflow-y-auto p-4">
          {!result ? (
            <div className="h-full flex items-center justify-center text-text-muted">
              <div className="text-center">
                <Package size={48} className="mx-auto mb-4 opacity-20" />
                <p>Configure and generate a simulated payload</p>
                <p className="text-xs mt-2">Results will appear here</p>
              </div>
            </div>
          ) : (
            <div className="space-y-4">
              {/* Payload Summary */}
              <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
                <div className="flex items-start justify-between">
                  <div>
                    <h3 className="text-lg font-semibold text-text-primary flex items-center gap-2">
                      <CheckCircle size={18} className="text-green-400" />
                      {result.name}
                    </h3>
                    <p className="text-xs text-text-muted mt-1">
                      ID: {result.id}
                    </p>
                  </div>
                  <div className={clsx(
                    'px-3 py-1 rounded text-xs font-medium',
                    getRiskColor(result.risk_analysis.risk_level),
                    'bg-dark-700'
                  )}>
                    Risk: {result.risk_analysis.risk_level.toUpperCase()}
                  </div>
                </div>

                <div className="grid grid-cols-3 gap-4 mt-4">
                  <div className="bg-dark-700 rounded p-3">
                    <div className="text-xs text-text-muted">Simulated Path</div>
                    <div className="text-sm text-text-primary font-mono truncate">
                      {result.simulated_path}
                    </div>
                  </div>
                  <div className="bg-dark-700 rounded p-3">
                    <div className="text-xs text-text-muted">Simulated Size</div>
                    <div className="text-sm text-text-primary">
                      {(result.simulated_size_bytes / 1024).toFixed(1)} KB
                    </div>
                  </div>
                  <div className="bg-dark-700 rounded p-3">
                    <div className="text-xs text-text-muted">SHA256 (simulated)</div>
                    <div className="text-sm text-text-primary font-mono truncate">
                      {result.simulated_hash.substring(0, 16)}...
                    </div>
                  </div>
                </div>
              </div>

              {/* Build Log */}
              <CollapsibleSection
                title="Build Log"
                icon={<Terminal size={16} />}
                expanded={expandedSections.buildLog}
                onToggle={() => toggleSection('buildLog')}
              >
                <div className="bg-dark-900 rounded p-3 font-mono text-xs space-y-1">
                  {result.build_log.map((entry, i) => (
                    <div key={i} className="flex items-start gap-2">
                      {getLogIcon(entry.level)}
                      <span className="text-text-muted">
                        {new Date(entry.timestamp).toLocaleTimeString()}
                      </span>
                      <span className="text-text-primary">{entry.message}</span>
                    </div>
                  ))}
                </div>
              </CollapsibleSection>

              {/* Risk Analysis */}
              <CollapsibleSection
                title="Risk Analysis"
                icon={<AlertTriangle size={16} />}
                expanded={expandedSections.riskAnalysis}
                onToggle={() => toggleSection('riskAnalysis')}
                badge={
                  <span className={clsx('text-xs', getRiskColor(result.risk_analysis.risk_level))}>
                    Score: {result.risk_analysis.risk_score}/100
                  </span>
                }
              >
                <div className="space-y-3">
                  {result.risk_analysis.factors.map((factor, i) => (
                    <div key={i} className="bg-dark-900 rounded p-3">
                      <div className="flex items-center justify-between">
                        <span className="text-sm text-text-primary">{factor.name}</span>
                        <span className={clsx(
                          'text-xs',
                          factor.score <= 30 ? 'text-green-400' :
                          factor.score <= 60 ? 'text-yellow-400' : 'text-red-400'
                        )}>
                          {factor.score}/100
                        </span>
                      </div>
                      <p className="text-xs text-text-muted mt-1">{factor.description}</p>
                    </div>
                  ))}

                  <div className="mt-4">
                    <h4 className="text-xs text-text-secondary mb-2">Recommendations</h4>
                    <ul className="space-y-1">
                      {result.risk_analysis.recommendations.map((rec, i) => (
                        <li key={i} className="text-xs text-text-muted flex items-start gap-2">
                          <span className="text-ferox-green">•</span>
                          {rec}
                        </li>
                      ))}
                    </ul>
                  </div>
                </div>
              </CollapsibleSection>

              {/* Detection Analysis */}
              <CollapsibleSection
                title="Detection Analysis"
                icon={<Eye size={16} />}
                expanded={expandedSections.detection}
                onToggle={() => toggleSection('detection')}
                badge={
                  <span className="text-xs text-yellow-400">
                    Est. Detection: {(result.detection_analysis.estimated_detection_rate * 100).toFixed(0)}%
                  </span>
                }
              >
                <div className="space-y-3">
                  <div>
                    <h4 className="text-xs text-text-secondary mb-2">Likely Detectors</h4>
                    <div className="flex flex-wrap gap-2">
                      {result.detection_analysis.likely_detectors.map((d, i) => (
                        <span key={i} className="px-2 py-1 bg-red-500/20 text-red-400 rounded text-xs">
                          {d}
                        </span>
                      ))}
                    </div>
                  </div>

                  <div>
                    <h4 className="text-xs text-text-secondary mb-2">Behavioral Indicators</h4>
                    <ul className="space-y-1">
                      {result.detection_analysis.behavioral_indicators.map((ind, i) => (
                        <li key={i} className="text-xs text-text-muted">• {ind}</li>
                      ))}
                    </ul>
                  </div>

                  <div>
                    <h4 className="text-xs text-text-secondary mb-2">Evasion Notes</h4>
                    <ul className="space-y-1">
                      {result.detection_analysis.evasion_notes.map((note, i) => (
                        <li key={i} className="text-xs text-text-muted">• {note}</li>
                      ))}
                    </ul>
                  </div>
                </div>
              </CollapsibleSection>

              {/* MITRE Mapping */}
              <CollapsibleSection
                title="MITRE ATT&CK Mapping"
                icon={<Target size={16} />}
                expanded={expandedSections.mitre}
                onToggle={() => toggleSection('mitre')}
              >
                <div className="space-y-2">
                  {result.mitre_mapping.map((mapping, i) => (
                    <div key={i} className="bg-dark-900 rounded p-3">
                      <div className="flex items-center gap-2">
                        <span className="px-2 py-0.5 bg-purple-500/20 text-purple-400 rounded text-xs font-mono">
                          {mapping.technique_id}
                        </span>
                        <span className="text-sm text-text-primary">{mapping.technique_name}</span>
                      </div>
                      <div className="flex items-center gap-2 mt-1">
                        <span className="text-xs text-text-muted">Tactic:</span>
                        <span className="text-xs text-ferox-green">{mapping.tactic}</span>
                      </div>
                      <p className="text-xs text-text-muted mt-1">{mapping.description}</p>
                    </div>
                  ))}
                </div>
              </CollapsibleSection>

              {/* Execution Hints */}
              <CollapsibleSection
                title="Execution Commands (Educational)"
                icon={<Terminal size={16} />}
                expanded={expandedSections.execution}
                onToggle={() => toggleSection('execution')}
              >
                <div className="space-y-2">
                  {result.execution_hints.map((hint, i) => (
                    <div key={i} className="bg-dark-900 rounded p-3">
                      <div className="flex items-center justify-between mb-2">
                        <span className="text-sm text-text-primary">{hint.name}</span>
                        <span className="text-xs px-2 py-0.5 bg-dark-700 text-text-muted rounded">
                          {hint.os}
                        </span>
                      </div>
                      <div className="flex items-center gap-2">
                        <code className="flex-1 px-3 py-2 bg-dark-800 rounded text-xs font-mono text-ferox-green overflow-x-auto">
                          {hint.command}
                        </code>
                        <button
                          onClick={() => copyToClipboard(hint.command, hint.name)}
                          className="p-2 hover:bg-dark-700 rounded transition-colors"
                          title="Copy to clipboard"
                        >
                          <Copy size={14} className="text-text-muted" />
                        </button>
                      </div>
                      <p className="text-xs text-text-muted mt-2">{hint.description}</p>
                    </div>
                  ))}
                </div>
              </CollapsibleSection>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

// Collapsible Section Component
interface CollapsibleSectionProps {
  title: string;
  icon: React.ReactNode;
  expanded: boolean;
  onToggle: () => void;
  badge?: React.ReactNode;
  children: React.ReactNode;
}

function CollapsibleSection({
  title,
  icon,
  expanded,
  onToggle,
  badge,
  children
}: CollapsibleSectionProps) {
  return (
    <div className="bg-dark-800 rounded-lg border border-dark-600 overflow-hidden">
      <button
        onClick={onToggle}
        className="w-full px-4 py-3 flex items-center justify-between hover:bg-dark-700/50 transition-colors"
      >
        <div className="flex items-center gap-2">
          <span className="text-text-muted">{icon}</span>
          <span className="text-sm font-medium text-text-primary">{title}</span>
          {badge}
        </div>
        {expanded ? (
          <ChevronUp size={16} className="text-text-muted" />
        ) : (
          <ChevronDown size={16} className="text-text-muted" />
        )}
      </button>
      {expanded && (
        <div className="px-4 pb-4">
          {children}
        </div>
      )}
    </div>
  );
}

export default PayloadBuilder;
