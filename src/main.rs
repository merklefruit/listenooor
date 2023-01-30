#![allow(unused)] // TODO: remove for release

use std::str::FromStr;

use crate::prelude::*;
use ethers::prelude::*;

mod alchemy;
mod error;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let url = f!(
        "wss://eth-mainnet.g.alchemy.com/v2/{}",
        std::env::var("ALCHEMY_API_KEY")?
    );

    let provider = alchemy::AlchemyWebSocketProvider::new(&url).await;

    let eth2_deposits_address =
        Address::from_str("0x00000000219ab540356cBB839Cbe05303d7705Fa").unwrap();
    let usd_coin_address = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
    let topics =
        vec!["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string()];

    let eth2_stream = provider
        .alchemy_subscribe_logs(usd_coin_address, Some(topics))
        .await?;

    // let eth2_stream = provider
    //     .alchemy_subscribe_pending_transactions(Some(usd_coin_address), None)
    //     .await?;

    let example = eth2_stream.take(1).collect::<Vec<_>>().await;
    println!("{:?}", example);

    Ok(())
}
