use ethers::providers::{ProviderError, WsClientError};
use std::env::VarError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String), // TODO: Remove for release

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Websocket client error: {0}")]
    WsClient(#[from] WsClientError),

    #[error("Environment variable error: {0}")]
    Env(#[from] VarError),

    #[error("Provider error: {0}")]
    Provider(#[from] ProviderError),

    #[error("Sqlite error: {0}")]
    Sqlite(#[from] sqlite::Error),

    #[error("Actix error: {0}")]
    Actix(#[from] actix_web::Error),
}
