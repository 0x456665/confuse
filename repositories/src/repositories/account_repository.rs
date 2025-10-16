use crate::traits::AccountRepositoryTrait;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use models::Account;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct AccountRepository {
    db: PgPool,
}

impl AccountRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AccountRepositoryTrait for AccountRepository {
    async fn create_account(
        &self,
        user_id: Uuid,
        provider: String,
        provider_account_id: String,
        refresh_token: Option<String>,
        expires_at: Option<DateTime<Utc>>,
        token_type: Option<String>,
    ) -> Result<Account, sqlx::Error> {
        sqlx::query_as!(
            Account,
            r#"
            INSERT INTO accounts (
                user_id,
                provider,
                provider_account_id,
                refresh_token,
                expires_at,
                token_type
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                user_id,
                provider,
                provider_account_id,
                refresh_token,
                expires_at as "expires_at: DateTime<Utc>",
                token_type,
                updated_at as "updated_at!: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>"
            "#,
            user_id,
            provider,
            provider_account_id,
            refresh_token,
            expires_at,
            token_type
        )
        .fetch_one(&self.db)
        .await
    }

    async fn get_account_by_id(&self, account_id: Uuid) -> Result<Account, sqlx::Error> {
        sqlx::query_as!(
            Account,
            r#"
            SELECT
                id,
                user_id,
                provider,
                provider_account_id,
                refresh_token,
                expires_at as "expires_at: DateTime<Utc>",
                token_type,
                updated_at as "updated_at!: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>"
            FROM accounts
            WHERE id = $1
            "#,
            account_id
        )
        .fetch_one(&self.db)
        .await
    }

    async fn get_account_by_provider_id(
        &self,
        provider: String,
        provider_account_id: String,
    ) -> Result<Account, sqlx::Error> {
        sqlx::query_as!(
            Account,
            r#"
            SELECT
                id,
                user_id,
                provider,
                provider_account_id,
                refresh_token,
                expires_at as "expires_at?: DateTime<Utc>",
                token_type,
                updated_at as "updated_at!: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>"
            FROM accounts
            WHERE provider = $1 AND provider_account_id = $2
            "#,
            provider,
            provider_account_id
        )
        .fetch_one(&self.db)
        .await
    }

    async fn get_accounts_by_user_id(&self, user_id: Uuid) -> Result<Vec<Account>, sqlx::Error> {
        sqlx::query_as!(
            Account,
            r#"
            SELECT
                id,
                user_id,
                provider,
                provider_account_id,
                refresh_token,
                expires_at as "expires_at?: DateTime<Utc>",
                token_type,
                updated_at as "updated_at!: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>"
            FROM accounts
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.db)
        .await
    }

    async fn update_account(
        &self,
        account_id: Uuid,
        refresh_token: Option<String>,
        expires_at: Option<DateTime<Utc>>,
        token_type: Option<String>,
    ) -> Result<Account, sqlx::Error> {
        sqlx::query_as!(
            Account,
            r#"
            UPDATE accounts
            SET
                refresh_token = $2,
                expires_at = $3,
                token_type = $4,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING
                id,
                user_id,
                provider,
                provider_account_id,
                refresh_token,
                expires_at as "expires_at?: DateTime<Utc>",
                token_type,
                updated_at as "updated_at!: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>"
            "#,
            account_id,
            refresh_token,
            expires_at,
            token_type
        )
        .fetch_one(&self.db)
        .await
    }

    async fn delete_account(&self, account_id: Uuid) -> Result<Account, sqlx::Error> {
        sqlx::query_as!(
            Account,
            r#"
            DELETE FROM accounts
            WHERE id = $1
            RETURNING
                id,
                user_id,
                provider,
                provider_account_id,
                refresh_token,
                expires_at as "expires_at?: DateTime<Utc>",
                token_type,
                updated_at as "updated_at!: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>"
            "#,
            account_id
        )
        .fetch_one(&self.db)
        .await
    }
}
