use crate::schema::request::CreateUserRequest;
use crate::schema::response::UserResponse;
use axum::{Json, extract::State};
use shared::{auth_utils::hash_password, errors::AppError, state::AppState};
use validator::Validate;

pub async fn create_user(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate()?;
    let check_user_exists = app_state
        .repos
        .user
        .get_user_by_email(payload.email.clone().to_string())
        .await?;
    if check_user_exists.is_some() {
        return Err(AppError::AlreadyExists("User Already exists".to_string()));
    }
    let password_hash = hash_password(&payload.password)?;
    let user = app_state
        .repos
        .user
        .create_user(
            payload.email.to_string(),
            Some(password_hash),
            payload.display_name.to_string(),
            None,
            payload.first_name,
            payload.last_name,
            None,
            None,
        )
        .await?;

    Ok(Json(user.into()))
}