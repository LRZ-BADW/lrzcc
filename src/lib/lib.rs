use anyhow::Context;
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::rc::Rc;

mod accounting;
mod common;
pub mod error;
mod hello;
mod pricing;
mod resources;
mod user;

use accounting::ServerStateApi;
use error::ApiError;
use hello::HelloApi;
use pricing::FlavorPriceApi;
use resources::FlavorApi;
use resources::FlavorGroupApi;
use user::ProjectApi;
use user::UserApi;

pub struct Api {
    // url: Rc<str>,
    // token: String,
    // client: Rc<Client>,
    pub hello: HelloApi,
    pub project: ProjectApi,
    pub user: UserApi,
    pub flavor: FlavorApi,
    pub flavor_group: FlavorGroupApi,
    pub flavor_price: FlavorPriceApi,
    pub server_state: ServerStateApi,
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
        Ok(Api {
            hello: HelloApi::new(&url, &client),
            project: ProjectApi::new(&url, &client),
            user: UserApi::new(&url, &client),
            flavor: FlavorApi::new(&url, &client),
            flavor_group: FlavorGroupApi::new(&url, &client),
            flavor_price: FlavorPriceApi::new(&url, &client),
            server_state: ServerStateApi::new(&url, &client),
        })
    }
}
