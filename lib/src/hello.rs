use std::rc::Rc;

use avina_wire::hello::Hello;
use reqwest::{blocking::Client, Method, StatusCode};

use crate::{
    common::{request, SerializableNone},
    error::ApiError,
};

pub struct HelloApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl HelloApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> HelloApi {
        HelloApi {
            url: format!("{base_url}/hello"),
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
