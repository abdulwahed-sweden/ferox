/**
 * Mock Network Topology Data
 * For demo/training visualization purposes
 */

export type NodeType =
  | "attacker"
  | "target"
  | "compromised"
  | "server"
  | "router"
  | "workstation";
export type NodeStatus =
  | "active"
  | "idle"
  | "scanning"
  | "exploited"
  | "offline";
export type EdgeType = "control" | "lateral" | "scan" | "exfil";
export type OsType = "windows" | "linux" | "macos" | "network";

export interface NetworkNode {
  id: string;
  label: string;
  ip: string;
  type: NodeType;
  status: NodeStatus;
  os?: OsType;
  ports?: number[];
  x: number; // Position percentage (0-100)
  y: number;
}

export interface NetworkEdge {
  id: string;
  from: string;
  to: string;
  type: EdgeType;
  active: boolean;
  bandwidth?: number;
}

export const mockNodes: NetworkNode[] = [
  {
    id: "attacker",
    label: "ATTACKER",
    ip: "10.0.0.1",
    type: "attacker",
    status: "active",
    x: 8,
    y: 50,
  },
  {
    id: "router1",
    label: "Gateway",
    ip: "192.168.1.1",
    type: "router",
    status: "active",
    os: "network",
    x: 28,
    y: 50,
  },
  {
    id: "srv1",
    label: "DC-01",
    ip: "192.168.1.10",
    type: "server",
    status: "exploited",
    os: "windows",
    ports: [445, 3389, 88, 389],
    x: 48,
    y: 25,
  },
  {
    id: "srv2",
    label: "WEB-01",
    ip: "192.168.1.20",
    type: "server",
    status: "scanning",
    os: "linux",
    ports: [80, 443, 22],
    x: 48,
    y: 75,
  },
  {
    id: "ws1",
    label: "WS-001",
    ip: "192.168.1.101",
    type: "workstation",
    status: "exploited",
    os: "windows",
    ports: [445, 3389],
    x: 68,
    y: 25,
  },
  {
    id: "ws2",
    label: "WS-002",
    ip: "192.168.1.102",
    type: "workstation",
    status: "idle",
    os: "windows",
    x: 68,
    y: 50,
  },
  {
    id: "ws3",
    label: "WS-003",
    ip: "192.168.1.103",
    type: "workstation",
    status: "offline",
    os: "macos",
    x: 68,
    y: 75,
  },
  {
    id: "target",
    label: "DB-PROD",
    ip: "192.168.1.200",
    type: "target",
    status: "idle",
    os: "linux",
    ports: [5432, 22, 6379],
    x: 88,
    y: 50,
  },
];

export const mockEdges: NetworkEdge[] = [
  { id: "e1", from: "attacker", to: "router1", type: "control", active: true },
  { id: "e2", from: "router1", to: "srv1", type: "lateral", active: true },
  { id: "e3", from: "router1", to: "srv2", type: "scan", active: true },
  { id: "e4", from: "srv1", to: "ws1", type: "lateral", active: true },
  { id: "e5", from: "srv1", to: "ws2", type: "scan", active: false },
  { id: "e6", from: "srv2", to: "ws3", type: "scan", active: false },
  {
    id: "e7",
    from: "ws1",
    to: "target",
    type: "exfil",
    active: true,
    bandwidth: 75,
  },
];

// Color configurations
export const typeColors: Record<NodeType, string> = {
  attacker: "#ff3366",
  target: "#ffd700",
  compromised: "#ff6b35",
  server: "#00d4ff",
  router: "#a855f7",
  workstation: "#6b7280",
};

export const statusColors: Record<NodeStatus, string> = {
  active: "#00ff88",
  idle: "#6b7280",
  scanning: "#00d4ff",
  exploited: "#ff3366",
  offline: "#374151",
};

export const edgeColors: Record<EdgeType, string> = {
  control: "#00ff88",
  lateral: "#ff6b35",
  scan: "#00d4ff",
  exfil: "#ff3366",
};
