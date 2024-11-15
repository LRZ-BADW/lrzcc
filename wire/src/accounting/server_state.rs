use crate::common::display_option;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct ServerStateImport {
    pub new_state_count: u32,
    pub end_state_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerStateListParams {
    pub server: Option<String>,
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerStateCreateData {
    pub begin: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<FixedOffset>>,
    pub instance_id: String, // UUIDv4
    pub instance_name: String,
    pub flavor: u32,
    // TODO we need an enum here
    pub status: String,
    pub user: u32,
}

impl ServerStateCreateData {
    pub fn new(
        begin: DateTime<FixedOffset>,
        instance_id: String, // UUIDv4
        instance_name: String,
        flavor: u32,
        status: String,
        user: u32,
    ) -> Self {
        Self {
            begin,
            end: None,
            instance_id,
            instance_name,
            flavor,
            status,
            user,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerStateModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO we need an enum here
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<u32>,
}

impl ServerStateModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            begin: None,
            end: None,
            instance_id: None,
            instance_name: None,
            flavor: None,
            status: None,
            user: None,
        }
    }
}
