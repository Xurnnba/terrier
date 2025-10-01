use axum::{
    Json,
    extract::{Query, State},
    http::{StatusCode, Uri},
    response::{IntoResponse, Redirect},
};
use axum_oidc::{EmptyAdditionalClaims, OidcClaims, OidcRpInitiatedLogout};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    AppState,
    entities::{prelude::*, users},
};

#[derive(Serialize, ToSchema)]
pub struct UserInfo {
    id: String,
    email: String,
    name: Option<String>,
    picture: Option<String>,
    is_admin: bool,
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
    State(state): State<AppState>,
    claims: Option<OidcClaims<EmptyAdditionalClaims>>,
) -> Result<Json<UserInfo>, StatusCode> {
    match claims {
        Some(claims) => {
            let email = claims
                .0
                .email()
                .map(|s| s.to_string())
                .ok_or(StatusCode::UNAUTHORIZED)?;

            let is_admin = state.config.admin_emails.contains(&email.to_lowercase());

            let oidc_sub = claims.0.subject().to_string();
            let user = Users::find()
                .filter(users::Column::OidcSub.eq(&oidc_sub))
                .one(&state.db)
                .await
                .ok()
                .flatten();

            user.map(|user| {
                Json(UserInfo {
                    id: user.id.to_string(),
                    email: user.email,
                    name: user.name,
                    picture: user.picture,
                    is_admin,
                })
            })
            .ok_or(StatusCode::UNAUTHORIZED)
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
