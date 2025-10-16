use async_trait::async_trait;
use sqlx::{PgPool, query, query_as, query_scalar};
use uuid::Uuid;
use models::TaskRating;
use chrono::{DateTime, Utc};
use crate::traits::TaskRatingRepositoryTrait;

pub struct TaskRatingRepository {
    pool: PgPool,
}

impl TaskRatingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRatingRepositoryTrait for TaskRatingRepository {
    async fn create(
        &self,
        task_id: Uuid,
        rater_id: Uuid,
        rating_value: i32,
    ) -> Result<TaskRating, sqlx::Error> {
        let id = Uuid::new_v4();

        let task_rating = query_as!(
            TaskRating,
            r#"
            INSERT INTO task_ratings (id, task_id, rater_id, rating_value)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (task_id, rater_id) 
            DO UPDATE SET 
                rating_value = EXCLUDED.rating_value,
                updated_at = NOW()
            RETURNING
                id,
                task_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            id,
            task_id,
            rater_id,
            rating_value
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(task_rating)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskRating>, sqlx::Error> {
        query_as!(
            TaskRating,
            r#"
            SELECT
                id,
                task_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM task_ratings
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<TaskRating>, sqlx::Error> {
        query_as!(
            TaskRating,
            r#"
            SELECT
                id,
                task_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM task_ratings
            WHERE task_id = $1
            ORDER BY created_at DESC
            "#,
            task_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_rater(&self, rater_id: Uuid) -> Result<Vec<TaskRating>, sqlx::Error> {
        query_as!(
            TaskRating,
            r#"
            SELECT
                id,
                task_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM task_ratings
            WHERE rater_id = $1
            ORDER BY created_at DESC
            "#,
            rater_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_task_and_rater(
        &self,
        task_id: Uuid,
        rater_id: Uuid,
    ) -> Result<Option<TaskRating>, sqlx::Error> {
        query_as!(
            TaskRating,
            r#"
            SELECT
                id,
                task_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM task_ratings
            WHERE task_id = $1 AND rater_id = $2
            "#,
            task_id,
            rater_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_average_rating(&self, task_id: Uuid) -> Result<Option<f64>, sqlx::Error> {
        let result = query_scalar!(
            r#"
            SELECT AVG(rating_value)::FLOAT8 as "avg"
            FROM task_ratings
            WHERE task_id = $1
            "#,
            task_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_rating_count(&self, task_id: Uuid) -> Result<i64, sqlx::Error> {
        let count = query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM task_ratings
            WHERE task_id = $1
            "#,
            task_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.unwrap_or(0))
    }

    async fn update_rating(
        &self,
        id: Uuid,
        rating_value: i32,
    ) -> Result<TaskRating, sqlx::Error> {
        query_as!(
            TaskRating,
            r#"
            UPDATE task_ratings
            SET
                rating_value = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id,
                task_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            id,
            rating_value
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "DELETE FROM task_ratings WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn delete_by_task_and_rater(
        &self,
        task_id: Uuid,
        rater_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        query!(
            "DELETE FROM task_ratings WHERE task_id = $1 AND rater_id = $2",
            task_id,
            rater_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}