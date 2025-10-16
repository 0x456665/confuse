use chrono::{DateTime, Utc};
use models::SubmissionRating;
use sqlx::{PgPool, query, query_as, query_scalar};
use uuid::Uuid;
use async_trait::async_trait;
use crate::traits::SubmissionRatingRepositoryTrait;

pub struct SubmissionRatingRepository {
    pool: PgPool,
}

impl SubmissionRatingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubmissionRatingRepositoryTrait for SubmissionRatingRepository {
    async fn create(
        &self,
        submission_id: Uuid,
        rater_id: Uuid,
        rating_value: i32,
    ) -> Result<SubmissionRating, sqlx::Error> {
        let id = Uuid::new_v4();

        query_as!(
            SubmissionRating,
            r#"
            INSERT INTO submission_ratings (id, submission_id, rater_id, rating_value)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                submission_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            id,
            submission_id,
            rater_id,
            rating_value,
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SubmissionRating>, sqlx::Error> {
        query_as!(
            SubmissionRating,
            r#"
            SELECT
                id,
                submission_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM submission_ratings
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_submission_and_rater(
        &self,
        submission_id: Uuid,
        rater_id: Uuid,
    ) -> Result<Option<SubmissionRating>, sqlx::Error> {
        query_as!(
            SubmissionRating,
            r#"
            SELECT
                id,
                submission_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM submission_ratings
            WHERE submission_id = $1 AND rater_id = $2
            "#,
            submission_id,
            rater_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_submission(&self, submission_id: Uuid) -> Result<Vec<SubmissionRating>, sqlx::Error> {
        query_as!(
            SubmissionRating,
            r#"
            SELECT
                id,
                submission_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM submission_ratings
            WHERE submission_id = $1
            ORDER BY created_at DESC
            "#,
            submission_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_by_rater(&self, rater_id: Uuid) -> Result<Vec<SubmissionRating>, sqlx::Error> {
        query_as!(
            SubmissionRating,
            r#"
            SELECT
                id,
                submission_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM submission_ratings
            WHERE rater_id = $1
            ORDER BY created_at DESC
            "#,
            rater_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn get_average_rating(&self, submission_id: Uuid) -> Result<Option<f64>, sqlx::Error> {
        query_scalar!(
            r#"
            SELECT AVG(rating_value)::FLOAT8 as "avg"
            FROM submission_ratings
            WHERE submission_id = $1
            "#,
            submission_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn count_by_submission(&self, submission_id: Uuid) -> Result<i64, sqlx::Error> {
        query_scalar!(
            r#"
            SELECT COUNT(*)::BIGINT as "count!"
            FROM submission_ratings
            WHERE submission_id = $1
            "#,
            submission_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        rating_value: i32,
    ) -> Result<SubmissionRating, sqlx::Error> {
        query_as!(
            SubmissionRating,
            r#"
            UPDATE submission_ratings
            SET
                rating_value = $2,
                updated_at = $3
            WHERE id = $1
            RETURNING
                id,
                submission_id,
                rater_id,
                rating_value as "rating_value!: i32",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            id,
            rating_value,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        query!(
            "DELETE FROM submission_ratings WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_by_submission_and_rater(
        &self,
        submission_id: Uuid,
        rater_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        query!(
            "DELETE FROM submission_ratings WHERE submission_id = $1 AND rater_id = $2",
            submission_id,
            rater_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}