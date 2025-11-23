import { useAppStore } from '../store';
import { Terminal } from './Terminal';
import { FileBrowser, ProcessViewer } from './modules';
import { Globe } from 'lucide-react';

export function TabContent() {
  const { tabs, activeTabId } = useAppStore();

  const activeTab = tabs.find((t) => t.id === activeTabId);

  if (!activeTab) {
    return (
      <div className="h-full flex items-center justify-center text-text-muted">
        <p>No tab selected</p>
      </div>
    );
  }

  switch (activeTab.type) {
    case 'terminal':
      return <Terminal tabId={activeTab.id} sessionId={activeTab.sessionId} />;
    case 'filebrowser':
      return <FileBrowser sessionId={activeTab.sessionId} />;
    case 'processes':
      return <ProcessViewer sessionId={activeTab.sessionId} />;
    case 'network':
      return (
        <div className="h-full flex flex-col items-center justify-center text-text-muted bg-dark-900">
          <Globe size={48} className="mb-4 opacity-30" />
          <p className="text-lg">Network Discovery</p>
          <p className="text-sm mt-2">Coming soon...</p>
        </div>
      );
    default:
      return (
        <div className="h-full flex items-center justify-center text-text-muted">
          <p>Unknown tab type</p>
        </div>
      );
  }
}
