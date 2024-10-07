use crate::common::is_false;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
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

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
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
pub struct UserBudgetOver {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    pub over: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct UserBudgetCombined {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    pub project_budget_id: u32,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct UserBudgetDetail {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    pub over: bool,
    pub cost: f64,
    pub budget: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct UserBudgetCombinedDetail {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    pub project_budget_id: u32,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
    pub project_cost: f64,
    pub project_budget: u32,
    pub user_cost: f64,
    pub user_budget: u32,
}
