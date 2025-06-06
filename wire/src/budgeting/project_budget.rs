use std::fmt::Display;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tabled::Tabled;

use crate::common::is_false;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq, FromRow)]
pub struct ProjectBudget {
    #[sqlx(try_from = "i32")]
    pub id: u32,
    #[sqlx(try_from = "i32")]
    pub project: u32,
    pub project_name: String,
    pub year: u32,
    pub amount: u32,
}

impl Display for ProjectBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ProjectBudget(id={})", self.id))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectBudgetListParams {
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
    pub year: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectBudgetOverParams {
    pub end: Option<DateTime<FixedOffset>>,
    pub budget: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
    pub detail: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectBudgetCreateData {
    pub project: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

impl ProjectBudgetCreateData {
    pub fn new(project: u32) -> Self {
        Self {
            project,
            year: None,
            amount: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectBudgetModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(skip_serializing_if = "is_false")]
    pub force: bool,
}

impl ProjectBudgetModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            amount: None,
            force: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct ProjectBudgetOverSimple {
    pub budget_id: u32,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct ProjectBudgetOverDetail {
    pub budget_id: u32,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
    pub cost: f64,
    pub budget: u32,
}
