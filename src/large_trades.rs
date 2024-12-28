use serde::{Deserialize, Serialize};
use crate::types::{SubscriptionMessage, SubscriptionType, BirdeyeError};

#[derive(Debug, Deserialize)]
pub struct TradeTokenInfo {
    pub symbol: String,
    pub decimals: u8,
    pub address: String,
    #[serde(rename = "uiAmount")]
    pub ui_amount: f64,
    pub price: Option<f64>,
    #[serde(rename = "nearestPrice")]
    pub nearest_price: Option<f64>,
    #[serde(rename = "uiChangeAmount")]
    pub ui_change_amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct LargeTradeData {
    #[serde(rename = "blockUnixTime")]
    pub block_unix_time: i64,
    #[serde(rename = "blockHumanTime")]
    pub block_human_time: String,
    pub owner: String,
    pub source: String,
    #[serde(rename = "poolAddress")]
    pub pool_address: String,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: f64,
    pub network: String,
    pub from: TradeTokenInfo,
    pub to: TradeTokenInfo,
}

/// Options for large trades subscription
#[derive(Debug, Clone)]
pub struct LargeTradeOptions {
    /// Minimum volume requirement in USD (must be >= 1000)
    pub min_volume: f64,
    /// Maximum volume requirement (must be > min_volume if provided)
    pub max_volume: Option<f64>,
}

impl LargeTradeOptions {
    /// Create new options with the minimum volume requirement
    /// Note: min_volume must be >= 1000 USD
    pub fn new(min_volume: f64) -> Self {
        assert!(min_volume >= 1000.0, "min_volume must be at least 1000 USD");
        Self {
            min_volume,
            max_volume: None,
        }
    }

    pub fn with_max_volume(mut self, max_volume: f64) -> Self {
        assert!(max_volume > self.min_volume, "max_volume must be greater than min_volume");
        self.max_volume = Some(max_volume);
        self
    }
}

/// Create a subscription for large trades
/// Note: min_volume must be at least 1000 USD
pub fn create_large_trades_subscription(options: LargeTradeOptions) -> SubscriptionMessage {
    let mut json_data = serde_json::Map::new();
    json_data.insert("min_volume".to_string(), serde_json::json!(options.min_volume));
    
    if let Some(max) = options.max_volume {
        json_data.insert("max_volume".to_string(), serde_json::json!(max));
    }

    SubscriptionMessage {
        msg_type: SubscriptionType::SubscribeLargeTradeTxs,
        data: serde_json::Value::Object(json_data),
    }
}

pub fn parse_large_trade_data(data: serde_json::Value) -> Result<LargeTradeData, BirdeyeError> {
    Ok(serde_json::from_value(data)?)
} 