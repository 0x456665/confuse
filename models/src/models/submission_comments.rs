use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct SubmissionComment {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub user_id: Uuid,
    pub comment: String,
    pub is_edited: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
