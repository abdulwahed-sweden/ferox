import { useAppStore } from '../store';
import { Wifi, WifiOff, Monitor, Clock } from 'lucide-react';
import { useEffect, useState } from 'react';
import { ThemeToggle } from './ThemeToggle';

export function StatusBar() {
  const { sessions } = useAppStore();
  const [time, setTime] = useState(new Date());
  const [connected] = useState(true);

  const activeCount = sessions.filter((s) => s.status === 'active').length;
  const totalCount = sessions.length;

  useEffect(() => {
    const interval = setInterval(() => {
      setTime(new Date());
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <footer className="h-6 bg-[var(--bg-secondary)] border-t border-[var(--border-primary)] flex items-center px-3 text-xs text-[var(--text-muted)] select-none">
      {/* Connection status */}
      <div className="flex items-center gap-1.5">
        {connected ? (
          <>
            <Wifi size={12} className="text-[var(--color-ferox-green)]" />
            <span>Connected</span>
          </>
        ) : (
          <>
            <WifiOff size={12} className="text-[var(--color-error)]" />
            <span className="text-[var(--color-error)]">Disconnected</span>
          </>
        )}
      </div>

      <div className="w-px h-3 bg-[var(--border-primary)] mx-3" />

      {/* Session count */}
      <div className="flex items-center gap-1.5">
        <Monitor size={12} />
        <span>
          {activeCount} active / {totalCount} total sessions
        </span>
      </div>

      <div className="flex-1" />

      {/* Theme toggle */}
      <ThemeToggle compact showLabel />

      <div className="w-px h-3 bg-[var(--border-primary)] mx-3" />

      {/* Clock */}
      <div className="flex items-center gap-1.5">
        <Clock size={12} />
        <span>{time.toLocaleTimeString()}</span>
      </div>
    </footer>
  );
}
