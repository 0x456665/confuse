use axum::http::StatusCode;
use redis::RedisError;
use std::fmt;

/// Application error types
#[derive(Debug)]
pub enum AppError {
    // 400 Bad Request
    BadRequest(String),
    ValidationError(String),
    InvalidInput(String),

    // 401 Unauthorized
    Unauthorized(String),
    InvalidToken(String),
    TokenExpired,

    // 403 Forbidden
    Forbidden(String),

    // 404 Not Found
    NotFound(String),
    ResourceNotFound(String),

    // 409 Conflict
    Conflict(String),
    DuplicateEntry(String),
    AlreadyExists(String),

    // 422 Unprocessable Entity
    UnprocessableEntity(String),

    // 429 Too Many Requests
    TooManyRequests(String),
    RateLimitExceeded,

    // 500 Internal Server Error
    InternalServerError(String),
    DatabaseError(String),

    // 503 Service Unavailable
    ServiceUnavailable(String),
}

impl AppError {
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) | AppError::ValidationError(_) | AppError::InvalidInput(_) => {
                StatusCode::BAD_REQUEST
            }

            AppError::Unauthorized(_) | AppError::InvalidToken(_) | AppError::TokenExpired => {
                StatusCode::UNAUTHORIZED
            }

            AppError::Forbidden(_) => StatusCode::FORBIDDEN,

            AppError::NotFound(_) | AppError::ResourceNotFound(_) => StatusCode::NOT_FOUND,

            AppError::Conflict(_) | AppError::DuplicateEntry(_) | AppError::AlreadyExists(_) => {
                StatusCode::CONFLICT
            }

            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,

            AppError::TooManyRequests(_) | AppError::RateLimitExceeded => {
                StatusCode::TOO_MANY_REQUESTS
            }

            AppError::InternalServerError(_) | AppError::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }

            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// Get the error code (for API clients)
    pub fn error_code(&self) -> &str {
        match self {
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::InvalidInput(_) => "INVALID_INPUT",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::InvalidToken(_) => "INVALID_TOKEN",
            AppError::TokenExpired => "TOKEN_EXPIRED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::ResourceNotFound(_) => "RESOURCE_NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::DuplicateEntry(_) => "DUPLICATE_ENTRY",
            AppError::AlreadyExists(_) => "ALREADY_EXISTS",
            AppError::UnprocessableEntity(_) => "UNPROCESSABLE_ENTITY",
            AppError::TooManyRequests(_) => "TOO_MANY_REQUESTS",
            AppError::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            AppError::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
        }
    }

    /// Get the error message
    pub fn message(&self) -> String {
        match self {
            AppError::BadRequest(msg)
            | AppError::ValidationError(msg)
            | AppError::InvalidInput(msg)
            | AppError::Unauthorized(msg)
            | AppError::InvalidToken(msg)
            | AppError::Forbidden(msg)
            | AppError::NotFound(msg)
            | AppError::ResourceNotFound(msg)
            | AppError::Conflict(msg)
            | AppError::DuplicateEntry(msg)
            | AppError::AlreadyExists(msg)
            | AppError::UnprocessableEntity(msg)
            | AppError::TooManyRequests(msg)
            | AppError::DatabaseError(msg)
            | AppError::ServiceUnavailable(msg) => msg.clone(),

            AppError::TokenExpired => "Token has expired".to_string(),
            AppError::RateLimitExceeded => "Too many requests. Please try again later".to_string(),
            AppError::InternalServerError(msg) => {
                // Don't expose internal details in production
                #[cfg(debug_assertions)]
                return msg.clone();
            }
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_code(), self.message())
    }
}

impl std::error::Error for AppError {}

// ============================================
// CONVERSIONS FROM OTHER ERROR TYPES
// ============================================

/// Convert from sqlx errors
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                // Check for unique constraint violations
                if let Some(constraint) = db_err.constraint() {
                    AppError::DuplicateEntry(format!("Duplicate entry: {}", constraint))
                } else {
                    AppError::DatabaseError(db_err.to_string())
                }
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

/// Convert from validation errors
impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(err.to_string())
    }
}

/// Convert from serde JSON errors
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::BadRequest(format!("Invalid JSON: {}", err))
    }
}

/// Convert from JWT errors
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;

        match err.kind() {
            ErrorKind::ExpiredSignature => AppError::TokenExpired,
            ErrorKind::InvalidToken => AppError::InvalidToken("Invalid token".to_string()),
            _ => AppError::Unauthorized(err.to_string()),
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl From<RedisError> for AppError {
    fn from(err: RedisError) -> Self {
        AppError::ServiceUnavailable(format!("Failed to send redis command: {}", err))
    }
}
