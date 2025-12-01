import { useAppStore } from "../store";
import { Terminal } from "./Terminal";
import {
  FileBrowser,
  ProcessViewer,
  NetworkScanner,
  CredentialsViewer,
  EventLog,
  TaskScheduler,
  Notes,
  NetworkMap,
  MitreAttack,
  Reports,
} from "./modules";
import { PayloadBuilder } from "./PayloadBuilder";
import { PostExploitation } from "./post_exploitation";
import { OpsecDashboard } from "./modules/opsec";
import { WorkflowWizard } from "./WorkflowWizard";
import { ErrorBoundary } from "./ErrorBoundary";

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

  // Wrap each module component with ErrorBoundary for graceful error handling
  switch (activeTab.type) {
    case "terminal":
      return <Terminal tabId={activeTab.id} sessionId={activeTab.sessionId} />;
    case "filebrowser":
      return (
        <ErrorBoundary name="FileBrowser">
          <FileBrowser sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "processes":
      return (
        <ErrorBoundary name="ProcessViewer">
          <ProcessViewer sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "payloads":
      return (
        <ErrorBoundary name="PayloadBuilder">
          <PayloadBuilder />
        </ErrorBoundary>
      );
    case "network":
      return (
        <ErrorBoundary name="NetworkScanner">
          <NetworkScanner sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "scanner":
      return (
        <ErrorBoundary name="NetworkScanner">
          <NetworkScanner sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "credentials":
      return (
        <ErrorBoundary name="CredentialsViewer">
          <CredentialsViewer sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "eventlog":
      return (
        <ErrorBoundary name="EventLog">
          <EventLog sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "scheduler":
      return (
        <ErrorBoundary name="TaskScheduler">
          <TaskScheduler sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "notes":
      return (
        <ErrorBoundary name="Notes">
          <Notes sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "postexploitation":
      return (
        <ErrorBoundary name="PostExploitation">
          <PostExploitation sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "networkmap":
      return (
        <ErrorBoundary name="NetworkMap">
          <NetworkMap sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "mitre":
      return (
        <ErrorBoundary name="MitreAttack">
          <MitreAttack sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "reports":
      return (
        <ErrorBoundary name="Reports">
          <Reports sessionId={activeTab.sessionId} />
        </ErrorBoundary>
      );
    case "opsec":
      return (
        <ErrorBoundary name="OpsecDashboard">
          <OpsecDashboard />
        </ErrorBoundary>
      );
    case "workflow":
      return (
        <ErrorBoundary name="WorkflowWizard">
          <WorkflowWizard />
        </ErrorBoundary>
      );
    default:
      return (
        <div className="h-full flex items-center justify-center text-text-muted">
          <p>Unknown tab type</p>
        </div>
      );
  }
}
