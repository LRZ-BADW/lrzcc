use crate::common::{request, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use chrono::{DateTime, FixedOffset};
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ServerCost {
    pub total: f64,
}

#[derive(Debug)]
pub struct ServerCostRequest {
    url: String,
    client: Rc<Client>,

    begin: Option<DateTime<FixedOffset>>,
    end: Option<DateTime<FixedOffset>>,
}

impl ServerCostRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            begin: None,
            end: None,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(begin) = self.begin {
            params.push(("begin", begin.to_rfc3339().to_string()));
        } else if let Some(end) = self.end {
            params.push(("end", end.to_rfc3339().to_string()));
        }
        params
    }

    pub fn begin(&mut self, begin: DateTime<FixedOffset>) -> &mut Self {
        self.begin = Some(begin);
        self
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.end = Some(end);
        self
    }

    pub fn send(&self) -> Result<ServerCost, ApiError> {
        let url = Url::parse_with_params(self.url.as_str(), self.params())
            .context("Could not parse URL GET parameters.")?;
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }
}

pub struct ServerCostApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl ServerCostApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ServerCostApi {
        ServerCostApi {
            url: format!("{}/accounting/servercost/", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn get(&self) -> ServerCostRequest {
        ServerCostRequest::new(self.url.as_str(), &self.client)
    }
}
