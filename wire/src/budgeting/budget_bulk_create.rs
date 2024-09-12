use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BudgetBulkCreateData {
    pub year: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct BudgetBulkCreate {
    pub new_user_budget_count: u32,
    pub new_project_budget_count: u32,
}
