use crate::common::display_option;
use crate::resources::FlavorGroupMinimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::Display;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq, FromRow)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct FlavorImport {
    pub new_flavor_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlavorCreateData {
    pub name: String,
    pub openstack_id: String, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
}

impl FlavorCreateData {
    pub fn new(name: String, openstack_id: String) -> Self {
        Self {
            name,
            openstack_id,
            group: None,
            weight: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlavorModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openstack_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Option<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
}

impl FlavorModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            openstack_id: None,
            group: None,
            weight: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorUsage {
    pub user_id: u32,
    pub user_name: String,
    pub flavor_id: u32,
    pub flavor_name: String,
    #[tabled(display_with = "display_option")]
    pub flavorgroup_id: Option<u32>,
    #[tabled(display_with = "display_option")]
    pub flavorgroup_name: Option<String>,
    pub count: u32,
    pub usage: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorUsageAggregate {
    pub flavor_id: u32,
    pub flavor_name: String,
    #[tabled(display_with = "display_option")]
    pub flavorgroup_id: Option<u32>,
    #[tabled(display_with = "display_option")]
    pub flavorgroup_name: Option<String>,
    pub count: u32,
    pub usage: u32,
}
