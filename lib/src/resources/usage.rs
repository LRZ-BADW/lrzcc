use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use lrzcc_wire::resources::CloudUsage;
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use std::rc::Rc;

pub struct UsageApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl UsageApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> UsageApi {
        UsageApi {
            // TODO add the missing / that the end
            url: format!("{}/resources/usage", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn get(&self) -> Result<CloudUsage, ApiError> {
        request(
            &self.client,
            Method::GET,
            self.url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }
}
