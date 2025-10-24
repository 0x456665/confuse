use crate::schema::request::{ForgotPasswordRequest, VerifyEmailRequest};
use crate::schema::response::{ResponeOnlyMessage, UserResponse};
use crate::utils::constants;
use crate::utils::email_templates::{activate_email_template, forgot_password_email_template};
use crate::utils::otp::generate_otp;
use crate::{schema::request::CreateUserRequest, utils::email_utils::send_email};
use axum::Json;
use chrono::Utc;
use redis::AsyncCommands;
use shared::{auth_utils::hash_password, errors::AppError, state::AppState};
use uuid::Uuid;
use validator::Validate;

/// check if user with email or display_name exists
/// hashes password
/// generates otp
/// adds otp to redis
/// sends email
/// returns message
pub async fn create_user(
    app_state: AppState,
    payload: CreateUserRequest,
) -> Result<Json<ResponeOnlyMessage>, AppError> {
    payload.validate()?;

    let check_user_exists_with_email = app_state
        .repos
        .user
        .get_user_by_email(&payload.email)
        .await?;

    let check_user_exists_with_display_name = app_state
        .repos
        .user
        .get_user_by_display_name(&payload.display_name)
        .await?;

    if check_user_exists_with_display_name.is_some() {
        return Err(AppError::AlreadyExists(
            "Display name already taken".to_string(),
        ));
    }

    if check_user_exists_with_email.is_some() {
        return Err(AppError::AlreadyExists(
            "Email already registered".to_string(),
        ));
    }

    let password_hash = hash_password(&payload.password)?;

    let user = app_state
        .repos
        .user
        .create_user(
            payload.email.as_str(),
            Some(&password_hash),
            payload.display_name.as_str(),
            None,
            payload.first_name.as_deref(),
            payload.last_name.as_deref(),
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
                    .expect("redis key not properly configured"),
                user.email
            ),
            &otp,
            app_state.config.otp_expiry_minutes * 60,
        )
        .await?;

    // Generate activation link
    let activation_link = format!(
        "{}/activate?email={}&otp={}",
        app_state.config.frontend_url, user.email, otp
    );

    // send email
    send_email(
        &app_state.config.from_email,
        &user.email,
        "Activate Your Account",
        activate_email_template(
            &user.display_name,
            &otp,
            app_state.config.otp_expiry_minutes,
            &activation_link,
        ),
        None,
        &app_state.config,
    )?;

    let message = ResponeOnlyMessage {
        status: "Successful".to_string(),
        message: "User created successfully, verify email address to activate your account"
            .to_string(),
    };

    Ok(Json(message))
}

pub async fn verify_otp(
    app_state: AppState,
    payload: VerifyEmailRequest,
) -> Result<Json<UserResponse>, AppError> {
    let user = app_state
        .repos
        .user
        .get_user_by_email(&payload.email)
        .await?;

    if user.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    let user = user.unwrap();

    if user.email_verified_at.is_some() {
        return Err(AppError::AlreadyExists(
            "Email already verified".to_string(),
        ));
    }

    let mut redis_conn = app_state.redis.clone();
    let otp: Option<String> = redis_conn
        .get_del(format!(
            "{}:{}",
            constants::get_redis_keys()
                .get("email_activation")
                .expect("redis key not properly configured"),
            user.email
        ))
        .await?;

    if otp.is_none() {
        return Err(AppError::NotFound("OTP expired or not found".to_string()));
    }

    if otp.unwrap() != payload.otp {
        return Err(AppError::InvalidInput("Invalid OTP".to_string()));
    }

    // update user - set email_verified_at
    let updated_user = app_state
        .repos
        .user
        .update_user(
            &user.id,
            None,
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

    Ok(Json(updated_user.into()))
}

pub async fn forgot_password(
    app_state: AppState,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<ResponeOnlyMessage>, AppError> {
    let user_with_email = app_state
        .repos
        .user
        .get_user_by_email(&payload.email)
        .await?;

    if user_with_email.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    let user = user_with_email.unwrap();
    let token = Uuid::new_v4().to_string();

    let mut redis_conn = app_state.redis.clone();
    let _: () = redis_conn
        .set_ex(
            format!(
                "{}:{}",
                constants::get_redis_keys()
                    .get("forgot_password")
                    .expect("redis keys not configured properly"),
                user.email
            ),
            &token,
            app_state.config.otp_expiry_minutes * 60 * 2,
        )
        .await?;

    // Generate reset link
    let reset_link = format!(
        "{}/reset-password?token={}",
        app_state.config.frontend_url, token
    );

    // Generate email
    let email_html = forgot_password_email_template(
        &user.display_name,
        &reset_link,
        app_state.config.otp_expiry_minutes * 2, // doubled expiry
    );

    // Send email
    send_email(
        &app_state.config.from_email,
        &user.email,
        "Reset Your Password",
        email_html,
        None,
        &app_state.config,
    )?;

    let message = ResponeOnlyMessage {
        status: "Successful".to_string(),
        message: "Password reset email sent successfully".to_string(),
    };

    Ok(Json(message))
}

pub fn login() -> Result<(), AppError> {
    Ok(())
}

pub fn refresh_access_token() -> Result<(), AppError> {
    todo!()
}
