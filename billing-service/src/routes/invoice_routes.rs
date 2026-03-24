use axum::{routing::{get, post, patch}, Router};

use crate::{
    app_state::AppState,
    handlers::invoice_handler,
};

pub fn invoice_routes() -> Router<AppState> {
    Router::new()
        .route("/invoices/generate", post(invoice_handler::generate_invoice))
        .route("/invoices/user/:user_id", get(invoice_handler::get_invoices_by_user_id))
        .route("/invoices/:id/pay", patch(invoice_handler::mark_invoice_paid))
}