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
pub struct UserBudget {
    id: u32,
    user: u32,
    username: String,
    year: u32,
    amount: u32,
}

impl Display for UserBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("UserBudget(id={})", self.id))
    }
}

pub struct UserBudgetApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct UserBudgetListRequest {
    url: String,
    client: Rc<Client>,

    user: Option<u32>,
    project: Option<u32>,
    all: bool,
    year: Option<u32>,
}

impl UserBudgetListRequest {
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

    pub fn send(&self) -> Result<Vec<UserBudget>, ApiError> {
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
struct UserBudgetCreateData {
    user: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
}

impl UserBudgetCreateData {
    fn new(user: u32) -> Self {
        Self {
            user,
            year: None,
            amount: None,
        }
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

#[derive(Clone, Debug, Serialize)]
struct UserBudgetModifyData {
    id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<u32>,
    #[serde(skip_serializing_if = "is_false")]
    force: bool,
}

impl UserBudgetModifyData {
    fn new(id: u32) -> Self {
        Self {
            id,
            amount: None,
            force: false,
        }
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
}
