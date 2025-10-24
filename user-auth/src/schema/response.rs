use chrono::{DateTime, Utc};
use models::User;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub reputation_score: Decimal,
    pub total_ratings_given: i32,
    pub total_ratings_received: i32,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            bio: user.bio,
            first_name: user.first_name,
            last_name: user.last_name,
            avatar_url: user.avatar_url,
            reputation_score: user.reputation_score,
            total_ratings_given: user.total_ratings_given,
            total_ratings_received: user.total_ratings_received,
            email_verified_at: user.email_verified_at,
            created_at: user.created_at,
            updated_at: user.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UserResponseWithToken {
    pub user: UserResponse,
    pub token: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ResponeOnlyMessage {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserResponseWithMessage {
    pub user: UserResponse,
    pub message: String,
}