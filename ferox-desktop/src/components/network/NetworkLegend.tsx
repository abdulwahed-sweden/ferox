/**
 * NetworkLegend - Status and type legend component
 */

import { motion } from "framer-motion";
import { Skull, Server, Monitor, Router, Database } from "lucide-react";
import { legendVariants, legendItemVariants } from "./animations";

const nodeTypes = [
  { icon: Skull, label: "Attacker", color: "#ff3366" },
  { icon: Database, label: "Target", color: "#ffd700" },
  { icon: Server, label: "Server", color: "#00d4ff" },
  { icon: Monitor, label: "Workstation", color: "#6b7280" },
  { icon: Router, label: "Router", color: "#a855f7" },
];

const statusTypes = [
  { label: "Active", color: "#00ff88" },
  { label: "Scanning", color: "#00d4ff" },
  { label: "Exploited", color: "#ff3366" },
  { label: "Idle", color: "#6b7280" },
  { label: "Offline", color: "#374151" },
];

const edgeTypes = [
  { label: "C2 Control", color: "#00ff88" },
  { label: "Lateral", color: "#ff6b35" },
  { label: "Scan", color: "#00d4ff" },
  { label: "Exfil", color: "#ff3366" },
];

export function NetworkLegend() {
  return (
    <motion.div
      className="absolute bottom-4 left-4 bg-dark-800/90 backdrop-blur-sm rounded-lg border border-dark-600 p-3 text-xs"
      variants={legendVariants}
      initial="hidden"
      animate="visible"
    >
      {/* Node Types */}
      <div className="mb-3">
        <div className="text-text-muted font-medium mb-2">Node Types</div>
        <div className="space-y-1.5">
          {nodeTypes.map(({ icon: Icon, label, color }) => (
            <motion.div
              key={label}
              className="flex items-center gap-2"
              variants={legendItemVariants}
            >
              <div
                className="w-5 h-5 rounded flex items-center justify-center"
                style={{
                  backgroundColor: `${color}20`,
                  boxShadow: `0 0 8px ${color}40`,
                }}
              >
                <Icon size={12} style={{ color }} />
              </div>
              <span className="text-text-secondary">{label}</span>
            </motion.div>
          ))}
        </div>
      </div>

      {/* Status */}
      <div className="mb-3">
        <div className="text-text-muted font-medium mb-2">Status</div>
        <div className="grid grid-cols-2 gap-x-4 gap-y-1.5">
          {statusTypes.map(({ label, color }) => (
            <motion.div
              key={label}
              className="flex items-center gap-2"
              variants={legendItemVariants}
            >
              <div
                className="w-2.5 h-2.5 rounded-full"
                style={{
                  backgroundColor: color,
                  boxShadow: `0 0 6px ${color}`,
                }}
              />
              <span className="text-text-secondary">{label}</span>
            </motion.div>
          ))}
        </div>
      </div>

      {/* Edge Types */}
      <div>
        <div className="text-text-muted font-medium mb-2">Connections</div>
        <div className="grid grid-cols-2 gap-x-4 gap-y-1.5">
          {edgeTypes.map(({ label, color }) => (
            <motion.div
              key={label}
              className="flex items-center gap-2"
              variants={legendItemVariants}
            >
              <div
                className="w-4 h-0.5 rounded"
                style={{
                  backgroundColor: color,
                  boxShadow: `0 0 4px ${color}`,
                }}
              />
              <span className="text-text-secondary">{label}</span>
            </motion.div>
          ))}
        </div>
      </div>
    </motion.div>
  );
}

export default NetworkLegend;
