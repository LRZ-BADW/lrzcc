use crate::common::{is_false, is_true, request, SerializableNone};
use crate::error::ApiError;
use crate::user::ProjectMinimal;
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

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserMinimal {
    id: u32,
    name: String,
}

impl Display for UserMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User(id={}, name={}", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserDetailed {
    id: u32,
    name: String,
    openstack_id: String, // UUIDv4 without dashes
    project: ProjectMinimal,
    project_name: String,
    role: u32,
    is_staff: bool,
    is_active: bool,
}

// TODO can we merge this with UserDetailed via some enum
// in the project field
#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserCreated {
    id: u32,
    name: String,
    openstack_id: String, // UUIDv4 without dashes
    project: u32,
    project_name: String,
    role: u32,
    is_staff: bool,
    is_active: bool,
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

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.project = Some(project);
        self
    }
}

#[derive(Clone, Debug, Serialize)]
struct UserCreateData {
    name: String,
    openstack_id: String, // UUIDv4
    // TODO can't this be optional?
    project: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    // this could be an enum right
    role: Option<u32>,
    #[serde(skip_serializing_if = "is_false")]
    is_staff: bool,
    #[serde(skip_serializing_if = "is_true")]
    is_active: bool,
}

impl UserCreateData {
    fn new(name: String, openstack_id: String, project: u32) -> Self {
        Self {
            name,
            openstack_id,
            project,
            role: None,
            is_staff: false,
            is_active: true,
        }
    }
}

pub struct UserCreateRequest {
    url: String,
    client: Rc<Client>,

    data: UserCreateData,
}

impl UserCreateRequest {
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        name: String,
        openstack_id: String,
        project: u32,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: UserCreateData::new(name, openstack_id, project),
        }
    }

    pub fn role(&mut self, role: u32) -> &mut Self {
        self.data.role = Some(role);
        self
    }

    pub fn staff(&mut self) -> &mut Self {
        self.data.is_staff = true;
        self
    }

    pub fn inactive(&mut self) -> &mut Self {
        self.data.is_active = false;
        self
    }

    pub fn send(&self) -> Result<UserCreated, ApiError> {
        request(
            &self.client,
            Method::POST,
            &self.url,
            Some(&self.data),
            StatusCode::CREATED,
        )
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

    pub fn get(&self, id: u32) -> Result<UserDetailed, ApiError> {
        // TODO use Url.join
        let url = format!("{}/{}", self.url, id.to_string());
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
        project: u32,
    ) -> UserCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        UserCreateRequest::new(
            url.as_ref(),
            &self.client,
            name,
            openstack_id,
            project,
        )
    }
}
