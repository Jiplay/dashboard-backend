#!/bin/bash

# Dashboard Backend Build & Test Script

set -e  # Exit on error

echo "================================"
echo "Dashboard Backend Build Script"
echo "================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo -e "${YELLOW}1. Checking project with default features...${NC}"
cargo check || {
    echo -e "${RED}Build check failed${NC}"
    exit 1
}
echo -e "${GREEN}✓ Default features check passed${NC}"
echo ""

echo -e "${YELLOW}2. Checking project with all features...${NC}"
cargo check --all-features || {
    echo -e "${RED}All features check failed${NC}"
    exit 1
}
echo -e "${GREEN}✓ All features check passed${NC}"
echo ""

echo -e "${YELLOW}3. Running tests...${NC}"
cargo test --all-features || {
    echo -e "${RED}Tests failed${NC}"
    exit 1
}
echo -e "${GREEN}✓ Tests passed${NC}"
echo ""

echo -e "${YELLOW}4. Building release version...${NC}"
cargo build --release --all-features || {
    echo -e "${RED}Release build failed${NC}"
    exit 1
}
echo -e "${GREEN}✓ Release build successful${NC}"
echo ""

echo -e "${GREEN}================================${NC}"
echo -e "${GREEN}All checks passed successfully!${NC}"
echo -e "${GREEN}================================${NC}"
echo ""
echo "You can now:"
echo "  • Run examples: cargo run --example library_usage"
echo "  • Start server: cargo run --features full --bin dashboard-server"
echo ""
