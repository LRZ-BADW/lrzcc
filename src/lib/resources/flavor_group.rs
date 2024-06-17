use crate::common::request;
use crate::error::ApiError;
use crate::resources::FlavorMinimal;
use crate::user::ProjectMinimal;
use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorGroup {
    id: u32,
    name: String,
    #[tabled(skip)]
    flavors: Vec<u32>,
    project: u32,
}

impl Display for FlavorGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
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
pub struct FlavorGroupDetailed {
    id: u32,
    name: String,
    #[tabled(skip)]
    flavors: Vec<FlavorMinimal>,
    project: ProjectMinimal,
}

impl Display for FlavorGroupDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

pub struct FlavorGroupApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorGroupListRequest {
    url: String,
    client: Rc<Client>,

    all: bool,
}

impl FlavorGroupListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            all: false,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if self.all {
            params.push(("all", "1".to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<FlavorGroup>, ApiError> {
        let url = Url::parse_with_params(self.url.as_str(), self.params())
            .context("Could not parse URL GET parameters.")?;
        request(&self.client, Method::GET, url.as_str(), StatusCode::OK)
    }

    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }
}

impl FlavorGroupApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorGroupApi {
        FlavorGroupApi {
            url: format!("{}/resources/flavorgroups", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorGroupListRequest {
        FlavorGroupListRequest::new(self.url.as_ref(), &self.client)
    }

    pub fn get(&self, id: u32) -> Result<FlavorGroupDetailed, ApiError> {
        // TODO use Url.join
        let url = format!("{}/{}", self.url, id.to_string());
        request(&self.client, Method::GET, url.as_str(), StatusCode::OK)
    }
}
