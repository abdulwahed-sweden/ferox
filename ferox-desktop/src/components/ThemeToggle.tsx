// ferox-desktop/src/components/ThemeToggle.tsx
// Animated theme toggle with Framer Motion

import { Sun, Moon } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import { useTheme } from '../hooks/useTheme';

interface ThemeToggleProps {
  /** Show label text next to icon */
  showLabel?: boolean;
  /** Compact mode for status bar */
  compact?: boolean;
}

export function ThemeToggle({ showLabel = false, compact = false }: ThemeToggleProps) {
  const { theme, toggleTheme } = useTheme();
  const isDark = theme === 'dark';

  const iconSize = compact ? 12 : 16;

  return (
    <motion.button
      onClick={toggleTheme}
      className={`
        flex items-center gap-1.5 rounded transition-colors relative overflow-hidden
        ${compact
          ? 'px-1.5 py-0.5 hover:bg-[var(--bg-hover)]'
          : 'px-3 py-2 hover:bg-[var(--bg-hover)]'
        }
      `}
      title={`Switch to ${isDark ? 'light' : 'dark'} theme`}
      aria-label={`Switch to ${isDark ? 'light' : 'dark'} theme`}
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      transition={{ type: 'spring', stiffness: 400, damping: 25 }}
    >
      <AnimatePresence mode="wait" initial={false}>
        {isDark ? (
          <motion.div
            key="sun"
            initial={{ rotate: -90, opacity: 0, scale: 0.5 }}
            animate={{ rotate: 0, opacity: 1, scale: 1 }}
            exit={{ rotate: 90, opacity: 0, scale: 0.5 }}
            transition={{ duration: 0.2, ease: 'easeInOut' }}
          >
            <Sun size={iconSize} className="text-[var(--color-warning)]" />
          </motion.div>
        ) : (
          <motion.div
            key="moon"
            initial={{ rotate: 90, opacity: 0, scale: 0.5 }}
            animate={{ rotate: 0, opacity: 1, scale: 1 }}
            exit={{ rotate: -90, opacity: 0, scale: 0.5 }}
            transition={{ duration: 0.2, ease: 'easeInOut' }}
          >
            <Moon size={iconSize} className="text-[var(--color-primary)]" />
          </motion.div>
        )}
      </AnimatePresence>

      {showLabel && (
        <AnimatePresence mode="wait" initial={false}>
          <motion.span
            key={isDark ? 'light-label' : 'dark-label'}
            initial={{ opacity: 0, y: 5 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -5 }}
            transition={{ duration: 0.15 }}
            className={`text-[var(--text-secondary)] ${compact ? 'text-xs' : 'text-sm'}`}
          >
            {isDark ? 'Light' : 'Dark'}
          </motion.span>
        </AnimatePresence>
      )}
    </motion.button>
  );
}

export default ThemeToggle;
