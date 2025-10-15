use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct ProblemOrTask {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub file_url: Option<String>,
    pub tags: Vec<String>,
    pub difficulty: String,
    pub average_rating: u8,
    pub total_ratings: u32,
    pub total_submissions: u32,
    pub view_count: i32,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
