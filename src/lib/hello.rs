use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct Hello {
    pub message: String,
}

impl Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

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
