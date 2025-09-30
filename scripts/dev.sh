#!/usr/bin/env bash
set -euo pipefail

cleanup() {
    echo "Cleaning up..."
    kill $(jobs -p) 2>/dev/null || true
    docker-compose stop postgres minio pgadmin nginx-dev
    exit
}

trap cleanup SIGINT SIGTERM

ROOT_DIR=$(pwd)

# Start Docker services
docker-compose up -d postgres minio pgadmin nginx-dev

# Run migrations
cd "$ROOT_DIR/terrier-backend/migration"
cargo run

# Start backend in background and wait for it to be ready
cd "$ROOT_DIR/terrier-backend"
cargo run &

echo "Waiting for backend to be ready..."
until curl -s http://localhost:3000/api/openapi.json > /dev/null; do
    sleep 1
done
echo "Backend is ready"

# Sync OpenAPI spec
cd "$ROOT_DIR/terrier-client"
bun run generate-types

# Start frontend in background
bun run dev &
echo "Visit http://localhost:8080 in your browser"

# Wait for all background jobs
wait
