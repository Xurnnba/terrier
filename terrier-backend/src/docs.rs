use utoipa::{
    Modify, OpenApi,
    openapi::{
        SecurityRequirement,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

use crate::{auth, hackathons};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::handlers::status,
        auth::handlers::login,
        auth::handlers::logout,
        hackathons::handlers::list_hackathons,
        hackathons::handlers::get_user_role,
    ),
    components(schemas(
        auth::handlers::LoginQuery,
        auth::handlers::UserInfo,
        hackathons::handlers::HackathonInfo,
        hackathons::handlers::UserRoleResponse,
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "Hackathons", description = "Hackathon endpoints"),
        (name = "Authentication", description = "Authentication endpoints")
    ),
    info(
        title = "Terrier API",
        version = "1.0.0",
        description = "Terrier API",
        license(
            name = "MIT OR Apache-2.0",
            identifier = "MIT OR Apache-2.0"
        )
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

        // Add global security requirement
        openapi.security = Some(vec![SecurityRequirement::new("jwt", Vec::<String>::new())]);
    }
}
