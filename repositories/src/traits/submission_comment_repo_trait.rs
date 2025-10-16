use async_trait::async_trait;
use uuid::Uuid;
use models::SubmissionComment;

#[async_trait]
pub trait SubmissionCommentRepositoryTrait: Send + Sync {
    /// Create a new submission comment
    async fn create(
        &self,
        submission_id: Uuid,
        user_id: Uuid,
        comment: String,
    ) -> Result<SubmissionComment, sqlx::Error>;
    
    /// Find a comment by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SubmissionComment>, sqlx::Error>;
    
    /// Find all comments for a specific submission (excluding deleted)
    async fn find_by_submission(&self, submission_id: Uuid) -> Result<Vec<SubmissionComment>, sqlx::Error>;
    
    /// Find all comments by a specific user (excluding deleted)
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<SubmissionComment>, sqlx::Error>;
    
    /// Find all comments (including deleted)
    async fn find_all(&self) -> Result<Vec<SubmissionComment>, sqlx::Error>;
    
    /// Count comments for a submission (excluding deleted)
    async fn count_by_submission(&self, submission_id: Uuid) -> Result<i64, sqlx::Error>;
    
    /// Update a comment
    async fn update(
        &self,
        id: Uuid,
        comment: String,
    ) -> Result<SubmissionComment, sqlx::Error>;
    
    /// Soft delete a comment
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    
    /// Permanently delete a comment
    async fn hard_delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    
    /// Restore a soft-deleted comment
    async fn restore(&self, id: Uuid) -> Result<(), sqlx::Error>;
}