// src/components/cli/QuickCommands.tsx
// Quick commands widget with search and copy functionality

import { useState } from "react";
import { Copy, Check, Search, Terminal } from "lucide-react";
import { CLI_COMMANDS, searchCommands } from "@/data/cli-commands";

interface QuickCommandsProps {
  className?: string;
}

export function QuickCommands({ className = "" }: QuickCommandsProps) {
  const [search, setSearch] = useState("");
  const [copied, setCopied] = useState<string | null>(null);

  const filteredCommands = search
    ? searchCommands(search)
    : CLI_COMMANDS.slice(0, 6);

  const copyCommand = async (command: string, id: string) => {
    await navigator.clipboard.writeText(command);
    setCopied(id);
    setTimeout(() => setCopied(null), 2000);
  };

  return (
    <div className={`scenario-card ${className}`}>
      {/* Header */}
      <div className="scenario-header">
        <div className="scenario-header-main">
          <Terminal className="w-5 h-5 text-primary-text" />
          <h3 className="scenario-title">Quick Commands</h3>
        </div>
      </div>

      {/* Search */}
      <div className="p-4 border-b border-border-subtle">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-content-tertiary" />
          <input
            type="text"
            placeholder="Search commands..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full pl-10 pr-4 py-2.5 rounded-lg bg-surface-input border border-border-default
                       text-sm text-content-primary placeholder:text-content-tertiary
                       focus:outline-none focus:ring-2 focus:ring-primary-new/50 focus:border-border-focus"
          />
        </div>
      </div>

      {/* Commands List */}
      <div className="divide-y divide-border-subtle">
        {filteredCommands.length === 0 ? (
          <div className="p-8 text-center text-content-secondary">
            <Terminal className="w-8 h-8 mx-auto mb-2 opacity-50" />
            <p>No commands found</p>
          </div>
        ) : (
          filteredCommands.map((cmd) => (
            <div
              key={cmd.id}
              className="p-4 hover:bg-surface-hover transition-colors"
            >
              <div className="flex items-start justify-between gap-4">
                <div className="flex-1 min-w-0">
                  <code className="font-mono text-sm font-semibold text-primary-text">
                    {cmd.command}
                  </code>
                  <p className="mt-1 text-xs text-content-secondary line-clamp-1">
                    {cmd.description}
                  </p>
                  <div className="mt-2 flex flex-wrap gap-1">
                    {cmd.tags.slice(0, 3).map((tag) => (
                      <span
                        key={tag}
                        className="text-xs px-1.5 py-0.5 rounded bg-surface-hover text-content-tertiary"
                      >
                        {tag}
                      </span>
                    ))}
                  </div>
                </div>
                <button
                  onClick={() =>
                    copyCommand(cmd.examples[0]?.command || cmd.command, cmd.id)
                  }
                  className="flex-shrink-0 p-2 rounded-lg hover:bg-surface-active transition-colors"
                  title="Copy command"
                >
                  {copied === cmd.id ? (
                    <Check className="w-4 h-4 text-success-text" />
                  ) : (
                    <Copy className="w-4 h-4 text-content-tertiary" />
                  )}
                </button>
              </div>
            </div>
          ))
        )}
      </div>

      {/* View All Link */}
      <div className="p-4 border-t border-border-subtle bg-surface-hover text-center">
        <button className="text-sm text-primary-text hover:underline font-medium">
          View All Commands
        </button>
      </div>
    </div>
  );
}

export default QuickCommands;
