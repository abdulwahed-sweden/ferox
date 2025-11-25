import { Sun, Moon } from 'lucide-react';
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

  return (
    <button
      onClick={toggleTheme}
      className={`
        flex items-center gap-1.5 rounded transition-colors
        ${compact
          ? 'px-1.5 py-0.5 hover:bg-[var(--bg-hover)]'
          : 'px-3 py-2 hover:bg-[var(--bg-hover)]'
        }
      `}
      title={`Switch to ${isDark ? 'light' : 'dark'} theme`}
      aria-label={`Switch to ${isDark ? 'light' : 'dark'} theme`}
    >
      {isDark ? (
        <Sun size={compact ? 12 : 16} className="text-[var(--color-warning)]" />
      ) : (
        <Moon size={compact ? 12 : 16} className="text-[var(--color-primary)]" />
      )}
      {showLabel && (
        <span className={`text-[var(--text-secondary)] ${compact ? 'text-xs' : 'text-sm'}`}>
          {isDark ? 'Light' : 'Dark'}
        </span>
      )}
    </button>
  );
}

export default ThemeToggle;
