#!/bin/bash
# Test coverage script for kao-backend
# Uses cargo-llvm-cov for coverage reporting
#
# Usage:
#   ./scripts/test-coverage.sh          # Run all tests with coverage
#   ./scripts/test-coverage.sh unit     # Run only unit tests with coverage
#   ./scripts/test-coverage.sh integration  # Run only integration tests with coverage
#   ./scripts/test-coverage.sh --html    # Generate HTML report

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
COVERAGE_DIR="$PROJECT_DIR/target/coverage"

# Default flags
TEST_TARGET=""
REPORT_FORMAT="text"
OPEN_REPORT=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        unit)
            TEST_TARGET="--test unit"
            shift
            ;;
        integration)
            TEST_TARGET="--test integration"
            shift
            ;;
        --html)
            REPORT_FORMAT="html"
            shift
            ;;
        --open)
            OPEN_REPORT=true
            shift
            ;;
        --lcov)
            REPORT_FORMAT="lcov"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [unit|integration] [--html] [--open] [--lcov]"
            exit 1
            ;;
    esac
done

echo -e "${YELLOW}Running test coverage...${NC}"

# Ensure cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo -e "${YELLOW}Installing cargo-llvm-cov...${NC}"
    cargo install cargo-llvm-cov
fi

# Create coverage directory
mkdir -p "$COVERAGE_DIR"

# Run tests with coverage
case "$REPORT_FORMAT" in
    text)
        cargo llvm-cov --all-features --no-fail-fast $TEST_TARGET
        ;;
    html)
        cargo llvm-cov --all-features --no-fail-fast $TEST_TARGET --html --output-dir "$COVERAGE_DIR/html"
        echo -e "${GREEN}HTML coverage report generated at: $COVERAGE_DIR/html/index.html${NC}"
        if [ "$OPEN_REPORT" = true ]; then
            open "$COVERAGE_DIR/html/index.html"
        fi
        ;;
    lcov)
        cargo llvm-cov --all-features --no-fail-fast $TEST_TARGET --lcov --output-path "$COVERAGE_DIR/lcov.info"
        echo -e "${GREEN}LCOV coverage report generated at: $COVERAGE_DIR/lcov.info${NC}"
        ;;
esac

# Show summary
echo ""
echo -e "${YELLOW}Coverage Summary${NC}"
cargo llvm-cov --all-features --no-fail-fast $TEST_TARGET --summary-only 2>/dev/null || true

# Check coverage percentage
COVERAGE=$(cargo llvm-cov --all-features --no-fail-fast $TEST_TARGET --summary-only 2>/dev/null | grep -oP 'TOTAL.*?\K\d+%' | tail -1 || echo "unknown")
echo ""
if [ "$COVERAGE" != "unknown" ]; then
    COVERAGE_NUM=${COVERAGE%\%}
    if [ "$COVERAGE_NUM" -ge 50 ]; then
        echo -e "${GREEN}Coverage: $COVERAGE (target: >50%) - PASSED${NC}"
    else
        echo -e "${RED}Coverage: $COVERAGE (target: >50%) - NEEDS IMPROVEMENT${NC}"
    fi
fi
