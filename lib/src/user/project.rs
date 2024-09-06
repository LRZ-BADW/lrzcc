use crate::common::{request, request_bare, SerializableNone};
use crate::error::ApiError;
use crate::resources::FlavorGroupMinimal;
use crate::user::UserMinimal;
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
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub user_class: u32,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={}", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ProjectMinimal {
    pub id: u32,
    pub name: String,
    pub user_class: u32,
}

impl Display for ProjectMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ProjectDetailed {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub user_class: u32,
    // TODO rethink list output in detailed structs:
    // maybe we could have only the first few entries followed by ...
    // in the output
    #[tabled(skip)]
    pub users: Vec<UserMinimal>,
    #[tabled(skip)]
    pub flavor_groups: Vec<FlavorGroupMinimal>,
}

impl Display for ProjectDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={}", self.id, self.name))
    }
}

// TODO can we merge this with ProjectDetailed via some enum
// in the project field
#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ProjectCreated {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub user_class: u32,
    #[tabled(skip)]
    pub users: Vec<u32>,
    #[tabled(skip)]
    pub flavor_groups: Vec<u32>,
}

impl Display for ProjectCreated {
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

#[derive(Clone, Debug, Serialize)]
struct ProjectCreateData {
    name: String,
    openstack_id: String, // UUIDv4
    // #[serde(skip_serializing_if = "Option::is_none")]
    user_class: Option<u32>,
}

impl ProjectCreateData {
    fn new(name: String, openstack_id: String) -> Self {
        Self {
            name,
            openstack_id,
            user_class: None,
        }
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

#[derive(Clone, Debug, Serialize)]
struct ProjectModifyData {
    id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    openstack_id: Option<String>, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    user_class: Option<u32>,
}

impl ProjectModifyData {
    fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            openstack_id: None,
            user_class: None,
        }
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

    pub fn get(&self, id: u32) -> Result<ProjectDetailed, ApiError> {
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
