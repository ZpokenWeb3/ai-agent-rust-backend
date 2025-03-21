use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;

/// Represents a single price forecast.
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceForecast {
    /// The timestamp of the forecast.
    pub timestamp: String,
    /// The forecasted median price of the token.
    pub price: BigDecimal,
    /// The lower confidence bound of the forecast.
    pub lower_confidence_bound: BigDecimal,
    /// The upper confidence bound of the forecast.
    pub upper_confidence_bound: BigDecimal,
}

/// Represents the response for a price forecast request.
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceForecastResponse {
    /// The reasoning behind the forecast.
    pub reasoning: String,
    /// The list of forecasted prices.
    pub forecast: Vec<PriceForecast>,
}
