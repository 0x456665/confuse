use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use uuid::Uuid;
use crate::errors::AppError;


/// Extractor for authenticated user ID
pub struct CurrentUser(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;
    
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Uuid>()
            .copied()
            .map(CurrentUser)
            .ok_or_else(|| AppError::Unauthorized("User not authenticated".to_string()))
    }
}