use sqlx::PgPool;
use uuid::Uuid;

use crate::models::gate::Gate;

pub async fn get_gate_by_id(
    pool: &PgPool,
    gate_id: Uuid,
) -> Result<Option<Gate>, sqlx::Error> {
    let gate = sqlx::query_as::<_, Gate>(
        r#"
        SELECT id, name, gate_type, qr_value, is_active, created_at
        FROM gates
        WHERE id = $1
        "#,
    )
    .bind(gate_id)
    .fetch_optional(pool)
    .await?;

    Ok(gate)
}