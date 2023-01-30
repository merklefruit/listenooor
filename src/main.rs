#![allow(unused)] // TODO: remove for release

use std::str::FromStr;

use crate::prelude::*;
use ethers::prelude::*;

mod alchemy;
mod error;
mod listener;
mod prelude;
mod storage;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let url = f!(
        "wss://eth-mainnet.g.alchemy.com/v2/{}",
        std::env::var("ALCHEMY_API_KEY")?
    );

    let provider = alchemy::AlchemyWebSocketProvider::new(&url).await;
    let storage = storage::SqliteStorage::init(None);

    let usd_coin_address = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();

    // Transfer event signature
    let topics =
        vec!["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string()];

    let stream = provider
        .alchemy_subscribe_logs(usd_coin_address, Some(topics))
        .await?;

    let mut stream_listener = listener::StreamListener::new("usdc_transfers", stream, storage);

    stream_listener.listen().await?;

    Ok(())
}
