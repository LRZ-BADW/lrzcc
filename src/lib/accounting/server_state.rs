use crate::common::{display_option, request};
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
pub struct ServerState {
    id: u32,
    begin: DateTime<Utc>,
    #[tabled(display_with = "display_option")]
    end: Option<DateTime<Utc>>,
    instance_id: String, // UUIDv4
    instance_name: String,
    flavor: u32,
    flavor_name: String,
    status: String,
    user: u32,
    username: String,
}

impl Display for ServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ServerState(id={})", self.id))
    }
}

pub struct ServerStateApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct ServerStateListRequest {
    url: String,
    client: Rc<Client>,

    server: Option<u32>,
    user: Option<u32>,
    project: Option<u32>,
    all: bool,
}

impl ServerStateListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            server: None,
            user: None,
            project: None,
            all: false,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(server) = self.server {
            params.push(("server", server.to_string()));
        } else if let Some(user) = self.user {
            params.push(("user", user.to_string()));
        } else if let Some(project) = self.project {
            params.push(("project", project.to_string()));
        } else if self.all {
            params.push(("all", "1".to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<ServerState>, ApiError> {
        let url = Url::parse_with_params(self.url.as_str(), self.params())
            .context("Could not parse URL GET parameters.")?;
        request(&self.client, Method::GET, url.as_str(), StatusCode::OK)
    }

    pub fn server(&mut self, server: u32) -> &mut Self {
        self.server = Some(server);
        self
    }

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.project = Some(project);
        self
    }

    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }
}

impl ServerStateApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ServerStateApi {
        ServerStateApi {
            url: format!("{}/resources/flavors", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> ServerStateListRequest {
        ServerStateListRequest::new(self.url.as_ref(), &self.client)
    }
}
