use reqwest::Client;
use serde_json::json;
use chrono::{DateTime, Utc};
use crate::alchemy::models::{HistoricalPriceBySymbol, HistoricalPriceByAddress, Transfer, Balance};

/// Alchemy API client for fetching blockchain data.
pub struct AlchemyClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl AlchemyClient {
    /// Initializes the Alchemy client with an API key.
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.g.alchemy.com".to_string(),
            api_key,
        }
    }

    /// Fetch historical price data by symbol.
    pub async fn get_historical_prices_by_symbol(
        &self, symbol: &str, start_time: DateTime<Utc>, end_time: DateTime<Utc>, interval: &str
    ) -> Result<HistoricalPriceBySymbol, reqwest::Error> {
        let url = format!("{}/prices/v1/{}/tokens/historical", self.base_url, self.api_key);
        let body = json!({
            "symbol": symbol,
            "startTime": start_time.to_rfc3339(),
            "endTime": end_time.to_rfc3339(),
            "interval": interval
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let data = response.json::<HistoricalPriceBySymbol>().await?;
        Ok(data)
    }

    /// Fetch historical price data by address.
    pub async fn get_historical_prices_by_address(
        &self, address: &str, network: &str, start_time: DateTime<Utc>, end_time: DateTime<Utc>, interval: &str
    ) -> Result<HistoricalPriceByAddress, reqwest::Error> {
        let url = format!("{}/prices/v1/{}/tokens/historical", self.base_url, self.api_key);
        let body = json!({
            "address": address,
            "network": network,
            "startTime": start_time.to_rfc3339(),
            "endTime": end_time.to_rfc3339(),
            "interval": interval
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let data = response.json::<HistoricalPriceByAddress>().await?;
        Ok(data)
    }

    /// Fetch token transfers for a given wallet.
    pub async fn get_transfers(&self, wallet: &str, chain: &str, incoming: bool) -> Result<Vec<Transfer>, reqwest::Error> {
        let address_key = if incoming { "toAddress" } else { "fromAddress" };
        let url = format!("{}/v2/{}/alchemy_getAssetTransfers", self.base_url, self.api_key);

        let body = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "alchemy_getAssetTransfers",
            "params": [{
                "fromBlock": "0x0",
                "toBlock": "latest",
                address_key: wallet,
                "category": ["erc20"],
                "order": "asc",
                "withMetadata": true,
                "excludeZeroValue": true,
                "maxCount": "0x3e8",
            }]
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let data = response.json::<Vec<Transfer>>().await?;
        Ok(data)
    }

    /// Fetch token balances for a given wallet.
    pub async fn get_token_balances(&self, wallet: &str, chain: &str) -> Result<Vec<Balance>, reqwest::Error> {
        let url = format!("{}/v2/{}/alchemy_getTokenBalances", self.base_url, self.api_key);
        let body = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "alchemy_getTokenBalances",
            "params": [wallet],
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let data = response.json::<Vec<Balance>>().await?;
        Ok(data)
    }
}

// usage example
// mod alchemy;

// use alchemy::AlchemyClient;
// use alchemy::describe_alchemy_api;

// async fn some_service_function() {
//     let client = AlchemyClient::new("your_api_key".to_string());
//     let description = describe_alchemy_api().await;

//     println!("Alchemy API Description: {}", description);
// }
