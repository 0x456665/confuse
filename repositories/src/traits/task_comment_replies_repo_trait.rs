use async_trait::async_trait;
use uuid::Uuid;
use models::TaskCommentReply;

#[async_trait]
pub trait TaskCommentReplyRepositoryTrait: Send + Sync {
    /// Create a new reply to a task comment
    async fn create(
        &self,
        task_comment_id: Uuid,
        user_id: Uuid,
        reply: String,
    ) -> Result<TaskCommentReply, sqlx::Error>;
    
    /// Find a reply by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskCommentReply>, sqlx::Error>;
    
    /// Find all replies for a specific task comment (excluding deleted)
    async fn find_by_comment(&self, task_comment_id: Uuid) -> Result<Vec<TaskCommentReply>, sqlx::Error>;
    
    /// Find all replies by a specific user (excluding deleted)
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<TaskCommentReply>, sqlx::Error>;
    
    /// Find all replies (including deleted)
    async fn find_all(&self) -> Result<Vec<TaskCommentReply>, sqlx::Error>;
    
    /// Count replies for a comment (excluding deleted)
    async fn count_by_comment(&self, task_comment_id: Uuid) -> Result<i64, sqlx::Error>;
    
    /// Update a reply
    async fn update(
        &self,
        id: Uuid,
        reply: String,
    ) -> Result<TaskCommentReply, sqlx::Error>;
    
    /// Soft delete a reply
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    
    /// Permanently delete a reply
    async fn hard_delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    
    /// Restore a soft-deleted reply
    async fn restore(&self, id: Uuid) -> Result<(), sqlx::Error>;
}