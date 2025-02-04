#!/bin/sh
set -e

# Run migrations
if [ -z "$DATABASE_URL" ]; then
    echo "DATABASE_URL is not set. Exiting."
    exit 1
fi

echo "DATABASE_URL: $DATABASE_URL"

# Run Diesel migrations
diesel migration run --database-url "$DATABASE_URL"

# Start application
echo "Starting application..."
exec "$@"