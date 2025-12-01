// src/components/cli/ModuleSimulationPanel.tsx
// Module simulation panel with CLI commands display

import { useState } from "react";
import { Play, Code, Settings, AlertTriangle } from "lucide-react";
import { CLICommandDisplay } from "./CLICommandDisplay";
import { getCommandByModule } from "@/data/cli-commands";
import { InfoBox, WarningBox } from "@/components/ui";

interface ModuleSimulationPanelProps {
  moduleId: string;
  moduleName: string;
  isSimulation?: boolean;
  onExecute?: (params: Record<string, unknown>) => void;
  className?: string;
}

type TabId = "run" | "cli" | "options";

export function ModuleSimulationPanel({
  moduleId,
  moduleName,
  isSimulation = true,
  className = "",
}: ModuleSimulationPanelProps) {
  const [activeTab, setActiveTab] = useState<TabId>("cli");
  const command = getCommandByModule(moduleId);

  const tabs = [
    { id: "run" as const, label: "Run", icon: Play },
    { id: "cli" as const, label: "CLI Commands", icon: Code },
    { id: "options" as const, label: "Options", icon: Settings },
  ];

  return (
    <div className={`scenario-card ${className}`}>
      {/* Header */}
      <div className="scenario-header">
        <div className="scenario-header-main">
          <h3 className="scenario-title">{moduleName}</h3>
        </div>
        {isSimulation && (
          <span className="scenario-status status-in-progress">
            <AlertTriangle size={12} />
            Simulation Mode
          </span>
        )}
      </div>

      {/* Tabs */}
      <div className="flex border-b border-border-subtle">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`flex items-center gap-2 px-6 py-3 text-sm font-medium transition-colors ${
              activeTab === tab.id
                ? "bg-primary-soft text-primary-text border-b-2 border-primary-new"
                : "text-content-secondary hover:text-content-primary hover:bg-surface-hover"
            }`}
          >
            <tab.icon className="w-4 h-4" />
            {tab.label}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="scenario-content">
        {activeTab === "cli" && command && (
          <div className="space-y-6">
            <InfoBox title="CLI Reference">
              These commands work in the Ferox CLI. Copy and paste them directly
              into your terminal for real execution.
            </InfoBox>

            <CLICommandDisplay command={command} showExamples={true} />
          </div>
        )}

        {activeTab === "cli" && !command && (
          <div className="text-center py-12 text-content-secondary">
            <Code className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>No CLI commands available for this module yet.</p>
          </div>
        )}

        {activeTab === "run" && (
          <div className="space-y-6">
            {isSimulation && (
              <WarningBox title="Simulation Mode Active">
                This module is running in simulation mode. No actual commands
                will be executed. Switch to the CLI Commands tab to see the real
                commands you can use.
              </WarningBox>
            )}

            {/* Module execution UI placeholder */}
            <div className="rounded-xl bg-surface-hover p-8 text-center">
              <Play className="w-12 h-12 mx-auto mb-4 text-content-tertiary" />
              <p className="text-content-secondary">
                Module execution interface
              </p>
            </div>
          </div>
        )}

        {activeTab === "options" && command && (
          <div className="space-y-4">
            <h4 className="font-mono text-sm font-semibold text-content-primary">
              Available Options
            </h4>
            <div className="data-table-container">
              <table className="data-table">
                <thead>
                  <tr>
                    <th>Flag</th>
                    <th>Type</th>
                    <th>Required</th>
                    <th>Description</th>
                  </tr>
                </thead>
                <tbody>
                  {command.parameters.map((param) => (
                    <tr key={param.name}>
                      <td className="font-mono text-primary-text">
                        {param.flag}
                      </td>
                      <td className="text-content-secondary">{param.type}</td>
                      <td>
                        {param.required ? (
                          <span className="text-danger-text">Yes</span>
                        ) : (
                          <span className="text-content-tertiary">No</span>
                        )}
                      </td>
                      <td className="text-content-primary">
                        {param.description}
                        {param.options && (
                          <span className="block text-xs text-content-tertiary mt-1">
                            Options: {param.options.join(", ")}
                          </span>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {activeTab === "options" && !command && (
          <div className="text-center py-12 text-content-secondary">
            <Settings className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>No options available for this module.</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default ModuleSimulationPanel;
