use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use models::Submission;

#[async_trait]
pub trait SubmissionRepositoryTrait: Send + Sync {
    async fn create(
        &self,
        user_id: Uuid,
        task_id: Uuid,
        content: String,
        file_url: Option<String>,
        status: Option<String>,
        average_rating: Decimal,
        total_ratings: i32,
        is_featured: bool,
        submitted_at: Option<DateTime<Utc>>,
    ) -> Result<Submission, sqlx::Error>;
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Submission>, sqlx::Error>;
    
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Submission>, sqlx::Error>;
    
    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<Submission>, sqlx::Error>;
    
    async fn find_all(&self) -> Result<Vec<Submission>, sqlx::Error>;
    
    async fn find_by_status(&self, status: &str) -> Result<Vec<Submission>, sqlx::Error>;
    
    async fn find_featured(&self) -> Result<Vec<Submission>, sqlx::Error>;
    
    async fn update_status(&self, id: Uuid, status: &str) -> Result<(), sqlx::Error>;
    
    async fn update_submission(
        &self,
        id: Uuid,
        content: Option<String>,
        file_url: Option<String>,
        status: Option<String>,
        is_featured: Option<bool>,
        average_rating: Option<Decimal>,
        total_ratings: Option<i32>,
    ) -> Result<Submission, sqlx::Error>;
    
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}