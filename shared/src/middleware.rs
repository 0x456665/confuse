use crate::{
    auth_utils::{extract_bearer_token, extract_user_id},
    errors::AppError,
    state::AppState,
};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use tracing::error;

/// Global error handling middleware
pub async fn error_handler_middleware(req: Request, next: Next) -> Response {
    let uri = req.uri().clone();
    let method = req.method().clone();

    let response = next.run(req).await;

    // Log errors (4xx and 5xx)
    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        error!(
            method = %method,
            uri = %uri,
            status = %status,
            "Request failed"
        );
    }

    response
}

pub async fn auth_middleware(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer_token(&req)?;
    let user_id = extract_user_id(token, &app_state.config.access_secret)?;

    // Store user_id in request extensions
    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}

/// Refresh token middleware - validates refresh token
/// eventually convert to cookies
pub async fn refresh_token_middleware(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer_token(&req)?;
    let user_id = extract_user_id(token, &app_state.config.refresh_secret)?;

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
