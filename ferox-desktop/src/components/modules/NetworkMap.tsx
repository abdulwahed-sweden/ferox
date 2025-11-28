/**
 * NetworkMap - Interactive Cyber-Neon Network Visualization
 * For demo/training purposes only
 */

import { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Globe, RefreshCw, ZoomIn, ZoomOut, Maximize2 } from 'lucide-react';
import { mockNodes, mockEdges } from '../../data/mockNetwork';
import type { NetworkNode as NodeType } from '../../data/mockNetwork';
import { NetworkNode } from '../network/NetworkNode';
import { NetworkEdge } from '../network/NetworkEdge';
import { NetworkLegend } from '../network/NetworkLegend';
import { NodeTooltip } from '../network/NodeTooltip';
import '../../styles/network-glow.css';

interface NetworkMapProps {
  sessionId?: string;
}

export function NetworkMap({ sessionId: _sessionId }: NetworkMapProps) {
  const [selectedNode, setSelectedNode] = useState<NodeType | null>(null);
  const [zoom, setZoom] = useState(1);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 500 });

  // Update dimensions on resize
  useEffect(() => {
    const updateDimensions = () => {
      if (containerRef.current) {
        setDimensions({
          width: containerRef.current.clientWidth,
          height: containerRef.current.clientHeight
        });
      }
    };

    updateDimensions();
    window.addEventListener('resize', updateDimensions);
    return () => window.removeEventListener('resize', updateDimensions);
  }, []);

  // Calculate stats
  const stats = {
    total: mockNodes.length,
    compromised: mockNodes.filter(n => n.status === 'exploited' || n.type === 'compromised').length,
    active: mockNodes.filter(n => n.status === 'active' || n.status === 'scanning').length,
    connections: mockEdges.filter(e => e.active).length,
  };

  const handleRefresh = () => {
    setIsRefreshing(true);
    setTimeout(() => setIsRefreshing(false), 2000);
  };

  const handleZoomIn = () => setZoom(z => Math.min(2, z + 0.2));
  const handleZoomOut = () => setZoom(z => Math.max(0.5, z - 0.2));
  const handleResetZoom = () => setZoom(1);

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <motion.div
              animate={{ rotate: isRefreshing ? 360 : 0 }}
              transition={{ duration: 1, repeat: isRefreshing ? Infinity : 0, ease: 'linear' }}
            >
              <Globe className="text-info-text" size={22} style={{ filter: 'drop-shadow(0 0 8px #00d4ff)' }} />
            </motion.div>
            <h2 className="text-lg font-semibold text-text-primary">Network Topology</h2>
            <span className="text-xs bg-info-soft text-info-text px-2 py-0.5 rounded">LIVE</span>
          </div>

          {/* Stats badges */}
          <div className="flex items-center gap-3">
            <StatBadge label="Nodes" value={stats.total} color="#00d4ff" />
            <StatBadge label="Compromised" value={stats.compromised} color="#ff3366" />
            <StatBadge label="Active" value={stats.active} color="#00ff88" />
            <StatBadge label="Links" value={stats.connections} color="#a855f7" />
          </div>

          {/* Controls */}
          <div className="flex items-center gap-1">
            <ControlButton icon={ZoomOut} onClick={handleZoomOut} title="Zoom Out" />
            <span className="text-xs text-text-muted px-2">{Math.round(zoom * 100)}%</span>
            <ControlButton icon={ZoomIn} onClick={handleZoomIn} title="Zoom In" />
            <ControlButton icon={Maximize2} onClick={handleResetZoom} title="Reset" />
            <div className="w-px h-6 bg-dark-600 mx-2" />
            <ControlButton
              icon={RefreshCw}
              onClick={handleRefresh}
              spinning={isRefreshing}
              title="Refresh"
            />
          </div>
        </div>
      </div>

      {/* Map Container */}
      <div
        ref={containerRef}
        className="flex-1 relative overflow-hidden network-grid network-vignette"
        style={{ backgroundColor: 'var(--surface-base)' }}
      >
        {/* Scan line effect */}
        <div className="network-scanline" />

        {/* SVG Network Visualization */}
        <motion.svg
          className="w-full h-full"
          style={{
            transform: `scale(${zoom})`,
            transformOrigin: 'center center'
          }}
          animate={{ scale: zoom }}
          transition={{ type: 'spring', stiffness: 300, damping: 30 }}
        >
          {/* Defs for gradients and filters */}
          <defs>
            <filter id="glow" x="-50%" y="-50%" width="200%" height="200%">
              <feGaussianBlur stdDeviation="4" result="coloredBlur" />
              <feMerge>
                <feMergeNode in="coloredBlur" />
                <feMergeNode in="SourceGraphic" />
              </feMerge>
            </filter>
          </defs>

          {/* Render edges first (behind nodes) */}
          <g className="edges">
            {mockEdges.map(edge => (
              <NetworkEdge
                key={edge.id}
                edge={edge}
                nodes={mockNodes}
                containerWidth={dimensions.width}
                containerHeight={dimensions.height}
              />
            ))}
          </g>

          {/* Render nodes */}
          <g className="nodes">
            {mockNodes.map(node => (
              <NetworkNode
                key={node.id}
                node={node}
                selected={selectedNode?.id === node.id}
                onClick={setSelectedNode}
                containerWidth={dimensions.width}
                containerHeight={dimensions.height}
              />
            ))}
          </g>
        </motion.svg>

        {/* Legend */}
        <NetworkLegend />

        {/* Node Tooltip */}
        <AnimatePresence>
          {selectedNode && (
            <NodeTooltip
              node={selectedNode}
              onClose={() => setSelectedNode(null)}
            />
          )}
        </AnimatePresence>

        {/* Watermark */}
        <div className="absolute bottom-4 right-4 text-[10px] text-text-muted/30 font-mono">
          FEROX NETWORK MAPPER v1.0
        </div>
      </div>
    </div>
  );
}

// Stat Badge Component
function StatBadge({ label, value, color }: { label: string; value: number; color: string }) {
  return (
    <div className="flex items-center gap-1.5 text-xs">
      <div
        className="w-2 h-2 rounded-full"
        style={{
          backgroundColor: color,
          boxShadow: `0 0 6px ${color}`
        }}
      />
      <span className="text-text-muted">{label}:</span>
      <span className="font-semibold" style={{ color }}>{value}</span>
    </div>
  );
}

// Control Button Component
function ControlButton({
  icon: Icon,
  onClick,
  spinning,
  title
}: {
  icon: typeof ZoomIn;
  onClick: () => void;
  spinning?: boolean;
  title: string;
}) {
  return (
    <motion.button
      onClick={onClick}
      className="p-2 bg-dark-700 border border-dark-600 rounded hover:bg-dark-600 hover:border-dark-500 transition-colors"
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      title={title}
    >
      <motion.div
        animate={{ rotate: spinning ? 360 : 0 }}
        transition={{ duration: 1, repeat: spinning ? Infinity : 0, ease: 'linear' }}
      >
        <Icon size={14} className="text-text-secondary" />
      </motion.div>
    </motion.button>
  );
}

export default NetworkMap;
