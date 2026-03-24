use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateInvoiceRequest {
    pub session_id: Uuid,
    pub user_id: i32,
    pub reservation_id: Option<Uuid>,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub id: i32,
    pub session_id: Uuid,
    pub invoice_number: String,
    pub qr_code_data: String,
    pub amount: f64,
    pub status: String,
    pub issued_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceListItemResponse {
    pub id: i32,
    pub session_id: Uuid,
    pub invoice_number: String,
    pub qr_code_data: String,
    pub amount: f64,
    pub status: String,
    pub issued_at: String,
    pub paid_at: Option<String>,
}