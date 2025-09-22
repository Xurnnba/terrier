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

Edit `.env` with your OIDC credentials:

```bash
# Database
POSTGRES_PASSWORD=secure_password

# OIDC configuration
OIDC_CLIENT_ID=client_id
OIDC_CLIENT_SECRET=client_secret
OIDC_DISCOVERY_URL=https://accounts.google.com/.well-known/openid_configuration

# Admin configuration
ADMIN_EMAILS=admin@acme.com

RUST_LOG=info
```

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
make help      # Show all available commands
```

## Configuration

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `POSTGRES_PASSWORD` | Yes | Database password |
| `OIDC_CLIENT_ID` | Yes | OAuth client ID |
| `OIDC_CLIENT_SECRET` | Yes | OAuth client secret |
| `OIDC_DISCOVERY_URL` | Yes | OIDC discovery endpoint |
| `ADMIN_EMAILS` | Yes | Comma-separated admin emails |
| `RUST_LOG` | No | Logging level (debug, info, warn, error) |

### Database

The platform uses PostgreSQL. Data is persisted in a Docker volume called `postgres_data`.

### Logs

```bash
# All
make logs

# Backend only
docker-compose logs -f backend

# Database only
docker-compose logs -f postgres
```
