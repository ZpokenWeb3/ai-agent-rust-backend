pub mod models;
pub mod service;

pub use service::init_price_forecast_model;
pub use models::{PriceForecast, PriceForecastResponse};
