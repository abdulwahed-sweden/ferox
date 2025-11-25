/**
 * ThreatList - Displays detected threats with severity indicators
 * Real-time threat monitoring display
 */

import { motion, AnimatePresence } from 'framer-motion';
import { AlertTriangle, AlertCircle, Info, ShieldAlert, Clock, ChevronRight } from 'lucide-react';
import { clsx } from 'clsx';

export interface Threat {
  id: string;
  category: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  title: string;
  description: string;
  mitigation: string;
  timestamp: string;
  source: string;
  indicators: string[];
}

interface ThreatListProps {
  threats: Threat[];
  onThreatClick?: (threat: Threat) => void;
  className?: string;
}

const severityConfig = {
  low: {
    icon: Info,
    bg: 'bg-blue-500/10',
    border: 'border-blue-500/30',
    text: 'text-blue-400',
    badge: 'bg-blue-500/20',
  },
  medium: {
    icon: AlertCircle,
    bg: 'bg-yellow-500/10',
    border: 'border-yellow-500/30',
    text: 'text-yellow-400',
    badge: 'bg-yellow-500/20',
  },
  high: {
    icon: AlertTriangle,
    bg: 'bg-orange-500/10',
    border: 'border-orange-500/30',
    text: 'text-orange-400',
    badge: 'bg-orange-500/20',
  },
  critical: {
    icon: ShieldAlert,
    bg: 'bg-red-500/10',
    border: 'border-red-500/30',
    text: 'text-red-400',
    badge: 'bg-red-500/20',
  },
};

export function ThreatList({ threats, onThreatClick, className }: ThreatListProps) {
  if (threats.length === 0) {
    return (
      <div className={clsx('p-6 text-center', className)}>
        <ShieldAlert className="mx-auto text-green-400 mb-2" size={32} />
        <p className="text-text-secondary text-sm">No active threats detected</p>
        <p className="text-text-muted text-xs mt-1">OPSEC monitoring active</p>
      </div>
    );
  }

  return (
    <div className={clsx('space-y-2', className)}>
      <AnimatePresence>
        {threats.map((threat, index) => {
          const config = severityConfig[threat.severity];
          const Icon = config.icon;

          return (
            <motion.div
              key={threat.id}
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: 20 }}
              transition={{ delay: index * 0.1 }}
              className={clsx(
                'p-3 rounded-lg border cursor-pointer transition-all',
                config.bg,
                config.border,
                'hover:brightness-110'
              )}
              onClick={() => onThreatClick?.(threat)}
            >
              <div className="flex items-start gap-3">
                <div className={clsx('p-2 rounded', config.badge)}>
                  <Icon className={config.text} size={18} />
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center justify-between gap-2">
                    <h4 className={clsx('font-medium text-sm', config.text)}>{threat.title}</h4>
                    <span className={clsx('text-xs px-2 py-0.5 rounded uppercase font-medium', config.badge, config.text)}>
                      {threat.severity}
                    </span>
                  </div>
                  <p className="text-xs text-text-muted mt-1 line-clamp-2">{threat.description}</p>
                  <div className="flex items-center gap-3 mt-2">
                    <span className="text-xs text-text-muted flex items-center gap-1">
                      <Clock size={10} />
                      {new Date(threat.timestamp).toLocaleTimeString()}
                    </span>
                    <span className="text-xs text-text-muted bg-dark-600 px-1.5 py-0.5 rounded">
                      {threat.category}
                    </span>
                  </div>
                </div>
                <ChevronRight className="text-text-muted" size={16} />
              </div>
            </motion.div>
          );
        })}
      </AnimatePresence>
    </div>
  );
}

export default ThreatList;
