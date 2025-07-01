use std::rc::Rc;

use anyhow::Context;
use avina_wire::resources::{
    FlavorGroup, FlavorGroupCreateData, FlavorGroupCreated,
    FlavorGroupDetailed, FlavorGroupInitialize, FlavorGroupListParams,
    FlavorGroupModifyData, FlavorGroupUsage, FlavorGroupUsageAggregate,
};
use reqwest::{Method, StatusCode, Url, blocking::Client};

use crate::{
    common::{SerializableNone, request, request_bare},
    error::ApiError,
};

pub struct FlavorGroupApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorGroupListRequest {
    url: String,
    client: Rc<Client>,

    params: FlavorGroupListParams,
}

impl FlavorGroupListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: FlavorGroupListParams { all: None },
        }
    }

    // TODO: only the return type changes, pull these functions into a macro
    pub fn send(&self) -> Result<Vec<FlavorGroup>, ApiError> {
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
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

    pub fn all(&mut self) -> &mut Self {
        self.params.all = Some(true);
        self
    }
}

pub struct FlavorGroupCreateRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorGroupCreateData,
}

impl FlavorGroupCreateRequest {
    pub fn new(url: &str, client: &Rc<Client>, name: String) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorGroupCreateData::new(name),
        }
    }

    pub fn send(&self) -> Result<FlavorGroupCreated, ApiError> {
        request(
            &self.client,
            Method::POST,
            &self.url,
            Some(&self.data),
            StatusCode::CREATED,
        )
    }
}

pub struct FlavorGroupModifyRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorGroupModifyData,
}

impl FlavorGroupModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorGroupModifyData::new(id),
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.data.name = Some(name);
        self
    }

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.data.project = Some(project);
        self
    }

    pub fn send(&self) -> Result<FlavorGroup, ApiError> {
        request(
            &self.client,
            Method::PATCH,
            &self.url,
            Some(&self.data),
            StatusCode::OK,
        )
    }
}

pub struct FlavorGroupUsageRequest {
    url: String,
    client: Rc<Client>,

    user: Option<u32>,
    project: Option<u32>,
    all: bool,
    aggregate: bool,
}

impl FlavorGroupUsageRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            user: None,
            project: None,
            all: false,
            aggregate: false,
        }
    }

    pub fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(user) = self.user {
            params.push(("user", user.to_string()));
        } else if let Some(project) = self.project {
            params.push(("project", project.to_string()));
        } else if self.all {
            params.push(("all", "1".to_string()));
        }
        if self.aggregate {
            params.push(("aggregate", "1".to_string()));
        }
        params
    }

    pub fn user(
        &mut self,
        user: u32,
    ) -> Result<Vec<FlavorGroupUsage>, ApiError> {
        self.user = Some(user);
        self.aggregate = false;
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

    pub fn user_aggregate(
        &mut self,
        user: u32,
    ) -> Result<Vec<FlavorGroupUsageAggregate>, ApiError> {
        self.user = Some(user);
        self.aggregate = true;
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
    ) -> Result<Vec<FlavorGroupUsage>, ApiError> {
        self.project = Some(project);
        self.aggregate = false;
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

    pub fn project_aggregate(
        &mut self,
        project: u32,
    ) -> Result<Vec<FlavorGroupUsageAggregate>, ApiError> {
        self.project = Some(project);
        self.aggregate = true;
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

    pub fn all(&mut self) -> Result<Vec<FlavorGroupUsage>, ApiError> {
        self.all = true;
        self.aggregate = false;
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

    pub fn all_aggregate(
        &mut self,
    ) -> Result<Vec<FlavorGroupUsageAggregate>, ApiError> {
        self.all = true;
        self.aggregate = true;
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

    pub fn mine(&mut self) -> Result<Vec<FlavorGroupUsage>, ApiError> {
        self.aggregate = false;
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

    pub fn mine_aggregate(
        &mut self,
    ) -> Result<Vec<FlavorGroupUsageAggregate>, ApiError> {
        // TODO use Url.join
        self.aggregate = true;
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

impl FlavorGroupApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorGroupApi {
        FlavorGroupApi {
            url: format!("{base_url}/resources/flavorgroups"),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorGroupListRequest {
        FlavorGroupListRequest::new(self.url.as_ref(), &self.client)
    }

    pub fn get(&self, id: u32) -> Result<FlavorGroupDetailed, ApiError> {
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

    pub fn create(&self, name: String) -> FlavorGroupCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        FlavorGroupCreateRequest::new(url.as_ref(), &self.client, name)
    }

    pub fn modify(&self, id: u32) -> FlavorGroupModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        FlavorGroupModifyRequest::new(url.as_ref(), &self.client, id)
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

    pub fn initialize(&self) -> Result<FlavorGroupInitialize, ApiError> {
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

    pub fn usage(&self) -> FlavorGroupUsageRequest {
        let url = format!("{}/usage/", self.url);
        FlavorGroupUsageRequest::new(url.as_ref(), &self.client)
    }
}
