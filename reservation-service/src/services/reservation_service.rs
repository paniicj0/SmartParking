use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::reservation_dto::{CreateReservationRequest, ReservationResponse, MyReservationResponse, ValidForEntryResponse},
    repositories::reservation_repository::{self, has_overlapping_reservation_for_vehicle},
};

pub async fn create_reservation(
    pool: &PgPool,
    user_id: i32,
    request: CreateReservationRequest,
) -> Result<ReservationResponse, String> { 
    let start_time = NaiveDateTime::parse_from_str(&request.start_time, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| "Neispravan format start_time".to_string())?;

    let end_time = NaiveDateTime::parse_from_str(&request.end_time, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| "Neispravan format end_time".to_string())?;

    if start_time >= end_time {
        return Err("Početno vreme mora biti pre krajnjeg vremena.".to_string());
    }

    let now = Utc::now().naive_utc();
    if start_time < now {
        return Err("Rezervacija ne može biti u prošlosti.".to_string());
    }
    
    let has_overlap: bool = has_overlapping_reservation_for_vehicle(
        pool,
        request.vehicle_id,
        start_time,
        end_time,
    )
    .await
    .map_err(|_| "Greška pri proveri postojećih rezervacija.".to_string())?;
    
    if has_overlap {
        return Err("Za ovo vozilo već postoji rezervacija u izabranom periodu.".to_string());
    }

    let spot = reservation_repository::get_parking_spot_by_id(pool, request.parking_spot_id)
        .await
        .map_err(|_| "Greška pri čitanju parking mesta.".to_string())?;

    let spot = match spot {
        Some(spot) => spot,
        None => return Err("Parking mesto ne postoji.".to_string()),
    };

    if !spot.is_active {
        return Err("Parking mesto nije aktivno.".to_string());
    }

    let has_conflict = reservation_repository::has_conflicting_reservation(
        pool,
        request.parking_spot_id,
        start_time,
        end_time,
    )
    .await
    .map_err(|_| "Greška pri proveri konflikta rezervacije.".to_string())?;

    if has_conflict {
        return Err("Izabrano parking mesto nije dostupno za taj vremenski interval.".to_string());
    }

    let reservation = reservation_repository::create_reservation(
        pool,
        user_id,
        request.vehicle_id,
        request.parking_spot_id,
        start_time,
        end_time,
    )
    .await
    .map_err(|_| "Greška pri kreiranju rezervacije.".to_string())?;

    Ok(ReservationResponse {
        id: reservation.id,
        user_id: reservation.user_id,
        vehicle_id: reservation.vehicle_id,
        parking_spot_id: reservation.parking_spot_id,
        start_time: reservation.start_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
        end_time: reservation.end_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
        status: reservation.status,
    })
}

pub async fn get_my_reservations(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<MyReservationResponse>, String> {
    let reservations = reservation_repository::get_my_reservations(pool, user_id)
        .await
        .map_err(|_| "Greška pri učitavanju rezervacija.".to_string())?;

    let response = reservations
        .into_iter()
        .map(|reservation| MyReservationResponse {
            id: reservation.id,
            parking_spot_id: reservation.parking_spot_id,
            parking_spot_label: reservation.parking_spot_label,
            zone: reservation.zone,
            start_time: reservation.start_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
            end_time: reservation.end_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
            status: reservation.status
        })
        .collect();

    Ok(response)
}

pub async fn cancel_reservation(
    pool: &PgPool,
    user_id: i32,
    reservation_id: Uuid,
) -> Result<String, String> {
    let reservation = reservation_repository::get_reservation_by_id(pool, reservation_id)
        .await
        .map_err(|_| "Greška pri učitavanju rezervacije.".to_string())?;

    let reservation = match reservation {
        Some(reservation) => reservation,
        None => return Err("Rezervacija ne postoji.".to_string()),
    };

    if reservation.user_id != user_id {
        return Err("Nemate pravo da otkažete ovu rezervaciju.".to_string());
    }

    if reservation.status == "Cancelled" {
        return Err("Rezervacija je već otkazana.".to_string());
    }

    let now = Utc::now().naive_utc();

    if reservation.start_time <= now {
        return Err("Rezervacija koja je već počela ne može se otkazati.".to_string());
    }

    reservation_repository::cancel_reservation(pool, reservation_id)
        .await
        .map_err(|_| "Greška pri otkazivanju rezervacije.".to_string())?;

    Ok("Rezervacija je uspešno otkazana.".to_string())
}



pub async fn get_valid_for_entry(
    pool: &PgPool,
    user_id: i32,
    vehicle_id: i32,
    time: NaiveDateTime,
) -> Result<Option<ValidForEntryResponse>, sqlx::Error> {
    let reservation = reservation_repository::find_valid_for_entry(pool, user_id, vehicle_id, time).await?;

    Ok(reservation.map(|r| ValidForEntryResponse {
        id: r.id,
        user_id: r.user_id,
        vehicle_id: r.vehicle_id,
        parking_spot_id: r.parking_spot_id,
        start_time: r.start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        end_time: r.end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        status: r.status,
    }))
}

pub async fn mark_used(
    pool: &PgPool,
    reservation_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let affected = reservation_repository::mark_used(pool, reservation_id).await?;
    Ok(affected > 0)
}

pub async fn expire_old(
    pool: &PgPool,
    now: NaiveDateTime,
) -> Result<u64, sqlx::Error> {
    reservation_repository::expire_old(pool, now).await
}