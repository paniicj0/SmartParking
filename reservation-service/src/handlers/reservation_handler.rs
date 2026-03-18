use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    dto::reservation_dto::{
        CreateReservationRequest,
        MessageResponse,
        MyReservationResponse,
        ReservationResponse,
    },
    middleware::auth::AuthUser,
    services::reservation_service,
};

pub async fn create_reservation(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<CreateReservationRequest>,
) -> Result<Json<ReservationResponse>, (StatusCode, Json<Value>)> {
    let reservation =
        reservation_service::create_reservation(&state.db, auth_user.user_id, request)
            .await
            .map_err(|message| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "message": message })),
                )
            })?;

    Ok(Json(reservation))
}

pub async fn get_my_reservations(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<MyReservationResponse>>, (StatusCode, Json<Value>)> {
    let reservations = reservation_service::get_my_reservations(&state.db, auth_user.user_id)
        .await
        .map_err(|message| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": message })),
            )
        })?;

    Ok(Json(reservations))
}

pub async fn cancel_reservation(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<Value>)> {
    let message =
        reservation_service::cancel_reservation(&state.db, auth_user.user_id, reservation_id)
            .await
            .map_err(|message| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "message": message })),
                )
            })?;

    Ok(Json(MessageResponse { message }))
}