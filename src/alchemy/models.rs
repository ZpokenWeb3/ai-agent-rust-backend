use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;

/// Represents a historical price data point.
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalPrice {
    pub value: BigDecimal,
    pub timestamp: DateTime<Utc>,
}

/// Represents historical price data for a token symbol.
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalPriceBySymbol {
    pub symbol: String,
    pub data: Vec<HistoricalPrice>,
}

/// Represents historical price data for a token address.
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalPriceByAddress {
    pub address: String,
    pub network: String,
    pub data: Vec<HistoricalPrice>,
}

/// Represents metadata of a blockchain transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
}

/// Represents a raw blockchain contract structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct RawContract {
    pub address: String,
    pub value: i64,
    pub decimal: i32,
}

/// Represents a blockchain token transfer.
#[derive(Debug, Serialize, Deserialize)]
pub struct Transfer {
    pub block_number: i64,
    pub tx_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub value: BigDecimal,
    pub raw_contract: RawContract,
    pub metadata: Metadata,
    pub asset: String,
    pub category: String,
}

/// Represents a token balance.
#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub contract_address: String,
    pub value: i64,
    pub error: Option<String>,
}
