#!/bin/bash
# pre-commit hook for Ferox framework
# Install with: cp scripts/pre-commit.sh .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit

set -e

echo "🔍 Running Ferox pre-commit checks..."
echo "────────────────────────────────────"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track failures
FAILED=0

# 1. Check for uncommitted Cargo.lock changes
echo "Checking Cargo.lock..."
if ! cargo update --dry-run > /dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Cargo dependencies may need updating${NC}"
fi

# 2. Run module visibility tests
echo "Checking module visibility..."
if ! cargo test --test module_visibility --lib 2>&1 | grep -q "test result: ok"; then
    echo -e "${RED}❌ Module visibility check failed${NC}"
    FAILED=$((FAILED + 1))
else
    echo -e "${GREEN}✅ Module visibility OK${NC}"
fi

# 3. Check for forbidden patterns in code
echo "Checking for code quality issues..."

# Look for TODO/FIXME in src files (warning only)
if grep -r "TODO\|FIXME" src/ --include="*.rs" 2>/dev/null | grep -v "^Binary"; then
    echo -e "${YELLOW}⚠️  Found TODO/FIXME markers${NC}"
fi

# Check for debug print statements
if grep -r "println!\|dbg!" src/ --include="*.rs" 2>/dev/null | grep -v "^Binary"; then
    echo -e "${YELLOW}⚠️  Found debug statements${NC}"
fi

# 4. Verify documentation consistency
echo "Checking documentation..."
if [ -f "docs/overview.md" ] && [ -f "README.md" ]; then
    echo -e "${GREEN}✅ Documentation present${NC}"
else
    echo -e "${YELLOW}⚠️  Missing key documentation${NC}"
fi

# 5. Check for uncommitted documentation changes
DOCS_CHANGED=$(git diff --cached --name-only | grep -c "docs/" || true)
if [ "$DOCS_CHANGED" -gt 0 ]; then
    echo "  Docs changes: $DOCS_CHANGED file(s)"
fi

# 6. Verify Cargo.toml syntax
echo "Validating Cargo.toml..."
if ! cargo metadata --format-version 1 > /dev/null 2>&1; then
    echo -e "${RED}❌ Cargo.toml has syntax errors${NC}"
    FAILED=$((FAILED + 1))
else
    echo -e "${GREEN}✅ Cargo.toml valid${NC}"
fi

# 7. Check for common merge markers
echo "Checking for merge conflicts..."
if git diff --cached | grep -E "^[<>=]{7}" > /dev/null; then
    echo -e "${RED}❌ Merge conflict markers found${NC}"
    FAILED=$((FAILED + 1))
else
    echo -e "${GREEN}✅ No merge conflicts${NC}"
fi

# 8. Verify no large files are being committed
echo "Checking file sizes..."
LARGE_FILES=$(git diff --cached --name-only --diff-filter=ACM | while read FILE; do
    SIZE=$(stat -f%z "$FILE" 2>/dev/null || stat -c%s "$FILE" 2>/dev/null || echo 0)
    if [ "$SIZE" -gt 5242880 ]; then # 5MB limit
        echo "$FILE: $((SIZE / 1024 / 1024))MB"
    fi
done)

if [ -n "$LARGE_FILES" ]; then
    echo -e "${YELLOW}⚠️  Large files detected:${NC}"
    echo "$LARGE_FILES"
fi

# Summary
echo "────────────────────────────────────"
if [ "$FAILED" -eq 0 ]; then
    echo -e "${GREEN}✅ All pre-commit checks passed!${NC}"
    echo "Ready to commit."
    exit 0
else
    echo -e "${RED}❌ Pre-commit checks failed ($FAILED issue(s))${NC}"
    echo "Please fix issues before committing."
    exit 1
fi
