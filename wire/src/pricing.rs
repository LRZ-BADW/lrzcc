use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorPrice {
    pub id: u32,
    pub flavor: u32,
    pub flavor_name: String,
    pub user_class: u32,
    pub unit_price: f64,
    pub start_time: DateTime<FixedOffset>,
}

impl Display for FlavorPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "FlavorPrice(id={}, flavor={})",
            self.id, self.flavor_name
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorPriceInitialize {
    pub new_flavor_price_count: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct FlavorPriceCreateData {
    pub flavor: u32,
    // TODO use an enum for this
    pub user_class: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<FixedOffset>>,
}

impl FlavorPriceCreateData {
    pub fn new(flavor: u32, user_class: u32) -> Self {
        Self {
            flavor,
            user_class,
            price: None,
            start_time: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FlavorPriceModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_class: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<FixedOffset>>,
}

impl FlavorPriceModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            flavor: None,
            user_class: None,
            unit_price: None,
            start_time: None,
        }
    }
}
