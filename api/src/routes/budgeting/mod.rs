use actix_web::web::scope;
use actix_web::Scope;

mod project_budget;
use project_budget::project_budgets_scope;
mod user_budget;
use user_budget::user_budgets_scope;

pub fn budgeting_scope() -> Scope {
    scope("/budgeting")
        .service(project_budgets_scope())
        .service(user_budgets_scope())
}
