import { useEffect } from 'react';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { useAppStore } from '../store';
import toast from 'react-hot-toast';
import { sessionToasts, moduleToasts, errorToasts } from '../lib/toast';
import type { Session } from '../types';

// Event types from the Tauri backend
interface SessionCreatedEvent {
  session: Session;
}

interface SessionUpdatedEvent {
  session_id: string;
  updates: Partial<Session>;
}

interface SessionClosedEvent {
  session_id: string;
  hostname?: string;
}

interface SessionDiedEvent {
  session_id: string;
  hostname: string;
}

interface SessionPrivilegeChangedEvent {
  session_id: string;
  hostname: string;
  old_privilege: string;
  new_privilege: string;
}

interface OpsecAlertEvent {
  session_id: string;
  level: 'info' | 'warning' | 'critical';
  message: string;
  recommendation: string;
}

interface CredentialsFoundEvent {
  session_id: string;
  count: number;
  types: string[];
}

interface CommandOutputEvent {
  session_id: string;
  command: string;
  output: string;
  success: boolean;
}

interface LateralMoveEvent {
  session_id: string;
  target_host: string;
  success: boolean;
  new_session_id?: string;
}

interface DiscoveryCompleteEvent {
  session_id: string;
  hosts_found: number;
}

export function useTauriEvents() {
  const { addSession, updateSession, removeSession, sessions } = useAppStore();

  useEffect(() => {
    const unlisteners: UnlistenFn[] = [];

    // Listen for new sessions
    listen<SessionCreatedEvent>('session:created', (event) => {
      const { session } = event.payload;
      addSession(session);
      sessionToasts.created(session.hostname, session.ip_address);
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for session updates
    listen<SessionUpdatedEvent>('session:updated', (event) => {
      updateSession(event.payload.session_id, event.payload.updates);
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for session died
    listen<SessionDiedEvent>('session:died', (event) => {
      updateSession(event.payload.session_id, { status: 'dead' });
      sessionToasts.died(event.payload.hostname);
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for session closures
    listen<SessionClosedEvent>('session:closed', (event) => {
      const session = sessions.find((s) => s.id === event.payload.session_id);
      removeSession(event.payload.session_id);
      sessionToasts.terminated(event.payload.hostname || session?.hostname || 'Unknown');
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for privilege changes
    listen<SessionPrivilegeChangedEvent>('session:privilege_changed', (event) => {
      // Cast to PrivilegeLevel - backend ensures valid values
      updateSession(event.payload.session_id, {
        privileges: event.payload.new_privilege as Session['privileges']
      });
      sessionToasts.privilegeEscalated(event.payload.hostname, event.payload.new_privilege);
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for OPSEC alerts
    listen<OpsecAlertEvent>('opsec:alert', (event) => {
      const { level, message, recommendation } = event.payload;
      if (level === 'critical') {
        toast.error(`OPSEC Alert: ${message}\n${recommendation}`, { duration: 8000, icon: '🚨' });
      } else if (level === 'warning') {
        toast(message, { icon: '⚠️', duration: 5000 });
      } else {
        toast(message, { icon: 'ℹ️', duration: 3000 });
      }
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for credentials found
    listen<CredentialsFoundEvent>('credentials:found', (event) => {
      moduleToasts.credentialHarvest(event.payload.count);
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for command output
    listen<CommandOutputEvent>('command:output', (event) => {
      // Commands are handled inline, but we can show errors
      if (!event.payload.success) {
        errorToasts.generic(`Command failed on session`);
      }
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for lateral movement
    listen<LateralMoveEvent>('lateral:complete', (event) => {
      if (event.payload.success) {
        toast.success(`Established session on ${event.payload.target_host}`, {
          duration: 4000,
          icon: '🚀',
        });
      } else {
        toast.error(`Lateral movement to ${event.payload.target_host} failed`, {
          duration: 5000,
          icon: '❌',
        });
      }
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for discovery complete
    listen<DiscoveryCompleteEvent>('discovery:complete', (event) => {
      moduleToasts.discoveryComplete(event.payload.hosts_found);
    }).then((unlisten) => unlisteners.push(unlisten));

    // Cleanup
    return () => {
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, [addSession, updateSession, removeSession, sessions]);
}
