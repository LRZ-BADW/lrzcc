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
pub struct Project {
    id: u32,
    name: String,
    openstack_id: String, // UUIDv4 without dashes
    user_class: u32,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={}", self.id, self.name))
    }
}

pub struct ProjectApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct ProjectListRequest {
    url: String,
    client: Rc<Client>,

    all: bool,
    user_class: Option<u32>,
}

impl ProjectListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            all: false,
            user_class: None,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if self.all {
            params.push(("all", "1".to_string()));
        } else if let Some(user_class) = self.user_class {
            params.push(("userclass", user_class.to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<Project>, ApiError> {
        let url = Url::parse_with_params(self.url.as_str(), self.params())
            .context("Could not parse URL GET parameters.")?;
        request(&self.client, Method::GET, url.as_str(), StatusCode::OK)
    }

    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }

    // TODO: use enum for this
    pub fn user_class(&mut self, user_class: u32) -> &mut Self {
        self.user_class = Some(user_class);
        self
    }
}

impl ProjectApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ProjectApi {
        ProjectApi {
            url: format!("{}/user/projects", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> ProjectListRequest {
        ProjectListRequest::new(self.url.as_ref(), &self.client)
    }
}
