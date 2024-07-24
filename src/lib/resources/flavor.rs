use crate::common::{display_option, request, request_bare, SerializableNone};
use crate::error::ApiError;
use crate::resources::FlavorGroupMinimal;
use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct Flavor {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4
    #[tabled(display_with = "display_option")]
    pub group: Option<u32>,
    #[tabled(display_with = "display_option")]
    group_name: Option<String>,
    pub weight: u32,
}

impl Display for Flavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Flavor(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorMinimal {
    pub id: u32,
    pub name: String,
}

// TODO maybe rethink the Display implementations
impl Display for FlavorMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Flavor(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorDetailed {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4
    #[tabled(display_with = "display_option")]
    pub group: Option<FlavorGroupMinimal>,
    #[tabled(display_with = "display_option")]
    pub group_name: Option<String>,
    pub weight: u32,
}

impl Display for FlavorDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Flavor(id={}, name={})", self.id, self.name))
    }
}

pub struct FlavorApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorListRequest {
    url: String,
    client: Rc<Client>,

    all: bool,
    group: Option<u32>,
}

impl FlavorListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            all: false,
            group: None,
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
        }
        params
    }

    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }

    pub fn group(&mut self, group: u32) -> &mut Self {
        self.group = Some(group);
        self
    }

    pub fn send(&self) -> Result<Vec<Flavor>, ApiError> {
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

#[derive(Clone, Debug, Serialize)]
struct FlavorCreateData {
    name: String,
    openstack_id: String, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<u32>,
}

impl FlavorCreateData {
    fn new(name: String, openstack_id: String) -> Self {
        Self {
            name,
            openstack_id,
            group: None,
            weight: None,
        }
    }
}

pub struct FlavorCreateRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorCreateData,
}

impl FlavorCreateRequest {
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        name: String,
        openstack_id: String,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorCreateData::new(name, openstack_id),
        }
    }

    pub fn group(&mut self, group: u32) -> &mut Self {
        self.data.group = Some(group);
        self
    }

    pub fn weight(&mut self, weight: u32) -> &mut Self {
        self.data.weight = Some(weight);
        self
    }

    pub fn send(&self) -> Result<FlavorDetailed, ApiError> {
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
struct FlavorModifyData {
    id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    openstack_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<Option<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<u32>,
}

impl FlavorModifyData {
    fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            openstack_id: None,
            group: None,
            weight: None,
        }
    }
}

pub struct FlavorModifyRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorModifyData,
}

impl FlavorModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorModifyData::new(id),
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.data.name = Some(name);
        self
    }

    pub fn openstack_id(&mut self, openstack_id: String) -> &mut Self {
        self.data.openstack_id = Some(openstack_id);
        self
    }

    pub fn group(&mut self, group: u32) -> &mut Self {
        self.data.group = Some(Some(group));
        self
    }

    pub fn no_group(&mut self) -> &mut Self {
        self.data.group = Some(None);
        self
    }

    pub fn weight(&mut self, weight: u32) -> &mut Self {
        self.data.weight = Some(weight);
        self
    }

    pub fn send(&self) -> Result<Flavor, ApiError> {
        request(
            &self.client,
            Method::PATCH,
            &self.url,
            Some(&self.data),
            StatusCode::OK,
        )
    }
}

impl FlavorApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorApi {
        FlavorApi {
            url: format!("{}/resources/flavors", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorListRequest {
        FlavorListRequest::new(self.url.as_ref(), &self.client)
    }

    pub fn get(&self, id: u32) -> Result<FlavorDetailed, ApiError> {
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
        name: String,
        openstack_id: String,
    ) -> FlavorCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        FlavorCreateRequest::new(url.as_ref(), &self.client, name, openstack_id)
    }

    pub fn modify(&self, id: u32) -> FlavorModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        FlavorModifyRequest::new(url.as_ref(), &self.client, id)
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
