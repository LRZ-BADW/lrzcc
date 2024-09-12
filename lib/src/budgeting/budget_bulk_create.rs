use crate::common::request;
use crate::error::ApiError;
use lrzcc_wire::budgeting::{BudgetBulkCreate, BudgetBulkCreateData};
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use std::rc::Rc;

pub struct BudgetBulkCreateApi {
    pub url: String,
    pub client: Rc<Client>,
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
