// ferox-desktop/src/components/modules/opsec/InjectionPanel.tsx
// Process Injection Panel

import { useState } from "react";
import {
  Syringe,
  Search,
  Target,
  CheckCircle,
  XCircle,
  AlertTriangle,
} from "lucide-react";
import { useOpsec } from "../../../hooks/useOpsec";
import type {
  InjectionTechnique,
  InjectionResult,
  TargetProcess,
} from "../../../types/opsec";

const INJECTION_TECHNIQUES: {
  id: InjectionTechnique;
  name: string;
  description: string;
  mitre: string;
  stealth: number;
  complexity: "low" | "medium" | "high";
}[] = [
  {
    id: "ClassicRemoteThread",
    name: "Classic Remote Thread",
    description: "CreateRemoteThread injection - widely detected",
    mitre: "T1055.002",
    stealth: 3,
    complexity: "low",
  },
  {
    id: "NtCreateThreadEx",
    name: "NtCreateThreadEx",
    description: "Direct syscall for thread creation - less monitored",
    mitre: "T1055.002",
    stealth: 6,
    complexity: "medium",
  },
  {
    id: "QueueUserApc",
    name: "APC Queue Injection",
    description: "Queue APC to alertable thread - stealthy",
    mitre: "T1055.004",
    stealth: 7,
    complexity: "medium",
  },
  {
    id: "EarlyBird",
    name: "Early Bird",
    description: "APC injection to suspended process - very stealthy",
    mitre: "T1055.004",
    stealth: 8,
    complexity: "high",
  },
  {
    id: "ThreadHijack",
    name: "Thread Hijack",
    description: "Hijack existing thread context - no new threads",
    mitre: "T1055.003",
    stealth: 8,
    complexity: "high",
  },
  {
    id: "ProcessHollowing",
    name: "Process Hollowing",
    description: "Replace process memory - appears legitimate",
    mitre: "T1055.012",
    stealth: 9,
    complexity: "high",
  },
  {
    id: "ModuleStomping",
    name: "Module Stomping",
    description: "Overwrite legitimate DLL in memory",
    mitre: "T1055.001",
    stealth: 9,
    complexity: "high",
  },
  {
    id: "DirectSyscall",
    name: "Direct Syscall",
    description: "Bypass userland hooks with direct syscalls",
    mitre: "T1055",
    stealth: 10,
    complexity: "high",
  },
];

const TARGET_CRITERIA = [
  {
    id: "SystemProcess",
    label: "System Process",
    description: "svchost.exe, RuntimeBroker.exe",
  },
  {
    id: "BrowserProcess",
    label: "Browser",
    description: "chrome.exe, firefox.exe, msedge.exe",
  },
  {
    id: "SignedMicrosoft",
    label: "Signed Microsoft",
    description: "Any Microsoft-signed binary",
  },
  { id: "ByName", label: "Custom", description: "Specify process name" },
];

export function InjectionPanel() {
  const { findTargets, inject, loading } = useOpsec();
  const [selectedTechnique, setSelectedTechnique] =
    useState<InjectionTechnique>("QueueUserApc");
  const [targetCriteria, setTargetCriteria] = useState("SystemProcess");
  const [customTarget, setCustomTarget] = useState("");
  const [targets, setTargets] = useState<TargetProcess[]>([]);
  const [selectedTarget, setSelectedTarget] = useState<TargetProcess | null>(
    null,
  );
  const [result, setResult] = useState<InjectionResult | null>(null);
  const [shellcode, setShellcode] = useState("");

  const handleFindTargets = async () => {
    try {
      const criteria =
        targetCriteria === "ByName" ? customTarget : targetCriteria;
      const found = await findTargets(criteria);
      setTargets(found);
      setSelectedTarget(null);
    } catch (e) {
      console.error("Failed to find targets:", e);
    }
  };

  const handleInject = async () => {
    if (!selectedTarget) return;
    try {
      const injResult = await inject({
        technique: selectedTechnique,
        targetPid: selectedTarget.pid,
        shellcode,
      });
      setResult(injResult);
    } catch (e) {
      console.error("Injection failed:", e);
    }
  };

  // Find technique for reference
  void INJECTION_TECHNIQUES.find((t) => t.id === selectedTechnique);

  return (
    <div className="space-y-6">
      {/* Technique Selection */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h3 className="text-lg font-semibold mb-4">Injection Technique</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
          {INJECTION_TECHNIQUES.map((tech) => (
            <button
              key={tech.id}
              onClick={() => setSelectedTechnique(tech.id)}
              className={`p-3 rounded-lg border text-left transition-colors ${
                selectedTechnique === tech.id
                  ? "border-cyan-400 bg-cyan-400/10"
                  : "border-dark-600 hover:border-dark-500"
              }`}
            >
              <div className="flex items-center justify-between mb-1">
                <span className="font-medium text-sm">{tech.name}</span>
                <span className="text-xs text-text-muted">{tech.mitre}</span>
              </div>
              <p className="text-xs text-text-muted mb-2">{tech.description}</p>
              <div className="flex items-center gap-2">
                <div className="flex-1">
                  <div className="flex items-center justify-between text-xs mb-0.5">
                    <span className="text-text-muted">Stealth</span>
                    <span>{tech.stealth}/10</span>
                  </div>
                  <div className="h-1 bg-dark-700 rounded-full overflow-hidden">
                    <div
                      className="h-full bg-cyan-400 rounded-full"
                      style={{ width: `${tech.stealth * 10}%` }}
                    />
                  </div>
                </div>
                <span
                  className={`px-2 py-0.5 rounded text-xs ${
                    tech.complexity === "low"
                      ? "bg-success-soft text-success-text"
                      : tech.complexity === "medium"
                        ? "bg-warning-soft text-warning-text"
                        : "bg-danger-soft text-danger-text"
                  }`}
                >
                  {tech.complexity}
                </span>
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Target Selection */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h3 className="text-lg font-semibold mb-4">Target Selection</h3>

        {/* Criteria */}
        <div className="flex gap-2 mb-4">
          {TARGET_CRITERIA.map((criteria) => (
            <button
              key={criteria.id}
              onClick={() => setTargetCriteria(criteria.id)}
              className={`px-3 py-2 rounded-lg text-sm transition-colors ${
                targetCriteria === criteria.id
                  ? "bg-cyan-600 text-white"
                  : "bg-dark-700 hover:bg-dark-600"
              }`}
            >
              {criteria.label}
            </button>
          ))}
        </div>

        {/* Custom Target Input */}
        {targetCriteria === "ByName" && (
          <input
            type="text"
            value={customTarget}
            onChange={(e) => setCustomTarget(e.target.value)}
            placeholder="Process name (e.g., notepad.exe)"
            className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded-lg mb-4
              focus:outline-none focus:border-cyan-400"
          />
        )}

        {/* Find Targets Button */}
        <button
          onClick={handleFindTargets}
          disabled={loading}
          className="w-full flex items-center justify-center gap-2 px-4 py-2 bg-dark-700 hover:bg-dark-600
            rounded-lg font-medium transition-colors border border-dark-600 mb-4"
        >
          <Search className="w-4 h-4" />
          {loading ? "Searching..." : "Find Targets"}
        </button>

        {/* Target List */}
        {targets.length > 0 && (
          <div className="space-y-2 max-h-48 overflow-auto">
            {targets.map((target) => (
              <button
                key={target.pid}
                onClick={() => setSelectedTarget(target)}
                className={`w-full p-3 rounded-lg border text-left transition-colors ${
                  selectedTarget?.pid === target.pid
                    ? "border-cyan-400 bg-cyan-400/10"
                    : "border-dark-600 hover:border-dark-500"
                }`}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Target className="w-4 h-4 text-text-muted" />
                    <span className="font-medium">{target.name}</span>
                    <span className="text-xs text-text-muted">
                      PID: {target.pid}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-text-muted">
                      {target.is64bit ? "x64" : "x86"}
                    </span>
                    <span
                      className={`px-2 py-0.5 rounded text-xs ${
                        target.suitability > 7
                          ? "bg-success-soft text-success-text"
                          : target.suitability > 4
                            ? "bg-warning-soft text-warning-text"
                            : "bg-danger-soft text-danger-text"
                      }`}
                    >
                      Score: {target.suitability}/10
                    </span>
                  </div>
                </div>
                {target.path && (
                  <p className="text-xs text-text-muted mt-1 truncate">
                    {target.path}
                  </p>
                )}
              </button>
            ))}
          </div>
        )}
      </div>

      {/* Shellcode Input */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h3 className="text-lg font-semibold mb-4">Shellcode (Hex)</h3>
        <textarea
          value={shellcode}
          onChange={(e) => setShellcode(e.target.value)}
          placeholder="\\x90\\x90\\x90... or paste raw hex"
          rows={4}
          className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded-lg font-mono text-sm
            focus:outline-none focus:border-cyan-400 resize-none"
        />
        <p className="text-xs text-text-muted mt-2">
          {shellcode.length > 0
            ? `${Math.floor(shellcode.replace(/\\x/g, "").length / 2)} bytes`
            : "Enter shellcode in hex format"}
        </p>
      </div>

      {/* Execute Button */}
      <button
        onClick={handleInject}
        disabled={loading || !selectedTarget || !shellcode}
        className="w-full flex items-center justify-center gap-2 px-4 py-3 bg-cyan-600 hover:bg-cyan-500
          rounded-lg font-medium disabled:opacity-50 transition-colors"
      >
        <Syringe className="w-4 h-4" />
        {loading ? "Injecting..." : "Execute Injection"}
      </button>

      {/* Result */}
      {result && (
        <div
          className={`rounded-lg p-4 border ${
            result.success
              ? "bg-success-soft border-green-400/30"
              : "bg-danger-soft border-red-400/30"
          }`}
        >
          <div className="flex items-center gap-2 mb-2">
            {result.success ? (
              <CheckCircle className="w-5 h-5 text-success-text" />
            ) : (
              <XCircle className="w-5 h-5 text-danger-text" />
            )}
            <span className="font-medium">
              {result.success ? "Injection Successful" : "Injection Failed"}
            </span>
          </div>
          <p className="text-sm text-text-secondary">{result.message}</p>
          {result.threadId && (
            <p className="text-xs text-text-muted mt-1">
              Thread ID: {result.threadId}
            </p>
          )}
        </div>
      )}

      {/* Warning */}
      <div className="bg-warning-soft border border-yellow-400/30 rounded-lg p-4">
        <div className="flex items-start gap-3">
          <AlertTriangle className="w-5 h-5 text-warning-text mt-0.5" />
          <div>
            <p className="font-medium text-warning-text">Security Notice</p>
            <p className="text-sm text-text-secondary mt-1">
              Process injection is a monitored technique. Use appropriate
              stealth level and avoid injecting into protected processes like
              lsass.exe.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
