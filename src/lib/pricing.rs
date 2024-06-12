use crate::common::request;
use crate::error::ApiError;
use anyhow::Context;
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorPrice {
    id: u32,
    flavor: u32,
    flavor_name: String,
    user_class: u32,
    unit_price: f64,
    start_time: DateTime<Utc>,
}

impl Display for FlavorPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "FlavorPrice(id={}, flavor={}",
            self.id, self.flavor_name
        ))
    }
}

pub struct FlavorPriceApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorPriceListRequest {
    url: String,
    client: Rc<Client>,
}

impl FlavorPriceListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
        }
    }

    pub fn send(&self) -> Result<Vec<FlavorPrice>, ApiError> {
        let url = Url::parse(self.url.as_str())
            .context("Could not parse URL GET parameters.")?;
        request(&self.client, Method::GET, url.as_str(), StatusCode::OK)
    }
}

impl FlavorPriceApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorPriceApi {
        FlavorPriceApi {
            url: format!("{}/pricing/flavorprices", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorPriceListRequest {
        FlavorPriceListRequest::new(self.url.as_ref(), &self.client)
    }
}
