/**
 * TrafficMonitor - Network traffic analysis display
 * Shows bandwidth usage and suspicious pattern detection
 */

import { motion } from 'framer-motion';
import { Activity, ArrowUp, ArrowDown, Radio, AlertTriangle, Shield } from 'lucide-react';
import { clsx } from 'clsx';

export interface TrafficAnalysis {
  total_bytes_sent: number;
  total_bytes_received: number;
  connections_active: number;
  suspicious_patterns: string[];
  beacon_detected: boolean;
  exfil_risk: 'low' | 'medium' | 'high' | 'critical';
}

interface TrafficMonitorProps {
  traffic: TrafficAnalysis;
  className?: string;
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

const riskColors = {
  low: 'text-green-400',
  medium: 'text-yellow-400',
  high: 'text-orange-400',
  critical: 'text-red-400',
};

const riskBg = {
  low: 'bg-green-500/10 border-green-500/30',
  medium: 'bg-yellow-500/10 border-yellow-500/30',
  high: 'bg-orange-500/10 border-orange-500/30',
  critical: 'bg-red-500/10 border-red-500/30',
};

export function TrafficMonitor({ traffic, className }: TrafficMonitorProps) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className={clsx('space-y-4', className)}
    >
      {/* Traffic Stats */}
      <div className="grid grid-cols-3 gap-3">
        <div className="bg-dark-700 rounded-lg p-4 border border-dark-600">
          <div className="flex items-center gap-2 mb-2">
            <ArrowUp size={14} className="text-cyan-400" />
            <span className="text-xs text-text-muted">Sent</span>
          </div>
          <span className="text-lg font-bold text-cyan-400">
            {formatBytes(traffic.total_bytes_sent)}
          </span>
        </div>

        <div className="bg-dark-700 rounded-lg p-4 border border-dark-600">
          <div className="flex items-center gap-2 mb-2">
            <ArrowDown size={14} className="text-purple-400" />
            <span className="text-xs text-text-muted">Received</span>
          </div>
          <span className="text-lg font-bold text-purple-400">
            {formatBytes(traffic.total_bytes_received)}
          </span>
        </div>

        <div className="bg-dark-700 rounded-lg p-4 border border-dark-600">
          <div className="flex items-center gap-2 mb-2">
            <Activity size={14} className="text-green-400" />
            <span className="text-xs text-text-muted">Active</span>
          </div>
          <span className="text-lg font-bold text-green-400">
            {traffic.connections_active}
          </span>
        </div>
      </div>

      {/* Risk Assessment */}
      <div className={clsx('p-4 rounded-lg border', riskBg[traffic.exfil_risk])}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Shield size={18} className={riskColors[traffic.exfil_risk]} />
            <span className="text-sm text-text-primary">Exfiltration Risk</span>
          </div>
          <span className={clsx('text-sm font-bold uppercase', riskColors[traffic.exfil_risk])}>
            {traffic.exfil_risk}
          </span>
        </div>
      </div>

      {/* Beacon Detection */}
      {traffic.beacon_detected && (
        <motion.div
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          className="p-4 bg-red-500/10 border border-red-500/30 rounded-lg"
        >
          <div className="flex items-center gap-3">
            <motion.div
              animate={{ scale: [1, 1.2, 1] }}
              transition={{ repeat: Infinity, duration: 2 }}
            >
              <Radio className="text-red-400" size={20} />
            </motion.div>
            <div>
              <span className="text-sm font-medium text-red-400">Beacon Pattern Detected</span>
              <p className="text-xs text-text-muted mt-0.5">
                Regular interval traffic detected - may indicate C2 communication
              </p>
            </div>
          </div>
        </motion.div>
      )}

      {/* Suspicious Patterns */}
      {traffic.suspicious_patterns.length > 0 && (
        <div className="p-4 bg-yellow-500/10 border border-yellow-500/30 rounded-lg">
          <div className="flex items-center gap-2 mb-2">
            <AlertTriangle size={16} className="text-yellow-400" />
            <span className="text-sm font-medium text-yellow-400">Suspicious Patterns</span>
          </div>
          <div className="space-y-1">
            {traffic.suspicious_patterns.map((pattern, i) => (
              <div key={i} className="text-xs text-text-secondary bg-dark-700 px-2 py-1 rounded">
                {pattern}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* All Clear */}
      {!traffic.beacon_detected && traffic.suspicious_patterns.length === 0 && traffic.exfil_risk === 'low' && (
        <div className="p-4 bg-green-500/10 border border-green-500/30 rounded-lg flex items-center gap-3">
          <Shield className="text-green-400" size={20} />
          <div>
            <span className="text-sm font-medium text-green-400">Traffic Analysis Clear</span>
            <p className="text-xs text-text-muted">No suspicious network patterns detected</p>
          </div>
        </div>
      )}
    </motion.div>
  );
}

export default TrafficMonitor;
