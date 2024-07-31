use crate::common::{request, request_bare, SerializableNone};
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
    pub id: u32,
    pub flavor: u32,
    pub flavor_name: String,
    pub user_class: u32,
    pub unit_price: f64,
    pub start_time: DateTime<Utc>,
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
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }
}

#[derive(Clone, Debug, Serialize)]
struct FlavorPriceCreateData {
    flavor: u32,
    // TODO use an enum for this
    user_class: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<DateTime<Utc>>,
}

impl FlavorPriceCreateData {
    fn new(flavor: u32, user_class: u32) -> Self {
        Self {
            flavor,
            user_class,
            price: None,
            start_time: None,
        }
    }
}

pub struct FlavorPriceCreateRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorPriceCreateData,
}

impl FlavorPriceCreateRequest {
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        flavor: u32,
        user_class: u32,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorPriceCreateData::new(flavor, user_class),
        }
    }

    pub fn price(&mut self, price: f64) -> &mut Self {
        self.data.price = Some(price);
        self
    }

    pub fn start_time(&mut self, start_time: DateTime<Utc>) -> &mut Self {
        self.data.start_time = Some(start_time);
        self
    }

    pub fn send(&self) -> Result<FlavorPrice, ApiError> {
        request(
            &self.client,
            Method::POST,
            &self.url,
            Some(&self.data),
            StatusCode::CREATED,
        )
    }
}

#[derive(Clone, Debug, Serialize)]
struct FlavorPriceModifyData {
    id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    flavor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_class: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<DateTime<Utc>>,
}

impl FlavorPriceModifyData {
    fn new(id: u32) -> Self {
        Self {
            id,
            flavor: None,
            user_class: None,
            unit_price: None,
            start_time: None,
        }
    }
}

pub struct FlavorPriceModifyRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorPriceModifyData,
}

impl FlavorPriceModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorPriceModifyData::new(id),
        }
    }

    pub fn flavor(&mut self, flavor: u32) -> &mut Self {
        self.data.flavor = Some(flavor);
        self
    }

    pub fn user_class(&mut self, user_class: u32) -> &mut Self {
        self.data.user_class = Some(user_class);
        self
    }

    pub fn unit_price(&mut self, unit_price: f64) -> &mut Self {
        self.data.unit_price = Some(unit_price);
        self
    }

    pub fn start_time(&mut self, start_time: DateTime<Utc>) -> &mut Self {
        self.data.start_time = Some(start_time);
        self
    }

    pub fn send(&self) -> Result<FlavorPrice, ApiError> {
        request(
            &self.client,
            Method::PATCH,
            &self.url,
            Some(&self.data),
            StatusCode::OK,
        )
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

    pub fn get(&self, id: u32) -> Result<FlavorPrice, ApiError> {
        // TODO use Url.join
        let url = format!("{}/{}", self.url, id);
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }

    pub fn create(
        &self,
        flavor: u32,
        user_class: u32,
    ) -> FlavorPriceCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        FlavorPriceCreateRequest::new(
            url.as_ref(),
            &self.client,
            flavor,
            user_class,
        )
    }

    pub fn modify(&self, id: u32) -> FlavorPriceModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        FlavorPriceModifyRequest::new(url.as_ref(), &self.client, id)
    }

    pub fn delete(&self, id: u32) -> Result<(), ApiError> {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        request_bare(
            &self.client,
            Method::DELETE,
            url.as_str(),
            SerializableNone!(),
            StatusCode::NO_CONTENT,
        )?;
        Ok(())
    }

    pub fn initialize(&self) -> Result<FlavorPriceInitialize, ApiError> {
        // TODO use Url.join
        let url = format!("{}/initialize/", self.url);
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }
}
