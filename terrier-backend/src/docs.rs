use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::handlers::logout,
        crate::auth::handlers::me,
    ),
    components(schemas(
        crate::auth::handlers::UserInfo,
    )),
    tags(
        (name = "Authentication", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;
