use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct TaskRating {
    pub id: Uuid,
    pub task_id: Uuid,
    pub rater_id: Uuid,
    pub rating_value: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
