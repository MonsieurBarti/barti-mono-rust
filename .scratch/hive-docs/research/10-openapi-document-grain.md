# OpenAPI document merge (utoipa 5.5.0)

Probed 2026-09-12 in `/tmp/utoipa-merge-probe`. Repo untouched.

Pins that compiled: `utoipa` 5.5.0, `utoipa-axum` 0.2.0, `axum` 0.8.9. rustc 1.98.1.

## Cell export

A cell can return `OpenApiRouter` after `with_state`. `routes!(create_load)` compiles with no turbofish. State type on the returned router is `()`.

`split_for_parts()` then `OpenApi::merge` equals `OpenApiRouter::merge`. `OpenApi: PartialEq` was true.

`OpenApiRouter::nest("/prefix", cell)` rewrites operation paths to `/prefix/loads`. `merge` keeps `/loads`.

`OpenApiRouter::new()` without a later call that pins `S` does not compile. Write `OpenApiRouter::<()>::new()`.

## Duplicate schemas

`OpenApi::merge` is first-wins and silent. The losing cell's operation still `$ref`s `#/components/schemas/Problem`. `OpenApiRouter::merge` and `OpenApiRouter::nest` match that.

`OpenApiRouter::routes` on one router uses `schemas.extend`. That path is last-wins.

## Duplicate paths

`OpenApi::merge` keeps the first operation. No error.

`axum::Router::merge` panics: `Overlapping method route. Handler for \`POST /loads\` already exists`.

`OpenApiRouter::merge` panics at the axum step. The OpenAPI merge has already first-won.

## info and servers

`OpenApiRouter::new()` stamps utoipa-axum 0.2.0 crate metadata into `info` (`title` is `utoipa-axum`, `version` is `0.2.0`).

`OpenApi::merge` does not merge `info`. App `with_openapi(AppApi)` then cell merge keeps app `info`. Reverse order keeps the cell title.

Cell `servers` entries append on merge. A cell that declares none leaves app `servers` alone.

`OpenApiRouter::default()` is an empty `OpenApi`. It does not inject `info` or `servers`.
