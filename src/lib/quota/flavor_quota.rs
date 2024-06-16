use crate::common::request;
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
pub struct FlavorQuota {
    id: u32,
    user: u32,
    username: String,
    quota: u32,
    flavor_group: u32,
    flavor_group_name: String,
}

impl Display for FlavorQuota {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "FlavorQuota(id={}, user={}, flavor_group={})",
            self.id, self.user, self.flavor_group
        ))
    }
}

pub struct FlavorQuotaApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorQuotaListRequest {
    url: String,
    client: Rc<Client>,

    all: bool,
    group: Option<u32>,
    user: Option<u32>,
}

impl FlavorQuotaListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            all: false,
            group: None,
            user: None,
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
        } else if let Some(user) = self.user {
            params.push(("user", user.to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<FlavorQuota>, ApiError> {
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

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.user = Some(user);
        self
    }
}

impl FlavorQuotaApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorQuotaApi {
        FlavorQuotaApi {
            url: format!("{}/quota/flavorquotas", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorQuotaListRequest {
        FlavorQuotaListRequest::new(self.url.as_ref(), &self.client)
    }
}
