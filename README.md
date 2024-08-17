# lrzcc
API bindings and CLI application written in Rust for LRZ-specific features of
the Openstack-based LRZ Compute Cloud, [https://cc.lrz.de](https://cc.lrz.de), first and
foremost the budgeting system.

## Installation
Install the tool via cargo:
```bash
cargo install lrzcc
```

## Usage
As a general note, use the `-h/--help` to find out more about the specific
commands of the tool.

### API Access
As with the normal Openstack CLI client you need to source your Openstack RC
file to access the API. You can download it when clicking on you username
in the top-right corner in the webui and then "OpenStack RC File v3".

Source it via:
```bash
. di12abc-openrc.sh
```
replacing `di12abc` with you own username, and then enter your password.
Note: this stores you credentials in environment variables starting with
`OS_` for OpenStack.

### User Workflows

#### Get Own User
```bash
lrzcc user me
```
Note: role 1 indicates that you are a normal user, role 2 means you are a
master user.

#### Display Cloud Usage
```bash
lrzcc -f json usage
```
Note: the `-f json` tells the tool to simply output the JSON response from
the API.

#### List Flavor Prices
```bash
lrzcc flavor-price list
```

#### Calculate Own Consumption and Cost
```bash
lrzcc server-consumption
lrzcc server-cost
```

#### View User and Project Budget
```bash
lrzcc user-budget list
lrzcc project-budget list
```

#### Check of Budget is Over
```bash
lrzcc user-budget over -dc
```

#### Show Budget Over Tree
This hierarchical view also shows a breakdown of the cost down to the
individual servers and is what the webui uses:
```bash
lrzcc -f json budget-over-tree
```
Note: the `-f json` tells the tool to simply output the JSON response from
the API.

### Master User Workflows

#### List Own Project and Users
```bash
lrzcc project list
lrzcc user list -p <project_id/name>
```

#### List Budgets of Own Project
```bash
lrzcc user-budget list -p <project_id/name>
```

#### List Budget Over Status of Project's Users
```bash
lrzcc user-budget over -p <project_id/name> -dc
```

#### Show Budget Over Tree of Project's Users
This hierarchical view also shows a breakdown of the cost down to the
individual users and servers and is what the webui uses:
```bash
lrzcc -f json budget-over-tree -p <project_id/name>
```
Note: the `-f json` tells the tool to simply output the JSON response from
the API.

#### Modify Budgets
```bash
lrzcc user-budget modify <user_budget_id> -a <amount>
lrzcc project-budget modify <project_budget_id> -a <amount>
```
Note: you cannot set a budget below the already acrued costs or modify the
budget of a past year.

## Library
To use the library add the following to your `Cargo.toml` under `[dependencies]`:
```toml
lrzcc = 1
```
After that you create a `Token` and `Api` object to interact with the API:
```rust
use lrzcc::{Token, Api};

// let token = Token::from_str("abcdefg...").unwrap();
let token = Token::new(
                auth_url.as_str(),
                username.as_str(),
                password.as_str(),
                project_name.as_str(),
                user_domain_name.as_str(),
                project_domain_id.as_str(),
            ).unwrap();
let api = Api::new("https://cc.lrz.de:1337/api", token, None, None).unwrap();
println!("{:?}", api.user.me());
```
