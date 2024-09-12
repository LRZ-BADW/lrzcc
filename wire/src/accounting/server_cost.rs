use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ServerCostSimple {
    pub total: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerCostServer {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerCostUser {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
    pub servers: HashMap<String, ServerCostServer>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerCostProject {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
    pub users: HashMap<String, ServerCostUser>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerCostAll {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
    pub projects: HashMap<String, ServerCostProject>,
}
