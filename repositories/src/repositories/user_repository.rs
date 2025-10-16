use async_trait::async_trait;
use chrono::{DateTime, Utc};
use models::User;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;
use crate::traits::UserRepositoryTrait;

#[derive(Debug)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create_user(
        &self,
        email: String,
        password_hash: Option<String>,
        display_name: String,
        bio: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        avatar_url: Option<String>,
        email_verified_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                email,
                password_hash,
                display_name,
                bio,
                first_name,
                last_name,
                avatar_url,
                email_verified_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                id,
                email,
                password_hash,
                display_name,
                bio,
                first_name,
                last_name,
                avatar_url,
                reputation_score as "reputation_score!: Decimal",
                total_ratings_given as "total_ratings_given!: i32",
                total_ratings_received as "total_ratings_received!: i32",
                email_verified_at as "email_verified_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            email,
            password_hash,
            display_name,
            bio,
            first_name,
            last_name,
            avatar_url,
            email_verified_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                email,
                password_hash,
                display_name,
                bio,
                first_name,
                last_name,
                avatar_url,
                reputation_score as "reputation_score!: Decimal",
                total_ratings_given as "total_ratings_given!: i32",
                total_ratings_received as "total_ratings_received!: i32",
                email_verified_at as "email_verified_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn get_user_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                email,
                password_hash,
                display_name,
                bio,
                first_name,
                last_name,
                avatar_url,
                reputation_score as "reputation_score!: Decimal",
                total_ratings_given as "total_ratings_given!: i32",
                total_ratings_received as "total_ratings_received!: i32",
                email_verified_at as "email_verified_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn get_user_by_display_name(&self, display_name: String) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                email,
                password_hash,
                display_name,
                bio,
                first_name,
                last_name,
                avatar_url,
                reputation_score as "reputation_score!: Decimal",
                total_ratings_given as "total_ratings_given!: i32",
                total_ratings_received as "total_ratings_received!: i32",
                email_verified_at as "email_verified_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM users
            WHERE display_name = $1
            "#,
            display_name
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update_user(
        &self,
        user_id: Uuid,
        email: String,
        password_hash: Option<String>,
        display_name: String,
        bio: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        avatar_url: Option<String>,
        reputation_score: Decimal,
        total_ratings_given: i32,
        total_ratings_received: i32,
        email_verified_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                email = $2,
                password_hash = $3,
                display_name = $4,
                bio = $5,
                first_name = $6,
                last_name = $7,
                avatar_url = $8,
                reputation_score = $9,
                total_ratings_given = $10,
                total_ratings_received = $11,
                email_verified_at = $12,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING
                id,
                email,
                password_hash,
                display_name,
                bio,
                first_name,
                last_name,
                avatar_url,
                reputation_score as "reputation_score!: Decimal",
                total_ratings_given as "total_ratings_given!: i32",
                total_ratings_received as "total_ratings_received!: i32",
                email_verified_at as "email_verified_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            user_id,
            email,
            password_hash,
            display_name,
            bio,
            first_name,
            last_name,
            avatar_url,
            reputation_score,
            total_ratings_given,
            total_ratings_received,
            email_verified_at
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING
                id,
                email,
                password_hash,
                display_name,
                bio,
                first_name,
                last_name,
                avatar_url,
                reputation_score as "reputation_score!: Decimal",
                total_ratings_given as "total_ratings_given!: i32",
                total_ratings_received as "total_ratings_received!: i32",
                email_verified_at as "email_verified_at: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }
}
