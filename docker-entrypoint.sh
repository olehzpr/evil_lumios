#!/bin/sh
set -e

# Run migrations
echo "Running database migrations..."
diesel migration run

# Start application
echo "Starting application..."
exec "$@"