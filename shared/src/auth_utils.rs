use crate::errors::AppError;
use axum::{extract::Request, http::header::AUTHORIZATION};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode,
    errors::{self, ErrorKind},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum TokenType {
    Refresh,
    Access,
}

impl TokenType {
    pub fn as_string(&self) -> String {
        match self {
            TokenType::Refresh => "refresh".to_string(),
            TokenType::Access => "access".to_string(),
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    // Cost factor 12 - good balance of security vs performance
    hash(password, DEFAULT_COST + 2)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // user_id
    pub exp: usize,         // expiration
    pub token_type: String, //type of token "Refresh" or "Access"
    pub iat: usize,         // issued_at
}

pub fn generate_token(
    user_id: &Uuid,
    token_type: TokenType,
    secret: &str,
    duration: u64,
) -> Result<String, errors::Error> {
    let now = Utc::now();
    let exp = (now
        + Duration::hours(
            duration
                .try_into()
                .map_err(|_| errors::Error::from(ErrorKind::InvalidSignature))?,
        ))
    .timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        token_type: token_type.as_string(),
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?
    .claims;

    let now = Utc::now().timestamp() as usize;
    if claims.exp < now {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::ExpiredSignature,
        ));
    }

    Ok(claims)
}

pub fn extract_user_id(token: &str, secret: &str) -> Result<Uuid, AppError> {
    let claims = validate_token(token, secret)?;

    Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::InvalidToken("Invalid user ID in token".to_string()))
}

pub fn extract_bearer_token(req: &Request) -> Result<&str, AppError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        AppError::Unauthorized(
            "Invalid authorization format. Expected 'Bearer <token>'".to_string(),
        )
    })
}
