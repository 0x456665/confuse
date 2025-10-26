// ============================================================================
// handlers/auth_handlers.rs - Thin HTTP Layer
//
// Responsibilities:
// - Extract HTTP-specific data (headers, cookies, path params)
// - Validate request payloads
// - Call service layer
// - Transform service results into HTTP responses
// - Handle HTTP-specific concerns (status codes, cookies)
// ============================================================================

use crate::schema::dto::RefreshTokenDto;
use crate::schema::request::{
    CreateUserRequest, ForgotPasswordRequest, LoginRequest, ResetPasswordRequest,
    VerifyEmailRequest,
};
use crate::schema::response::{
    LoginResponse, RefreshTokenResponse, ResponeOnlyMessage, UserResponse,
};
use crate::services::auth_service::AuthService;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use shared::{errors::AppError, state::AppState};
use time::Duration;
use validator::Validate;

// ============================================================================
// Helper Functions
// ============================================================================

/// Create HTTP-only refresh token cookie
fn create_refresh_cookie(
    token: String,
    duration_days: u64,
    is_production: bool,
) -> Cookie<'static> {
    Cookie::build(("refresh_token", token))
        .path("/")
        .max_age(Duration::days(duration_days as i64))
        .same_site(SameSite::Strict)
        .http_only(true)
        .secure(is_production)
        .build()
}

/// Create expired cookie for logout
fn create_expired_cookie() -> Cookie<'static> {
    Cookie::build(("refresh_token", ""))
        .path("/")
        .max_age(Duration::seconds(0))
        .same_site(SameSite::Strict)
        .http_only(true)
        .build()
}

// ============================================================================
// Handler Functions
// ============================================================================

/// POST /api/auth/register
///
/// Create a new user account and send verification email
pub async fn register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ResponeOnlyMessage>, AppError> {
    // 1. Validate request
    payload.validate()?;

    // 2. Call service
    let service = AuthService::new(app_state);
    let message = service.create_user(payload).await?;

    // 3. Return HTTP response
    Ok(Json(ResponeOnlyMessage {
        status: "Success".to_string(),
        message,
    }))
}

/// POST /api/auth/verify-email
///
/// Verify user email with OTP code
pub async fn verify_email_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<Json<UserResponse>, AppError> {
    // 1. Validate request
    payload.validate()?;

    // 3. Call service
    let service = AuthService::new(app_state);
    let user = service.verify_email(payload).await?;

    // 4. Return response
    Ok(Json(user.into()))
}

/// POST /api/auth/resend-verification
///
/// Resend email verification OTP
pub async fn resend_verification_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<ResponeOnlyMessage>, AppError> {
    payload.validate()?;

    // 1. Extract email
    let email = &payload.email;

    // 2. Call service
    let service = AuthService::new(app_state);
    let message = service.resend_verification_otp(email).await?;

    // 3. Return response
    Ok(Json(ResponeOnlyMessage {
        status: "Success".to_string(),
        message,
    }))
}

/// POST /api/auth/login
///
/// Authenticate user and return tokens
pub async fn login_handler(
    State(app_state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Validate request
    payload.validate()?;

    // 3. Call service
    let service = AuthService::new(app_state.clone());
    let result = service.login(payload).await?;

    // 4. Create HTTP-only cookie
    let refresh_cookie = create_refresh_cookie(
        result.refresh_token,
        app_state.config.refresh_token_duration,
        app_state.config.environment == "production",
    );

    // 5. Build response
    let response = LoginResponse {
        status: "Success".to_string(),
        message: "Login successful".to_string(),
        access_token: result.access_token,
        user: result.user.into(),
    };

    Ok((jar.add(refresh_cookie), Json(response)))
}

/// POST /api/auth/refresh
///
/// Refresh access token using refresh token from cookie
pub async fn refresh_token_handler(
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    // 1. Extract refresh token from cookie
    let refresh_token = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or(AppError::Unauthorized("No refresh token found".to_string()))?;

    // 2. Transform to DTO
    let dto = RefreshTokenDto { refresh_token };

    // 3. Call service
    let service = AuthService::new(app_state.clone());
    let result = service.refresh_token(dto).await?;

    // 4. Create new cookie with rotated token
    let refresh_cookie = create_refresh_cookie(
        result.refresh_token,
        app_state.config.refresh_token_duration,
        app_state.config.environment == "production",
    );

    // 5. Build response
    Ok((
        StatusCode::OK,
        jar.add(refresh_cookie),
        Json(RefreshTokenResponse {
            status: "Success".to_string(),
            access_token: result.access_token,
        }),
    ))
}

/// POST /api/auth/forgot-password
///
/// Initiate password reset flow
pub async fn forgot_password_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<ResponeOnlyMessage>, AppError> {
    // 1. Validate request
    payload.validate()?;

    // 3. Call service
    let service = AuthService::new(app_state);
    let message = service.forgot_password(payload).await?;

    // 4. Return response
    Ok(Json(ResponeOnlyMessage {
        status: "Success".to_string(),
        message,
    }))
}

/// POST /api/auth/reset-password
///
/// Reset password using token from email
pub async fn reset_password_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<ResponeOnlyMessage>, AppError> {
    // 1. Validate request
    payload.validate()?;

    // 3. Call service
    let service = AuthService::new(app_state);
    let message = service.reset_password(payload).await?;

    // 4. Return response
    Ok(Json(ResponeOnlyMessage {
        status: "Success".to_string(),
        message,
    }))
}

/// POST /api/auth/logout
///
/// Logout user and invalidate refresh token
pub async fn logout_handler(
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    // 1. Extract refresh token and decode to get user_id
    if let Some(cookie) = jar.get("refresh_token") {
        let refresh_token = cookie.value();

        // Try to validate token to get user_id
        if let Ok(token_data) =
            shared::auth_utils::validate_token(refresh_token, &app_state.config.refresh_secret)
        {
            if let Ok(user_id) = uuid::Uuid::parse_str(&token_data.sub) {
                // 2. Call service
                let service = AuthService::new(app_state.clone());
                let _ = service.logout(&user_id).await; // Ignore errors
            }
        }
    }

    // 3. Always clear the cookie (even if token validation fails)
    let expired_cookie = create_expired_cookie();

    // 4. Return response
    Ok((
        jar.add(expired_cookie),
        Json(ResponeOnlyMessage {
            status: "Success".to_string(),
            message: "Logged out successfully".to_string(),
        }),
    ))
}
