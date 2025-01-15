use crate::common::{request, request_bare, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use chrono::{DateTime, FixedOffset};
use lrzcc_wire::budgeting::{
    ProjectBudget, ProjectBudgetCreateData, ProjectBudgetDetail,
    ProjectBudgetListParams, ProjectBudgetModifyData, ProjectBudgetOver,
};
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use std::rc::Rc;

pub struct ProjectBudgetApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct ProjectBudgetListRequest {
    url: String,
    client: Rc<Client>,

    params: ProjectBudgetListParams,
}

impl ProjectBudgetListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: ProjectBudgetListParams {
                user: None,
                project: None,
                all: None,
                year: None,
            },
        }
    }

    pub fn send(&self) -> Result<Vec<ProjectBudget>, ApiError> {
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

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.params.user = Some(user);
        self
    }

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.params.project = Some(project);
        self
    }

    pub fn all(&mut self) -> &mut Self {
        self.params.all = Some(true);
        self
    }

    pub fn year(&mut self, year: u32) -> &mut Self {
        self.params.year = Some(year);
        self
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

#[derive(Debug)]
pub struct ProjectBudgetOverRequest {
    url: String,
    client: Rc<Client>,

    end: Option<DateTime<FixedOffset>>,
    budget: Option<u32>,
    project: Option<u32>,
    all: bool,
    detail: bool,
}

impl ProjectBudgetOverRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            end: None,
            budget: None,
            project: None,
            all: false,
            detail: false,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(end) = self.end {
            params.push(("end", end.to_rfc3339()));
        }
        if let Some(budget) = self.budget {
            params.push(("budget", budget.to_string()));
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

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.end = Some(end);
        self
    }

    pub fn budget(&mut self, budget: u32) -> &mut Self {
        self.budget = Some(budget);
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

    pub fn normal(&mut self) -> Result<Vec<ProjectBudgetOver>, ApiError> {
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

    pub fn detail(&mut self) -> Result<Vec<ProjectBudgetDetail>, ApiError> {
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

    pub fn over(&self) -> ProjectBudgetOverRequest {
        let url = format!("{}/over/", self.url);
        ProjectBudgetOverRequest::new(url.as_ref(), &self.client)
    }
}
