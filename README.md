# lrzcc
API bindings, CLI application and partial API-backend written in Rust for
LRZ-specific features of the Openstack-based LRZ Compute Cloud,
[https://cc.lrz.de](https://cc.lrz.de), first and foremost the budgeting system.

## Crates
- [api](api): partial rewrite of the API server application
- [cli](cli): full rewrite of the CLI application
- [lib](lib): client-side API-binding library
- [test](test): shared test helpers and end-to-end tests
- [wire](wire): library for shared data structures used for API communication
