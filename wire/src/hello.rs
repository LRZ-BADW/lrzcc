use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct Hello {
    pub message: String,
}

impl Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}
