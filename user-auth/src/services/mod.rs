use crate::schema::response::UserResponse;
use crate::utils::email_templates::activate_email_template;
use crate::utils::otp::generate_otp;
use crate::{schema::request::CreateUserRequest, utils::email_utils::send_email};
use axum::Json;
use redis::AsyncCommands;
use shared::{auth_utils::hash_password, errors::AppError, state::AppState};
use validator::Validate;

pub async fn create_user(
    app_state: AppState,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate()?;

    let check_user_exists = app_state
        .repos
        .user
        .get_user_by_email(payload.email.clone())
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

    // generate otp
    let otp = generate_otp(8);

    let mut redis_conn = app_state.redis.clone();
    let _: () = redis_conn
        .set_ex(
            format!("otp_activation:{}", user.email),
            &otp,
            app_state.config.otp_expiry_minutes * 60,
        )
        .await?;

    // send email
    send_email(
        &app_state.config.from_email,
        &user.email,
        "EMAIL ACTIVATION",
        // get email template and populate with value
        //
        activate_email_template(
            &user.display_name[..],
            &otp[..],
            app_state.config.otp_expiry_minutes,
            "",
            "",
        ),
        None,
        &app_state.config,
    )?;
    
    // shoudl be changed to message or something
    Ok(Json(user.into()))
}
