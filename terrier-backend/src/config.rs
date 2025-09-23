#[derive(Clone, Debug)]
pub struct Config {
    pub app_url: String,
    pub database_url: String,
    pub oidc_issuer: String,
    pub oidc_client_id: String,
    pub oidc_client_secret: String,
    pub admin_emails: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let admin_emails = dotenvy::var("ADMIN_EMAILS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_lowercase().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Config {
            app_url: dotenvy::var("APP_URL")?,
            database_url: dotenvy::var("DATABASE_URL")?,
            oidc_issuer: dotenvy::var("OIDC_ISSUER")?,
            oidc_client_id: dotenvy::var("OIDC_CLIENT_ID")?,
            oidc_client_secret: dotenvy::var("OIDC_CLIENT_SECRET")?,
            admin_emails,
        })
    }
}
