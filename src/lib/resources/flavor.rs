use crate::common::{display_option, request};
use crate::error::ApiError;
use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct Flavor {
    id: u32,
    name: String,
    openstack_id: String, // UUIDv4
    #[tabled(display_with = "display_option")]
    group: Option<u32>,
    #[tabled(display_with = "display_option")]
    group_name: Option<String>,
    weight: u32,
}

impl Display for Flavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Flavor(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorGroupMinimal {
    id: u32,
    name: String,
}

// TODO maybe rethink the Display implementations
impl Display for FlavorGroupMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorDetailed {
    id: u32,
    name: String,
    openstack_id: String, // UUIDv4
    #[tabled(display_with = "display_option")]
    group: Option<FlavorGroupMinimal>,
    #[tabled(display_with = "display_option")]
    group_name: Option<String>,
    weight: u32,
}

impl Display for FlavorDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Flavor(id={}, name={})", self.id, self.name))
    }
}

pub struct FlavorApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorListRequest {
    url: String,
    client: Rc<Client>,

    all: bool,
    group: Option<u32>,
}

impl FlavorListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            all: false,
            group: None,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        // TODO maybe flip the order here, since the most specific
        // should take precedence
        if self.all {
            params.push(("all", "1".to_string()));
        } else if let Some(group) = self.group {
            params.push(("flavorgroup", group.to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<Flavor>, ApiError> {
        let url = Url::parse_with_params(self.url.as_str(), self.params())
            .context("Could not parse URL GET parameters.")?;
        request(&self.client, Method::GET, url.as_str(), StatusCode::OK)
    }

    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }

    pub fn group(&mut self, group: u32) -> &mut Self {
        self.group = Some(group);
        self
    }
}

impl FlavorApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorApi {
        FlavorApi {
            url: format!("{}/resources/flavors", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorListRequest {
        FlavorListRequest::new(self.url.as_ref(), &self.client)
    }

    pub fn get(&self, id: u32) -> Result<FlavorDetailed, ApiError> {
        // TODO use Url.join
        let url = format!("{}/{}", self.url, id.to_string());
        request(&self.client, Method::GET, url.as_str(), StatusCode::OK)
    }
}
