use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use lrzcc_wire::hello::Hello;
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use std::rc::Rc;

pub struct HelloApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl HelloApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> HelloApi {
        HelloApi {
            url: format!("{}/hello", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn admin(&self) -> Result<Hello, ApiError> {
        request(
            &self.client,
            Method::GET,
            format!("{}/admin", self.url).as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }

    pub fn user(&self) -> Result<Hello, ApiError> {
        request(
            &self.client,
            Method::GET,
            self.url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }
}
