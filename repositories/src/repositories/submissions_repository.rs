use crate::traits::SubmissionRepositoryTrait;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use models::Submission;
use rust_decimal::Decimal;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

pub struct SubmissionRepository {
    pool: PgPool,
}

impl SubmissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubmissionRepositoryTrait for SubmissionRepository {
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
    ) -> Result<Submission, sqlx::Error> {
        let id = Uuid::new_v4();

        let submission = query_as!(
            Submission,
            r#"
            INSERT INTO submissions (
                id, user_id, task_id, content, file_url, status,
                average_rating, total_ratings, is_featured, submitted_at
            )
            VALUES (
                $1, $2, $3, $4, $5, COALESCE($6, 'draft'),
                $7, $8, $9, $10
            )
            RETURNING
                id, user_id, task_id, content, file_url, status,
                average_rating as "average_rating!: Decimal",
                total_ratings as "total_ratings!: i32",
                is_featured as "is_featured!: bool",
                submitted_at as "submitted_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            "#,
            id,
            user_id,
            task_id,
            content,
            file_url,
            status,
            average_rating,
            total_ratings,
            is_featured,
            submitted_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(submission)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Submission>, sqlx::Error> {
        query_as!(
            Submission,
            r#"
            SELECT
                id, user_id, task_id, content, file_url, status,
                average_rating as "average_rating!: Decimal",
                total_ratings as "total_ratings!: i32",
                is_featured as "is_featured!: bool",
                submitted_at as "submitted_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submissions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Submission>, sqlx::Error> {
        query_as!(
            Submission,
            r#"
            SELECT
                id, user_id, task_id, content, file_url, status,
                average_rating as "average_rating!: Decimal",
                total_ratings as "total_ratings!: i32",
                is_featured as "is_featured!: bool",
                submitted_at as "submitted_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submissions
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<Submission>, sqlx::Error> {
        query_as!(
            Submission,
            r#"
            SELECT
                id, user_id, task_id, content, file_url, status,
                average_rating as "average_rating!: Decimal",
                total_ratings as "total_ratings!: i32",
                is_featured as "is_featured!: bool",
                submitted_at as "submitted_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submissions
            WHERE task_id = $1
            ORDER BY created_at DESC
            "#,
            task_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<Submission>, sqlx::Error> {
        query_as!(
            Submission,
            r#"
            SELECT
                id, user_id, task_id, content, file_url, status,
                average_rating as "average_rating!: Decimal",
                total_ratings as "total_ratings!: i32",
                is_featured as "is_featured!: bool",
                submitted_at as "submitted_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submissions
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_status(&self, status: &str) -> Result<Vec<Submission>, sqlx::Error> {
        query_as!(
            Submission,
            r#"
            SELECT
                id, user_id, task_id, content, file_url, status,
                average_rating as "average_rating!: Decimal",
                total_ratings as "total_ratings!: i32",
                is_featured as "is_featured!: bool",
                submitted_at as "submitted_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submissions
            WHERE status = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
            status
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_featured(&self) -> Result<Vec<Submission>, sqlx::Error> {
        query_as!(
            Submission,
            r#"
            SELECT
                id, user_id, task_id, content, file_url, status,
                average_rating as "average_rating!: Decimal",
                total_ratings as "total_ratings!: i32",
                is_featured as "is_featured!: bool",
                submitted_at as "submitted_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            FROM submissions
            WHERE is_featured = true AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn update_status(&self, id: Uuid, status: &str) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE submissions SET status = $1, updated_at = $2 WHERE id = $3",
            status,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_submission(
        &self,
        id: Uuid,
        content: Option<String>,
        file_url: Option<String>,
        status: Option<String>,
        is_featured: Option<bool>,
        average_rating: Option<Decimal>,
        total_ratings: Option<i32>,
    ) -> Result<Submission, sqlx::Error> {
        query_as!(
            Submission,
            r#"
            UPDATE submissions
            SET
                content = COALESCE($2, content),
                file_url = COALESCE($3, file_url),
                status = COALESCE($4, status),
                is_featured = COALESCE($5, is_featured),
                average_rating = COALESCE($6, average_rating),
                total_ratings = COALESCE($7, total_ratings),
                updated_at = $8
            WHERE id = $1
            RETURNING
                id, user_id, task_id, content, file_url, status,
                average_rating as "average_rating!: Decimal",
                total_ratings as "total_ratings!: i32",
                is_featured as "is_featured!: bool",
                submitted_at as "submitted_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                deleted_at as "deleted_at: DateTime<Utc>"
            "#,
            id,
            content,
            file_url,
            status,
            is_featured,
            average_rating,
            total_ratings,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE submissions SET deleted_at = $1 WHERE id = $2",
            Some(Utc::now()),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}