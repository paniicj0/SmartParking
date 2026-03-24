use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    app_state::AppState,
    dto::invoice_dto::{
        GenerateInvoiceRequest, InvoiceListItemResponse, InvoiceResponse, MessageResponse,
    },
    service::invoice_service,
};

pub async fn generate_invoice(
    State(state): State<AppState>,
    Json(request): Json<GenerateInvoiceRequest>,
) -> Result<(StatusCode, Json<InvoiceResponse>), (StatusCode, Json<MessageResponse>)> {
    match invoice_service::generate_invoice(&state.db, request).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(message) => Err((StatusCode::BAD_REQUEST, Json(MessageResponse { message }))),
    }
}

pub async fn get_invoices_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<(StatusCode, Json<Vec<InvoiceListItemResponse>>), (StatusCode, Json<MessageResponse>)> {
    match invoice_service::get_invoices_by_user_id(&state.db, user_id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(message) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MessageResponse { message }))),
    }
}

pub async fn mark_invoice_paid(
    State(state): State<AppState>,
    Path(invoice_id): Path<i32>,
) -> Result<(StatusCode, Json<InvoiceResponse>), (StatusCode, Json<MessageResponse>)> {
    match invoice_service::mark_invoice_paid(&state.db, invoice_id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(message) => {
            let status = if message == "Račun nije pronađen." {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };

            Err((status, Json(MessageResponse { message })))
        }
    }
}