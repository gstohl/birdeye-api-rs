use serde::{Deserialize, Serialize};
use crate::types::{SubscriptionMessage, SubscriptionType, BirdeyeError};

#[derive(Debug, Deserialize)]
pub struct PriceData {
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
    pub v: f64,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "type")]
    pub chart_type: String,
    #[serde(rename = "unixTime")]
    pub unix_time: i64,
    pub symbol: String,
    pub address: String,
}

pub fn create_price_subscription(address: impl Into<String>, chart_type: impl Into<String>, currency: impl Into<String>) -> SubscriptionMessage {
    SubscriptionMessage {
        msg_type: SubscriptionType::SubscribePrice,
        data: serde_json::json!({
            "queryType": "simple",
            "chartType": chart_type.into(),
            "address": address.into(),
            "currency": currency.into(),
        }),
    }
}

pub fn create_multi_price_subscription(queries: Vec<(String, String, String)>) -> SubscriptionMessage {
    let query = queries
        .into_iter()
        .map(|(address, chart_type, currency)| {
            format!(
                "(address = {} AND chartType = {} AND currency = {})",
                address, chart_type, currency
            )
        })
        .collect::<Vec<_>>()
        .join(" OR ");

    SubscriptionMessage {
        msg_type: SubscriptionType::SubscribePrice,
        data: serde_json::json!({
            "queryType": "complex",
            "query": query,
        }),
    }
}

pub fn parse_price_data(data: serde_json::Value) -> Result<PriceData, BirdeyeError> {
    Ok(serde_json::from_value(data)?)
} 