use chrono::NaiveDateTime;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidReservationResponse {
    pub id: Uuid,
    pub user_id: i32,
    pub vehicle_id: i32,
    pub parking_spot_id: Uuid,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
}

pub async fn get_valid_reservation_for_entry(
    reservation_service_url: &str,
    user_id: i32,
    vehicle_id: i32,
    time: NaiveDateTime,
) -> Result<Option<ValidReservationResponse>, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "{}/internal/reservations/valid-for-entry",
            reservation_service_url
        ))
        .query(&[
            ("userId", user_id.to_string()),
            ("vehicleId", vehicle_id.to_string()),
            ("time", time.format("%Y-%m-%dT%H:%M:%S").to_string()),
        ])
        .send()
        .await?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let response = response.error_for_status()?;
    let reservation = response.json::<ValidReservationResponse>().await?;
    Ok(Some(reservation))
}

pub async fn mark_reservation_used(
    reservation_service_url: &str,
    reservation_id: Uuid,
) -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .patch(format!(
            "{}/internal/reservations/{}/mark-used",
            reservation_service_url, reservation_id
        ))
        .send()
        .await?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(false);
    }

    response.error_for_status()?;
    Ok(true)
}