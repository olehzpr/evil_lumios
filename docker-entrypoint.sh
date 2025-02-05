#!/bin/sh
set -e

until pg_isready --dbname=$DATABASE_URL; do
  echo "Waiting for database..."
  sleep 2
done

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