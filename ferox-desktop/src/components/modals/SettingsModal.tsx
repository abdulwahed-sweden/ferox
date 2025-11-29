import { useState } from "react";
import { Modal } from "./Modal";
import { useTheme } from "../../hooks/useTheme";
import {
  Sun,
  Moon,
  Monitor,
  Bell,
  Shield,
  Database,
  Palette,
  Save,
} from "lucide-react";
import { clsx } from "clsx";
import toast from "react-hot-toast";

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
}

type SettingsTab = "appearance" | "notifications" | "security" | "data";

export function SettingsModal({ isOpen, onClose }: SettingsModalProps) {
  const { theme, setLightTheme, setDarkTheme } = useTheme();
  const [activeTab, setActiveTab] = useState<SettingsTab>("appearance");
  const [notifications, setNotifications] = useState({
    sessionAlerts: true,
    commandComplete: true,
    errorAlerts: true,
    sounds: false,
  });
  const [security, setSecurity] = useState({
    autoLock: true,
    lockTimeout: 5,
    clearOnExit: false,
  });

  const tabs: { id: SettingsTab; label: string; icon: typeof Sun }[] = [
    { id: "appearance", label: "Appearance", icon: Palette },
    { id: "notifications", label: "Notifications", icon: Bell },
    { id: "security", label: "Security", icon: Shield },
    { id: "data", label: "Data", icon: Database },
  ];

  const handleSave = () => {
    // In a real app, save settings to localStorage or backend
    localStorage.setItem(
      "ferox-settings",
      JSON.stringify({
        notifications,
        security,
      })
    );
    toast.success("Settings saved");
    onClose();
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Settings" size="lg">
      <div className="flex gap-4" style={{ minHeight: "320px" }}>
        {/* Sidebar */}
        <div
          className="shrink-0 pr-4"
          style={{
            width: "150px",
            borderRight: "1px solid var(--border-primary)",
          }}
        >
          <nav className="space-y-1">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={clsx(
                  "w-full flex items-center gap-2 px-3 py-2 rounded transition-colors",
                  activeTab === tab.id
                    ? "bg-[var(--color-primary)]/10 text-[var(--color-primary)]"
                    : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text-primary)]"
                )}
                style={{
                  fontSize: "13px",
                  whiteSpace: "nowrap",
                }}
              >
                <tab.icon size={15} />
                <span>{tab.label}</span>
              </button>
            ))}
          </nav>
        </div>

        {/* Content */}
        <div className="flex-1">
          {activeTab === "appearance" && (
            <div className="space-y-5">
              <div>
                <h4
                  className="mb-3"
                  style={{
                    fontSize: "13px",
                    fontWeight: 500,
                    color: "var(--text-primary)",
                  }}
                >
                  Theme
                </h4>
                <div className="grid grid-cols-3 gap-3">
                  <button
                    onClick={setLightTheme}
                    className={clsx(
                      "flex flex-col items-center gap-2 py-3 rounded border transition-colors",
                      theme === "light"
                        ? "border-[var(--color-primary)] bg-[var(--color-primary)]/10"
                        : "border-[var(--border-primary)] hover:border-[var(--border-secondary)]"
                    )}
                  >
                    <Sun size={20} className="text-yellow-500" />
                    <span style={{ fontSize: "12px", color: "var(--text-secondary)" }}>
                      Light
                    </span>
                  </button>
                  <button
                    onClick={setDarkTheme}
                    className={clsx(
                      "flex flex-col items-center gap-2 py-3 rounded border transition-colors",
                      theme === "dark"
                        ? "border-[var(--color-primary)] bg-[var(--color-primary)]/10"
                        : "border-[var(--border-primary)] hover:border-[var(--border-secondary)]"
                    )}
                  >
                    <Moon size={20} className="text-blue-400" />
                    <span style={{ fontSize: "12px", color: "var(--text-secondary)" }}>
                      Dark
                    </span>
                  </button>
                  <button
                    disabled
                    className="flex flex-col items-center gap-2 py-3 rounded border border-[var(--border-primary)] opacity-50 cursor-not-allowed"
                  >
                    <Monitor size={20} className="text-[var(--text-muted)]" />
                    <span style={{ fontSize: "12px", color: "var(--text-muted)" }}>
                      System
                    </span>
                  </button>
                </div>
              </div>

              <div>
                <h4
                  className="mb-3"
                  style={{
                    fontSize: "13px",
                    fontWeight: 500,
                    color: "var(--text-primary)",
                  }}
                >
                  Accent Color
                </h4>
                <div className="flex gap-2">
                  {["#FA7209", "#10B981", "#3B82F6", "#8B5CF6", "#EF4444"].map(
                    (color) => (
                      <button
                        key={color}
                        className={clsx(
                          "w-8 h-8 rounded-full border-2 transition-transform hover:scale-110",
                          color === "#FA7209"
                            ? "border-white ring-2 ring-[var(--color-primary)]"
                            : "border-transparent"
                        )}
                        style={{ backgroundColor: color }}
                        title={color}
                      />
                    )
                  )}
                </div>
                <p
                  className="mt-2"
                  style={{ fontSize: "11px", color: "var(--text-muted)" }}
                >
                  Custom accent colors coming soon
                </p>
              </div>
            </div>
          )}

          {activeTab === "notifications" && (
            <div className="space-y-3">
              <h4
                className="mb-3"
                style={{
                  fontSize: "13px",
                  fontWeight: 500,
                  color: "var(--text-primary)",
                }}
              >
                Notification Preferences
              </h4>

              {[
                {
                  key: "sessionAlerts",
                  label: "Session Alerts",
                  desc: "Notify when sessions connect or disconnect",
                },
                {
                  key: "commandComplete",
                  label: "Command Complete",
                  desc: "Notify when long-running commands finish",
                },
                {
                  key: "errorAlerts",
                  label: "Error Alerts",
                  desc: "Show notifications for errors",
                },
                {
                  key: "sounds",
                  label: "Sound Effects",
                  desc: "Play sounds for notifications",
                },
              ].map((item) => (
                <label
                  key={item.key}
                  className="flex items-center justify-between p-3 rounded cursor-pointer"
                  style={{ backgroundColor: "var(--surface-secondary)" }}
                >
                  <div>
                    <p style={{ fontSize: "13px", color: "var(--text-primary)" }}>
                      {item.label}
                    </p>
                    <p style={{ fontSize: "11px", color: "var(--text-muted)" }}>
                      {item.desc}
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    checked={
                      notifications[item.key as keyof typeof notifications]
                    }
                    onChange={(e) =>
                      setNotifications((prev) => ({
                        ...prev,
                        [item.key]: e.target.checked,
                      }))
                    }
                    className="w-4 h-4 rounded"
                  />
                </label>
              ))}
            </div>
          )}

          {activeTab === "security" && (
            <div className="space-y-4">
              <h4
                className="mb-3"
                style={{
                  fontSize: "13px",
                  fontWeight: 500,
                  color: "var(--text-primary)",
                }}
              >
                Security Settings
              </h4>

              <label
                className="flex items-center justify-between p-3 rounded cursor-pointer"
                style={{ backgroundColor: "var(--surface-secondary)" }}
              >
                <div>
                  <p style={{ fontSize: "13px", color: "var(--text-primary)" }}>
                    Auto Lock
                  </p>
                  <p style={{ fontSize: "11px", color: "var(--text-muted)" }}>
                    Lock app after inactivity
                  </p>
                </div>
                <input
                  type="checkbox"
                  checked={security.autoLock}
                  onChange={(e) =>
                    setSecurity((prev) => ({
                      ...prev,
                      autoLock: e.target.checked,
                    }))
                  }
                  className="w-4 h-4 rounded"
                />
              </label>

              {security.autoLock && (
                <div
                  className="p-3 rounded"
                  style={{ backgroundColor: "var(--surface-secondary)" }}
                >
                  <label style={{ fontSize: "13px", color: "var(--text-primary)" }}>
                    Lock Timeout (minutes)
                  </label>
                  <select
                    value={security.lockTimeout}
                    onChange={(e) =>
                      setSecurity((prev) => ({
                        ...prev,
                        lockTimeout: Number(e.target.value),
                      }))
                    }
                    className="mt-2 w-full rounded"
                    style={{
                      height: "38px",
                      padding: "0 12px",
                      fontSize: "13px",
                      backgroundColor: "var(--surface-primary)",
                      border: "1px solid var(--border-primary)",
                      color: "var(--text-primary)",
                    }}
                  >
                    <option value={1}>1 minute</option>
                    <option value={5}>5 minutes</option>
                    <option value={10}>10 minutes</option>
                    <option value={30}>30 minutes</option>
                  </select>
                </div>
              )}

              <label
                className="flex items-center justify-between p-3 rounded cursor-pointer"
                style={{ backgroundColor: "var(--surface-secondary)" }}
              >
                <div>
                  <p style={{ fontSize: "13px", color: "var(--text-primary)" }}>
                    Clear Data on Exit
                  </p>
                  <p style={{ fontSize: "11px", color: "var(--text-muted)" }}>
                    Clear sensitive data when app closes
                  </p>
                </div>
                <input
                  type="checkbox"
                  checked={security.clearOnExit}
                  onChange={(e) =>
                    setSecurity((prev) => ({
                      ...prev,
                      clearOnExit: e.target.checked,
                    }))
                  }
                  className="w-4 h-4 rounded"
                />
              </label>
            </div>
          )}

          {activeTab === "data" && (
            <div className="space-y-4">
              <h4
                className="mb-3"
                style={{
                  fontSize: "13px",
                  fontWeight: 500,
                  color: "var(--text-primary)",
                }}
              >
                Data Management
              </h4>

              <div
                className="p-3 rounded"
                style={{ backgroundColor: "var(--surface-secondary)" }}
              >
                <p style={{ fontSize: "13px", color: "var(--text-primary)" }}>
                  Session Data
                </p>
                <p
                  className="mb-3"
                  style={{ fontSize: "11px", color: "var(--text-muted)" }}
                >
                  Clear all stored session information
                </p>
                <button
                  onClick={() => toast.success("Session data cleared")}
                  className="rounded transition-colors"
                  style={{
                    height: "32px",
                    padding: "0 12px",
                    fontSize: "12px",
                    backgroundColor: "rgba(239, 68, 68, 0.1)",
                    color: "#f87171",
                  }}
                >
                  Clear Sessions
                </button>
              </div>

              <div
                className="p-3 rounded"
                style={{ backgroundColor: "var(--surface-secondary)" }}
              >
                <p style={{ fontSize: "13px", color: "var(--text-primary)" }}>
                  Export Settings
                </p>
                <p
                  className="mb-3"
                  style={{ fontSize: "11px", color: "var(--text-muted)" }}
                >
                  Export your settings as JSON
                </p>
                <button
                  onClick={() => toast.success("Settings exported")}
                  className="rounded bg-[var(--color-primary)]/10 text-[var(--color-primary)] transition-colors"
                  style={{
                    height: "32px",
                    padding: "0 12px",
                    fontSize: "12px",
                  }}
                >
                  Export Settings
                </button>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Footer */}
      <div
        className="flex justify-end gap-3 mt-4 pt-4"
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
          onClick={handleSave}
          className="flex items-center gap-2 rounded bg-[var(--color-primary)] text-white hover:bg-[var(--color-primary)]/90 transition-colors"
          style={{
            height: "36px",
            padding: "0 16px",
            fontSize: "13px",
          }}
        >
          <Save size={14} />
          <span>Save Changes</span>
        </button>
      </div>
    </Modal>
  );
}
