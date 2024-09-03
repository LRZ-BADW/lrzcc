use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use chrono::{DateTime, FixedOffset};
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub type ServerConsumptionFlavors = HashMap<String, f64>;
pub type ServerConsumptionServer = ServerConsumptionFlavors;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConsumptionUser {
    pub total: ServerConsumptionFlavors,
    pub servers: HashMap<String, ServerConsumptionServer>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConsumptionProject {
    pub total: ServerConsumptionFlavors,
    pub users: HashMap<String, ServerConsumptionUser>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConsumptionAll {
    pub total: ServerConsumptionFlavors,
    pub projects: HashMap<String, ServerConsumptionProject>,
}

#[derive(Debug)]
pub struct ServerConsumptionRequest {
    url: String,
    client: Rc<Client>,

    begin: Option<DateTime<FixedOffset>>,
    end: Option<DateTime<FixedOffset>>,
    server: Option<String>,
    user: Option<u32>,
    project: Option<u32>,
    all: bool,
    detail: bool,
}

impl ServerConsumptionRequest {
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
            detail: false,
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
        if self.detail {
            params.push(("detail", "1".to_string()));
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

    pub fn server(
        &mut self,
        server: &str,
    ) -> Result<ServerConsumptionFlavors, ApiError> {
        self.server = Some(server.to_string());
        self.detail = false;
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

    pub fn server_detail(
        &mut self,
        server: &str,
    ) -> Result<ServerConsumptionServer, ApiError> {
        self.server = Some(server.to_string());
        self.detail = true;
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

    pub fn user(
        &mut self,
        user: u32,
    ) -> Result<ServerConsumptionFlavors, ApiError> {
        self.user = Some(user);
        self.detail = false;
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

    pub fn user_detail(
        &mut self,
        user: u32,
    ) -> Result<ServerConsumptionUser, ApiError> {
        self.user = Some(user);
        self.detail = true;
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

    pub fn project(
        &mut self,
        project: u32,
    ) -> Result<ServerConsumptionFlavors, ApiError> {
        self.project = Some(project);
        self.detail = false;
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

    pub fn project_detail(
        &mut self,
        project: u32,
    ) -> Result<ServerConsumptionProject, ApiError> {
        self.project = Some(project);
        self.detail = true;
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

    pub fn all(&mut self) -> Result<ServerConsumptionFlavors, ApiError> {
        self.all = true;
        self.detail = false;
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

    pub fn all_detail(&mut self) -> Result<ServerConsumptionAll, ApiError> {
        self.all = true;
        self.detail = true;
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

    pub fn mine(&mut self) -> Result<ServerConsumptionFlavors, ApiError> {
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

    pub fn mine_detail(&mut self) -> Result<ServerConsumptionUser, ApiError> {
        self.detail = true;
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

pub struct ServerConsumptionApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl ServerConsumptionApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ServerConsumptionApi {
        ServerConsumptionApi {
            url: format!("{}/accounting/serverconsumption/", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn get(&self) -> ServerConsumptionRequest {
        ServerConsumptionRequest::new(self.url.as_str(), &self.client)
    }
}
