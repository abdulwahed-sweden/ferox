import { useEffect, lazy, Suspense, useState } from "react";
import { Toaster } from "react-hot-toast";
import { useAppStore } from "./store";
import { getSessions, getSessionTree } from "./lib/tauri";
import { SessionTree } from "./components/SessionTree";
import { SessionFilters } from "./components/SessionFilters";
import { TabBar } from "./components/TabBar";
import { ContextMenu } from "./components/ContextMenu";
import { StatusBar } from "./components/StatusBar";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { Spinner } from "./components/Loading";
import { useTauriEvents } from "./hooks/useTauriEvents";
import { useKeyboardShortcuts } from "./hooks/useKeyboardShortcuts";
import { useResizable } from "./hooks/useResizable";
import { MenuBar } from "./components/menus/MenuBar";
import {
  AboutModal,
  ShortcutsModal,
  SettingsModal,
  NewSessionModal,
} from "./components/modals";
import { Search, X, Plus } from "lucide-react";
import { Logo } from "./components/ui/Logo";
import { clsx } from "clsx";

// Lazy load heavy components for better initial load time
const TabContent = lazy(() =>
  import("./components/TabContent").then((m) => ({ default: m.TabContent }))
);

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
    sessions,
  } = useAppStore();

  // Modal states
  const [showAbout, setShowAbout] = useState(false);
  const [showShortcuts, setShowShortcuts] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [showNewSession, setShowNewSession] = useState(false);

  // Sidebar visibility
  const [sidebarVisible, setSidebarVisible] = useState(true);

  // Resizable sidebar
  const {
    size: sidebarWidth,
    isResizing,
    handleMouseDown,
  } = useResizable({
    initialSize: 280,
    minSize: 200,
    maxSize: 450,
    onResize: setSidebarWidth,
  });

  // Subscribe to Tauri events
  useTauriEvents();

  // Register keyboard shortcuts
  const { searchInputRef } = useKeyboardShortcuts();

  // Enhanced keyboard shortcuts for menu actions
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const isMeta = e.metaKey || e.ctrlKey;

      // Cmd+N: New Session
      if (isMeta && e.key === "n") {
        e.preventDefault();
        setShowNewSession(true);
        return;
      }

      // Cmd+,: Settings
      if (isMeta && e.key === ",") {
        e.preventDefault();
        setShowSettings(true);
        return;
      }

      // Cmd+B: Toggle Sidebar
      if (isMeta && e.key === "b") {
        e.preventDefault();
        setSidebarVisible((prev) => !prev);
        return;
      }

      // Cmd+Shift+T: Toggle Theme
      if (isMeta && e.shiftKey && e.key === "T") {
        e.preventDefault();
        // Theme toggle is handled by useTheme hook in MenuBar
        return;
      }

      // Cmd+Shift+/: Show Shortcuts
      if (isMeta && e.shiftKey && e.key === "?") {
        e.preventDefault();
        setShowShortcuts(true);
        return;
      }

      // F1: Documentation
      if (e.key === "F1") {
        e.preventDefault();
        window.open("https://github.com/abdulwahed-sweden/ferox", "_blank");
        return;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

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
        console.error("Failed to load sessions:", error);
        if (mounted) {
          setSessionsError(
            error instanceof Error ? error.message : "Failed to load sessions"
          );
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

    window.addEventListener("click", handleClick);
    return () => window.removeEventListener("click", handleClick);
  }, [contextMenu.visible, hideContextMenu]);

  return (
    <div className="h-screen flex flex-col bg-[var(--bg-base)] text-[var(--text-primary)]">
      {/* Header / Menu Bar */}
      <header
        className="h-10 flex items-center px-4 gap-4 select-none"
        style={{
          backgroundColor: "var(--surface-primary)",
          borderBottom: "1px solid var(--border-primary)",
        }}
      >
        <Logo variant="wordmark" size="md" color="auto" />
        <MenuBar
          onNewSession={() => setShowNewSession(true)}
          onSettings={() => setShowSettings(true)}
          onAbout={() => setShowAbout(true)}
          onShortcuts={() => setShowShortcuts(true)}
          sidebarVisible={sidebarVisible}
          onToggleSidebar={() => setSidebarVisible((prev) => !prev)}
        />
        <div className="flex-1" />
        <div className="text-xs text-[var(--text-muted)]">v4.0.0</div>
      </header>

      {/* Main Content */}
      <div
        className={clsx("flex-1 flex min-h-0", isResizing && "select-none")}
      >
        {/* Sidebar - Session Tree */}
        {sidebarVisible && (
          <aside
            className="flex flex-col relative"
            style={{
              width: sidebarWidth,
              backgroundColor: "var(--surface-primary)",
            }}
          >
            <div className="p-3 border-b border-[var(--border-primary)]">
              <div className="flex items-center justify-between mb-2">
                <h2 className="text-sm font-semibold text-[var(--text-primary)]">
                  Sessions
                </h2>
                <button
                  onClick={() => setShowNewSession(true)}
                  className="p-1 rounded hover:bg-[var(--bg-hover)] text-[var(--text-secondary)] hover:text-[var(--color-primary)] transition-colors"
                  title="New Session (Cmd+N)"
                >
                  <Plus size={16} />
                </button>
              </div>
              {/* Search Input */}
              <div className="relative">
                <Search
                  size={14}
                  className="absolute left-2 top-1/2 -translate-y-1/2 text-[var(--text-muted)]"
                />
                <input
                  ref={searchInputRef}
                  type="text"
                  placeholder="Search... (/)"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full pl-7 pr-7 py-1.5 text-xs bg-[var(--surface-secondary)] border border-[var(--border-primary)] rounded text-[var(--text-primary)] placeholder:text-[var(--text-muted)] focus:outline-none focus:border-[var(--color-primary)]/50"
                />
                {searchQuery && (
                  <button
                    onClick={() => setSearchQuery("")}
                    className="absolute right-2 top-1/2 -translate-y-1/2 text-[var(--text-muted)] hover:text-[var(--text-primary)]"
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

            {/* Empty state with demo session button */}
            {sessions.length === 0 && (
              <div className="p-4 text-center">
                <p className="text-sm text-[var(--text-muted)] mb-3">
                  No sessions yet
                </p>
                <button
                  onClick={() => setShowNewSession(true)}
                  className="px-3 py-1.5 rounded bg-[var(--color-primary)] text-white text-xs hover:bg-[var(--color-primary)]/90 transition-colors"
                >
                  Create Demo Session
                </button>
              </div>
            )}

            {/* Resize handle */}
            <div
              onMouseDown={handleMouseDown}
              className={clsx(
                "absolute right-0 top-0 bottom-0 w-1 cursor-col-resize",
                "hover:bg-[var(--color-primary)]/30 transition-colors",
                isResizing && "bg-[var(--color-primary)]/50"
              )}
            />
          </aside>
        )}

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
              <div className="h-full flex flex-col items-center justify-center text-[var(--text-muted)]">
                <div className="mb-4 opacity-20">
                  <Logo variant="icon" size="xl" color="auto" />
                </div>
                <p className="text-lg">No session selected</p>
                <p className="text-sm mt-2">
                  Double-click a session to open a terminal
                </p>
                <button
                  onClick={() => setShowNewSession(true)}
                  className="mt-4 px-4 py-2 rounded bg-[var(--color-primary)] text-white text-sm hover:bg-[var(--color-primary)]/90 transition-colors"
                >
                  Create Demo Session
                </button>
              </div>
            )}
          </div>
        </main>
      </div>

      {/* Status Bar */}
      <StatusBar />

      {/* Context Menu */}
      {contextMenu.visible && <ContextMenu />}

      {/* Modals */}
      <AboutModal isOpen={showAbout} onClose={() => setShowAbout(false)} />
      <ShortcutsModal
        isOpen={showShortcuts}
        onClose={() => setShowShortcuts(false)}
      />
      <SettingsModal
        isOpen={showSettings}
        onClose={() => setShowSettings(false)}
      />
      <NewSessionModal
        isOpen={showNewSession}
        onClose={() => setShowNewSession(false)}
      />

      {/* Toast notifications */}
      <Toaster
        position="top-right"
        toastOptions={{
          style: {
            background: "var(--toast-bg)",
            color: "var(--toast-text)",
            border: "1px solid var(--toast-border)",
          },
          success: {
            iconTheme: {
              primary: "var(--toast-success-primary)",
              secondary: "var(--toast-success-secondary)",
            },
          },
          error: {
            iconTheme: {
              primary: "var(--toast-error-primary)",
              secondary: "var(--toast-error-secondary)",
            },
          },
        }}
      />
    </div>
  );
}

export default App;
