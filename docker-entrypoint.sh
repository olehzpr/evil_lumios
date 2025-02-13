#!/bin/sh
set -e

echo "Waiting for database to be ready..."
sleep 3

# Run migrations
if [ -z "$DATABASE_URL" ]; then
    echo "DATABASE_URL is not set. Exiting."
    exit 1
fi

# Run Diesel migrations
echo "DATABASE_URL=$DATABASE_URL" > .env
diesel setup || echo "Database already initialized"
diesel migration run --database-url "$DATABASE_URL"

# Start application
echo "Starting application..."
echo "/ directory"
ls -la /
echo "/app directory"
ls -la /app
exec /app/evil_lumios