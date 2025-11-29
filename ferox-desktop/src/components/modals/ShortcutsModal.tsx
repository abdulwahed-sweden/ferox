import { Modal } from "./Modal";
import { Keyboard } from "lucide-react";

interface ShortcutsModalProps {
  isOpen: boolean;
  onClose: () => void;
}

interface ShortcutGroup {
  title: string;
  shortcuts: { keys: string[]; description: string }[];
}

const shortcutGroups: ShortcutGroup[] = [
  {
    title: "General",
    shortcuts: [
      { keys: ["Cmd", "N"], description: "New Session" },
      { keys: ["Cmd", "O"], description: "Open Project" },
      { keys: ["Cmd", "S"], description: "Save Project" },
      { keys: ["Cmd", "E"], description: "Export Results" },
      { keys: ["Cmd", ","], description: "Settings" },
      { keys: ["Cmd", "Q"], description: "Quit Application" },
    ],
  },
  {
    title: "Navigation",
    shortcuts: [
      { keys: ["J"], description: "Next Session" },
      { keys: ["K"], description: "Previous Session" },
      { keys: ["Enter"], description: "Open Terminal" },
      { keys: ["/"], description: "Focus Search" },
      { keys: ["Esc"], description: "Clear Search / Close Modal" },
    ],
  },
  {
    title: "Tabs",
    shortcuts: [
      { keys: ["Cmd", "T"], description: "New Tab" },
      { keys: ["Cmd", "W"], description: "Close Tab" },
      { keys: ["Ctrl", "Tab"], description: "Next Tab" },
      { keys: ["Ctrl", "Shift", "Tab"], description: "Previous Tab" },
    ],
  },
  {
    title: "View",
    shortcuts: [
      { keys: ["Cmd", "B"], description: "Toggle Sidebar" },
      { keys: ["Cmd", "Shift", "T"], description: "Toggle Theme" },
      { keys: ["Cmd", "+"], description: "Zoom In" },
      { keys: ["Cmd", "-"], description: "Zoom Out" },
      { keys: ["Cmd", "0"], description: "Reset Zoom" },
      { keys: ["Cmd", "Ctrl", "F"], description: "Fullscreen" },
    ],
  },
  {
    title: "Help",
    shortcuts: [
      { keys: ["F1"], description: "Documentation" },
      { keys: ["Cmd", "Shift", "/"], description: "Show Shortcuts" },
    ],
  },
];

export function ShortcutsModal({ isOpen, onClose }: ShortcutsModalProps) {
  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title="Keyboard Shortcuts"
      size="lg"
    >
      <div className="flex items-center gap-2 mb-4 text-[var(--text-secondary)]">
        <Keyboard size={16} />
        <span className="text-sm">
          Use these shortcuts to navigate faster
        </span>
      </div>

      <div className="grid grid-cols-2 gap-6 max-h-96 overflow-y-auto pr-2">
        {shortcutGroups.map((group) => (
          <div key={group.title}>
            <h4 className="text-sm font-semibold text-[var(--text-primary)] mb-2">
              {group.title}
            </h4>
            <div className="space-y-1.5">
              {group.shortcuts.map((shortcut) => (
                <div
                  key={shortcut.description}
                  className="flex items-center justify-between text-xs"
                >
                  <span className="text-[var(--text-secondary)]">
                    {shortcut.description}
                  </span>
                  <div className="flex items-center gap-1">
                    {shortcut.keys.map((key, i) => (
                      <span key={i}>
                        <kbd className="px-1.5 py-0.5 rounded bg-[var(--surface-secondary)] border border-[var(--border-primary)] text-[var(--text-primary)] font-mono text-xs">
                          {key}
                        </kbd>
                        {i < shortcut.keys.length - 1 && (
                          <span className="text-[var(--text-muted)] mx-0.5">
                            +
                          </span>
                        )}
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>

      <div className="mt-4 pt-4 border-t border-[var(--border-primary)]">
        <p className="text-xs text-[var(--text-muted)] text-center">
          Press <kbd className="px-1 py-0.5 rounded bg-[var(--surface-secondary)] border border-[var(--border-primary)] text-[var(--text-primary)]">Esc</kbd> to close
        </p>
      </div>
    </Modal>
  );
}
