# Setup

## Prerequisites

- **Docker**
- **OIDC Provider** (Google, GitHub, Okta, etc.)

## Quick Start

### 1. Clone

```bash
# Clone the repository
git clone https://github.com/ScottyLabs/terrier.git
cd terrier

# Copy environment template
cp .env.example .env
```

### 2. Configure

```bash
# Generate secure passwords for PosgreSQL, pgAdmin, and MinIO
python3 -c "import secrets; print(secrets.token_urlsafe(32))"
```

Modify `.env` with these values, your OIDC credentials, and admin emails (comma-separated).

### 3. Start

```bash
make setup    # Runs migrations and starts services
make logs     # View logs
```

## Setting Up OIDC Providers

Most OIDC providers follow similar patterns. You need:

- Client ID and Secret
- Discovery URL (usually `https://provider.com/.well-known/openid-configuration`)
- Callback URL pointing to the backend

## Available Commands

```bash
make help      # Show all available commands
make build     # Build all services
make up        # Start all services
make down      # Stop all services
make migrate   # Run database migrations
make fresh     # Fresh database (drop all tables and reapply migrations)
make status    # Check migration status
make logs      # Show logs for all services
make clean     # Remove all containers, volumes, and images
make restart   # Restart all services
make setup     # Run migrations and start services
make dev       # Start supporting services, run migrations, and launch backend locally
```

## Configuration

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `POSTGRES_PASSWORD` | Yes | Database password |
| `PGADMIN_EMAIL` | No | pgAdmin admin email (default: <admin@terrier.local>) |
| `PGADMIN_PASSWORD` | Yes | pgAdmin admin password |
| `MINIO_ROOT_USER` | No | MinIO admin username (default: minioadmin) |
| `MINIO_ROOT_PASSWORD` | Yes | MinIO admin password |
| `S3_BUCKET_NAME` | No | Storage bucket name (default: terrier-files) |
| `OIDC_CLIENT_ID` | Yes | OAuth client ID |
| `OIDC_CLIENT_SECRET` | Yes | OAuth client secret |
| `OIDC_DISCOVERY_URL` | Yes | OIDC discovery endpoint |
| `ADMIN_EMAILS` | Yes | Comma-separated admin emails |
| `RUST_LOG` | No | Logging level [debug, info, warn, error] (default: info) |

### Volumes

The platform uses PostgreSQL. Data is persisted in a Docker volume called `postgres_data`, and pgAdmin data is in the `pgadmin_data` volume. The storage bucket uses MinIO, with data in the `minio_data` volume.

### Logs

```bash
# All
make logs

# Backend only
docker-compose logs -f backend

# Database only
docker-compose logs -f postgres

# Bucket only
docker-compose logs -f minio
```
