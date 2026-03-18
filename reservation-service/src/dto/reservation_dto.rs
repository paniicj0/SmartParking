use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateReservationRequest {
    pub vehicle_id: i32,
    pub parking_spot_id: Uuid,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Serialize)]
pub struct ReservationResponse {
    pub id: Uuid,
    pub user_id: i32,
    pub vehicle_id: i32,
    pub parking_spot_id: Uuid,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct MyReservationResponse {
    pub id: Uuid,
    pub parking_spot_id: Uuid,
    pub parking_spot_label: String,
    pub zone: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
}
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
