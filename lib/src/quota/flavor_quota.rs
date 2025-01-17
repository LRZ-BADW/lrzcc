use crate::common::{request, request_bare, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use lrzcc_wire::quota::{
    FlavorQuota, FlavorQuotaCheck, FlavorQuotaCreateData,
    FlavorQuotaListParams, FlavorQuotaModifyData,
};
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use std::rc::Rc;

pub struct FlavorQuotaApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorQuotaListRequest {
    url: String,
    client: Rc<Client>,

    params: FlavorQuotaListParams,
}

impl FlavorQuotaListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: FlavorQuotaListParams {
                all: None,
                group: None,
                user: None,
            },
        }
    }

    pub fn send(&self) -> Result<Vec<FlavorQuota>, ApiError> {
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }

    pub fn all(&mut self) -> &mut Self {
        self.params.all = Some(true);
        self
    }

    pub fn group(&mut self, group: u32) -> &mut Self {
        self.params.group = Some(group);
        self
    }

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.params.user = Some(user);
        self
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

pub struct FlavorQuotaCheckRequest {
    url: String,
    client: Rc<Client>,

    user: u32,
    flavor: u32,
    count: Option<u32>,
}

impl FlavorQuotaCheckRequest {
    pub fn new(url: &str, client: &Rc<Client>, user: u32, flavor: u32) -> Self {
        Self {
            url: format!("{}/check/", url),
            client: Rc::clone(client),

            user,
            flavor,
            count: None,
        }
    }

    pub fn count(&mut self, count: u32) -> &mut Self {
        self.count = Some(count);
        self
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        params.push(("user", self.user.to_string()));
        params.push(("flavor", self.flavor.to_string()));
        if let Some(count) = self.count {
            params.push(("flavorcount", count.to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<FlavorQuotaCheck, ApiError> {
        let url = Url::parse_with_params(self.url.as_str(), self.params())
            .context("Could not parse URL GET parameters.")?;
        request(
            &self.client,
            Method::GET,
            url.as_ref(),
            SerializableNone!(),
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

    pub fn check(&self, user: u32, flavor: u32) -> FlavorQuotaCheckRequest {
        FlavorQuotaCheckRequest::new(&self.url, &self.client, user, flavor)
    }
}
