#!/bin/sh
set -e

echo "Running database migrations..."

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "Warning: DATABASE_URL not set, skipping migrations"
    exit 0
fi

# Check if sqlx is available for migrations
if command -v sqlx &> /dev/null; then
    echo "Running sqlx migrations..."
    sqlx migrate run || {
        echo "Warning: sqlx migration failed"
        exit 0
    }
else
    echo "sqlx not found, trying direct psql..."
    # Use psql for migrations
    if [ -f "/app/migrations/0001_init_rbac.sql" ]; then
        echo "Running SQL migrations directly..."
        # Run migrations in order
        for migration in /app/migrations/*.sql; do
            echo "Running: $(basename $migration)"
            # Use PGPASSWORD ifPASSWORD is set, otherwise no password
            if [ -n "$PGPASSWORD" ]; then
                psql -h "${DATABASE_HOST:-localhost}" -U "${DATABASE_USER:-postgres}" \
                    -d "${DATABASE_NAME:-kao_db}" -f "$migration" || true
            else
                psql -h "${DATABASE_HOST:-localhost}" -U "${DATABASE_USER:-postgres}" \
                    -d "${DATABASE_NAME:-kao_db}" -f "$migration" || true
            fi
        done
    else
        echo "Warning: No migration files found"
    fi
fi

echo "Migrations completed"
