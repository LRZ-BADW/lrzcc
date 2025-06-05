//! **ATTENTION:** This has been renamed to [**avina-wire**](https://crates.io/crates/avina-wire).
pub mod common;
pub mod error;

#[cfg(feature = "accounting")]
pub mod accounting;
#[cfg(feature = "budgeting")]
pub mod budgeting;
#[cfg(feature = "hello")]
pub mod hello;
#[cfg(feature = "pricing")]
pub mod pricing;
#[cfg(feature = "quota")]
pub mod quota;
#[cfg(feature = "resources")]
pub mod resources;
#[cfg(feature = "user")]
pub mod user;
