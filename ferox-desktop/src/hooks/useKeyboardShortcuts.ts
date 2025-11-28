import { useEffect, useCallback, useRef } from "react";
import { useAppStore } from "../store";
import { createTerminal } from "../lib/tauri";
import { terminalToasts, errorToasts } from "../lib/toast";
import type { Tab } from "../types";

/**
 * Keyboard shortcuts hook for global navigation
 *
 * Shortcuts:
 * - j/k: Navigate session tree down/up
 * - Enter: Open terminal for selected session
 * - Ctrl+Tab / Ctrl+Shift+Tab: Cycle tabs forward/backward
 * - Ctrl+W: Close current tab
 * - /: Focus search bar
 * - Escape: Clear search / unfocus
 */
export function useKeyboardShortcuts() {
  const {
    selectedSessionId,
    sessions,
    tabs,
    selectNextSession,
    selectPrevSession,
    cycleTabForward,
    cycleTabBackward,
    closeActiveTab,
    addTab,
    setSearchQuery,
    setSearchInputFocused,
  } = useAppStore();

  // Ref for search input element
  const searchInputRef = useRef<HTMLInputElement | null>(null);

  // Open terminal for selected session
  const openTerminalForSelectedSession = useCallback(async () => {
    if (!selectedSessionId) return;

    const session = sessions.find((s) => s.id === selectedSessionId);
    if (!session) return;

    // Check if tab already exists for this session
    const existingTab = tabs.find((t) => t.sessionId === selectedSessionId);
    if (existingTab) {
      useAppStore.getState().setActiveTab(existingTab.id);
      return;
    }

    try {
      const response = await createTerminal({
        session_id: selectedSessionId,
        rows: 24,
        cols: 80,
      });

      const newTab: Tab = {
        id: response.terminal_id,
        sessionId: selectedSessionId,
        title: `${session.hostname} - Terminal`,
        type: "terminal",
        icon: "terminal",
      };

      addTab(newTab);
      terminalToasts.created(session.hostname);
    } catch (error) {
      console.error("Failed to create terminal:", error);
      errorToasts.generic("Failed to create terminal");
    }
  }, [selectedSessionId, sessions, tabs, addTab]);

  // Main keyboard handler
  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      const target = event.target as HTMLElement;
      const isInputFocused =
        target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.isContentEditable;

      // If typing in terminal or input, only handle Escape
      if (isInputFocused && event.key !== "Escape") {
        return;
      }

      // Ctrl+Tab: Cycle tabs forward
      if (event.ctrlKey && event.key === "Tab" && !event.shiftKey) {
        event.preventDefault();
        cycleTabForward();
        return;
      }

      // Ctrl+Shift+Tab: Cycle tabs backward
      if (event.ctrlKey && event.shiftKey && event.key === "Tab") {
        event.preventDefault();
        cycleTabBackward();
        return;
      }

      // Ctrl+W: Close active tab
      if (event.ctrlKey && event.key === "w") {
        event.preventDefault();
        closeActiveTab();
        return;
      }

      // Skip other shortcuts if input is focused
      if (isInputFocused) {
        if (event.key === "Escape") {
          (target as HTMLInputElement).blur();
          setSearchQuery("");
          setSearchInputFocused(false);
        }
        return;
      }

      switch (event.key) {
        // j: Navigate down in session tree
        case "j":
          event.preventDefault();
          selectNextSession();
          break;

        // k: Navigate up in session tree
        case "k":
          event.preventDefault();
          selectPrevSession();
          break;

        // Enter: Open terminal for selected session
        case "Enter":
          event.preventDefault();
          openTerminalForSelectedSession();
          break;

        // /: Focus search
        case "/":
          event.preventDefault();
          if (searchInputRef.current) {
            searchInputRef.current.focus();
            setSearchInputFocused(true);
          }
          break;

        // Escape: Clear and unfocus
        case "Escape":
          setSearchQuery("");
          setSearchInputFocused(false);
          break;
      }
    },
    [
      cycleTabForward,
      cycleTabBackward,
      closeActiveTab,
      selectNextSession,
      selectPrevSession,
      openTerminalForSelectedSession,
      setSearchQuery,
      setSearchInputFocused,
    ],
  );

  // Register global keyboard listener
  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [handleKeyDown]);

  // Return ref for search input
  return { searchInputRef };
}
