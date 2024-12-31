use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ServerConsumptionFlavors = HashMap<String, f64>;

pub type ServerConsumptionServer = ServerConsumptionFlavors;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerConsumptionUser {
    pub total: ServerConsumptionFlavors,
    pub servers: HashMap<String, ServerConsumptionServer>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerConsumptionProject {
    pub total: ServerConsumptionFlavors,
    pub users: HashMap<String, ServerConsumptionUser>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerConsumptionAll {
    pub total: ServerConsumptionFlavors,
    pub projects: HashMap<String, ServerConsumptionProject>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConsumptionParams {
    pub begin: Option<DateTime<FixedOffset>>,
    pub end: Option<DateTime<FixedOffset>>,
    pub server: Option<String>,
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: bool,
    pub detail: bool,
}
