use crate::common::{
    is_false, is_true, request, request_bare, SerializableNone,
};
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
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub project: u32,
    pub project_name: String,
    pub role: u32,
    pub is_staff: bool,
    pub is_active: bool,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User(id={}, name={}", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserMinimal {
    pub id: u32,
    pub name: String,
}

impl Display for UserMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User(id={}, name={}", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserDetailed {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub project: ProjectMinimal,
    pub project_name: String,
    pub role: u32,
    pub is_staff: bool,
    pub is_active: bool,
}

// TODO can we merge this with UserDetailed via some enum
// in the project field
#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserCreated {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub project: u32,
    pub project_name: String,
    pub role: u32,
    pub is_staff: bool,
    pub is_active: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserImport {
    pub new_project_count: u32,
    pub new_user_count: u32,
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

#[derive(Clone, Debug, Serialize)]
struct UserModifyData {
    id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    openstack_id: Option<String>, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_staff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_active: Option<bool>,
}

impl UserModifyData {
    fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            openstack_id: None,
            project: None,
            role: None,
            is_staff: None,
            is_active: None,
        }
    }
}

pub struct UserModifyRequest {
    url: String,
    client: Rc<Client>,

    data: UserModifyData,
}

impl UserModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: UserModifyData::new(id),
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

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.data.project = Some(project);
        self
    }

    pub fn role(&mut self, role: u32) -> &mut Self {
        self.data.role = Some(role);
        self
    }

    pub fn is_staff(&mut self, is_staff: bool) -> &mut Self {
        self.data.is_staff = Some(is_staff);
        self
    }

    pub fn is_active(&mut self, is_active: bool) -> &mut Self {
        self.data.is_active = Some(is_active);
        self
    }

    pub fn send(&self) -> Result<User, ApiError> {
        request(
            &self.client,
            Method::PATCH,
            &self.url,
            Some(&self.data),
            StatusCode::OK,
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

    pub fn modify(&self, id: u32) -> UserModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        UserModifyRequest::new(url.as_ref(), &self.client, id)
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

    pub fn me(&self) -> Result<UserDetailed, ApiError> {
        // TODO use Url.join
        let url = format!(
            "{}/me",
            self.url
                .rfind('/')
                .map(|i| &self.url[..i])
                .unwrap_or(&self.url)
        );
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }

    pub fn import(&self) -> Result<UserImport, ApiError> {
        // TODO use Url.join
        let url = format!(
            "{}/import/",
            self.url
                .rfind('/')
                .map(|i| &self.url[..i])
                .unwrap_or(&self.url)
        );
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }
}