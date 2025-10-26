use crate::handlers::auth_handlers::{
    forgot_password_handler, login_handler, logout_handler, refresh_token_handler,
    register_handler, resend_verification_handler, reset_password_handler, verify_email_handler,
};
use axum::{Router, routing::post};
use shared::state::AppState;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/verify-email", post(verify_email_handler))
        .route("/resend-verification", post(resend_verification_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_token_handler))
        .route("/forgot-password", post(forgot_password_handler))
        .route("/reset-password", post(reset_password_handler))
        .route("/logout", post(logout_handler))
}
