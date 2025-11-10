#!/bin/bash
# Phase 3 Demo Script
# Demonstrates new modules in Ferox

echo "=== Ferox Phase 3 Demonstration ==="
echo ""

# Create a test file for OneDrive exfil demo
echo "Test data for demo" > /tmp/ferox_demo_data.txt

# Run Ferox with automated commands
cat <<'EOF' | timeout 10 ./target/release/ferox || true
modules
use c2/teams_tunnel
info
options
back
use post/browser/deep_session_hijack
info
options
back
use auxiliary/cloud/onedrive_sync_exfil
info
options
back
use evasion/edr/silent_shadow
info
options
exit
EOF

echo ""
echo "=== Demo Complete ==="
echo ""
echo "Modules demonstrated:"
echo "  ✓ c2/teams_tunnel - Microsoft Teams C2 channel"
echo "  ✓ post/browser/deep_session_hijack - Browser session extraction"
echo "  ✓ auxiliary/cloud/onedrive_sync_exfil - OneDrive exfiltration"
echo "  ✓ evasion/edr/silent_shadow - EDR detection & evasion"
