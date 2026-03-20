use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use uuid::Uuid;
use chrono::Local;

use crate::{
    app_state::AppState,
    dto::reservation_dto::{
        CreateReservationRequest,
        MessageResponse,
        MyReservationResponse,
        ReservationResponse,
        ExpireOldResponse,  ValidForEntryQuery, ValidForEntryResponse
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


pub async fn get_valid_for_entry(
    State(state): State<AppState>,
    Query(query): Query<ValidForEntryQuery>,
) -> Result<(StatusCode, Json<ValidForEntryResponse>), (StatusCode, Json<MessageResponse>)> {
    match reservation_service::get_valid_for_entry(&state.db, query.user_id, query.vehicle_id, query.time).await {
        Ok(Some(reservation)) => Ok((StatusCode::OK, Json(reservation))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(MessageResponse {
                message: "Ne postoji validna rezervacija za ovo vozilo u dozvoljenom vremenskom periodu.".to_string(),
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Greška prilikom provere validne rezervacije.".to_string(),
            }),
        )),
    }
}

pub async fn mark_used(
    State(state): State<AppState>,
    Path(reservation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<MessageResponse>), (StatusCode, Json<MessageResponse>)> {
    match reservation_service::mark_used(&state.db, reservation_id).await {
        Ok(true) => Ok((
            StatusCode::OK,
            Json(MessageResponse {
                message: "Rezervacija je uspešno označena kao Used.".to_string(),
            }),
        )),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(MessageResponse {
                message: "Rezervacija nije pronađena ili više nije Confirmed.".to_string(),
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Greška prilikom izmene statusa rezervacije.".to_string(),
            }),
        )),
    }
}

pub async fn expire_old(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<ExpireOldResponse>), (StatusCode, Json<MessageResponse>)> {
    let now = Local::now().naive_local();

    match reservation_service::expire_old(&state.db, now).await {
        Ok(expired_count) => Ok((
            StatusCode::OK,
            Json(ExpireOldResponse {
                message: "Stare rezervacije su obrađene.".to_string(),
                expired_count,
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Greška prilikom obrade isteklih rezervacija.".to_string(),
            }),
        )),
    }
}