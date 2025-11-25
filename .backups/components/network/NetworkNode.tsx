/**
 * NetworkNode - Animated node component with cyber-neon styling
 */

import { motion } from 'framer-motion';
import {
  Skull,
  Server,
  Monitor,
  Router,
  Database,
  Shield,
} from 'lucide-react';
import type { NetworkNode as NodeType } from '../../data/mockNetwork';
import { typeColors, statusColors } from '../../data/mockNetwork';
import { nodeVariants, pulseVariants } from './animations';

interface NetworkNodeProps {
  node: NodeType;
  selected: boolean;
  onClick: (node: NodeType) => void;
  containerWidth: number;
  containerHeight: number;
}

const iconMap: Record<string, typeof Monitor> = {
  attacker: Skull,
  server: Server,
  workstation: Monitor,
  router: Router,
  target: Database,
  compromised: Shield,
};

export function NetworkNode({ node, selected, onClick, containerWidth, containerHeight }: NetworkNodeProps) {
  const Icon = iconMap[node.type] || Monitor;
  const color = typeColors[node.type];
  const statusColor = statusColors[node.status];

  // Calculate actual pixel positions
  const x = (node.x / 100) * containerWidth;
  const y = (node.y / 100) * containerHeight;

  return (
    <motion.g
      className="cursor-pointer"
      style={{ x, y }}
      onClick={() => onClick(node)}
      variants={nodeVariants}
      animate={node.status}
      whileHover={{ scale: 1.15 }}
      whileTap={{ scale: 0.95 }}
    >
      {/* Pulse ring for active/scanning/exploited nodes */}
      {(node.status === 'active' || node.status === 'scanning' || node.status === 'exploited') && (
        <motion.circle
          r="28"
          fill="none"
          stroke={statusColor}
          strokeWidth="2"
          variants={pulseVariants}
          animate="pulse"
          style={{ opacity: 0.5 }}
        />
      )}

      {/* Glow background circle */}
      <motion.circle
        r="24"
        fill={`${color}20`}
        stroke={color}
        strokeWidth={selected ? 3 : 1.5}
        style={{
          filter: node.status !== 'offline' ? `drop-shadow(0 0 8px ${color})` : 'none'
        }}
        animate={node.status !== 'offline' ? {
          filter: [
            `drop-shadow(0 0 4px ${color})`,
            `drop-shadow(0 0 12px ${color})`,
            `drop-shadow(0 0 4px ${color})`
          ]
        } : {}}
        transition={{ duration: 2, repeat: Infinity, ease: 'easeInOut' }}
      />

      {/* Icon */}
      <foreignObject x="-12" y="-12" width="24" height="24">
        <div className="w-full h-full flex items-center justify-center">
          <Icon size={18} style={{ color }} />
        </div>
      </foreignObject>

      {/* Status indicator dot */}
      <circle
        cx="16"
        cy="-16"
        r="5"
        fill={statusColor}
        stroke="#0d1117"
        strokeWidth="2"
        style={{
          filter: node.status !== 'offline' ? `drop-shadow(0 0 4px ${statusColor})` : 'none'
        }}
      />

      {/* Label */}
      <text
        y="40"
        textAnchor="middle"
        className="text-[10px] font-bold fill-current"
        style={{ fill: color }}
      >
        {node.label}
      </text>

      {/* IP Address */}
      <text
        y="52"
        textAnchor="middle"
        className="text-[8px] fill-current"
        style={{ fill: '#6b7280' }}
      >
        {node.ip}
      </text>
    </motion.g>
  );
}

export default NetworkNode;
