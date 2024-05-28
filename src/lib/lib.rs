use anyhow::Context;
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::rc::Rc;

mod common;
pub mod error;
pub mod hello;

use error::ApiError;
use hello::HelloApi;

pub struct Api {
    // url: Rc<str>,
    // token: String,
    // client: Rc<Client>,
    pub hello: HelloApi,
}

impl Api {
    // TODO add impersonation
    pub fn new(url: String, token: String) -> Result<Api, ApiError> {
        let mut headers = HeaderMap::new();
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let value = HeaderValue::from_str(token.trim())
            .context("Failed to create token header value")?;
        headers.insert("X-Auth-Token", value);
        let client = Rc::new(
            ClientBuilder::new()
                .default_headers(headers)
                .build()
                .context("Failed to build http client")?,
        );
        let hello = HelloApi::new(&url, &client);
        Ok(Api { hello })
    }
}
