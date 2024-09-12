use crate::common::{is_false, is_true};
use crate::user::ProjectMinimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub project: u32,
    pub project_name: String,
    pub role: u32,
    pub is_staff: bool,
    pub is_active: bool,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User(id={}, name={}", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserMinimal {
    pub id: u32,
    pub name: String,
}

impl Display for UserMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User(id={}, name={}", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserDetailed {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub project: ProjectMinimal,
    pub project_name: String,
    pub role: u32,
    pub is_staff: bool,
    pub is_active: bool,
}

// TODO can we merge this with UserDetailed via some enum
// in the project field
#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserCreated {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub project: u32,
    pub project_name: String,
    pub role: u32,
    pub is_staff: bool,
    pub is_active: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct UserImport {
    pub new_project_count: u32,
    pub new_user_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserCreateData {
    pub name: String,
    pub openstack_id: String, // UUIDv4
    // TODO can't this be optional?
    pub project: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    // this could be an enum right
    pub role: Option<u32>,
    #[serde(skip_serializing_if = "is_false")]
    pub is_staff: bool,
    #[serde(skip_serializing_if = "is_true")]
    pub is_active: bool,
}

impl UserCreateData {
    pub fn new(name: String, openstack_id: String, project: u32) -> Self {
        Self {
            name,
            openstack_id,
            project,
            role: None,
            is_staff: false,
            is_active: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openstack_id: Option<String>, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl UserModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            openstack_id: None,
            project: None,
            role: None,
            is_staff: None,
            is_active: None,
        }
    }
}