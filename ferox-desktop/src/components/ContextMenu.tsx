import { useAppStore } from '../store';
import {
  Terminal,
  FolderOpen,
  Activity,
  Key,
  Network,
  Upload,
  Download,
  Trash2,
  Tag,
  StickyNote,
  Play,
  Zap,
  Lock,
  Globe,
} from 'lucide-react';
import { clsx } from 'clsx';
import toast from 'react-hot-toast';
import { terminateSession } from '../lib/tauri';

interface MenuItemProps {
  icon: typeof Terminal;
  label: string;
  shortcut?: string;
  danger?: boolean;
  disabled?: boolean;
  onClick: () => void;
}

function MenuItem({
  icon: Icon,
  label,
  shortcut,
  danger,
  disabled,
  onClick,
}: MenuItemProps) {
  return (
    <button
      className={clsx(
        'context-menu-item w-full',
        danger && 'text-danger hover:bg-danger/10',
        disabled && 'opacity-50 cursor-not-allowed'
      )}
      onClick={onClick}
      disabled={disabled}
    >
      <Icon size={14} />
      <span className="flex-1 text-left">{label}</span>
      {shortcut && (
        <span className="text-xs text-text-muted ml-4">{shortcut}</span>
      )}
    </button>
  );
}

function MenuDivider() {
  return <div className="context-menu-divider" />;
}

function MenuHeader({ label }: { label: string }) {
  return <div className="context-menu-header">{label}</div>;
}

export function ContextMenu() {
  const {
    contextMenu,
    hideContextMenu,
    sessions,
    addTab,
    removeSession,
  } = useAppStore();

  const session = sessions.find((s) => s.id === contextMenu.sessionId);

  if (!session) return null;

  const handleOpenTerminal = () => {
    addTab({
      id: `tab-${Date.now()}`,
      type: 'terminal',
      sessionId: session.id,
      title: session.hostname,
      icon: 'terminal',
    });
    hideContextMenu();
  };

  const handleOpenFileBrowser = () => {
    addTab({
      id: `tab-${Date.now()}`,
      type: 'filebrowser',
      sessionId: session.id,
      title: `Files - ${session.hostname}`,
      icon: 'folder',
    });
    hideContextMenu();
  };

  const handleOpenProcesses = () => {
    addTab({
      id: `tab-${Date.now()}`,
      type: 'processes',
      sessionId: session.id,
      title: `Processes - ${session.hostname}`,
      icon: 'activity',
    });
    hideContextMenu();
  };

  const handlePrivEsc = () => {
    toast.success('Running privilege escalation scan...');
    hideContextMenu();
  };

  const handleHarvestCreds = () => {
    toast.success('Harvesting credentials...');
    hideContextMenu();
  };

  const handleNetworkDiscovery = () => {
    toast.success('Running network discovery...');
    hideContextMenu();
  };

  const handleLateralMove = () => {
    toast('Opening lateral movement wizard...');
    hideContextMenu();
  };

  const handlePersistence = () => {
    toast('Opening persistence options...');
    hideContextMenu();
  };

  const handleUpload = () => {
    toast('Opening file upload dialog...');
    hideContextMenu();
  };

  const handleDownload = () => {
    toast('Opening file download dialog...');
    hideContextMenu();
  };

  const handleKillSession = async () => {
    try {
      await terminateSession(session.id);
      removeSession(session.id);
      toast.success('Session terminated');
    } catch (error) {
      toast.error(`Failed to terminate session: ${error}`);
    }
    hideContextMenu();
  };

  const handleAddNote = () => {
    toast('Opening note editor...');
    hideContextMenu();
  };

  const handleAddTag = () => {
    toast('Opening tag editor...');
    hideContextMenu();
  };

  // Position menu within viewport bounds
  const menuStyle: React.CSSProperties = {
    left: Math.min(contextMenu.x, window.innerWidth - 220),
    top: Math.min(contextMenu.y, window.innerHeight - 400),
  };

  return (
    <div
      className="context-menu"
      style={menuStyle}
      onClick={(e) => e.stopPropagation()}
    >
      {/* Session info header */}
      <div className="px-3 py-2 border-b border-dark-600">
        <p className="font-medium text-text-primary text-sm">{session.hostname}</p>
        <p className="text-xs text-text-muted">
          {session.username} @ {session.ip_address}
        </p>
      </div>

      <MenuHeader label="Explore" />
      <MenuItem icon={Terminal} label="Terminal" shortcut="Enter" onClick={handleOpenTerminal} />
      <MenuItem icon={FolderOpen} label="File Browser" onClick={handleOpenFileBrowser} />
      <MenuItem icon={Activity} label="Processes" onClick={handleOpenProcesses} />

      <MenuDivider />
      <MenuHeader label="Access" />
      <MenuItem icon={Play} label="Execute Command" onClick={handleOpenTerminal} />
      <MenuItem icon={Zap} label="Escalate Privileges" onClick={handlePrivEsc} />
      <MenuItem icon={Key} label="Harvest Credentials" onClick={handleHarvestCreds} />

      <MenuDivider />
      <MenuHeader label="Pivot" />
      <MenuItem icon={Network} label="Network Discovery" onClick={handleNetworkDiscovery} />
      <MenuItem icon={Globe} label="Lateral Movement" onClick={handleLateralMove} />
      <MenuItem icon={Lock} label="Install Persistence" onClick={handlePersistence} />

      <MenuDivider />
      <MenuHeader label="Transfer" />
      <MenuItem icon={Upload} label="Upload File" onClick={handleUpload} />
      <MenuItem icon={Download} label="Download File" onClick={handleDownload} />

      <MenuDivider />
      <MenuHeader label="Session" />
      <MenuItem icon={StickyNote} label="Add Note" onClick={handleAddNote} />
      <MenuItem icon={Tag} label="Add Tag" onClick={handleAddTag} />
      <MenuItem icon={Trash2} label="Kill Session" danger onClick={handleKillSession} />
    </div>
  );
}
