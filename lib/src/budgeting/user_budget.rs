use crate::common::{request, request_bare, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use chrono::{DateTime, FixedOffset};
use lrzcc_wire::budgeting::{
    UserBudget, UserBudgetCombined, UserBudgetCombinedDetail,
    UserBudgetCreateData, UserBudgetDetail, UserBudgetListParams,
    UserBudgetModifyData, UserBudgetOver, UserBudgetSync,
};
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use std::rc::Rc;

pub struct UserBudgetApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct UserBudgetListRequest {
    url: String,
    client: Rc<Client>,

    params: UserBudgetListParams,
}

impl UserBudgetListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: UserBudgetListParams {
                user: None,
                project: None,
                all: None,
                year: None,
            },
        }
    }

    pub fn send(&self) -> Result<Vec<UserBudget>, ApiError> {
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

pub struct UserBudgetCreateRequest {
    url: String,
    client: Rc<Client>,

    data: UserBudgetCreateData,
}

impl UserBudgetCreateRequest {
    pub fn new(url: &str, client: &Rc<Client>, user: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: UserBudgetCreateData::new(user),
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

    pub fn send(&self) -> Result<UserBudget, ApiError> {
        request(
            &self.client,
            Method::POST,
            &self.url,
            Some(&self.data),
            StatusCode::CREATED,
        )
    }
}

pub struct UserBudgetModifyRequest {
    url: String,
    client: Rc<Client>,

    data: UserBudgetModifyData,
}

impl UserBudgetModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: UserBudgetModifyData::new(id),
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

    pub fn send(&self) -> Result<UserBudget, ApiError> {
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
pub struct UserBudgetOverRequest {
    url: String,
    client: Rc<Client>,

    end: Option<DateTime<FixedOffset>>,
    budget: Option<u32>,
    user: Option<u32>,
    project: Option<u32>,
    all: bool,
    combined: bool,
    detail: bool,
}

impl UserBudgetOverRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            end: None,
            budget: None,
            user: None,
            project: None,
            all: false,
            combined: false,
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
        } else if let Some(user) = self.user {
            params.push(("user", user.to_string()));
        } else if let Some(project) = self.project {
            params.push(("project", project.to_string()));
        } else if self.all {
            params.push(("all", "1".to_string()));
        }
        if self.combined {
            params.push(("combined", "1".to_string()));
        }
        if self.detail {
            params.push(("detail", "1".to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<UserBudgetOver>, ApiError> {
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

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.end = Some(end);
        self
    }

    pub fn budget(&mut self, budget: u32) -> &mut Self {
        self.budget = Some(budget);
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

    pub fn normal(&mut self) -> Result<Vec<UserBudgetOver>, ApiError> {
        self.combined = false;
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

    pub fn combined(&mut self) -> Result<Vec<UserBudgetCombined>, ApiError> {
        self.combined = true;
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

    pub fn detail(&mut self) -> Result<Vec<UserBudgetDetail>, ApiError> {
        self.combined = false;
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

    pub fn combined_detail(
        &mut self,
    ) -> Result<Vec<UserBudgetCombinedDetail>, ApiError> {
        self.combined = true;
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

impl UserBudgetApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> UserBudgetApi {
        UserBudgetApi {
            url: format!("{}/budgeting/userbudgets", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> UserBudgetListRequest {
        UserBudgetListRequest::new(self.url.as_ref(), &self.client)
    }

    pub fn get(&self, id: u32) -> Result<UserBudget, ApiError> {
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

    pub fn create(&self, user: u32) -> UserBudgetCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        UserBudgetCreateRequest::new(url.as_ref(), &self.client, user)
    }

    pub fn modify(&self, id: u32) -> UserBudgetModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        UserBudgetModifyRequest::new(url.as_ref(), &self.client, id)
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

    pub fn over(&self) -> UserBudgetOverRequest {
        let url = format!("{}/over/", self.url);
        UserBudgetOverRequest::new(url.as_ref(), &self.client)
    }

    pub fn sync(&self) -> Result<UserBudgetSync, ApiError> {
        let url = format!("{}/sync/", self.url);
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }
}
