use async_trait::async_trait;
use uuid::Uuid;
use models::SubmissionRating;

#[async_trait]
pub trait SubmissionRatingRepositoryTrait: Send + Sync {
    /// Create a new submission rating
    async fn create(
        &self,
        submission_id: Uuid,
        rater_id: Uuid,
        rating_value: i32,
    ) -> Result<SubmissionRating, sqlx::Error>;
    
    /// Find a rating by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SubmissionRating>, sqlx::Error>;
    
    /// Find a specific rating by submission and rater (unique constraint)
    async fn find_by_submission_and_rater(
        &self,
        submission_id: Uuid,
        rater_id: Uuid,
    ) -> Result<Option<SubmissionRating>, sqlx::Error>;
    
    /// Find all ratings for a specific submission
    async fn find_by_submission(&self, submission_id: Uuid) -> Result<Vec<SubmissionRating>, sqlx::Error>;
    
    /// Find all ratings by a specific rater
    async fn find_by_rater(&self, rater_id: Uuid) -> Result<Vec<SubmissionRating>, sqlx::Error>;
    
    /// Get average rating for a submission
    async fn get_average_rating(&self, submission_id: Uuid) -> Result<Option<f64>, sqlx::Error>;
    
    /// Get total count of ratings for a submission
    async fn count_by_submission(&self, submission_id: Uuid) -> Result<i64, sqlx::Error>;
    
    /// Update an existing rating
    async fn update(
        &self,
        id: Uuid,
        rating_value: i32,
    ) -> Result<SubmissionRating, sqlx::Error>;
    
    /// Delete a rating
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    
    /// Delete a rating by submission and rater
    async fn delete_by_submission_and_rater(
        &self,
        submission_id: Uuid,
        rater_id: Uuid,
    ) -> Result<(), sqlx::Error>;
}