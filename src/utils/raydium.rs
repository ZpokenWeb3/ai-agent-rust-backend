use reqwest::Client;
use serde_json::Value;
use std::error::Error;

const RAYDIUM_API_URL: &str = "https://api-v3.raydium.io";
const WSOL_ADDRESS: &str = "So11111111111111111111111111111111111111112";

pub struct RaydiumClient {
    client: Client,
}

impl RaydiumClient {
    pub fn new_raydium_client() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_pool_quote_token_info(&self, pool_id: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/pools/info/ids?ids={}", RAYDIUM_API_URL, pool_id);
        let resp = self.client.get(&url).send().await?.json::<Value>().await?;

        if resp["success"].as_bool().unwrap_or(false) {
            let data = &resp["data"][0];
            let mint_address = data["mintB"]["address"].as_str().unwrap_or("");

            if mint_address != WSOL_ADDRESS {
                Ok(data["mintB"].clone())
            } else {
                Ok(data["mintA"].clone())
            }
        } else {
            Err(format!("Invalid pool_id or no data found for pool_id: {}", pool_id).into())
        }
    }

    pub async fn get_pool_info(&self, pool_id: &str) -> Result<(Value, bool), Box<dyn Error>> {
        let url = format!("{}/pools/info/ids?ids={}", RAYDIUM_API_URL, pool_id);
        let resp = self.client.get(&url).send().await?.json::<Value>().await?;

        if resp["success"].as_bool().unwrap_or(false) {
            let data = &resp["data"][0];
            let mint_address = data["mintB"]["address"].as_str().unwrap_or("");

            if mint_address != WSOL_ADDRESS {
                Ok((data.clone(), true))
            } else {
                Ok((data.clone(), false))
            }
        } else {
            Err(format!("Invalid pool_id or no data found for pool_id: {}", pool_id).into())
        }
    }
}
