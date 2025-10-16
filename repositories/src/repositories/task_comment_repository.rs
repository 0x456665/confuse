use async_trait::async_trait;
use chrono::{DateTime, Utc};
use models::TaskComment;
use sqlx::{PgPool, query, query_as, query_scalar};
use uuid::Uuid;

use crate::traits::TaskCommentRepositoryTrait;

pub struct TaskCommentRepository {
    pool: PgPool,
}

impl TaskCommentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskCommentRepositoryTrait for TaskCommentRepository {
    async fn create(
        &self,
        task_id: Uuid,
        user_id: Uuid,
        comment: String,
    ) -> Result<TaskComment, sqlx::Error> {
        let id = Uuid::new_v4();

        query_as!(
            TaskComment,
            r#"
            INSERT INTO task_comments (id, task_id, user_id, comment)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                task_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            "#,
            id,
            task_id,
            user_id,
            comment,
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskComment>, sqlx::Error> {
        query_as!(
            TaskComment,
            r#"
            SELECT
                id,
                task_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM task_comments
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<TaskComment>, sqlx::Error> {
        query_as!(
            TaskComment,
            r#"
            SELECT
                id,
                task_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM task_comments
            WHERE task_id = $1 AND deleted_at IS NULL
            ORDER BY created_at ASC
            "#,
            task_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<TaskComment>, sqlx::Error> {
        query_as!(
            TaskComment,
            r#"
            SELECT
                id,
                task_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM task_comments
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<TaskComment>, sqlx::Error> {
        query_as!(
            TaskComment,
            r#"
            SELECT
                id,
                task_id,
                user_id,
                comment,
                is_edited as "is_edited!: bool",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM task_comments
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn count_by_task(&self, task_id: Uuid) -> Result<i64, sqlx::Error> {
        query_scalar!(
            r#"
            SELECT COUNT(*)::BIGINT as "count!"
            FROM task_comments
            WHERE task_id = $1 AND deleted_at IS NULL
            "#,
            task_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, id: Uuid, comment: String) -> Result<TaskComment, sqlx::Error> {
        query_as!(
            TaskComment,
            r#"
            UPDATE task_comments
            SET
                comment = $2,
                is_edited = true,
                updated_at = $3
            WHERE id = $1
            RETURNING
                id,
                task_id,
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
            "UPDATE task_comments SET deleted_at = $1 WHERE id = $2",
            Some(Utc::now()),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn hard_delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!("DELETE FROM task_comments WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn restore(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE task_comments SET deleted_at = NULL WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
