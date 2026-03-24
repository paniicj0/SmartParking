use axum::{
    routing::{get, post, patch, put},
    Router,
};

use crate::app_state::AppState;
use crate::handlers::{
    parking_spot_handler,
    reservation_handler
};

use crate::{
    handlers::reservation_handler::{expire_old, get_valid_for_entry, mark_used},
    handlers::parking_spot_handler::{create_parking_spot,delete_parking_spot,update_parking_spot, get_all_spots,get_daily_occupancy,get_occupancy_summary}
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/reservations/spots", get(parking_spot_handler::get_all_spots))
        .route("/api/reservations/spots/status", get(parking_spot_handler::get_spot_statuses))
        .route("/api/reservations", post(reservation_handler::create_reservation))
        .route("/api/reservations/my", get(reservation_handler::get_my_reservations))
        .route("/api/reservations/:id/cancel", patch(reservation_handler::cancel_reservation))
        .route("/internal/reservations/valid-for-entry", get(get_valid_for_entry))
        .route("/internal/reservations/:id/mark-used", patch(mark_used))
        .route("/internal/reservations/expire-old", post(expire_old))
        .route("/admin/parking-spots", get(get_all_spots).post(create_parking_spot))
        .route("/admin/parking-spots/:id", put(update_parking_spot).delete(delete_parking_spot))
        .route("/admin/parking-spots/occupancy", get(get_occupancy_summary))
        .route("/admin/parking-spots/analytics/daily-occupancy", get(get_daily_occupancy))

        .with_state(app_state)
}