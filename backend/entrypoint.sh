#!/bin/sh
set -e

echo "Starting kao-backend..."

# Run database migrations before starting
echo "Running database migrations..."
/app/migrate.sh || {
    echo "Warning: Migration script not found, skipping..."
}

echo "Starting kao-backend server..."
exec "$@"
