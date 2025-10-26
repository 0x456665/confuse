use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, Serialize)]
pub struct CreateUserRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "username must be between 3 and 50 characters"
    ))]
    pub display_name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(max = 50, message = "firstname can not be more than 50 characters"))]
    pub first_name: Option<String>,

    #[validate(length(max = 50, message = "firstname can not be more than 50 characters"))]
    pub last_name: Option<String>,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct UpdateUserRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "username must be between 3 and 50 characters"
    ))]
    pub display_name: Option<String>,
    #[validate(length(max = 500, message = "Bio can not exceed 500 characters"))]
    pub bio: Option<String>,

    #[validate(length(max = 50, message = "firstname can not be more than 50 characters"))]
    pub first_name: Option<String>,

    #[validate(length(max = 50, message = "firstname can not be more than 50 characters"))]
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct VerifyEmailRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub otp: String,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct ResetPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,

    pub token: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "password must be atleast 8 characters"))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RefreshToken {
    pub refresh_token: String,
}
