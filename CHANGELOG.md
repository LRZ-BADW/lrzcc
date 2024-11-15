# Changelog
This is the combined changelog of all contained `lrzcc` crates.

## [Unreleased]
- api: add migrations for remaining active tables
- api: add accounting module
- api: add quota module
- api: flavor pricing module
- api: add budgeting module
- api: add accounting::server_state_delete endpoint
- api: add resources::flavor_group_delete endpoint
- api: add resources::flavor_delete endpoint
- api: add quota::flavor_quota_delete endpoint
- api: add pricing::flavor_price_delete endpoint
- api: add budgeting::project_budget_delete endpoint
- api: add budgeting::user_budget_delete endpoint
- api: add accounting::server_state_create endpoint
- api: add database module for shared database functions
- api: add quota::flavor_quota_create endpoint
- api: add resources::flavor_group_create endpoint
- api: add resources::flavor_create endpoint
- api: add pricing::flavor_price_create endpoint
- wire: derive Deserialize for FlavorPriceCreateData
- api: add budgeting::project_budget_create endpoint
- wire: derive Deserialize for UserBudgetCreateData
- api: add budgeting::user_budget_create endpoint
- api: add resources::select_flavor_group_from_db to database module
- api: add resources::flavor_group_modify endpoint
- deb: bump uuid from 1.10.0 to 1.11.0
- deb: bump anyhow from 1.0.89 to 1.0.90
- deb: bump tracing-actix-web from 0.7.13 to 0.7.14
- deb: bump serde_json from 1.0.128 to 1.0.132
- deb: bump serde from 1.0.210 to 1.0.211
- wire: derive FromRow for Flavor and make group_name field public
- api: add resources::select_flavor_from_db to database module
- api: add resources::flavor_modify endpoint
- api: add database::pricing::flavor_price submodule
- wire: derive Deserialize for FlavorPriceModifyData
- api: add flavor_price_modify endpoint
- TODO: add remaining crud endpoints for all new modules
- TODO: add tests for all new endpoints
- deb: bump config from 0.14.0 to 0.14.1
- wire: derive FromRow for ProjectBudget and UserBudget
- wire: derive Deserialize for ProjectBudgetModifyData and UserBudgetModifyData
- api: add database::budgeting::project/user_budget submodule with helpers
- api: add simplified budgeting::project_budget_modify endpoint
- api: add simplified budgeting::user_budget_modify endpoint
- wire: derive FromRow for FlavorQuota
- api: add database::quota submodule with helper functions
- api: minor naming fixes in user and pricing modules
- api: add simplified quota::flavor_quota_modify endpoint
- api: add database::accounting submodule with helper functions
- api: add simplified accounting::server_state_modify endpoint
- api: implement ResponseError for NotFoundOrUnexpectedApiError
- api: add simplified server_state_get endpoint
- api: add select_project_minimal_from_db function
- api: add select_minimal_flavors_by_group_from_db function
- wire: derive FromRow for FlavorMinimal
- api: add simplified flavor_group_get endpoint
- api: require admin user for server_state_get
- api: require admin user for flavor_group_get
- api: add select_flavor_detail_from_db function
- api: add simplified flavor_get endpoint
- api: add simplified flavor_quota_get endpoint
- api: add simplified flavor_price_get endpoint
- api: add simplified project_budget_get
- api: add simplified user_budget_get endpoint
- deb: bump reqwest from 0.12.8 to 0.12.9
- deb: bump anyhow from 1.0.90 to 1.0.93
- deb: bump serde from 1.0.211 to 1.0.214
- deb: bump thiserror from 1.0.64 to 2.0.0
- deb: bump tokio from 1.40.0 to 1.41.1
- api: move all select functions from routes to database module
- lib,wire: move ServerStateListParams from lib to wire
- api: add server state select functions to database module
- api: add simplified server_state_list endpoint
- lib,wire: move FlavorGroupListParams from lib to wire
- lib,wire: move FlavorListParams from lib to wire
- lib,wire: move FlavorQuotaListParams from lib to wire
- lib,wire: move ProjectBudgetListParams from lib to wire
- lib,wire: move UserBudgetListParams from lib to wire
- api: add project budget select functions to database module
- api: add user budget select functions to database module
- api: add flavor price select functions to database module
- api: add flavor quota select functions to database module

## [lrzcc-cli-v1.3.0] - 2024-10-08

### Features
- point user commands except import to rust api as well

### Dependencies
- bump wire from 1.1 to 1.2
- bump lib from 1.2 to 1.3
- bump clap version from 4.5.18 to 4.5.20

## [lrzcc-test-v0.2.0] - 2024-10-08

### Features
- add tests for all user endpoints but import
- add TestUser/Project and TestApp.setup_test_user/project
- simplify assertions by using PartialEq implementations
- add tests for master user authorization on user and project endpoints
- deactivate admin user insert in test setup

### Dependencies
- bump wire from 1.1 to 1.2
- bump api from 0.2 to 0.3
- bump lib from 1.2 to 1.3
- test: bump reqwest version from 0.12.7 to 0.12.8
- test: bump once_cell from 1.20.1 to 1.20.2

## [lrzcc-lib-v1.3.0] - 2024-10-08

### Features
- use User instead of UserCreated for UserApi::create call
- revise UserCreateRequest for new UserCreateData
- revise UserListRequest to use UserListParams

### Dependencies
- bump wire from 1.1 to 1.2
- bump reqwest version from 0.12.7 to 0.12.8

## [lrzcc-api-v0.3.0] - 2024-10-08

### Features
- add authorization module and move require_admin_user there
- add require_master_user to authorization module
- add user me, create, delete, get, list, and modify endpoints
- implement proper master user access to user get and list endpoint
- make user and project create submodules public
- add ApplicationSettings.insert_user_admin
- insert admin user and project on application start when set

### Dependencies
- bump wire from 1.1 to 1.2
- bump once_cell from 1.20.1 to 1.20.2

## [lrzcc-wire-v1.2.0] - 2024-10-08

### Features
- remove UserCreated
- make UserCreateData.is_staff/is_active Options
- derive FromRow for User, UserDetailed, and ProjectMinimal
- add UserListParams
- impl PartialEq for all response structs
- implement inter-type PartialEqs for User and Project structs

## [lrzcc-cli-v1.2.1] - 2024-09-30

### Features
- use Rust API for project commands by default

## [lrzcc-cli-v1.2.0] - 2024-09-30

### Dependencies
- bump lib from 1.1 to 1.2
- bump wire from 1.0 to 1.1

## [lrzcc-test-v0.1.0] - 2024-09-30

### Features
- add test crate for shared test helpers and cross-crate testing
- add test/tests with two library e2e tests for hello endpoint
- revise project list tests to use all parameter
- split project tests into submodules and add more tests
- move api/tests/helpers to test crate

## [lrzcc-lib-v1.2.0] - 2024-09-30

### Features
- refactor to use Project for project create response
- use wire.project.ProjectListParams in project list with serde_urlencoded

## [lrzcc-api-v0.2.0] - 2024-09-30

### Features
- move api/tests/helpers to test crate
- move endpoint scope creation to respective modules
- add not_found(_error) and default_service

#### Implement Project Endpoint
- add rudimentary project endpoint
- split project endpoint into submodules
- move require_admin_user from middleware to handlers
- add hierarchical api errors
- split of query functions from all project handlers
- add proper error handling to all project handlers
- implement all and user class filters for project list
- implement limited normal user access for project get
- fill users and flavor_groups field in project get handler
- refactor to use Project for project create response

### Dependencies
- bump secrecy from 0.10.1 to 0.10.2
- bump thiserror from 1.0.63 to 1.0.64
- bump once_cell from 1.20.0 to 1.20.1

## [lrzcc-wire-v1.1.0] - 2024-09-30

### Features
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
