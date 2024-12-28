use serde::{Deserialize, Serialize};
use crate::types::{SubscriptionMessage, SubscriptionType, BirdeyeError};

#[derive(Debug, Deserialize)]
pub struct TokenListingData {
    pub address: String,
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub liquidity: String,
    #[serde(rename = "liquidityAddedAt")]
    pub liquidity_added_at: i64,
}

/// Options for token listing subscription
#[derive(Debug, Clone)]
pub struct TokenListingOptions {
    /// Whether to include listings from meme platforms (e.g., pump.fun)
    pub meme_platform_enabled: Option<bool>,
    /// Minimum liquidity requirement (must be > 10)
    pub min_liquidity: Option<f64>,
    /// Maximum liquidity requirement (must be > min_liquidity if provided)
    pub max_liquidity: Option<f64>,
}

impl Default for TokenListingOptions {
    fn default() -> Self {
        Self {
            meme_platform_enabled: None,
            min_liquidity: None,
            max_liquidity: None,
        }
    }
}

impl TokenListingOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_meme_platform(mut self, enabled: bool) -> Self {
        self.meme_platform_enabled = Some(enabled);
        self
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

/// Create a subscription for new token listings
pub fn create_token_listing_subscription(options: Option<TokenListingOptions>) -> SubscriptionMessage {
    let data = if let Some(opts) = options {
        let mut json_data = serde_json::Map::new();
        
        if let Some(meme_enabled) = opts.meme_platform_enabled {
            json_data.insert("meme_platform_enabled".to_string(), serde_json::json!(meme_enabled));
        }
        
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
        msg_type: SubscriptionType::SubscribeTokenNewListing,
        data,
    }
}

pub fn parse_token_listing_data(data: serde_json::Value) -> Result<TokenListingData, BirdeyeError> {
    Ok(serde_json::from_value(data)?)
} 