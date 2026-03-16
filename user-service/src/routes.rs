use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{get_me, health, login_user, register_user, AppState};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .route("/users/me", get(get_me))
        .with_state(state)
}