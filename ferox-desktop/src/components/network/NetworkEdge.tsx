/**
 * NetworkEdge - Connection lines with animated data flow
 */

import { motion } from "framer-motion";
import type {
  NetworkEdge as EdgeType,
  NetworkNode,
} from "../../data/mockNetwork";
import { edgeColors } from "../../data/mockNetwork";
import { edgeVariants } from "./animations";

interface NetworkEdgeProps {
  edge: EdgeType;
  nodes: NetworkNode[];
  containerWidth: number;
  containerHeight: number;
}

export function NetworkEdge({
  edge,
  nodes,
  containerWidth,
  containerHeight,
}: NetworkEdgeProps) {
  const fromNode = nodes.find((n) => n.id === edge.from);
  const toNode = nodes.find((n) => n.id === edge.to);

  if (!fromNode || !toNode) return null;

  const color = edgeColors[edge.type];

  // Calculate actual positions
  const x1 = (fromNode.x / 100) * containerWidth;
  const y1 = (fromNode.y / 100) * containerHeight;
  const x2 = (toNode.x / 100) * containerWidth;
  const y2 = (toNode.y / 100) * containerHeight;

  const pathId = `path-${edge.id}`;

  return (
    <g>
      {/* Glow effect layer */}
      {edge.active && (
        <motion.line
          x1={x1}
          y1={y1}
          x2={x2}
          y2={y2}
          stroke={color}
          strokeWidth="6"
          strokeOpacity="0.3"
          strokeLinecap="round"
          style={{ filter: `blur(4px)` }}
          variants={edgeVariants}
          animate="active"
        />
      )}

      {/* Main edge line */}
      <motion.line
        x1={x1}
        y1={y1}
        x2={x2}
        y2={y2}
        stroke={color}
        strokeWidth={edge.active ? 2 : 1}
        strokeOpacity={edge.active ? 0.8 : 0.2}
        strokeLinecap="round"
        strokeDasharray={edge.active ? "none" : "4 4"}
        variants={edgeVariants}
        animate={edge.active ? "active" : "idle"}
        style={{
          filter: edge.active ? `drop-shadow(0 0 4px ${color})` : "none",
        }}
      />

      {/* Path for data flow animation */}
      <path
        id={pathId}
        d={`M ${x1} ${y1} L ${x2} ${y2}`}
        fill="none"
        stroke="none"
      />

      {/* Data flow particles for active exfil/control connections */}
      {edge.active && (edge.type === "exfil" || edge.type === "control") && (
        <>
          <motion.circle
            r="4"
            fill={color}
            style={{
              offsetPath: `path("M ${x1} ${y1} L ${x2} ${y2}")`,
              filter: `drop-shadow(0 0 6px ${color})`,
            }}
            animate={{
              offsetDistance: ["0%", "100%"],
            }}
            transition={{
              duration: 1.5,
              repeat: Infinity,
              ease: "linear",
            }}
          />
          <motion.circle
            r="3"
            fill={color}
            style={{
              offsetPath: `path("M ${x1} ${y1} L ${x2} ${y2}")`,
              filter: `drop-shadow(0 0 4px ${color})`,
            }}
            animate={{
              offsetDistance: ["0%", "100%"],
            }}
            transition={{
              duration: 1.5,
              repeat: Infinity,
              ease: "linear",
              delay: 0.5,
            }}
          />
        </>
      )}

      {/* Edge type label for active connections */}
      {edge.active && (
        <text
          x={(x1 + x2) / 2}
          y={(y1 + y2) / 2 - 8}
          textAnchor="middle"
          className="text-[8px] font-medium uppercase"
          style={{ fill: color, opacity: 0.7 }}
        >
          {edge.type}
        </text>
      )}
    </g>
  );
}

export default NetworkEdge;
