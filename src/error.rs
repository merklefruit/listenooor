use ethers::providers::{ProviderError, WsClientError};
use std::env::VarError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String), // TODO: Remove for release

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    WsClient(#[from] WsClientError),

    #[error(transparent)]
    Env(#[from] VarError),

    #[error(transparent)]
    Provider(#[from] ProviderError),

    #[error(transparent)]
    Sqlite(#[from] sqlite::Error),
}
