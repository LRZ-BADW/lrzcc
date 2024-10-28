use actix_web::web::{
    delete,
    // get,
    patch,
    post,
    scope,
};
use actix_web::Scope;
use serde::Deserialize;

mod create;
use create::user_budget_create;
// mod list;
// use list::user_budget_list;
// mod get;
// use get::user_budget_get;
mod modify;
use modify::user_budget_modify;
mod delete;
use delete::user_budget_delete;

pub fn user_budgets_scope() -> Scope {
    scope("/userbudgets")
        .route("/", post().to(user_budget_create))
        // .route("", get().to(user_budget_list))
        // .route("/{user_budget_id}", get().to(user_budget_get))
        // TODO: what about PUT?
        .route("/{user_budget_id}/", patch().to(user_budget_modify))
        .route("/{user_budget_id}/", delete().to(user_budget_delete))
}

// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
struct UserBudgetIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    user_budget_id: u32,
}
