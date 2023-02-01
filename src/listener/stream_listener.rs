use ethers::prelude::*;
use std::sync::Arc;

use crate::{prelude::*, storage::SqliteStorage};

pub struct StreamListener<'a> {
    name: String,
    stream: SubscriptionStream<'a, Ws, Log>,
    storage: SqliteStorage,
}

unsafe impl<'a> Send for StreamListener<'a> {}
unsafe impl<'a> Sync for StreamListener<'a> {}

impl<'a> StreamListener<'a> {
    pub fn new(
        name: &str,
        stream: SubscriptionStream<'a, Ws, Log>,
        storage: SqliteStorage,
    ) -> Self {
        let name = name.to_string();
        let init_statement = create_init_statement(&name);

        storage.run_query(&init_statement).unwrap();

        Self {
            name,
            stream,
            storage,
        }
    }

    pub async fn listen(&mut self) -> Result<()> {
        while let Some(log) = self.stream.next().await {
            println!("Received log: {:?}", log);
            self.add_log_to_storage(log).unwrap();
        }

        println!("Stream ended");
        Ok(())
    }

    pub async fn stop_listening(&self) -> Result<()> {
        self.stream.unsubscribe().await?;
        Ok(())
    }

    fn add_log_to_storage(&self, log: Log) -> Result<()> {
        let insert_statement = insert_log_statement(&self.name, &log);
        self.storage.run_query(&insert_statement)?;
        Ok(())
    }
}

fn create_init_statement(name: &str) -> String {
    f!(
        "CREATE TABLE IF NOT EXISTS logs_{name} (
            block_number INTEGER,
            block_hash TEXT,
            transaction_hash TEXT,
            transaction_index INTEGER,
            log_index INTEGER,
            address TEXT,
            data TEXT,
            topics TEXT
        );",
        name = name
    )
}

fn insert_log_statement(name: &str, log: &Log) -> String {
    let topics = log
        .topics
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join(",");

    f!(
        "INSERT INTO logs_{name} VALUES (
            {block_number},
            '{block_hash}',
            '{transaction_hash}',
            {transaction_index},
            {log_index},
            '{address}',
            '{data}',
            '{topics}'
        );",
        name = name,
        block_number = log.block_number.unwrap(),
        block_hash = log.block_hash.unwrap(),
        transaction_hash = log.transaction_hash.unwrap(),
        transaction_index = log.transaction_index.unwrap(),
        log_index = log.log_index.unwrap(),
        address = log.address,
        data = log.data,
        topics = topics
    )
}
