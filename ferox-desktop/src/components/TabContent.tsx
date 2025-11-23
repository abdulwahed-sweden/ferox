import { useAppStore } from '../store';
import { Terminal } from './Terminal';
import {
  FileBrowser,
  ProcessViewer,
  NetworkScanner,
  CredentialsViewer,
  EventLog,
  TaskScheduler,
  Notes
} from './modules';
import { PayloadBuilder } from './PayloadBuilder';

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
    case 'payloads':
      return <PayloadBuilder />;
    case 'network':
      return <NetworkScanner sessionId={activeTab.sessionId} />;
    case 'scanner':
      return <NetworkScanner sessionId={activeTab.sessionId} />;
    case 'credentials':
      return <CredentialsViewer sessionId={activeTab.sessionId} />;
    case 'eventlog':
      return <EventLog sessionId={activeTab.sessionId} />;
    case 'scheduler':
      return <TaskScheduler sessionId={activeTab.sessionId} />;
    case 'notes':
      return <Notes sessionId={activeTab.sessionId} />;
    default:
      return (
        <div className="h-full flex items-center justify-center text-text-muted">
          <p>Unknown tab type</p>
        </div>
      );
  }
}
