use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use chrono::{DateTime, FixedOffset};
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ServerCost {
    pub total: f64,
}

#[derive(Debug)]
pub struct ServerCostRequest {
    url: String,
    client: Rc<Client>,

    begin: Option<DateTime<FixedOffset>>,
    end: Option<DateTime<FixedOffset>>,
    server: Option<String>,
    user: Option<u32>,
    project: Option<u32>,
    all: bool,
}

impl ServerCostRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            begin: None,
            end: None,
            server: None,
            user: None,
            project: None,
            all: false,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(begin) = self.begin {
            params.push(("begin", begin.to_rfc3339().to_string()));
        }
        if let Some(end) = self.end {
            params.push(("end", end.to_rfc3339().to_string()));
        }
        if let Some(server) = &self.server {
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

    pub fn begin(&mut self, begin: DateTime<FixedOffset>) -> &mut Self {
        self.begin = Some(begin);
        self
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.end = Some(end);
        self
    }

    pub fn server(&mut self, server: &str) -> &mut Self {
        self.server = Some(server.to_string());
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

    pub fn send(&self) -> Result<ServerCost, ApiError> {
        let url = Url::parse_with_params(self.url.as_str(), self.params())
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

pub struct ServerCostApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl ServerCostApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ServerCostApi {
        ServerCostApi {
            url: format!("{}/accounting/servercost/", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn get(&self) -> ServerCostRequest {
        ServerCostRequest::new(self.url.as_str(), &self.client)
    }
}
