use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct VehicleResponse {
    pub id: i32,
    pub license_plate: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateVehicleRequest {
    pub license_plate: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateVehicleRequest {
    pub license_plate: String,
    pub name: Option<String>,
}
