use crate::resources::FlavorGroupMinimal;
use crate::user::UserMinimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub user_class: u32,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={}", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ProjectMinimal {
    pub id: u32,
    pub name: String,
    pub user_class: u32,
}

impl Display for ProjectMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ProjectDetailed {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub user_class: u32,
    // TODO rethink list output in detailed structs:
    // maybe we could have only the first few entries followed by ...
    // in the output
    #[tabled(skip)]
    pub users: Vec<UserMinimal>,
    #[tabled(skip)]
    pub flavor_groups: Vec<FlavorGroupMinimal>,
}

impl Display for ProjectDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={}", self.id, self.name))
    }
}

// TODO can we merge this with ProjectDetailed via some enum
// in the project field
#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct ProjectCreated {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub user_class: u32,
    #[tabled(skip)]
    pub users: Vec<u32>,
    #[tabled(skip)]
    pub flavor_groups: Vec<u32>,
}

impl Display for ProjectCreated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={}", self.id, self.name))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectCreateData {
    pub name: String,
    pub openstack_id: String, // UUIDv4
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub user_class: Option<u32>,
}

impl ProjectCreateData {
    pub fn new(name: String, openstack_id: String) -> Self {
        Self {
            name,
            openstack_id,
            user_class: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openstack_id: Option<String>, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_class: Option<u32>,
}

impl ProjectModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            openstack_id: None,
            user_class: None,
        }
    }
}
