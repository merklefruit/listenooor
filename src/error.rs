use ethers::providers::{ProviderError, WsClientError};
use std::env::VarError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String), // TODO: Remove for release

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl From<WsClientError> for Error {
    fn from(e: WsClientError) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<ProviderError> for Error {
    fn from(e: ProviderError) -> Self {
        Self::Generic(e.to_string())
    }
}
