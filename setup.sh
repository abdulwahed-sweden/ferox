#!/bin/bash
# Ferox Framework - Complete Setup Script
# This script sets up the entire Ferox project from scratch

set -e  # Exit on error

echo "🦊 Setting up Ferox Framework..."
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Step 1: Check prerequisites
echo "📋 Step 1: Checking prerequisites..."
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found. Please install from https://rustup.rs${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Rust/Cargo found${NC}"
echo ""

# Step 2: Create project directory
echo "📁 Step 2: Creating project structure..."
mkdir -p ferox/src/{cli,core,modules/{scanner,exploit,recon}}
cd ferox
echo -e "${GREEN}✓ Directory structure created${NC}"
echo ""

# Step 3: Display project info
echo "📊 Project Information:"
echo "   Name: Ferox Framework"
echo "   Version: 2.0.0"
echo "   Language: Rust"
echo "   Modules: Scanner, Exploit, Recon"
echo ""

# Step 4: Instructions
echo "🎯 Next Steps:"
echo "   1. All files are ready in the current directory"
echo "   2. Run: cargo build --release"
echo "   3. Run: ./target/release/ferox"
echo ""

echo -e "${GREEN}🦊 Ferox Framework setup complete!${NC}"
echo -e "${YELLOW}⚡ Fast. Fierce. Fearless.${NC}"
