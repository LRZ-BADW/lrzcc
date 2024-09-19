use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Clone, Debug, Serialize, Deserialize, Tabled)]
pub struct ErrorResponse {
    pub detail: String,
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "caused by: {}", cause)?;
        current = cause.source();
    }
    Ok(())
}
