# Ferox Architecture

## Overview

Ferox is a modern Command & Control (C2) framework built with Rust and React. It provides a professional-grade operations console for red team engagements.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Ferox Desktop                                │
│  ┌─────────────────┐     ┌─────────────────┐     ┌───────────────┐  │
│  │  React Frontend │────▶│  Tauri Backend  │────▶│  Ferox Core   │  │
│  │  (TypeScript)   │     │  (Rust)         │     │  (Rust)       │  │
│  │                 │     │                 │     │               │  │
│  │  - Components   │     │  - Commands     │     │  - Session    │  │
│  │  - Hooks        │     │  - Bridge       │     │    Manager    │  │
│  │  - Store        │     │  - Events       │     │  - Modules    │  │
│  │  - Services     │     │  - Security     │     │  - Engines    │  │
│  └─────────────────┘     └─────────────────┘     └───────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
                    ┌───────────────────────────────┐
                    │         Target Systems        │
                    │   (Sessions / Implants)       │
                    └───────────────────────────────┘
```

## Components

### Ferox Core (`src/`)
The core Rust library providing:
- **Session Management**: Track and manage active sessions
- **Post-Exploitation Modules**: PrivEsc, credential harvesting, persistence
- **Memory Forensics**: Dump analysis and artifact extraction
- **OPSEC Engine**: Detection avoidance and safety checks

### Ferox Desktop (`ferox-desktop/`)
The Tauri-based desktop application:
- **Frontend**: React + TypeScript + Zustand
- **Backend**: Tauri commands wrapping Ferox Core
- **Bridge**: `FeroxBridge` connecting UI to core engine

## Data Flow

### Session Creation
```
User Action → Tauri Command → FeroxBridge → CoreSessionManager → Database
                                    ↓
                              Tauri Event → React Store → UI Update
```

### Command Execution
```
Terminal Input → execute_terminal_command → FeroxBridge.execute_command
                                                    ↓
                                          CoreSessionManager.execute
                                                    ↓
                                            Session → Result
                                                    ↓
                                    TerminalHistory + AuditLog + UI Event
```

## Key Patterns

### FeroxBridge
Central integration point between Tauri and Ferox Core:
```rust
pub struct FeroxBridge {
    core_manager: Arc<CoreSessionManager>,
    ui_sessions: Arc<RwLock<HashMap<String, UiSession>>>,
    event_tx: broadcast::Sender<BridgeEvent>,
    // Post-ex engines
    privesc_engine: Arc<RwLock<PrivEscEngine>>,
    cred_engine: Arc<RwLock<CredentialHarvestEngine>>,
    persistence_engine: Arc<RwLock<PersistenceEngine>>,
}
```

### Event System
Real-time updates via Tauri events:
- `session:created` - New session established
- `session:died` - Session terminated
- `sessions:refreshed` - Periodic sync complete
- `credentials:found` - Credentials harvested

### State Management
Frontend uses Zustand for state:
```typescript
interface SessionStore {
  sessions: Session[];
  selectedSessionId: string | null;
  // Actions
  setSessions: (sessions: Session[]) => void;
  selectSession: (id: string) => void;
}
```

## Technologies

| Layer | Technology |
|-------|------------|
| Core Engine | Rust, Tokio, SQLite |
| Desktop Backend | Tauri 2.0, parking_lot |
| Desktop Frontend | React 18, TypeScript, Vite |
| State Management | Zustand |
| Styling | Tailwind CSS |
| Testing | Vitest, cargo test |

## Directory Structure

```
ferox/
├── src/                    # Ferox Core library
│   ├── core/               # Core types and session management
│   ├── modules/            # Post-exploitation modules
│   ├── network/            # Network scanning and communication
│   └── ...
│
├── ferox-desktop/          # Desktop application
│   ├── src/                # React frontend
│   │   ├── components/     # UI components
│   │   ├── hooks/          # Custom hooks
│   │   ├── store/          # Zustand stores
│   │   └── services/       # Tauri service wrappers
│   │
│   └── src-tauri/          # Tauri backend
│       └── src/
│           ├── bridge/     # FeroxBridge
│           ├── commands/   # Tauri commands
│           ├── security/   # Input validation, audit
│           └── ...
│
├── config/                 # Configuration templates
├── docs/                   # Documentation
└── tests/                  # Integration tests
```

## Security Considerations

- Input validation on all Tauri commands
- Audit logging for sensitive operations
- CSP policies for the webview
- No hardcoded credentials
- Secure session storage
