#!/bin/sh
set -e

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "DATABASE_URL is not set. Exiting."
    exit 1
fi

# Run Diesel migrations
RUST_LOG=warn diesel setup || echo "Database already initialized"
RUST_LOG=warn diesel migration run --database-url "$DATABASE_URL"

# Start application
exec /app/evil_lumios