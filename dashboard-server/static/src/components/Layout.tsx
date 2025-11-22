import React from 'react';
import { useDashboardStore } from '../store';
import {
  LayoutDashboard,
  Monitor,
  Network,
  Key,
  Shield,
  FileText,
  ChevronLeft,
  ChevronRight,
  Wifi,
  WifiOff,
  Terminal,
  Settings,
} from 'lucide-react';
import { clsx } from 'clsx';

interface LayoutProps {
  children: React.ReactNode;
}

const navItems = [
  { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
  { id: 'sessions', label: 'Sessions', icon: Monitor },
  { id: 'terminal', label: 'Terminal', icon: Terminal },
  { id: 'network', label: 'Network', icon: Network },
  { id: 'credentials', label: 'Credentials', icon: Key },
  { id: 'mitre', label: 'MITRE ATT&CK', icon: Shield },
  { id: 'reports', label: 'Reports', icon: FileText },
];

export function Layout({ children }: LayoutProps) {
  const { sidebarOpen, toggleSidebar, activeTab, setActiveTab, isConnected, sessions } =
    useDashboardStore();

  const activeSessions = sessions.filter((s) => s.status === 'active').length;

  return (
    <div className="min-h-screen bg-dark-900 flex">
      {/* Sidebar */}
      <aside
        className={clsx(
          'bg-dark-800 border-r border-dark-600 flex flex-col transition-all duration-300',
          sidebarOpen ? 'w-64' : 'w-16'
        )}
      >
        {/* Logo */}
        <div className="h-16 flex items-center justify-between px-4 border-b border-dark-600">
          {sidebarOpen && (
            <div className="flex items-center gap-2">
              <span className="text-2xl font-bold text-ferox-green text-glow-green">
                FEROX
              </span>
            </div>
          )}
          <button
            onClick={toggleSidebar}
            className="p-1.5 rounded-lg hover:bg-dark-600 text-text-secondary hover:text-text-primary transition-colors"
          >
            {sidebarOpen ? <ChevronLeft size={20} /> : <ChevronRight size={20} />}
          </button>
        </div>

        {/* Navigation */}
        <nav className="flex-1 p-2 space-y-1">
          {navItems.map((item) => {
            const Icon = item.icon;
            const isActive = activeTab === item.id;

            return (
              <button
                key={item.id}
                onClick={() => setActiveTab(item.id)}
                className={clsx(
                  'w-full flex items-center gap-3 px-3 py-2.5 rounded-lg transition-all duration-200',
                  isActive
                    ? 'bg-dark-600 text-ferox-green border-l-2 border-ferox-green'
                    : 'text-text-secondary hover:bg-dark-700 hover:text-text-primary'
                )}
                title={!sidebarOpen ? item.label : undefined}
              >
                <Icon size={20} />
                {sidebarOpen && <span className="font-medium">{item.label}</span>}
              </button>
            );
          })}
        </nav>

        {/* Bottom section */}
        <div className="p-2 border-t border-dark-600">
          <button
            className={clsx(
              'w-full flex items-center gap-3 px-3 py-2.5 rounded-lg transition-colors',
              'text-text-secondary hover:bg-dark-700 hover:text-text-primary'
            )}
          >
            <Settings size={20} />
            {sidebarOpen && <span>Settings</span>}
          </button>
        </div>
      </aside>

      {/* Main content */}
      <div className="flex-1 flex flex-col min-w-0">
        {/* Header */}
        <header className="h-16 bg-dark-800 border-b border-dark-600 flex items-center justify-between px-6">
          <div className="flex items-center gap-4">
            <h1 className="text-xl font-semibold text-text-primary">
              {navItems.find((item) => item.id === activeTab)?.label || 'Dashboard'}
            </h1>
          </div>

          <div className="flex items-center gap-4">
            {/* Active sessions count */}
            <div className="flex items-center gap-2 px-3 py-1.5 bg-dark-700 rounded-lg">
              <Monitor size={16} className="text-ferox-green" />
              <span className="text-sm text-text-secondary">
                <span className="text-ferox-green font-medium">{activeSessions}</span> active
              </span>
            </div>

            {/* Connection status */}
            <div
              className={clsx(
                'flex items-center gap-2 px-3 py-1.5 rounded-lg',
                isConnected ? 'bg-ferox-green/10' : 'bg-danger/10'
              )}
            >
              {isConnected ? (
                <>
                  <div className="w-2 h-2 bg-ferox-green rounded-full animate-pulse" />
                  <Wifi size={16} className="text-ferox-green" />
                  <span className="text-sm text-ferox-green">Connected</span>
                </>
              ) : (
                <>
                  <div className="w-2 h-2 bg-danger rounded-full" />
                  <WifiOff size={16} className="text-danger" />
                  <span className="text-sm text-danger">Disconnected</span>
                </>
              )}
            </div>

            {/* User */}
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 bg-dark-600 rounded-full flex items-center justify-center">
                <span className="text-sm font-medium text-ferox-green">OP</span>
              </div>
            </div>
          </div>
        </header>

        {/* Page content */}
        <main className="flex-1 overflow-auto p-6">{children}</main>
      </div>
    </div>
  );
}
