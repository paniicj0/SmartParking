use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct UserProfileResponse {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserEmailResponse {
    pub user_id: i32,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}