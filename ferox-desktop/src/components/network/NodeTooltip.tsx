/**
 * NodeTooltip - Detailed node information panel
 */

import { motion } from 'framer-motion';
import { X, Terminal, Globe, HardDrive, Cpu, Network } from 'lucide-react';
import type { NetworkNode } from '../../data/mockNetwork';
import { typeColors, statusColors } from '../../data/mockNetwork';
import { tooltipVariants } from './animations';
import toast from 'react-hot-toast';

interface NodeTooltipProps {
  node: NetworkNode;
  onClose: () => void;
}

export function NodeTooltip({ node, onClose }: NodeTooltipProps) {
  const color = typeColors[node.type];
  const statusColor = statusColors[node.status];

  const handleAction = (action: string) => {
    toast.loading(`${action} ${node.label}...`, { id: action });
    setTimeout(() => {
      toast.success(`${action} completed`, { id: action });
    }, 2000);
  };

  return (
    <motion.div
      className="absolute top-4 right-4 w-64 bg-dark-800/95 backdrop-blur-sm rounded-lg border overflow-hidden"
      style={{ borderColor: `${color}40` }}
      variants={tooltipVariants}
      initial="hidden"
      animate="visible"
      exit="exit"
    >
      {/* Header */}
      <div
        className="p-3 flex items-center justify-between"
        style={{ backgroundColor: `${color}15` }}
      >
        <div className="flex items-center gap-2">
          <div
            className="w-3 h-3 rounded-full"
            style={{
              backgroundColor: statusColor,
              boxShadow: `0 0 8px ${statusColor}`
            }}
          />
          <span className="font-semibold text-sm" style={{ color }}>
            {node.label}
          </span>
        </div>
        <button
          onClick={onClose}
          className="p-1 hover:bg-dark-600 rounded transition-colors"
        >
          <X size={14} className="text-text-muted" />
        </button>
      </div>

      {/* Content */}
      <div className="p-3 space-y-3">
        <InfoRow icon={Globe} label="IP Address" value={node.ip} />
        <InfoRow icon={Cpu} label="Type" value={node.type.charAt(0).toUpperCase() + node.type.slice(1)} />
        <InfoRow
          icon={Network}
          label="Status"
          value={node.status.charAt(0).toUpperCase() + node.status.slice(1)}
          valueColor={statusColor}
        />
        {node.os && (
          <InfoRow icon={HardDrive} label="OS" value={node.os.charAt(0).toUpperCase() + node.os.slice(1)} />
        )}

        {/* Open Ports */}
        {node.ports && node.ports.length > 0 && (
          <div className="pt-2 border-t border-dark-600">
            <div className="text-[10px] text-text-muted mb-1.5">Open Ports</div>
            <div className="flex flex-wrap gap-1">
              {node.ports.map(port => (
                <span
                  key={port}
                  className="px-1.5 py-0.5 rounded text-[10px] font-mono"
                  style={{
                    backgroundColor: `${color}20`,
                    color: color
                  }}
                >
                  {port}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Action Buttons */}
        <div className="pt-2 border-t border-dark-600 flex gap-2">
          <ActionButton
            label="Scan"
            color="#00d4ff"
            onClick={() => handleAction('Scanning')}
          />
          <ActionButton
            label="Exploit"
            color="#ff3366"
            onClick={() => handleAction('Exploiting')}
          />
          <ActionButton
            label="Shell"
            color="#00ff88"
            icon={Terminal}
            onClick={() => handleAction('Opening shell to')}
          />
        </div>
      </div>
    </motion.div>
  );
}

function InfoRow({
  icon: Icon,
  label,
  value,
  valueColor
}: {
  icon: typeof Globe;
  label: string;
  value: string;
  valueColor?: string;
}) {
  return (
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-1.5 text-text-muted">
        <Icon size={12} />
        <span className="text-[10px]">{label}</span>
      </div>
      <span
        className="text-xs font-medium"
        style={{ color: valueColor || '#e5e7eb' }}
      >
        {value}
      </span>
    </div>
  );
}

function ActionButton({
  label,
  color,
  icon: Icon,
  onClick
}: {
  label: string;
  color: string;
  icon?: typeof Terminal;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className="flex-1 py-1.5 rounded text-[10px] font-medium flex items-center justify-center gap-1 transition-all hover:scale-105"
      style={{
        backgroundColor: `${color}20`,
        color: color,
        border: `1px solid ${color}40`
      }}
    >
      {Icon && <Icon size={10} />}
      {label}
    </button>
  );
}

export default NodeTooltip;
