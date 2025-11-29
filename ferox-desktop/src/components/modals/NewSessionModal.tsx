import { useState } from "react";
import { Modal } from "./Modal";
import { useAppStore } from "../../store";
import {
  Server,
  Monitor,
  Apple,
  Globe,
  Plus,
  Wifi,
  Key,
} from "lucide-react";
import { clsx } from "clsx";
import toast from "react-hot-toast";
import type { Session, OsType, PrivilegeLevel } from "../../types";

interface NewSessionModalProps {
  isOpen: boolean;
  onClose: () => void;
}

type ConnectionMethod = "direct" | "listener" | "demo";

export function NewSessionModal({ isOpen, onClose }: NewSessionModalProps) {
  const { addSession, setSessionTree, sessionTree } = useAppStore();
  const [method, setMethod] = useState<ConnectionMethod>("demo");
  const [formData, setFormData] = useState({
    hostname: "",
    ipAddress: "",
    os: "windows" as OsType,
    username: "",
    port: 4444,
  });
  const [isConnecting, setIsConnecting] = useState(false);

  const osOptions: { value: OsType; label: string; icon: typeof Monitor }[] = [
    { value: "windows", label: "Windows", icon: Monitor },
    { value: "linux", label: "Linux", icon: Server },
    { value: "macos", label: "macOS", icon: Apple },
  ];

  const createDemoSession = () => {
    const demoSessions: Partial<Session>[] = [
      {
        hostname: "DC01.corp.local",
        ip_address: "10.10.10.100",
        os: "windows",
        os_version: "Windows Server 2022",
        username: "Administrator",
        privileges: "system",
      },
      {
        hostname: "web-server-01",
        ip_address: "192.168.1.50",
        os: "linux",
        os_version: "Ubuntu 22.04 LTS",
        username: "www-data",
        privileges: "user",
      },
      {
        hostname: "MacBook-Pro",
        ip_address: "192.168.1.75",
        os: "macos",
        os_version: "macOS Ventura 13.5",
        username: "developer",
        privileges: "administrator",
      },
      {
        hostname: "DB-Server",
        ip_address: "10.10.10.200",
        os: "linux",
        os_version: "CentOS 8",
        username: "postgres",
        privileges: "user",
      },
    ];

    // Pick a random demo session
    const template = demoSessions[Math.floor(Math.random() * demoSessions.length)];
    const sessionId = `demo-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;

    const newSession: Session = {
      id: sessionId,
      hostname: template.hostname!,
      ip_address: template.ip_address!,
      os: template.os!,
      os_version: template.os_version!,
      architecture: "x64",
      username: template.username!,
      privileges: template.privileges as PrivilegeLevel,
      status: "active",
      established_at: new Date().toISOString(),
      last_seen: new Date().toISOString(),
      parent_id: null,
      intelligence: {
        domain: template.os === "windows" ? "corp.local" : null,
        is_domain_joined: template.os === "windows",
        detected_av: template.os === "windows" ? ["Windows Defender"] : [],
        stealth_mode: "normal",
        network_segment: "Corporate",
      },
      metrics: {
        credentials_count: Math.floor(Math.random() * 10),
        commands_executed: Math.floor(Math.random() * 50),
        files_transferred: Math.floor(Math.random() * 20),
        persistence_methods: Math.floor(Math.random() * 3),
      },
      tags: ["demo"],
      note: "Demo session for testing",
    };

    addSession(newSession);

    // Update session tree
    setSessionTree([
      ...sessionTree,
      { session: newSession, children: [] },
    ]);

    toast.success(`Demo session created: ${newSession.hostname}`);
    onClose();
  };

  const handleConnect = async () => {
    if (method === "demo") {
      createDemoSession();
      return;
    }

    // Validate form
    if (!formData.hostname || !formData.ipAddress) {
      toast.error("Please fill in all required fields");
      return;
    }

    setIsConnecting(true);

    // Simulate connection attempt
    await new Promise((resolve) => setTimeout(resolve, 1500));

    // For now, just show a message that real connections aren't implemented
    toast.error("Real connections not implemented in demo mode");
    setIsConnecting(false);
  };

  const handleFormChange = (field: string, value: string | number) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="New Session" size="md">
      {/* Connection Method Tabs */}
      <div className="flex gap-2 mb-4">
        {[
          { id: "demo", label: "Demo Session", icon: Plus },
          { id: "direct", label: "Direct Connect", icon: Globe },
          { id: "listener", label: "Start Listener", icon: Wifi },
        ].map((tab) => (
          <button
            key={tab.id}
            onClick={() => setMethod(tab.id as ConnectionMethod)}
            className={clsx(
              "flex-1 flex items-center justify-center gap-2 px-3 py-2 rounded text-sm transition-colors",
              method === tab.id
                ? "bg-[var(--color-primary)] text-white"
                : "bg-[var(--surface-secondary)] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]"
            )}
          >
            <tab.icon size={14} />
            {tab.label}
          </button>
        ))}
      </div>

      {/* Demo Mode */}
      {method === "demo" && (
        <div className="space-y-4">
          <div className="p-4 rounded bg-[var(--surface-secondary)] text-center">
            <div className="w-12 h-12 rounded-full bg-[var(--color-primary)]/10 flex items-center justify-center mx-auto mb-3">
              <Plus size={24} className="text-[var(--color-primary)]" />
            </div>
            <h4 className="text-sm font-medium text-[var(--text-primary)] mb-1">
              Create Demo Session
            </h4>
            <p className="text-xs text-[var(--text-secondary)] mb-4">
              Creates a simulated session with random demo data.
              Perfect for testing the UI and exploring features.
            </p>
            <button
              onClick={handleConnect}
              className="px-4 py-2 rounded bg-[var(--color-primary)] text-white text-sm hover:bg-[var(--color-primary)]/90 transition-colors"
            >
              Create Demo Session
            </button>
          </div>

          <p className="text-xs text-[var(--text-muted)] text-center">
            Demo sessions are simulated and don&apos;t connect to real targets.
          </p>
        </div>
      )}

      {/* Direct Connect */}
      {method === "direct" && (
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-[var(--text-secondary)] mb-1">
              Target IP Address *
            </label>
            <input
              type="text"
              value={formData.ipAddress}
              onChange={(e) => handleFormChange("ipAddress", e.target.value)}
              placeholder="192.168.1.100"
              className="w-full px-3 py-2 rounded bg-[var(--surface-secondary)] border border-[var(--border-primary)] text-sm text-[var(--text-primary)] placeholder:text-[var(--text-muted)] focus:outline-none focus:border-[var(--color-primary)]"
            />
          </div>

          <div>
            <label className="block text-sm text-[var(--text-secondary)] mb-1">
              Hostname *
            </label>
            <input
              type="text"
              value={formData.hostname}
              onChange={(e) => handleFormChange("hostname", e.target.value)}
              placeholder="target-server"
              className="w-full px-3 py-2 rounded bg-[var(--surface-secondary)] border border-[var(--border-primary)] text-sm text-[var(--text-primary)] placeholder:text-[var(--text-muted)] focus:outline-none focus:border-[var(--color-primary)]"
            />
          </div>

          <div>
            <label className="block text-sm text-[var(--text-secondary)] mb-1">
              Operating System
            </label>
            <div className="grid grid-cols-3 gap-2">
              {osOptions.map((os) => (
                <button
                  key={os.value}
                  onClick={() => handleFormChange("os", os.value)}
                  className={clsx(
                    "flex flex-col items-center gap-1 p-2 rounded border transition-colors",
                    formData.os === os.value
                      ? "border-[var(--color-primary)] bg-[var(--color-primary)]/10"
                      : "border-[var(--border-primary)] hover:border-[var(--border-secondary)]"
                  )}
                >
                  <os.icon size={16} className="text-[var(--text-secondary)]" />
                  <span className="text-xs text-[var(--text-secondary)]">
                    {os.label}
                  </span>
                </button>
              ))}
            </div>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm text-[var(--text-secondary)] mb-1">
                Username
              </label>
              <input
                type="text"
                value={formData.username}
                onChange={(e) => handleFormChange("username", e.target.value)}
                placeholder="admin"
                className="w-full px-3 py-2 rounded bg-[var(--surface-secondary)] border border-[var(--border-primary)] text-sm text-[var(--text-primary)] placeholder:text-[var(--text-muted)] focus:outline-none focus:border-[var(--color-primary)]"
              />
            </div>
            <div>
              <label className="block text-sm text-[var(--text-secondary)] mb-1">
                Port
              </label>
              <input
                type="number"
                value={formData.port}
                onChange={(e) =>
                  handleFormChange("port", parseInt(e.target.value))
                }
                className="w-full px-3 py-2 rounded bg-[var(--surface-secondary)] border border-[var(--border-primary)] text-sm text-[var(--text-primary)] focus:outline-none focus:border-[var(--color-primary)]"
              />
            </div>
          </div>
        </div>
      )}

      {/* Listener Mode */}
      {method === "listener" && (
        <div className="space-y-4">
          <div className="p-4 rounded bg-[var(--surface-secondary)]">
            <div className="flex items-center gap-2 mb-2">
              <Wifi size={16} className="text-[var(--color-primary)]" />
              <span className="text-sm font-medium text-[var(--text-primary)]">
                Listener Configuration
              </span>
            </div>
            <p className="text-xs text-[var(--text-secondary)] mb-4">
              Start a listener to wait for incoming connections from implants.
            </p>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs text-[var(--text-secondary)] mb-1">
                  Listen Address
                </label>
                <input
                  type="text"
                  defaultValue="0.0.0.0"
                  className="w-full px-3 py-2 rounded bg-[var(--surface-primary)] border border-[var(--border-primary)] text-sm text-[var(--text-primary)] focus:outline-none focus:border-[var(--color-primary)]"
                />
              </div>
              <div>
                <label className="block text-xs text-[var(--text-secondary)] mb-1">
                  Listen Port
                </label>
                <input
                  type="number"
                  defaultValue={4444}
                  className="w-full px-3 py-2 rounded bg-[var(--surface-primary)] border border-[var(--border-primary)] text-sm text-[var(--text-primary)] focus:outline-none focus:border-[var(--color-primary)]"
                />
              </div>
            </div>
          </div>

          <div className="flex items-center gap-2 p-3 rounded bg-yellow-500/10 border border-yellow-500/20">
            <Key size={14} className="text-yellow-500" />
            <span className="text-xs text-yellow-500">
              Listener mode requires proper authorization and network access
            </span>
          </div>
        </div>
      )}

      {/* Footer */}
      {method !== "demo" && (
        <div className="flex justify-end gap-2 mt-6 pt-4 border-t border-[var(--border-primary)]">
          <button
            onClick={onClose}
            className="px-4 py-2 rounded text-sm text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleConnect}
            disabled={isConnecting}
            className={clsx(
              "flex items-center gap-2 px-4 py-2 rounded text-sm text-white transition-colors",
              isConnecting
                ? "bg-[var(--color-primary)]/50 cursor-not-allowed"
                : "bg-[var(--color-primary)] hover:bg-[var(--color-primary)]/90"
            )}
          >
            {isConnecting ? (
              <>
                <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                Connecting...
              </>
            ) : method === "listener" ? (
              <>
                <Wifi size={14} />
                Start Listener
              </>
            ) : (
              <>
                <Globe size={14} />
                Connect
              </>
            )}
          </button>
        </div>
      )}
    </Modal>
  );
}
