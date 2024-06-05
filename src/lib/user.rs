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
pub struct User {
    id: u32,
    name: String,
    openstack_id: String, // UUIDv4 without dashes
    project: u32,
    project_name: String,
    role: u32,
    is_staff: bool,
    is_active: bool,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User(id={}, name={}", self.id, self.name))
    }
}

pub struct UserApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct UserListRequest {
    url: String,
    client: Rc<Client>,

    all: bool,
    project: Option<u32>,
}

impl UserListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            all: false,
            project: None,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if self.all {
            params.push(("all", "1".to_string()));
        } else if let Some(project) = self.project {
            params.push(("project", project.to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<User>, ApiError> {
        let url = Url::parse_with_params(self.url.as_str(), self.params())
            .context("Could not parse URL GET parameters.")?;
        println!("{}", url);
        request(&self.client, Method::GET, url.as_str(), StatusCode::OK)
    }

    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.project = Some(project);
        self
    }
}

impl UserApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> UserApi {
        UserApi {
            url: format!("{}/user/users", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> UserListRequest {
        UserListRequest::new(self.url.as_ref(), &self.client)
    }
}
