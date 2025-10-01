use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_oidc::{EmptyAdditionalClaims, OidcClaims};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    AppState,
    entities::{hackathons, prelude::*, user_hackathon_roles, users},
};

#[derive(Serialize, ToSchema)]
pub struct HackathonInfo {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub is_active: bool,
}

/// List all active hackathons
#[utoipa::path(
    get,
    path = "/hackathons",
    responses(
        (status = 200, description = "List of active hackathons", body = Vec<HackathonInfo>)
    ),
    tag = "Hackathons"
)]
pub async fn list_hackathons(
    State(state): State<AppState>,
) -> Result<Json<Vec<HackathonInfo>>, StatusCode> {
    let hackathons = Hackathons::find()
        .filter(hackathons::Column::IsActive.eq(true))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        hackathons
            .into_iter()
            .map(|h| HackathonInfo {
                id: h.id,
                name: h.name,
                slug: h.slug,
                description: h.description,
                start_date: h.start_date,
                end_date: h.end_date,
                is_active: h.is_active,
            })
            .collect(),
    ))
}

#[derive(Serialize, ToSchema)]
pub struct UserRoleResponse {
    pub role: String,
}

/// Get user's role for a specific hackathon
#[utoipa::path(
    get,
    path = "/hackathons/{slug}/role",
    params(
        ("slug" = String, Path, description = "Hackathon slug")
    ),
    responses(
        (status = 200, description = "User's role in this hackathon", body = UserRoleResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "No access to this hackathon"),
        (status = 404, description = "Hackathon not found")
    ),
    tag = "Hackathons"
)]
pub async fn get_user_role(
    claims: OidcClaims<EmptyAdditionalClaims>,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<UserRoleResponse>, StatusCode> {
    let oidc_sub = claims.subject().to_string();
    let email = claims.email().map(|e| e.to_string()).unwrap_or_default();

    // Verify hackathon exists
    let hackathon_exists = Hackathons::find()
        .filter(hackathons::Column::Slug.eq(&slug))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some();

    if !hackathon_exists {
        return Err(StatusCode::NOT_FOUND);
    }

    // Check if global admin
    if state.config.admin_emails.contains(&email.to_lowercase()) {
        return Ok(Json(UserRoleResponse {
            role: "admin".to_string(),
        }));
    }

    // Get user role for the hackathon
    let role = UserHackathonRoles::find()
        .select_only()
        .column(user_hackathon_roles::Column::Role)
        .join(
            JoinType::InnerJoin,
            user_hackathon_roles::Relation::Users.def(),
        )
        .join(
            JoinType::InnerJoin,
            user_hackathon_roles::Relation::Hackathons.def(),
        )
        .filter(users::Column::OidcSub.eq(oidc_sub))
        .filter(hackathons::Column::Slug.eq(slug))
        .into_tuple::<String>()
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match role {
        Some(role) => Ok(Json(UserRoleResponse { role })),
        None => Err(StatusCode::FORBIDDEN),
    }
}
