use crate::dto::user_dto::UserEmailResponse;

pub async fn get_user_email(user_id: i32) -> Result<String, String> {
    let user_service_url = std::env::var("USER_SERVICE_URL")
        .map_err(|_| "USER_SERVICE_URL nije setovan.".to_string())?;

    let url = format!("{}/internal/users/{}/email", user_service_url, user_id);

    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Greška pri pozivu User Service: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "User Service nije vratio uspešan odgovor. Status: {}",
            response.status()
        ));
    }
    let body = response
    .json::<UserEmailResponse>()
    .await
    .map_err(|_| "Greška pri parsiranju odgovora".to_string())?;

    Ok(body.email)
}