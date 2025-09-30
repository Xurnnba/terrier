use axum::{
    Json,
    extract::{Query, State},
    http::{StatusCode, Uri},
    response::{IntoResponse, Redirect},
};
use axum_oidc::{EmptyAdditionalClaims, OidcClaims, OidcRpInitiatedLogout};
use serde::{Deserialize, Serialize};
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
    path = "/auth/status",
    responses(
        (status = 200, description = "User information", body = UserInfo),
        (status = 401, description = "Not authenticated")
    ),
    tag = "Authentication"
)]
#[axum::debug_handler]
pub async fn status(
    claims: Option<OidcClaims<EmptyAdditionalClaims>>,
) -> Result<Json<UserInfo>, StatusCode> {
    match claims {
        Some(claims) => {
            let user_info = UserInfo {
                sub: claims.0.subject().to_string(),
                email: claims.0.email().map(|e| e.to_string()),
                name: claims
                    .0
                    .name()
                    .and_then(|name| name.get(None))
                    .map(|n| n.to_string()),
            };
            Ok(Json(user_info))
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct LoginQuery {
    redirect_uri: Option<String>,
}

/// Initiate login flow
#[utoipa::path(
    get,
    path = "/auth/login",
    params(
        ("redirect_uri" = Option<String>, Query, description = "URI to redirect to after login")
    ),
    responses(
        (status = 302, description = "Redirect to app after login")
    ),
    tag = "Authentication"
)]
pub async fn login(
    _claims: OidcClaims<EmptyAdditionalClaims>,
    State(state): State<AppState>,
    Query(params): Query<LoginQuery>,
) -> impl IntoResponse {
    // OidcLoginLayer will have handled login, so redirect the user back at this point
    let redirect_to = params
        .redirect_uri
        .filter(|uri| uri.starts_with(&state.config.app_url))
        .unwrap_or_else(|| state.config.app_url.clone());

    Redirect::to(&redirect_to)
}

/// Log the current user out
#[utoipa::path(
    get,
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
