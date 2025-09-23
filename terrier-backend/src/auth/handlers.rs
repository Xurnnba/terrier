use axum::{Json, extract::State, http::Uri, response::IntoResponse};
use axum_oidc::{EmptyAdditionalClaims, OidcClaims, OidcRpInitiatedLogout};
use serde::Serialize;
use utoipa::ToSchema;

use crate::AppState;

#[derive(Serialize, ToSchema)]
pub struct UserInfo {
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

/// Get current user information
#[utoipa::path(
    get,
    path = "/auth/me",
    responses(
        (status = 200, description = "User information", body = UserInfo),
        (status = 401, description = "Not authenticated")
    ),
    tag = "Authentication"
)]
#[axum::debug_handler]
pub async fn me(claims: OidcClaims<EmptyAdditionalClaims>) -> impl IntoResponse {
    let user_info = UserInfo {
        sub: claims.0.subject().to_string(),
        email: claims.0.email().map(|e| e.to_string()),
        name: claims
            .0
            .name()
            .and_then(|name| name.get(None))
            .map(|n| n.to_string()),
    };

    Json(user_info)
}

/// Log the current user out
#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 302, description = "Redirect to logout")
    ),
    tag = "Authentication"
)]
#[axum::debug_handler]
pub async fn logout(
    logout: OidcRpInitiatedLogout,
    State(state): State<AppState>,
) -> impl IntoResponse {
    logout
        .with_post_logout_redirect(
            Uri::from_maybe_shared(state.config.app_url).expect("valid APP_URL"),
        )
        .into_response()
}
