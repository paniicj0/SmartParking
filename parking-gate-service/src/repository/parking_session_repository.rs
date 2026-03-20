use chrono::NaiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::parking_session::ParkingSession;

pub async fn get_active_session_by_user_id(
    pool: &PgPool,
    user_id: i32,
) -> Result<Option<ParkingSession>, sqlx::Error> {
    let session = sqlx::query_as::<_, ParkingSession>(
        r#"
        SELECT
            id,
            reservation_id,
            user_id,
            vehicle_id,
            parking_spot_id,
            entry_gate_id,
            exit_gate_id,
            entry_time,
            exit_time,
            status,
            planned_start_time,
            planned_end_time,
            created_at,
            updated_at
        FROM parking_sessions
        WHERE user_id = $1
          AND status = 'Active'
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(session)
}

pub async fn create_parking_session(
    pool: &PgPool,
    reservation_id: Uuid,
    user_id: i32,
    vehicle_id: i32,
    parking_spot_id: Uuid,
    entry_gate_id: Uuid,
    entry_time: NaiveDateTime,
    planned_start_time: NaiveDateTime,
    planned_end_time: NaiveDateTime,
) -> Result<ParkingSession, sqlx::Error> {
    let session = sqlx::query_as::<_, ParkingSession>(
        r#"
        INSERT INTO parking_sessions (
            id,
            reservation_id,
            user_id,
            vehicle_id,
            parking_spot_id,
            entry_gate_id,
            exit_gate_id,
            entry_time,
            exit_time,
            status,
            planned_start_time,
            planned_end_time,
            created_at,
            updated_at
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, NULL, $7, NULL, $8, $9, $10, NOW(), NOW()
        )
        RETURNING
            id,
            reservation_id,
            user_id,
            vehicle_id,
            parking_spot_id,
            entry_gate_id,
            exit_gate_id,
            entry_time,
            exit_time,
            status,
            planned_start_time,
            planned_end_time,
            created_at,
            updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(reservation_id)
    .bind(user_id)
    .bind(vehicle_id)
    .bind(parking_spot_id)
    .bind(entry_gate_id)
    .bind(entry_time)
    .bind("Active")
    .bind(planned_start_time)
    .bind(planned_end_time)
    .fetch_one(pool)
    .await?;

    Ok(session)
}

pub async fn complete_active_session(
    pool: &PgPool,
    session_id: Uuid,
    exit_gate_id: Uuid,
    exit_time: NaiveDateTime,
) -> Result<ParkingSession, sqlx::Error> {
    let session = sqlx::query_as::<_, ParkingSession>(
        r#"
        UPDATE parking_sessions
        SET exit_gate_id = $2,
            exit_time = $3,
            status = 'Completed',
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            reservation_id,
            user_id,
            vehicle_id,
            parking_spot_id,
            entry_gate_id,
            exit_gate_id,
            entry_time,
            exit_time,
            status,
            planned_start_time,
            planned_end_time,
            created_at,
            updated_at
        "#,
    )
    .bind(session_id)
    .bind(exit_gate_id)
    .bind(exit_time)
    .fetch_one(pool)
    .await?;

    Ok(session)
}

pub async fn get_history_by_user_id(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<ParkingSession>, sqlx::Error> {
    let sessions = sqlx::query_as::<_, ParkingSession>(
        r#"
        SELECT
            id,
            reservation_id,
            user_id,
            vehicle_id,
            parking_spot_id,
            entry_gate_id,
            exit_gate_id,
            entry_time,
            exit_time,
            status,
            planned_start_time,
            planned_end_time,
            created_at,
            updated_at
        FROM parking_sessions
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(sessions)
}

pub async fn get_history_by_user_id_and_status(
    pool: &PgPool,
    user_id: i32,
    status: &str,
) -> Result<Vec<ParkingSession>, sqlx::Error> {
    let sessions = sqlx::query_as::<_, ParkingSession>(
        r#"
        SELECT
            id,
            reservation_id,
            user_id,
            vehicle_id,
            parking_spot_id,
            entry_gate_id,
            exit_gate_id,
            entry_time,
            exit_time,
            status,
            planned_start_time,
            planned_end_time,
            created_at,
            updated_at
        FROM parking_sessions
        WHERE user_id = $1
          AND status = $2
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .bind(status)
    .fetch_all(pool)
    .await?;

    Ok(sessions)
}