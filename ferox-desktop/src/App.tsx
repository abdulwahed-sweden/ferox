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
import { useTauriEvents } from './hooks/useTauriEvents';
import { useKeyboardShortcuts } from './hooks/useKeyboardShortcuts';
import { useResizable } from './hooks/useResizable';
import { Shield, Search, X, Package, Radar, KeyRound, FileText, Clock, StickyNote, ChevronDown } from 'lucide-react';
import { useState, useRef } from 'react';
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
    addTab,
    tabs,
  } = useAppStore();

  // Tools dropdown state
  const [toolsOpen, setToolsOpen] = useState(false);
  const toolsRef = useRef<HTMLDivElement>(null);

  // Generic tab opener
  const openToolTab = (type: 'payloads' | 'scanner' | 'credentials' | 'eventlog' | 'scheduler' | 'notes', title: string, icon: string) => {
    const existing = tabs.find(t => t.type === type);
    if (existing) {
      useAppStore.getState().setActiveTab(existing.id);
      setToolsOpen(false);
      return;
    }

    addTab({
      id: `${type}-${Date.now()}`,
      type,
      sessionId: '',
      title,
      icon,
    });
    setToolsOpen(false);
  };


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

  // Close context menu and tools dropdown on click outside
  useEffect(() => {
    const handleClick = (e: MouseEvent) => {
      if (contextMenu.visible) {
        hideContextMenu();
      }
      // Close tools dropdown when clicking outside
      if (toolsOpen && toolsRef.current && !toolsRef.current.contains(e.target as Node)) {
        setToolsOpen(false);
      }
    };

    window.addEventListener('click', handleClick);
    return () => window.removeEventListener('click', handleClick);
  }, [contextMenu.visible, hideContextMenu, toolsOpen]);

  return (
    <div className="h-screen flex flex-col bg-dark-900 text-text-primary">
      {/* Header / Menu Bar */}
      <header className="h-10 bg-dark-800 border-b border-dark-600 flex items-center px-4 gap-4 select-none">
        <div className="flex items-center gap-2 text-ferox-green">
          <Shield size={18} />
          <span className="font-semibold text-sm">Ferox C2</span>
        </div>
        <nav className="flex items-center gap-1 text-sm">
          <button className="px-3 py-1 rounded hover:bg-dark-600 text-text-secondary hover:text-text-primary transition-colors">
            File
          </button>
          <button className="px-3 py-1 rounded hover:bg-dark-600 text-text-secondary hover:text-text-primary transition-colors">
            Session
          </button>
          <div className="relative" ref={toolsRef}>
            <button
              onClick={() => setToolsOpen(!toolsOpen)}
              className="px-3 py-1 rounded hover:bg-dark-600 text-text-secondary hover:text-text-primary transition-colors flex items-center gap-1"
            >
              Tools
              <ChevronDown size={12} className={clsx('transition-transform', toolsOpen && 'rotate-180')} />
            </button>
            {toolsOpen && (
              <div className="absolute top-full left-0 mt-1 bg-dark-800 border border-dark-600 rounded-lg shadow-xl py-1 min-w-48 z-50">
                <button
                  onClick={() => openToolTab('payloads', 'Payload Builder', 'package')}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-dark-700 text-text-secondary hover:text-text-primary flex items-center gap-2"
                >
                  <Package size={14} className="text-purple-400" />
                  Payload Builder
                </button>
                <button
                  onClick={() => openToolTab('scanner', 'Network Scanner', 'radar')}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-dark-700 text-text-secondary hover:text-text-primary flex items-center gap-2"
                >
                  <Radar size={14} className="text-blue-400" />
                  Network Scanner
                </button>
                <button
                  onClick={() => openToolTab('credentials', 'Credentials Viewer', 'key-round')}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-dark-700 text-text-secondary hover:text-text-primary flex items-center gap-2"
                >
                  <KeyRound size={14} className="text-yellow-400" />
                  Credentials Viewer
                </button>
                <div className="h-px bg-dark-600 my-1" />
                <button
                  onClick={() => openToolTab('eventlog', 'Event Log', 'file-text')}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-dark-700 text-text-secondary hover:text-text-primary flex items-center gap-2"
                >
                  <FileText size={14} className="text-cyan-400" />
                  Event Log
                </button>
                <button
                  onClick={() => openToolTab('scheduler', 'Task Scheduler', 'clock')}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-dark-700 text-text-secondary hover:text-text-primary flex items-center gap-2"
                >
                  <Clock size={14} className="text-orange-400" />
                  Task Scheduler
                </button>
                <button
                  onClick={() => openToolTab('notes', 'Notes', 'sticky-note')}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-dark-700 text-text-secondary hover:text-text-primary flex items-center gap-2"
                >
                  <StickyNote size={14} className="text-pink-400" />
                  Notes
                </button>
              </div>
            )}
          </div>
          <button className="px-3 py-1 rounded hover:bg-dark-600 text-text-secondary hover:text-text-primary transition-colors">
            View
          </button>
          <button className="px-3 py-1 rounded hover:bg-dark-600 text-text-secondary hover:text-text-primary transition-colors">
            Help
          </button>
        </nav>
        <div className="flex-1" />
        <div className="text-xs text-text-muted">v1.0.0</div>
      </header>

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
