use crate::common::request;
use crate::error::ApiError;
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct BudgetBulkCreate {
    pub new_user_budget_count: u32,
    pub new_project_budget_count: u32,
}

pub struct BudgetBulkCreateApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BudgetBulkCreateData {
    pub year: i32,
}

impl BudgetBulkCreateApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> BudgetBulkCreateApi {
        BudgetBulkCreateApi {
            url: format!("{}/budgeting/budgetbulkcreate/", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn run(&self, year: i32) -> Result<BudgetBulkCreate, ApiError> {
        request(
            &self.client,
            Method::POST,
            self.url.as_str(),
            Some(&BudgetBulkCreateData { year }),
            StatusCode::OK,
        )
    }
}
