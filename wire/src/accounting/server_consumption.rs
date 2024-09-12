use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ServerConsumptionFlavors = HashMap<String, f64>;

pub type ServerConsumptionServer = ServerConsumptionFlavors;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConsumptionUser {
    pub total: ServerConsumptionFlavors,
    pub servers: HashMap<String, ServerConsumptionServer>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConsumptionProject {
    pub total: ServerConsumptionFlavors,
    pub users: HashMap<String, ServerConsumptionUser>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConsumptionAll {
    pub total: ServerConsumptionFlavors,
    pub projects: HashMap<String, ServerConsumptionProject>,
}
