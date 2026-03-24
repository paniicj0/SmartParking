use chrono::NaiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::prelude::ToPrimitive;
use crate::dto::invoice_dto::{
    GenerateInvoiceRequest, InvoiceListItemResponse, InvoiceResponse,
};
use crate::repository::invoice_repository;
use crate::service::{email_service, qr_service, user_service_client};

pub async fn generate_invoice(
    pool: &PgPool,
    request: GenerateInvoiceRequest,
) -> Result<InvoiceResponse, String> {
    let start_time = NaiveDateTime::parse_from_str(&request.start_time, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(&request.start_time, "%Y-%m-%dT%H:%M"))
        .map_err(|_| "Neispravan format start_time.".to_string())?;

    let end_time = NaiveDateTime::parse_from_str(&request.end_time, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(&request.end_time, "%Y-%m-%dT%H:%M"))
        .map_err(|_| "Neispravan format end_time.".to_string())?;

    if end_time <= start_time {
        return Err("Vreme završetka mora biti nakon vremena početka.".to_string());
    }

    let duration_minutes = (end_time - start_time).num_minutes();
    let started_hours = ((duration_minutes + 59) / 60).max(1);
    let price_per_hour = 100.0;
    let total_amount = started_hours as f64 * price_per_hour;

    let invoice_number = format!("INV-{}", Uuid::new_v4());
    let qr_code_data = format!(
        "invoice_number={};session_id={};amount={:.2}",
        invoice_number, request.session_id, total_amount
    );

    let invoice = invoice_repository::create_invoice(
        pool,
        request.session_id,
        request.user_id,
        request.reservation_id,
        invoice_number,
        qr_code_data,
        total_amount,
    )
    .await
    .map_err(|e| format!("Greška pri kreiranju računa: {}", e))?;

    match user_service_client::get_user_email(request.user_id).await {
        Ok(user_email) => {
            match qr_service::generate_qr_png_bytes(&invoice.qr_code_data) {
                Ok(qr_png) => {
                    match email_service::send_invoice_email(
                        &user_email,
                        &invoice.invoice_number,
                        total_amount,
                        &invoice.qr_code_data,
                        qr_png,
                    )
                    .await
                    {
                        Ok(_) => println!("Email sa inline QR kodom uspešno poslat."),
                        Err(e) => eprintln!("Greška pri slanju email-a: {}", e),
                    }
                }
                Err(e) => eprintln!("Greška pri generisanju QR PNG: {}", e),
            }
        }
        Err(e) => eprintln!("Greška pri dobavljanju email adrese korisnika: {}", e),
    }

    Ok(InvoiceResponse {
        id: invoice.id,
        session_id: invoice.session_id,
        invoice_number: invoice.invoice_number,
        qr_code_data: invoice.qr_code_data,
        amount: total_amount,
        status: invoice.status,
        issued_at: invoice.issued_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    })
}

pub async fn get_invoices_by_user_id(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<InvoiceListItemResponse>, String> {
    let invoices = invoice_repository::get_invoices_by_user_id(pool, user_id)
        .await
        .map_err(|e| format!("Greška pri učitavanju računa: {}", e))?;

    let response = invoices
        .into_iter()
        .map(|invoice| InvoiceListItemResponse {
            id: invoice.id,
            session_id: invoice.session_id,
            invoice_number: invoice.invoice_number,
            qr_code_data: invoice.qr_code_data,
            amount: invoice.amount.to_f64().unwrap_or(0.0),
            status: invoice.status,
            issued_at: invoice.issued_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            paid_at: invoice
                .paid_at
                .map(|p| p.format("%Y-%m-%dT%H:%M:%S").to_string()),
        })
        .collect();

    Ok(response)
}

pub async fn mark_invoice_paid(
    pool: &PgPool,
    invoice_id: i32,
) -> Result<InvoiceResponse, String> {
    let invoice = invoice_repository::mark_invoice_paid(pool, invoice_id)
        .await
        .map_err(|_| "Greška pri ažuriranju računa.".to_string())?;

    let invoice = invoice.ok_or_else(|| "Račun nije pronađen.".to_string())?;

    Ok(InvoiceResponse {
        id: invoice.id,
        session_id: invoice.session_id,
        invoice_number: invoice.invoice_number,
        qr_code_data: invoice.qr_code_data,
        amount: invoice.amount.to_f64().unwrap_or(0.0),
        status: invoice.status,
        issued_at: invoice.issued_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    })
}