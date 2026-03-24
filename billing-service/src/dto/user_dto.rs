use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEmailResponse {
    pub user_id: i32,
    pub email: String,
}