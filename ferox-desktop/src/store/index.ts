import { create } from "zustand";
import type { Session, SessionTreeNode, Tab } from "../types";

interface AppState {
  // Sessions
  sessions: Session[];
  sessionTree: SessionTreeNode[];
  selectedSessionId: string | null;
  sessionsLoading: boolean;
  sessionsError: string | null;
  setSessions: (sessions: Session[]) => void;
  setSessionTree: (tree: SessionTreeNode[]) => void;
  selectSession: (id: string | null) => void;
  addSession: (session: Session) => void;
  updateSession: (id: string, updates: Partial<Session>) => void;
  removeSession: (id: string) => void;
  setSessionsLoading: (loading: boolean) => void;
  setSessionsError: (error: string | null) => void;

  // Session Navigation (for keyboard shortcuts)
  selectNextSession: () => void;
  selectPrevSession: () => void;

  // Tabs
  tabs: Tab[];
  activeTabId: string | null;
  addTab: (tab: Tab) => void;
  closeTab: (id: string) => void;
  closeActiveTab: () => void;
  setActiveTab: (id: string) => void;
  updateTab: (id: string, updates: Partial<Tab>) => void;
  reorderTabs: (fromIndex: number, toIndex: number) => void;

  // Tab Navigation (for keyboard shortcuts)
  cycleTabForward: () => void;
  cycleTabBackward: () => void;

  // UI State
  sidebarWidth: number;
  setSidebarWidth: (width: number) => void;
  searchQuery: string;
  setSearchQuery: (query: string) => void;
  searchInputFocused: boolean;
  setSearchInputFocused: (focused: boolean) => void;

  // Session Filters
  sessionFilters: {
    status: ("active" | "sleeping" | "dead")[];
    os: string[];
  };
  setStatusFilter: (statuses: ("active" | "sleeping" | "dead")[]) => void;
  setOsFilter: (os: string[]) => void;
  clearFilters: () => void;

  contextMenu: {
    visible: boolean;
    x: number;
    y: number;
    sessionId: string | null;
  };
  showContextMenu: (x: number, y: number, sessionId: string) => void;
  hideContextMenu: () => void;
}

// Helper to flatten session tree for navigation
const flattenSessionTree = (nodes: SessionTreeNode[]): string[] => {
  const result: string[] = [];
  const traverse = (items: SessionTreeNode[]) => {
    for (const node of items) {
      result.push(node.session.id);
      if (node.children.length > 0) {
        traverse(node.children);
      }
    }
  };
  traverse(nodes);
  return result;
};

export const useAppStore = create<AppState>((set, get) => ({
  // Sessions
  sessions: [],
  sessionTree: [],
  selectedSessionId: null,
  sessionsLoading: true,
  sessionsError: null,
  setSessions: (sessions) => set({ sessions }),
  setSessionTree: (sessionTree) => set({ sessionTree }),
  selectSession: (selectedSessionId) => set({ selectedSessionId }),
  setSessionsLoading: (sessionsLoading) => set({ sessionsLoading }),
  setSessionsError: (sessionsError) => set({ sessionsError }),
  addSession: (session) =>
    set((state) => ({ sessions: [...state.sessions, session] })),
  updateSession: (id, updates) =>
    set((state) => ({
      sessions: state.sessions.map((s) =>
        s.id === id ? { ...s, ...updates } : s,
      ),
    })),
  removeSession: (id) =>
    set((state) => ({
      sessions: state.sessions.filter((s) => s.id !== id),
      selectedSessionId:
        state.selectedSessionId === id ? null : state.selectedSessionId,
    })),

  // Session Navigation
  selectNextSession: () => {
    const { sessionTree, selectedSessionId } = get();
    const flatIds = flattenSessionTree(sessionTree);
    if (flatIds.length === 0) return;

    if (!selectedSessionId) {
      set({ selectedSessionId: flatIds[0] });
      return;
    }

    const currentIndex = flatIds.indexOf(selectedSessionId);
    const nextIndex = (currentIndex + 1) % flatIds.length;
    set({ selectedSessionId: flatIds[nextIndex] });
  },

  selectPrevSession: () => {
    const { sessionTree, selectedSessionId } = get();
    const flatIds = flattenSessionTree(sessionTree);
    if (flatIds.length === 0) return;

    if (!selectedSessionId) {
      set({ selectedSessionId: flatIds[flatIds.length - 1] });
      return;
    }

    const currentIndex = flatIds.indexOf(selectedSessionId);
    const prevIndex = currentIndex <= 0 ? flatIds.length - 1 : currentIndex - 1;
    set({ selectedSessionId: flatIds[prevIndex] });
  },

  // Tabs
  tabs: [],
  activeTabId: null,
  addTab: (tab) =>
    set((state) => ({
      tabs: [...state.tabs, tab],
      activeTabId: tab.id,
    })),
  closeTab: (id) =>
    set((state) => {
      const newTabs = state.tabs.filter((t) => t.id !== id);
      const wasActive = state.activeTabId === id;
      return {
        tabs: newTabs,
        activeTabId: wasActive
          ? (newTabs[newTabs.length - 1]?.id ?? null)
          : state.activeTabId,
      };
    }),
  closeActiveTab: () => {
    const { activeTabId, tabs } = get();
    if (!activeTabId) return;

    const currentIndex = tabs.findIndex((t) => t.id === activeTabId);
    const newTabs = tabs.filter((t) => t.id !== activeTabId);

    // Select previous tab, or next if at start, or null if no tabs left
    let newActiveId: string | null = null;
    if (newTabs.length > 0) {
      newActiveId =
        currentIndex > 0
          ? (newTabs[currentIndex - 1]?.id ?? newTabs[0]?.id)
          : (newTabs[0]?.id ?? null);
    }

    set({ tabs: newTabs, activeTabId: newActiveId });
  },
  setActiveTab: (activeTabId) => set({ activeTabId }),
  updateTab: (id, updates) =>
    set((state) => ({
      tabs: state.tabs.map((t) => (t.id === id ? { ...t, ...updates } : t)),
    })),
  reorderTabs: (fromIndex, toIndex) =>
    set((state) => {
      const newTabs = [...state.tabs];
      const [removed] = newTabs.splice(fromIndex, 1);
      newTabs.splice(toIndex, 0, removed);
      return { tabs: newTabs };
    }),

  // Tab Navigation
  cycleTabForward: () => {
    const { tabs, activeTabId } = get();
    if (tabs.length === 0) return;

    if (!activeTabId) {
      set({ activeTabId: tabs[0].id });
      return;
    }

    const currentIndex = tabs.findIndex((t) => t.id === activeTabId);
    const nextIndex = (currentIndex + 1) % tabs.length;
    set({ activeTabId: tabs[nextIndex].id });
  },

  cycleTabBackward: () => {
    const { tabs, activeTabId } = get();
    if (tabs.length === 0) return;

    if (!activeTabId) {
      set({ activeTabId: tabs[tabs.length - 1].id });
      return;
    }

    const currentIndex = tabs.findIndex((t) => t.id === activeTabId);
    const prevIndex = currentIndex <= 0 ? tabs.length - 1 : currentIndex - 1;
    set({ activeTabId: tabs[prevIndex].id });
  },

  // UI State
  sidebarWidth: 280,
  setSidebarWidth: (sidebarWidth) => set({ sidebarWidth }),
  searchQuery: "",
  setSearchQuery: (searchQuery) => set({ searchQuery }),
  searchInputFocused: false,
  setSearchInputFocused: (searchInputFocused) => set({ searchInputFocused }),

  // Session Filters
  sessionFilters: {
    status: [],
    os: [],
  },
  setStatusFilter: (status) =>
    set((state) => ({
      sessionFilters: { ...state.sessionFilters, status },
    })),
  setOsFilter: (os) =>
    set((state) => ({
      sessionFilters: { ...state.sessionFilters, os },
    })),
  clearFilters: () =>
    set({
      sessionFilters: { status: [], os: [] },
    }),

  contextMenu: {
    visible: false,
    x: 0,
    y: 0,
    sessionId: null,
  },
  showContextMenu: (x, y, sessionId) =>
    set({ contextMenu: { visible: true, x, y, sessionId } }),
  hideContextMenu: () =>
    set((state) => ({
      contextMenu: { ...state.contextMenu, visible: false },
    })),
}));
