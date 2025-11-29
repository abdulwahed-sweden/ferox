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
      <div className="flex gap-4 min-h-80">
        {/* Sidebar */}
        <div className="w-40 shrink-0 border-r border-[var(--border-primary)] pr-4">
          <nav className="space-y-1">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={clsx(
                  "w-full flex items-center gap-2 px-3 py-2 rounded text-sm transition-colors",
                  activeTab === tab.id
                    ? "bg-[var(--color-primary)]/10 text-[var(--color-primary)]"
                    : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text-primary)]"
                )}
              >
                <tab.icon size={16} />
                {tab.label}
              </button>
            ))}
          </nav>
        </div>

        {/* Content */}
        <div className="flex-1">
          {activeTab === "appearance" && (
            <div className="space-y-4">
              <div>
                <h4 className="text-sm font-medium text-[var(--text-primary)] mb-2">
                  Theme
                </h4>
                <div className="grid grid-cols-3 gap-2">
                  <button
                    onClick={setLightTheme}
                    className={clsx(
                      "flex flex-col items-center gap-2 p-3 rounded border transition-colors",
                      theme === "light"
                        ? "border-[var(--color-primary)] bg-[var(--color-primary)]/10"
                        : "border-[var(--border-primary)] hover:border-[var(--border-secondary)]"
                    )}
                  >
                    <Sun size={20} className="text-yellow-500" />
                    <span className="text-xs text-[var(--text-secondary)]">
                      Light
                    </span>
                  </button>
                  <button
                    onClick={setDarkTheme}
                    className={clsx(
                      "flex flex-col items-center gap-2 p-3 rounded border transition-colors",
                      theme === "dark"
                        ? "border-[var(--color-primary)] bg-[var(--color-primary)]/10"
                        : "border-[var(--border-primary)] hover:border-[var(--border-secondary)]"
                    )}
                  >
                    <Moon size={20} className="text-blue-400" />
                    <span className="text-xs text-[var(--text-secondary)]">
                      Dark
                    </span>
                  </button>
                  <button
                    disabled
                    className="flex flex-col items-center gap-2 p-3 rounded border border-[var(--border-primary)] opacity-50 cursor-not-allowed"
                  >
                    <Monitor size={20} className="text-[var(--text-muted)]" />
                    <span className="text-xs text-[var(--text-muted)]">
                      System
                    </span>
                  </button>
                </div>
              </div>

              <div>
                <h4 className="text-sm font-medium text-[var(--text-primary)] mb-2">
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
                <p className="text-xs text-[var(--text-muted)] mt-2">
                  Custom accent colors coming soon
                </p>
              </div>
            </div>
          )}

          {activeTab === "notifications" && (
            <div className="space-y-3">
              <h4 className="text-sm font-medium text-[var(--text-primary)] mb-2">
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
                  className="flex items-center justify-between p-3 rounded bg-[var(--surface-secondary)] cursor-pointer"
                >
                  <div>
                    <p className="text-sm text-[var(--text-primary)]">
                      {item.label}
                    </p>
                    <p className="text-xs text-[var(--text-muted)]">
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
                    className="w-4 h-4 rounded border-[var(--border-primary)] text-[var(--color-primary)] focus:ring-[var(--color-primary)]"
                  />
                </label>
              ))}
            </div>
          )}

          {activeTab === "security" && (
            <div className="space-y-4">
              <h4 className="text-sm font-medium text-[var(--text-primary)] mb-2">
                Security Settings
              </h4>

              <label className="flex items-center justify-between p-3 rounded bg-[var(--surface-secondary)] cursor-pointer">
                <div>
                  <p className="text-sm text-[var(--text-primary)]">Auto Lock</p>
                  <p className="text-xs text-[var(--text-muted)]">
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
                <div className="p-3 rounded bg-[var(--surface-secondary)]">
                  <label className="text-sm text-[var(--text-primary)]">
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
                    className="mt-1 w-full px-3 py-2 rounded bg-[var(--surface-primary)] border border-[var(--border-primary)] text-sm text-[var(--text-primary)]"
                  >
                    <option value={1}>1 minute</option>
                    <option value={5}>5 minutes</option>
                    <option value={10}>10 minutes</option>
                    <option value={30}>30 minutes</option>
                  </select>
                </div>
              )}

              <label className="flex items-center justify-between p-3 rounded bg-[var(--surface-secondary)] cursor-pointer">
                <div>
                  <p className="text-sm text-[var(--text-primary)]">
                    Clear Data on Exit
                  </p>
                  <p className="text-xs text-[var(--text-muted)]">
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
              <h4 className="text-sm font-medium text-[var(--text-primary)] mb-2">
                Data Management
              </h4>

              <div className="p-3 rounded bg-[var(--surface-secondary)]">
                <p className="text-sm text-[var(--text-primary)]">
                  Session Data
                </p>
                <p className="text-xs text-[var(--text-muted)] mb-2">
                  Clear all stored session information
                </p>
                <button
                  onClick={() => toast.success("Session data cleared")}
                  className="px-3 py-1.5 rounded bg-red-500/10 text-red-400 text-sm hover:bg-red-500/20 transition-colors"
                >
                  Clear Sessions
                </button>
              </div>

              <div className="p-3 rounded bg-[var(--surface-secondary)]">
                <p className="text-sm text-[var(--text-primary)]">
                  Export Settings
                </p>
                <p className="text-xs text-[var(--text-muted)] mb-2">
                  Export your settings as JSON
                </p>
                <button
                  onClick={() => toast.success("Settings exported")}
                  className="px-3 py-1.5 rounded bg-[var(--color-primary)]/10 text-[var(--color-primary)] text-sm hover:bg-[var(--color-primary)]/20 transition-colors"
                >
                  Export Settings
                </button>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Footer */}
      <div className="flex justify-end gap-2 mt-4 pt-4 border-t border-[var(--border-primary)]">
        <button
          onClick={onClose}
          className="px-4 py-2 rounded text-sm text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] transition-colors"
        >
          Cancel
        </button>
        <button
          onClick={handleSave}
          className="flex items-center gap-2 px-4 py-2 rounded bg-[var(--color-primary)] text-white text-sm hover:bg-[var(--color-primary)]/90 transition-colors"
        >
          <Save size={14} />
          Save Changes
        </button>
      </div>
    </Modal>
  );
}
