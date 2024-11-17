mod accounting;
mod budgeting;
mod health_check;
mod hello;
mod pricing;
mod quota;
mod resources;
pub mod user;

pub use accounting::*;
pub use budgeting::*;
pub use health_check::*;
pub use hello::*;
pub use pricing::*;
pub use quota::*;
pub use resources::*;
pub use user::*;

// TODO: list endpoints for
// - resources::flavor_group
// - resources::flavor
// - quota::flavor_quota
// - pricing::flavor_price
// - budgeting::user_budget

// TODO: improve the following endpoints
// - budgeting::project_budget::modify
// - budgeting::user_budget::modify
// - pricing::flavor_price::modify
// - quota::flavor_quota::modify
// - accounting::server_state::modify
// - accounting::server_state::get
// - resources::flavor_group::get
// - resources::flavor::get
// - quota::flavor_quota::get
// - pricing::flavor_price::get
// - budgeting::project_budget::get
// - budgeting::user_budget::get
// - accounting::server_state::list
