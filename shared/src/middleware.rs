use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tracing::error;

/// Global error handling middleware
pub async fn error_handler_middleware(
    req: Request,
    next: Next,
) -> Response {
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

pub async fn check_access_token_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req.headers().get("Authorization");
    
    match token {
        Some(token) => {
            let token = token.to_str().unwrap();
            
            if token.starts_with("Bearer ") {
                let token = token.strip_prefix("Bearer ").unwrap();
                
                //TODO: Validate token
                
                Ok(next.run(req).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}