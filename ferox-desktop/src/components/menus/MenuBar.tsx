import { useState, useRef, useEffect, useCallback } from "react";
import { useAppStore } from "../../store";
import { useTheme } from "../../hooks/useTheme";
import {
  FilePlus,
  FolderOpen,
  Save,
  Download,
  Settings,
  LogOut,
  Plus,
  Globe,
  Wifi,
  Unplug,
  Trash2,
  Info,
  PanelLeft,
  Sun,
  Moon,
  ZoomIn,
  ZoomOut,
  RotateCcw,
  Maximize,
  HelpCircle,
  Keyboard,
  RefreshCw,
  ChevronDown,
  Package,
  Radar,
  KeyRound,
  FileText,
  Clock,
  StickyNote,
  Crosshair,
  Grid3X3,
  ClipboardList,
  Eye,
  Shield,
} from "lucide-react";
import { clsx } from "clsx";
import { motion, AnimatePresence } from "framer-motion";
import toast from "react-hot-toast";

interface MenuBarProps {
  onNewSession: () => void;
  onSettings: () => void;
  onAbout: () => void;
  onShortcuts: () => void;
  sidebarVisible: boolean;
  onToggleSidebar: () => void;
}

interface MenuItem {
  id: string;
  label: string;
  icon?: React.ReactNode;
  shortcut?: string;
  disabled?: boolean;
  danger?: boolean;
  onClick?: () => void;
}

type MenuId = "file" | "session" | "tools" | "view" | "help" | null;

export function MenuBar({
  onNewSession,
  onSettings,
  onAbout,
  onShortcuts,
  sidebarVisible,
  onToggleSidebar,
}: MenuBarProps) {
  const [openMenu, setOpenMenu] = useState<MenuId>(null);
  const menuBarRef = useRef<HTMLDivElement>(null);
  const { theme, toggleTheme } = useTheme();
  const { selectedSessionId, sessions, tabs, addTab, removeSession } =
    useAppStore();

  // Close menu on outside click
  useEffect(() => {
    const handleClick = (e: MouseEvent) => {
      if (
        menuBarRef.current &&
        !menuBarRef.current.contains(e.target as Node)
      ) {
        setOpenMenu(null);
      }
    };

    document.addEventListener("click", handleClick);
    return () => document.removeEventListener("click", handleClick);
  }, []);

  // Close menu on escape
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        setOpenMenu(null);
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, []);

  const toggleMenu = (menu: MenuId) => {
    setOpenMenu((prev) => (prev === menu ? null : menu));
  };

  // Tool tab opener
  const openToolTab = useCallback(
    (
      type:
        | "payloads"
        | "scanner"
        | "credentials"
        | "eventlog"
        | "scheduler"
        | "notes"
        | "postexploitation"
        | "networkmap"
        | "mitre"
        | "reports"
        | "opsec"
        | "workflow",
      title: string,
      icon: string
    ) => {
      const existing = tabs.find((t) => t.type === type);
      if (existing) {
        useAppStore.getState().setActiveTab(existing.id);
        setOpenMenu(null);
        return;
      }

      addTab({
        id: `${type}-${Date.now()}`,
        type,
        sessionId: "",
        title,
        icon,
      });
      setOpenMenu(null);
    },
    [tabs, addTab]
  );

  // File menu items
  const fileMenuItems: (MenuItem | { separator: true })[] = [
    {
      id: "new-session",
      label: "New Session",
      icon: <FilePlus size={14} />,
      shortcut: "Cmd+N",
      onClick: onNewSession,
    },
    { separator: true },
    {
      id: "open-project",
      label: "Open Project...",
      icon: <FolderOpen size={14} />,
      shortcut: "Cmd+O",
      onClick: () => toast("Open project dialog (not implemented)"),
    },
    {
      id: "save-project",
      label: "Save Project",
      icon: <Save size={14} />,
      shortcut: "Cmd+S",
      onClick: () => toast.success("Project saved"),
    },
    { separator: true },
    {
      id: "export-results",
      label: "Export Results...",
      icon: <Download size={14} />,
      shortcut: "Cmd+E",
      onClick: () => toast("Export dialog (not implemented)"),
    },
    { separator: true },
    {
      id: "settings",
      label: "Settings...",
      icon: <Settings size={14} />,
      shortcut: "Cmd+,",
      onClick: onSettings,
    },
    { separator: true },
    {
      id: "quit",
      label: "Quit Ferox",
      icon: <LogOut size={14} />,
      shortcut: "Cmd+Q",
      danger: true,
      onClick: () => {
        if (confirm("Are you sure you want to quit?")) {
          window.close();
        }
      },
    },
  ];

  // Session menu items
  const selectedSession = sessions.find((s) => s.id === selectedSessionId);
  const sessionMenuItems: (MenuItem | { separator: true })[] = [
    {
      id: "new-session-2",
      label: "New Session",
      icon: <Plus size={14} />,
      shortcut: "Cmd+N",
      onClick: onNewSession,
    },
    {
      id: "connect-target",
      label: "Connect to Target...",
      icon: <Globe size={14} />,
      onClick: onNewSession,
    },
    {
      id: "start-listener",
      label: "Start Listener...",
      icon: <Wifi size={14} />,
      onClick: () => toast("Listener dialog (not implemented)"),
    },
    { separator: true },
    {
      id: "disconnect",
      label: "Disconnect",
      icon: <Unplug size={14} />,
      disabled: !selectedSession || selectedSession.status !== "active",
      onClick: () => {
        if (selectedSessionId) {
          useAppStore.getState().updateSession(selectedSessionId, {
            status: "dead",
          });
          toast.success("Session disconnected");
        }
      },
    },
    {
      id: "kill-session",
      label: "Kill Session",
      icon: <Trash2 size={14} />,
      danger: true,
      disabled: !selectedSession,
      onClick: () => {
        if (selectedSessionId && confirm("Kill this session?")) {
          removeSession(selectedSessionId);
          toast.success("Session killed");
        }
      },
    },
    { separator: true },
    {
      id: "session-properties",
      label: "Session Properties",
      icon: <Info size={14} />,
      disabled: !selectedSession,
      onClick: () => toast("Session properties (not implemented)"),
    },
  ];

  // Tools menu items (consolidated)
  const toolsMenuItems: (MenuItem | { separator: true })[] = [
    {
      id: "workflow",
      label: "Assessment Wizard",
      icon: <Shield size={14} className="text-[var(--color-primary)]" />,
      onClick: () => openToolTab("workflow", "Assessment Wizard", "shield"),
    },
    { separator: true },
    {
      id: "payloads",
      label: "Payload Builder",
      icon: <Package size={14} className="text-purple-400" />,
      onClick: () => openToolTab("payloads", "Payload Builder", "package"),
    },
    {
      id: "scanner",
      label: "Network Scanner",
      icon: <Radar size={14} className="text-blue-400" />,
      onClick: () => openToolTab("scanner", "Network Scanner", "radar"),
    },
    {
      id: "credentials",
      label: "Credentials Viewer",
      icon: <KeyRound size={14} className="text-yellow-400" />,
      onClick: () =>
        openToolTab("credentials", "Credentials Viewer", "key-round"),
    },
    { separator: true },
    {
      id: "eventlog",
      label: "Event Log",
      icon: <FileText size={14} className="text-cyan-400" />,
      onClick: () => openToolTab("eventlog", "Event Log", "file-text"),
    },
    {
      id: "scheduler",
      label: "Task Scheduler",
      icon: <Clock size={14} className="text-orange-400" />,
      onClick: () => openToolTab("scheduler", "Task Scheduler", "clock"),
    },
    {
      id: "notes",
      label: "Notes",
      icon: <StickyNote size={14} className="text-pink-400" />,
      onClick: () => openToolTab("notes", "Notes", "sticky-note"),
    },
    { separator: true },
    {
      id: "postexploitation",
      label: "Post-Exploitation",
      icon: <Crosshair size={14} className="text-red-400" />,
      onClick: () =>
        openToolTab("postexploitation", "Post-Exploitation", "crosshair"),
    },
    { separator: true },
    {
      id: "networkmap",
      label: "Network Map",
      icon: <Globe size={14} className="text-cyan-400" />,
      onClick: () => openToolTab("networkmap", "Network Map", "globe"),
    },
    {
      id: "mitre",
      label: "MITRE ATT&CK",
      icon: <Grid3X3 size={14} className="text-purple-400" />,
      onClick: () => openToolTab("mitre", "MITRE ATT&CK", "grid"),
    },
    {
      id: "reports",
      label: "Reports",
      icon: <ClipboardList size={14} className="text-emerald-400" />,
      onClick: () => openToolTab("reports", "Reports", "clipboard-list"),
    },
    { separator: true },
    {
      id: "opsec",
      label: "OPSEC Dashboard",
      icon: <Eye size={14} className="text-cyan-400" />,
      onClick: () => openToolTab("opsec", "OPSEC Dashboard", "eye"),
    },
  ];

  // View menu items
  const viewMenuItems: (MenuItem | { separator: true })[] = [
    {
      id: "toggle-sidebar",
      label: sidebarVisible ? "Hide Sidebar" : "Show Sidebar",
      icon: <PanelLeft size={14} />,
      shortcut: "Cmd+B",
      onClick: onToggleSidebar,
    },
    { separator: true },
    {
      id: "toggle-theme",
      label: theme === "dark" ? "Light Mode" : "Dark Mode",
      icon: theme === "dark" ? <Sun size={14} /> : <Moon size={14} />,
      shortcut: "Cmd+Shift+T",
      onClick: toggleTheme,
    },
    { separator: true },
    {
      id: "zoom-in",
      label: "Zoom In",
      icon: <ZoomIn size={14} />,
      shortcut: "Cmd++",
      onClick: () => toast("Zoom in (not implemented)"),
    },
    {
      id: "zoom-out",
      label: "Zoom Out",
      icon: <ZoomOut size={14} />,
      shortcut: "Cmd+-",
      onClick: () => toast("Zoom out (not implemented)"),
    },
    {
      id: "zoom-reset",
      label: "Reset Zoom",
      icon: <RotateCcw size={14} />,
      shortcut: "Cmd+0",
      onClick: () => toast("Zoom reset (not implemented)"),
    },
    { separator: true },
    {
      id: "fullscreen",
      label: "Toggle Fullscreen",
      icon: <Maximize size={14} />,
      shortcut: "Cmd+Ctrl+F",
      onClick: () => {
        if (document.fullscreenElement) {
          document.exitFullscreen();
        } else {
          document.documentElement.requestFullscreen();
        }
      },
    },
  ];

  // Help menu items
  const helpMenuItems: (MenuItem | { separator: true })[] = [
    {
      id: "documentation",
      label: "Documentation",
      icon: <HelpCircle size={14} />,
      shortcut: "F1",
      onClick: () =>
        window.open("https://github.com/abdulwahed-sweden/ferox", "_blank"),
    },
    {
      id: "shortcuts",
      label: "Keyboard Shortcuts",
      icon: <Keyboard size={14} />,
      shortcut: "Cmd+Shift+/",
      onClick: onShortcuts,
    },
    { separator: true },
    {
      id: "check-updates",
      label: "Check for Updates",
      icon: <RefreshCw size={14} />,
      onClick: () => toast.success("You're running the latest version (4.0.0)"),
    },
    { separator: true },
    {
      id: "about",
      label: "About Ferox",
      icon: <Info size={14} />,
      onClick: onAbout,
    },
  ];

  const renderMenu = (
    id: MenuId,
    label: string,
    items: (MenuItem | { separator: true })[]
  ) => (
    <div className="relative">
      <button
        onClick={(e) => {
          e.stopPropagation();
          toggleMenu(id);
        }}
        className={clsx(
          "px-3 py-1 rounded text-sm transition-colors flex items-center gap-1",
          openMenu === id
            ? "bg-[var(--bg-hover)] text-[var(--text-primary)]"
            : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text-primary)]"
        )}
      >
        {label}
        {id === "tools" && (
          <ChevronDown
            size={12}
            className={clsx(
              "transition-transform",
              openMenu === id && "rotate-180"
            )}
          />
        )}
      </button>

      <AnimatePresence>
        {openMenu === id && (
          <motion.div
            initial={{ opacity: 0, y: -4 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -4 }}
            transition={{ duration: 0.1 }}
            className="absolute top-full left-0 mt-1 rounded-lg z-50"
            style={{
              minWidth: "220px",
              maxWidth: "320px",
              padding: "4px",
              backgroundColor: "var(--surface-primary)",
              border: "1px solid var(--border-primary)",
              boxShadow: "0 10px 25px -5px rgba(0, 0, 0, 0.3), 0 8px 10px -6px rgba(0, 0, 0, 0.2)",
            }}
          >
            {items.map((item, index) =>
              "separator" in item ? (
                <div
                  key={`sep-${index}`}
                  className="h-px my-1"
                  style={{ backgroundColor: "var(--border-primary)" }}
                />
              ) : (
                <button
                  key={item.id}
                  onClick={() => {
                    if (!item.disabled && item.onClick) {
                      item.onClick();
                      setOpenMenu(null);
                    }
                  }}
                  disabled={item.disabled}
                  className={clsx(
                    "w-full flex items-center px-3 py-2 rounded transition-colors",
                    item.disabled
                      ? "cursor-not-allowed"
                      : item.danger
                        ? "hover:bg-red-500/10"
                        : "hover:bg-[var(--bg-hover)]"
                  )}
                  style={{
                    fontSize: "13px",
                    lineHeight: "1.4",
                    whiteSpace: "nowrap",
                    color: item.disabled
                      ? "var(--text-muted)"
                      : item.danger
                        ? "#f87171"
                        : "var(--text-secondary)",
                  }}
                >
                  <span className="flex items-center gap-2 flex-shrink-0">
                    {item.icon}
                    <span>{item.label}</span>
                  </span>
                  {item.shortcut && (
                    <span
                      className="ml-auto pl-4 flex-shrink-0"
                      style={{
                        fontSize: "11px",
                        color: "var(--text-muted)",
                        opacity: 0.7,
                      }}
                    >
                      {item.shortcut}
                    </span>
                  )}
                </button>
              )
            )}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );

  return (
    <nav ref={menuBarRef} className="flex items-center gap-1">
      {renderMenu("file", "File", fileMenuItems)}
      {renderMenu("session", "Session", sessionMenuItems)}
      {renderMenu("tools", "Tools", toolsMenuItems)}
      {renderMenu("view", "View", viewMenuItems)}
      {renderMenu("help", "Help", helpMenuItems)}
    </nav>
  );
}
