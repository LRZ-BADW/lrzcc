# avina-ui
Dioxus web UI for LRZ-specific features of the Openstack-based
of the LRZ Compute Cloud, [https://cc.lrz.de](https://cc.lrz.de).

## Development

### Requirements
You need to have `dioxus-cli` and `wasm-bindgen-cli` installed.

### Run
Make sure the API runs locally, see [here](../api/README.md) and then execute:

```bash
scripts/run_ui.sh
```

This spawns the web UI on `http://localhost:8080`.
**WARNING**: This is not fully functional like this, though, continue
[here](../wrapper/README.md).
