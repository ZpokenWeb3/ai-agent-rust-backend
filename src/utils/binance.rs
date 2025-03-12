use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct BinancePrice {
    price: String,
}

pub async fn fetch_solana_price_binance() -> Result<f64, Box<dyn Error>> {
    let url = "https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT";
    let response: BinancePrice = reqwest::get(url).await?.json().await?;
    Ok(response.price.parse::<f64>()?)
}
