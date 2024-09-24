use crate::common::{request, request_bare, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use lrzcc_wire::user::{
    Project, ProjectCreateData, ProjectCreated, ProjectModifyData,
    ProjectRetrieved,
};
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use std::rc::Rc;

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
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
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

pub struct ProjectCreateRequest {
    url: String,
    client: Rc<Client>,

    data: ProjectCreateData,
}

impl ProjectCreateRequest {
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        name: String,
        openstack_id: String,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ProjectCreateData::new(name, openstack_id),
        }
    }

    pub fn user_class(&mut self, user_class: u32) -> &mut Self {
        self.data.user_class = Some(user_class);
        self
    }

    pub fn send(&self) -> Result<ProjectCreated, ApiError> {
        request(
            &self.client,
            Method::POST,
            &self.url,
            Some(&self.data),
            StatusCode::CREATED,
        )
    }
}

pub struct ProjectModifyRequest {
    url: String,
    client: Rc<Client>,

    data: ProjectModifyData,
}

impl ProjectModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ProjectModifyData::new(id),
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.data.name = Some(name);
        self
    }

    pub fn openstack_id(&mut self, openstack_id: String) -> &mut Self {
        self.data.openstack_id = Some(openstack_id);
        self
    }

    pub fn user_class(&mut self, user_class: u32) -> &mut Self {
        self.data.user_class = Some(user_class);
        self
    }

    pub fn send(&self) -> Result<Project, ApiError> {
        request(
            &self.client,
            Method::PATCH,
            &self.url,
            Some(&self.data),
            StatusCode::OK,
        )
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

    pub fn get(&self, id: u32) -> Result<ProjectRetrieved, ApiError> {
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

    pub fn create(
        &self,
        name: String,
        openstack_id: String,
    ) -> ProjectCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        ProjectCreateRequest::new(
            url.as_ref(),
            &self.client,
            name,
            openstack_id,
        )
    }

    pub fn modify(&self, id: u32) -> ProjectModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        ProjectModifyRequest::new(url.as_ref(), &self.client, id)
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
}
