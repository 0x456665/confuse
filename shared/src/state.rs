use crate::config::Config;
use axum::extract::FromRef;
use redis::aio::MultiplexedConnection;
use repositories::{
    repositories::{
        AccountRepository, ProblemOrTaskRepository, SubmissionCommentReplyRepository,
        SubmissionCommentRepository, SubmissionRatingRepository, SubmissionRepository,
        TaskCommentReplyRepository, TaskCommentRepository, TaskRatingRepository, UserRepository,
    },
    traits::{
        AccountRepositoryTrait, ProblemOrTaskRepositoryTrait,
        SubmissionCommentReplyRepositoryTrait, SubmissionCommentRepositoryTrait,
        SubmissionRatingRepositoryTrait, SubmissionRepositoryTrait,
        TaskCommentReplyRepositoryTrait, TaskCommentRepositoryTrait, TaskRatingRepositoryTrait,
        UserRepositoryTrait,
    },
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
    pub repos: AppRepositories,
    pub redis: MultiplexedConnection,
}

#[derive(Clone)]
pub struct AppRepositories {
    pub user: Arc<dyn UserRepositoryTrait>,
    pub account: Arc<dyn AccountRepositoryTrait>,
    pub problem_or_task: Arc<dyn ProblemOrTaskRepositoryTrait>,
    pub submission: Arc<dyn SubmissionRepositoryTrait>,
    pub task_rating: Arc<dyn TaskRatingRepositoryTrait>,
    pub submission_rating: Arc<dyn SubmissionRatingRepositoryTrait>,
    pub task_comment: Arc<dyn TaskCommentRepositoryTrait>,
    pub submission_comment: Arc<dyn SubmissionCommentRepositoryTrait>,
    pub task_comment_reply: Arc<dyn TaskCommentReplyRepositoryTrait>,
    pub submission_comment_reply: Arc<dyn SubmissionCommentReplyRepositoryTrait>,
}

impl AppState {
    pub fn new(db: PgPool, config: Config, redis: MultiplexedConnection) -> Self {
        Self {
            db: db.clone(),
            redis: redis.clone(),
            config,
            repos: AppRepositories::new(db.clone()),
        }
    }
}

impl AppRepositories {
    pub fn new(db: PgPool) -> Self {
        Self {
            user: Arc::new(UserRepository::new(db.clone())),
            account: Arc::new(AccountRepository::new(db.clone())),
            problem_or_task: Arc::new(ProblemOrTaskRepository::new(db.clone())),
            submission: Arc::new(SubmissionRepository::new(db.clone())),
            task_rating: Arc::new(TaskRatingRepository::new(db.clone())),
            submission_rating: Arc::new(SubmissionRatingRepository::new(db.clone())),
            task_comment: Arc::new(TaskCommentRepository::new(db.clone())),
            submission_comment: Arc::new(SubmissionCommentRepository::new(db.clone())),
            task_comment_reply: Arc::new(TaskCommentReplyRepository::new(db.clone())),
            submission_comment_reply: Arc::new(SubmissionCommentReplyRepository::new(db)),
        }
    }
}
