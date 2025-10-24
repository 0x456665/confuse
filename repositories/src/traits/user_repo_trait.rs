use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use models::User;
use uuid::Uuid;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create_user(
        &self,
        email: String,
        password_hash: Option<String>,
        display_name: String,
        bio: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        avatar_url: Option<String>,
        email_verified_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error>;

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn get_user_by_email(&self, email: String) -> Result<Option<User>, sqlx::Error>;

    async fn get_user_by_display_name(&self, display_name: String) -> Result<Option<User>, sqlx::Error>;

    async fn update_user(
        &self,
        user_id: Uuid,
        email: Option<String>,
        password_hash: Option<String>,
        display_name: Option<String>,
        bio: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        avatar_url: Option<String>,
        reputation_score: Option<Decimal>,
        total_ratings_given: Option<i32>,
        total_ratings_received: Option<i32>,
        email_verified_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error>;

    async fn delete_user(&self, user_id: Uuid) -> Result<User, sqlx::Error>;
}
