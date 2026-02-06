use utoipa::OpenApi;

use super::dtos::create::CreateUserRequest;
use super::dtos::response::UserResponse;
use super::dtos::update::UpdateUserRequest;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::handlers::get_all_users,
        super::handlers::create_user,
        super::handlers::find_one_user,
        super::handlers::update_user,
        super::handlers::delete_user
    ),
    components(schemas(UserResponse, CreateUserRequest, UpdateUserRequest))
)]
pub struct UsersApi;
