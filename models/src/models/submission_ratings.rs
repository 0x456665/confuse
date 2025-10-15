use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct SubmissionRating {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub rater_id: Uuid,
    pub rating_value: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

