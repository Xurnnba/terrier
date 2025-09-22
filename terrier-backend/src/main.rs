use sea_orm::DatabaseConnection;

use crate::config::Config;

mod config;
mod docs;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
}

// pub async fn create_app(app_state: AppState) -> Router {
//     Router::new()
//         // Auth routes
//         .route("/auth/login", get(auth::handlers::login))
//         .route("/auth/callback", get(auth::handlers::callback))
//         .route("/auth/logout", post(auth::handlers::logout))
//         .route("/auth/me", get(auth::handlers::me))
//         // API docs
//         .merge(SwaggerUi::new("/swagger").url("/openapi.json", ApiDoc::openapi()))
//         .with_state(app_state)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let config = Config::from_env()?;
    // let app_state = AppState {
    //     db: sea_orm::Database::connect(&config.database_url).await?,
    //     config,
    // };

    // let app = create_app(app_state).await;

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    // axum::serve(listener, app).await?;

    Ok(())
}
