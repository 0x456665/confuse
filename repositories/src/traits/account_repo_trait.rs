use async_trait::async_trait;
use chrono::{DateTime, Utc};
use models::Account;
use uuid::Uuid;

#[async_trait]
pub trait AccountRepositoryTrait: Send + Sync {
    async fn create_account(
        &self,
        user_id: Uuid,
        provider: String,
        provider_account_id: String,
        refresh_token: Option<String>,
        expires_at: Option<DateTime<Utc>>,
        token_type: Option<String>,
    ) -> Result<Account, sqlx::Error>;

    async fn get_account_by_id(&self, account_id: Uuid) -> Result<Account, sqlx::Error>;

    async fn get_account_by_provider_id(
        &self,
        provider: String,
        provider_account_id: String,
    ) -> Result<Account, sqlx::Error>;

    async fn get_accounts_by_user_id(&self, user_id: Uuid) -> Result<Vec<Account>, sqlx::Error>;

    async fn update_account(
        &self,
        account_id: Uuid,
        refresh_token: Option<String>,
        expires_at: Option<DateTime<Utc>>,
        token_type: Option<String>,
    ) -> Result<Account, sqlx::Error>;

    async fn delete_account(&self, account_id: Uuid) -> Result<Account, sqlx::Error>;
}
