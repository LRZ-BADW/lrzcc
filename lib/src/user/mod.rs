mod project;
#[allow(clippy::module_inception)]
mod user;

// TODO rethink the public export of minimal structs
pub use project::{ProjectApi, ProjectMinimal};
pub use user::{UserApi, UserMinimal};
