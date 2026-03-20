use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::parking_session_handler::{enter_parking, get_active_session, exit_parking, get_parking_history},
    app_state::AppState,
};

async fn health() -> &'static str {
    "Parking/Gate Service is running"
}

pub fn parking_session_routes() -> Router<AppState> {
    Router::new()
    .route("/health", get(health))
    .route("/parking-sessions/entry", post(enter_parking))
    .route("/parking-sessions/active", get(get_active_session))
    .route("/parking-sessions/exit", post(exit_parking))
    .route("/parking-sessions/history", get(get_parking_history))
}