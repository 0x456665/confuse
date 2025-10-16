use async_trait::async_trait;
use uuid::Uuid;
use models::ProblemOrTask;

#[async_trait]
pub trait ProblemOrTaskRepositoryTrait: Send + Sync {
    async fn create(
        &self,
        user_id: Uuid,
        title: String,
        content: String,
        file_url: Option<String>,
        tags: Vec<String>,
        difficulty: String,
    ) -> Result<ProblemOrTask, sqlx::Error>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ProblemOrTask>, sqlx::Error>;
    async fn find_all(&self) -> Result<Vec<ProblemOrTask>, sqlx::Error>;
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<ProblemOrTask>, sqlx::Error>;

    async fn update(
        &self,
        id: Uuid,
        title: Option<String>,
        content: Option<String>,
        file_url: Option<String>,
        tags: Option<Vec<String>>,
        difficulty: Option<String>,
    ) -> Result<ProblemOrTask, sqlx::Error>;

    async fn increment_views(&self, id: Uuid) -> Result<(), sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}
