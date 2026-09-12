# Emit OpenAPI from loads presentation

Type: task
Label: wayfinder:task
Blocked by: 05
Status: resolved

## Question

Make `loads` emit public REST as `OpenApiRouter`.

Pin `utoipa` 5.5.0 in `domain/api` and `presentation/`. Pin `utoipa-axum` 0.2.0 in `presentation/` only. `lib.rs` re-exports `pub fn router` as `OpenApiRouter`. Public REST uses `routes!(handler)`. Published Language derives `ToSchema`. Handlers carry `#[utoipa::path]`. Envelope types do not derive `ToSchema`. Ignore `RUSTSEC-2024-0436` for transitive `paste` until the pin bumps.

Do not merge in `app`. Do not write `docs/openapi/openapi.json`.

## Answer

`loads` `router` returns `OpenApiRouter`. Public REST uses `routes!(create_load)` with no turbofish. Published Language derives `ToSchema`. The handler carries `#[utoipa::path]`. Envelope enums do not. `deny.toml` ignores `RUSTSEC-2024-0436`. `app` does not merge. No `docs/openapi/openapi.json`.
