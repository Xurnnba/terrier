# Terrier

## Getting started

See [SETUP.md](./SETUP.md) for instructions on how to host Terrier.

## Development

1. First, copy the environment variables to `.env`. You may want to customize some of these for development:

    ```bash
    # Application Configuration
    APP_URL=http://localhost:8080
    API_URL=http://localhost:8080/api

    # For local development
    DATABASE_URL=postgres://terrier:secure_password@localhost:5432/terrier

    # Admin Configuration
    RUST_LOG=debug
    ```

2. This project uses `just`, a make-like command runner. After [installing it](https://github.com/casey/just?tab=readme-ov-file#packages), you can use the following command to start the application in development mode:

    ```bash
    just dev
    ```

    Then visit [http://localhost:8080](http://localhost:8080) in the browser.
