use serde::{Deserialize, Serialize};
use crate::types::BirdeyeError;

const API_BASE_URL: &str = "https://public-api.birdeye.so";

#[derive(Debug, Clone)]
pub struct BirdeyeRest {
    api_key: String,
    chain: String,
}

impl BirdeyeRest {
    pub fn new(api_key: impl Into<String>, chain: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            chain: chain.into(),
        }
    }

    /// Fetch OHLCV data for a token
    /// 
    /// # Arguments
    /// * `address` - Token address
    /// * `interval_type` - OHLCV interval type (e.g., "15m", "1h", "1d")
    /// * `time_from` - Start timestamp (Unix)
    /// * `time_to` - End timestamp (Unix)
    pub async fn get_ohlcv(
        &self,
        address: &str,
        interval_type: &str,
        time_from: i64,
        time_to: i64,
    ) -> Result<OHLCVResponse, BirdeyeError> {
        let url = format!(
            "{}/defi/ohlcv?address={}&type={}&time_from={}&time_to={}",
            API_BASE_URL, address, interval_type, time_from, time_to
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("accept", "application/json")
            .header("x-chain", &self.chain)
            .header("X-API-KEY", &self.api_key)
            .send()
            .await?;

        let data = response.json().await?;
        Ok(data)
    }

    /// Get token overview
    /// 
    /// # Arguments
    /// * `address` - Token address
    pub async fn get_token_overview(&self, address: &str) -> Result<TokenOverviewResponse, BirdeyeError> {
        let url = format!("{}/defi/token_overview?address={}", API_BASE_URL, address);

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("accept", "application/json")
            .header("x-chain", &self.chain)
            .header("X-API-KEY", &self.api_key)
            .send()
            .await?;

        let data = response.json().await?;
        Ok(data)
    }
}

#[derive(Debug, Deserialize)]
pub struct OHLCVData {
    pub address: String,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "o")]
    pub open: f64,
    pub r#type: String,
    #[serde(rename = "unixTime")]
    pub unix_time: i64,
    #[serde(rename = "v")]
    pub volume: f64,
}

#[derive(Debug, Deserialize)]
pub struct OHLCVResponse {
    pub success: bool,
    pub data: OHLCVResponseData,
}

#[derive(Debug, Deserialize)]
pub struct OHLCVResponseData {
    pub items: Vec<OHLCVData>,
}

#[derive(Debug, Deserialize)]
pub struct TokenExtensions {
    #[serde(rename = "coingeckoId")]
    pub coingecko_id: Option<String>,
    #[serde(rename = "serumV3Usdc")]
    pub serum_v3_usdc: Option<String>,
    #[serde(rename = "serumV3Usdt")]
    pub serum_v3_usdt: Option<String>,
    pub website: Option<String>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub description: Option<String>,
    pub discord: Option<String>,
    pub medium: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenOverviewData {
    pub address: String,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
    pub extensions: TokenExtensions,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
    pub liquidity: f64,
    pub price: f64,
    pub supply: f64,
    pub mc: f64,
    #[serde(rename = "lastTradeUnixTime")]
    pub last_trade_unix_time: i64,
    #[serde(rename = "lastTradeHumanTime")]
    pub last_trade_human_time: String,
    // Add other fields as needed
}

#[derive(Debug, Deserialize)]
pub struct TokenOverviewResponse {
    pub success: bool,
    pub data: TokenOverviewData,
} 