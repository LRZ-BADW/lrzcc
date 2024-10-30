use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::Display;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq, FromRow)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct FlavorQuotaCheck {
    pub underquota: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlavorQuotaCreateData {
    pub flavor_group: u32,
    pub user: u32,
    // TODO: maybe use Option<i64> here
    pub quota: i64,
}

impl FlavorQuotaCreateData {
    pub fn new(flavor_group: u32, user: u32) -> Self {
        Self {
            flavor_group,
            user,
            quota: -1,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlavorQuotaModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor_group: Option<u32>,
}

impl FlavorQuotaModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            user: None,
            quota: None,
            flavor_group: None,
        }
    }
}
