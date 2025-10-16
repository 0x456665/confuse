use async_trait::async_trait;
use chrono::{DateTime, Utc};
use models::SubmissionComment;
use sqlx::{PgPool, query, query_as, query_scalar};
use uuid::Uuid;

use crate::traits::SubmissionCommentRepositoryTrait;

pub struct SubmissionCommentRepository {
    pool: PgPool,
}

impl SubmissionCommentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubmissionCommentRepositoryTrait for SubmissionCommentRepository {
    async fn create(
        &self,
        submission_id: Uuid,
        user_id: Uuid,
        comment: String,
    ) -> Result<SubmissionComment, sqlx::Error> {
        let id = Uuid::new_v4();

        query_as!(
            SubmissionComment,
            r#"
            INSERT INTO submission_comments (id, submission_id, user_id, comment)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                submission_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            "#,
            id,
            submission_id,
            user_id,
            comment,
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SubmissionComment>, sqlx::Error> {
        query_as!(
            SubmissionComment,
            r#"
            SELECT
                id,
                submission_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submission_comments
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_submission(&self, submission_id: Uuid) -> Result<Vec<SubmissionComment>, sqlx::Error> {
        query_as!(
            SubmissionComment,
            r#"
            SELECT
                id,
                submission_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submission_comments
            WHERE submission_id = $1 AND deleted_at IS NULL
            ORDER BY created_at ASC
            "#,
            submission_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<SubmissionComment>, sqlx::Error> {
        query_as!(
            SubmissionComment,
            r#"
            SELECT
                id,
                submission_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submission_comments
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<SubmissionComment>, sqlx::Error> {
        query_as!(
            SubmissionComment,
            r#"
            SELECT
                id,
                submission_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submission_comments
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn count_by_submission(&self, submission_id: Uuid) -> Result<i64, sqlx::Error> {
        query_scalar!(
            r#"
            SELECT COUNT(*)::BIGINT as "count!"
            FROM submission_comments
            WHERE submission_id = $1 AND deleted_at IS NULL
            "#,
            submission_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        comment: String,
    ) -> Result<SubmissionComment, sqlx::Error> {
        query_as!(
            SubmissionComment,
            r#"
            UPDATE submission_comments
            SET
                comment = $2,
                is_edited = true,
                updated_at = $3
            WHERE id = $1
            RETURNING
                id,
                submission_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            "#,
            id,
            comment,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE submission_comments SET deleted_at = $1 WHERE id = $2",
            Some(Utc::now()),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn hard_delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "DELETE FROM submission_comments WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn restore(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE submission_comments SET deleted_at = NULL WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}