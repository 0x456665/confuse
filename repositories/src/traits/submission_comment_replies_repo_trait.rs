use async_trait::async_trait;
use models::SubmissionCommentReply;
use uuid::Uuid;

#[async_trait]
pub trait SubmissionCommentReplyRepositoryTrait: Send + Sync {
    /// Create a new reply to a submission comment
    async fn create(
        &self,
        submission_comment_id: Uuid,
        user_id: Uuid,
        reply: String,
    ) -> Result<SubmissionCommentReply, sqlx::Error>;

    /// Find a reply by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SubmissionCommentReply>, sqlx::Error>;

    /// Find all replies for a specific submission comment (excluding deleted)
    async fn find_by_comment(
        &self,
        submission_comment_id: Uuid,
    ) -> Result<Vec<SubmissionCommentReply>, sqlx::Error>;

    /// Find all replies by a specific user (excluding deleted)
    async fn find_by_user(&self, user_id: Uuid)
    -> Result<Vec<SubmissionCommentReply>, sqlx::Error>;

    /// Find all replies (including deleted)
    async fn find_all(&self) -> Result<Vec<SubmissionCommentReply>, sqlx::Error>;

    /// Count replies for a comment (excluding deleted)
    async fn count_by_comment(&self, submission_comment_id: Uuid) -> Result<i64, sqlx::Error>;

    /// Update a reply
    async fn update(&self, id: Uuid, reply: String) -> Result<SubmissionCommentReply, sqlx::Error>;

    /// Soft delete a reply
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;

    /// Permanently delete a reply
    async fn hard_delete(&self, id: Uuid) -> Result<(), sqlx::Error>;

    /// Restore a soft-deleted reply
    async fn restore(&self, id: Uuid) -> Result<(), sqlx::Error>;
}
