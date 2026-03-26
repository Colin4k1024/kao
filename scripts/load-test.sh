#!/bin/bash

# Load Testing Script for Kao Backend
# Usage: ./load-test.sh [OPTIONS]
#
# Options:
#   -t, --time <seconds>     Duration of the test (default: 60)
#   -r, --rate <rps>         Requests per second (default: 10)
#   -n, --num <users>        Number of users (default: 100)
#   -h, --host <host>        Host URL (default: http://localhost:8080)
#   -o, --output <file>      Output file for results (default: load-test-results.json)
#   -l, --log <file>         Log file (default: load-test.log)
#   -H, --help               Show help

set -e

# Default values
TIME=60
RATE=10
NUM_USERS=100
HOST="http://localhost:8080"
OUTPUT="load-test-results.json"
LOG="load-test.log"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--time)
            TIME="$2"
            shift 2
            ;;
        -r|--rate)
            RATE="$2"
            shift 2
            ;;
        -n|--num)
            NUM_USERS="$2"
            shift 2
            ;;
        -h|--host)
            HOST="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT="$2"
            shift 2
            ;;
        -l|--log)
            LOG="$2"
            shift 2
            ;;
        -H|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Load Testing Script for Kao Backend"
            echo ""
            echo "Options:"
            echo "  -t, --time <seconds>     Duration of the test (default: 60)"
            echo "  -r, --rate <rps>         Requests per second (default: 10)"
            echo "  -n, --num <users>        Number of users (default: 100)"
            echo "  -h, --host <host>        Host URL (default: http://localhost:8080)"
            echo "  -o, --output <file>      Output file for results (default: load-test-results.json)"
            echo "  -l, --log <file>         Log file (default: load-test.log)"
            echo "  -H, --help               Show help"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Check if Locust is installed
if ! command -v locust &> /dev/null; then
    echo "Locust is not installed. Installing..."
    pip install locust requests pandas
fi

# Run load test
echo "Starting load test..."
echo "  Duration: ${TIME}s"
echo "  Rate: ${RATE} req/s"
echo "  Users: ${NUM_USERS}"
echo "  Host: ${HOST}"
echo "  Output: ${OUTPUT}"

locust -f load-test.py \
    --host "${HOST}" \
    --users ${NUM_USERS} \
    --spawn-rate ${RATE} \
    --run-time "${TIME}s" \
    --headless \
    --csv "${OUTPUT%.*}" \
    --loglevel INFO 2>&1 | tee "${LOG}"

# Analyze results
echo ""
echo "Load Test Complete!"
echo "Results saved to: ${OUTPUT}.csv"
echo "Log saved to: ${LOG}"

# Generate summary
echo ""
echo "=== Load Test Summary ==="

# Parse CSV files for summary
if [ -f "${OUTPUT%.*}_stats.csv" ]; then
    echo ""
    echo "Stats:"
    head -20 "${OUTPUT%.*}_stats.csv"
fi

if [ -f "${OUTPUT%.*}_failures.csv" ]; then
    echo ""
    echo "Failures:"
    cat "${OUTPUT%.*}_failures.csv"
fi

echo ""
echo "For detailed results, see: ${OUTPUT}"
