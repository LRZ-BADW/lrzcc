use crate::common::request;
use crate::error::ApiError;
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageOverviewInner {
    total: u64,
    used: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageOverviewInnerFloat {
    total: f64,
    used: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageOverview {
    vcpus: CloudUsageOverviewInner,
    ram: CloudUsageOverviewInner,
    gpus: CloudUsageOverviewInner,
    storage: CloudUsageOverviewInnerFloat,
    mwn_ips: CloudUsageOverviewInner,
    www_ips: CloudUsageOverviewInner,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageFlavorSlot {
    name: String,
    free: u32,
    total: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageAggregate {
    name: String,
    title: String,
    flavors: Vec<CloudUsageFlavorSlot>,
}

// TODO how could we handle a table representation of this?
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsage {
    overview: CloudUsageOverview,
    lrz_flavor_slots: Vec<CloudUsageAggregate>,
    ach_flavor_slots: Vec<CloudUsageAggregate>,
    other_flavor_slots: Vec<CloudUsageAggregate>,
    datetime: String,
}

pub struct UsageApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl UsageApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> UsageApi {
        UsageApi {
            url: format!("{}/resources/usage", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn get(&self) -> Result<CloudUsage, ApiError> {
        request(&self.client, Method::GET, self.url.as_str(), StatusCode::OK)
    }
}
