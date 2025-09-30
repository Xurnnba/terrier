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
just setup   # Runs migrations and starts services
just logs    # View logs
```

## Setting Up OIDC Providers

Most OIDC providers follow similar patterns. You need:

- Client ID and Secret
- Issuer URL
- Callback URL pointing to the backend (`/api/auth/callback`)

## Available Commands

```bash
just build   # Build all services
just clean   # Remove all containers, volumes, and images
just dev     # Start supporting services, run migrations, and launch apps locally
just down    # Stop all services
just fresh   # Fresh database (drop all tables and reapply migrations)
just help    # Show this help message
just logs    # Show logs for all services
just migrate # Run database migrations
just restart # Restart all services
just setup   # Run migrations and start services
just status  # Check migration status
just up      # Start all services
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
just logs

# Backend only
docker-compose logs -f backend

# Database only
docker-compose logs -f postgres

# Bucket only
docker-compose logs -f minio
```
