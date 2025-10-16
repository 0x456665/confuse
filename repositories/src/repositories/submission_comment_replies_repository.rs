use models::SubmissionCommentReply;
use sqlx::{PgPool, query, query_as, query_scalar};
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use uuid::Uuid;

use crate::traits::SubmissionCommentReplyRepositoryTrait;


pub struct SubmissionCommentReplyRepository {
    pool: PgPool,
}

impl SubmissionCommentReplyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubmissionCommentReplyRepositoryTrait for SubmissionCommentReplyRepository {
    async fn create(
        &self,
        submission_comment_id: Uuid,
        user_id: Uuid,
        reply: String,
    ) -> Result<SubmissionCommentReply, sqlx::Error> {
        let id = Uuid::new_v4();

        query_as!(
            SubmissionCommentReply,
            r#"
            INSERT INTO submission_comment_replies (id, submission_comment_id, user_id, reply)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                submission_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            "#,
            id,
            submission_comment_id,
            user_id,
            reply,
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SubmissionCommentReply>, sqlx::Error> {
        query_as!(
            SubmissionCommentReply,
            r#"
            SELECT
                id,
                submission_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submission_comment_replies
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_comment(&self, submission_comment_id: Uuid) -> Result<Vec<SubmissionCommentReply>, sqlx::Error> {
        query_as!(
            SubmissionCommentReply,
            r#"
            SELECT
                id,
                submission_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submission_comment_replies
            WHERE submission_comment_id = $1 AND deleted_at IS NULL
            ORDER BY created_at ASC
            "#,
            submission_comment_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<SubmissionCommentReply>, sqlx::Error> {
        query_as!(
            SubmissionCommentReply,
            r#"
            SELECT
                id,
                submission_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submission_comment_replies
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<SubmissionCommentReply>, sqlx::Error> {
        query_as!(
            SubmissionCommentReply,
            r#"
            SELECT
                id,
                submission_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submission_comment_replies
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn count_by_comment(&self, submission_comment_id: Uuid) -> Result<i64, sqlx::Error> {
        query_scalar!(
            r#"
            SELECT COUNT(*)::BIGINT as "count!"
            FROM submission_comment_replies
            WHERE submission_comment_id = $1 AND deleted_at IS NULL
            "#,
            submission_comment_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        reply: String,
    ) -> Result<SubmissionCommentReply, sqlx::Error> {
        query_as!(
            SubmissionCommentReply,
            r#"
            UPDATE submission_comment_replies
            SET
                reply = $2,
                is_edited = true,
                updated_at = $3
            WHERE id = $1
            RETURNING
                id,
                submission_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            "#,
            id,
            reply,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE submission_comment_replies SET deleted_at = $1 WHERE id = $2",
            Some(Utc::now()),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn hard_delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "DELETE FROM submission_comment_replies WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn restore(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE submission_comment_replies SET deleted_at = NULL WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}