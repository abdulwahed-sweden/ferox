import { create } from 'zustand';
import type { Session, Command, Credential, ServerEvent } from '../types';

interface DashboardState {
  // Connection
  isConnected: boolean;
  clientId: string | null;

  // Sessions
  sessions: Session[];
  selectedSessionId: string | null;

  // Commands (by session ID)
  commands: Record<string, Command[]>;

  // Credentials
  credentials: Credential[];

  // UI state
  sidebarOpen: boolean;
  activeTab: string;

  // Notifications
  notifications: Notification[];

  // Actions
  setConnected: (connected: boolean, clientId?: string | null) => void;
  setSessions: (sessions: Session[]) => void;
  updateSession: (session: Session) => void;
  removeSession: (sessionId: string) => void;
  selectSession: (sessionId: string | null) => void;
  addCommand: (sessionId: string, command: Command) => void;
  updateCommandOutput: (
    sessionId: string,
    commandId: string,
    output: string,
    isComplete: boolean,
    success?: boolean | null
  ) => void;
  setCredentials: (credentials: Credential[]) => void;
  addCredentials: (credentials: Credential[]) => void;
  toggleSidebar: () => void;
  setActiveTab: (tab: string) => void;
  addNotification: (notification: Omit<Notification, 'id' | 'timestamp'>) => void;
  removeNotification: (id: string) => void;
  handleServerEvent: (event: ServerEvent) => void;
}

interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message: string;
  timestamp: Date;
}

export const useDashboardStore = create<DashboardState>((set, get) => ({
  // Initial state
  isConnected: false,
  clientId: null,
  sessions: [],
  selectedSessionId: null,
  commands: {},
  credentials: [],
  sidebarOpen: true,
  activeTab: 'dashboard',
  notifications: [],

  // Actions
  setConnected: (connected, clientId = null) =>
    set({ isConnected: connected, clientId }),

  setSessions: (sessions) => set({ sessions }),

  updateSession: (session) =>
    set((state) => ({
      sessions: state.sessions.some((s) => s.id === session.id)
        ? state.sessions.map((s) => (s.id === session.id ? session : s))
        : [...state.sessions, session],
    })),

  removeSession: (sessionId) =>
    set((state) => ({
      sessions: state.sessions.filter((s) => s.id !== sessionId),
      selectedSessionId:
        state.selectedSessionId === sessionId ? null : state.selectedSessionId,
    })),

  selectSession: (sessionId) => set({ selectedSessionId: sessionId }),

  addCommand: (sessionId, command) =>
    set((state) => ({
      commands: {
        ...state.commands,
        [sessionId]: [...(state.commands[sessionId] || []), command],
      },
    })),

  updateCommandOutput: (sessionId, commandId, output, isComplete, success) =>
    set((state) => ({
      commands: {
        ...state.commands,
        [sessionId]: (state.commands[sessionId] || []).map((cmd) =>
          cmd.id === commandId
            ? {
                ...cmd,
                output: cmd.output + output,
                completed_at: isComplete ? new Date().toISOString() : null,
                success: success ?? cmd.success,
              }
            : cmd
        ),
      },
    })),

  setCredentials: (credentials) => set({ credentials }),

  addCredentials: (newCreds) =>
    set((state) => ({
      credentials: [...state.credentials, ...newCreds],
    })),

  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),

  setActiveTab: (tab) => set({ activeTab: tab }),

  addNotification: (notification) =>
    set((state) => ({
      notifications: [
        ...state.notifications,
        {
          ...notification,
          id: crypto.randomUUID(),
          timestamp: new Date(),
        },
      ],
    })),

  removeNotification: (id) =>
    set((state) => ({
      notifications: state.notifications.filter((n) => n.id !== id),
    })),

  handleServerEvent: (event) => {
    const state = get();

    switch (event.type) {
      case 'SessionCreated':
      case 'SessionUpdated': {
        const session = event.data as Session;
        state.updateSession(session);
        if (event.type === 'SessionCreated') {
          state.addNotification({
            type: 'success',
            title: 'New Session',
            message: `Session established on ${session.hostname}`,
          });
        }
        break;
      }

      case 'SessionClosed': {
        const { session_id } = event.data as { session_id: string };
        state.removeSession(session_id);
        state.addNotification({
          type: 'warning',
          title: 'Session Closed',
          message: `Session ${session_id.slice(0, 8)} terminated`,
        });
        break;
      }

      case 'CommandOutput': {
        const { command_id, session_id, output, is_complete, success } =
          event.data as {
            command_id: string;
            session_id: string;
            output: string;
            is_complete: boolean;
            success: boolean | null;
          };
        state.updateCommandOutput(
          session_id,
          command_id,
          output,
          is_complete,
          success
        );
        break;
      }

      case 'CredentialsFound': {
        const { credentials } = event.data as { credentials: Credential[] };
        state.addCredentials(credentials);
        state.addNotification({
          type: 'success',
          title: 'Credentials Found',
          message: `${credentials.length} new credential(s) harvested`,
        });
        break;
      }

      case 'OpsecAlert': {
        const { level, message } = event.data as {
          level: string;
          message: string;
        };
        state.addNotification({
          type: level === 'high' ? 'error' : 'warning',
          title: 'OPSEC Alert',
          message,
        });
        break;
      }

      case 'Connected': {
        const { client_id } = event.data as { client_id: string };
        state.setConnected(true, client_id);
        break;
      }

      case 'Error': {
        const { message } = event.data as { message: string };
        state.addNotification({
          type: 'error',
          title: 'Error',
          message,
        });
        break;
      }
    }
  },
}));
