/**
 * CountermeasureCard - Individual countermeasure toggle card
 * Displays countermeasure details with enable/disable toggle
 */

import { motion } from 'framer-motion';
import { Shield, ShieldCheck, ShieldOff, Zap, Activity } from 'lucide-react';
import { clsx } from 'clsx';

export interface Countermeasure {
  id: string;
  name: string;
  description: string;
  category: string;
  enabled: boolean;
  risk_reduction: number;
  performance_impact: 'none' | 'low' | 'medium' | 'high';
}

interface CountermeasureCardProps {
  countermeasure: Countermeasure;
  onToggle: (id: string, enabled: boolean) => void;
  isLoading?: boolean;
  className?: string;
}

const categoryIcons: Record<string, React.ElementType> = {
  network: Activity,
  evasion: Shield,
  forensic: ShieldOff,
};

const impactColors: Record<string, string> = {
  none: 'text-green-400',
  low: 'text-cyan-400',
  medium: 'text-yellow-400',
  high: 'text-orange-400',
};

export function CountermeasureCard({ countermeasure, onToggle, isLoading, className }: CountermeasureCardProps) {
  const Icon = categoryIcons[countermeasure.category] || Shield;

  return (
    <motion.div
      layout
      className={clsx(
        'p-4 rounded-lg border transition-all',
        countermeasure.enabled
          ? 'bg-cyan-500/10 border-cyan-500/30'
          : 'bg-dark-700 border-dark-600',
        className
      )}
    >
      <div className="flex items-start gap-3">
        <div
          className={clsx(
            'p-2 rounded',
            countermeasure.enabled ? 'bg-cyan-500/20' : 'bg-dark-600'
          )}
        >
          {countermeasure.enabled ? (
            <ShieldCheck className="text-cyan-400" size={20} />
          ) : (
            <Icon className="text-text-muted" size={20} />
          )}
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex items-center justify-between gap-2">
            <h4 className={clsx(
              'font-medium text-sm',
              countermeasure.enabled ? 'text-cyan-400' : 'text-text-primary'
            )}>
              {countermeasure.name}
            </h4>
            <button
              onClick={() => onToggle(countermeasure.id, !countermeasure.enabled)}
              disabled={isLoading}
              className={clsx(
                'relative w-10 h-5 rounded-full transition-colors',
                countermeasure.enabled ? 'bg-cyan-500' : 'bg-dark-500',
                isLoading && 'opacity-50 cursor-wait'
              )}
            >
              <motion.div
                className="absolute top-0.5 w-4 h-4 bg-white rounded-full shadow"
                animate={{ left: countermeasure.enabled ? '22px' : '2px' }}
                transition={{ type: 'spring', stiffness: 500, damping: 30 }}
              />
            </button>
          </div>

          <p className="text-xs text-text-muted mt-1">{countermeasure.description}</p>

          <div className="flex items-center gap-4 mt-3">
            <div className="flex items-center gap-1">
              <Zap size={12} className="text-green-400" />
              <span className="text-xs text-text-secondary">
                -{countermeasure.risk_reduction}% risk
              </span>
            </div>
            <div className="flex items-center gap-1">
              <Activity size={12} className={impactColors[countermeasure.performance_impact]} />
              <span className={clsx('text-xs', impactColors[countermeasure.performance_impact])}>
                {countermeasure.performance_impact} impact
              </span>
            </div>
            <span className="text-xs bg-dark-600 text-text-muted px-1.5 py-0.5 rounded">
              {countermeasure.category}
            </span>
          </div>
        </div>
      </div>
    </motion.div>
  );
}

export default CountermeasureCard;
