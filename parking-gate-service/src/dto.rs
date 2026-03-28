use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct EntryRequest {
    pub gate_id: Uuid,
    pub vehicle_id: i32,
}

#[derive(Debug, Serialize)]
pub struct EntryResponse {
    pub message: String,
    pub session_id: Uuid,
    pub reservation_id: Uuid,
    pub parking_spot_id: Uuid,
    pub entry_time: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ActiveParkingSessionResponse {
    pub session_id: Uuid,
    pub reservation_id: Uuid,
    pub vehicle_id: i32,
    pub parking_spot_id: Uuid,
    pub entry_time: String,
    pub planned_start_time: String,
    pub planned_end_time: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ExitRequest {
    pub gate_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ExitResponse {
    pub message: String,
    pub session_id: Uuid,
    pub entry_time: String,
    pub exit_time: String,
    pub duration_minutes: i64,
    pub billable_hours: i64,
    pub total_amount: i64,
}

#[derive(Debug, Serialize)]
pub struct ParkingHistoryItemResponse {
    pub session_id: Uuid,
    pub reservation_id: Uuid,
    pub vehicle_id: i32,
    pub parking_spot_id: Uuid,
    pub entry_time: String,
    pub exit_time: Option<String>,
    pub planned_start_time: String,
    pub planned_end_time: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ParkingHistoryResponse {
    pub items: Vec<ParkingHistoryItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ParkingHistoryQuery {
    pub status: Option<String>,
}


#[derive(Serialize)]
pub struct GenerateInvoiceRequest {
    pub session_id: Uuid,
    pub user_id: i32,
    pub reservation_id: Option<Uuid>,
    pub start_time: String,
    pub end_time: String,
    pub price_per_hour: f64,
}
