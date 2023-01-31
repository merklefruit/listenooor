use std::sync::Arc;

use crate::prelude::*;

#[derive(Clone)]
pub struct SqliteStorage {
    pub db_connection: Arc<sqlite::Connection>,
}

impl SqliteStorage {
    pub fn init(path: Option<&str>) -> Self {
        let path = match path {
            Some(path) => path,
            None => ":memory:",
        };

        let db_connection = Arc::new(sqlite::open(path).unwrap());

        Self { db_connection }
    }

    pub fn run_query(&self, query: &str) -> Result<()> {
        self.db_connection.execute(query)?;
        Ok(())
    }

    pub fn get_all_logs_for_event(&self, event_name: &str) -> Result<()> {
        todo!();
    }

    pub fn get_latest_log_for_event(&self, event_name: &str) -> Result<()> {
        todo!();
    }
}
