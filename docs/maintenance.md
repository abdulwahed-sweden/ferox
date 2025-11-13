# Maintenance & Diagnostics

Ferox Doctor is the guardian subsystem that validates dependencies, scores system integrity, and surfaces remediation steps. Run it regularly—especially before engagements or CI/CD promotions.

## Running Ferox Doctor
```bash
# Comprehensive check
ferox doctor check --format text

# Critical dependencies only (Rust, Cargo, Python)
ferox doctor check --critical --format json

# Inspect a specific dependency
ferox doctor dependency volatility

# Generate a JSON report on disk
ferox doctor report --output reports/doctor.json
```

## Integrity Score
Each run outputs an integrity score (0–100) computed from dependency readiness, sandbox configuration, and audit settings. Thresholds:
- **90–100:** Fully ready for live operations.
- **70–89:** Minor issues (missing optional tooling, stale configs).
- **<70:** Blockers (missing dependencies, audit logging disabled, unsafe config).

## Health Checks
- **Runtime:** Rust compiler, Cargo, target triple, release artifacts.
- **Tooling:** Python, Volatility3, YARA, Git, OpenSSL, TLS libraries.
- **Environment:** `ferox_security.toml`, audit log paths, SAFE_MODE status, sandbox rules.
- **Storage:** Session database availability, disk space, permissions.

Use `--fix` (when available) to display remediation hints or automatically create missing directories.

## Workflow Integration
1. Run `ferox doctor check` before and after each engagement.
2. Archive JSON/Markdown output with your after-action reports.
3. Fail CI pipelines if the integrity score drops below your policy threshold.

## Maintenance Tips
- Keep `ferox_security.toml` under version control and document deviations.
- Rotate API tokens referenced by the C2 layer (`FEROX_C2_TOKEN`).
- Clear session and handler stores between engagements.
- Update Rust toolchains frequently to benefit from compiler hardening.

When in doubt, rerun Ferox Doctor—it is the fastest way to confirm the platform’s health.
