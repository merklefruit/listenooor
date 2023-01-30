use ethers::prelude::*;
use serde::ser::Serialize;
use serde_json::{json, value::Value};
use std::sync::Arc;

use crate::prelude::*;

pub struct AlchemyWebSocketProvider {
    pub provider: Arc<Provider<Ws>>,
}

impl AlchemyWebSocketProvider {
    pub async fn new(url: &str) -> Self {
        let provider = Provider::connect(String::from(url))
            .await
            .expect("Error connecting to Alchemy WebSocket provider");

        Self {
            provider: Arc::new(provider),
        }
    }

    pub async fn alchemy_subscribe_pending_transactions<
        T: Into<Address> + Send + Sync + Serialize,
    >(
        &self,
        to: Option<T>,
        from: Option<T>,
    ) -> Result<SubscriptionStream<'_, Ws, Transaction>> {
        let mut param_map: serde_json::Map<String, Value> = serde_json::Map::new();

        if let Some(to) = to {
            param_map.insert(
                "toAddress".to_string(),
                Value::String(format!("{:?}", to.into())),
            );
        }
        if let Some(from) = from {
            param_map.insert(
                "fromAddress".to_string(),
                Value::String(format!("{:?}", from.into())),
            );
        }

        let params = json!(["alchemy_pendingTransactions", Value::Object(param_map)]);

        println!("Sending request with params: {}", params);

        let subscription_id: U256 = self.provider.request("eth_subscribe", params).await?;

        println!("Sent request with id: {:?}", subscription_id);

        SubscriptionStream::new(subscription_id, &self.provider).map_err(Into::into)
    }

    pub async fn alchemy_subscribe_logs<T: Into<Address> + Send + Sync + Serialize>(
        &self,
        address: T,
        topics: Option<Vec<String>>,
    ) -> Result<SubscriptionStream<'_, Ws, Log>> {
        let mut param_map: serde_json::Map<String, Value> = serde_json::Map::new();

        if let Some(topics) = topics {
            param_map.insert("topics".to_string(), json!(topics));
        }

        param_map.insert(
            "address".to_string(),
            Value::String(format!("{:?}", address.into())),
        );

        let params = json!(["logs", Value::Object(param_map)]);

        println!("Sending request with params: {}", params);

        let subscription_id: U256 = self.provider.request("eth_subscribe", params).await?;

        println!("Sent request with id: {:?}", subscription_id);

        SubscriptionStream::new(subscription_id, &self.provider).map_err(Into::into)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ethers::prelude::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_websocket_provider() -> Result<()> {
        dotenv::dotenv().ok();

        let url = f!(
            "wss://eth-mainnet.g.alchemy.com/v2/{}",
            std::env::var("ALCHEMY_API_KEY")?
        );

        let provider = AlchemyWebSocketProvider::new(&url).await;

        let usd_coin_address =
            Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();

        // Transfer event signature
        let topics =
            vec!["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string()];

        let usdc_transfers = provider
            .alchemy_subscribe_logs(usd_coin_address, Some(topics))
            .await?;

        let usdc_transactions = provider
            .alchemy_subscribe_pending_transactions(Some(usd_coin_address), None)
            .await?;

        let example_tx = usdc_transactions.take(1).collect::<Vec<_>>().await;
        println!("{:?}", example_tx);
        assert_eq!(example_tx[0].to.unwrap(), usd_coin_address);

        let example_log = usdc_transfers.take(1).collect::<Vec<_>>().await;
        println!("{:?}", example_log);
        assert_eq!(example_log[0].address, usd_coin_address);

        Ok(())
    }
}
