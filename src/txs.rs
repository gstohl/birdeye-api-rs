use serde::{Deserialize, Serialize};
use crate::types::{SubscriptionMessage, SubscriptionType, BirdeyeError};

#[derive(Debug, Deserialize)]
pub struct TokenTransferInfo {
    pub symbol: String,
    pub decimals: u8,
    pub address: String,
    pub amount: i64,
    #[serde(rename = "type")]
    pub transfer_type: String,
    pub typeSwap: String,
    #[serde(rename = "uiAmount")]
    pub ui_amount: f64,
    pub price: Option<f64>,
    #[serde(rename = "nearestPrice")]
    pub nearest_price: Option<f64>,
    #[serde(rename = "changeAmount")]
    pub change_amount: i64,
    #[serde(rename = "uiChangeAmount")]
    pub ui_change_amount: f64,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionData {
    #[serde(rename = "blockUnixTime")]
    pub block_unix_time: i64,
    pub owner: String,
    pub source: String,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub alias: Option<String>,
    #[serde(rename = "isTradeOnBe")]
    pub is_trade_on_be: bool,
    pub platform: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: f64,
    pub from: TokenTransferInfo,
    pub to: TokenTransferInfo,
}

/// Create a subscription for a single token's transactions
pub fn create_token_txs_subscription(token_address: impl Into<String>) -> SubscriptionMessage {
    SubscriptionMessage {
        msg_type: SubscriptionType::SubscribeTxs,
        data: serde_json::json!({
            "queryType": "simple",
            "address": token_address.into(),
        }),
    }
}

/// Create a subscription for a specific trading pair's transactions
pub fn create_pair_txs_subscription(pair_address: impl Into<String>) -> SubscriptionMessage {
    SubscriptionMessage {
        msg_type: SubscriptionType::SubscribeTxs,
        data: serde_json::json!({
            "queryType": "simple",
            "pairAddress": pair_address.into(),
        }),
    }
}

/// Create a subscription for multiple tokens and/or pairs (limit 100)
pub fn create_multi_txs_subscription(token_addresses: Vec<String>, pair_addresses: Vec<String>) -> SubscriptionMessage {
    let token_conditions = token_addresses
        .into_iter()
        .map(|addr| format!("address = {}", addr));
    
    let pair_conditions = pair_addresses
        .into_iter()
        .map(|addr| format!("pairAddress = {}", addr));
    
    let query = token_conditions
        .chain(pair_conditions)
        .collect::<Vec<_>>()
        .join(" OR ");

    SubscriptionMessage {
        msg_type: SubscriptionType::SubscribeTxs,
        data: serde_json::json!({
            "queryType": "complex",
            "query": query,
        }),
    }
}

pub fn parse_transaction_data(data: serde_json::Value) -> Result<TransactionData, crate::types::BirdeyeError> {
    Ok(serde_json::from_value(data)?)
} 