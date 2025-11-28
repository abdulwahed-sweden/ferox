import { useState, useCallback } from "react";
import { useAppStore } from "../store";
import {
  Terminal,
  FolderOpen,
  Activity,
  Network,
  X,
  Plus,
  Package,
  Radar,
  KeyRound,
  FileText,
  Clock,
  StickyNote,
  Crosshair,
  Globe,
  Grid3X3,
  ClipboardList,
  Eye,
} from "lucide-react";
import { clsx } from "clsx";
import type { TabType } from "../types";

const tabIcons: Record<TabType, typeof Terminal> = {
  terminal: Terminal,
  filebrowser: FolderOpen,
  processes: Activity,
  network: Network,
  payloads: Package,
  scanner: Radar,
  credentials: KeyRound,
  eventlog: FileText,
  scheduler: Clock,
  notes: StickyNote,
  postexploitation: Crosshair,
  networkmap: Globe,
  mitre: Grid3X3,
  reports: ClipboardList,
  opsec: Eye,
};

export function TabBar() {
  const {
    tabs,
    activeTabId,
    setActiveTab,
    closeTab,
    selectedSessionId,
    addTab,
    reorderTabs,
  } = useAppStore();

  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);

  const handleNewTab = () => {
    if (selectedSessionId) {
      addTab({
        id: `tab-${Date.now()}`,
        type: "terminal",
        sessionId: selectedSessionId,
        title: "New Terminal",
        icon: "terminal",
      });
    }
  };

  const handleDragStart = useCallback((e: React.DragEvent, index: number) => {
    setDraggedIndex(index);
    e.dataTransfer.effectAllowed = "move";
    e.dataTransfer.setData("text/plain", index.toString());
    // Add some transparency to the dragged element
    if (e.currentTarget instanceof HTMLElement) {
      e.currentTarget.style.opacity = "0.5";
    }
  }, []);

  const handleDragEnd = useCallback((e: React.DragEvent) => {
    setDraggedIndex(null);
    setDragOverIndex(null);
    if (e.currentTarget instanceof HTMLElement) {
      e.currentTarget.style.opacity = "1";
    }
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent, index: number) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
    setDragOverIndex(index);
  }, []);

  const handleDragLeave = useCallback(() => {
    setDragOverIndex(null);
  }, []);

  const handleDrop = useCallback(
    (e: React.DragEvent, toIndex: number) => {
      e.preventDefault();
      const fromIndex = parseInt(e.dataTransfer.getData("text/plain"), 10);

      if (fromIndex !== toIndex && !isNaN(fromIndex)) {
        reorderTabs(fromIndex, toIndex);
      }

      setDraggedIndex(null);
      setDragOverIndex(null);
    },
    [reorderTabs],
  );

  if (tabs.length === 0) {
    return (
      <div className="h-9 bg-dark-800 border-b border-dark-600 flex items-center px-2">
        <span className="text-sm text-text-muted">No tabs open</span>
      </div>
    );
  }

  return (
    <div className="h-9 bg-dark-800 border-b border-dark-600 flex items-center">
      {/* Tabs */}
      <div className="flex-1 flex items-center overflow-x-auto">
        {tabs.map((tab, index) => {
          const Icon = tabIcons[tab.type];
          const isActive = tab.id === activeTabId;
          const isDragging = draggedIndex === index;
          const isDragOver = dragOverIndex === index && draggedIndex !== index;

          return (
            <div
              key={tab.id}
              draggable
              onDragStart={(e) => handleDragStart(e, index)}
              onDragEnd={handleDragEnd}
              onDragOver={(e) => handleDragOver(e, index)}
              onDragLeave={handleDragLeave}
              onDrop={(e) => handleDrop(e, index)}
              className={clsx(
                "group flex items-center gap-2 px-3 h-full border-r border-dark-600 cursor-pointer transition-all min-w-[120px] max-w-[200px]",
                isActive
                  ? "bg-dark-700 text-text-primary border-b-2 border-b-ferox-green"
                  : "text-text-secondary hover:bg-dark-700 hover:text-text-primary",
                isDragging && "opacity-50",
                isDragOver && "border-l-2 border-l-ferox-green",
              )}
              onClick={() => setActiveTab(tab.id)}
            >
              <Icon size={14} className="flex-shrink-0" />
              <span className="text-sm truncate flex-1">{tab.title}</span>
              <button
                className={clsx(
                  "p-0.5 rounded hover:bg-dark-500 transition-colors",
                  "opacity-0 group-hover:opacity-100",
                  isActive && "opacity-100",
                )}
                onClick={(e) => {
                  e.stopPropagation();
                  closeTab(tab.id);
                }}
              >
                <X size={12} />
              </button>
            </div>
          );
        })}
      </div>

      {/* New tab button */}
      <button
        className="h-full px-3 text-text-muted hover:text-text-primary hover:bg-dark-700 transition-colors border-l border-dark-600"
        onClick={handleNewTab}
        disabled={!selectedSessionId}
        title="New terminal tab"
      >
        <Plus size={16} />
      </button>
    </div>
  );
}
