use chrono::{NaiveDateTime, Local};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::{EntryResponse, ActiveParkingSessionResponse, ExitResponse, ParkingHistoryItemResponse, ParkingHistoryResponse, GenerateInvoiceRequest},
    repository::{gate_repository, parking_session_repository},
    service::reservation_client
};


pub async fn enter_parking(
    pool: &PgPool,
    reservation_service_url: &str,
    user_id: i32,
    gate_id: uuid::Uuid,
    vehicle_id: i32,
) -> Result<EntryResponse, String> {

    println!("ENTRY user_id = {}", user_id);
println!("ENTRY vehicle_id = {}", vehicle_id);
println!("ENTRY gate_id = {}", gate_id);
    let gate = gate_repository::get_gate_by_id(pool, gate_id)
        .await
        .map_err(|_| "Greška prilikom učitavanja rampe.".to_string())?;

    let gate = gate.ok_or_else(|| "Rampa ne postoji.".to_string())?;

    if !gate.is_active {
        return Err("Rampa nije aktivna.".to_string());
    }

    if gate.gate_type != "ENTRY" {
        return Err("Izabrana rampa nije ulazna.".to_string());
    }

    let active_session = parking_session_repository::get_active_session_by_user_id(pool, user_id)
        .await
        .map_err(|_| "Greška prilikom provere aktivne sesije.".to_string())?;

    if active_session.is_some() {
        return Err("Korisnik već ima aktivnu parking sesiju.".to_string());
    }

    let now = Local::now().naive_local();
    println!("ENTRY now = {}", now);

    let reservation = reservation_client::get_valid_reservation_for_entry(
        reservation_service_url,
        user_id,
        vehicle_id,
        now,
    )
    .await
    .map_err(|_| "Greška prilikom provere rezervacije.".to_string())?;

    let reservation = reservation.ok_or_else(|| {
        "Ne postoji validna rezervacija za ovo vozilo u dozvoljenom vremenskom periodu."
            .to_string()
    })?;

    println!("Reservation response = {:?}", reservation);
    let planned_start_time =
        NaiveDateTime::parse_from_str(&reservation.start_time, "%Y-%m-%d %H:%M:%S")
            .or_else(|_| NaiveDateTime::parse_from_str(&reservation.start_time, "%Y-%m-%dT%H:%M:%S"))
            .map_err(|_| "Greška pri parsiranju start_time.".to_string())?;

    let planned_end_time =
        NaiveDateTime::parse_from_str(&reservation.end_time, "%Y-%m-%d %H:%M:%S")
            .or_else(|_| NaiveDateTime::parse_from_str(&reservation.end_time, "%Y-%m-%dT%H:%M:%S"))
            .map_err(|_| "Greška pri parsiranju end_time.".to_string())?;

    let session = parking_session_repository::create_parking_session(
        pool,
        reservation.id,
        user_id,
        vehicle_id,
        reservation.parking_spot_id,
        gate_id,
        now,
        planned_start_time,
        planned_end_time,
    )
    .await
    .map_err(|_| "Greška prilikom kreiranja parking sesije.".to_string())?;

    let marked = reservation_client::mark_reservation_used(
        reservation_service_url,
        reservation.id,
    )
    .await
    .map_err(|_| "Greška prilikom izmene statusa rezervacije.".to_string())?;

    if !marked {
        return Err("Rezervacija nije pronađena za označavanje kao Used.".to_string());
    }

    Ok(EntryResponse {
        message: "Ulazak evidentiran. Rampa otvorena.".to_string(),
        session_id: session.id,
        reservation_id: session.reservation_id,
        parking_spot_id: session.parking_spot_id,
        entry_time: session.entry_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        status: session.status,
    })
}


pub async fn get_active_session(
    pool: &PgPool,
    user_id: i32,
) -> Result<Option<ActiveParkingSessionResponse>, sqlx::Error> {
    let session = parking_session_repository::get_active_session_by_user_id(pool, user_id).await?;

    Ok(session.map(|s| ActiveParkingSessionResponse {
        session_id: s.id,
        reservation_id: s.reservation_id,
        vehicle_id: s.vehicle_id,
        parking_spot_id: s.parking_spot_id,
        entry_time: s.entry_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        planned_start_time: s.planned_start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        planned_end_time: s.planned_end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        status: s.status,
    }))
}

pub async fn exit_parking(
    pool: &PgPool,
    user_id: i32,
    gate_id: Uuid,
) -> Result<ExitResponse, String> {
    let gate = gate_repository::get_gate_by_id(pool, gate_id)
        .await
        .map_err(|_| "Greška prilikom učitavanja rampe.".to_string())?;

    let gate = gate.ok_or_else(|| "Rampa ne postoji.".to_string())?;

    if !gate.is_active {
        return Err("Rampa nije aktivna.".to_string());
    }

    if gate.gate_type != "EXIT" {
        return Err("Izabrana rampa nije izlazna.".to_string());
    }

    let active_session = parking_session_repository::get_active_session_by_user_id(pool, user_id)
        .await
        .map_err(|_| "Greška prilikom provere aktivne sesije.".to_string())?;

    let active_session = active_session
        .ok_or_else(|| "Korisnik nema aktivnu parking sesiju.".to_string())?;

    let exit_time = Local::now().naive_local();

    let completed_session = parking_session_repository::complete_active_session(
        pool,
        active_session.id,
        gate_id,
        exit_time,
    )
    .await
    .map_err(|_| "Greška prilikom evidentiranja izlaska.".to_string())?;

    let duration_minutes =
        (completed_session.exit_time.unwrap() - completed_session.entry_time).num_minutes();

    let billable_hours = ((duration_minutes + 59) / 60).max(1);

    let price_per_hour = 100_i64;
    let total_amount = billable_hours * price_per_hour;

    // 🔥 POZIV BILLING SERVISA
    let client = reqwest::Client::new();

    let request = GenerateInvoiceRequest {
        session_id: completed_session.id,
        user_id: completed_session.user_id,
        reservation_id: Some(completed_session.reservation_id),
        start_time: completed_session
            .entry_time
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string(),
        end_time: completed_session
            .exit_time
            .unwrap()
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string(),
        price_per_hour: price_per_hour as f64,
    };

    let _ = client
        .post("http://billing_service:8084/invoices/generate")
        .json(&request)
        .send()
        .await
        .map_err(|_| "Greška pri pozivu billing servisa.".to_string())?;

    Ok(ExitResponse {
        message: "Izlazak evidentiran.".to_string(),
        session_id: completed_session.id,
        entry_time: completed_session
            .entry_time
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
        exit_time: completed_session
            .exit_time
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
        duration_minutes,
        billable_hours,
        total_amount,
    })
}

pub async fn get_parking_history(
    pool: &PgPool,
    user_id: i32,
    status: Option<String>,
) -> Result<ParkingHistoryResponse, sqlx::Error> {
    let sessions = match status {
        Some(ref status_value) if !status_value.trim().is_empty() => {
            parking_session_repository::get_history_by_user_id_and_status(
                pool,
                user_id,
                status_value,
            )
            .await?
        }
        _ => parking_session_repository::get_history_by_user_id(pool, user_id).await?,
    };

    let items = sessions
        .into_iter()
        .map(|s| ParkingHistoryItemResponse {
            session_id: s.id,
            reservation_id: s.reservation_id,
            vehicle_id: s.vehicle_id,
            parking_spot_id: s.parking_spot_id,
            entry_time: s.entry_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            exit_time: s
                .exit_time
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            planned_start_time: s.planned_start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            planned_end_time: s.planned_end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            status: s.status,
        })
        .collect();

    Ok(ParkingHistoryResponse { items })
}