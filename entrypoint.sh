#!/bin/bash
set -e

# Run database migrations
diesel migration run

# Start the application
./evil_lumios