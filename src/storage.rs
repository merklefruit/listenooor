use crate::prelude::*;

pub struct SqliteStorage {
    pub db_connection: sqlite::Connection,
}

impl SqliteStorage {
    pub fn init(path: Option<&str>) -> Self {
        let path = match path {
            Some(path) => path,
            None => ":memory:",
        };

        let db_connection = sqlite::open(path).unwrap();

        Self { db_connection }
    }

    pub fn run_query(&mut self, query: &str) -> Result<()> {
        let mut conn = self.db_connection.execute(query)?;
        Ok(())
    }
}
