use axum::{
    routing::{get, post, patch},
    Router,
};

use crate::app_state::AppState;
use crate::handlers::{
    parking_spot_handler,
    reservation_handler,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/reservations/spots", get(parking_spot_handler::get_all_spots))
        .route("/api/reservations/spots/status", get(parking_spot_handler::get_spot_statuses))
        .route("/api/reservations", post(reservation_handler::create_reservation))
        .route("/api/reservations/my", get(reservation_handler::get_my_reservations))
        .route("/api/reservations/:id/cancel", patch(reservation_handler::cancel_reservation))
        .with_state(app_state)
}