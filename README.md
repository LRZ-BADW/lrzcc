<div align="center">

# lrzcc

![](https://raw.githubusercontent.com/LRZ-BADW/lrzcc/main/logo.png)

API bindings, CLI application and partial API-backend written in Rust for
LRZ-specific features of the Openstack-based LRZ Compute Cloud,
[https://cc.lrz.de](https://cc.lrz.de), first and foremost the budgeting system.

[![audit](https://github.com/LRZ-BADW/lrzcc/actions/workflows/audit.yml/badge.svg)](https://github.com/LRZ-BADW/lrzcc/actions/workflows/audit.yml)
[![lint](https://github.com/LRZ-BADW/lrzcc/actions/workflows/lint.yml/badge.svg)](https://github.com/LRZ-BADW/lrzcc/actions/workflows/lint.yml)
[![test](https://github.com/LRZ-BADW/lrzcc/actions/workflows/test.yml/badge.svg)](https://github.com/LRZ-BADW/lrzcc/actions/workflows/test.yml)

![Crates.io](https://img.shields.io/crates/l/lrzcc?link=https://crates.io/crates/lrzcc)

</div>

## Crates
- [api](api): partial rewrite of the API server application ![crates.io](https://img.shields.io/crates/v/lrzcc-api?link=https://crates.io/crates/lrzcc-api)
- [cli](cli): full rewrite of the CLI application ![crates.io](https://img.shields.io/crates/v/lrzcc-cli?link=https://crates.io/crates/lrzcc-cli)
- [lib](lib): client-side API-binding library ![crates.io](https://img.shields.io/crates/v/lrzcc?link=https://crates.io/crates/lrzcc)
- [test](test): shared test helpers and end-to-end tests
- [wire](wire): library for shared data structures used for API communication ![crates.io](https://img.shields.io/crates/v/lrzcc-wire?link=https://crates.io/crates/lrzcc-wire)
