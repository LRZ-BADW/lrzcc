# Changelog
This is the combined changelog of all contained `lrzcc` crates.

## [Unreleased]
...

## [lrzcc-api-v0.6.1] - 2024-12-29

### Fixes
- dynamically retrieve current year in sync_user_budgets_in_db
- correct typo in error message in sync_user_budgets_in_db

## [lrzcc-cli-v1.5.1] - 2024-12-29

### Fixes
- route user-budget sync and flavor/flavor-group modify to rust api

## [lrzcc-cli-v1.5.0] - 2024-12-29

### Features
- add user-budget sync command to bugdgeting module

### Dependencies
- bump lrzcc-wire from 1.4.0 to 1.5.0
- bump lrzcc from 1.5.0 to 1.6.0
- bump anyhow from 1.0.94 to 1.0.95
- bump serde_json from 1.0.133 to 1.0.134
- bump serde from 1.0.215 to 1.0.216
- bump colored from 2.1.0 to 2.2.0

## [lrzcc-lib-v1.6.0] - 2024-12-29

### Features
- add UserBudgetApi.sync function to budgeting module

### Fixes
- add user and end in BudgetOverTreeRequest.params in budgeting module

### Dependencies
- bump lrzcc-wire from 1.4.0 to 1.5.0
- bump anyhow from 1.0.94 to 1.0.95
- bump thiserror from 2.0.6 to 2.0.9
- bump reqwest from 0.12.9 to 0.12.10
- bump serde_json from 1.0.133 to 1.0.134
- bump serde from 1.0.215 to 1.0.216

## [lrzcc-api-v0.6.0] - 2024-12-29

### Features
- add user_budget_sync endpoint to budgeting module
- add sync_user_budgets_in_db function to database module

### Fixes
- correct query in select_maybe_project_minimal_from_db
- correct field types in select_maybe_flavor_group_from_db
- use left join in flavor select functions

### Dependencies
- bump lrzcc-wire from 1.4.0 to 1.5.0
- bump anyhow from 1.0.94 to 1.0.95
- bump thiserror from 2.0.6 to 2.0.9
- bump reqwest from 0.12.9 to 0.12.10
- bump serde_json from 1.0.133 to 1.0.134
- bump serde from 1.0.215 to 1.0.216
- bump config from 0.14.1 to 0.15.4

## [lrzcc-wire-v1.5.0] - 2024-12-29

### Features
- add UserBudgetSync struct to budgeting module

### Fixes
- add sqlx try_from and rename to Flavor fields in resource module

### Dependencies
- Bump serde from 1.0.215 to 1.0.216

## [lrzcc-test-v0.4.0] - 2024-12-29

### Tests
- add flavor modify tests to resource module
- add flavor modify tests to resources module

### Fixes
- correct get call in e2e_lib_flavor_price_delete_works in pricing module

### Dependencies
- bump lrzcc-wire from 1.4.0 to 1.5.0
- bump lrzcc from 1.5.0 to 1.6.0
- bump lrzcc-api from 0.5.0 to 0.6.0
- bump anyhow from 1.0.94 to 1.0.95
- bump reqwest from 0.12.9 to 0.12.10
- bump serde_json from 1.0.133 to 1.0.134

## Repository

### Docs
- add logo.png
- update readme with logo and badges
- remove version badge for test crate
- fix crate name in version badge link

### CI
- add workflow running zizmor to check workflows
- replace template injection by environment variable usage in test and lint workflows
- specify complete version on install action step in audit workflow
- replace install-action shorthand with full version in audit workflow
- set persist-credentials: false on all checkout steps
- change trigger from pull_request_target to pull_request in label workflow

### Dependencies
- bump taiki-e/install-action from 2.46.20 to 2.47.0
- bump taiki-e/install-action from 2.46.9 to 2.46.20
- bump taiki-e/install-action from 2.46.8 to 2.46.9

### Misc
- update sqlx offline query data
- increase mariadb connection limit in init_db.sh script

## [lrzcc-test-v0.3.0] - 2024-12-11

### Dependencies
- add chrono dependency
- add anyhow dependency
- bump chrono from 0.4.38 to 0.4.39
- run cargo update
- bump lrzcc-wire from 1.3 to 1.4
- bump lrzcc-lib from 1.4 to 1.5
- bump lrzcc-api from 0.4 to 0.5

### Tests
- add TestApp.setup_test_flavor
- add server state create tests
- reuse api::database::insert_flavor_into_db in test
- add TestApp.setup_test_server_state
- add server state delete tests
- add helper assert_equal_server_states
- use assert_equal_server_states in server state create tests
- add server_state_get tests
- fix some linting issues in server_state_get test
- adjust tests for new NotFoundErrors
- fix minor linting issues
- correct expected not found message in server_state_delete test
- use assert_equal_server_states in server_state_get tests
- revise expected not found error messages
- add equal_server_states and (assert_)contains_server_state
- add first few server_state_list tests
- add server_state_modify tests
- add TestApp.setup_test_server_state_with_server_id
- add e2e_lib_server_state_list_server_filter_works_across_projects_for_admin_user
- add e2e_lib_server_state_list_server_filter_stays_within_project_for_master_user
- e2e_lib_master_user_can_combine_server_state_list_filters
- e2e_lib_admin_user_can_combine_server_state_list_filters
- add flavor quota delete tests
- add flavor price delete tests
- add TestApp.setup_test_flavor_price function
- add flavor group delete tests
- fix typos in flavor delete test error messages
- add TestApp.setup_test_flavor_group
- add todo comment to project delete tests
- add flavor_delete tests
- add project_budget_delete tests
- add TestApp.setup_test_project_budget
- add user_budget_delete tests
- add TestApp.setup_test_user_budget

## [lrzcc-cli-v1.4.0] - 2024-12-11

### Features
- route server-state crud and other delete commands to Rust API

### Dependencies
- run cargo update
- bump tabled from 0.16.0 to 0.17.0
- bump clap from 4.5.22 to 4.5.23
- bump chrono from 0.4.38 to 0.4.39
- bump lrzcc-lib from 1.4 to 1.5
- bump lrzcc-wire from 1.3 to 1.4

## [lrzcc-lib-v1.5.0] - 2024-12-11

### Dependencies
- bump chrono from 0.4.38 to 0.4.39
- bump thiserror from 2.0.4 to 2.0.6
- run cargo update
- bump lrzcc-wire from 1.3 to 1.4

### Fixes
- add missing trailing slash in server_state_modify url

## [lrzcc-api-v0.5.0] - 2024-12-11

### Dependencies
- run cargo update
- bump tracing-subscriber from 0.3.18 to 0.3.19
- bump tracing from 0.1.40 to 0.1.41
- bump chrono from 0.4.38 to 0.4.39
- bump thiserror from 2.0.4 to 2.0.6
- bump lrzcc-wire from 1.3 to 1.4

### Database
- move insert_flavor_into_db to database module
- move insert_server_state_into_db to database module
- add missing sqlx try_froms to ServerStateRow id fields
- adjust all getters for new NotFoundErrors
- add select_server_states_by_server_and_project_from_db
- add select_server_states_by_server_and_user_from_db
- move insert_flavor_quota_into_db into database module
- move insert_flavor_price_into_db to database module
- move insert_flavor_group_into_db to database module
- move insert_project_budget_into_db to database module
- move insert_user_budget_into_db to database module

### Error
- match messages for all NotFoundError variants
- add NotFoundOnlyError with impls

### Endpoints
- revise getters for new NotFoundErrors
- remove done todo comment
- use require_master_user_or_return_not_found in user_get
- correct authorization check in server_state_get
- homogenize errors of server_state_list
- complete server_state_list endpoint

### Authorization
- add require_*_or_return_not_found functions

## [lrzcc-wire-v1.4.0] - 2024-12-11

### Dependencies
- bump chrono from 0.4.38 to 0.4.39
- run cargo update

## [lrzcc-test-v0.2.1] - 2024-11-22

### Dependencies
- bump wire from 1.2 to 1.3
- bump api from 0.3 to 0.4
- bump lib from 1.3 to 1.4
- bump serde from 1.0.210 to 1.0.214
- bump serde_json from 1.0.128 to 1.0.133
- bump tokio from 1.40.0 to 1.41.1
- bump reqwest from 0.12.8 to 0.12.9

## [lrzcc-cli-v1.3.1] - 2024-11-22

### Dependencies
- bump anyhow from 1.0.89 to 1.0.93
- bump serde from 1.0.210 to 1.0.214
- bump serde_json from 1.0.128 to 1.0.133

## [lrzcc-lib-v1.4.0] - 2024-11-22

### Refactors
- move ServerStateListParams from lib to wire
- move FlavorGroupListParams from lib to wire
- move FlavorListParams from lib to wire
- move FlavorQuotaListParams from lib to wire
- move ProjectBudgetListParams from lib to wire
- move UserBudgetListParams from lib to wire

### Dependencies
- bump anyhow from 1.0.89 to 1.0.93
- bump config from 0.14.0 to 0.14.1
- bump reqwest from 0.12.8 to 0.12.9
- bump serde from 1.0.210 to 1.0.214
- bump serde_json from 1.0.128 to 1.0.133
- bump thiserror from 1.0.64 to 2.0.0

## [lrzcc-api-v0.4.0] - 2024-11-22

### Features

#### General
- add migrations for remaining active tables
- add accounting, quota, pricing, budgeting modules
- implement ResponseError for NotFoundOrUnexpectedApiError

#### Database
- add database module for shared database functions
- add resources::select_flavor_group_from_db to database module
- add resources::select_flavor_from_db to database module
- add database::pricing::flavor_price submodule
- add database::budgeting::project/user_budget submodule with helpers
- add database::quota submodule with helper functions
- add database::accounting submodule with helper functions
- add select_project_minimal_from_db function
- add select_minimal_flavors_by_group_from_db function
- add select_flavor_detail_from_db function
- move all select functions from routes to database module
- add server state select functions to database module
- add project budget select functions to database module
- add user budget select functions to database module
- add flavor price select functions to database module
- add flavor quota select functions to database module
- add select_all_flavor_groups_from_db to database module
- add select_lrz_flavor_groups_from_db to database module
- add select_all_flavors_from_db to database module
- add select_lrz_flavors_from_db to database module
- add select_flavors_by_flavor_group_from_db to database module

#### Endpoints
- add accounting::server_state_delete endpoint
- add resources::flavor_group_delete endpoint
- add resources::flavor_delete endpoint
- add quota::flavor_quota_delete endpoint
- add pricing::flavor_price_delete endpoint
- add budgeting::project_budget_delete endpoint
- add budgeting::user_budget_delete endpoint
- add accounting::server_state_create endpoint
- add quota::flavor_quota_create endpoint
- add resources::flavor_group_create endpoint
- add resources::flavor_create endpoint
- add pricing::flavor_price_create endpoint
- add budgeting::project_budget_create endpoint
- add budgeting::user_budget_create endpoint
- add resources::flavor_group_modify endpoint
- add resources::flavor_modify endpoint
- add pricing::flavor_price_modify endpoint
- add simplified budgeting::project_budget_modify endpoint
- add simplified budgeting::user_budget_modify endpoint
- add simplified quota::flavor_quota_modify endpoint
- add simplified accounting::server_state_modify endpoint
- add simplified accounting::erver_state_get endpoint
- add simplified resources::flavor_group_get endpoint
- require admin user for accounting::server_state_get
- require admin user for resources::flavor_group_get
- add simplified resources::flavor_get endpoint
- add simplified quota::flavor_quota_get endpoint
- add simplified pricing::flavor_price_get endpoint
- add simplified budgeting::project_budget_get
- add simplified budgeting::user_budget_get endpoint
- add simplified accounting::server_state_list endpoint
- add budgeting::project_budget_list endpoint
- add budgeting::user_budget_list endpoint
- add pricing::flavor_price_list endpoint
- add quota::flavor_quota_list endpoint
- add resources::flavor_group_list endpoint
- add resources::flavor_list endpoint

### Fixes
- minor naming fixes in user and pricing modules
- fix typo in select_project_budget_by_user_from_db function name

### Dependencies
- bump anyhow from 1.0.89 to 1.0.93
- bump config from 0.14.0 to 0.14.1
- bump reqwest from 0.12.8 to 0.12.9
- bump serde from 1.0.210 to 1.0.214
- bump serde_json from 1.0.128 to 1.0.133
- bump thiserror from 1.0.64 to 2.0.0
- bump tokio from 1.40.0 to 1.41.1
- bump tracing-actix-web from 0.7.13 to 0.7.15

## [lrzcc-wire-v1.3.0] - 2024-11-22

### Features
- derive Deserialize for FlavorPriceCreateData
- derive Deserialize for UserBudgetCreateData
- derive FromRow for Flavor and make group_name field public
- derive Deserialize for FlavorPriceModifyData
- derive FromRow for ProjectBudget and UserBudget
- derive Deserialize for ProjectBudgetModifyData and UserBudgetModifyData
- derive FromRow for FlavorQuota
- derive FromRow for FlavorMinimal
- move ServerStateListParams from lib to wire
- move FlavorGroupListParams from lib to wire
- move FlavorListParams from lib to wire
- move FlavorQuotaListParams from lib to wire
- move ProjectBudgetListParams from lib to wire
- move UserBudgetListParams from lib to wire

### Dependencies
- bump serde from 1.0.210 to 1.0.214
- bump uuid from 1.10.0 to 1.11.0

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
