use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageOverviewInner {
    pub total: u64,
    pub used: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageOverviewInnerFloat {
    pub total: f64,
    pub used: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageOverview {
    pub vcpus: CloudUsageOverviewInner,
    pub ram: CloudUsageOverviewInner,
    pub gpus: CloudUsageOverviewInner,
    pub storage: CloudUsageOverviewInnerFloat,
    pub mwn_ips: CloudUsageOverviewInner,
    pub www_ips: CloudUsageOverviewInner,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageFlavorSlot {
    pub name: String,
    pub free: u32,
    pub total: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsageAggregate {
    pub name: String,
    pub title: String,
    pub flavors: Vec<CloudUsageFlavorSlot>,
}

// TODO how could we handle a table representation of this?
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudUsage {
    pub overview: CloudUsageOverview,
    pub lrz_flavor_slots: Vec<CloudUsageAggregate>,
    pub ach_flavor_slots: Vec<CloudUsageAggregate>,
    pub other_flavor_slots: Vec<CloudUsageAggregate>,
    pub datetime: String,
}

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
