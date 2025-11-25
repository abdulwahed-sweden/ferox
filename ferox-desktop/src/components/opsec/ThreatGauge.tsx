/**
 * ThreatGauge - Circular OPSEC score gauge with animated gradient
 * Displays the current operational security score (0-100)
 */

import { motion } from 'framer-motion';
import { clsx } from 'clsx';

interface ThreatGaugeProps {
  score: number;
  threatLevel: 'low' | 'medium' | 'high' | 'critical';
  size?: number;
  className?: string;
}

const threatColors = {
  low: { primary: '#22c55e', secondary: '#4ade80', glow: 'rgba(34, 197, 94, 0.3)' },
  medium: { primary: '#eab308', secondary: '#facc15', glow: 'rgba(234, 179, 8, 0.3)' },
  high: { primary: '#f97316', secondary: '#fb923c', glow: 'rgba(249, 115, 22, 0.3)' },
  critical: { primary: '#ef4444', secondary: '#f87171', glow: 'rgba(239, 68, 68, 0.4)' },
};

export function ThreatGauge({ score, threatLevel, size = 180, className }: ThreatGaugeProps) {
  const colors = threatColors[threatLevel];
  const radius = (size - 20) / 2;
  const circumference = 2 * Math.PI * radius;
  const strokeDashoffset = circumference - (score / 100) * circumference;
  const center = size / 2;

  return (
    <div className={clsx('relative', className)} style={{ width: size, height: size }}>
      <svg width={size} height={size} className="transform -rotate-90">
        {/* Background circle */}
        <circle
          cx={center}
          cy={center}
          r={radius}
          fill="none"
          stroke="rgba(255,255,255,0.1)"
          strokeWidth="10"
        />
        {/* Animated progress circle */}
        <motion.circle
          cx={center}
          cy={center}
          r={radius}
          fill="none"
          stroke={`url(#gradient-${threatLevel})`}
          strokeWidth="10"
          strokeLinecap="round"
          strokeDasharray={circumference}
          initial={{ strokeDashoffset: circumference }}
          animate={{ strokeDashoffset }}
          transition={{ duration: 1.5, ease: 'easeOut' }}
          style={{
            filter: `drop-shadow(0 0 10px ${colors.glow})`,
          }}
        />
        {/* Gradient definition */}
        <defs>
          <linearGradient id={`gradient-${threatLevel}`} x1="0%" y1="0%" x2="100%" y2="0%">
            <stop offset="0%" stopColor={colors.primary} />
            <stop offset="100%" stopColor={colors.secondary} />
          </linearGradient>
        </defs>
      </svg>

      {/* Center content */}
      <div className="absolute inset-0 flex flex-col items-center justify-center">
        <motion.span
          className="text-4xl font-bold"
          style={{ color: colors.primary }}
          initial={{ opacity: 0, scale: 0.5 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ delay: 0.5, duration: 0.5 }}
        >
          {score}
        </motion.span>
        <span className="text-xs text-text-muted uppercase tracking-wider">OPSEC Score</span>
        <motion.span
          className={clsx(
            'text-xs font-semibold uppercase mt-1 px-2 py-0.5 rounded',
            threatLevel === 'low' && 'bg-green-500/20 text-green-400',
            threatLevel === 'medium' && 'bg-yellow-500/20 text-yellow-400',
            threatLevel === 'high' && 'bg-orange-500/20 text-orange-400',
            threatLevel === 'critical' && 'bg-red-500/20 text-red-400'
          )}
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.8 }}
        >
          {threatLevel}
        </motion.span>
      </div>
    </div>
  );
}

export default ThreatGauge;
