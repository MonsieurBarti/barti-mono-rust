# Merge OpenAPI in app and write the document

Type: task
Label: wayfinder:task
Blocked by: 06
Status: resolved


## Question

`app` merges cell routers and writes `docs/openapi/openapi.json`.

Start from `OpenApiRouter::with_openapi(AppApi::openapi())`. `info.title` is `Barti Freight`. `info.version` is `0.1.0`. One `servers` entry: `url` is `/`. Merge. Do not nest. Call `split_for_parts()` once, then layer middleware. `GET /health` stays on the axum router and off the spec.

Add extra binary `openapi` in the `app` crate. `cargo run -p app --bin openapi` writes pretty JSON with stable keys to `docs/openapi/openapi.json`. Commit that file. The `loads` slice contract in [the locked spec](../../hive-docs/spec.md) must be present. `app` does not serve the file. `migrate` and `serve` stay the two process entries.

## Answer

`app` starts from `OpenApiRouter::with_openapi(AppApi::openapi())`. It merges the loads router. It does not nest. `split_for_parts()` runs once, then middleware. `GET /health` stays on the axum router and off the spec. Extra binary `openapi` writes pretty JSON to `docs/openapi/openapi.json`. `migrate` and `serve` stay the two process entries. `app` does not serve the file.
