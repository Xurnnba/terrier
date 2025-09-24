#!/usr/bin/env -S just --justfile

# Show this help message
help:
    @just --list

# Build all services
build:
    docker-compose build

# Start all services
up:
    docker-compose up -d

# Stop all services
down:
    docker-compose down

# Run database migrations
migrate:
    docker-compose --profile migration run --rm migration

# Fresh database (drop all tables and reapply migrations)
fresh:
    docker-compose --profile migration run --rm migration migration fresh

# Check migration status
status:
    docker-compose --profile migration run --rm migration migration status

# Show logs for all services
logs:
    docker-compose logs -f

# Remove all containers, volumes, and images
clean:
    docker-compose down -v --rmi all

# Restart all services
restart: down up

# Restart with rebuild (for code changes)
restart-build: down build up

# Run migrations and start services
setup: migrate up

# Start supporting services, run migrations, and launch apps locally
dev:
    docker-compose up -d postgres minio pgadmin
    cd terrier-backend/migration && cargo run
    cd terrier-backend && cargo run &
    cd terrier-client && bun run dev
