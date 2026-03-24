pub mod invoice_routes;

use axum::Router;
use crate::app_state::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new().merge(invoice_routes::invoice_routes())
}