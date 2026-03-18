use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Reservation {
    pub id: Uuid,
    pub user_id: i32,
    pub vehicle_id: i32,
    pub parking_spot_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ReservationWithSpotInfo {
    pub id: Uuid,
    pub parking_spot_id: Uuid,
    pub parking_spot_label: String,
    pub zone: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub status: String,
}