use std::fmt::Debug;

use lrzcc_wire::error::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    ResponseError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
