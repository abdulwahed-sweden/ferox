// ferox-desktop/src/components/ui/PulseIndicator.tsx
// Animated pulse indicator for status display

import { motion } from 'framer-motion';

type StatusType = 'active' | 'warning' | 'error' | 'idle' | 'success';

interface PulseIndicatorProps {
  status: StatusType;
  size?: 'sm' | 'md' | 'lg';
  showRing?: boolean;
  className?: string;
}

const statusColors: Record<StatusType, { dot: string; ring: string }> = {
  active: { dot: 'bg-info', ring: 'bg-info/30' },
  success: { dot: 'bg-success', ring: 'bg-success/30' },
  warning: { dot: 'bg-warning', ring: 'bg-warning/30' },
  error: { dot: 'bg-danger', ring: 'bg-danger/30' },
  idle: { dot: 'bg-[var(--content-tertiary)]', ring: 'bg-[var(--content-tertiary)]/30' },
};

const sizes = {
  sm: { dot: 'w-2 h-2', ring: 'w-4 h-4' },
  md: { dot: 'w-3 h-3', ring: 'w-6 h-6' },
  lg: { dot: 'w-4 h-4', ring: 'w-8 h-8' },
};

export function PulseIndicator({
  status,
  size = 'md',
  showRing = true,
  className = '',
}: PulseIndicatorProps) {
  const colors = statusColors[status];
  const sizeClasses = sizes[size];
  const shouldPulse = status === 'active' || status === 'warning' || status === 'error';

  return (
    <div className={`relative flex items-center justify-center ${className}`}>
      {/* Pulse ring */}
      {showRing && shouldPulse && (
        <motion.div
          className={`absolute ${sizeClasses.ring} ${colors.ring} rounded-full`}
          animate={{
            scale: [1, 1.5, 1],
            opacity: [0.5, 0, 0.5],
          }}
          transition={{
            duration: status === 'error' ? 0.8 : 1.5,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      )}
      {/* Main dot */}
      <motion.div
        className={`${sizeClasses.dot} ${colors.dot} rounded-full`}
        animate={
          shouldPulse
            ? {
                scale: [1, 1.1, 1],
              }
            : {}
        }
        transition={{
          duration: 1.5,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />
    </div>
  );
}

interface StatusDotProps {
  active: boolean;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

export function StatusDot({ active, size = 'md', className = '' }: StatusDotProps) {
  return <PulseIndicator status={active ? 'active' : 'idle'} size={size} className={className} />;
}

interface ConnectionIndicatorProps {
  connected: boolean;
  className?: string;
}

export function ConnectionIndicator({ connected, className = '' }: ConnectionIndicatorProps) {
  return (
    <div className={`flex items-center gap-2 ${className}`}>
      <PulseIndicator status={connected ? 'success' : 'error'} size="sm" />
      <span className={`text-sm ${connected ? 'text-success-text' : 'text-danger-text'}`}>
        {connected ? 'Connected' : 'Disconnected'}
      </span>
    </div>
  );
}
