# Changelog
This is the combined changelog of all contained `lrzcc` crates.

## [Unreleased]
...

## [lrzcc-test-v0.1.0] - 2024-09-30
- test: add test crate for shared test helpers and cross-crate testing
- test: add test/tests with two library e2e tests for hello endpoint
- test: revise project list tests to use all parameter
- test: split project tests into submodules and add more tests
- test: move api/tests/helpers to test crate

## [lrzcc-lib-v1.2.0] - 2024-09-30
- lib: refactor to use Project for project create response
- lib: use wire.project.ProjectListParams in project list with serde_urlencoded

## [lrzcc-api-v0.2.0] - 2024-09-30
- move api/tests/helpers to test crate
- move endpoint scope creation to respective modules
- add not_found(_error) and default_service
- add rudimentary project endpoint
- split project endpoint into submodules
- move require_admin_user from middleware to handlers
- add hierarchical api errors
- split of query functions from all project handlers
- add proper error handling to all project handlers
- bump secrecy from 0.10.1 to 0.10.2
- bump thiserror from 1.0.63 to 1.0.64
- implement all and user class filters for project list
- implement limited normal user access for project get
- fill users and flavor_groups field in project get handler
- refactor to use Project for project create response
- bump once_cell from 1.20.0 to 1.20.1

## [lrzcc-wire-v1.1.0] - 2024-09-30
- derive FromRow for Project, UserMinimal, FlavorGroupMinimal
- remove ProjectCreated
- add ProjectListParams

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
