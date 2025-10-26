use crate::schema::response::UserResponse;

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