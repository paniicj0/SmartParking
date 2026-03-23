use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::*,
    state::AppState,
};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .route("/auth/activate", get(activate_user))
        .route("/users/me", get(get_me).put(update_me))
        .route("/users/me/change-password", put(change_password))
        .route("/vehicles/me", get(get_my_vehicles))
        .route("/vehicles", post(create_vehicle))
        .route("/vehicles/:id", put(update_vehicle).delete(delete_vehicle))
        .route("/internal/users/:id/email", get(get_user_email_by_id))
        .with_state(state)
}