use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use chrono::{DateTime, FixedOffset};
use lrzcc_wire::accounting::{
    ServerConsumptionAll, ServerConsumptionFlavors, ServerConsumptionParams,
    ServerConsumptionProject, ServerConsumptionServer, ServerConsumptionUser,
};
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct ServerConsumptionRequest {
    url: String,
    client: Rc<Client>,

    params: ServerConsumptionParams,
}

impl ServerConsumptionRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: ServerConsumptionParams {
                begin: None,
                end: None,
                server: None,
                user: None,
                project: None,
                all: None,
                detail: None,
            },
        }
    }

    pub fn begin(&mut self, begin: DateTime<FixedOffset>) -> &mut Self {
        self.params.begin = Some(begin);
        self
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.params.end = Some(end);
        self
    }

    pub fn server(
        &mut self,
        server: &str,
    ) -> Result<ServerConsumptionFlavors, ApiError> {
        self.params.server = Some(server.to_string());
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
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
        self.params.server = Some(server.to_string());
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
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
        self.params.user = Some(user);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
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
        self.params.user = Some(user);
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
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
        self.params.project = Some(project);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
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
        self.params.project = Some(project);
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }

    pub fn all(&mut self) -> Result<ServerConsumptionFlavors, ApiError> {
        self.params.all = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }

    pub fn all_detail(&mut self) -> Result<ServerConsumptionAll, ApiError> {
        self.params.all = Some(true);
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }

    pub fn mine(&mut self) -> Result<ServerConsumptionFlavors, ApiError> {
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }

    pub fn mine_detail(&mut self) -> Result<ServerConsumptionUser, ApiError> {
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to envode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
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
