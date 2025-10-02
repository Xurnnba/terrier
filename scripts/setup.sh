#!/usr/bin/env bash
set -euo pipefail

# Parse command line arguments
DEV_MODE=false
if [[ "$#" -gt 0 && "$1" == "--dev" ]]; then
    DEV_MODE=true
fi

# Change to the root directory of the project
ROOT_DIR=$(dirname "$(dirname "$(realpath "$0")")")
cd "$ROOT_DIR"

# Generate unique secure passwords for each service
POSTGRES_PASSWORD=$(python3 -c "import secrets; print(secrets.token_urlsafe(32))")
PGADMIN_PASSWORD=$(python3 -c "import secrets; print(secrets.token_urlsafe(32))")
MINIO_PASSWORD=$(python3 -c "import secrets; print(secrets.token_urlsafe(32))")

# Copy .env.example to .env
cp .env.example .env

# Replace each password individually
sed -i '' "s/POSTGRES_PASSWORD=secure_password/POSTGRES_PASSWORD=$POSTGRES_PASSWORD/" .env
sed -i '' "s/PGADMIN_PASSWORD=secure_password/PGADMIN_PASSWORD=$PGADMIN_PASSWORD/" .env
sed -i '' "s/MINIO_ROOT_PASSWORD=secure_password/MINIO_ROOT_PASSWORD=$MINIO_PASSWORD/" .env

# Development-specific configuration
if [ "$DEV_MODE" = true ]; then
    # Update the DATABASE_URL to use the same password as POSTGRES_PASSWORD
    sed -i '' "s|# DATABASE_URL=postgres://terrier:secure_password@localhost:5432/terrier|DATABASE_URL=postgres://terrier:$POSTGRES_PASSWORD@localhost:5432/terrier|" .env

    # Update ports in APP_URL and API_URL to 8080
    sed -i '' 's|APP_URL=http://localhost:1420|APP_URL=http://localhost:8080|' .env
    sed -i '' 's|API_URL=http://localhost:3000/api|API_URL=http://localhost:8080/api|' .env

    # Change RUST_LOG from info to debug
    sed -i '' 's/RUST_LOG=info/RUST_LOG=debug/' .env

    echo "Finished configuring environment for development"
else
    echo "Finished configuring environment for production"
fi

echo "Make sure you:"
echo "  - Add your email to ADMIN_EMAILS (comma-separated)"
echo "  - Set the OIDC credentials"
if [ "$DEV_MODE" = false ]; then
    echo "  - Update APP_URL and API_URL to your production URLs"
fi
