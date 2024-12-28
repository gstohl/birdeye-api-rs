use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use tokio_tungstenite::tungstenite::http::{Request, HeaderValue};
use tokio::net::TcpStream;
use url::Url;
use crate::types::BirdeyeError;

pub struct BirdeyeWebSocket {
    api_key: String,
}

impl BirdeyeWebSocket {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    pub async fn connect(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, BirdeyeError> {
        println!("Connecting to Birdeye WebSocket...");
        let url = format!(
            "wss://public-api.birdeye.so/socket/solana?x-api-key={}",
            self.api_key
        );
        let url = Url::parse(&url)?;

        let request = Request::builder()
            .uri(url.as_str())
            .header("Host", "public-api.birdeye.so")
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Origin", "ws://public-api.birdeye.so")
            .header("Sec-WebSocket-Origin", "ws://public-api.birdeye.so")
            .header("Sec-WebSocket-Protocol", "echo-protocol")
            .header("Sec-WebSocket-Key", tokio_tungstenite::tungstenite::handshake::client::generate_key())
            .body(())
            .unwrap();
        let (ws_stream, response) = connect_async(request).await?;
        Ok(ws_stream)
    }

    pub fn parse_response(text: &str) -> Result<crate::types::WebSocketResponse, BirdeyeError> {
        Ok(serde_json::from_str(text)?)
    }
} 