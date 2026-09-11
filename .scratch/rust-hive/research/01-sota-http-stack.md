# SOTA Rust HTTP stack for a hive composition root

Researched 2026-09-11. Latest stable crates.io versions only.

## Verdict

Use **axum 0.8** on **tokio 1** + **hyper 1** + **tower 0.5**.

Pin majors: `axum` 0.8, `tokio` 1, `hyper` 1, `tower` 0.5, `tower-http` 0.6, `http` 1.

Pin exact crates.io versions as of 2026-09-11:

| Crate | Version | crates.io published |
| --- | --- | --- |
| `axum` | 0.8.9 | 2026-04-14 |
| `axum-core` | 0.5.6 | 2025-12-27 |
| `axum-extra` | 0.12.6 | 2026-04-14 |
| `tokio` | 1.53.1 | 2026-07-20 |
| `tower` | 0.5.3 | 2026-01-12 |
| `tower-http` | 0.6.11 | 2026-05-18 |
| `hyper` | 1.11.1 | 2026-08-28 |
| `hyper-util` | 0.1.20 | 2026-02-02 |
| `http` | 1.5.0 | 2026-07-29 |

Do not take `tower-http` 0.7.1. `axum` 0.8.9 depends on `tower-http ^0.6.8`. `tonic` 0.14.6 (published 2026-05-07) optionally depends on `axum ^0.8` and `tower ^0.5`. Keep it off the HTTP pin until a cell needs gRPC.

This stack is a library, not a Nest-like module system. `Router::nest` and `Router::merge` compose cell driving adapters in `main`. Process-edge AuthN is one `middleware::from_fn` that validates the token, inserts `actor_id` into request extensions, and never forwards the token. Handlers extract `ActorId` and pass that string into the cell API port.

## Compared

| Stack | Latest stable | Recent downloads | Router | Extractors | Middleware | Graceful shutdown | AuthN inject `actor_id` | Why it lost / won |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **axum 0.8.9** | 2026-04-14 | 115M | `Router::route` / `nest` / `merge`; path `{id}` / `{*rest}` since 0.8 | `FromRequest` / `FromRequestParts`; `Path`, `Query`, `Json`, `Extension`, `State` | No bespoke system. Uses `tower::Service` + `tower-http`. `middleware::from_fn` for app code. | `axum::serve(listener, app).with_graceful_shutdown(tokio::signal::ctrl_c())` | `from_fn` reads `Authorization`, inserts typed `ActorId` into extensions; handler extracts it | **Winner.** Tokio-rs first party. Shares middleware with hyper/tonic. Composition is a tree of routers, not a DI container. |
| actix-web 4.15.0 | 2026-08-21 | 10M | `App` + `Scope` + `Resource`; `{name}` path segments | `FromRequest`; `web::Path`, `Json`, `Query`, `ReqData` | Own `Service` + `Transform`. `wrap` / `wrap_fn` / `middleware::from_fn`. Does not speak tower. | OS signals by default. `shutdown_timeout` (30s). `shutdown_signal(fut)` override. `disable_signals` available. | `wrap_fn` can insert request-local data; `FromRequest` can extract `actor_id` | Lost. Own runtime (`actix-rt`) and middleware trait. Worker factory clones the app. No shared stack with tonic. More framework than the hive wants. |
| poem 3.1.12 | 2025-07-28 | 700k | `Route::at`; still `:name` captures | `FromRequest`; `web::Path`, `Query`, `Data` | Own `Middleware` + `EndpointExt::with`. Optional `tower-compat` feature. | `Server::run_with_graceful_shutdown(ep, signal, timeout)` | `Data<T>` from request extensions | Lost. Last crates.io release is 13 months old. Optional tower, not native. Tiny ecosystem vs axum. No successor crate. |

Losers outside the ticket trio, checked so they are not silent successors:

- **salvo 0.96.0** (2026-08-27): active, ~900k recent downloads. Own framework, not the tokio/tower default.
- **ntex 3.12.3** (4.0.0-beta.9 exists): actix-lineage network services. ~100k recent downloads.
- **warp 0.4.3**: still published, filter combinators, 5M recent downloads. Not a composition-root router for many cells.
- **rocket 0.5.1**: last crates.io release 2024-05-23. Stale.

### Router

axum `Router` is a `tower::Service`. `nest` mounts a child router at a prefix. `merge` concatenates two routers. The composition root builds one `Router`, nests each cell's presentation router, then calls `serve`. Path syntax since 0.8 is `/{user_id}` and `/{*rest}` (matchit 0.8). Old `/:id` panics.

actix `HttpServer::new` takes a `Send + Sync` factory. Each worker gets its own `App`. That is a process-global factory, not a cell module graph, but it still forces the composition root to be cloneable per worker.

poem `Route` nests by `nest` / `at`. Capture syntax remains `:name`.

### Extractors

axum extractors implement `FromRequestParts` (headers, path, extensions, state) or `FromRequest` (body, last argument). `Option<T>` only works when `T` implements `OptionalFromRequestParts` (0.8 change). A typed `ActorId` extractor implements `FromRequestParts`, rejects missing identity with 401, and never sees the token.

actix extractors implement `FromRequest`. Body extractors must run first. Up to 12 extractors per handler.

poem extractors implement `FromRequest`. Failed extractors default to 400.

### Middleware

axum uses tower. `Router::layer` wraps all routes. `Router::route_layer` runs only on matched routes, which is the right hook for AuthN. `tower::ServiceBuilder` orders layers top to bottom. `tower-http` 0.6.11 supplies `TraceLayer`, `CorsLayer`, `CompressionLayer`, `RequestIdLayer`, `TimeoutLayer`.

actix middleware implements `Transform` + `Service`. Last `wrap` runs first. It does not accept `tower::Layer`.

poem middleware implements `Middleware` and applies with `.with(...)`. `tower-compat` is an optional adapter.

### Graceful shutdown

axum: bind `tokio::net::TcpListener`, then `axum::serve(listener, app).with_graceful_shutdown(async { tokio::signal::ctrl_c().await.ok(); })`. 0.8.4 fixed a task leak when `serve` ran without shutdown. `serve` itself is intentionally thin. Use hyper-util for extra knobs.

actix: SIGTERM graceful, SIGINT/SIGQUIT force. Default worker drain is 30 seconds (`shutdown_timeout`). `shutdown_signal` replaces the built-in watcher.

poem: `run_with_graceful_shutdown(endpoint, signal, timeout)`.

### Process-edge AuthN

Required hive rule: the process edge authenticates. Cells receive `actor_id`. Cells never receive the token.

axum pattern that matches the official `from_fn` + `Extension` docs:

1. Composition-root `route_layer(middleware::from_fn(auth))` on the human HTTP tree.
2. `auth` reads `Authorization`, validates, inserts `ActorId(String)` into `request.extensions_mut()`.
3. The token stays in that function.
4. Presentation handlers take `ActorId` via `FromRequestParts` (or `Extension<ActorId>`).
5. The handler calls the cell API port with `actor_id` only.

actix and poem can do the same with request extensions. They do not share that layer with a later tonic adapter.

axum crate docs also show tokio `task_local!` for request identity. Prefer extensions plus a typed extractor. Task-locals leak identity into `IntoResponse` impls and couple the hive to tokio task locals.

## Fit to hive

Hive law (naboo `docs/architecture.md` ch. 1–3): the composition root is the only process entry that imports every cell and binds leaving SPIs. Human AuthN/AuthZ stays at that edge. Cells receive `actorId`. Driving adapters call API ports. Open Host is the set of those ports, not HTTP. HTTP is a driving adapter. There is no Nest-like module system to port.

| Hive piece | axum mapping |
| --- | --- |
| Composition root | `main` + one `Router`. Nests/merges each cell's presentation `Router`. Binds SPI adapters. Calls `axum::serve`. |
| Cell presentation (driving adapter) | A `Router` of handlers. Each handler extracts PL + `ActorId`, calls that cell's API port. The handler does not import another cell. |
| API port / Open Host | A Rust trait (or function set) in the cell. HTTP does not own it. |
| SPI | Cell-owned trait. Composition root injects the adapter when constructing the cell's router/state. |
| Published Language | Request/response types at the handler. Validated at the cell edge (separate ticket). |
| Process-edge AuthN | One `from_fn` / `route_layer` in the composition root. Injects `actor_id`. Token never crosses into a cell crate. |
| No Nest modules | `Router` is a value. No `AppModule`, no `@Global()`, no cell-to-cell HTTP inside one process. Cell → cell stays InProc SPI. |

`State<S>` on `Router<S>` is the composition-root bag of API-port handles and SPI adapters. `FromRef` lets a handler take one port without seeing the whole bag. That is wiring, not a module system.

Tick/cron driving adapters do not take `actorId`. Mount them off the AuthN `route_layer`.

Later extract of a cell to another process replaces that cell's InProc adapter with an HTTP/gRPC client. axum's `tower::Service` surface is the one that also hosts `tonic` 0.14.

## Sources

- https://crates.io/crates/axum/0.8.9
- https://crates.io/api/v1/crates/axum
- https://crates.io/api/v1/crates/axum/0.8.9/dependencies
- https://crates.io/crates/actix-web
- https://crates.io/api/v1/crates/actix-web
- https://crates.io/crates/poem
- https://crates.io/api/v1/crates/poem
- https://crates.io/api/v1/crates/tower
- https://crates.io/api/v1/crates/tower-http
- https://crates.io/api/v1/crates/hyper
- https://crates.io/api/v1/crates/hyper-util
- https://crates.io/api/v1/crates/tokio
- https://crates.io/api/v1/crates/http
- https://crates.io/api/v1/crates/axum-extra
- https://crates.io/api/v1/crates/axum-extra/0.12.6/dependencies
- https://crates.io/api/v1/crates/axum-core
- https://crates.io/api/v1/crates/tonic
- https://crates.io/api/v1/crates/tonic/0.14.6/dependencies
- https://crates.io/api/v1/crates/salvo
- https://crates.io/api/v1/crates/ntex
- https://crates.io/api/v1/crates/warp
- https://crates.io/api/v1/crates/rocket
- https://docs.rs/axum/0.8.9/axum/
- https://docs.rs/axum/0.8.9/axum/struct.Router.html
- https://docs.rs/axum/0.8.9/axum/extract/index.html
- https://docs.rs/axum/0.8.9/axum/middleware/index.html
- https://docs.rs/axum/0.8.9/axum/middleware/fn.from_fn.html
- https://docs.rs/axum/0.8.9/axum/struct.Extension.html
- https://docs.rs/axum/0.8.9/axum/serve/fn.serve.html
- https://docs.rs/axum/0.8.9/axum/serve/struct.Serve.html
- https://docs.rs/axum/0.8.9/axum/serve/struct.WithGracefulShutdown.html
- https://docs.rs/axum-extra/0.12.6/axum_extra/
- https://docs.rs/actix-web/4.15.0/actix_web/
- https://docs.rs/actix-web/4.15.0/actix_web/struct.HttpServer.html
- https://docs.rs/actix-web/4.15.0/actix_web/trait.FromRequest.html
- https://actix.rs/docs/server/
- https://actix.rs/docs/extractors/
- https://actix.rs/docs/middleware/
- https://docs.rs/poem/3.1.12/poem/
- https://docs.rs/poem/3.1.12/poem/struct.Server.html
- https://docs.rs/poem/3.1.12/poem/web/struct.Data.html
- https://docs.rs/tokio/1.53.1/tokio/signal/fn.ctrl_c.html
- https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0
- https://github.com/tokio-rs/axum/blob/main/axum/CHANGELOG.md
- https://github.com/actix/actix-web/releases/tag/web-v4.15.0
- https://github.com/poem-web/poem/blob/master/poem/CHANGELOG.md
