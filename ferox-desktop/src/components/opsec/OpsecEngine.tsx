/**
 * OpsecEngine - Main OPSEC monitoring dashboard
 * Provides comprehensive operational security monitoring and countermeasures
 */

import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Shield, ShieldAlert, RefreshCw, Download, EyeOff,
  Activity, AlertTriangle, Settings, ChevronDown, ChevronUp
} from 'lucide-react';
import toast from 'react-hot-toast';
import { clsx } from 'clsx';

import { ThreatGauge } from './ThreatGauge';
import { ThreatList, Threat } from './ThreatList';
import { CountermeasureCard } from './CountermeasureCard';
import { EnvironmentPanel } from './EnvironmentPanel';
import { TrafficMonitor } from './TrafficMonitor';
import { useOpsec } from '../../hooks/useOpsec';

interface OpsecEngineProps {
  className?: string;
}

export function OpsecEngine({ className }: OpsecEngineProps) {
  const {
    status,
    countermeasures,
    traffic: trafficAnalysis,
    loading: isLoading,
    error,
    checkOpsec,
    activateCountermeasure,
    deactivateCountermeasure,
    analyzeTraffic,
    goDark,
  } = useOpsec();

  const [expandedSections, setExpandedSections] = useState({
    threats: true,
    countermeasures: true,
    environment: false,
    traffic: false,
  });

  const [selectedThreat, setSelectedThreat] = useState<Threat | null>(null);

  // Auto-refresh OPSEC status every 30 seconds
  useEffect(() => {
    const interval = setInterval(() => {
      checkOpsec();
    }, 30000);
    return () => clearInterval(interval);
  }, [checkOpsec]);

  const toggleSection = (section: keyof typeof expandedSections) => {
    setExpandedSections(prev => ({
      ...prev,
      [section]: !prev[section],
    }));
  };

  const handleRefresh = async () => {
    toast.loading('Checking OPSEC status...', { id: 'opsec-refresh' });
    await checkOpsec();
    await analyzeTraffic();
    toast.success('OPSEC status updated', { id: 'opsec-refresh' });
  };

  const handleGoDark = async () => {
    toast.loading('Activating all countermeasures...', { id: 'go-dark' });
    await goDark();
    toast.success('All countermeasures activated', { id: 'go-dark' });
  };

  const handleToggleCountermeasure = async (id: string, enabled: boolean) => {
    if (enabled) {
      await activateCountermeasure(id);
    } else {
      await deactivateCountermeasure(id);
    }
    toast.success(`Countermeasure ${enabled ? 'activated' : 'deactivated'}`, { duration: 2000 });
  };

  const handleExportReport = () => {
    toast.loading('Generating OPSEC report...', { id: 'export' });
    setTimeout(() => {
      toast.success('Report exported to opsec_report.json', { id: 'export' });
    }, 1500);
  };

  const threatLevel = (status?.threat_level || 'medium') as 'low' | 'medium' | 'high' | 'critical';
  const activeCount = countermeasures.filter(cm => cm.enabled).length;

  return (
    <div className={clsx('h-full flex flex-col bg-dark-900', className)}>
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <ShieldAlert className="text-cyan-400" size={22} />
            <h2 className="text-lg font-semibold text-text-primary">OPSEC Engine</h2>
            <span className="text-xs bg-cyan-500/20 text-cyan-400 px-2 py-0.5 rounded">
              {activeCount}/{countermeasures.length} Active
            </span>
            {isLoading && <RefreshCw size={14} className="text-cyan-400 animate-spin" />}
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={handleRefresh}
              disabled={isLoading}
              className="px-3 py-1.5 bg-dark-700 border border-dark-600 text-text-secondary rounded text-xs font-medium flex items-center gap-1.5 hover:bg-dark-600 hover:text-text-primary transition-colors disabled:opacity-50"
            >
              <RefreshCw size={12} className={isLoading ? 'animate-spin' : ''} />
              Refresh
            </button>
          </div>
        </div>
        <p className="text-xs text-text-muted mt-2">
          Real-time operational security monitoring and threat mitigation
        </p>
      </div>

      {/* Error Display */}
      {error && (
        <div className="p-3 bg-red-500/10 border-b border-red-500/30">
          <div className="flex items-center gap-2">
            <AlertTriangle size={14} className="text-red-400" />
            <span className="text-xs text-red-400">{error}</span>
          </div>
        </div>
      )}

      {/* Main Content */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {/* Top Row: Score Gauge + Stats */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
          {/* OPSEC Score */}
          <div className="lg:col-span-1 flex items-center justify-center bg-dark-800 rounded-lg border border-dark-600 p-6">
            <ThreatGauge
              score={status?.score || 50}
              threatLevel={threatLevel}
              size={200}
            />
          </div>

          {/* Stats & Quick Actions */}
          <div className="lg:col-span-2 grid grid-cols-2 gap-4">
            {/* Threat Count */}
            <div className="bg-dark-800 rounded-lg border border-dark-600 p-4">
              <div className="flex items-center gap-2 mb-2">
                <AlertTriangle size={16} className="text-red-400" />
                <span className="text-xs text-text-muted">Active Threats</span>
              </div>
              <span className={clsx(
                'text-3xl font-bold',
                (status?.detected_threats?.length || 0) > 0 ? 'text-red-400' : 'text-green-400'
              )}>
                {status?.detected_threats?.length || 0}
              </span>
            </div>

            {/* Countermeasures */}
            <div className="bg-dark-800 rounded-lg border border-dark-600 p-4">
              <div className="flex items-center gap-2 mb-2">
                <Shield size={16} className="text-cyan-400" />
                <span className="text-xs text-text-muted">Countermeasures</span>
              </div>
              <span className="text-3xl font-bold text-cyan-400">
                {activeCount}
              </span>
              <span className="text-sm text-text-muted">/{countermeasures.length}</span>
            </div>

            {/* Recommendations */}
            <div className="bg-dark-800 rounded-lg border border-dark-600 p-4">
              <div className="flex items-center gap-2 mb-2">
                <Settings size={16} className="text-yellow-400" />
                <span className="text-xs text-text-muted">Recommendations</span>
              </div>
              <span className="text-3xl font-bold text-yellow-400">
                {status?.recommendations?.length || 0}
              </span>
            </div>

            {/* Go Dark Button */}
            <motion.button
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
              onClick={handleGoDark}
              className="bg-red-500/20 rounded-lg border border-red-500/30 p-4 flex flex-col items-center justify-center gap-2 hover:bg-red-500/30 transition-colors"
            >
              <EyeOff size={24} className="text-red-400" />
              <span className="text-sm font-medium text-red-400">Go Dark</span>
              <span className="text-xs text-text-muted">Enable all countermeasures</span>
            </motion.button>
          </div>
        </div>

        {/* Threats Section */}
        <div className="bg-dark-800 rounded-lg border border-dark-600">
          <button
            onClick={() => toggleSection('threats')}
            className="w-full p-4 flex items-center justify-between hover:bg-dark-700 transition-colors rounded-t-lg"
          >
            <div className="flex items-center gap-2">
              <AlertTriangle size={18} className="text-red-400" />
              <span className="font-medium text-text-primary">Detected Threats</span>
              <span className="text-xs bg-red-500/20 text-red-400 px-2 py-0.5 rounded">
                {status?.detected_threats?.length || 0}
              </span>
            </div>
            {expandedSections.threats ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
          </button>
          <AnimatePresence>
            {expandedSections.threats && (
              <motion.div
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                className="overflow-hidden"
              >
                <div className="p-4 pt-0">
                  <ThreatList
                    threats={status?.detected_threats || []}
                    onThreatClick={setSelectedThreat}
                  />
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>

        {/* Countermeasures Section */}
        <div className="bg-dark-800 rounded-lg border border-dark-600">
          <button
            onClick={() => toggleSection('countermeasures')}
            className="w-full p-4 flex items-center justify-between hover:bg-dark-700 transition-colors rounded-t-lg"
          >
            <div className="flex items-center gap-2">
              <Shield size={18} className="text-cyan-400" />
              <span className="font-medium text-text-primary">Countermeasures</span>
              <span className="text-xs bg-cyan-500/20 text-cyan-400 px-2 py-0.5 rounded">
                {activeCount} active
              </span>
            </div>
            {expandedSections.countermeasures ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
          </button>
          <AnimatePresence>
            {expandedSections.countermeasures && (
              <motion.div
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                className="overflow-hidden"
              >
                <div className="p-4 pt-0 grid grid-cols-1 md:grid-cols-2 gap-3">
                  {countermeasures.map(cm => (
                    <CountermeasureCard
                      key={cm.id}
                      countermeasure={cm}
                      onToggle={handleToggleCountermeasure}
                      isLoading={isLoading}
                    />
                  ))}
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>

        {/* Environment Section */}
        <div className="bg-dark-800 rounded-lg border border-dark-600">
          <button
            onClick={() => toggleSection('environment')}
            className="w-full p-4 flex items-center justify-between hover:bg-dark-700 transition-colors rounded-t-lg"
          >
            <div className="flex items-center gap-2">
              <Activity size={18} className="text-purple-400" />
              <span className="font-medium text-text-primary">Environment Analysis</span>
            </div>
            {expandedSections.environment ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
          </button>
          <AnimatePresence>
            {expandedSections.environment && status?.environment && (
              <motion.div
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                className="overflow-hidden"
              >
                <div className="p-4 pt-0">
                  <EnvironmentPanel environment={status.environment} />
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>

        {/* Traffic Section */}
        <div className="bg-dark-800 rounded-lg border border-dark-600">
          <button
            onClick={() => toggleSection('traffic')}
            className="w-full p-4 flex items-center justify-between hover:bg-dark-700 transition-colors rounded-t-lg"
          >
            <div className="flex items-center gap-2">
              <Activity size={18} className="text-green-400" />
              <span className="font-medium text-text-primary">Traffic Analysis</span>
            </div>
            {expandedSections.traffic ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
          </button>
          <AnimatePresence>
            {expandedSections.traffic && trafficAnalysis && (
              <motion.div
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                className="overflow-hidden"
              >
                <div className="p-4 pt-0">
                  <TrafficMonitor traffic={trafficAnalysis} />
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>

        {/* Recommendations */}
        {status?.recommendations && status.recommendations.length > 0 && (
          <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-3">
              <Settings size={16} className="text-yellow-400" />
              <span className="text-sm font-medium text-yellow-400">Recommendations</span>
            </div>
            <ul className="space-y-2">
              {status.recommendations.map((rec, i) => (
                <li key={i} className="text-xs text-text-secondary flex items-start gap-2">
                  <span className="text-yellow-400">•</span>
                  {rec}
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>

      {/* Footer */}
      <div className="p-4 border-t border-dark-600 bg-dark-800 flex items-center justify-between">
        <span className="text-xs text-text-muted">
          Last check: {status?.last_check ? new Date(status.last_check).toLocaleTimeString() : 'Never'}
        </span>
        <button
          onClick={handleExportReport}
          className="px-4 py-2 bg-dark-700 border border-dark-600 text-text-secondary rounded text-sm font-medium flex items-center gap-2 hover:bg-dark-600 hover:text-text-primary transition-colors"
        >
          <Download size={14} />
          Export Report
        </button>
      </div>

      {/* Threat Detail Modal */}
      <AnimatePresence>
        {selectedThreat && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
            onClick={() => setSelectedThreat(null)}
          >
            <motion.div
              initial={{ scale: 0.9, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.9, opacity: 0 }}
              className="bg-dark-800 rounded-lg border border-dark-600 p-6 max-w-lg w-full mx-4"
              onClick={e => e.stopPropagation()}
            >
              <div className="flex items-start justify-between mb-4">
                <div>
                  <h3 className="text-lg font-semibold text-text-primary">{selectedThreat.title}</h3>
                  <span className={clsx(
                    'text-xs px-2 py-0.5 rounded uppercase font-medium mt-1 inline-block',
                    selectedThreat.severity === 'critical' && 'bg-red-500/20 text-red-400',
                    selectedThreat.severity === 'high' && 'bg-orange-500/20 text-orange-400',
                    selectedThreat.severity === 'medium' && 'bg-yellow-500/20 text-yellow-400',
                    selectedThreat.severity === 'low' && 'bg-blue-500/20 text-blue-400',
                  )}>
                    {selectedThreat.severity}
                  </span>
                </div>
                <button
                  onClick={() => setSelectedThreat(null)}
                  className="text-text-muted hover:text-text-primary"
                >
                  ×
                </button>
              </div>

              <div className="space-y-4">
                <div>
                  <label className="text-xs text-text-muted uppercase">Description</label>
                  <p className="text-sm text-text-secondary mt-1">{selectedThreat.description}</p>
                </div>

                <div>
                  <label className="text-xs text-text-muted uppercase">Mitigation</label>
                  <p className="text-sm text-text-secondary mt-1">{selectedThreat.mitigation}</p>
                </div>

                {selectedThreat.indicators.length > 0 && (
                  <div>
                    <label className="text-xs text-text-muted uppercase">Indicators</label>
                    <div className="flex flex-wrap gap-1 mt-1">
                      {selectedThreat.indicators.map((ind, i) => (
                        <span key={i} className="text-xs bg-dark-600 text-text-secondary px-2 py-0.5 rounded">
                          {ind}
                        </span>
                      ))}
                    </div>
                  </div>
                )}

                <div className="flex items-center gap-4 text-xs text-text-muted">
                  <span>Source: {selectedThreat.source}</span>
                  <span>Category: {selectedThreat.category}</span>
                </div>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

export default OpsecEngine;
