use anyhow::Context;
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::rc::Rc;

mod common;
pub mod error;
mod hello;
mod project;
mod user;

use error::ApiError;
use hello::HelloApi;
use project::ProjectApi;
use user::UserApi;

pub struct Api {
    // url: Rc<str>,
    // token: String,
    // client: Rc<Client>,
    pub hello: HelloApi,
    pub project: ProjectApi,
    pub user: UserApi,
}

impl Api {
    pub fn new(
        url: String,
        token: String,
        impersonate: Option<u32>,
    ) -> Result<Api, ApiError> {
        let mut headers = HeaderMap::new();
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "X-Auth-Token",
            HeaderValue::from_str(token.trim())
                .context("Failed to create token header value")?,
        );
        if let Some(impersonate) = impersonate {
            headers.insert(
                "X-Impersonate",
                HeaderValue::from_str(format!("{impersonate}").as_str())
                    .context("Failed to create impersonate header value")?,
            );
        }
        let client = Rc::new(
            ClientBuilder::new()
                .default_headers(headers)
                .build()
                .context("Failed to build http client")?,
        );
        let hello = HelloApi::new(&url, &client);
        let project = ProjectApi::new(&url, &client);
        let user = UserApi::new(&url, &client);
        Ok(Api {
            hello,
            project,
            user,
        })
    }
}
