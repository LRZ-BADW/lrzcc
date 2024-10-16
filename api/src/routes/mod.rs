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

// TODO: modify endpoints for
// - accounting::server_state
// - resources::flavor_group
// - resources::flavor
// - quota::flavor_quota
// - pricing::flavor_price
// - budgeting::project_budget
// - budgeting::user_budget

// TODO: get endpoints for
// - accounting::server_state
// - resources::flavor_group
// - resources::flavor
// - quota::flavor_quota
// - pricing::flavor_price
// - budgeting::project_budget
// - budgeting::user_budget

// TODO: list endpoints for
// - accounting::server_state
// - resources::flavor_group
// - resources::flavor
// - quota::flavor_quota
// - pricing::flavor_price
// - budgeting::project_budget
// - budgeting::user_budget
