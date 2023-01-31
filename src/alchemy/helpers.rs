use crate::prelude::*;

pub fn get_websocket_url(chain_id: usize) -> Result<String> {
    let key = std::env::var("ALCHEMY_API_KEY")?;

    match chain_id {
        1 => Ok(f!("wss://eth-mainnet.g.alchemyapi.com/v2/{key}").to_string()),
        137 => Ok(f!("wss://polygon-mainnet.g.alchemy.com/v2/{key}").to_string()),
        42161 => Ok(f!("wss://arb-mainnet.g.alchemyapi.com/v2/{key}").to_string()),

        _ => Err(Error::Generic("Invalid chain_id provided.".to_string())),
    }
}
