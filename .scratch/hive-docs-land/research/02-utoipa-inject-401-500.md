# How does utoipa 5 inject 401 and 500 after merge?

Researched 2026-09-12. Ticket: [How does utoipa 5 inject 401 and 500 after merge?](../issues/02-utoipa-inject-401-500.md)

Pins: `utoipa` 5.5.0, `utoipa-axum` 0.2.0.

## Verdict

Walk the merged `utoipa::openapi::OpenApi` after `split_for_parts()`. Insert 401 and 500 on every operation. There is no first-class inject API.

`OpenApiRouter` in 0.2.0 exposes `merge`, `get_openapi_mut`, and `split_for_parts`. It does not add responses to operations.

`Modify` plus `#[openapi(modifiers(&...))]` on `AppApi` runs before `AppApi::openapi()` returns. `app` calls that at the start, before cell merge. Cell operations do not exist yet.

The crate documents this path as "Modify generated OpenAPI via types directly": take `&mut OpenApi` and write the fields.

## Code shape

```rust
use utoipa::openapi::path::PathItem;
use utoipa::openapi::{Content, Ref, RefOr, Response, ResponseBuilder};
use utoipa_axum::router::OpenApiRouter;

fn problem_response(description: &'static str) -> RefOr<Response> {
    ResponseBuilder::new()
        .description(description)
        .content(
            "application/problem+json",
            Content::new(Some(Ref::from_schema_name("Problem"))),
        )
        .into()
}

fn operations_mut(item: &mut PathItem) -> impl Iterator<Item = &mut utoipa::openapi::path::Operation> {
    [
        item.get.as_mut(),
        item.put.as_mut(),
        item.post.as_mut(),
        item.delete.as_mut(),
        item.options.as_mut(),
        item.head.as_mut(),
        item.patch.as_mut(),
        item.trace.as_mut(),
    ]
    .into_iter()
    .flatten()
}

fn inject_401_500(openapi: &mut utoipa::openapi::OpenApi) {
    let unauthenticated = problem_response("UNAUTHENTICATED");
    let internal = problem_response("Internal server error");
    for item in openapi.paths.paths.values_mut() {
        for operation in operations_mut(item) {
            operation
                .responses
                .responses
                .entry("401".into())
                .or_insert_with(|| unauthenticated.clone());
            operation
                .responses
                .responses
                .entry("500".into())
                .or_insert_with(|| internal.clone());
        }
    }
}

let (router, mut openapi) = OpenApiRouter::with_openapi(AppApi::openapi())
    .merge(loads::router())
    .split_for_parts();
inject_401_500(&mut openapi);
let router = router
    .route("/health", axum::routing::get(health))
    .layer(/* middleware */);
```

`PathItem` in 5.5.0 has no `operations()` helper. Walk the eight `Option<Operation>` fields. utoipa's own nested-API walker does the same.

`Ref::from_schema_name("Problem")` writes `$ref: "#/components/schemas/Problem"`. That matches the locked slice. `Ref::from_response_name` writes `#/components/responses/...`. Do not use it.

`Content::new` takes `Option<impl Into<RefOr<Schema>>>`. `ResponseBuilder::content` takes the media type as a string. `From<ResponseBuilder> for RefOr<Response>` exists.

`Responses.responses` is `BTreeMap<String, RefOr<Response>>`. `entry().or_insert_with` adds 401 and 500 when the cell omitted them. A cell that already documented one of those statuses keeps its response. `insert` would overwrite. The spec says cells document the statuses they choose, so keep `or_insert`.

Do not register `Problem` again. Merge is first-wins. The first cell that defines `Problem` already put that schema in `components.schemas`. Inject only `$ref`s the name.

## Forks

These are the same walk at a different call site. They are not a second API.

| Call site | When | Notes |
| --- | --- | --- |
| **After `split_for_parts()`** (recommended) | Merge, split once, mutate the owned `OpenApi`, layer middleware on the axum `Router` | Matches the spec order. Inject is a spec transform. Middleware is a router transform. They split apart here. |
| `get_openapi_mut()` after merge, before split | Same mutation on `&mut OpenApi` inside the router | `split_for_parts` consumes `self`. This is the only pre-split slot. Equivalent document. |

Hand-calling `Modify::modify` after merge is the same walk behind a trait. The derive hook still cannot see cell operations. Skip the trait.

These do not satisfy "after merge, `app` injects":

| Mechanism | Why not |
| --- | --- |
| `#[openapi(modifiers(&Inject))]` on `AppApi` | `Modify::modify` runs before `openapi()` returns. `app` calls `AppApi::openapi()` before merge. |
| `#[utoipa::path(responses((status = 401, ...), (status = 500, ...)))]` on each handler | Cell-side. Not post-merge. |
| `IntoResponses` / `ToResponse` on a shared type, listed in every path | Same. Per-operation, compile time. |
| `ResponseExt::json_schema_ref` (`openapi_extensions`) | Hard-codes `application/json`. The slice wants `application/problem+json`. |
| `components.responses` plus `Ref::from_response_name` | Works as a walk. The locked slice inlines the response object and `$ref`s the `Problem` schema. This fork emits `#/components/responses/...` instead. |

`utoipa-axum` 0.2.0 has no closer method. If the walk is rejected, the closest alternative is still walking `OpenApi` and inserting responses. Nothing else in these pins reaches every public operation after merge.

## Health and unpublished REST

`OpenApiRouter::route` is a pass-through to `axum::Router::route`. It copies the axum route and leaves the `OpenApi` value unchanged. `GET /health` registered that way never enters `paths`. Inject never sees it.

The official `axum-utoipa-bindings` example at tag `utoipa-axum-0.2.0` puts health on the spec with `routes!(health)`. Hive must not copy that. Unpublished REST already uses `.route`. Health follows that path, on the axum `Router` after split.

Walking every operation in the merged document is walking every public operation.

## Fit to hive

`app` starts `OpenApiRouter::with_openapi(AppApi::openapi())`, merges cell routers, does not nest, splits once. Inject sits on the `OpenApi` half after that split. Middleware sits on the axum half.

`Problem` stays the one shared component name. Inject `$ref`s `#/components/schemas/Problem`. It does not add `components.responses.Problem`.

## Sources

Fetched 2026-09-12.

- [utoipa 5.5.0 crate docs](https://docs.rs/utoipa/5.5.0/utoipa/) — "Modify OpenAPI at runtime" via generated types directly. `Modify` trait.
- [utoipa `Modify`](https://docs.rs/utoipa/5.5.0/utoipa/trait.Modify.html) — runs before `OpenApi::openapi()` returns. Derive examples use `#[openapi(modifiers(&...))]`.
- [utoipa `Modify` source](https://docs.rs/utoipa/5.5.0/src/utoipa/lib.rs.html) — `fn modify(&self, openapi: &mut openapi::OpenApi)` "before it is returned by `openapi::OpenApi::openapi`".
- [utoipa-gen `#[derive(OpenApi)]`](https://docs.rs/utoipa_gen/5.5.0/utoipa_gen/derive.OpenApi.html) — `modifiers(...)` is a list of `Modify` impls for runtime modification of that derived document.
- [utoipa `OpenApi`](https://docs.rs/utoipa/5.5.0/utoipa/openapi/struct.OpenApi.html) — public `paths`, `to_pretty_json`. `merge` is first-wins on schemas and responses by name.
- [utoipa `OpenApi::merge` source](https://docs.rs/utoipa/5.5.0/src/utoipa/openapi.rs.html) — `info` is not merged. Schema and `components.responses` retain names already in `self`.
- [utoipa `PathItem` source](https://docs.rs/utoipa/5.5.0/src/utoipa/openapi/path.rs.html) — `get`/`put`/`post`/`delete`/`options`/`head`/`patch`/`trace` as `Option<Operation>`. No `operations()` iterator. Nested-API walker in `lib.rs` updates those eight fields by hand.
- [utoipa `Operation` source](https://docs.rs/utoipa/5.5.0/src/utoipa/openapi/path.rs.html) — `pub responses: Responses`.
- [utoipa `Responses` source](https://docs.rs/utoipa/5.5.0/src/utoipa/openapi/response.rs.html) — `pub responses: BTreeMap<String, RefOr<Response>>`. `From<ResponseBuilder> for RefOr<Response>`.
- [utoipa `ResponseBuilder`](https://docs.rs/utoipa/5.5.0/utoipa/openapi/response/struct.ResponseBuilder.html) — `description`, `content(content_type, Content)`.
- [utoipa `Content::new` source](https://docs.rs/utoipa/5.5.0/src/utoipa/openapi/content.rs.html) — `fn new<I: Into<RefOr<Schema>>>(schema: Option<I>)`.
- [utoipa `Ref`](https://docs.rs/utoipa/5.5.0/src/utoipa/openapi/schema.rs.html) — `from_schema_name` → `#/components/schemas/{name}`. `from_response_name` → `#/components/responses/{name}`.
- [utoipa `Components`](https://docs.rs/utoipa/5.5.0/src/utoipa/openapi/schema.rs.html) — `schemas` and `responses` are separate maps.
- [utoipa `ResponseExt`](https://docs.rs/utoipa/5.5.0/src/utoipa/openapi/response.rs.html) — `json_schema_ref` inserts `application/json` only. Feature `openapi_extensions`.
- [utoipa-axum 0.2.0 crate docs](https://docs.rs/utoipa-axum/0.2.0/utoipa_axum/) — `OpenApiRouter`, `routes!`, `split_for_parts`.
- [utoipa-axum `OpenApiRouter`](https://docs.rs/utoipa-axum/0.2.0/utoipa_axum/router/struct.OpenApiRouter.html) — `with_openapi`, `merge`, `route`, `get_openapi_mut`, `split_for_parts`. No inject method.
- [utoipa-axum `router.rs` 0.2.0](https://docs.rs/utoipa-axum/0.2.0/src/utoipa_axum/router.rs.html) — `route` is `Self(self.0.route(path, method_router), self.1)`. `merge` is `self.1.merge(router.1)` then axum merge. `split_for_parts` returns `(self.0, self.1)`.
- [utoipa example `axum-utoipa-bindings` @ `utoipa-axum-0.2.0`](https://github.com/juhaku/utoipa/blob/utoipa-axum-0.2.0/examples/axum-utoipa-bindings/src/main.rs) — `OpenApiRouter::with_openapi(ApiDoc::openapi())`, `routes!`, `nest`, `split_for_parts`. Health uses `routes!(health)` and is on the spec.
- [hive-docs spec](../../hive-docs/spec.md) — merge, no nest, `split_for_parts` once, inject 401 and 500 after merge, `Problem` is the one shared name, `GET /health` off the spec, locked 401/500 slice.
