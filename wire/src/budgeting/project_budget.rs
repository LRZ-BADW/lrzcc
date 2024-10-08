use crate::common::is_false;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct ProjectBudget {
    pub id: u32,
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

#[derive(Clone, Debug, Serialize)]
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
pub struct ProjectBudgetOver {
    pub budget_id: u32,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct ProjectBudgetDetail {
    pub budget_id: u32,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
    pub cost: f64,
    pub budget: u32,
}
