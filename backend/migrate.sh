#!/bin/sh
set -e

echo "Running database migrations..."

# Let the backend application handle migrations internally
# The entrypoint will pass arguments to kao-backend which handles migrations

echo "Starting kao-backend..."
exec "$@"
