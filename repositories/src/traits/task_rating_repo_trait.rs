use async_trait::async_trait;
use uuid::Uuid;
use models::TaskRating;

#[async_trait]
pub trait TaskRatingRepositoryTrait: Send + Sync {
    async fn create(
        &self,
        task_id: Uuid,
        rater_id: Uuid,
        rating_value: i32,
    ) -> Result<TaskRating, sqlx::Error>;
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskRating>, sqlx::Error>;
    
    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<TaskRating>, sqlx::Error>;
    
    async fn find_by_rater(&self, rater_id: Uuid) -> Result<Vec<TaskRating>, sqlx::Error>;
    
    async fn find_by_task_and_rater(
        &self,
        task_id: Uuid,
        rater_id: Uuid,
    ) -> Result<Option<TaskRating>, sqlx::Error>;
    
    async fn get_average_rating(&self, task_id: Uuid) -> Result<Option<f64>, sqlx::Error>;
    
    async fn get_rating_count(&self, task_id: Uuid) -> Result<i64, sqlx::Error>;
    
    async fn update_rating(
        &self,
        id: Uuid,
        rating_value: i32,
    ) -> Result<TaskRating, sqlx::Error>;
    
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    
    async fn delete_by_task_and_rater(
        &self,
        task_id: Uuid,
        rater_id: Uuid,
    ) -> Result<(), sqlx::Error>;
}