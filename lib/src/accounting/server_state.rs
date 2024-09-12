use crate::common::{request, request_bare, SerializableNone};
use crate::error::ApiError;
use anyhow::Context;
use chrono::{DateTime, FixedOffset};
use lrzcc_wire::accounting::{
    ServerState, ServerStateCreateData, ServerStateImport,
    ServerStateModifyData,
};
use reqwest::blocking::Client;
use reqwest::Url;
use reqwest::{Method, StatusCode};
use std::rc::Rc;

pub struct ServerStateApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct ServerStateListRequest {
    url: String,
    client: Rc<Client>,

    server: Option<String>,
    user: Option<u32>,
    project: Option<u32>,
    all: bool,
}

impl ServerStateListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            server: None,
            user: None,
            project: None,
            all: false,
        }
    }

    fn params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(server) = &self.server {
            params.push(("server", server.to_string()));
        } else if let Some(user) = self.user {
            params.push(("user", user.to_string()));
        } else if let Some(project) = self.project {
            params.push(("project", project.to_string()));
        } else if self.all {
            params.push(("all", "1".to_string()));
        }
        params
    }

    pub fn send(&self) -> Result<Vec<ServerState>, ApiError> {
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

    pub fn server(&mut self, server: &str) -> &mut Self {
        self.server = Some(server.to_string());
        self
    }

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.project = Some(project);
        self
    }

    pub fn all(&mut self) -> &mut Self {
        self.all = true;
        self
    }
}

pub struct ServerStateCreateRequest {
    url: String,
    client: Rc<Client>,

    data: ServerStateCreateData,
}

impl ServerStateCreateRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        begin: DateTime<FixedOffset>,
        instance_id: String, // UUIDv4
        instance_name: String,
        flavor: u32,
        status: String,
        user: u32,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ServerStateCreateData::new(
                begin,
                instance_id,
                instance_name,
                flavor,
                status,
                user,
            ),
        }
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.data.end = Some(end);
        self
    }

    pub fn send(&self) -> Result<ServerState, ApiError> {
        request(
            &self.client,
            Method::POST,
            &self.url,
            Some(&self.data),
            StatusCode::CREATED,
        )
    }
}

pub struct ServerStateModifyRequest {
    url: String,
    client: Rc<Client>,

    data: ServerStateModifyData,
}

impl ServerStateModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ServerStateModifyData::new(id),
        }
    }

    pub fn begin(&mut self, begin: DateTime<FixedOffset>) -> &mut Self {
        self.data.begin = Some(begin);
        self
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.data.end = Some(end);
        self
    }

    pub fn instance_id(&mut self, instance_id: String) -> &mut Self {
        self.data.instance_id = Some(instance_id);
        self
    }

    pub fn instance_name(&mut self, instance_name: String) -> &mut Self {
        self.data.instance_name = Some(instance_name);
        self
    }

    pub fn flavor(&mut self, flavor: u32) -> &mut Self {
        self.data.flavor = Some(flavor);
        self
    }

    pub fn status(&mut self, status: String) -> &mut Self {
        self.data.status = Some(status);
        self
    }

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.data.user = Some(user);
        self
    }

    pub fn send(&self) -> Result<ServerState, ApiError> {
        request(
            &self.client,
            Method::PATCH,
            &self.url,
            Some(&self.data),
            StatusCode::OK,
        )
    }
}

impl ServerStateApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ServerStateApi {
        ServerStateApi {
            url: format!("{}/accounting/serverstates", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> ServerStateListRequest {
        ServerStateListRequest::new(self.url.as_ref(), &self.client)
    }

    pub fn get(&self, id: u32) -> Result<ServerState, ApiError> {
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
        begin: DateTime<FixedOffset>,
        instance_id: String, // UUIDv4
        instance_name: String,
        flavor: u32,
        status: String,
        user: u32,
    ) -> ServerStateCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        ServerStateCreateRequest::new(
            url.as_ref(),
            &self.client,
            begin,
            instance_id,
            instance_name,
            flavor,
            status,
            user,
        )
    }

    pub fn modify(&self, id: u32) -> ServerStateModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}", self.url, id);
        ServerStateModifyRequest::new(url.as_ref(), &self.client, id)
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

    pub fn import(&self) -> Result<ServerStateImport, ApiError> {
        // TODO use Url.join
        let url = format!("{}/import/", self.url);
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
    }
}
