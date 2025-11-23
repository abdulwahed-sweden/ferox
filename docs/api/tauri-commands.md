# Tauri Commands API Reference

## Overview

Ferox Desktop exposes Tauri commands for communication between the React frontend and Rust backend. All commands are async and return typed results.

## Session Commands

### `get_sessions`
Retrieve all active sessions.

**Returns:** `SessionListResponse`
```typescript
interface SessionListResponse {
  sessions: Session[];
  total: number;
  active_count: number;
}
```

### `get_session`
Get a single session by ID.

**Parameters:**
- `id: string` - Session UUID

**Returns:** `Session`

### `create_session`
Create a new session.

**Parameters:**
```typescript
interface CreateSessionRequest {
  hostname: string;
  ip_address: string;
  os: "windows" | "linux" | "macos";
  username: string;
  privileges: "user" | "administrator" | "system" | "root";
  parent_id?: string;
}
```

**Returns:** `Session`

### `terminate_session`
Terminate an active session.

**Parameters:**
- `id: string` - Session UUID

**Returns:** `void`

### `update_session_note`
Update a session's note.

**Parameters:**
- `id: string` - Session UUID
- `note: string | null` - Note content

**Returns:** `void`

### `get_session_tree`
Get hierarchical session tree.

**Returns:** `SessionTreeNode[]`

---

## Terminal Commands

### `create_terminal`
Create a new terminal for a session.

**Parameters:**
```typescript
interface CreateTerminalRequest {
  session_id: string;
  rows?: number;
  cols?: number;
  shell?: string;
}
```

**Returns:** `TerminalResponse`

### `execute_terminal_command`
Execute a command in a terminal.

**Parameters:**
```typescript
interface ExecuteTerminalCommandRequest {
  terminal_id: string;
  command: string;
}
```

**Returns:**
```typescript
interface ExecuteTerminalCommandResponse {
  output: string;
  success: boolean;
  execution_time_ms: number;
}
```

### `get_terminal_history`
Get command history for a terminal.

**Parameters:**
- `terminal_id: string`

**Returns:** `HistoryEntry[]`

### `close_terminal`
Close a terminal.

**Parameters:**
- `terminal_id: string`

**Returns:** `void`

---

## Module Commands

### `execute_command`
Execute a raw command on a session.

**Parameters:**
```typescript
interface ExecuteCommandRequest {
  session_id: string;
  command: string;
}
```

**Returns:** `CommandResult`

### `run_privesc`
Run privilege escalation scan.

**Parameters:**
```typescript
interface PrivEscRequest {
  session_id: string;
  auto_escalate: boolean;
  safe_mode: boolean;
}
```

**Returns:** `PrivEscResult`

### `harvest_credentials`
Harvest credentials from a session.

**Parameters:**
```typescript
interface CredentialHarvestRequest {
  session_id: string;
  sources: string[];
  safe_mode: boolean;
}
```

**Returns:** `CredentialHarvestResult`

### `install_persistence`
Install persistence mechanism.

**Parameters:**
```typescript
interface PersistenceRequest {
  session_id: string;
  method: string;
  name: string;
  safe_mode: boolean;
}
```

**Returns:** `PersistenceResult`

### `lateral_move`
Perform lateral movement.

**Parameters:**
```typescript
interface LateralMoveRequest {
  session_id: string;
  target_host: string;
  method: string;
  credential_id?: string;
  safe_mode: boolean;
}
```

**Returns:** `LateralMoveResult`

### `network_discovery`
Run network discovery scan.

**Parameters:**
```typescript
interface DiscoveryRequest {
  session_id: string;
  subnet?: string;
  ports?: number[];
}
```

**Returns:** `DiscoveryResult`

---

## Tauri Events

The backend emits these events for real-time updates:

| Event | Payload | Description |
|-------|---------|-------------|
| `session:created` | `{ session: Session }` | New session established |
| `session:died` | `{ session_id, hostname }` | Session terminated |
| `sessions:refreshed` | `void` | Periodic sync completed |
| `credentials:found` | `{ session_id, count }` | Credentials harvested |

### Listening to Events (Frontend)
```typescript
import { listen } from '@tauri-apps/api/event';

useEffect(() => {
  const unlisten = listen('session:created', (event) => {
    console.log('New session:', event.payload);
  });

  return () => { unlisten.then(fn => fn()); };
}, []);
```

---

## Error Handling

All commands return `Result<T, String>` in Rust, which maps to:
- **Success**: Returns the typed result
- **Error**: Throws with error message string

```typescript
try {
  const session = await invoke('get_session', { id: sessionId });
} catch (error) {
  console.error('Failed to get session:', error);
}
```
