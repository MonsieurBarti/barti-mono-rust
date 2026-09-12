# SOTA OpenAPI generation from an axum 0.8 presentation layer

Researched 2026-09-12. Latest stable crates.io versions only.

## Verdict

Use **utoipa 5** with **utoipa-axum 0.2**.

Pin majors: `utoipa` 5, `utoipa-axum` 0.2. Pin exact crates.io versions:

| Crate | Version | Published | Recent downloads | Home |
| --- | --- | --- | --- | --- |
| `utoipa` | 5.5.0 | 2026-05-04 | 14.3M | `domain/api`, cell `presentation/` |
| `utoipa-axum` | 0.2.0 | 2025-01-16 | 3.0M | cell `presentation/` only |

Annotations live in **both** layers, split along the seam the hive already has:

- `domain/api/<use-case>/` adds `#[derive(utoipa::ToSchema)]` beside the existing `Deserialize`, `Serialize`, and `Validate` derives on Published Language.
- `presentation/http/<use-case>/` adds `#[utoipa::path(...)]` on the handler, carrying method, path, `tag`, `operation_id`, the `Idempotency-Key` header parameter, status codes, and the `application/problem+json` error responses.

Neither placement works alone. `ToSchema` declares no operation. `#[utoipa::path]` cannot `$ref` a PL schema that does not implement `ToSchema`.

This reopens two pieces of law. Chapter 8 says "No OpenAPI crate pin" ([architecture.md#8-rest](../../../docs/architecture.md), line 386) and [Stack pins](../../../docs/adr/0001-stack-pins.md) lists "No OpenAPI crate" under Not pinned (line 44). Both must change, and chapter 8 gains a presentation duty, because the cell's exported `router` has to yield a spec alongside the `axum::Router`.

Every claim below marked *(probed)* was compiled and run against the real pins in throwaway crates at `/tmp/utoipa-probe` and `/tmp/aide-probe`, on rustc 1.98.1 with axum 0.8.9. Nothing was added to this repo.

## Compared

| Crate | Latest stable | Published | Recent downloads | axum 0.8 | OpenAPI version | Schema source | Generic handlers | Unpublished ports | Verdict |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **utoipa 5.5.0 + utoipa-axum 0.2.0** | 5.5.0 / 0.2.0 | 2026-05-04 / 2025-01-16 | 14.3M / 3.0M | `axum ^0.8.0`, default features off | 3.1.0 *(probed)* | own `ToSchema` derive | works, no turbofish *(probed)* | `OpenApiRouter::route` pass-through serves without documenting *(probed)* | **Winner.** Declarative attributes match a `Bytes` body handler. Core crate has no axum dependency, so `domain/api` stays axum-free. |
| aide 0.15.1 | 0.15.1 | 2025-08-19 | 754k | `axum ^0.8.1` | 3.1.0 | `schemars::JsonSchema` | works, turbofish fine *(probed)* | only `api_route` is documented | Lost. Non-optional `tracing` dependency breaks this repo's import wall. Type inference produces a wrong body for hive handlers. Stable release still on schemars 0.9. |
| oasgen 0.25.0 | 0.25.0 | 2025-02-25 | 199k | `axum ^0.8.1` | 3.0 | own `OaSchema` derive + `inventory` global registry | not assessed, ruled out earlier | process-global registry | Lost. `Server::axum()` replaces the router, so the cell could not export `pub fn router`. 3.0 only. Last release 19 months ago. |

Checked and ruled out as framework mismatches, so they are not silent successors: `poem-openapi` 5.1.16 (poem), `salvo-oapi` 0.96.0 (salvo), `apistos` 0.7.1 (actix-web), `paperclip` 0.9.7 (actix-web, still on `http` 0.2), `rocket_okapi` 0.9.0 (rocket).

Checked and ruled out as too small or derivative: `axum-openapi3` 0.2.3 (16k recent downloads, a thin wrapper that itself depends on `utoipa ^5`, so it adds a maintainer between the hive and utoipa), `rovo` 0.4.9 (1.2k recent), `stonehm` 0.2.2 (115 recent).

### Why utoipa wins on the hive's handler shape

The decisive fact is what a hive handler looks like. Chapter 9 requires presentation to map `serde_path_to_error` failures into the `VALIDATION_FAILED` document, so `create_load` takes `axum::body::Bytes` and returns `Response`. It does not use the `Json<T>` extractor.

aide generates documentation by inferring from extractor and response types. Against a `Bytes` body it produced this *(probed)*:

```json
"requestBody": { "content": { "application/octet-stream": {} }, "required": true }
```

No schema, wrong media type. aide's central value proposition does not reach a handler that deliberately takes raw bytes. Recovering the real body means wrapping the extractor in `UseApi` or `WithApi`, which is hand-written metadata, which is the thing utoipa does directly.

utoipa declares the body in the attribute instead of inferring it, and emitted the correct document *(probed)*:

```json
"requestBody": { "content": { "application/json": {
  "schema": { "$ref": "#/components/schemas/CreateLoadInput" } } }, "required": true }
```

### Generic handlers, the one real trap

Every cell handler is generic over the cell's SPI type parameters, because chapter 3 binds leaving SPIs as generic parameters rather than `Arc<dyn _>`. The live signature is `create_load<C, L, M>(State(cell): State<Loads<C, L, M>>, ...)`.

`utoipa_axum::routes!` takes `$handler:path` and builds the identifier `__path_<handler>` through `paste`, then re-destructures the path as `$part:ident $( :: $tt:tt )*` ([utoipa-axum 0.2.0 src/lib.rs, line 167](https://docs.rs/utoipa-axum/0.2.0/src/utoipa_axum/lib.rs.html)). A turbofish does not survive that match:

- `routes!(create_load::<C, L, M>)` fails to compile: `error: no rules expected `path` metavariable ... while trying to match meta-variable `$part:ident`` *(probed)*.
- `routes!(create_load)` compiles and infers `C, L, M` from the `OpenApiRouter` state type *(probed)*.

Write the bare name. The rule is worth putting in the layer rule for `presentation/`, because the failure is a macro-internal error message that reads like a utoipa bug rather than a usage mistake.

### Path written once

`routes!` registers the axum route and the OpenAPI operation from the same `#[utoipa::path]` attribute, so `path = "/loads"` is not repeated in a `Router::route` call. utoipa fixed divergence between the axum route and the spec in [PR 1199](https://github.com/juhaku/utoipa/blob/master/utoipa-axum/CHANGELOG.md) and shipped it in utoipa-axum 0.1.3. `OpenApiRouter::split_for_parts()` then returns `(axum::Router<S>, utoipa::openapi::OpenApi)`.

### Unpublished ports stay out by construction

Chapter 8 keeps webhook, PDF, and stream handlers in `presentation/http/` as unpublished REST. `OpenApiRouter::route` is documented as a pass-through for `axum::Router::route`, and the `routes!` documentation states that only handlers collected with `routes!` get registered to the OpenApi. A webhook registered with `.route(...)` was served by the router and absent from the spec *(probed)*. No allow-list, no annotation to forget, and the default is exclusion.

### Maintenance and dependency weight

utoipa 5.5.0 shipped 2026-05-04 and the unreleased branch already adds OpenAPI 3.2.0 support. utoipa core depends on `indexmap`, `serde`, `serde_json`, and optionally `utoipa-gen` and `serde_norway`. It names no web framework, which is why `domain/api` can derive `ToSchema` without pulling axum into the port layer. utoipa-axum adds `axum`, `paste`, `tower-layer`, `tower-service`.

aide's stable line has drifted. 0.15.1 is from 2025-08-19 and still depends on `schemars ^0.9`, while schemars stable is 1.2.2. The schemars 1.0 upgrade sits only in the 0.16 alphas, which have run from alpha.1 on 2025-11-08 to alpha.4 on 2026-04-14 without a stable release.

## Fit to hive

| Hive rule | utoipa mapping |
| --- | --- |
| Cell exports `pub fn router`, `app/http.rs` nests and merges | Cell builds an `OpenApiRouter`, exports the `axum::Router` and the `OpenApi` from `split_for_parts()`. `OpenApiRouter::nest` and `merge` carry paths, so `app` keeps its current composition shape. |
| PL lives in `domain/api`, handlers in `presentation/http/<use-case>/` | `ToSchema` on PL, `#[utoipa::path]` on handlers. The split is the same one the hive already draws between the port's wire shape and the HTTP adapter. |
| `domain/api` names serde and garde, the inner hexagon does not | `utoipa` joins serde and garde in `domain/api`. `domain/entities`, `domain/spi`, and `application/` never name it. |
| Kernel stays axum-free, cells import no tracing | utoipa core pulls neither axum nor tracing. Kernel needs no utoipa dependency for this proposal. |
| Envelope `context` never reaches the client | The envelope enum gets no `ToSchema`. It is not a public type. Only the RFC 9457 `Problem` type, owned by presentation, is documented. |
| RFC 9457 `application/problem+json` | `responses((status = 400, body = Problem, content_type = "application/problem+json"))` emitted the exact media type *(probed)*. |
| Additive JSON, `deny_unknown_fields` on input PL | serde attributes carry into the schema. `deny_unknown_fields` became `"additionalProperties": false` and `rename_all = "camelCase"` was honoured *(probed)*. |
| Public paths are resources, no `/v1` prefix | `path = "/loads"` in the attribute. utoipa adds no prefix of its own. |
| `Idempotency-Key` is a presentation concern that never enters the cell | Declared as a header parameter on the operation, next to the handler that reads it. It stays out of `domain/api`. |
| `loads` has no `presentation/` yet | It does now: `crates/cells/freight/loads/src/presentation/http/create_load.rs` exists and `lib.rs` exports `pub fn router`. The pattern is no longer greenfield, so the proposal was probed against that exact handler shape. |

Recursive schema collection reached `pub(crate)` PL types nested three deep, `CreateLoadInput` to `StopInput` to `StopKindPl`, with no manual component registration *(probed)*. Doc comments on the handler became the operation `summary` and `description` *(probed)*, which keeps operation prose next to the code rather than in a separate document.

### Two consequences the pin ticket must take

**`paste` trips the import wall.** utoipa-axum 0.2.0 depends on `paste` 1.0.15, which carries [RUSTSEC-2024-0436](https://github.com/rustsec/advisory-db/blob/main/crates/paste/RUSTSEC-2024-0436.md), an informational unmaintained advisory. cargo-deny's `advisories.unmaintained` defaults to `all`, meaning any matching crate fails, and this repo's `deny.toml` has no `[advisories]` section, so the default applies. Three ways out: wait for the utoipa-axum release that replaces paste with pastey, which is already merged on the unreleased branch ([PR 1452](https://github.com/juhaku/utoipa/blob/master/utoipa-axum/CHANGELOG.md)); add an `ignore` entry; or set `unmaintained = "workspace"`, under which a transitive dependency no longer fails. Decide this when the pin lands, not after CI turns red.

**aide would have broken the same wall harder.** `deny.toml` line 6 denies `tracing` outside a fixed wrapper list. aide 0.15.1 lists `tracing ^0.1` as a non-optional dependency, and the tree confirms `tracing v0.1.44 ├── aide v0.15.1 └── <cell>` *(probed)*. Admitting aide means adding `aide` as a `tracing` wrapper, which punches a hole in the wall that enforces "cells import none of tracing". That is a mechanical rejection, not a preference.

### Open seam for ticket 09 and 10

Every cell emits the same RFC 9457 document, so the `Problem` schema is one component repeated per cell. Presentation owns it, since chapter 8 gives presentation the envelope-type to public-string mapping. Whether `app` injects one shared component at merge time or each cell carries its own copy is the merge question, and it belongs to [ticket 10](../issues/10-openapi-document-grain.md). Putting `Problem` in the kernel would reopen "Kernel depends on `time` only" in ADR 0001, so that option is not free.

One honest caveat on layering. A cell is one crate, and its layers are modules, so `utoipa` and `utoipa-axum` both appear in the same `Cargo.toml`. Nothing mechanically stops `domain/entities` from writing `use utoipa`. That is the same situation serde is already in under chapter 9, where the wall is the rule and review, not cargo.

## Sources

- https://crates.io/api/v1/crates/utoipa
- https://crates.io/api/v1/crates/utoipa/5.5.0/dependencies
- https://crates.io/api/v1/crates/utoipa-axum
- https://crates.io/api/v1/crates/utoipa-axum/0.2.0/dependencies
- https://crates.io/api/v1/crates/utoipa-gen
- https://crates.io/api/v1/crates/aide
- https://crates.io/api/v1/crates/aide/0.15.1/dependencies
- https://crates.io/api/v1/crates/aide/0.16.0-alpha.4/dependencies
- https://crates.io/api/v1/crates/schemars
- https://crates.io/api/v1/crates/oasgen
- https://crates.io/api/v1/crates/oasgen/0.25.0/dependencies
- https://crates.io/api/v1/crates/axum-openapi3
- https://crates.io/api/v1/crates/poem-openapi
- https://crates.io/api/v1/crates/salvo-oapi
- https://crates.io/api/v1/crates/apistos
- https://crates.io/api/v1/crates/paperclip
- https://crates.io/api/v1/crates/rocket_okapi
- https://crates.io/api/v1/crates/rovo
- https://crates.io/api/v1/crates/stonehm
- https://docs.rs/utoipa/5.5.0/utoipa/
- https://docs.rs/utoipa/5.5.0/utoipa/attr.path.html
- https://docs.rs/utoipa/5.5.0/utoipa/derive.ToSchema.html
- https://docs.rs/utoipa-axum/0.2.0/utoipa_axum/
- https://docs.rs/utoipa-axum/0.2.0/utoipa_axum/macro.routes.html
- https://docs.rs/utoipa-axum/0.2.0/utoipa_axum/router/struct.OpenApiRouter.html
- https://docs.rs/utoipa-axum/0.2.0/src/utoipa_axum/lib.rs.html
- https://docs.rs/aide/0.15.1/aide/
- https://docs.rs/aide/0.15.1/aide/axum/index.html
- https://docs.rs/oasgen/0.25.0/oasgen/
- https://github.com/juhaku/utoipa/blob/master/utoipa/CHANGELOG.md
- https://github.com/juhaku/utoipa/blob/master/utoipa-axum/CHANGELOG.md
- https://github.com/kurtbuilds/oasgen/blob/master/README.md
- https://github.com/rustsec/advisory-db/blob/main/crates/paste/RUSTSEC-2024-0436.md
- https://github.com/EmbarkStudios/cargo-deny/blob/main/docs/src/checks/advisories/cfg.md
