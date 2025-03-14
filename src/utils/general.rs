use uuid::Uuid;
use std::error::Error;
use reqwest;

pub fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

pub async fn check_users_retwitt() -> Result<bool, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.agent.zpoken.dev/api/v1/general/check_retwitts")
        .send()
        .await?;

    Ok(response.status().is_success())
}
