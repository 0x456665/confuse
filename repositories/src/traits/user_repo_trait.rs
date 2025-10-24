use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use models::User;
use uuid::Uuid;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create_user(
        &self,
        email: &str,
        password_hash: Option<&str>,
        display_name: &str,
        bio: Option<&str>,
        first_name: Option<&str>,
        last_name: Option<&str>,
        avatar_url: Option<&str>,
        email_verified_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error>;
    
    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, sqlx::Error>;
    
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    
    async fn get_user_by_display_name(&self, display_name: &str) -> Result<Option<User>, sqlx::Error>;
    
    async fn update_user(
        &self,
        user_id: &Uuid,
        email: Option<&str>,
        password_hash: Option<&str>,
        display_name: Option<&str>,
        bio: Option<&str>,
        first_name: Option<&str>,
        last_name: Option<&str>,
        avatar_url: Option<&str>,
        reputation_score: Option<Decimal>,
        total_ratings_given: Option<i32>,
        total_ratings_received: Option<i32>,
        email_verified_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error>;
    
    async fn delete_user(&self, user_id: &Uuid) -> Result<User, sqlx::Error>;
}