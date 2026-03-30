#!/bin/sh
set -e

echo "Database migrations are handled automatically by the backend application on startup."
echo "Starting kao-backend..."
exec "$@"
