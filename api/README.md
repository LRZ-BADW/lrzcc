# lrzcc-api
Partial Rust-rewrite of the API server application for the Openstack-based
LRZ Compute Cloud, [https://cc.lrz.de](https://cc.lrz.de), first and foremost the budgeting
system.

## Development

### Running the API server locally
```bash
scripts/init.sh
source admin-openrc.sh
source scripts/config_env.sh

# optional: insert database dump
scripts/enter_db.sh
source lrz_budgeting.sql
quit

RUST_LOG=info cargo run --bin lrzcc-api | bunyan
```

### Calling the local API server
```bash
source admin-openrc.sh
cargo run --bin lrzcc -- -u http://localhost:8000/api -r http://localhost:8000/api user me
```
