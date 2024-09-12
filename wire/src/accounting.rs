use crate::common::display_option;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ServerState {
    pub id: u32,
    pub begin: DateTime<FixedOffset>,
    #[tabled(display_with = "display_option")]
    pub end: Option<DateTime<FixedOffset>>,
    pub instance_id: String, // UUIDv4
    pub instance_name: String,
    pub flavor: u32,
    pub flavor_name: String,
    pub status: String,
    pub user: u32,
    pub username: String,
}

impl Display for ServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ServerState(id={})", self.id))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ServerStateImport {
    pub new_state_count: u32,
    pub end_state_count: u32,
}
