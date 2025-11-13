# Ferox Console

The Ferox Console is the interactive surface that appears after the CLI Integration Layer routes commands. It delivers module discovery, execution, handler control, and theming.

## Launch Paths
- `ferox console` — skip directly into the console.
- `ferox` with no subcommand — defaults to the console after the router prints banners and dependency probes.

## Interface Essentials
- **Prompt:** `ferox(module/path)>` reflects the selected module.
- **Mixed Predator Theme:** Provides color-safe output on TTYs with fallback to ASCII symbols when colors/Unicode are disabled.
- **Aliases:** Shortcuts like `ls`, `s`, `x`, `?`, and `q` match traditional operator muscle memory.

## Common Commands
| Command | Description |
| --- | --- |
| `help` | Show categorized help with module, session, and handler references. |
| `modules` / `list` | Display all registered modules. |
| `use recon/subdomains` | Select a module. |
| `set RHOSTS 10.0.0.0/24` | Configure module options. |
| `check` | Run safe fingerprinting if the module supports it. |
| `run` | Execute the module (prompts if destructive). |
| `sessions` | Summaries powered by the session manager. |
| `handlers` | Inspect shell or payload handlers. |
| `memory` | Bridge into the memory CLI from within the console. |

## Automation Hooks
- **History:** The console stores commands via `rustyline` so multi-line workflows can be recalled quickly.
- **Result Store:** Module results persist to the result store for later export (HTML, JSON, PDF when enabled).
- **Themes:** Use `NO_COLOR=1` or `NO_EMOJI=1` to force ASCII-safe output in CI or serial consoles.

## Exiting Safely
- `quit` / `exit` or `Ctrl+D` cleanly tear down the console and flush history.
- Safe-mode banners remind you when destructive execution is disabled.

For scripting without the interactive console, rely on the CLI Integration Layer (`ferox doctor …`, `ferox memory …`, etc.) and automation-friendly JSON outputs.
