import { useEffect, lazy, Suspense } from 'react';
import { Toaster } from 'react-hot-toast';
import { useAppStore } from './store';
import { getSessions, getSessionTree } from './lib/tauri';
import { SessionTree } from './components/SessionTree';
import { SessionFilters } from './components/SessionFilters';
import { TabBar } from './components/TabBar';
import { ContextMenu } from './components/ContextMenu';
import { StatusBar } from './components/StatusBar';
import { ErrorBoundary } from './components/ErrorBoundary';
import { Spinner } from './components/Loading';
import { MenuBar } from './components/MenuBar';
import { useTauriEvents } from './hooks/useTauriEvents';
import { useKeyboardShortcuts } from './hooks/useKeyboardShortcuts';
import { useResizable } from './hooks/useResizable';
import { Shield, Search, X } from 'lucide-react';
import { clsx } from 'clsx';

// Lazy load heavy components for better initial load time
const TabContent = lazy(() => import('./components/TabContent').then(m => ({ default: m.TabContent })));

function App() {
  const {
    setSessions,
    setSessionTree,
    activeTabId,
    contextMenu,
    hideContextMenu,
    searchQuery,
    setSearchQuery,
    setSessionsLoading,
    setSessionsError,
    setSidebarWidth,
  } = useAppStore();

  // Resizable sidebar
  const { size: sidebarWidth, isResizing, handleMouseDown } = useResizable({
    initialSize: 280,
    minSize: 200,
    maxSize: 450,
    onResize: setSidebarWidth,
  });

  // Subscribe to Tauri events
  useTauriEvents();

  // Register keyboard shortcuts
  const { searchInputRef } = useKeyboardShortcuts();

  // Load sessions on mount
  useEffect(() => {
    let mounted = true;

    const loadSessions = async (isInitial = false) => {
      if (isInitial) {
        setSessionsLoading(true);
        setSessionsError(null);
      }

      try {
        const response = await getSessions();
        if (!mounted) return;
        setSessions(response.sessions);
        const tree = await getSessionTree();
        if (!mounted) return;
        setSessionTree(tree);
        setSessionsError(null);
      } catch (error) {
        console.error('Failed to load sessions:', error);
        if (mounted) {
          setSessionsError(error instanceof Error ? error.message : 'Failed to load sessions');
        }
      } finally {
        if (mounted && isInitial) {
          setSessionsLoading(false);
        }
      }
    };

    loadSessions(true);

    // Refresh sessions periodically
    const interval = setInterval(() => loadSessions(false), 5000);
    return () => {
      mounted = false;
      clearInterval(interval);
    };
  }, [setSessions, setSessionTree, setSessionsLoading, setSessionsError]);

  // Close context menu on click outside
  useEffect(() => {
    const handleClick = () => {
      if (contextMenu.visible) {
        hideContextMenu();
      }
    };

    window.addEventListener('click', handleClick);
    return () => window.removeEventListener('click', handleClick);
  }, [contextMenu.visible, hideContextMenu]);

  return (
    <div className="h-screen flex flex-col bg-dark-900 text-text-primary">
      {/* Menu Bar */}
      <MenuBar />

      {/* Main Content */}
      <div className={clsx('flex-1 flex min-h-0', isResizing && 'select-none')}>
        {/* Sidebar - Session Tree */}
        <aside
          className="bg-dark-800 flex flex-col relative"
          style={{ width: sidebarWidth }}
        >
          <div className="p-3 border-b border-dark-600">
            <h2 className="text-sm font-semibold text-text-primary mb-2">Sessions</h2>
            {/* Search Input */}
            <div className="relative">
              <Search size={14} className="absolute left-2 top-1/2 -translate-y-1/2 text-text-muted" />
              <input
                ref={searchInputRef}
                type="text"
                placeholder="Search... (/)"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-7 pr-7 py-1.5 text-xs bg-dark-700 border border-dark-600 rounded text-text-primary placeholder:text-text-muted focus:outline-none focus:border-ferox-green/50"
              />
              {searchQuery && (
                <button
                  onClick={() => setSearchQuery('')}
                  className="absolute right-2 top-1/2 -translate-y-1/2 text-text-muted hover:text-text-primary"
                >
                  <X size={14} />
                </button>
              )}
            </div>
          </div>
          <SessionFilters />
          <div className="flex-1 overflow-auto">
            <ErrorBoundary name="Sessions">
              <SessionTree />
            </ErrorBoundary>
          </div>

          {/* Resize handle */}
          <div
            onMouseDown={handleMouseDown}
            className={clsx(
              'absolute right-0 top-0 bottom-0 w-1 cursor-col-resize',
              'hover:bg-ferox-green/30 transition-colors',
              isResizing && 'bg-ferox-green/50'
            )}
          />
        </aside>

        {/* Main Panel */}
        <main className="flex-1 flex flex-col min-w-0">
          {/* Tab Bar */}
          <TabBar />

          {/* Tab Content */}
          <div className="flex-1 min-h-0">
            {activeTabId ? (
              <ErrorBoundary name="Terminal">
                <Suspense
                  fallback={
                    <div className="h-full flex items-center justify-center">
                      <Spinner size="lg" />
                    </div>
                  }
                >
                  <TabContent />
                </Suspense>
              </ErrorBoundary>
            ) : (
              <div className="h-full flex flex-col items-center justify-center text-text-muted">
                <Shield size={64} className="mb-4 opacity-20" />
                <p className="text-lg">No session selected</p>
                <p className="text-sm mt-2">
                  Double-click a session to open a terminal
                </p>
              </div>
            )}
          </div>
        </main>
      </div>

      {/* Status Bar */}
      <StatusBar />

      {/* Context Menu */}
      {contextMenu.visible && <ContextMenu />}

      {/* Toast notifications */}
      <Toaster
        position="top-right"
        toastOptions={{
          style: {
            background: '#151d30',
            color: '#fff',
            border: '1px solid #243049',
          },
          success: {
            iconTheme: {
              primary: '#00ff88',
              secondary: '#151d30',
            },
          },
          error: {
            iconTheme: {
              primary: '#ff3366',
              secondary: '#151d30',
            },
          },
        }}
      />
    </div>
  );
}

export default App;
