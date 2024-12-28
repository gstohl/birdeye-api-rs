use serde::{Deserialize, Serialize};
use crate::types::{SubscriptionMessage, SubscriptionType, BirdeyeError};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletTokenInfo {
    pub symbol: String,
    pub decimals: u8,
    pub address: String,
    #[serde(rename = "uiAmount")]
    pub ui_amount: f64,
    #[serde(rename = "amount")]
    pub amount_raw: Value,
    pub price: Option<f64>,
    #[serde(rename = "nearestPrice")]
    pub nearest_price: Option<f64>,
    #[serde(rename = "uiChangeAmount")]
    pub ui_change_amount: f64,
}

impl WalletTokenInfo {
    pub fn amount(&self) -> String {
        match &self.amount_raw {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            _ => "0".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletTxData {
    #[serde(rename = "type")]
    pub tx_type: String,
    #[serde(rename = "blockUnixTime")]
    pub block_unix_time: i64,
    #[serde(rename = "blockHumanTime")]
    pub block_human_time: String,
    pub owner: String,
    pub source: String,
    #[serde(rename = "poolAddress")]
    pub pool_address: Option<String>,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: f64,
    pub network: String,
    #[serde(flatten)]
    pub extra_fields: Value,
    // These fields might not always be present depending on tx_type
    pub from: Option<WalletTokenInfo>,
    pub to: Option<WalletTokenInfo>,
}

/// Create a subscription for wallet transactions
/// 
/// # Arguments
/// * `address` - The wallet address to monitor:
///   - For EVM chains (Ethereum, BSC): Can be in any case (lowercase, uppercase, checksum)
///   - For Solana: Must be a valid Solana address format
/// 
/// # Example
/// ```rust
/// // For Ethereum/BSC
/// let sub = create_wallet_txs_subscription("0xae2Fc483527B8EF99EB5D9B44875F005ba1FaE13");
/// 
/// // For Solana
/// let sub = create_wallet_txs_subscription("9SeRj4LjgENeKQujfxRNkGbXYPM3X2vr9C37Jg9AARfg");
/// ```
pub fn create_wallet_txs_subscription(address: impl Into<String>) -> SubscriptionMessage {
    SubscriptionMessage {
        msg_type: SubscriptionType::SubscribeWalletTxs,
        data: serde_json::json!({
            "address": address.into(),
        }),
    }
}

/// Create an unsubscription message for wallet transactions
pub fn create_wallet_txs_unsubscription() -> SubscriptionMessage {
    SubscriptionMessage {
        msg_type: SubscriptionType::UnsubscribeWalletTxs,
        data: serde_json::json!({}),
    }
}

pub fn parse_wallet_txs_data(data: serde_json::Value) -> Result<WalletTxData, BirdeyeError> {
    Ok(serde_json::from_value(data)?)
} 