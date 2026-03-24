use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: i32,
    pub session_id: Uuid,
    pub user_id: i32,
    pub reservation_id: Option<Uuid>,
    pub invoice_number: String,
    pub qr_code_data: String,
    pub amount: Decimal,
    pub status: String,
    pub issued_at: NaiveDateTime,
    pub paid_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}