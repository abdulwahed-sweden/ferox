/**
 * MenuBar - Application menu bar with dropdown menus
 * Provides File, Session, Tools, View, and Help menus
 */

import { useState, useRef, useEffect } from 'react';
import { clsx } from 'clsx';
import toast from 'react-hot-toast';
import {
  Shield,
  ChevronDown,
  // File menu
  FolderPlus,
  FolderOpen,
  Save,
  FileJson,
  FileText,
  FileCode,
  Settings,
  LogOut,
  // Session menu
  Plus,
  List,
  XCircle,
  Trash2,
  Info,
  // Tools menu
  Package,
  Radar,
  KeyRound,
  Clock,
  StickyNote,
  Crosshair,
  Globe,
  Grid3X3,
  ClipboardList,
  ShieldAlert,
  // View menu
  PanelLeft,
  Moon,
  Sun,
  ZoomIn,
  ZoomOut,
  LayoutGrid,
  Maximize,
  // Help menu
  Book,
  Keyboard,
  Download,
  HelpCircle,
  Bug,
} from 'lucide-react';
import { useAppStore } from '../store';
import type { TabType } from '../types';

interface MenuItemProps {
  icon?: React.ReactNode;
  label: string;
  shortcut?: string;
  onClick?: () => void;
  disabled?: boolean;
  danger?: boolean;
}

function MenuItem({ icon, label, shortcut, onClick, disabled, danger }: MenuItemProps) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={clsx(
        'w-full px-3 py-1.5 text-left text-sm whitespace-nowrap flex items-center gap-2 transition-colors',
        disabled
          ? 'text-text-muted cursor-not-allowed'
          : danger
          ? 'hover:bg-red-500/20 text-red-400 hover:text-red-300'
          : 'hover:bg-dark-700 text-text-secondary hover:text-text-primary'
      )}
    >
      {icon && <span className="w-3.5 h-3.5 flex items-center justify-center">{icon}</span>}
      <span className="flex-1">{label}</span>
      {shortcut && <span className="text-xs text-text-muted">{shortcut}</span>}
    </button>
  );
}

function MenuDivider() {
  return <div className="h-px bg-dark-600 my-1" />;
}

interface DropdownMenuProps {
  label: string;
  children: React.ReactNode;
  isOpen: boolean;
  onToggle: () => void;
  onClose: () => void;
}

function DropdownMenu({ label, children, isOpen, onToggle, onClose }: DropdownMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        onClose();
      }
    };

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, [isOpen, onClose]);

  return (
    <div className="relative" ref={menuRef}>
      <button
        onClick={(e) => {
          e.stopPropagation();
          onToggle();
        }}
        className={clsx(
          'px-3 py-1 rounded text-text-secondary hover:text-text-primary transition-colors flex items-center gap-1',
          isOpen && 'bg-dark-600 text-text-primary'
        )}
      >
        {label}
        <ChevronDown size={12} className={clsx('transition-transform', isOpen && 'rotate-180')} />
      </button>
      {isOpen && (
        <div className="absolute top-full left-0 mt-1 bg-dark-800 border border-dark-600 rounded-lg shadow-xl py-1 min-w-52 z-50">
          {children}
        </div>
      )}
    </div>
  );
}

export function MenuBar() {
  const [openMenu, setOpenMenu] = useState<string | null>(null);
  const [isDarkTheme, setIsDarkTheme] = useState(true);
  const { addTab, tabs, sessions, selectedSessionId } = useAppStore();

  const closeMenu = () => setOpenMenu(null);
  const toggleMenu = (menu: string) => {
    setOpenMenu(openMenu === menu ? null : menu);
  };

  // Generic tab opener
  const openToolTab = (type: TabType, title: string, icon: string) => {
    const existing = tabs.find(t => t.type === type);
    if (existing) {
      useAppStore.getState().setActiveTab(existing.id);
      closeMenu();
      return;
    }

    addTab({
      id: `${type}-${Date.now()}`,
      type,
      sessionId: '',
      title,
      icon,
    });
    closeMenu();
  };

  // File menu actions
  const handleNewProject = () => {
    toast.success('New project created');
    closeMenu();
  };

  const handleOpenProject = () => {
    toast('Open project dialog...', { icon: '📂' });
    closeMenu();
  };

  const handleSaveSession = () => {
    if (!selectedSessionId) {
      toast.error('No session selected');
      return;
    }
    toast.success('Session saved');
    closeMenu();
  };

  const handleExport = (format: string) => {
    toast.loading(`Exporting to ${format}...`, { id: 'export' });
    setTimeout(() => {
      toast.success(`Exported to report.${format.toLowerCase()}`, { id: 'export' });
    }, 1000);
    closeMenu();
  };

  const handleSettings = () => {
    toast('Settings panel opening...', { icon: '⚙️' });
    closeMenu();
  };

  const handleExit = () => {
    toast('Closing application...', { icon: '👋' });
    closeMenu();
    // In a real app, would call: window.close() or Tauri's exit
  };

  // Session menu actions
  const handleNewSession = () => {
    toast('Creating new session...', { icon: '🔗' });
    closeMenu();
  };

  const handleListSessions = () => {
    toast(`${sessions.length} active sessions`, { icon: '📋' });
    closeMenu();
  };

  const handleKillSession = () => {
    if (!selectedSessionId) {
      toast.error('No session selected');
      return;
    }
    toast.success('Session terminated');
    closeMenu();
  };

  const handleKillAllSessions = () => {
    if (sessions.length === 0) {
      toast.error('No active sessions');
      return;
    }
    toast.success(`Terminated ${sessions.length} sessions`);
    closeMenu();
  };

  const handleSessionInfo = () => {
    if (!selectedSessionId) {
      toast.error('No session selected');
      return;
    }
    const session = sessions.find(s => s.id === selectedSessionId);
    if (session) {
      toast(`Session: ${session.hostname} (${session.ip_address})`, { icon: 'ℹ️' });
    }
    closeMenu();
  };

  // View menu actions
  const handleToggleSidebar = () => {
    const current = useAppStore.getState().sidebarWidth;
    useAppStore.getState().setSidebarWidth(current > 0 ? 0 : 280);
    toast.success(current > 0 ? 'Sidebar hidden' : 'Sidebar shown');
    closeMenu();
  };

  const handleToggleTheme = () => {
    setIsDarkTheme(!isDarkTheme);
    toast.success(isDarkTheme ? 'Light theme enabled' : 'Dark theme enabled');
    closeMenu();
  };

  const handleZoomIn = () => {
    toast('Zoom in', { icon: '🔍' });
    closeMenu();
  };

  const handleZoomOut = () => {
    toast('Zoom out', { icon: '🔍' });
    closeMenu();
  };

  const handleResetLayout = () => {
    useAppStore.getState().setSidebarWidth(280);
    toast.success('Layout reset');
    closeMenu();
  };

  const handleFullScreen = () => {
    if (document.fullscreenElement) {
      document.exitFullscreen();
      toast.success('Exited fullscreen');
    } else {
      document.documentElement.requestFullscreen();
      toast.success('Entered fullscreen');
    }
    closeMenu();
  };

  // Help menu actions
  const handleDocumentation = () => {
    window.open('https://github.com/ferox-c2/docs', '_blank');
    closeMenu();
  };

  const handleKeyboardShortcuts = () => {
    toast(
      <div className="text-sm">
        <div className="font-semibold mb-2">Keyboard Shortcuts</div>
        <div className="space-y-1 text-xs">
          <div><kbd className="bg-dark-600 px-1 rounded">Ctrl+N</kbd> New tab</div>
          <div><kbd className="bg-dark-600 px-1 rounded">Ctrl+W</kbd> Close tab</div>
          <div><kbd className="bg-dark-600 px-1 rounded">/</kbd> Search sessions</div>
          <div><kbd className="bg-dark-600 px-1 rounded">Ctrl+K</kbd> Command palette</div>
        </div>
      </div>,
      { duration: 5000 }
    );
    closeMenu();
  };

  const handleCheckUpdates = () => {
    toast.loading('Checking for updates...', { id: 'updates' });
    setTimeout(() => {
      toast.success('You are on the latest version', { id: 'updates' });
    }, 1500);
    closeMenu();
  };

  const handleAbout = () => {
    toast(
      <div className="text-sm">
        <div className="flex items-center gap-2 font-semibold mb-2">
          <Shield size={16} className="text-ferox-green" />
          Ferox C2 Framework
        </div>
        <div className="text-xs text-text-muted">
          Version 1.0.0<br />
          Built with Tauri + React
        </div>
      </div>,
      { duration: 4000 }
    );
    closeMenu();
  };

  const handleReportBug = () => {
    window.open('https://github.com/ferox-c2/issues', '_blank');
    closeMenu();
  };

  return (
    <header className="h-10 bg-dark-800 border-b border-dark-600 flex items-center px-4 gap-4 select-none">
      {/* Logo */}
      <div className="flex items-center gap-2 text-ferox-green">
        <Shield size={18} />
        <span className="font-semibold text-sm">Ferox C2</span>
      </div>

      {/* Menu Items */}
      <nav className="flex items-center gap-1 text-sm">
        {/* File Menu */}
        <DropdownMenu
          label="File"
          isOpen={openMenu === 'file'}
          onToggle={() => toggleMenu('file')}
          onClose={closeMenu}
        >
          <MenuItem
            icon={<FolderPlus size={14} className="text-green-400" />}
            label="New Project"
            shortcut="Ctrl+Shift+N"
            onClick={handleNewProject}
          />
          <MenuItem
            icon={<FolderOpen size={14} className="text-blue-400" />}
            label="Open Project"
            shortcut="Ctrl+O"
            onClick={handleOpenProject}
          />
          <MenuDivider />
          <MenuItem
            icon={<Save size={14} className="text-cyan-400" />}
            label="Save Session"
            shortcut="Ctrl+S"
            onClick={handleSaveSession}
          />
          <MenuDivider />
          <div className="px-3 py-1 text-xs text-text-muted uppercase">Export Results</div>
          <MenuItem
            icon={<FileJson size={14} className="text-yellow-400" />}
            label="Export as JSON"
            onClick={() => handleExport('JSON')}
          />
          <MenuItem
            icon={<FileCode size={14} className="text-orange-400" />}
            label="Export as HTML"
            onClick={() => handleExport('HTML')}
          />
          <MenuItem
            icon={<FileText size={14} className="text-red-400" />}
            label="Export as PDF"
            onClick={() => handleExport('PDF')}
          />
          <MenuDivider />
          <MenuItem
            icon={<Settings size={14} className="text-text-muted" />}
            label="Settings"
            shortcut="Ctrl+,"
            onClick={handleSettings}
          />
          <MenuDivider />
          <MenuItem
            icon={<LogOut size={14} />}
            label="Exit"
            shortcut="Alt+F4"
            onClick={handleExit}
            danger
          />
        </DropdownMenu>

        {/* Session Menu */}
        <DropdownMenu
          label="Session"
          isOpen={openMenu === 'session'}
          onToggle={() => toggleMenu('session')}
          onClose={closeMenu}
        >
          <MenuItem
            icon={<Plus size={14} className="text-green-400" />}
            label="New Session"
            shortcut="Ctrl+N"
            onClick={handleNewSession}
          />
          <MenuItem
            icon={<List size={14} className="text-blue-400" />}
            label="List Sessions"
            onClick={handleListSessions}
          />
          <MenuDivider />
          <MenuItem
            icon={<Info size={14} className="text-cyan-400" />}
            label="Session Info"
            onClick={handleSessionInfo}
            disabled={!selectedSessionId}
          />
          <MenuDivider />
          <MenuItem
            icon={<XCircle size={14} className="text-orange-400" />}
            label="Kill Session"
            onClick={handleKillSession}
            disabled={!selectedSessionId}
          />
          <MenuItem
            icon={<Trash2 size={14} />}
            label="Kill All Sessions"
            onClick={handleKillAllSessions}
            danger
            disabled={sessions.length === 0}
          />
        </DropdownMenu>

        {/* Tools Menu */}
        <DropdownMenu
          label="Tools"
          isOpen={openMenu === 'tools'}
          onToggle={() => toggleMenu('tools')}
          onClose={closeMenu}
        >
          <MenuItem
            icon={<Package size={14} className="text-purple-400" />}
            label="Payload Builder"
            onClick={() => openToolTab('payloads', 'Payload Builder', 'package')}
          />
          <MenuItem
            icon={<Radar size={14} className="text-blue-400" />}
            label="Network Scanner"
            onClick={() => openToolTab('scanner', 'Network Scanner', 'radar')}
          />
          <MenuItem
            icon={<KeyRound size={14} className="text-yellow-400" />}
            label="Credentials Viewer"
            onClick={() => openToolTab('credentials', 'Credentials Viewer', 'key-round')}
          />
          <MenuDivider />
          <MenuItem
            icon={<FileText size={14} className="text-cyan-400" />}
            label="Event Log"
            onClick={() => openToolTab('eventlog', 'Event Log', 'file-text')}
          />
          <MenuItem
            icon={<Clock size={14} className="text-orange-400" />}
            label="Task Scheduler"
            onClick={() => openToolTab('scheduler', 'Task Scheduler', 'clock')}
          />
          <MenuItem
            icon={<StickyNote size={14} className="text-pink-400" />}
            label="Notes"
            onClick={() => openToolTab('notes', 'Notes', 'sticky-note')}
          />
          <MenuDivider />
          <MenuItem
            icon={<Crosshair size={14} className="text-red-400" />}
            label="Post-Exploitation"
            onClick={() => openToolTab('postexploitation', 'Post-Exploitation', 'crosshair')}
          />
          <MenuItem
            icon={<ShieldAlert size={14} className="text-cyan-400" />}
            label="OPSEC Engine"
            onClick={() => openToolTab('opsec', 'OPSEC Engine', 'shield-alert')}
          />
          <MenuDivider />
          <MenuItem
            icon={<Globe size={14} className="text-cyan-400" />}
            label="Network Map"
            onClick={() => openToolTab('networkmap', 'Network Map', 'globe')}
          />
          <MenuItem
            icon={<Grid3X3 size={14} className="text-purple-400" />}
            label="MITRE ATT&CK"
            onClick={() => openToolTab('mitre', 'MITRE ATT&CK', 'grid')}
          />
          <MenuItem
            icon={<ClipboardList size={14} className="text-emerald-400" />}
            label="Reports"
            onClick={() => openToolTab('reports', 'Reports', 'clipboard-list')}
          />
        </DropdownMenu>

        {/* View Menu */}
        <DropdownMenu
          label="View"
          isOpen={openMenu === 'view'}
          onToggle={() => toggleMenu('view')}
          onClose={closeMenu}
        >
          <MenuItem
            icon={<PanelLeft size={14} className="text-blue-400" />}
            label="Toggle Sidebar"
            shortcut="Ctrl+B"
            onClick={handleToggleSidebar}
          />
          <MenuItem
            icon={isDarkTheme ? <Sun size={14} className="text-yellow-400" /> : <Moon size={14} className="text-purple-400" />}
            label={isDarkTheme ? 'Light Theme' : 'Dark Theme'}
            onClick={handleToggleTheme}
          />
          <MenuDivider />
          <MenuItem
            icon={<ZoomIn size={14} className="text-green-400" />}
            label="Zoom In"
            shortcut="Ctrl+="
            onClick={handleZoomIn}
          />
          <MenuItem
            icon={<ZoomOut size={14} className="text-orange-400" />}
            label="Zoom Out"
            shortcut="Ctrl+-"
            onClick={handleZoomOut}
          />
          <MenuDivider />
          <MenuItem
            icon={<LayoutGrid size={14} className="text-cyan-400" />}
            label="Reset Layout"
            onClick={handleResetLayout}
          />
          <MenuItem
            icon={<Maximize size={14} className="text-text-muted" />}
            label="Full Screen"
            shortcut="F11"
            onClick={handleFullScreen}
          />
        </DropdownMenu>

        {/* Help Menu */}
        <DropdownMenu
          label="Help"
          isOpen={openMenu === 'help'}
          onToggle={() => toggleMenu('help')}
          onClose={closeMenu}
        >
          <MenuItem
            icon={<Book size={14} className="text-blue-400" />}
            label="Documentation"
            shortcut="F1"
            onClick={handleDocumentation}
          />
          <MenuItem
            icon={<Keyboard size={14} className="text-cyan-400" />}
            label="Keyboard Shortcuts"
            shortcut="Ctrl+/"
            onClick={handleKeyboardShortcuts}
          />
          <MenuDivider />
          <MenuItem
            icon={<Download size={14} className="text-green-400" />}
            label="Check for Updates"
            onClick={handleCheckUpdates}
          />
          <MenuDivider />
          <MenuItem
            icon={<HelpCircle size={14} className="text-purple-400" />}
            label="About Ferox"
            onClick={handleAbout}
          />
          <MenuItem
            icon={<Bug size={14} className="text-red-400" />}
            label="Report Bug"
            onClick={handleReportBug}
          />
        </DropdownMenu>
      </nav>

      {/* Spacer */}
      <div className="flex-1" />

      {/* Version */}
      <div className="text-xs text-text-muted">v1.0.0</div>
    </header>
  );
}

export default MenuBar;
