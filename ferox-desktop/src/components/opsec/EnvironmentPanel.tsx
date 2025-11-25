/**
 * EnvironmentPanel - Environment analysis display
 * Shows VM detection, AV/EDR status, and system info
 */

import { motion } from 'framer-motion';
import {
  Monitor, Server, Shield, AlertTriangle, Eye, Bug,
  Cpu, HardDrive, Network, CheckCircle, XCircle
} from 'lucide-react';
import { clsx } from 'clsx';

export interface EnvironmentAnalysis {
  hostname: string;
  os: string;
  os_version: string;
  is_vm: boolean;
  vm_type: string | null;
  is_sandbox: boolean;
  sandbox_indicators: string[];
  av_detected: string[];
  edr_detected: string[];
  monitoring_tools: string[];
  network_monitoring: boolean;
  debug_mode: boolean;
}

interface EnvironmentPanelProps {
  environment: EnvironmentAnalysis;
  className?: string;
}

interface DetectionItemProps {
  label: string;
  detected: boolean;
  details?: string[];
  icon: React.ElementType;
  dangerColor?: boolean;
}

function DetectionItem({ label, detected, details, icon: Icon, dangerColor }: DetectionItemProps) {
  return (
    <div className={clsx(
      'p-3 rounded-lg border',
      detected
        ? dangerColor
          ? 'bg-red-500/10 border-red-500/30'
          : 'bg-yellow-500/10 border-yellow-500/30'
        : 'bg-green-500/10 border-green-500/30'
    )}>
      <div className="flex items-center gap-2">
        <Icon
          size={16}
          className={detected
            ? dangerColor ? 'text-red-400' : 'text-yellow-400'
            : 'text-green-400'
          }
        />
        <span className="text-xs text-text-secondary">{label}</span>
        {detected ? (
          <XCircle size={14} className={dangerColor ? 'text-red-400' : 'text-yellow-400'} />
        ) : (
          <CheckCircle size={14} className="text-green-400" />
        )}
      </div>
      {detected && details && details.length > 0 && (
        <div className="mt-2 space-y-1">
          {details.map((detail, i) => (
            <span
              key={i}
              className={clsx(
                'text-xs px-2 py-0.5 rounded inline-block mr-1',
                dangerColor ? 'bg-red-500/20 text-red-400' : 'bg-yellow-500/20 text-yellow-400'
              )}
            >
              {detail}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

export function EnvironmentPanel({ environment, className }: EnvironmentPanelProps) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className={clsx('space-y-4', className)}
    >
      {/* System Info */}
      <div className="bg-dark-700 rounded-lg p-4 border border-dark-600">
        <h4 className="text-sm font-medium text-text-primary flex items-center gap-2 mb-3">
          <Monitor size={16} className="text-cyan-400" />
          System Information
        </h4>
        <div className="grid grid-cols-2 gap-3">
          <div className="flex items-center gap-2">
            <Server size={14} className="text-text-muted" />
            <span className="text-xs text-text-muted">Hostname:</span>
            <span className="text-xs text-text-primary font-mono">{environment.hostname}</span>
          </div>
          <div className="flex items-center gap-2">
            <Cpu size={14} className="text-text-muted" />
            <span className="text-xs text-text-muted">OS:</span>
            <span className="text-xs text-text-primary">{environment.os} {environment.os_version}</span>
          </div>
        </div>
      </div>

      {/* Detection Status Grid */}
      <div className="grid grid-cols-2 gap-3">
        <DetectionItem
          label="Virtual Machine"
          detected={environment.is_vm}
          details={environment.vm_type ? [environment.vm_type] : undefined}
          icon={HardDrive}
        />
        <DetectionItem
          label="Sandbox"
          detected={environment.is_sandbox}
          details={environment.sandbox_indicators}
          icon={Bug}
          dangerColor
        />
        <DetectionItem
          label="Antivirus"
          detected={environment.av_detected.length > 0}
          details={environment.av_detected}
          icon={Shield}
          dangerColor
        />
        <DetectionItem
          label="EDR Solution"
          detected={environment.edr_detected.length > 0}
          details={environment.edr_detected}
          icon={Eye}
          dangerColor
        />
        <DetectionItem
          label="Monitoring Tools"
          detected={environment.monitoring_tools.length > 0}
          details={environment.monitoring_tools}
          icon={AlertTriangle}
        />
        <DetectionItem
          label="Network Monitoring"
          detected={environment.network_monitoring}
          icon={Network}
          dangerColor
        />
      </div>

      {/* Debug Mode Warning */}
      {environment.debug_mode && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="p-3 bg-orange-500/10 border border-orange-500/30 rounded-lg flex items-center gap-2"
        >
          <Bug size={16} className="text-orange-400" />
          <span className="text-xs text-orange-400">Debug mode enabled - additional telemetry may be present</span>
        </motion.div>
      )}
    </motion.div>
  );
}

export default EnvironmentPanel;
