use sonic_rs::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct PairStats {
    pub std_dev: f32,
    pub mean: f32,
    pub most_recent_price: f32,
}

pub(crate) struct DataStore {
    pub price_history: HashMap<String, Vec<f32>>,
    pub stats: HashMap<String, PairStats>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct SymbolPricePairAPI {
    pub pair: String,
    pub price: String,
    percentChange24h: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Alert {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub log_level: String,
    pub trading_pair: Option<String>,
    pub is_deviation: bool,
    pub data: AlertMetaData,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AlertMetaData {
    pub error_message: Option<String>,
    pub last_price: Option<f32>,
    pub avg_price: Option<f32>,
    pub deviation: Option<f32>,
    pub price_change: Option<f32>,
}
