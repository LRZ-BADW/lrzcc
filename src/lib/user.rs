use crate::common::request;
use crate::error::ApiError;
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::rc::Rc;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct User {
    id: u32,
    name: String,
    openstack_id: String, // UUIDv4 without dashes
    project: u32,
    project_name: String,
    role: u32,
    is_staff: bool,
    is_active: bool,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User(id={}, name={}", self.id, self.name))
    }
}

pub struct UserApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl UserApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> UserApi {
        UserApi {
            url: format!("{}/user/users", base_url),
            client: Rc::clone(client),
        }
    }

    // TODO we probably need some form of request builder for arguments
    pub fn list(&self) -> Result<Vec<User>, ApiError> {
        request(
            &self.client,
            Method::GET,
            format!("{}/", self.url).as_str(),
            StatusCode::OK,
        )
    }
}
