use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_tungstenite::tungstenite::http::header::InvalidHeaderValue;

#[derive(Debug, thiserror::Error)]
pub enum BirdeyeError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionType {
    SubscribePrice,
    SubscribeTxs,
    SubscribeTokenNewListing,
    SubscribeNewPair,
    SubscribeWalletTxs,
    SubscribeBaseQuotePrice,
    SubscribeLargeTradeTxs,
    UnsubscribePrice,
    UnsubscribeTxs,
    UnsubscribeTokenNewListing,
    UnsubscribeNewPair,
    UnsubscribeWalletTxs,
    UnsubscribeBaseQuotePrice,
    UnsubscribeLargeTradeTxs,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseType {
    PriceData,
    TxsData,
    TokenNewListing,
    NewPair,
    WalletTxsData,
    BaseQuotePriceData,
    TxsLargeTradeData,
    Error,
}

#[derive(Debug, Serialize)]
pub struct SubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: SubscriptionType,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct WebSocketResponse {
    #[serde(rename = "type")]
    pub response_type: ResponseType,
    pub data: serde_json::Value,
} 