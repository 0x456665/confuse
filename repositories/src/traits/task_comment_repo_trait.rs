use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use models::TaskComment;

#[async_trait]
pub trait TaskCommentRepositoryTrait: Send + Sync {
    /// Create a new task comment
    async fn create(
        &self,
        task_id: Uuid,
        user_id: Uuid,
        comment: String,
    ) -> Result<TaskComment, sqlx::Error>;
    
    /// Find a comment by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskComment>, sqlx::Error>;
    
    /// Find all comments for a specific task (excluding deleted)
    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<TaskComment>, sqlx::Error>;
    
    /// Find all comments by a specific user (excluding deleted)
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<TaskComment>, sqlx::Error>;
    
    /// Find all comments (including deleted)
    async fn find_all(&self) -> Result<Vec<TaskComment>, sqlx::Error>;
    
    /// Count comments for a task (excluding deleted)
    async fn count_by_task(&self, task_id: Uuid) -> Result<i64, sqlx::Error>;
    
    /// Update a comment
    async fn update(
        &self,
        id: Uuid,
        comment: String,
    ) -> Result<TaskComment, sqlx::Error>;
    
    /// Soft delete a comment
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    
    /// Permanently delete a comment
    async fn hard_delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    
    /// Restore a soft-deleted comment
    async fn restore(&self, id: Uuid) -> Result<(), sqlx::Error>;
}