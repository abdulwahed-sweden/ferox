# Troubleshooting

Use this checklist when Ferox refuses to build, detects missing tooling, or emits warnings at startup.

## CLI Router Errors
| Symptom | Resolution |
| --- | --- |
| `cargo run` exits with code 101 | Specify a binary: `cargo run --bin ferox -- console`. |
| Dependency probe reports `python3: not found` | Install Python (`brew install python` or distribution packages) and ensure it is on `PATH`. |
| `volatility3` missing | Install via pip (`pip install volatility3`) or point `PATH` to the Volatility repo. |
| `yara` missing | Install via package manager (`brew install yara`, `apt install yara`). |

## Memory Workflows
- Verify dumps are accessible and not compressed.
- Run `ferox doctor dependency volatility` or `yara` to confirm detection.
- Large dumps may require `ulimit -n` adjustments or more disk space.

## Session Manager Issues
| Symptom | Fix |
| --- | --- |
| Warning: "Session DB disabled" | Ensure the current directory is writable or set `FEROX_SESSION_DB=/path/to/db`. |
| `Session not found` | List sessions with `ferox sessions list --all` to confirm UUIDs. |
| History empty | Confirm runs used the same database and that commands completed successfully. |

## Build Failures
- Update Rust: `rustup update`.
- Clean artifacts: `cargo clean && cargo build --release`.
- Ensure OpenSSL/TLS system libraries exist (macOS: `xcode-select --install`).

## Mixed Predator Theme Artifacts
- Set `NO_COLOR=1` to force plain output in limited terminals.
- Set `NO_EMOJI=1` when Unicode icons render poorly.

## Integrity Score Below Threshold
1. Run `ferox doctor check --format markdown` to capture details.
2. Address the failing dependency or configuration.
3. Re-run the doctor until the score recovers.

## Getting Help
- Review [docs/maintenance.md](maintenance.md) for Ferox Doctor details.
- Consult the issues page on GitHub with logs and `doctor report` output.
- For security-sensitive disclosures, email `security@ferox.local`.
