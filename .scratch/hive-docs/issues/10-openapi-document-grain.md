# One OpenAPI document or per-cell specs merged at app?

Type: grilling
Label: wayfinder:grilling
Blocked by: 09
Status: resolved

## Question

Does each cell emit a fragment that `app` merges, or does the composition root own one document?

Public REST is nested in `app/http.rs`. Cell name is not a path segment. Collision is a review reject.

## Answer

Each cell's `pub fn router` returns `utoipa_axum::router::OpenApiRouter`. The cell applies `with_state` inside. Public REST uses `routes!(handler)` with no turbofish. Unpublished REST uses `OpenApiRouter::route`. It is served and absent from the spec.

`app` starts from `OpenApiRouter::with_openapi(AppApi::openapi())`. `AppApi` carries `info` and `servers`. `app` `merge`s each cell router. It does not `nest`. It calls `split_for_parts()` once, then layers middleware on the axum `Router`. `GET /health` stays on that router and off the spec.

Cells declare no `servers`. Starting the merge from `OpenApiRouter::new()` is forbidden. That title is utoipa-axum's crate metadata.

Operations are tagged with the cell name.

Schema names and `operationId`s are unique across cells. Collision is a review reject. `Problem` is the one shared component name. Each cell presentation defines that type with chapter 8's fields. Merge is first-wins. [What does the CI gate require?](12-ci-gate-contract.md) asserts every cell's copy serialises identically.

After merge, `app` injects 401 and 500 onto every public operation. Cells document the statuses they choose.

Chapter 8 gains a presentation duty: `router` returns `OpenApiRouter`, and `app` owns the assembled document. Crate pins stay on [How does presentation emit OpenAPI, and which crate do we pin?](09-openapi-crate-and-annotations.md).

Exact `info.title` and `info.version` wait on [Write the cell-docs and OpenAPI spec](14-write-the-spec.md). The inject mechanism is implementation.

Probe: [../research/10-openapi-document-grain.md](../research/10-openapi-document-grain.md)
