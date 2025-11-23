import { describe, it, expect, beforeEach } from 'vitest';
import { useAppStore } from './index';
import { act } from '@testing-library/react';
import type { Session, Tab } from '../types';

const mockSession: Session = {
  id: 'session-1',
  hostname: 'test-host',
  ip_address: '192.168.1.1',
  username: 'testuser',
  os: 'linux',
  os_version: '5.15',
  architecture: 'x64',
  privileges: 'user',
  status: 'active',
  established_at: '2024-01-01T00:00:00Z',
  last_seen: '2024-01-01T00:00:00Z',
  parent_id: null,
  intelligence: {
    domain: null,
    is_domain_joined: false,
    detected_av: [],
    stealth_mode: 'normal',
    network_segment: null,
  },
  metrics: {
    credentials_count: 0,
    commands_executed: 0,
    files_transferred: 0,
    persistence_methods: 0,
  },
  tags: [],
  note: null,
};

const mockTab: Tab = {
  id: 'tab-1',
  type: 'terminal',
  sessionId: 'session-1',
  title: 'Test Tab',
  icon: 'terminal',
};

describe('useAppStore', () => {
  beforeEach(() => {
    // Reset store state before each test
    act(() => {
      useAppStore.setState({
        sessions: [],
        sessionTree: [],
        selectedSessionId: null,
        sessionsLoading: true,
        sessionsError: null,
        tabs: [],
        activeTabId: null,
        sidebarWidth: 280,
        searchQuery: '',
        searchInputFocused: false,
        sessionFilters: { status: [], os: [] },
        contextMenu: { visible: false, x: 0, y: 0, sessionId: null },
      });
    });
  });

  describe('Sessions', () => {
    it('should add a session', () => {
      const { addSession } = useAppStore.getState();

      act(() => {
        addSession(mockSession);
      });

      expect(useAppStore.getState().sessions).toHaveLength(1);
      expect(useAppStore.getState().sessions[0]).toEqual(mockSession);
    });

    it('should update a session', () => {
      const { addSession, updateSession } = useAppStore.getState();

      act(() => {
        addSession(mockSession);
        updateSession('session-1', { hostname: 'updated-host' });
      });

      expect(useAppStore.getState().sessions[0].hostname).toBe('updated-host');
    });

    it('should remove a session', () => {
      const { addSession, removeSession } = useAppStore.getState();

      act(() => {
        addSession(mockSession);
        removeSession('session-1');
      });

      expect(useAppStore.getState().sessions).toHaveLength(0);
    });

    it('should select a session', () => {
      const { selectSession } = useAppStore.getState();

      act(() => {
        selectSession('session-1');
      });

      expect(useAppStore.getState().selectedSessionId).toBe('session-1');
    });

    it('should clear selected session when removed', () => {
      const { addSession, selectSession, removeSession } = useAppStore.getState();

      act(() => {
        addSession(mockSession);
        selectSession('session-1');
        removeSession('session-1');
      });

      expect(useAppStore.getState().selectedSessionId).toBeNull();
    });
  });

  describe('Tabs', () => {
    it('should add a tab and set it as active', () => {
      const { addTab } = useAppStore.getState();

      act(() => {
        addTab(mockTab);
      });

      expect(useAppStore.getState().tabs).toHaveLength(1);
      expect(useAppStore.getState().activeTabId).toBe('tab-1');
    });

    it('should close a tab', () => {
      const { addTab, closeTab } = useAppStore.getState();

      act(() => {
        addTab(mockTab);
        closeTab('tab-1');
      });

      expect(useAppStore.getState().tabs).toHaveLength(0);
      expect(useAppStore.getState().activeTabId).toBeNull();
    });

    it('should update a tab', () => {
      const { addTab, updateTab } = useAppStore.getState();

      act(() => {
        addTab(mockTab);
        updateTab('tab-1', { title: 'Updated Title' });
      });

      expect(useAppStore.getState().tabs[0].title).toBe('Updated Title');
    });

    it('should reorder tabs', () => {
      const { addTab, reorderTabs } = useAppStore.getState();

      act(() => {
        addTab({ ...mockTab, id: 'tab-1', title: 'Tab 1' });
        addTab({ ...mockTab, id: 'tab-2', title: 'Tab 2' });
        addTab({ ...mockTab, id: 'tab-3', title: 'Tab 3' });
        reorderTabs(0, 2);
      });

      const tabs = useAppStore.getState().tabs;
      expect(tabs[0].id).toBe('tab-2');
      expect(tabs[1].id).toBe('tab-3');
      expect(tabs[2].id).toBe('tab-1');
    });

    it('should cycle tabs forward', () => {
      const { addTab, cycleTabForward } = useAppStore.getState();

      act(() => {
        addTab({ ...mockTab, id: 'tab-1' });
        addTab({ ...mockTab, id: 'tab-2' });
        useAppStore.setState({ activeTabId: 'tab-1' });
        cycleTabForward();
      });

      expect(useAppStore.getState().activeTabId).toBe('tab-2');
    });

    it('should cycle tabs backward', () => {
      const { addTab, cycleTabBackward } = useAppStore.getState();

      act(() => {
        addTab({ ...mockTab, id: 'tab-1' });
        addTab({ ...mockTab, id: 'tab-2' });
        useAppStore.setState({ activeTabId: 'tab-2' });
        cycleTabBackward();
      });

      expect(useAppStore.getState().activeTabId).toBe('tab-1');
    });
  });

  describe('Session Filters', () => {
    it('should set status filter', () => {
      const { setStatusFilter } = useAppStore.getState();

      act(() => {
        setStatusFilter(['active', 'sleeping']);
      });

      expect(useAppStore.getState().sessionFilters.status).toEqual(['active', 'sleeping']);
    });

    it('should set OS filter', () => {
      const { setOsFilter } = useAppStore.getState();

      act(() => {
        setOsFilter(['linux', 'windows']);
      });

      expect(useAppStore.getState().sessionFilters.os).toEqual(['linux', 'windows']);
    });

    it('should clear all filters', () => {
      const { setStatusFilter, setOsFilter, clearFilters } = useAppStore.getState();

      act(() => {
        setStatusFilter(['active']);
        setOsFilter(['linux']);
        clearFilters();
      });

      expect(useAppStore.getState().sessionFilters.status).toEqual([]);
      expect(useAppStore.getState().sessionFilters.os).toEqual([]);
    });
  });

  describe('UI State', () => {
    it('should update sidebar width', () => {
      const { setSidebarWidth } = useAppStore.getState();

      act(() => {
        setSidebarWidth(350);
      });

      expect(useAppStore.getState().sidebarWidth).toBe(350);
    });

    it('should update search query', () => {
      const { setSearchQuery } = useAppStore.getState();

      act(() => {
        setSearchQuery('test query');
      });

      expect(useAppStore.getState().searchQuery).toBe('test query');
    });

    it('should show and hide context menu', () => {
      const { showContextMenu, hideContextMenu } = useAppStore.getState();

      act(() => {
        showContextMenu(100, 200, 'session-1');
      });

      expect(useAppStore.getState().contextMenu).toEqual({
        visible: true,
        x: 100,
        y: 200,
        sessionId: 'session-1',
      });

      act(() => {
        hideContextMenu();
      });

      expect(useAppStore.getState().contextMenu.visible).toBe(false);
    });
  });
});
