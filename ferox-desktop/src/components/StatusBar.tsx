// ferox-desktop/src/components/StatusBar.tsx
// Enhanced StatusBar with animations

import { useAppStore } from '../store';
import { Wifi, WifiOff, Monitor, Clock, Activity } from 'lucide-react';
import { useEffect, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ThemeToggle } from './ThemeToggle';
import { PulseIndicator } from './ui/PulseIndicator';

export function StatusBar() {
  const { sessions } = useAppStore();
  const [time, setTime] = useState(new Date());
  const [connected] = useState(true);
  const [cpuUsage] = useState(Math.floor(Math.random() * 30) + 10);

  const activeCount = sessions.filter((s) => s.status === 'active').length;
  const totalCount = sessions.length;

  useEffect(() => {
    const interval = setInterval(() => {
      setTime(new Date());
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <motion.footer
      initial={{ y: 24, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ duration: 0.3, delay: 0.2 }}
      className="h-6 bg-[var(--bg-secondary)] border-t border-[var(--border-primary)] flex items-center px-3 text-xs text-[var(--text-muted)] select-none"
    >
      {/* Connection status with pulse */}
      <AnimatePresence mode="wait">
        <motion.div
          key={connected ? 'connected' : 'disconnected'}
          initial={{ opacity: 0, x: -10 }}
          animate={{ opacity: 1, x: 0 }}
          exit={{ opacity: 0, x: 10 }}
          transition={{ duration: 0.2 }}
          className="flex items-center gap-1.5"
        >
          {connected ? (
            <>
              <PulseIndicator status="success" size="sm" showRing={false} />
              <Wifi size={12} className="text-[var(--color-ferox-green)]" />
              <span>Connected</span>
            </>
          ) : (
            <>
              <PulseIndicator status="error" size="sm" />
              <WifiOff size={12} className="text-[var(--color-error)]" />
              <span className="text-[var(--color-error)]">Disconnected</span>
            </>
          )}
        </motion.div>
      </AnimatePresence>

      <div className="w-px h-3 bg-[var(--border-primary)] mx-3" />

      {/* Session count with animation */}
      <motion.div
        className="flex items-center gap-1.5"
        whileHover={{ scale: 1.02 }}
        transition={{ type: 'spring', stiffness: 400, damping: 25 }}
      >
        <Monitor size={12} />
        <span>
          <motion.span
            key={activeCount}
            initial={{ opacity: 0, y: -5 }}
            animate={{ opacity: 1, y: 0 }}
            className="inline-block"
          >
            {activeCount}
          </motion.span>{' '}
          active /{' '}
          <motion.span
            key={totalCount}
            initial={{ opacity: 0, y: -5 }}
            animate={{ opacity: 1, y: 0 }}
            className="inline-block"
          >
            {totalCount}
          </motion.span>{' '}
          total sessions
        </span>
      </motion.div>

      <div className="w-px h-3 bg-[var(--border-primary)] mx-3" />

      {/* CPU indicator */}
      <div className="flex items-center gap-1.5">
        <Activity size={12} className="text-[var(--text-muted)]" />
        <div className="flex items-center gap-1">
          <div className="w-12 h-1.5 bg-[var(--bg-tertiary)] rounded-full overflow-hidden">
            <motion.div
              className={`h-full rounded-full ${
                cpuUsage > 70
                  ? 'bg-danger-text'
                  : cpuUsage > 40
                  ? 'bg-warning-text'
                  : 'bg-info-text'
              }`}
              initial={{ width: 0 }}
              animate={{ width: `${cpuUsage}%` }}
              transition={{ duration: 0.5, ease: 'easeOut' }}
            />
          </div>
          <span className="w-8">{cpuUsage}%</span>
        </div>
      </div>

      <div className="flex-1" />

      {/* Theme toggle */}
      <ThemeToggle compact showLabel />

      <div className="w-px h-3 bg-[var(--border-primary)] mx-3" />

      {/* Clock with tick animation */}
      <motion.div
        className="flex items-center gap-1.5"
        whileHover={{ scale: 1.02 }}
      >
        <motion.div
          animate={{ rotate: [0, 5, 0, -5, 0] }}
          transition={{ duration: 0.5, repeat: Infinity, repeatDelay: 59.5 }}
        >
          <Clock size={12} />
        </motion.div>
        <span className="font-mono">{time.toLocaleTimeString()}</span>
      </motion.div>
    </motion.footer>
  );
}
