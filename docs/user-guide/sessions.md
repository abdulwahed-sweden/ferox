# Session Management

The Ferox Session Manager tracks active implants, module executions, and operator-issued commands. It uses an async in-memory map with optional SQLite persistence for durability.

## Features
- **Add & list:** Every successful module run can register a session with metadata (module name, target, platform, user).
- **Heartbeat:** Updates `last_seen` timestamps whenever commands execute.
- **History:** Stores command + output pairs for auditing.
- **Cleanup:** Removes stale or inactive sessions after a configurable age.

## CLI Usage
```bash
ferox sessions list           # Active sessions
ferox sessions list --all     # Include inactive
ferox sessions show <uuid>
ferox sessions kill <uuid>
ferox sessions exec <uuid> "whoami"
ferox sessions history <uuid>
ferox sessions cleanup --hours 48
```

From inside the console, `sessions`, `sessions -i <id>`, and `sessions -k <id>` map to the same functionality.

## Persistence
The CLI router attempts to load `ferox_sessions.db`. If the database cannot be opened, it falls back to an in-memory manager and logs a warning. You can supply a path via the `SessionManager::with_db` API for custom deployments.

## Best Practices
- Tag sessions with metadata (e.g., engagement code names) to simplify cleanup.
- Use `kill` before destructive operations to prevent resurrecting stale handlers.
- Export history before deleting sessions when evidence retention is required.
- Run `ferox sessions stats` to monitor total vs. active counts.

The session layer underpins the C2 helpers, console automation, and reporting pipeline, so keep it healthy and regularly prune stale entries.
