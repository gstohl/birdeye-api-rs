use serde::{Deserialize, Serialize};
use crate::types::{SubscriptionMessage, SubscriptionType};

#[derive(Debug, Deserialize)]
pub struct BaseQuotePriceData {
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
    #[serde(rename = "baseAddress")]
    pub base_address: String,
    #[serde(rename = "quoteAddress")]
    pub quote_address: String,
}

/// Chart type intervals for base-quote price data
#[derive(Debug, Clone, Copy)]
pub enum ChartType {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    FourHours,
    OneDay,
    OneWeek,
}

impl ChartType {
    fn as_str(&self) -> &'static str {
        match self {
            ChartType::OneMinute => "1m",
            ChartType::ThreeMinutes => "3m",
            ChartType::FiveMinutes => "5m",
            ChartType::FifteenMinutes => "15m",
            ChartType::ThirtyMinutes => "30m",
            ChartType::OneHour => "1h",
            ChartType::FourHours => "4h",
            ChartType::OneDay => "1d",
            ChartType::OneWeek => "1w",
        }
    }
}

/// Create a subscription for base-quote price updates
/// Note: Only one base-quote pair is supported per WebSocket connection
pub fn create_base_quote_subscription(
    base_address: impl Into<String>,
    quote_address: impl Into<String>,
    chart_type: ChartType,
) -> SubscriptionMessage {
    SubscriptionMessage {
        msg_type: SubscriptionType::SubscribeBaseQuotePrice,
        data: serde_json::json!({
            "baseAddress": base_address.into(),
            "quoteAddress": quote_address.into(),
            "chartType": chart_type.as_str(),
        }),
    }
}

pub fn parse_base_quote_price_data(data: serde_json::Value) -> Result<BaseQuotePriceData, crate::types::BirdeyeError> {
    Ok(serde_json::from_value(data)?)
} 