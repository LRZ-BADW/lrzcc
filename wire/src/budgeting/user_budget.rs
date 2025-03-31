use std::fmt::Display;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

use crate::common::{display_option, is_false};

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq, FromRow)]
pub struct UserBudget {
    pub id: u32,
    pub user: u32,
    pub username: String,
    pub year: u32,
    pub amount: u32,
}

impl Display for UserBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("UserBudget(id={})", self.id))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserBudgetListParams {
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
    pub year: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserBudgetOverParams {
    pub end: Option<DateTime<FixedOffset>>,
    pub budget: Option<u32>,
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
    pub combined: Option<bool>,
    pub detail: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserBudgetCreateData {
    pub user: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

impl UserBudgetCreateData {
    pub fn new(user: u32) -> Self {
        Self {
            user,
            year: None,
            amount: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserBudgetModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(skip_serializing_if = "is_false")]
    pub force: bool,
}

impl UserBudgetModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            amount: None,
            force: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct UserBudgetOverSimple {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    pub over: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct UserBudgetOverCombined {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    #[tabled(display = "display_option")]
    pub project_budget_id: Option<u32>,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct UserBudgetOverDetail {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    pub over: bool,
    pub cost: f64,
    pub budget: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct UserBudgetOverCombinedDetail {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    #[tabled(display = "display_option")]
    pub project_budget_id: Option<u32>,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
    pub project_cost: f64,
    #[tabled(display = "display_option")]
    pub project_budget: Option<u32>,
    pub user_cost: f64,
    pub user_budget: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct UserBudgetSync {
    pub updated_budget_count: u32,
}
