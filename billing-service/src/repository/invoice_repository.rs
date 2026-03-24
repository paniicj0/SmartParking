use sqlx::PgPool;
use uuid::Uuid;

use crate::models::invoice::Invoice;

pub async fn create_invoice(
    pool: &PgPool,
    session_id: Uuid,
    user_id: i32,
    reservation_id: Option<Uuid>,
    invoice_number: String,
    qr_code_data: String,
    amount: f64,
) -> Result<Invoice, sqlx::Error> {
    let invoice = sqlx::query_as::<_, Invoice>(
        r#"
        INSERT INTO invoices
        (session_id, user_id, reservation_id, invoice_number, qr_code_data, amount, status, issued_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, 'Unpaid', NOW(), NOW(), NOW())
        RETURNING id, session_id, user_id, reservation_id, invoice_number, qr_code_data, amount, status, issued_at, paid_at, created_at, updated_at
        "#,
    )
    .bind(session_id)
    .bind(user_id)
    .bind(reservation_id)
    .bind(invoice_number)
    .bind(qr_code_data)
    .bind(amount)
    .fetch_one(pool)
    .await?;

    Ok(invoice)
}

pub async fn get_invoices_by_user_id(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<Invoice>, sqlx::Error> {
    let invoices = sqlx::query_as::<_, Invoice>(
        r#"
        SELECT id, session_id, user_id, reservation_id, invoice_number, qr_code_data, amount, status, issued_at, paid_at, created_at, updated_at
        FROM invoices
        WHERE user_id = $1
        ORDER BY issued_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(invoices)
}

pub async fn mark_invoice_paid(
    pool: &PgPool,
    invoice_id: i32,
) -> Result<Option<Invoice>, sqlx::Error> {
    let invoice = sqlx::query_as::<_, Invoice>(
        r#"
        UPDATE invoices
        SET status = 'Paid',
            paid_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, session_id, user_id, reservation_id, invoice_number, qr_code_data, amount, status, issued_at, paid_at, created_at, updated_at
        "#,
    )
    .bind(invoice_id)
    .fetch_optional(pool)
    .await?;

    Ok(invoice)
}