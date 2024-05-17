use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use std::fmt::Debug;
use std::rc::Rc;

mod hello;
use hello::HelloApi;

#[derive(Deserialize)]
pub(crate) struct ErrorResponse {
    pub(crate) detail: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    ResponseError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct Api {
    // url: Rc<str>,
    // token: String,
    // client: Rc<Client>,
    pub hello: HelloApi,
}

impl Api {
    pub fn new(url: String, token: String) -> Api {
        let mut headers = HeaderMap::new();
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let value = match HeaderValue::from_str(token.trim()) {
            Ok(value) => value,
            Err(e) => {
                println!("Error: {}", e);
                HeaderValue::from_static("")
            }
        };
        headers.insert("X-Auth-Token", value);
        let client = Rc::new(
            ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
        );
        let hello = HelloApi::new(&url, &client);
        Api { hello }
    }
}
