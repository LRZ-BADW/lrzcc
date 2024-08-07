use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ServerCost {
    pub total: f64,
}

pub struct ServerCostApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl ServerCostApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ServerCostApi {
        ServerCostApi {
            url: format!("{}/accounting/servercost/", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn get(&self) -> Result<ServerCost, ApiError> {
        request(
            &self.client,
            Method::GET,
            self.url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }
}
