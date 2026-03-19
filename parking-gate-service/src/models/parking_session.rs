use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ParkingSession {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub user_id: i32,
    pub vehicle_id: i32,
    pub parking_spot_id: Uuid,
    pub entry_gate_id: Uuid,
    pub exit_gate_id: Option<Uuid>,
    pub entry_time: NaiveDateTime,
    pub exit_time: Option<NaiveDateTime>,
    pub status: String,
    pub planned_start_time: NaiveDateTime,
    pub planned_end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}