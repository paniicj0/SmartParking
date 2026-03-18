use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ParkingSpot {
    pub id: Uuid,
    pub label: String,
    pub floor: Option<i32>,
    pub zone: String,
    pub spot_type: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}