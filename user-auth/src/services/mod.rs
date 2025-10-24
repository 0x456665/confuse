use crate::schema::request::VerifyEmailRequest;
use crate::schema::response::{ResponeOnlyMessage, UserResponse};
use crate::utils::constants;
use crate::utils::email_templates::activate_email_template;
use crate::utils::otp::generate_otp;
use crate::{schema::request::CreateUserRequest, utils::email_utils::send_email};
use axum::Json;
use chrono::Utc;
use redis::AsyncCommands;
use shared::{auth_utils::hash_password, errors::AppError, state::AppState};
use validator::Validate;

pub async fn create_user(
    app_state: AppState,
    payload: CreateUserRequest,
) -> Result<Json<ResponeOnlyMessage>, AppError> {
    payload.validate()?;

    let check_user_exists_with_email = app_state
        .repos
        .user
        .get_user_by_email(payload.email.clone())
        .await?;

    let check_user_exists_with_display_name = app_state
        .repos
        .user
        .get_user_by_display_name(payload.display_name.clone())
        .await?;

    if check_user_exists_with_display_name.is_some() {
        return Err(AppError::AlreadyExists("User Already exists".to_string()));
    };

    if check_user_exists_with_email.is_some() {
        return Err(AppError::AlreadyExists("User Already exists".to_string()));
    };

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
            format!(
                "{}:{}",
                constants::get_redis_keys()
                    .get("email_activation")
                    .expect("redis key not properly configures"),
                user.email
            ),
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

    let message = ResponeOnlyMessage {
        status: "Successful".to_string(),
        message: "User created successfully, verify email address to activate your account"
            .to_string(),
    };
    // shoudl be changed to message or something
    Ok(Json(message))
}

pub async fn veryify_otp(
    app_state: AppState,
    payload: VerifyEmailRequest,
) -> Result<Json<UserResponse>, AppError> {
    let user = app_state
        .repos
        .user
        .get_user_by_email(payload.email.clone())
        .await?;

    if user.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    let user = user.unwrap();

    let mut redis_conn = app_state.redis.clone();
    let otp: Option<String> = redis_conn
        .get_del(format!(
            "{}:{}",
            constants::get_redis_keys()
                .get("email_activation")
                .expect("redis key not properly configures"),
            user.email
        ))
        .await?;

    if otp.is_none() {
        return Err(AppError::NotFound("OTP not found".to_string()));
    }

    if otp.unwrap() != payload.otp {
        return Err(AppError::InvalidInput("Invalid OTP".to_string()));
    }

    // update user
    app_state
        .repos
        .user
        .update_user(
            user.id,
            Some(payload.email.to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Utc::now()),
        )
        .await?;

    Ok(Json(user.into()))
}

pub fn forgot_password(app_state: AppState) -> Result<(), AppError> {
    todo!()
}

pub fn login() -> Result<(), AppError> {
    todo!()
}

pub fn refresh_access_token() -> Result<(), AppError> {
    todo!()
}
