// src/components/ConnectionStatus.tsx
// Connection status indicator showing Tauri backend connectivity

import { useState, useEffect } from 'react';
import { Wifi, WifiOff, RefreshCw } from 'lucide-react';
import { getConnectionState, onConnectionStateChange } from '@/lib/tauri';
import { cn } from '@/lib/utils';

type ConnectionState = 'connected' | 'disconnected' | 'connecting';

interface ConnectionStatusProps {
  showLabel?: boolean;
  compact?: boolean;
  className?: string;
  onReconnect?: () => void;
}

const stateConfig: Record<ConnectionState, {
  icon: typeof Wifi;
  label: string;
  dotColor: string;
  textColor: string;
}> = {
  connected: {
    icon: Wifi,
    label: 'Connected',
    dotColor: 'bg-success-new',
    textColor: 'text-success-text',
  },
  disconnected: {
    icon: WifiOff,
    label: 'Disconnected',
    dotColor: 'bg-danger-new',
    textColor: 'text-danger-text',
  },
  connecting: {
    icon: RefreshCw,
    label: 'Connecting...',
    dotColor: 'bg-warning-new',
    textColor: 'text-warning-text',
  },
};

export function ConnectionStatus({
  showLabel = true,
  compact = false,
  className,
  onReconnect,
}: ConnectionStatusProps) {
  const [state, setState] = useState<ConnectionState>(getConnectionState);
  
  useEffect(() => {
    const unsubscribe = onConnectionStateChange((newState) => {
      setState(newState);
    });
    return unsubscribe;
  }, []);

  const config = stateConfig[state];
  const Icon = config.icon;

  if (compact) {
    return (
      <div
        className={cn(
          'w-2 h-2 rounded-full',
          config.dotColor,
          state === 'connected' && 'animate-pulse',
          className
        )}
        title={config.label}
        role="status"
        aria-label={config.label}
      />
    );
  }

  return (
    <div
      className={cn(
        'flex items-center gap-2 px-3 py-1.5 rounded-lg',
        'bg-surface-hover border border-border-subtle',
        className
      )}
      role="status"
      aria-live="polite"
    >
      <div
        className={cn(
          'w-2 h-2 rounded-full',
          config.dotColor,
          state === 'connected' && 'animate-pulse'
        )}
      />
      <Icon
        className={cn(
          'w-4 h-4',
          config.textColor,
          state === 'connecting' && 'animate-spin'
        )}
      />
      {showLabel && (
        <span className={cn('text-sm font-medium', config.textColor)}>
          {config.label}
        </span>
      )}
      {state === 'disconnected' && onReconnect && (
        <button
          onClick={onReconnect}
          className="ml-2 text-xs text-primary-text hover:text-primary-new transition-colors"
          aria-label="Reconnect"
        >
          Retry
        </button>
      )}
    </div>
  );
}

// Toast/notification variant for connection changes
export function ConnectionToast({
  state,
  onDismiss,
}: {
  state: ConnectionState;
  onDismiss?: () => void;
}) {
  const config = stateConfig[state];
  const Icon = config.icon;

  return (
    <div
      className={cn(
        'flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg',
        'bg-surface-elevated border border-border-subtle',
        'animate-fade-in'
      )}
      role="alert"
    >
      <Icon className={cn('w-5 h-5', config.textColor)} />
      <div className="flex-1">
        <p className={cn('text-sm font-medium', config.textColor)}>
          {state === 'connected' ? 'Connection Restored' : 
           state === 'disconnected' ? 'Connection Lost' : 
           'Reconnecting...'}
        </p>
        <p className="text-xs text-content-tertiary">
          {state === 'connected'
            ? 'Backend connection is stable'
            : state === 'disconnected'
            ? 'Unable to reach the Tauri backend'
            : 'Attempting to reconnect...'}
        </p>
      </div>
      {onDismiss && (
        <button
          onClick={onDismiss}
          className="text-content-tertiary hover:text-content-primary transition-colors"
          aria-label="Dismiss"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      )}
    </div>
  );
}

// Hook for using connection status in components
export function useConnectionStatus() {
  const [state, setState] = useState<ConnectionState>(getConnectionState);

  useEffect(() => {
    const unsubscribe = onConnectionStateChange((newState) => {
      setState(newState);
    });
    return unsubscribe;
  }, []);

  return {
    state,
    isConnected: state === 'connected',
    isDisconnected: state === 'disconnected',
    isConnecting: state === 'connecting',
  };
}

export default ConnectionStatus;
