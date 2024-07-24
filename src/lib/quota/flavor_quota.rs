use crate::common::{request, request_bare, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorQuota {
    pub id: u32,
    pub user: u32,
    pub username: String,
    pub quota: i64,
    pub flavor_group: u32,
    pub flavor_group_name: String,
}

impl Display for FlavorQuota {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "FlavorQuota(id={}, user={}, flavor_group={})",
            self.id, self.user, self.flavor_group
        ))
    }
}

pub struct FlavorQuotaApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorQuotaListRequest {
    url: String,
    client: Rc<Client>,

    all: bool,
    group: Option<u32>,
    user: Option<u32>,
}

impl FlavorQuotaListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            all: false,
            group: None,
            user: None,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        // TODO maybe flip the order here, since the most specific
        // should take precedence
        if self.all {
            params.push(("all", "1".to_string()));
        } else if let Some(group) = self.group {
            params.push(("flavorgroup", group.to_string()));
        } else if let Some(user) = self.user {
            params.push(("user", user.to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<FlavorQuota>, ApiError> {
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

    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }

    pub fn group(&mut self, group: u32) -> &mut Self {
        self.group = Some(group);
        self
    }

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.user = Some(user);
        self
    }
}

#[derive(Clone, Debug, Serialize)]
struct FlavorQuotaCreateData {
    flavor_group: u32,
    user: u32,
    // TODO: maybe use Option<i64> here
    quota: i64,
}

impl FlavorQuotaCreateData {
    fn new(flavor_group: u32, user: u32) -> Self {
        Self {
            flavor_group,
            user,
            quota: -1,
        }
    }
}

pub struct FlavorQuotaCreateRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorQuotaCreateData,
}

impl FlavorQuotaCreateRequest {
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        flavor_group: u32,
        user: u32,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorQuotaCreateData::new(flavor_group, user),
        }
    }

    pub fn quota(&mut self, quota: i64) -> &mut Self {
        self.data.quota = quota;
        self
    }

    pub fn send(&self) -> Result<FlavorQuota, ApiError> {
        request(
            &self.client,
            Method::POST,
            &self.url,
            Some(&self.data),
            StatusCode::CREATED,
        )
    }
}

#[derive(Clone, Debug, Serialize)]
struct FlavorQuotaModifyData {
    id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flavor_group: Option<u32>,
}

impl FlavorQuotaModifyData {
    fn new(id: u32) -> Self {
        Self {
            id,
            user: None,
            quota: None,
            flavor_group: None,
        }
    }
}

pub struct FlavorQuotaModifyRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorQuotaModifyData,
}

impl FlavorQuotaModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorQuotaModifyData::new(id),
        }
    }

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.data.user = Some(user);
        self
    }

    pub fn quota(&mut self, quota: i64) -> &mut Self {
        self.data.quota = Some(quota);
        self
    }

    pub fn flavor_group(&mut self, flavor_group: u32) -> &mut Self {
        self.data.flavor_group = Some(flavor_group);
        self
    }

    pub fn send(&self) -> Result<FlavorQuota, ApiError> {
        request(
            &self.client,
            Method::PATCH,
            &self.url,
            Some(&self.data),
            StatusCode::OK,
        )
    }
}

impl FlavorQuotaApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorQuotaApi {
        FlavorQuotaApi {
            url: format!("{}/quota/flavorquotas", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorQuotaListRequest {
        FlavorQuotaListRequest::new(self.url.as_ref(), &self.client)
    }

    pub fn get(&self, id: u32) -> Result<FlavorQuota, ApiError> {
        // TODO use Url.join
        let url = format!("{}/{}", self.url, id);
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }

    pub fn create(
        &self,
        flavor_group: u32,
        user: u32,
    ) -> FlavorQuotaCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        FlavorQuotaCreateRequest::new(
            url.as_ref(),
            &self.client,
            flavor_group,
            user,
        )
    }

    pub fn modify(&self, id: u32) -> FlavorQuotaModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        FlavorQuotaModifyRequest::new(url.as_ref(), &self.client, id)
    }

    pub fn delete(&self, id: u32) -> Result<(), ApiError> {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        request_bare(
            &self.client,
            Method::DELETE,
            url.as_str(),
            SerializableNone!(),
            StatusCode::NO_CONTENT,
        )?;
        Ok(())
    }
}
