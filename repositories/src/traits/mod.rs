pub mod account_repo_trait;
pub mod problems_or_task_repo_trait;
pub mod submission_comment_replies_repo_trait;
pub mod submission_comment_repo_trait;
pub mod submission_rating_repo_trait;
pub mod submission_repo_trait;
pub mod task_comment_replies_repo_trait;
pub mod task_comment_repo_trait;
pub mod task_rating_repo_trait;
pub mod user_repo_trait;

// Re-export the traits
pub use account_repo_trait::*;
pub use problems_or_task_repo_trait::*;
pub use submission_comment_replies_repo_trait::*;
pub use submission_comment_repo_trait::*;
pub use submission_rating_repo_trait::*;
pub use submission_repo_trait::*;
pub use task_comment_replies_repo_trait::*;
pub use task_comment_repo_trait::*;
pub use task_rating_repo_trait::*;
pub use user_repo_trait::*;
