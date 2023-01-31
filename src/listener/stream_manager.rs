use std::sync::Arc;

use ethers::prelude::*;

use crate::alchemy::{helpers, AlchemyWebSocketProvider};
use crate::listener::StreamListener;
use crate::prelude::*;
use crate::storage::SqliteStorage;

pub struct StreamManager<'a> {
    storage: SqliteStorage,
    provider: AlchemyWebSocketProvider,
    streams: Vec<Arc<StreamListener<'a>>>,
}

impl<'a> StreamManager<'a> {
    pub async fn new(chain_id: usize, storage: SqliteStorage) -> StreamManager<'a> {
        let url = helpers::get_websocket_url(chain_id).unwrap();
        let provider = AlchemyWebSocketProvider::new(&url).await;
        let streams = Vec::new();

        Self {
            storage,
            provider,
            streams,
        }
    }

    pub async fn add_event_stream(
        &'static mut self,
        name: &str,
        address: Address,
        topics: Option<Vec<String>>,
    ) -> Result<()> {
        let stream = self
            .provider
            .alchemy_subscribe_logs(address, topics)
            .await?;

        let listener: Arc<StreamListener> =
            Arc::new(StreamListener::new(name, stream, self.storage.clone()));

        self.streams.push(listener);

        Ok(())
    }
}
