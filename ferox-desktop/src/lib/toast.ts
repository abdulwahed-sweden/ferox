/**
 * Toast Notification Utilities
 *
 * Centralized toast functions for consistent notifications across the app.
 */

import toast from "react-hot-toast";

// Session Events
export const sessionToasts = {
  created: (hostname: string, ip: string) =>
    toast.success(`New session: ${hostname} (${ip})`, {
      duration: 4000,
      icon: "🎯",
    }),

  died: (hostname: string) =>
    toast.error(`Session died: ${hostname}`, {
      duration: 5000,
      icon: "💀",
    }),

  privilegeEscalated: (hostname: string, newPrivilege: string) =>
    toast.success(`Privilege escalated on ${hostname}: ${newPrivilege}`, {
      duration: 4000,
      icon: "🔓",
    }),

  terminated: (hostname: string) =>
    toast(`Session terminated: ${hostname}`, {
      duration: 3000,
      icon: "🔴",
    }),

  sleeping: (hostname: string) =>
    toast(`Session sleeping: ${hostname}`, {
      duration: 3000,
      icon: "💤",
    }),
};

// Command Execution
export const commandToasts = {
  executing: (command: string) => {
    const truncated =
      command.length > 30 ? `${command.slice(0, 30)}...` : command;
    return toast.loading(`Executing: ${truncated}`, {
      duration: Infinity,
    });
  },

  success: (toastId: string) => {
    toast.dismiss(toastId);
    toast.success("Command completed", {
      duration: 3000,
      icon: "✅",
    });
  },

  failed: (toastId: string, error: string) => {
    toast.dismiss(toastId);
    toast.error(`Command failed: ${error}`, {
      duration: 5000,
      icon: "❌",
    });
  },
};

// Module Operations
export const moduleToasts = {
  privescStarted: () =>
    toast.loading("Running privilege escalation scan...", {
      duration: Infinity,
    }),

  privescComplete: (
    toastId: string,
    vectorsFound: number,
    escalated: boolean,
  ) => {
    toast.dismiss(toastId);
    if (escalated) {
      toast.success("Privilege escalation successful!", {
        duration: 4000,
        icon: "🔓",
      });
    } else if (vectorsFound > 0) {
      toast.success(
        `Found ${vectorsFound} potential vector${vectorsFound > 1 ? "s" : ""}`,
        {
          duration: 4000,
          icon: "🔍",
        },
      );
    } else {
      toast("No escalation vectors found", {
        duration: 3000,
        icon: "🔒",
      });
    }
  },

  credentialHarvest: (count: number) => {
    if (count > 0) {
      toast.success(`Harvested ${count} credential${count > 1 ? "s" : ""}`, {
        duration: 4000,
        icon: "🔑",
      });
    } else {
      toast("No credentials found", {
        duration: 3000,
        icon: "🔍",
      });
    }
  },

  persistenceInstalled: (method: string, success: boolean) => {
    if (success) {
      toast.success(`Persistence installed via ${method}`, {
        duration: 4000,
        icon: "📌",
      });
    } else {
      toast.error(`Persistence failed: ${method}`, {
        duration: 5000,
        icon: "❌",
      });
    }
  },

  lateralMoveStarted: (target: string) =>
    toast.loading(`Moving to ${target}...`, {
      duration: Infinity,
    }),

  lateralMoveComplete: (toastId: string, target: string, success: boolean) => {
    toast.dismiss(toastId);
    if (success) {
      toast.success(`Established session on ${target}`, {
        duration: 4000,
        icon: "🚀",
      });
    } else {
      toast.error(`Lateral movement to ${target} failed`, {
        duration: 5000,
        icon: "❌",
      });
    }
  },

  discoveryComplete: (hostsFound: number) => {
    toast.success(
      `Discovery complete: ${hostsFound} host${hostsFound !== 1 ? "s" : ""} found`,
      {
        duration: 4000,
        icon: "🌐",
      },
    );
  },
};

// Terminal Events
export const terminalToasts = {
  created: (sessionName: string) =>
    toast.success(`Terminal opened: ${sessionName}`, {
      duration: 2000,
      icon: "💻",
    }),

  closed: () =>
    toast("Terminal closed", {
      duration: 2000,
      icon: "🔴",
    }),

  error: (message: string) =>
    toast.error(`Terminal error: ${message}`, {
      duration: 4000,
      icon: "⚠️",
    }),
};

// Generic Errors
export const errorToasts = {
  connection: (message: string) =>
    toast.error(`Connection error: ${message}`, {
      duration: 5000,
      icon: "🔌",
    }),

  validation: (message: string) =>
    toast.error(message, {
      duration: 4000,
      icon: "⚠️",
    }),

  generic: (message: string) =>
    toast.error(message, {
      duration: 4000,
      icon: "❌",
    }),
};

// Success Messages
export const successToasts = {
  generic: (message: string) =>
    toast.success(message, {
      duration: 3000,
      icon: "✅",
    }),

  copied: () =>
    toast.success("Copied to clipboard", {
      duration: 2000,
      icon: "📋",
    }),
};
