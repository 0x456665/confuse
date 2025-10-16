use crate::traits::ProblemOrTaskRepositoryTrait;
use async_trait::async_trait;
use chrono::Utc;
use models::ProblemOrTask;
use serde_json::json;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

pub struct ProblemOrTaskRepository {
    pool: PgPool,
}

impl ProblemOrTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProblemOrTaskRepositoryTrait for ProblemOrTaskRepository {
    async fn create(
        &self,
        user_id: Uuid,
        title: String,
        content: String,
        file_url: Option<String>,
        tags: Vec<String>,
        difficulty: String,
    ) -> Result<ProblemOrTask, sqlx::Error> {
        let id = Uuid::new_v4();

        let record = query_as!(
            ProblemOrTask,
            r#"
            INSERT INTO problems_or_tasks (
                id, user_id, title, content, file_url, tags, difficulty
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING
                id, user_id, title, content, file_url,
                tags as "tags!: Vec<String>",
                difficulty,
                average_rating as "average_rating!: f64",
                total_ratings as "total_ratings!: i32",
                total_submissions as "total_submissions!: i32",
                view_count as "view_count!: i32",
                created_at as "created_at!: chrono::DateTime<chrono::Utc>",
                updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
                deleted_at as "deleted_at: chrono::DateTime<chrono::Utc>"
            "#,
            id,
            user_id,
            title,
            content,
            file_url,
            json!(tags),
            difficulty
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ProblemOrTask>, sqlx::Error> {
        query_as!(
            ProblemOrTask,
            r#"
            SELECT
                id, user_id, title, content, file_url,
                tags as "tags!: Vec<String>",
                difficulty,
                average_rating::FLOAT8 as "average_rating!: f64",
                total_ratings as "total_ratings!: i32",
                total_submissions as "total_submissions!: i32",
                view_count as "view_count!: i32",
                created_at as "created_at!: chrono::DateTime<chrono::Utc>",
                updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
                deleted_at as "deleted_at: chrono::DateTime<chrono::Utc>"
            FROM problems_or_tasks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<ProblemOrTask>, sqlx::Error> {
        query_as!(
            ProblemOrTask,
            r#"
            SELECT
                id, user_id, title, content, file_url,
                tags as "tags!: Vec<String>",
                difficulty,
                average_rating::FLOAT8 as "average_rating!: f64",
                total_ratings as "total_ratings!: i32",
                total_submissions as "total_submissions!: i32",
                view_count as "view_count!: i32",
                created_at as "created_at!: chrono::DateTime<chrono::Utc>",
                updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
                deleted_at as "deleted_at: chrono::DateTime<chrono::Utc>"
            FROM problems_or_tasks
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<ProblemOrTask>, sqlx::Error> {
        query_as!(
            ProblemOrTask,
            r#"
            SELECT
                id, user_id, title, content, file_url,
                tags as "tags!: Vec<String>",
                difficulty,
                average_rating::FLOAT8 as "average_rating!: f64",
                total_ratings as "total_ratings!: i32",
                total_submissions as "total_submissions!: i32",
                view_count as "view_count!: i32",
                created_at as "created_at!: chrono::DateTime<chrono::Utc>",
                updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
                deleted_at as "deleted_at: chrono::DateTime<chrono::Utc>"
            FROM problems_or_tasks
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        title: Option<String>,
        content: Option<String>,
        file_url: Option<String>,
        tags: Option<Vec<String>>,
        difficulty: Option<String>,
    ) -> Result<ProblemOrTask, sqlx::Error> {
        query_as!(
            ProblemOrTask,
            r#"
            UPDATE problems_or_tasks
            SET
                title = COALESCE($2, title),
                content = COALESCE($3, content),
                file_url = COALESCE($4, file_url),
                tags = COALESCE($5, tags),
                difficulty = COALESCE($6, difficulty),
                updated_at = $7
            WHERE id = $1
            RETURNING
                id, user_id, title, content, file_url,
                tags as "tags!: Vec<String>",
                difficulty,
                average_rating::FLOAT8 as "average_rating!: f64",
                total_ratings as "total_ratings!: i32",
                total_submissions as "total_submissions!: i32",
                view_count as "view_count!: i32",
                created_at as "created_at!: chrono::DateTime<chrono::Utc>",
                updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
                deleted_at as "deleted_at: chrono::DateTime<chrono::Utc>"
            "#,
            id,
            title,
            content,
            file_url,
            json!(tags),
            difficulty,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn increment_views(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE problems_or_tasks SET view_count = view_count + 1 WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE problems_or_tasks SET deleted_at = $1 WHERE id = $2",
            Some(Utc::now()),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
