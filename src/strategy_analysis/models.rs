use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Represents an alert triggered by the strategy.
#[derive(Debug, Serialize, Deserialize)]
pub struct AlertItem {
    /// Token metadata including symbol, address, chain, etc.
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Description of the rule that was triggered.
    pub rule_description: String,
    /// The measured value related to this alert.
    pub value: f64,
    /// Additional context about why the rule was triggered.
    pub supporting_data: Option<HashMap<String, serde_json::Value>>,
}

/// Represents the result of the strategy analysis.
#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyAnalysis {
    /// A concise summary of the overall analysis.
    pub summary: String,
    /// List of triggered rules and their details.
    pub alerts: Vec<AlertItem>,
}
