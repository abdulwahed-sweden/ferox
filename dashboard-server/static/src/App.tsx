import { useEffect } from 'react';
import { QueryClient, QueryClientProvider, useQuery } from '@tanstack/react-query';
import { Toaster, toast } from 'react-hot-toast';
import { Layout } from './components/Layout';
import { useWebSocket } from './hooks/useWebSocket';
import { useApi, queryKeys } from './hooks/useApi';
import { useDashboardStore } from './store';
import { DashboardPage } from './pages/Dashboard';
import { SessionsPage } from './pages/Sessions';
import { TerminalPage } from './pages/Terminal';
import { CredentialsPage } from './pages/Credentials';
import { NetworkPage } from './pages/Network';
import { MitrePage } from './pages/Mitre';
import { ReportsPage } from './pages/Reports';
import { ConnectionBanner } from './components/Skeleton';
import { ErrorBoundary } from './components/ErrorBoundary';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
    },
  },
});

function DashboardApp() {
  const api = useApi();
  const { setSessions, setCredentials, handleServerEvent, setConnected, activeTab } =
    useDashboardStore();

  // WebSocket connection
  const wsUrl = `ws://${window.location.host}/ws`;
  useWebSocket(wsUrl, {
    onMessage: (event) => {
      handleServerEvent(event);
      // Show toast for important events
      switch (event.type) {
        case 'SessionCreated': {
          const session = event.data as { hostname: string };
          toast.success(`New session on ${session.hostname}`, { icon: '🖥️' });
          break;
        }
        case 'SessionClosed': {
          toast.error('Session terminated', { icon: '❌' });
          break;
        }
        case 'CredentialsFound': {
          const { credentials } = event.data as { credentials: unknown[] };
          toast.success(`${credentials.length} credential(s) harvested!`, { icon: '🔑' });
          break;
        }
        case 'OpsecAlert': {
          const { level, message } = event.data as { level: string; message: string };
          if (level === 'high') {
            toast.error(`OPSEC: ${message}`, { icon: '🚨', duration: 6000 });
          } else {
            toast(message, { icon: '⚠️', duration: 4000 });
          }
          break;
        }
      }
    },
    onConnect: () => {
      setConnected(true);
      toast.success('Connected to server', { icon: '🔗', duration: 2000 });
    },
    onDisconnect: () => {
      setConnected(false);
      toast.error('Disconnected from server', { icon: '🔌', duration: 4000 });
    },
  });

  // Initial data fetch
  const { data: sessionsData } = useQuery({
    queryKey: queryKeys.sessions,
    queryFn: api.getSessions,
  });

  const { data: credentialsData } = useQuery({
    queryKey: queryKeys.credentials,
    queryFn: api.getCredentials,
  });

  // Update store when data loads
  useEffect(() => {
    if (sessionsData?.sessions) {
      setSessions(sessionsData.sessions);
    }
  }, [sessionsData, setSessions]);

  useEffect(() => {
    if (credentialsData) {
      setCredentials(credentialsData);
    }
  }, [credentialsData, setCredentials]);

  // Render active page
  const renderPage = () => {
    switch (activeTab) {
      case 'dashboard':
        return <DashboardPage />;
      case 'sessions':
        return <SessionsPage />;
      case 'terminal':
        return <TerminalPage />;
      case 'credentials':
        return <CredentialsPage />;
      case 'network':
        return <NetworkPage />;
      case 'mitre':
        return <MitrePage />;
      case 'reports':
        return <ReportsPage />;
      default:
        return <DashboardPage />;
    }
  };

  const { isConnected } = useDashboardStore();

  return (
    <>
      <ConnectionBanner isConnected={isConnected} />
      <Layout>{renderPage()}</Layout>
      <Toaster
        position="top-right"
        toastOptions={{
          duration: 3000,
          style: {
            background: '#1a1f3a',
            color: '#ffffff',
            border: '1px solid #2d3561',
            borderRadius: '8px',
          },
          success: {
            iconTheme: {
              primary: '#00ff88',
              secondary: '#0a0e27',
            },
          },
          error: {
            iconTheme: {
              primary: '#ff3366',
              secondary: '#0a0e27',
            },
          },
        }}
      />
    </>
  );
}

export default function App() {
  return (
    <ErrorBoundary>
      <QueryClientProvider client={queryClient}>
        <DashboardApp />
      </QueryClientProvider>
    </ErrorBoundary>
  );
}
