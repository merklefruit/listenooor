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
