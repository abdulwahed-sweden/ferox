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
      <div className="flex gap-2 mb-5">
        {[
          { id: "demo", label: "Demo", icon: Plus },
          { id: "direct", label: "Direct", icon: Globe },
          { id: "listener", label: "Listener", icon: Wifi },
        ].map((tab) => (
          <button
            key={tab.id}
            onClick={() => setMethod(tab.id as ConnectionMethod)}
            className={clsx(
              "flex-1 flex items-center justify-center gap-2 rounded transition-colors",
              method === tab.id
                ? "bg-[var(--color-primary)] text-white"
                : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]"
            )}
            style={{
              height: "36px",
              fontSize: "13px",
              fontWeight: 500,
              backgroundColor: method === tab.id ? undefined : "var(--surface-secondary)",
            }}
          >
            <tab.icon size={14} />
            <span style={{ whiteSpace: "nowrap" }}>{tab.label}</span>
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
            <label
              className="block mb-1.5"
              style={{ fontSize: "13px", color: "var(--text-secondary)" }}
            >
              Target IP Address *
            </label>
            <input
              type="text"
              value={formData.ipAddress}
              onChange={(e) => handleFormChange("ipAddress", e.target.value)}
              placeholder="192.168.1.100"
              className="w-full rounded focus:outline-none"
              style={{
                height: "38px",
                padding: "0 12px",
                fontSize: "13px",
                backgroundColor: "var(--surface-secondary)",
                border: "1px solid var(--border-primary)",
                color: "var(--text-primary)",
              }}
            />
          </div>

          <div>
            <label
              className="block mb-1.5"
              style={{ fontSize: "13px", color: "var(--text-secondary)" }}
            >
              Hostname *
            </label>
            <input
              type="text"
              value={formData.hostname}
              onChange={(e) => handleFormChange("hostname", e.target.value)}
              placeholder="target-server"
              className="w-full rounded focus:outline-none"
              style={{
                height: "38px",
                padding: "0 12px",
                fontSize: "13px",
                backgroundColor: "var(--surface-secondary)",
                border: "1px solid var(--border-primary)",
                color: "var(--text-primary)",
              }}
            />
          </div>

          <div>
            <label
              className="block mb-1.5"
              style={{ fontSize: "13px", color: "var(--text-secondary)" }}
            >
              Operating System
            </label>
            <div className="grid grid-cols-3 gap-2">
              {osOptions.map((os) => (
                <button
                  key={os.value}
                  onClick={() => handleFormChange("os", os.value)}
                  className={clsx(
                    "flex flex-col items-center gap-1.5 py-2.5 rounded border transition-colors",
                    formData.os === os.value
                      ? "border-[var(--color-primary)] bg-[var(--color-primary)]/10"
                      : "border-[var(--border-primary)] hover:border-[var(--border-secondary)]"
                  )}
                >
                  <os.icon size={18} className="text-[var(--text-secondary)]" />
                  <span style={{ fontSize: "12px", color: "var(--text-secondary)" }}>
                    {os.label}
                  </span>
                </button>
              ))}
            </div>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <label
                className="block mb-1.5"
                style={{ fontSize: "13px", color: "var(--text-secondary)" }}
              >
                Username
              </label>
              <input
                type="text"
                value={formData.username}
                onChange={(e) => handleFormChange("username", e.target.value)}
                placeholder="admin"
                className="w-full rounded focus:outline-none"
                style={{
                  height: "38px",
                  padding: "0 12px",
                  fontSize: "13px",
                  backgroundColor: "var(--surface-secondary)",
                  border: "1px solid var(--border-primary)",
                  color: "var(--text-primary)",
                }}
              />
            </div>
            <div>
              <label
                className="block mb-1.5"
                style={{ fontSize: "13px", color: "var(--text-secondary)" }}
              >
                Port
              </label>
              <input
                type="number"
                value={formData.port}
                onChange={(e) =>
                  handleFormChange("port", parseInt(e.target.value))
                }
                className="w-full rounded focus:outline-none"
                style={{
                  height: "38px",
                  padding: "0 12px",
                  fontSize: "13px",
                  backgroundColor: "var(--surface-secondary)",
                  border: "1px solid var(--border-primary)",
                  color: "var(--text-primary)",
                }}
              />
            </div>
          </div>
        </div>
      )}

      {/* Listener Mode */}
      {method === "listener" && (
        <div className="space-y-4">
          <div
            className="p-4 rounded"
            style={{ backgroundColor: "var(--surface-secondary)" }}
          >
            <div className="flex items-center gap-2 mb-2">
              <Wifi size={16} className="text-[var(--color-primary)]" />
              <span
                style={{
                  fontSize: "14px",
                  fontWeight: 500,
                  color: "var(--text-primary)",
                }}
              >
                Listener Configuration
              </span>
            </div>
            <p
              className="mb-4"
              style={{ fontSize: "12px", color: "var(--text-secondary)" }}
            >
              Start a listener to wait for incoming connections from implants.
            </p>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label
                  className="block mb-1.5"
                  style={{ fontSize: "12px", color: "var(--text-secondary)" }}
                >
                  Listen Address
                </label>
                <input
                  type="text"
                  defaultValue="0.0.0.0"
                  className="w-full rounded focus:outline-none"
                  style={{
                    height: "38px",
                    padding: "0 12px",
                    fontSize: "13px",
                    backgroundColor: "var(--surface-primary)",
                    border: "1px solid var(--border-primary)",
                    color: "var(--text-primary)",
                  }}
                />
              </div>
              <div>
                <label
                  className="block mb-1.5"
                  style={{ fontSize: "12px", color: "var(--text-secondary)" }}
                >
                  Listen Port
                </label>
                <input
                  type="number"
                  defaultValue={4444}
                  className="w-full rounded focus:outline-none"
                  style={{
                    height: "38px",
                    padding: "0 12px",
                    fontSize: "13px",
                    backgroundColor: "var(--surface-primary)",
                    border: "1px solid var(--border-primary)",
                    color: "var(--text-primary)",
                  }}
                />
              </div>
            </div>
          </div>

          <div
            className="flex items-center gap-2 p-3 rounded"
            style={{
              backgroundColor: "rgba(234, 179, 8, 0.1)",
              border: "1px solid rgba(234, 179, 8, 0.2)",
            }}
          >
            <Key size={14} style={{ color: "#eab308" }} />
            <span style={{ fontSize: "12px", color: "#eab308" }}>
              Listener mode requires proper authorization and network access
            </span>
          </div>
        </div>
      )}

      {/* Footer */}
      {method !== "demo" && (
        <div
          className="flex justify-end gap-3 mt-6 pt-4"
          style={{ borderTop: "1px solid var(--border-primary)" }}
        >
          <button
            onClick={onClose}
            className="rounded hover:bg-[var(--bg-hover)] transition-colors"
            style={{
              height: "36px",
              padding: "0 16px",
              fontSize: "13px",
              color: "var(--text-secondary)",
            }}
          >
            Cancel
          </button>
          <button
            onClick={handleConnect}
            disabled={isConnecting}
            className={clsx(
              "flex items-center gap-2 rounded text-white transition-colors",
              isConnecting
                ? "bg-[var(--color-primary)]/50 cursor-not-allowed"
                : "bg-[var(--color-primary)] hover:bg-[var(--color-primary)]/90"
            )}
            style={{
              height: "36px",
              padding: "0 16px",
              fontSize: "13px",
            }}
          >
            {isConnecting ? (
              <>
                <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                <span>Connecting...</span>
              </>
            ) : method === "listener" ? (
              <>
                <Wifi size={14} />
                <span>Start Listener</span>
              </>
            ) : (
              <>
                <Globe size={14} />
                <span>Connect</span>
              </>
            )}
          </button>
        </div>
      )}
    </Modal>
  );
}
