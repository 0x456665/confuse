use crate::schema::request::{
    CreateUserRequest, ForgotPasswordRequest, LoginRequest, ResetPasswordRequest, VerifyEmailRequest
};
use crate::schema::response::UserResponse;
use crate::utils::constant::redis_key_map;
use crate::utils::email_templates::{activate_email_template, forgot_password_email_template};
use crate::utils::email_utils::send_email;
use crate::utils::otp::generate_otp;
use chrono::Utc;
use redis::AsyncCommands;
use shared::auth_utils::{
    TokenType, generate_token, hash_password, validate_token, verify_password,
};
use shared::errors::AppError;
use shared::state::AppState;
use uuid::Uuid;

// Constants
const FORGOT_PASSWORD_EXPIRY_MULTIPLIER: u64 = 2;
const SECONDS_PER_MINUTE: u64 = 60;
const DAYS_TO_SECONDS: u64 = 24 * 60 * 60;

#[derive(Debug, Clone)]
pub struct LoginResultDto {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct RefreshResultDto {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct RefreshTokenDto {
    pub refresh_token: String,
}

// ============================================================================
// AuthService - Business Logic Layer
// ============================================================================

pub struct AuthService {
    state: AppState,
}

impl AuthService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    /// Create a new user account
    ///
    /// Returns: Success message
    ///
    /// Side effects:
    /// - Creates user in database
    /// - Generates and stores OTP in Redis
    /// - Sends verification email
    pub async fn create_user(&self, dto: CreateUserRequest) -> Result<String, AppError> {
        // 1. Check if user already exists
        let existing_email = self.state.repos.user.get_user_by_email(&dto.email).await?;

        if existing_email.is_some() {
            return Err(AppError::AlreadyExists(
                "Email already registered".to_string(),
            ));
        }

        let existing_display_name = self
            .state
            .repos
            .user
            .get_user_by_display_name(&dto.display_name)
            .await?;

        if existing_display_name.is_some() {
            return Err(AppError::AlreadyExists(
                "Display name already taken".to_string(),
            ));
        }

        // 2. Hash password
        let password_hash = hash_password(&dto.password)?;

        // 3. Create user
        let user = self
            .state
            .repos
            .user
            .create_user(
                &dto.email,
                Some(&password_hash),
                &dto.display_name,
                None,
                dto.first_name.as_deref(),
                dto.last_name.as_deref(),
                None,
                None,
            )
            .await?;

        // 4. Generate OTP
        let otp = generate_otp(8);

        // 5. Store OTP in Redis
        let email_activation_key = self.get_redis_key("email_activation")?;
        let mut redis_conn = self.state.redis.clone();
        let _: () = redis_conn
            .set_ex(
                format!("{}:{}", email_activation_key, user.email),
                &otp,
                self.state.config.otp_expiry_minutes * SECONDS_PER_MINUTE,
            )
            .await?;

        // 6. Send verification email
        let activation_link = format!(
            "{}/activate?email={}&otp={}",
            self.state.config.frontend_url, user.email, otp
        );

        send_email(
            &self.state.config.from_email,
            &user.email,
            "Activate Your Account",
            activate_email_template(
                &user.display_name,
                &otp,
                self.state.config.otp_expiry_minutes,
                &activation_link,
            ),
            None,
            &self.state.config,
        )?;

        Ok(
            "User created successfully. Please check your email to verify your account."
                .to_string(),
        )
    }

    /// Verify user email with OTP
    ///
    /// Returns: Verified user data
    pub async fn verify_email(&self, dto: VerifyEmailRequest) -> Result<UserResponse, AppError> {
        // 1. Get user
        let user = self
            .state
            .repos
            .user
            .get_user_by_email(&dto.email)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        // 2. Check if already verified (idempotent operation)
        if user.email_verified_at.is_some() {
            return Ok(user.into());
        }

        // 3. Retrieve and validate OTP
        let email_activation_key = self.get_redis_key("email_activation")?;
        let mut redis_conn = self.state.redis.clone();

        let stored_otp: Option<String> = redis_conn
            .get_del(format!("{}:{}", email_activation_key, user.email))
            .await?;

        let stored_otp =
            stored_otp.ok_or(AppError::NotFound("OTP expired or not found".to_string()))?;

        // TODO: Use constant-time comparison
        if stored_otp != dto.otp {
            return Err(AppError::InvalidInput("Invalid OTP".to_string()));
        }

        // 4. Update user
        let updated_user = self
            .state
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

        Ok(updated_user.into())
    }

    /// Authenticate user and generate tokens
    ///
    /// Returns: User data and tokens
    ///
    /// Side effects:
    /// - Stores refresh token in Redis
    pub async fn login(&self, dto: LoginRequest) -> Result<LoginResultDto, AppError> {
        // 1. Get user
        let user = self
            .state
            .repos
            .user
            .get_user_by_email(&dto.email)
            .await?
            .ok_or(AppError::Unauthorized(
                "Invalid email or password".to_string(),
            ))?;

        // 2. Verify password exists (OAuth users might not have password)
        let password_hash = user.password_hash.as_ref().ok_or(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ))?;

        // 3. Verify password
        if !verify_password(&dto.password, password_hash)? {
            return Err(AppError::Unauthorized(
                "Invalid email or password".to_string(),
            ));
        }

        // 4. Check email verification
        if user.email_verified_at.is_none() {
            return Err(AppError::Unauthorized(
                "Please verify your email before logging in".to_string(),
            ));
        }

        // 5. Generate tokens
        let access_token = generate_token(
            &user.id,
            TokenType::Access,
            &self.state.config.access_secret,
            self.state.config.access_token_duration,
        )?;

        let refresh_token = generate_token(
            &user.id,
            TokenType::Refresh,
            &self.state.config.refresh_secret,
            self.state.config.refresh_token_duration,
        )?;

        // 6. Store refresh token
        self.store_refresh_token(&user.id, &refresh_token).await?;

        Ok(LoginResultDto {
            user: user.into(),
            access_token,
            refresh_token,
        })
    }

    /// Refresh access token using refresh token
    ///
    /// Returns: New token pair
    ///
    /// Side effects:
    /// - Rotates refresh token in Redis
    pub async fn refresh_token(
        &self,
        dto: RefreshTokenDto,
    ) -> Result<RefreshResultDto, AppError> {
        // 1. Validate refresh token
        let token_data = validate_token(&dto.refresh_token, &self.state.config.refresh_secret)?;

        let user_id = Uuid::parse_str(&token_data.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        // 2. Verify token in Redis (not revoked)
        let refresh_token_key = self.get_redis_key("refresh_token")?;
        let mut redis_conn = self.state.redis.clone();
        let stored_token: Option<String> = redis_conn
            .get(format!("{}:{}", refresh_token_key, user_id))
            .await?;

        match stored_token {
            None => {
                return Err(AppError::Unauthorized(
                    "Refresh token not found".to_string(),
                ));
            }
            Some(token) if token != dto.refresh_token => {
                return Err(AppError::Unauthorized("Invalid refresh token".to_string()));
            }
            _ => {}
        }

        // 3. Get user (ensure still exists and active)
        let user = self
            .state
            .repos
            .user
            .get_user_by_id(&user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        // 4. Generate new tokens (token rotation)
        let new_access_token = generate_token(
            &user.id,
            TokenType::Access,
            &self.state.config.access_secret,
            self.state.config.access_token_duration,
        )?;

        let new_refresh_token = generate_token(
            &user.id,
            TokenType::Refresh,
            &self.state.config.refresh_secret,
            self.state.config.refresh_token_duration,
        )?;

        // 5. Store new refresh token (invalidates old one)
        self.store_refresh_token(&user.id, &new_refresh_token)
            .await?;

        Ok(RefreshResultDto {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
        })
    }

    /// Initiate password reset flow
    ///
    /// Returns: Success message
    ///
    /// Side effects:
    /// - Generates and stores reset token in Redis
    /// - Sends password reset email
    pub async fn forgot_password(&self, dto: ForgotPasswordRequest) -> Result<String, AppError> {
        // 1. Get user
        let user = self
            .state
            .repos
            .user
            .get_user_by_email(&dto.email)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        // 2. Generate secure token
        let token = Uuid::new_v4().to_string();

        // 3. Store token in Redis
        let forgot_password_key = self.get_redis_key("forgot_password")?;
        let mut redis_conn = self.state.redis.clone();

        let expiry_seconds = self.state.config.otp_expiry_minutes
            * SECONDS_PER_MINUTE
            * FORGOT_PASSWORD_EXPIRY_MULTIPLIER;

        let _: () = redis_conn
            .set_ex(
                format!("{}:{}", forgot_password_key, user.email),
                &token,
                expiry_seconds,
            )
            .await?;

        // 4. Send email
        let reset_link = format!(
            "{}/reset-password?token={}",
            self.state.config.frontend_url, token
        );

        let email_html = forgot_password_email_template(
            &user.display_name,
            &reset_link,
            self.state.config.otp_expiry_minutes * FORGOT_PASSWORD_EXPIRY_MULTIPLIER,
        );

        send_email(
            &self.state.config.from_email,
            &user.email,
            "Reset Your Password",
            email_html,
            None,
            &self.state.config,
        )?;

        Ok("Password reset email sent successfully".to_string())
    }

    /// Reset password using token
    ///
    /// Returns: Success message
    ///
    /// Side effects:
    /// - Updates user password
    /// - Invalidates all refresh tokens (forces re-login)
    /// - Consumes reset token (one-time use)
    pub async fn reset_password(&self, dto: ResetPasswordRequest) -> Result<String, AppError> {
        // 1. Find which email this token belongs to
        let forgot_password_key = self.get_redis_key("forgot_password")?;
        let mut redis_conn = self.state.redis.clone();

        let pattern = format!("{}:{}", forgot_password_key, dto.email);
        let token: Option<String> = redis_conn.get_del(pattern).await?;

        if token.is_none() {
            return Err(AppError::NotFound(
                "Invalid or expired reset token".to_string(),
            ));
        }
        // 2. Get user
        let user = self
            .state
            .repos
            .user
            .get_user_by_email(&dto.email)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        // 3. Verify email is confirmed (security measure)
        if user.email_verified_at.is_none() {
            return Err(AppError::Unauthorized(
                "Please verify your email before resetting password".to_string(),
            ));
        }

        // 4. Hash new password
        let new_password_hash = hash_password(&dto.new_password)?;

        // 5. Update password
        self.state
            .repos
            .user
            .update_user(
                &user.id,
                None,
                Some(&new_password_hash),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await?;

        // 6. Invalidate all refresh tokens (security measure)
        self.revoke_all_tokens(&user.id).await?;

        Ok("Password reset successfully. Please login with your new password.".to_string())
    }

    /// Logout user
    ///
    /// Side effects:
    /// - Removes refresh token from Redis
    pub async fn logout(&self, user_id: &Uuid) -> Result<String, AppError> {
        self.revoke_all_tokens(user_id).await?;
        Ok("Logged out successfully".to_string())
    }

    /// Resend email verification OTP
    ///
    /// Returns: Success message
    ///
    /// Side effects:
    /// - Generates new OTP and stores in Redis
    /// - Sends verification email
    pub async fn resend_verification_otp(&self, email: &str) -> Result<String, AppError> {
        // 1. Get user
        let user = self
            .state
            .repos
            .user
            .get_user_by_email(email)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        // 2. Check if already verified
        if user.email_verified_at.is_some() {
            return Err(AppError::AlreadyExists(
                "Email already verified".to_string(),
            ));
        }

        // 3. Generate new OTP
        let otp = generate_otp(8);

        // 4. Store in Redis
        let email_activation_key = self.get_redis_key("email_activation")?;
        let mut redis_conn = self.state.redis.clone();
        let _: () = redis_conn
            .set_ex(
                format!("{}:{}", email_activation_key, user.email),
                &otp,
                self.state.config.otp_expiry_minutes * SECONDS_PER_MINUTE,
            )
            .await?;

        // 5. Send email
        let activation_link = format!(
            "{}/activate?email={}&otp={}",
            self.state.config.frontend_url, user.email, otp
        );

        send_email(
            &self.state.config.from_email,
            &user.email,
            "Activate Your Account",
            activate_email_template(
                &user.display_name,
                &otp,
                self.state.config.otp_expiry_minutes,
                &activation_link,
            ),
            None,
            &self.state.config,
        )?;

        Ok("Verification email resent successfully".to_string())
    }

    // ========================================================================
    // Private Helper Methods
    // ========================================================================

    fn get_redis_key(&self, key_name: &str) -> Result<&str, AppError> {
        redis_key_map().get(key_name).cloned().ok_or_else(|| {
            AppError::InternalServerError(format!("Redis key '{}' not configured", key_name))
        })
    }

    async fn store_refresh_token(&self, user_id: &Uuid, token: &str) -> Result<(), AppError> {
        let refresh_token_key = self.get_redis_key("refresh_token")?;
        let mut redis_conn = self.state.redis.clone();

        let expiry: u64 = self.state.config.refresh_token_duration * DAYS_TO_SECONDS;
        // .try_into()
        // .map_err(|_| AppError::InternalServerError("Invalid token duration".to_string()))?;

        let _: () = redis_conn
            .set_ex(format!("{}:{}", refresh_token_key, user_id), token, expiry)
            .await?;

        Ok(())
    }

    async fn revoke_all_tokens(&self, user_id: &Uuid) -> Result<(), AppError> {
        let refresh_token_key = self.get_redis_key("refresh_token")?;
        let mut redis_conn = self.state.redis.clone();

        let _: () = redis_conn
            .del(format!("{}:{}", refresh_token_key, user_id))
            .await?;

        Ok(())
    }
}
