#!/bin/sh
set -e

echo "Waiting for database..."
sleep 3

# Run migrations
if [ -z "$DATABASE_URL" ]; then
    echo "DATABASE_URL is not set. Exiting."
    exit 1
fi

# Run Diesel migrations
diesel setup || echo "Database already initialized"
diesel migration run --database-url "$DATABASE_URL"

# Start application
echo "Starting application..."
exec "$@"