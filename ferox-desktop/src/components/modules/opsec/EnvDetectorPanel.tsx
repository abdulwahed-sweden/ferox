// ferox-desktop/src/components/modules/opsec/EnvDetectorPanel.tsx
// Environment Detection Panel (VM/Sandbox)

import { useState } from "react";
import {
  Search,
  AlertTriangle,
  CheckCircle,
  XCircle,
  Clock,
  Info,
  Shield,
} from "lucide-react";
import { useOpsec } from "../../../hooks/useOpsec";
import type { EnvironmentReport } from "../../../types/opsec";

const VM_SIGNATURES = [
  {
    type: "VMware",
    indicators: ["vmtools.exe", "vmware SVGA", "VMware MAC prefix"],
  },
  {
    type: "VirtualBox",
    indicators: ["VBoxService.exe", "VBox MAC prefix", "ACPI tables"],
  },
  { type: "HyperV", indicators: ["vmicheartbeat", "Hyper-V drivers", "vmbus"] },
  {
    type: "QEMU",
    indicators: ["QEMU Guest Agent", "virtio drivers", "BOCHS BIOS"],
  },
  { type: "KVM", indicators: ["QEMU/KVM CPU", "virtio devices", "kvmclock"] },
];

const SANDBOX_SIGNATURES = [
  { type: "Cuckoo", indicators: ["agent.py", "cuckoomon", "Python hooks"] },
  {
    type: "Joe Sandbox",
    indicators: ["joeboxserver", "analysis hooks", "report collection"],
  },
  {
    type: "Any.Run",
    indicators: ["anyrun_agent", "interactive analysis", "recording"],
  },
  {
    type: "VirusTotal",
    indicators: ["vt_scan_agent", "multi-av scan", "hash submission"],
  },
];

export function EnvDetectorPanel() {
  const { scanEnvironment, loading } = useOpsec();
  const [report, setReport] = useState<EnvironmentReport | null>(null);

  const handleScan = async () => {
    try {
      const result = await scanEnvironment();
      setReport(result);
    } catch (e) {
      console.error("Environment scan failed:", e);
    }
  };

  const getSuspicionColor = (score: number) => {
    if (score > 0.7) return "text-danger-text bg-danger-soft";
    if (score > 0.4) return "text-warning-text bg-warning-soft";
    return "text-success-text bg-success-soft";
  };

  return (
    <div className="space-y-6">
      {/* Scan Section */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h3 className="text-lg font-semibold">Environment Detection</h3>
            <p className="text-sm text-text-muted">
              VM/Sandbox detection (MITRE T1497)
            </p>
          </div>
        </div>

        <button
          onClick={handleScan}
          disabled={loading}
          className="w-full flex items-center justify-center gap-2 px-4 py-3 bg-cyan-600 hover:bg-cyan-500
            rounded-lg font-medium disabled:opacity-50 transition-colors"
        >
          <Search className="w-4 h-4" />
          {loading ? "Scanning Environment..." : "Scan Environment"}
        </button>
      </div>

      {/* Results */}
      {report && (
        <>
          {/* Summary Cards */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {/* Suspicion Score */}
            <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
              <h4 className="text-sm text-text-muted mb-2">Suspicion Score</h4>
              <div className="flex items-center gap-3">
                <div
                  className={`text-3xl font-bold ${getSuspicionColor(report.suspicionScore)}`}
                >
                  {(report.suspicionScore * 100).toFixed(0)}%
                </div>
                <div
                  className={`px-2 py-1 rounded text-xs font-medium ${getSuspicionColor(
                    report.suspicionScore,
                  )}`}
                >
                  {report.suspicionScore > 0.7
                    ? "High Risk"
                    : report.suspicionScore > 0.4
                      ? "Medium Risk"
                      : "Low Risk"}
                </div>
              </div>
            </div>

            {/* VM Detection */}
            <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
              <h4 className="text-sm text-text-muted mb-2">VM Detection</h4>
              <div className="flex items-center gap-2">
                {report.detectedVm ? (
                  <>
                    <AlertTriangle className="w-5 h-5 text-warning-text" />
                    <span className="text-lg font-medium text-warning-text">
                      {report.detectedVm}
                    </span>
                  </>
                ) : (
                  <>
                    <CheckCircle className="w-5 h-5 text-success-text" />
                    <span className="text-lg font-medium text-success-text">
                      No VM Detected
                    </span>
                  </>
                )}
              </div>
            </div>

            {/* Sandbox Detection */}
            <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
              <h4 className="text-sm text-text-muted mb-2">
                Sandbox Detection
              </h4>
              <div className="flex items-center gap-2">
                {report.detectedSandbox ? (
                  <>
                    <XCircle className="w-5 h-5 text-danger-text" />
                    <span className="text-lg font-medium text-danger-text">
                      {report.detectedSandbox}
                    </span>
                  </>
                ) : (
                  <>
                    <CheckCircle className="w-5 h-5 text-success-text" />
                    <span className="text-lg font-medium text-success-text">
                      No Sandbox Detected
                    </span>
                  </>
                )}
              </div>
            </div>
          </div>

          {/* Safety Status */}
          <div
            className={`rounded-lg p-4 border ${
              report.isSafeToExecute
                ? "bg-success-soft border-green-400/30"
                : "bg-danger-soft border-red-400/30"
            }`}
          >
            <div className="flex items-center gap-3">
              {report.isSafeToExecute ? (
                <Shield className="w-6 h-6 text-success-text" />
              ) : (
                <AlertTriangle className="w-6 h-6 text-danger-text" />
              )}
              <div>
                <p
                  className={`font-medium ${
                    report.isSafeToExecute
                      ? "text-success-text"
                      : "text-danger-text"
                  }`}
                >
                  {report.isSafeToExecute
                    ? "Environment is Safe for Operation"
                    : "Environment NOT Safe - Analysis Detected"}
                </p>
                <p className="text-sm text-text-secondary mt-1">
                  {report.isSafeToExecute
                    ? "No indicators of analysis environment detected"
                    : "Consider aborting or using maximum stealth"}
                </p>
              </div>
            </div>
          </div>

          {/* Analysis Tools */}
          {report.analysisTools.length > 0 && (
            <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
              <h4 className="font-medium mb-3 flex items-center gap-2">
                <AlertTriangle className="w-4 h-4 text-warning-text" />
                Analysis Tools Detected
              </h4>
              <div className="flex flex-wrap gap-2">
                {report.analysisTools.map((tool, i) => (
                  <span
                    key={i}
                    className="px-3 py-1 bg-warning-soft text-warning-text rounded-lg text-sm"
                  >
                    {tool}
                  </span>
                ))}
              </div>
            </div>
          )}

          {/* Timing Anomalies */}
          {report.timingAnomalies.length > 0 && (
            <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
              <h4 className="font-medium mb-3 flex items-center gap-2">
                <Clock className="w-4 h-4 text-warning-text" />
                Timing Anomalies
              </h4>
              <ul className="space-y-2">
                {report.timingAnomalies.map((anomaly, i) => (
                  <li
                    key={i}
                    className="flex items-center gap-2 text-sm text-text-secondary"
                  >
                    <span className="w-1 h-1 bg-warning-text rounded-full" />
                    {anomaly}
                  </li>
                ))}
              </ul>
            </div>
          )}

          {/* Recommendations */}
          {report.recommendations.length > 0 && (
            <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
              <h4 className="font-medium mb-3 flex items-center gap-2">
                <Info className="w-4 h-4 text-cyan-400" />
                Recommendations
              </h4>
              <ul className="space-y-2">
                {report.recommendations.map((rec, i) => (
                  <li
                    key={i}
                    className="flex items-center gap-2 text-sm text-text-secondary"
                  >
                    <span className="w-1 h-1 bg-cyan-400 rounded-full" />
                    {rec}
                  </li>
                ))}
              </ul>
            </div>
          )}
        </>
      )}

      {/* Detection Methods Info */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h4 className="font-medium mb-4">Detection Methods</h4>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {/* VM Signatures */}
          <div>
            <h5 className="text-sm text-text-secondary mb-2">VM Signatures</h5>
            <div className="space-y-2">
              {VM_SIGNATURES.map((vm) => (
                <div key={vm.type} className="p-2 bg-dark-700/50 rounded">
                  <p className="font-medium text-sm">{vm.type}</p>
                  <p className="text-xs text-text-muted">
                    {vm.indicators.join(" • ")}
                  </p>
                </div>
              ))}
            </div>
          </div>

          {/* Sandbox Signatures */}
          <div>
            <h5 className="text-sm text-text-secondary mb-2">
              Sandbox Signatures
            </h5>
            <div className="space-y-2">
              {SANDBOX_SIGNATURES.map((sb) => (
                <div key={sb.type} className="p-2 bg-dark-700/50 rounded">
                  <p className="font-medium text-sm">{sb.type}</p>
                  <p className="text-xs text-text-muted">
                    {sb.indicators.join(" • ")}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Tips */}
      <div className="bg-dark-800/50 rounded-lg p-4 border border-dark-600">
        <h4 className="font-medium mb-2 flex items-center gap-2">
          <Info className="w-4 h-4 text-cyan-400" />
          Environment Detection Tips
        </h4>
        <ul className="text-sm text-text-secondary space-y-1">
          <li>• Timing checks can detect accelerated execution</li>
          <li>• Low core/RAM counts often indicate analysis VMs</li>
          <li>• Check for known analysis tool processes</li>
          <li>• Mouse movement and user interaction patterns</li>
        </ul>
      </div>
    </div>
  );
}
