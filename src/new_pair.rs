use serde::{Deserialize, Serialize};
use crate::types::{SubscriptionMessage, SubscriptionType, BirdeyeError};

#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Deserialize)]
pub struct NewPairData {
    pub address: String,
    pub name: String,
    pub source: String,
    pub base: TokenInfo,
    pub quote: TokenInfo,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    #[serde(rename = "blockTime")]
    pub block_time: i64,
}

/// Options for new pair subscription
#[derive(Debug, Clone)]
pub struct NewPairOptions {
    /// Minimum liquidity requirement (must be > 10)
    pub min_liquidity: Option<f64>,
    /// Maximum liquidity requirement (must be > min_liquidity if provided)
    pub max_liquidity: Option<f64>,
}

impl Default for NewPairOptions {
    fn default() -> Self {
        Self {
            min_liquidity: None,
            max_liquidity: None,
        }
    }
}

impl NewPairOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_min_liquidity(mut self, min: f64) -> Self {
        assert!(min > 10.0, "min_liquidity must be greater than 10");
        self.min_liquidity = Some(min);
        self
    }

    pub fn with_max_liquidity(mut self, max: f64) -> Self {
        if let Some(min) = self.min_liquidity {
            assert!(max > min, "max_liquidity must be greater than min_liquidity");
        }
        self.max_liquidity = Some(max);
        self
    }
}

/// Create a subscription for new trading pairs
/// Note: DEX Openbook pairs are not supported
pub fn create_new_pair_subscription(options: Option<NewPairOptions>) -> SubscriptionMessage {
    let data = if let Some(opts) = options {
        let mut json_data = serde_json::Map::new();
        
        if let Some(min) = opts.min_liquidity {
            json_data.insert("min_liquidity".to_string(), serde_json::json!(min));
        }
        
        if let Some(max) = opts.max_liquidity {
            json_data.insert("max_liquidity".to_string(), serde_json::json!(max));
        }
        
        serde_json::Value::Object(json_data)
    } else {
        serde_json::json!({})
    };

    SubscriptionMessage {
        msg_type: SubscriptionType::SubscribeNewPair,
        data,
    }
}

pub fn parse_new_pair_data(data: serde_json::Value) -> Result<NewPairData, BirdeyeError> {
    Ok(serde_json::from_value(data)?)
} 