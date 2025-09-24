use axum::{
    BoxError, Router,
    error_handling::HandleErrorLayer,
    http::Uri,
    response::IntoResponse,
    routing::{get, post},
};
use axum_oidc::{EmptyAdditionalClaims, OidcAuthLayer, OidcLoginLayer, error::MiddlewareError};
use sea_orm::DatabaseConnection;
use tokio::signal;
use tower::ServiceBuilder;
use tower_sessions::{
    Expiry, MemoryStore, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod config;
mod docs;
mod entities;

use config::Config;
use docs::ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
}

pub async fn create_app(app_state: AppState) -> Result<Router, BoxError> {
    // Set up session management
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(120)));

    let oidc_login_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(OidcLoginLayer::<EmptyAdditionalClaims>::new());

    let config = app_state.config.clone();
    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(
            OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
                Uri::from_maybe_shared(config.api_url).expect("valid API_URL"),
                config.oidc_issuer,
                config.oidc_client_id,
                Some(config.oidc_client_secret),
                vec!["openid".into(), "email".into(), "profile".into()],
            )
            .await
            .unwrap(),
        );

    let router = Router::new()
        // Protected routes
        .route("/auth/me", get(auth::handlers::me))
        .route("/auth/logout", post(auth::handlers::logout))
        // OIDC authentication layer
        .layer(oidc_login_service)
        // Public routes
        .route(
            "/",
            get(|| async { "Visit /swagger for API documentation" }),
        )
        .route("/health", get(|| async { "OK" }))
        .merge(SwaggerUi::new("/swagger").url("/openapi.json", ApiDoc::openapi()))
        // Middleware layers
        .layer(oidc_auth_service)
        .layer(session_layer)
        .with_state(app_state);

    Ok(router)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| "axum_oidc=debug,tower_sessions=debug,sqlx=warn,tower_http=debug".into(),
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let config = Config::from_env()?;
    let app_state = AppState {
        db: sea_orm::Database::connect(&config.database_url).await?,
        config,
    };

    let app = create_app(app_state).await.unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
