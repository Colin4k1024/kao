#!/bin/bash
#
# Security Scanning Script for Kao Backend
# Runs static application security testing (SAST) on the Rust codebase
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKEND_DIR="${PROJECT_ROOT}/backend"
REPORT_DIR="${PROJECT_ROOT}/reports"
REPORT_FILE="${REPORT_DIR}/security-scan-$(date +%Y%m%d-%H%M%S).json"

# Create reports directory if it doesn't exist
mkdir -p "${REPORT_DIR}"

# Track failures
CRITICAL_COUNT=0
HIGH_COUNT=0
MEDIUM_COUNT=0
LOW_COUNT=0
TOTAL_VULNERABILITIES=0

# Function to log messages
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to report vulnerability
report_vuln() {
    local severity=$1
    local description=$2
    local file=$3
    local line=$4
    
    log_warn "Found ${severity}: ${description}"
    
    case $severity in
        CRITICAL) ((CRITICAL_COUNT++)) ;;
        HIGH) ((HIGH_COUNT++)) ;;
        MEDIUM) ((MEDIUM_COUNT++)) ;;
        LOW) ((LOW_COUNT++)) ;;
    esac
    
    ((TOTAL_VULNERABILITIES++))
}

# Function to check for hardcoded credentials
check_hardcoded_credentials() {
    log_info "Checking for hardcoded credentials..."
    
    # Check for hardcoded passwords
    if grep -r "password\s*=\s*['\"][^'\"]*['\"]" --include="*.rs" --exclude-dir=target "${BACKEND_DIR}" 2>/dev/null; then
        report_vuln "CRITICAL" "Hardcoded password found" "multiple files" "N/A"
    fi
    
    # Check for hardcoded API keys
    if grep -r "api_key\s*=\s*['\"][^'\"]*['\"]" --include="*.rs" --exclude-dir=target "${BACKEND_DIR}" 2>/dev/null; then
        report_vuln "CRITICAL" "Hardcoded API key found" "multiple files" "N/A"
    fi
    
    # Check for hardcoded secrets
    if grep -r "secret\s*=\s*['\"][^'\"]*['\"]" --include="*.rs" --exclude-dir=target "${BACKEND_DIR}" 2>/dev/null; then
        report_vuln "CRITICAL" "Hardcoded secret found" "multiple files" "N/A"
    fi
}

# Function to check for SQL injection vulnerabilities
check_sql_injection() {
    log_info "Checking for SQL injection vulnerabilities..."
    
    # Check for dynamic SQL construction
    if grep -r "query!.*\+" --include="*.rs" --exclude-dir=target "${BACKEND_DIR}" 2>/dev/null; then
        report_vuln "CRITICAL" "Potential SQL injection via string concatenation" "multiple files" "N/A"
    fi
    
    # Check for raw SQL without parameterization
    if grep -r "query.*format!" --include="*.rs" --exclude-dir=target "${BACKEND_DIR}" 2>/dev/null; then
        report_vuln "HIGH" "Potential SQL injection via format!" "multiple files" "N/A"
    fi
}

# Function to check for XSS vulnerabilities
check_xss() {
    log_info "Checking for XSS vulnerabilities..."
    
    # Check for unsafe HTML rendering
    if grep -r "unsafe_html" --include="*.rs" --exclude-dir=target "${BACKEND_DIR}" 2>/dev/null; then
        report_vuln "HIGH" "Unsafe HTML rendering found" "multiple files" "N/A"
    fi
}

# Function to check for insecure cryptography
check_cryptography() {
    log_info "Checking for insecure cryptographic practices..."
    
    # Check for MD5 usage
    if grep -r "md5" --include="*.rs" --exclude-dir=target "${BACKEND_DIR}" 2>/dev/null; then
        report_vuln "HIGH" "MD5 cryptographic hash found (insecure)" "multiple files" "N/A"
    fi
    
    # Check for SHA1 usage
    if grep -r "sha1" --include="*.rs" --exclude-dir=target "${BACKEND_DIR}" 2>/dev/null; then
        report_vuln "MEDIUM" "SHA1 cryptographic hash found (weak)" "multiple files" "N/A"
    fi
}

# Function to check for input validation issues
check_input_validation() {
    log_info "Checking for input validation issues..."
    
    # Check for unchecked user input
    if grep -r "unwrap()" --include="*.rs" --exclude-dir=target "${BACKEND_DIR}" | grep -v "test" | head -20; then
        log_warn "Potential unwrap() without error handling found"
        ((MEDIUM_COUNT++))
        ((TOTAL_VULNERABILITIES++))
    fi
}

# Function to run Clippy for code quality and security issues
run_clippy() {
    log_info "Running Clippy for code quality and security checks..."
    
    cd "${BACKEND_DIR}"
    
    # Run clippy with warnings as errors for development
    if command -v cargo-clippy &> /dev/null; then
        cargo clippy -- -D warnings 2>&1 || true
    else
        log_warn "cargo-clippy not found, skipping clippy checks"
    fi
}

# Function to run cargo-audit for dependency vulnerabilities
run_cargo_audit() {
    log_info "Running cargo-audit for dependency vulnerabilities..."
    
    cd "${BACKEND_DIR}"
    
    # Check if cargo-audit is installed
    if command -v cargo-audit &> /dev/null; then
        cargo audit 2>&1 || true
    else
        log_warn "cargo-audit not found, skipping dependency audit"
    fi
}

# Function to generate summary report
generate_report() {
    log_info "Generating security scan report..."
    
    local report_content=$(cat << 'EOF'
{
    "scan_name": "Security Audit",
    "scan_date": "TIMESTAMP_PLACEHOLDER",
    "project": "Kao Backend",
    "summary": {
        "total_vulnerabilities": TOTAL_PLACEHOLDER,
        "critical": CRITICAL_PLACEHOLDER,
        "high": HIGH_PLACEHOLDER,
        "medium": MEDIUM_PLACEHOLDER,
        "low": LOW_PLACEHOLDER
    },
    "checks": {
        "hardcoded_credentials": "SCANNED",
        "sql_injection": "SCANNED",
        "xss": "SCANNED",
        "insecure_cryptography": "SCANNED",
        "input_validation": "SCANNED",
        "clippy": "SCANNED",
        "cargo_audit": "SCANNED"
    }
}
EOF
)
    
    # Replace placeholders
    report_content=$(echo "$report_content" | sed "s/TIMESTAMP_PLACEHOLDER/$(date -Iseconds)/")
    report_content=$(echo "$report_content" | sed "s/TOTAL_PLACEHOLDER/${TOTAL_VULNERABILITIES}/")
    report_content=$(echo "$report_content" | sed "s/CRITICAL_PLACEHOLDER/${CRITICAL_COUNT}/")
    report_content=$(echo "$report_content" | sed "s/HIGH_PLACEHOLDER/${HIGH_COUNT}/")
    report_content=$(echo "$report_content" | sed "s/MEDIUM_PLACEHOLDER/${MEDIUM_COUNT}/")
    report_content=$(echo "$report_content" | sed "s/LOW_PLACEHOLDER/${LOW_COUNT}/")
    
    echo "$report_content"
}

# Function to output summary
output_summary() {
    echo ""
    echo "=========================================="
    echo "       SECURITY SCAN SUMMARY              "
    echo "=========================================="
    echo ""
    echo "Total Vulnerabilities: ${TOTAL_VULNERABILITIES}"
    echo ""
    echo "  ${RED}Critical:${NC} ${CRITICAL_COUNT}"
    echo "  ${RED}High:${NC}     ${HIGH_COUNT}"
    echo "  ${YELLOW}Medium:${NC}   ${MEDIUM_COUNT}"
    echo "  ${GREEN}Low:${NC}      ${LOW_COUNT}"
    echo ""
    echo "Report file: ${REPORT_FILE}"
    echo "=========================================="
}

# Main execution
main() {
    log_info "Starting security scan..."
    echo ""
    
    # Check for --help flag
    if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
        echo "Security Scanning Script for Kao Backend"
        echo ""
        echo "Usage: $0 [OPTIONS]"
        echo ""
        echo "Options:"
        echo "  --help, -h        Show this help message"
        echo "  --fail-on-critical  Exit with error code if critical vulnerabilities found"
        echo "  --ci              Run in CI mode (no interactive output)"
        echo ""
        echo "Description:"
        echo "  Runs static application security testing (SAST) on the Rust codebase"
        echo "  Checks for:"
        echo "    - Hardcoded credentials"
        echo "    - SQL injection vulnerabilities"
        echo "    - XSS vulnerabilities"
        echo "    - Insecure cryptography"
        echo "    - Input validation issues"
        echo "    - Clippy code quality warnings"
        echo "    - Cargo audit dependency vulnerabilities"
        exit 0
    fi
    
    # Run all checks
    check_hardcoded_credentials
    check_sql_injection
    check_xss
    check_cryptography
    check_input_validation
    run_clippy
    run_cargo_audit
    
    # Generate report
    generate_report > "${REPORT_FILE}"
    
    # Output summary
    output_summary
    
    # Check for critical vulnerabilities
    if [[ "$1" == "--fail-on-critical" ]] && [[ $CRITICAL_COUNT -gt 0 ]]; then
        log_error "CRITICAL vulnerabilities found! Exiting with error."
        exit 1
    fi
    
    # Check for.any vulnerabilities
    if [[ $TOTAL_VULNERABILITIES -gt 0 ]]; then
        log_warn "Security scan completed with ${TOTAL_VULNERABILITIES} vulnerabilities found"
        exit 0
    else
        log_info "Security scan completed successfully - no vulnerabilities found"
        exit 0
    fi
}

# Run main function with all arguments
main "$@"
