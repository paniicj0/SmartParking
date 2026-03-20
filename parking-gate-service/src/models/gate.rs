use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Gate {
    pub id: Uuid,
    pub name: String,
    pub gate_type: String,
    pub qr_value: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}