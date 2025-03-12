use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct DexScreenerResponse {
    success: bool,
    data: Vec<DexScreenerToken>,
}

#[derive(Deserialize, Debug)]
pub struct DexScreenerToken {
    symbol: String,
    address: String,
    price_native: String,
}

pub async fn fetch_dexscreener_data(token_address: &str) -> Result<DexScreenerResponse, Box<dyn Error>> {
    let url = format!("https://api.dexscreener.com/tokens/v1/solana/{}", token_address);
    let response: DexScreenerResponse = reqwest::get(&url).await?.json().await?;
    Ok(response)
}
