use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/users", api = crate::domain::users::api_doc::UsersApi)
    )
)]
pub struct ApiDoc;
