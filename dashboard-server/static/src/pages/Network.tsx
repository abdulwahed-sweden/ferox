import { useRef, useState, useEffect, useCallback } from 'react';
// @ts-expect-error - no types available for react-cytoscapejs
import CytoscapeComponent from 'react-cytoscapejs';
import cytoscape, { Core } from 'cytoscape';
// @ts-expect-error - no types available for cytoscape-dagre
import dagre from 'cytoscape-dagre';
import { useDashboardStore } from '../store';
import {
  Server,
  Monitor,
  Database,
  ZoomIn,
  ZoomOut,
  Maximize2,
  Target,
  RefreshCw,
  Layout as LayoutIcon,
} from 'lucide-react';
import { clsx } from 'clsx';
import type { Session } from '../types';

// Register dagre layout
cytoscape.use(dagre);

interface NetworkNode {
  id: string;
  label: string;
  type: 'dc' | 'server' | 'workstation' | 'unknown';
  ip: string;
  isCompromised: boolean;
  session?: Session;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const cytoscapeStylesheet: any[] = [
  {
    selector: 'node',
    style: {
      'background-color': '#1a1f3a',
      'border-width': 2,
      'border-color': '#2d3561',
      label: 'data(label)',
      color: '#8892b0',
      'text-valign': 'bottom',
      'text-margin-y': 8,
      'font-size': '11px',
      width: 50,
      height: 50,
      'text-wrap': 'wrap',
      'text-max-width': '80px',
    },
  },
  {
    selector: 'node[type="dc"]',
    style: {
      shape: 'diamond',
      'border-color': '#ff3366',
      'background-color': 'rgba(255, 51, 102, 0.2)',
    },
  },
  {
    selector: 'node[type="server"]',
    style: {
      shape: 'rectangle',
      'border-color': '#00ccff',
      'background-color': 'rgba(0, 204, 255, 0.2)',
    },
  },
  {
    selector: 'node[type="workstation"]',
    style: {
      shape: 'ellipse',
      'border-color': '#8892b0',
      'background-color': 'rgba(136, 146, 176, 0.2)',
    },
  },
  {
    selector: 'node[?isCompromised]',
    style: {
      'border-color': '#00ff88',
      'border-width': 3,
      'background-color': 'rgba(0, 255, 136, 0.15)',
    },
  },
  {
    selector: 'node:selected',
    style: {
      'border-width': 4,
      'border-color': '#ffffff',
      'background-color': 'rgba(255, 255, 255, 0.1)',
    },
  },
  {
    selector: 'edge',
    style: {
      width: 2,
      'line-color': '#2d3561',
      'target-arrow-color': '#2d3561',
      'target-arrow-shape': 'triangle',
      'curve-style': 'bezier',
      opacity: 0.7,
    },
  },
  {
    selector: 'edge[?compromisedPath]',
    style: {
      'line-color': '#00ff88',
      'target-arrow-color': '#00ff88',
      width: 3,
      opacity: 1,
    },
  },
];

export function NetworkPage() {
  const { sessions, setActiveTab, selectSession } = useDashboardStore();
  const cyRef = useRef<Core | null>(null);
  const [selectedNode, setSelectedNode] = useState<NetworkNode | null>(null);
  const [layoutType, setLayoutType] = useState<'dagre' | 'circle' | 'grid'>('dagre');

  // Generate network nodes from sessions
  const generateElements = useCallback(() => {
    const nodes: cytoscape.ElementDefinition[] = [];
    const edges: cytoscape.ElementDefinition[] = [];

    // Add session nodes
    sessions.forEach((session) => {
      const nodeType = session.intelligence.is_domain_joined && session.tags.includes('domain_controller')
        ? 'dc'
        : session.os === 'linux'
        ? 'server'
        : 'workstation';

      nodes.push({
        data: {
          id: session.id,
          label: `${session.hostname}\n${session.ip_address}`,
          type: nodeType,
          ip: session.ip_address,
          isCompromised: true,
          session: session,
        },
      });
    });

    // Add some discovered (non-compromised) nodes for demo
    const discoveredHosts = [
      { id: 'disc-1', label: 'FILESERVER\n192.168.1.30', type: 'server', ip: '192.168.1.30' },
      { id: 'disc-2', label: 'WS-HR01\n192.168.1.60', type: 'workstation', ip: '192.168.1.60' },
      { id: 'disc-3', label: 'WS-FIN01\n192.168.1.70', type: 'workstation', ip: '192.168.1.70' },
      { id: 'disc-4', label: 'DC-PRIMARY\n192.168.1.1', type: 'dc', ip: '192.168.1.1' },
    ];

    discoveredHosts.forEach((host) => {
      if (!nodes.find((n) => n.data.id === host.id)) {
        nodes.push({
          data: {
            id: host.id,
            label: host.label,
            type: host.type,
            ip: host.ip,
            isCompromised: false,
          },
        });
      }
    });

    // Create edges between compromised nodes and discovered nodes
    const allNodeIds = nodes.map((n) => n.data.id);
    const compromisedIds = nodes.filter((n) => n.data.isCompromised).map((n) => n.data.id);

    compromisedIds.forEach((compId, idx) => {
      // Connect compromised nodes to each other
      if (idx > 0) {
        edges.push({
          data: {
            id: `edge-${compromisedIds[idx - 1]}-${compId}`,
            source: compromisedIds[idx - 1] as string,
            target: compId as string,
            compromisedPath: true,
          },
        });
      }

      // Connect to discovered nodes
      const discoveredIds = allNodeIds.filter((id) => id?.toString().startsWith('disc-'));
      discoveredIds.forEach((discId, i) => {
        if (i % (compromisedIds.length || 1) === idx) {
          edges.push({
            data: {
              id: `edge-${compId}-${discId}`,
              source: compId as string,
              target: discId as string,
              compromisedPath: false,
            },
          });
        }
      });
    });

    return [...nodes, ...edges];
  }, [sessions]);

  const elements = generateElements();

  // Handle node selection
  useEffect(() => {
    if (!cyRef.current) return;

    const cy = cyRef.current;

    cy.on('tap', 'node', (evt) => {
      const node = evt.target;
      const data = node.data();
      setSelectedNode({
        id: data.id,
        label: data.label?.split('\n')[0] || data.id,
        type: data.type,
        ip: data.ip,
        isCompromised: data.isCompromised,
        session: data.session,
      });
    });

    cy.on('tap', (evt) => {
      if (evt.target === cy) {
        setSelectedNode(null);
      }
    });

    return () => {
      cy.removeAllListeners();
    };
  }, []);

  const handleZoomIn = () => {
    if (cyRef.current) {
      cyRef.current.zoom(cyRef.current.zoom() * 1.2);
    }
  };

  const handleZoomOut = () => {
    if (cyRef.current) {
      cyRef.current.zoom(cyRef.current.zoom() / 1.2);
    }
  };

  const handleFit = () => {
    if (cyRef.current) {
      cyRef.current.fit(undefined, 50);
    }
  };

  const handleRelayout = () => {
    if (cyRef.current) {
      const layout = cyRef.current.layout({
        name: layoutType,
        nodeDimensionsIncludeLabels: true,
        rankDir: 'TB',
        spacingFactor: 1.5,
      } as cytoscape.LayoutOptions);
      layout.run();
    }
  };

  const handleOpenTerminal = (session: Session) => {
    selectSession(session.id);
    setActiveTab('terminal');
  };

  const getNodeIcon = (type: NetworkNode['type']) => {
    switch (type) {
      case 'dc':
        return <Database size={24} />;
      case 'server':
        return <Server size={24} />;
      case 'workstation':
        return <Monitor size={24} />;
      default:
        return <Server size={24} />;
    }
  };

  return (
    <div className="h-[calc(100vh-8rem)] flex flex-col gap-4 animate-fade-in">
      {/* Controls */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <h2 className="text-lg font-semibold text-text-primary">Network Topology</h2>
          <div className="flex items-center gap-4 text-sm">
            <span className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-ferox-green border-2 border-ferox-green" />
              Compromised ({sessions.length})
            </span>
            <span className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-dark-700 border-2 border-info" />
              Discovered (4)
            </span>
            <span className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-dark-700 border-2 border-danger rotate-45" style={{ borderRadius: '2px' }} />
              Domain Controller
            </span>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <select
            value={layoutType}
            onChange={(e) => setLayoutType(e.target.value as 'dagre' | 'circle' | 'grid')}
            className="input text-sm py-1"
          >
            <option value="dagre">Hierarchical</option>
            <option value="circle">Circle</option>
            <option value="grid">Grid</option>
          </select>
          <button onClick={handleRelayout} className="btn-ghost p-2" title="Re-layout">
            <LayoutIcon size={18} />
          </button>
          <div className="h-6 w-px bg-dark-600" />
          <button onClick={handleZoomOut} className="btn-ghost p-2" title="Zoom out">
            <ZoomOut size={18} />
          </button>
          <button onClick={handleZoomIn} className="btn-ghost p-2" title="Zoom in">
            <ZoomIn size={18} />
          </button>
          <button onClick={handleFit} className="btn-ghost p-2" title="Fit to view">
            <Maximize2 size={18} />
          </button>
          <button className="btn-ghost p-2" title="Refresh">
            <RefreshCw size={18} />
          </button>
        </div>
      </div>

      {/* Network visualization */}
      <div className="flex-1 flex gap-4">
        <div className="flex-1 bg-dark-800 rounded-lg border border-dark-600 relative overflow-hidden">
          <CytoscapeComponent
            elements={elements}
            stylesheet={cytoscapeStylesheet}
            layout={{
              name: layoutType,
              nodeDimensionsIncludeLabels: true,
              rankDir: 'TB',
              spacingFactor: 1.5,
            } as cytoscape.LayoutOptions}
            cy={(cy: Core) => {
              cyRef.current = cy;
            }}
            style={{
              width: '100%',
              height: '100%',
              background: '#0f1535',
            }}
            minZoom={0.3}
            maxZoom={3}
            wheelSensitivity={0.3}
          />
        </div>

        {/* Selected node details */}
        {selectedNode && (
          <div className="w-80 card animate-slide-in overflow-auto">
            <div className="flex items-center gap-3 mb-4">
              <div
                className={clsx(
                  'p-3 rounded-lg',
                  selectedNode.isCompromised
                    ? 'bg-ferox-green/20 text-ferox-green'
                    : 'bg-dark-600 text-text-secondary'
                )}
              >
                {getNodeIcon(selectedNode.type)}
              </div>
              <div>
                <h3 className="font-semibold text-text-primary">{selectedNode.label}</h3>
                <p className="text-sm text-text-muted">{selectedNode.ip}</p>
              </div>
            </div>

            <div className="space-y-3">
              <div className="flex justify-between text-sm">
                <span className="text-text-secondary">Status</span>
                <span className={selectedNode.isCompromised ? 'text-ferox-green' : 'text-text-muted'}>
                  {selectedNode.isCompromised ? 'Compromised' : 'Discovered'}
                </span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-text-secondary">Type</span>
                <span className="text-text-primary capitalize">{selectedNode.type.replace('_', ' ')}</span>
              </div>
              {selectedNode.session && (
                <>
                  <div className="flex justify-between text-sm">
                    <span className="text-text-secondary">User</span>
                    <span className="text-text-primary">{selectedNode.session.username}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-text-secondary">Privileges</span>
                    <span
                      className={clsx(
                        'badge',
                        selectedNode.session.privileges === 'system' ||
                          selectedNode.session.privileges === 'root'
                          ? 'badge-danger'
                          : selectedNode.session.privileges === 'administrator'
                          ? 'badge-warning'
                          : 'badge-gray'
                      )}
                    >
                      {selectedNode.session.privileges}
                    </span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-text-secondary">OS</span>
                    <span className="text-text-primary capitalize">
                      {selectedNode.session.os} {selectedNode.session.architecture}
                    </span>
                  </div>
                </>
              )}
            </div>

            <div className="mt-4 space-y-2">
              {selectedNode.isCompromised && selectedNode.session ? (
                <button
                  onClick={() => handleOpenTerminal(selectedNode.session!)}
                  className="btn-primary w-full justify-center"
                >
                  <Monitor size={16} />
                  Open Terminal
                </button>
              ) : (
                <button className="btn-primary w-full justify-center">
                  <Target size={16} />
                  Attack Target
                </button>
              )}
            </div>
          </div>
        )}
      </div>

      {/* Legend */}
      <div className="flex items-center gap-6 text-sm text-text-secondary">
        <span className="flex items-center gap-2">
          <Database size={16} className="text-danger" /> Domain Controller
        </span>
        <span className="flex items-center gap-2">
          <Server size={16} className="text-info" /> Server
        </span>
        <span className="flex items-center gap-2">
          <Monitor size={16} /> Workstation
        </span>
        <span className="flex items-center gap-2 ml-4">
          <div className="w-8 h-0.5 bg-ferox-green" /> Attack Path
        </span>
        <span className="flex items-center gap-2">
          <div className="w-8 h-0.5 bg-dark-500 border-dashed border-b-2 border-dark-400" /> Network Link
        </span>
      </div>
    </div>
  );
}
