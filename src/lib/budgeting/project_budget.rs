use crate::common::{is_false, request, request_bare, SerializableNone};
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
pub struct ProjectBudget {
    id: u32,
    project: u32,
    project_name: String,
    year: u32,
    amount: u32,
}

impl Display for ProjectBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ProjectBudget(id={})", self.id))
    }
}

pub struct ProjectBudgetApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct ProjectBudgetListRequest {
    url: String,
    client: Rc<Client>,

    user: Option<u32>,
    project: Option<u32>,
    all: bool,
    year: Option<u32>,
}

impl ProjectBudgetListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            user: None,
            project: None,
            all: false,
            year: None,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(user) = self.user {
            params.push(("user", user.to_string()));
        } else if let Some(project) = self.project {
            params.push(("project", project.to_string()));
        } else if self.all {
            params.push(("all", "1".to_string()));
        }
        if let Some(year) = self.year {
            params.push(("year", year.to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<ProjectBudget>, ApiError> {
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

    pub fn year(&mut self, year: u32) -> &mut Self {
        self.year = Some(year);
        self
    }
}

#[derive(Clone, Debug, Serialize)]
struct ProjectBudgetCreateData {
    project: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
}

impl ProjectBudgetCreateData {
    fn new(project: u32) -> Self {
        Self {
            project,
            year: None,
            amount: None,
        }
    }
}

pub struct ProjectBudgetCreateRequest {
    url: String,
    client: Rc<Client>,

    data: ProjectBudgetCreateData,
}

impl ProjectBudgetCreateRequest {
    pub fn new(url: &str, client: &Rc<Client>, project: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ProjectBudgetCreateData::new(project),
        }
    }

    pub fn year(&mut self, year: u32) -> &mut Self {
        self.data.year = Some(year);
        self
    }

    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.data.amount = Some(amount);
        self
    }

    pub fn send(&self) -> Result<ProjectBudget, ApiError> {
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
struct ProjectBudgetModifyData {
    id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<u32>,
    #[serde(skip_serializing_if = "is_false")]
    force: bool,
}

impl ProjectBudgetModifyData {
    fn new(id: u32) -> Self {
        Self {
            id,
            amount: None,
            force: false,
        }
    }
}

pub struct ProjectBudgetModifyRequest {
    url: String,
    client: Rc<Client>,

    data: ProjectBudgetModifyData,
}

impl ProjectBudgetModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ProjectBudgetModifyData::new(id),
        }
    }

    pub fn amount(&mut self, amount: u32) -> &mut Self {
        self.data.amount = Some(amount);
        self
    }

    pub fn force(&mut self) -> &mut Self {
        self.data.force = true;
        self
    }

    pub fn send(&self) -> Result<ProjectBudget, ApiError> {
        request(
            &self.client,
            Method::PATCH,
            &self.url,
            Some(&self.data),
            StatusCode::OK,
        )
    }
}

impl ProjectBudgetApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ProjectBudgetApi {
        ProjectBudgetApi {
            url: format!("{}/budgeting/projectbudgets", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> ProjectBudgetListRequest {
        ProjectBudgetListRequest::new(self.url.as_ref(), &self.client)
    }

    pub fn get(&self, id: u32) -> Result<ProjectBudget, ApiError> {
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

    pub fn create(&self, project: u32) -> ProjectBudgetCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        ProjectBudgetCreateRequest::new(url.as_ref(), &self.client, project)
    }

    pub fn modify(&self, id: u32) -> ProjectBudgetModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        ProjectBudgetModifyRequest::new(url.as_ref(), &self.client, id)
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
