pub mod handlers;
pub mod routes;
pub mod schema;
pub mod services;
pub mod utils;

use crate::routes::auth_router::auth_router;
use axum::Router;
use shared::state::AppState;

pub async fn app() -> Router<AppState> {
    Router::new().nest("/auth", auth_router())
}
