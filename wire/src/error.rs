use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Clone, Debug, Serialize, Deserialize, Tabled)]
pub struct ErrorResponse {
    pub detail: String,
}
