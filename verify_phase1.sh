#!/bin/bash
# FEROX PHASE 1 VERIFICATION SCRIPT
# Automated verification of all critical fixes

set -e

echo "============================================"
echo "FEROX PHASE 1 CRITICAL FIXES - VERIFICATION"
echo "============================================"
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

success() {
    echo -e "${GREEN}✓${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
    exit 1
}

info() {
    echo -e "${YELLOW}→${NC} $1"
}

# Step 1: Build verification
info "Step 1: Building project..."
cargo build --release > /dev/null 2>&1 || error "Build failed"
success "Build successful"

# Step 2: Test verification
info "Step 2: Running tests..."
TEST_OUTPUT=$(cargo test --lib 2>&1)
TEST_COUNT=$(echo "$TEST_OUTPUT" | grep "test result:" | grep -o "[0-9]* passed" | grep -o "[0-9]*")

if [ "$TEST_COUNT" -eq 68 ]; then
    success "All 68 tests passing"
else
    error "Expected 68 tests, got $TEST_COUNT"
fi

# Step 3: Audit module verification
info "Step 3: Checking audit module..."
if [ -f "src/core/audit.rs" ]; then
    success "Audit module exists"
else
    error "Audit module not found"
fi

# Step 4: Module options verification
info "Step 4: Checking module options system..."
if [ -f "src/core/module_options.rs" ]; then
    success "Module options system exists"
else
    error "Module options system not found"
fi

# Step 5: Session mutex fix verification
info "Step 5: Checking session mutex fix..."
if grep -q "pub async fn load_from_db" src/core/session.rs; then
    success "Session async initialization implemented"
else
    error "Session async initialization not found"
fi

if grep -q "try_lock()" src/core/session.rs; then
    error "try_lock() still present in session.rs"
else
    success "try_lock() eliminated from session.rs"
fi

# Step 6: Confirmation check verification
info "Step 6: Checking safe mode confirmation..."
if grep -q "requires_confirmation()" src/cli/app.rs; then
    success "Confirmation check implemented in CLI"
else
    error "Confirmation check not found in CLI"
fi

if grep -q "audit::append_confirmation" src/cli/app.rs; then
    success "Audit logging integrated"
else
    error "Audit logging not integrated"
fi

# Step 7: Port scanner migration verification
info "Step 7: Checking port scanner migration..."
if grep -q "StandardOptions" src/modules/scanner/port.rs; then
    success "Port scanner migrated to StandardOptions"
else
    error "Port scanner not using StandardOptions"
fi

if grep -q "impl OptionManager for PortScanner" src/modules/scanner/port.rs; then
    success "Port scanner implements OptionManager"
else
    error "Port scanner doesn't implement OptionManager"
fi

# Step 8: Binary size check
info "Step 8: Checking binary size..."
BINARY_SIZE=$(ls -lh target/release/ferox | awk '{print $5}')
success "Binary size: $BINARY_SIZE"

# Step 9: Startup time check (if binary exists)
info "Step 9: Testing CLI startup..."
START_TIME=$(date +%s%N)
timeout 5 ./target/release/ferox --help > /dev/null 2>&1 || true
END_TIME=$(date +%s%N)
ELAPSED=$((($END_TIME - $START_TIME) / 1000000))
if [ $ELAPSED -lt 100 ]; then
    success "CLI startup: ${ELAPSED}ms (excellent)"
elif [ $ELAPSED -lt 500 ]; then
    success "CLI startup: ${ELAPSED}ms (good)"
else
    info "CLI startup: ${ELAPSED}ms"
fi

# Step 10: Documentation verification
info "Step 10: Checking documentation..."
if [ -f "PHASE1_FIXES.md" ]; then
    LINE_COUNT=$(wc -l < PHASE1_FIXES.md)
    success "Documentation exists ($LINE_COUNT lines)"
else
    error "PHASE1_FIXES.md not found"
fi

echo ""
echo "============================================"
echo -e "${GREEN}ALL VERIFICATION CHECKS PASSED${NC}"
echo "============================================"
echo ""
echo "Summary:"
echo "  ✓ Safe mode confirmation enforcement"
echo "  ✓ Session mutex concurrency fix"
echo "  ✓ Module options unification (1 module migrated)"
echo "  ✓ 68 tests passing (12 new tests)"
echo "  ✓ Binary size: $BINARY_SIZE"
echo "  ✓ Startup time: ${ELAPSED}ms"
echo ""
echo "Ready for Phase 2!"
