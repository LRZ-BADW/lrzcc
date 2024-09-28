# Changelog
This is the combined changelog of all contained `lrzcc` crates.

## [Unreleased]

### lrzcc-api and lrzcc-test
- test: add test crate for shared test helpers and cross-crate testing
- api,test: move api/tests/helpers to test crate
- test: add test/tests with two library e2e tests for hello endpoint
- api: move endpoint scope creation to respective modules
- api: add not_found(_error) and default_service
- api: add rudimentary project endpoint
- api: split project endpoint into submodules
- api: move require_admin_user from middleware to handlers
- api: add hierarchical api errors
- api: split of query functions from all project handlers
- api: add proper error handling to all project handlers
- api: bump secrecy from 0.10.1 to 0.10.2
- api: bump thiserror from 1.0.63 to 1.0.64
- wire: remove ProjectCreated
- lib/api: refactor to use Project for project create response
- wire: add ProjectListParams
- lib: use wire.project.ProjectListParams in project list with serde_urlencoded
- api: implement all and user class filters for project list
- test: revise project list tests to use all parameter
- test: split project tests into submodules and add more tests

## [lrzcc-cli-v1.1.2] - 2024-09-24

### Fixes
- revise to parse ProjectRetrieved in project get command

### Dependencies
- bump clap from 4.5.17 to 4.5.18

## [lrzcc-lib-v1.1.1] - 2024-09-24

### Fixes
- revise to parse ProjectRetrieved during ProjectApi.get_project

### Dependencies
- bump thiserror from 1.0.63 to 1.0.64

### Testing
- add e2e tests for hello and project modules

## [lrzcc-wire-v1.0.1] - 2024-09-24

### Fixes
- add project.ProjectRetrieved enum

## [lrzcc-cli-v1.1.1] - 2024-09-15

### Fixes
- Add aliases `show` to all `get` commands to comply with former Python CLI client.

### Documentation
- Update name in `cargo install` command in README.

## [lrzcc-api-v0.1.0] - 2024-09-15
Initial release of the `lrzcc-api` crate containing a partial rewrite of the API
server with authentication, the hello endpoint, proper testing, and basic
docker deployment.

## [lrzcc-cli-v1.1.0] - 2024-09-15
Initial release of the `lrzcc-cli` crate containing just the CLI application.

## [lrzcc-v1.1.0] - 2024-09-15
Initial release of the `lrzcc` crate containing just the Rust bindings.

## [lrzcc-wire-v1.0.0] - 2024-09-15
Initial release of the `lrzcc-wire` crate containing just the shared data
structures used for API communication.

## [v1.0.0] - 2024-08-16
Initial release of the `lrzcc` crate containing the new Rust bindings as well
as a full Rust rewrite of the CLI application.
