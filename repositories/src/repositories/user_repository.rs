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
        email: &str,
        password_hash: Option<&str>,
        display_name: &str,
        bio: Option<&str>,
        first_name: Option<&str>,
        last_name: Option<&str>,
        avatar_url: Option<&str>,
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
    
    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, sqlx::Error> {
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
        .fetch_optional(&self.pool)
        .await
    }
    
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
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
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_user_by_display_name(&self, display_name: &str) -> Result<Option<User>, sqlx::Error> {
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
        .fetch_optional(&self.pool)
        .await
    }

    async fn update_user(
        &self,
        user_id: &Uuid,
        email: Option<&str>,
        password_hash: Option<&str>,
        display_name: Option<&str>,
        bio: Option<&str>,
        first_name: Option<&str>,
        last_name: Option<&str>,
        avatar_url: Option<&str>,
        reputation_score: Option<Decimal>,
        total_ratings_given: Option<i32>,
        total_ratings_received: Option<i32>,
        email_verified_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                email = COALESCE($2, email),
                password_hash = COALESCE($3, password_hash),
                display_name = COALESCE($4, display_name),
                bio = COALESCE($5, bio),
                first_name = COALESCE($6, first_name),
                last_name = COALESCE($7, last_name),
                avatar_url = COALESCE($8, avatar_url),
                reputation_score = COALESCE($9, reputation_score),
                total_ratings_given = COALESCE($10, total_ratings_given),
                total_ratings_received = COALESCE($11, total_ratings_received),
                email_verified_at = COALESCE($12, email_verified_at),
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

    async fn delete_user(&self, user_id: &Uuid) -> Result<User, sqlx::Error> {
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