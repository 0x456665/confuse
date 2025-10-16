use async_trait::async_trait;
use models::TaskCommentReply;
use sqlx::{PgPool, query, query_as, query_scalar};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::traits::TaskCommentReplyRepositoryTrait;

pub struct TaskCommentReplyRepository {
    pool: PgPool,
}

impl TaskCommentReplyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskCommentReplyRepositoryTrait for TaskCommentReplyRepository {
    async fn create(
        &self,
        task_comment_id: Uuid,
        user_id: Uuid,
        reply: String,
    ) -> Result<TaskCommentReply, sqlx::Error> {
        let id = Uuid::new_v4();

        query_as!(
            TaskCommentReply,
            r#"
            INSERT INTO task_comment_replies (id, task_comment_id, user_id, reply)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                task_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            "#,
            id,
            task_comment_id,
            user_id,
            reply,
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskCommentReply>, sqlx::Error> {
        query_as!(
            TaskCommentReply,
            r#"
            SELECT
                id,
                task_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM task_comment_replies
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_comment(&self, task_comment_id: Uuid) -> Result<Vec<TaskCommentReply>, sqlx::Error> {
        query_as!(
            TaskCommentReply,
            r#"
            SELECT
                id,
                task_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM task_comment_replies
            WHERE task_comment_id = $1 AND deleted_at IS NULL
            ORDER BY created_at ASC
            "#,
            task_comment_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<TaskCommentReply>, sqlx::Error> {
        query_as!(
            TaskCommentReply,
            r#"
            SELECT
                id,
                task_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM task_comment_replies
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<TaskCommentReply>, sqlx::Error> {
        query_as!(
            TaskCommentReply,
            r#"
            SELECT
                id,
                task_comment_id,
                user_id,
                reply,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM task_comment_replies
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn count_by_comment(&self, task_comment_id: Uuid) -> Result<i64, sqlx::Error> {
        query_scalar!(
            r#"
            SELECT COUNT(*)::BIGINT as "count!"
            FROM task_comment_replies
            WHERE task_comment_id = $1 AND deleted_at IS NULL
            "#,
            task_comment_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        reply: String,
    ) -> Result<TaskCommentReply, sqlx::Error> {
        query_as!(
            TaskCommentReply,
            r#"
            UPDATE task_comment_replies
            SET
                reply = $2,
                is_edited = true,
                updated_at = $3
            WHERE id = $1
            RETURNING
                id,
                task_comment_id,
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
            "UPDATE task_comment_replies SET deleted_at = $1 WHERE id = $2",
            Some(Utc::now()),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn hard_delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "DELETE FROM task_comment_replies WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn restore(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE task_comment_replies SET deleted_at = NULL WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}