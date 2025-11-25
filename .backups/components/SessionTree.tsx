import { useState, memo, useMemo, useCallback } from 'react';
import { useAppStore } from '../store';
import { useDebounce } from '../hooks/useDebounce';
import { SessionListSkeleton } from './Loading';
import type { SessionTreeNode } from '../types';
import {
  Monitor,
  ChevronRight,
  ChevronDown,
  Crown,
  Shield,
  User,
  AlertTriangle,
  RefreshCw,
} from 'lucide-react';
import { clsx } from 'clsx';

// OS icons - memoized
const OsIcon = memo(function OsIcon({ os }: { os: string }) {
  const iconClass = 'w-4 h-4';
  switch (os) {
    case 'windows':
      return <Monitor className={iconClass} />;
    case 'linux':
      return <Monitor className={iconClass} />;
    case 'macos':
      return <Monitor className={iconClass} />;
    default:
      return <Monitor className={iconClass} />;
  }
});

// Privilege icon - memoized
const PrivilegeIcon = memo(function PrivilegeIcon({ privilege }: { privilege: string }) {
  switch (privilege) {
    case 'system':
    case 'root':
      return <Crown size={12} className="text-danger" />;
    case 'administrator':
      return <Shield size={12} className="text-warning" />;
    default:
      return <User size={12} className="text-text-muted" />;
  }
});

// Status dot - memoized
const StatusDot = memo(function StatusDot({ status }: { status: string }) {
  return (
    <span
      className={clsx(
        'status-dot',
        status === 'active' && 'status-active',
        status === 'sleeping' && 'status-sleeping',
        status === 'dead' && 'status-dead'
      )}
    />
  );
});

interface SessionNodeProps {
  node: SessionTreeNode;
  depth?: number;
}

// Memoized session node for better performance with large trees
const SessionNode = memo(function SessionNode({ node, depth = 0 }: SessionNodeProps) {
  const [expanded, setExpanded] = useState(true);
  const { selectedSessionId, selectSession, showContextMenu, addTab, tabs } =
    useAppStore();

  const { session, children } = node;
  const hasChildren = children.length > 0;
  const isSelected = selectedSessionId === session.id;

  const handleClick = useCallback(() => {
    selectSession(session.id);
  }, [selectSession, session.id]);

  const handleDoubleClick = useCallback(() => {
    // Open terminal tab for this session
    const existingTab = tabs.find(
      (t) => t.sessionId === session.id && t.type === 'terminal'
    );
    if (!existingTab) {
      addTab({
        id: `tab-${Date.now()}`,
        type: 'terminal',
        sessionId: session.id,
        title: session.hostname,
        icon: 'terminal',
      });
    }
  }, [tabs, session.id, session.hostname, addTab]);

  const handleContextMenu = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    selectSession(session.id);
    showContextMenu(e.clientX, e.clientY, session.id);
  }, [selectSession, showContextMenu, session.id]);

  const handleToggleExpand = useCallback((e: React.MouseEvent) => {
    e.stopPropagation();
    setExpanded((prev) => !prev);
  }, []);

  return (
    <div>
      <div
        className={clsx(
          'flex items-center gap-1 px-2 py-1.5 cursor-pointer select-none transition-colors',
          'hover:bg-dark-600',
          isSelected && 'bg-dark-600 border-l-2 border-ferox-green'
        )}
        style={{ paddingLeft: `${depth * 16 + 8}px` }}
        onClick={handleClick}
        onDoubleClick={handleDoubleClick}
        onContextMenu={handleContextMenu}
      >
        {/* Expand/collapse button */}
        {hasChildren ? (
          <button
            className="p-0.5 hover:bg-dark-500 rounded"
            onClick={handleToggleExpand}
          >
            {expanded ? (
              <ChevronDown size={14} className="text-text-muted" />
            ) : (
              <ChevronRight size={14} className="text-text-muted" />
            )}
          </button>
        ) : (
          <span className="w-5" />
        )}

        {/* Status indicator */}
        <StatusDot status={session.status} />

        {/* OS Icon */}
        <OsIcon os={session.os} />

        {/* Session info */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-1.5">
            <span className="text-sm font-medium text-text-primary truncate">
              {session.hostname}
            </span>
            <PrivilegeIcon privilege={session.privileges} />
          </div>
          <div className="text-xs text-text-muted truncate">
            {session.username}@{session.ip_address}
          </div>
        </div>
      </div>

      {/* Children (pivoted sessions) */}
      {hasChildren && expanded && (
        <div>
          {children.map((child) => (
            <SessionNode key={child.session.id} node={child} depth={depth + 1} />
          ))}
        </div>
      )}
    </div>
  );
});

interface SessionFilters {
  status: ('active' | 'sleeping' | 'dead')[];
  os: string[];
}

// Filter sessions by search query and filters
function filterNodes(
  nodes: SessionTreeNode[],
  query: string,
  filters: SessionFilters
): SessionTreeNode[] {
  const hasQuery = query.trim().length > 0;
  const hasStatusFilter = filters.status.length > 0;
  const hasOsFilter = filters.os.length > 0;

  if (!hasQuery && !hasStatusFilter && !hasOsFilter) return nodes;

  const lowerQuery = query.toLowerCase();

  const matchSession = (node: SessionTreeNode): boolean => {
    const { session } = node;

    // Check status filter
    if (hasStatusFilter && !filters.status.includes(session.status as 'active' | 'sleeping' | 'dead')) {
      return false;
    }

    // Check OS filter
    if (hasOsFilter && !filters.os.includes(session.os)) {
      return false;
    }

    // Check search query
    if (hasQuery) {
      return (
        session.hostname.toLowerCase().includes(lowerQuery) ||
        session.ip_address.toLowerCase().includes(lowerQuery) ||
        session.username.toLowerCase().includes(lowerQuery) ||
        session.os.toLowerCase().includes(lowerQuery)
      );
    }

    return true;
  };

  const filterRecursive = (items: SessionTreeNode[]): SessionTreeNode[] => {
    return items
      .map((node) => {
        const filteredChildren = filterRecursive(node.children);
        if (matchSession(node) || filteredChildren.length > 0) {
          return { ...node, children: filteredChildren };
        }
        return null;
      })
      .filter((n): n is SessionTreeNode => n !== null);
  };

  return filterRecursive(nodes);
}

export function SessionTree() {
  const { sessionTree, sessions, searchQuery, sessionsLoading, sessionsError, sessionFilters } = useAppStore();

  // Debounce search query by 300ms for performance
  const debouncedQuery = useDebounce(searchQuery, 300);

  // Memoize base nodes computation
  const baseNodes = useMemo(() => {
    return sessionTree.length > 0
      ? sessionTree
      : sessions.map((s) => ({ session: s, children: [] }));
  }, [sessionTree, sessions]);

  // Memoize filtered nodes computation
  const nodes = useMemo(() => {
    return filterNodes(baseNodes, debouncedQuery, sessionFilters);
  }, [baseNodes, debouncedQuery, sessionFilters]);

  // Check if any filters are active
  const hasActiveFilters = sessionFilters.status.length > 0 || sessionFilters.os.length > 0;

  // Show loading skeleton on initial load
  if (sessionsLoading) {
    return <SessionListSkeleton count={4} />;
  }

  // Show error state
  if (sessionsError) {
    return (
      <div className="p-4 text-center text-text-muted">
        <AlertTriangle size={32} className="mx-auto mb-2 text-danger opacity-70" />
        <p className="text-sm text-danger">Failed to load sessions</p>
        <p className="text-xs mt-1 mb-3">{sessionsError}</p>
        <button
          onClick={() => window.location.reload()}
          className="inline-flex items-center gap-1 px-2 py-1 text-xs bg-dark-600 hover:bg-dark-500 rounded transition-colors"
        >
          <RefreshCw size={12} />
          Retry
        </button>
      </div>
    );
  }

  if (baseNodes.length === 0) {
    return (
      <div className="p-4 text-center text-text-muted">
        <Monitor size={32} className="mx-auto mb-2 opacity-50" />
        <p className="text-sm">No sessions</p>
        <p className="text-xs mt-1">Deploy a payload to establish connection</p>
      </div>
    );
  }

  if (nodes.length === 0 && (debouncedQuery || hasActiveFilters)) {
    return (
      <div className="p-4 text-center text-text-muted">
        <p className="text-sm">No matches found</p>
        <p className="text-xs mt-1">
          {hasActiveFilters ? 'Try adjusting filters or search' : 'Try a different search term'}
        </p>
      </div>
    );
  }

  return (
    <div className="py-1">
      {nodes.map((node) => (
        <SessionNode key={node.session.id} node={node} />
      ))}
      {debouncedQuery && (
        <div className="px-3 py-1 text-xs text-text-muted">
          {nodes.length} result{nodes.length !== 1 ? 's' : ''}
        </div>
      )}
    </div>
  );
}
