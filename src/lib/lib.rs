use anyhow::Context;
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::rc::Rc;

mod common;
pub mod error;

#[cfg(feature = "accounting")]
mod accounting;
#[cfg(feature = "budgeting")]
mod budgeting;
#[cfg(feature = "hello")]
mod hello;
#[cfg(feature = "pricing")]
mod pricing;
#[cfg(feature = "quota")]
mod quota;
#[cfg(feature = "resources")]
mod resources;
#[cfg(feature = "user")]
mod user;

use error::ApiError;

#[cfg(feature = "accounting")]
use accounting::ServerStateApi;
#[cfg(feature = "budgeting")]
use budgeting::ProjectBudgetApi;
#[cfg(feature = "budgeting")]
use budgeting::UserBudgetApi;
#[cfg(feature = "hello")]
use hello::HelloApi;
#[cfg(feature = "pricing")]
use pricing::FlavorPriceApi;
#[cfg(feature = "quota")]
use quota::FlavorQuotaApi;
#[cfg(feature = "resources")]
use resources::FlavorApi;
#[cfg(feature = "resources")]
use resources::FlavorGroupApi;
#[cfg(feature = "resources")]
use resources::UsageApi;
#[cfg(feature = "user")]
use user::ProjectApi;
#[cfg(feature = "user")]
use user::UserApi;

pub struct Api {
    // url: Rc<str>,
    // token: String,
    // client: Rc<Client>,
    #[cfg(feature = "hello")]
    pub hello: HelloApi,
    #[cfg(feature = "user")]
    pub project: ProjectApi,
    #[cfg(feature = "user")]
    pub user: UserApi,
    #[cfg(feature = "resources")]
    pub flavor: FlavorApi,
    #[cfg(feature = "resources")]
    pub flavor_group: FlavorGroupApi,
    #[cfg(feature = "resources")]
    pub usage: UsageApi,
    #[cfg(feature = "pricing")]
    pub flavor_price: FlavorPriceApi,
    #[cfg(feature = "quota")]
    pub flavor_quota: FlavorQuotaApi,
    #[cfg(feature = "accounting")]
    pub server_state: ServerStateApi,
    #[cfg(feature = "budgeting")]
    pub project_budget: ProjectBudgetApi,
    #[cfg(feature = "budgeting")]
    pub user_budget: UserBudgetApi,
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
            #[cfg(feature = "hello")]
            hello: HelloApi::new(&url, &client),
            #[cfg(feature = "user")]
            project: ProjectApi::new(&url, &client),
            #[cfg(feature = "user")]
            user: UserApi::new(&url, &client),
            #[cfg(feature = "resources")]
            flavor: FlavorApi::new(&url, &client),
            #[cfg(feature = "resources")]
            flavor_group: FlavorGroupApi::new(&url, &client),
            #[cfg(feature = "resources")]
            usage: UsageApi::new(&url, &client),
            #[cfg(feature = "pricing")]
            flavor_price: FlavorPriceApi::new(&url, &client),
            #[cfg(feature = "quota")]
            flavor_quota: FlavorQuotaApi::new(&url, &client),
            #[cfg(feature = "accounting")]
            server_state: ServerStateApi::new(&url, &client),
            #[cfg(feature = "budgeting")]
            project_budget: ProjectBudgetApi::new(&url, &client),
            #[cfg(feature = "budgeting")]
            user_budget: UserBudgetApi::new(&url, &client),
        })
    }
}
