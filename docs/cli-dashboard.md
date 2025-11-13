---
title: Ferox CLI Control Dashboard
description: Comprehensive CLI-based project control, testing, and diagnostics dashboard
---

# Ferox CLI Control Dashboard

Complete interactive command-line interface for project management, testing, and diagnostics.

## Dashboard Architecture

### Main Dashboard Entry Point
```bash
ferox dashboard          # Launch interactive control dashboard
ferox dashboard --lite   # Minimal version for CI/CD
ferox dashboard --export # Export status as JSON/TOML
```

## Quick Control Commands

### Project Status & Health
```bash
# Get comprehensive project status
ferox status                    # Full status report
ferox health                    # Health metrics
ferox integrity-score           # Project integrity percentage
ferox report --format json      # Export status
ferox report --format markdown  # Export as markdown
```

### Module Management
```bash
# List and verify modules
ferox modules list              # List all modules
ferox modules check             # Verify module visibility
ferox modules info <name>       # Show module details
ferox modules validate          # Validate all modules
ferox modules fix               # Auto-fix module issues

# Module operations
ferox modules enable <name>     # Enable specific module
ferox modules disable <name>    # Disable module
ferox modules reload            # Reload module registry
ferox modules manifest          # Show module manifest
```

### Build & Compilation
```bash
# Build management
ferox build check               # Pre-build validation
ferox build run                 # Compile project
ferox build clean               # Clean build artifacts
ferox build release             # Release build
ferox build --features <list>   # Build with features
ferox build validate            # Validate build config

# Build optimization
ferox build optimize            # Optimize for performance
ferox build time-profile        # Profile build time
ferox build dependency-tree     # Show dependency graph
```

### Testing Controls
```bash
# Test execution
ferox test all                  # Run all tests
ferox test unit                 # Unit tests only
ferox test integration          # Integration tests
ferox test memory               # Memory forensics tests
ferox test module <name>        # Test specific module

# Advanced testing
ferox test --coverage           # Generate coverage report
ferox test --bench              # Run benchmarks
ferox test --re-run             # Re-run failed tests
ferox test --verbose            # Verbose output
ferox test --watch              # Watch mode (auto-run on changes)

# Smart testing with auto-fix
ferox test --autofix            # Auto-fix failing tests
ferox test --profile <name>     # Run predefined test profile
```

### Database Management
```bash
# Database operations
ferox database status           # Show database status
ferox database migrate          # Run migrations
ferox database reset            # Reset databases (with backup)
ferox database backup           # Backup all databases
ferox database restore <file>   # Restore from backup
ferox database export <type>    # Export database
ferox database import <file>    # Import database
```

### Audit & Logging
```bash
# Audit operations
ferox audit view                # Show audit log entries
ferox audit tail                # Real-time log tail
ferox audit export              # Export audit logs
ferox audit search <query>      # Search logs
ferox audit clear               # Clear audit logs
ferox audit stats               # Show audit statistics
```

### Security & Configuration
```bash
# Security management
ferox security check            # Security audit
ferox security policy review    # Review security policy
ferox security policy update    # Update policy
ferox security validate         # Validate security config

# Configuration management
ferox config show               # Show current configuration
ferox config edit               # Edit configuration
ferox config validate           # Validate config
ferox config reset              # Reset to defaults
ferox config export             # Export configuration
ferox config import <file>      # Import configuration
```

### Memory Forensics
```bash
# Memory forensics operations
ferox memory list-dumps         # List available dumps
ferox memory analyze <dump>     # Analyze dump
ferox memory pslist <dump>      # Process listing
ferox memory malfind <dump>     # Malware detection
ferox memory netscan <dump>     # Network analysis
ferox memory hashdump <dump>    # Extract credentials
ferox memory report <dump>      # Generate report
```

### Maintenance Operations
```bash
# Maintenance tasks
ferox maint health              # System health check
ferox maint diagnose            # Full diagnostics
ferox maint fix                 # Auto-fix detected issues
ferox maint fix --modules       # Fix module issues
ferox maint fix --config        # Fix configuration
ferox maint fix --retest        # Fix with automatic testing
ferox maint prune               # Clean temporary files
ferox maint optimize            # Optimize system
```

### Documentation & Help
```bash
# Documentation
ferox docs list                 # List documentation
ferox docs search <topic>       # Search docs
ferox docs view <file>          # View documentation
ferox docs generate             # Generate documentation
ferox docs validate             # Validate documentation

# Help system
ferox help                      # General help
ferox help <command>            # Command-specific help
ferox commands                  # List all commands
ferox completions               # Shell completions
```

## Interactive Dashboard Mode

### Launch Dashboard
```bash
$ ferox dashboard
```

### Dashboard Display
```
════════════════════════════════════════════════════════════════
🦊 FEROX CONTROL DASHBOARD
════════════════════════════════════════════════════════════════

┌─ PROJECT STATUS ─────────────────────────────────────────────┐
│ Version:              2.0.0                                   │
│ Build Status:         ✅ SUCCESS                              │
│ Last Build:           2025-11-12 20:58:00                    │
│ Binary Size:          12.2 MB                                 │
│ Compilation Time:     1.2 seconds                             │
│ Startup Time:         0.11 seconds                            │
└──────────────────────────────────────────────────────────────┘

┌─ HEALTH METRICS ─────────────────────────────────────────────┐
│ Overall Health:       ✅ HEALTHY (98%)                        │
│ Module Registry:      ✅ 52 modules loaded                    │
│ Tests Passing:        ✅ 112/113 (99%)                        │
│ Database Health:      ✅ 2 databases operational              │
│ Audit Trail:          ✅ 1,247 entries logged                 │
│ Configuration:        ✅ Valid                                │
│ Security Policies:    ✅ Enforced                             │
│ Memory Forensics:     ✅ All analyzers ready                  │
└──────────────────────────────────────────────────────────────┘

┌─ MODULE STATUS ──────────────────────────────────────────────┐
│ Scanner:              ✅ 8 modules operational                │
│ Reconnaissance:       ✅ 6 modules operational                │
│ Exploit:              ✅ 4 modules operational                │
│ Post-Exploitation:    ✅ 7 modules operational                │
│ C2 & Evasion:         ✅ 12 modules operational               │
│ Auxiliary:            ✅ 5 modules operational                │
│ Memory Forensics:     ✅ 8 analyzers operational              │
└──────────────────────────────────────────────────────────────┘

┌─ RECENT ACTIVITY ────────────────────────────────────────────┐
│ 20:58:00 ✅ Build completed successfully                     │
│ 20:57:45 ✅ All tests passed                                 │
│ 20:57:30 ✅ Module registry validated                        │
│ 20:57:15 ✅ Security audit completed                         │
│ 20:57:00 ✅ Database migrations applied                      │
└──────────────────────────────────────────────────────────────┘

┌─ QUICK ACTIONS ──────────────────────────────────────────────┐
│ [1] Run Full Test Suite        [2] Execute Build             │
│ [3] Check System Health        [4] View Module Status        │
│ [5] Run Security Audit         [6] Manage Databases          │
│ [7] View Audit Logs            [8] Generate Report           │
│ [9] Run Diagnostics            [0] Exit Dashboard            │
└──────────────────────────────────────────────────────────────┘

Enter command or number (type 'help' for details):
```

## Command Examples & Workflows

### Example 1: Complete Health Check
```bash
# Full system diagnostics
ferox maint diagnose --level full

# Output:
# ════════════════════════════════════════════════
# 🩺 FEROX COMPREHENSIVE DIAGNOSTICS
# ════════════════════════════════════════════════
# 
# BUILD SYSTEM
# ✅ Rust version: 1.82.0
# ✅ Cargo manifest: Valid
# ✅ Dependencies: 45 resolved
# ✅ Feature flags: 5 enabled
# 
# MODULE SYSTEM
# ✅ Registry loaded: 52 modules
# ✅ Memory forensics: 8 modules
# ✅ Metadata: Complete
# ✅ Dependencies: Resolved
# 
# DATABASE SYSTEM
# ✅ SQLite: Operational
# ✅ Sessions DB: 2.3 MB
# ✅ Memory DB: 1.8 MB
# ✅ Migrations: Up to date
# 
# TEST COVERAGE
# ✅ Unit tests: 88/88 passing
# ✅ Integration: 25/25 passing
# ✅ Memory tests: 12/12 passing
# 
# SECURITY
# ✅ Authorization: Enforced
# ✅ Audit logging: Active
# ✅ Safe mode: Functional
# ✅ Policy validation: OK
# 
# FILE STRUCTURE
# ✅ All source files present
# ✅ Documentation: Complete
# ✅ Configuration: Valid
# ✅ Plugin directory: Healthy
# 
# OVERALL STATUS: ✅ ALL SYSTEMS OPERATIONAL
# Integrity Score: 98%
# ════════════════════════════════════════════════
```

### Example 2: Test Suite with Auto-Fix
```bash
# Run all tests with auto-fix enabled
ferox test all --autofix --verbose

# Output:
# 🧪 FEROX TEST EXECUTION
# ═════════════════════════════════════════
# 
# Running unit tests...
#   ✅ core/audit_tests.rs         (0.12s)
#   ✅ core/config_tests.rs        (0.08s)
#   ✅ core/module_tests.rs        (0.15s)
#   ⚠️  handlers/security_tests.rs  (0.18s) - 1 warning
#   ✅ memory_forensics/tests.rs   (0.42s)
# 
# Test Results:
#   Passed:      112/113
#   Failed:      0
#   Warnings:    1
#   Skipped:     2
#   Total Time:  1.23s
# 
# Auto-fix applied:
#   ✅ Fixed 1 deprecation warning in security_tests.rs
# 
# Re-running failed tests...
#   ✅ All tests passing now!
# 
# 🎉 TEST SUITE PASSED (98.2% success rate)
```

### Example 3: Module Management
```bash
# Check all modules and fix issues
ferox modules validate --fix

# Output:
# 📦 MODULE VALIDATION REPORT
# ═════════════════════════════════════════
# 
# Scanner Module
#   ✅ port_scanner        - Operational
#   ✅ http_scanner        - Operational
#   ✅ ftp_scanner         - Operational
#   ✅ ssl_analyzer        - Operational
# 
# Reconnaissance Module
#   ✅ dns_enum            - Operational
#   ✅ whois_lookup        - Operational
#   ✅ asn_discovery       - Operational
#   ✅ subdomain_enum      - Operational
# 
# Memory Forensics Module
#   ✅ dump_parser         - Operational
#   ✅ process_analyzer    - Operational
#   ✅ malware_detector    - Operational
#   ✅ network_analyzer    - Operational
#   ✅ credential_extractor - Operational
#   ✅ mitre_mapper        - Operational
# 
# ✅ ALL MODULES VERIFIED (52/52 operational)
```

### Example 4: Generate Report
```bash
# Generate comprehensive JSON report
ferox report --format json > project_report.json

# Output content:
{
  "timestamp": "2025-11-12T20:58:00Z",
  "version": "2.0.0",
  "status": "healthy",
  "integrity_score": 98,
  "build": {
    "status": "success",
    "duration_seconds": 1.2,
    "binary_size_mb": 12.2,
    "startup_time_ms": 110
  },
  "modules": {
    "total": 52,
    "operational": 52,
    "categories": {
      "scanner": 8,
      "recon": 6,
      "exploit": 4,
      "post": 7,
      "c2_evasion": 12,
      "auxiliary": 5,
      "memory_forensics": 8
    }
  },
  "tests": {
    "total": 113,
    "passed": 112,
    "failed": 0,
    "skipped": 2,
    "pass_rate": 0.982
  },
  "databases": {
    "sessions": { "size_mb": 2.3, "status": "operational" },
    "memory_analysis": { "size_mb": 1.8, "status": "operational" }
  },
  "audit": {
    "entries": 1247,
    "last_entry": "2025-11-12T20:58:00Z"
  }
}
```

### Example 5: Build Optimization
```bash
# Profile and optimize build
ferox build time-profile

# Output:
# 🔨 BUILD TIME ANALYSIS
# ═════════════════════════════════════════
# 
# Top 10 Slowest Compilation Units:
#   1. src/memory_forensics/mod.rs    0.45s (37%)
#   2. src/core/exploit_framework.rs  0.28s (23%)
#   3. src/modules/mod.rs             0.18s (15%)
#   4. src/core/config.rs             0.14s (12%)
#   5. src/cli/app.rs                 0.08s (7%)
#   6. src/handlers/mod.rs            0.06s (5%)
#   7. src/core/session.rs            0.04s (3%)
#   8. src/core/module.rs             0.03s (2%)
# 
# Total Build Time: 1.23s
# 
# Recommendations:
#   • Consider splitting memory_forensics module
#   • Profile incremental builds for faster iteration
#   • Consider using sccache for distributed builds
```

### Example 6: Security Audit
```bash
# Run comprehensive security audit
ferox security check --detailed

# Output:
# 🔒 SECURITY AUDIT REPORT
# ═════════════════════════════════════════
# 
# AUTHORIZATION
#   ✅ AuthorizationContext enforced
#   ✅ Scope validation active
#   ✅ Time-bound checks implemented
# 
# AUDIT LOGGING
#   ✅ Append-only log format
#   ✅ 1,247 entries logged
#   ✅ No log tampering detected
# 
# SAFE MODE
#   ✅ High-risk operations protected
#   ✅ Confirmation prompts active
#   ✅ Mock mode functional
# 
# CONFIGURATION
#   ✅ ferox_security.toml validated
#   ✅ File access controls: 12 rules
#   ✅ Command execution filters: 8 rules
# 
# DATABASE SECURITY
#   ✅ SQLite databases encrypted
#   ✅ Permission bits: 0600
#   ✅ Backups verified
# 
# OVERALL SECURITY: ✅ EXCELLENT (100/100)
```

## Environment Variables

```bash
# Control dashboard behavior
FEROX_DASHBOARD_THEME=dark         # dark, light, auto
FEROX_DASHBOARD_REFRESH=5          # Refresh interval (seconds)
FEROX_DASHBOARD_COLOR=true         # Enable colors
FEROX_DASHBOARD_VERBOSE=true       # Verbose output

# Test configuration
FEROX_TEST_PARALLEL=8              # Parallel test jobs
FEROX_TEST_TIMEOUT=300             # Test timeout (seconds)
FEROX_TEST_REPORT=json             # Report format

# Build configuration
FEROX_BUILD_THREADS=4              # Parallel build threads
FEROX_BUILD_PROFILE=dev            # dev or release
FEROX_STRIP_BINARY=false           # Strip debug symbols

# Log configuration
FEROX_LOG_LEVEL=info               # Logging level
FEROX_LOG_FORMAT=pretty            # pretty or json
```

## Configuration Files

### Dashboard Configuration
```toml
# ~/.ferox/dashboard.toml
[display]
theme = "dark"
colors = true
refresh_interval_ms = 5000
show_timestamps = true
verbose_mode = false

[commands]
confirmation_required = true
auto_backup_before_fix = true
parallel_execution = true

[reporting]
default_format = "json"
export_directory = "~/ferox_reports"
include_metadata = true
include_timing = true

[testing]
auto_retry_failed = true
parallel_tests = 8
coverage_threshold = 80
benchmark_enabled = true
```

## Exit Codes Reference

```bash
0   # Success
1   # General error
2   # Build failed
3   # Tests failed
4   # Configuration error
5   # Database error
6   # Security validation failed
7   # Module not found
8   # Permission denied
9   # Network error
10  # Integrity check failed
```

## Keyboard Shortcuts (Dashboard Mode)

```
q       Quit dashboard
h       Show help
r       Refresh display
s       Show status
t       Run tests
b       Build project
m       Show modules
l       Show logs
d       Run diagnostics
f       Fix issues
c       Clear screen
?       Command help
```

## Integration Examples

### GitHub Actions Integration
```yaml
name: Ferox CI/CD

on: [push, pull_request]

jobs:
  ferox-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Ferox Status Check
        run: |
          cargo install ferox
          ferox dashboard --lite --export status.json
      
      - name: Run Tests
        run: ferox test all --autofix
      
      - name: Security Audit
        run: ferox security check
      
      - name: Upload Report
        uses: actions/upload-artifact@v3
        with:
          name: ferox-report
          path: status.json
```

### Local Pre-Commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "🦊 Running Ferox pre-commit checks..."

# Module validation
ferox modules validate --quiet
if [ $? -ne 0 ]; then
    echo "❌ Module validation failed"
    exit 1
fi

# Quick test
ferox test unit --quiet
if [ $? -ne 0 ]; then
    echo "❌ Unit tests failed"
    exit 1
fi

echo "✅ Pre-commit checks passed"
exit 0
```

---

**CLI Dashboard Version:** 1.0.0
**Last Updated:** 2025-11-12
**Status:** Production Ready

For detailed command help: `ferox help <command>`
