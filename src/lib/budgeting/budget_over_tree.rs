use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use chrono::{DateTime, FixedOffset};
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BudgetOverTreeServer {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BudgetOverTreeUser {
    pub cost: f64,
    pub budget_id: u32,
    pub budget: u64,
    pub over: bool,
    pub servers: HashMap<String, BudgetOverTreeServer>,
    pub flavors: HashMap<String, f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BudgetOverTreeProject {
    pub cost: f64,
    pub budget_id: u32,
    pub budget: u64,
    pub over: bool,
    pub users: HashMap<String, BudgetOverTreeUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavors: Option<HashMap<String, f64>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BudgetOverTree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    pub projects: HashMap<String, BudgetOverTreeProject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavors: Option<HashMap<String, f64>>,
}

pub struct BudgetOverTreeApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct BudgetOverTreeRequest {
    url: String,
    client: Rc<Client>,

    all: bool,
    project: Option<u32>,
    user: Option<u32>,
    end: Option<DateTime<FixedOffset>>,
}

impl BudgetOverTreeRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            all: false,
            project: None,
            user: None,
            end: None,
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

    pub fn send(&self) -> Result<BudgetOverTree, ApiError> {
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

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.end = Some(end);
        self
    }
}

impl BudgetOverTreeApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> BudgetOverTreeApi {
        BudgetOverTreeApi {
            url: format!("{}/budgeting/budgetovertree/", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn get(&self) -> BudgetOverTreeRequest {
        BudgetOverTreeRequest::new(&self.url, &self.client)
    }
}