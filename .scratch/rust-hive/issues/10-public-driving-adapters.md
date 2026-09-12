# What is the public driving-adapter surface?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 01, 06

## Question

Port chapters 8–10 for a greenfield Rust hive.

Decide: REST, GraphQL, MCP, or a subset. Default recommendation: HTTP REST via the chosen stack; MCP out of v1 unless a cell needs agent tools; GraphQL out unless a client exists.

This is the public edge only. InProc is not public. AuthN mapping to `actor_id` can stay fog if this grill would otherwise explode.

## Answer

Public HTTP is REST on axum. GraphQL is not in this architecture. MCP is not in this architecture. A later client or agent surface is a new grill.

Cell `presentation/` depends on axum. Handlers live in `presentation/http/<use-case>/`. `lib.rs` re-exports `pub fn router` beside `new`. That function is not Open Host. `app/http.rs` nests and merges. Kernel stays axum-free. Cell tests hit that router with fake SPIs and never boot `app`. Cell name and domain folder are not path segments. A cell chooses its prefix. Collision is a review reject.

Open Host stays cell-to-cell. A public REST handler may call a `pub(crate)` port. Tick, webhook, and any REST-only command other cells must not call stay `pub(crate)`.

Handlers extract `actor_id` as a string. The body omits it. JSON deserializes into the Published Language struct. The port still `decode`s. Success is PL as `application/json`. The handler chooses 200, 201, or 204. 204 has no body. Success is never an envelope. No business logic. The token never enters the cell.

Public paths are resources. Presentation maps verb plus path onto a use-case API port. No kebab use-case paths. No JSON:API. No `/v1` prefix. Additive JSON only. No pagination law. No OpenAPI crate pin.

Hide `context`. Cell presentation owns `type` → public string. `app` merges catalogs at boot. Unknown `type` is a generic string plus a server warn. Sensitive types stay generic. One composition-root suffix map, no per-cell override. Domain error carries no status. `*_NOT_FOUND` → 404, `*_CONFLICT` → 409, `UNAUTHENTICATED` → 401, `FORBIDDEN` → 403, `RATE_LIMITED` → 429, `VALIDATION_FAILED` → 400, else → 500.

Public error document is RFC 9457 `application/problem+json`: `type` (envelope type string, not a URI), `status`, catalog `detail`, `instance` (request path), `correlationId` when present. No `title`. No `context`. No Apollo `code`. `VALIDATION_FAILED` adds `violations: { path, code, message }[]` with no submitted values.

Unhandled: HTTP 500, `type` is `about:blank`, `detail` is `Internal server error`, `correlationId` when present. Logs keep stack and `context`.

Health lives on `app`, not a cell, not Open Host. Webhook, PDF, and stream handlers are unpublished REST in `presentation/http/`. Auth routes stay fog. Workers stay fog.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Driving adapter**, **ActorId**, **Unpublished**.

## Comments

### Round 1

Four arrows accepted:

- Q1 A: HTTP REST only. GraphQL is not in this architecture. MCP is not in this architecture. A later client or agent surface is a new grill.
- Q2 A: cell `presentation/` depends on axum and exports `fn router(&Cell<…>) -> Router`. `app/http.rs` nests and merges. Kernel stays axum-free. Cell tests hit that router with fake SPIs and never boot `app`.
- Q3 A: hide `context`. Each cell presentation owns `type` → public string. `app` merges catalogs at boot. Unknown `type` is a generic string plus a server warn. Sensitive types stay generic.
- Q4 A: one composition-root suffix map. No per-cell override. Domain error carries no status. `*_NOT_FOUND` → 404, `*_CONFLICT` → 409, `UNAUTHENTICATED` → 401, `FORBIDDEN` → 403, `RATE_LIMITED` → 429, `VALIDATION_FAILED` → 400, else → 500.

### Round 2

Seven arrows accepted:

- Q5 A: RFC 9457 `application/problem+json` with `type`, `status`, catalog `detail`, filter-generated `instance`, `correlationId` when present. No `title`. No `context`. No Apollo `code`. `VALIDATION_FAILED` adds `violations[]` with no submitted values. `type` is the envelope type string, not a URI.
- Q6 A: resource URLs. Presentation maps verb + path onto a use-case API port. No kebab use-case paths. No JSON:API.
- Q7 A: no version prefix. Additive JSON only. A breaking change is a new grill.
- Q8 A: `presentation/http/<use-case>/`. Drop `resolvers/`, `controllers/`, `mcp/`, `dtos/`. Workers stay fog.
- Q9 A: health on `app`, not a cell, not Open Host. Webhook / PDF / stream handlers are unpublished REST in `presentation/http/`. Auth routes stay fog.
- Q10 A: no OpenAPI crate pin.
- Q11 A: extract `actor_id` as a string, body omits it, JSON into PL, port still `decode`s, encode PL as `application/json`. Handler has no business logic. Token never enters the cell.

### Round 3

Four arrows accepted:

- Q12 A: unhandled is HTTP 500, `application/problem+json`, `type` is `about:blank`, `detail` is `Internal server error`, `correlationId` when present. Logs keep stack and `context`.
- Q13 A: `lib.rs` re-exports `pub fn router` beside `new`. It is not Open Host.
- Q14 A: handler chooses 200, 201, or 204. 204 has no body. Success is never an envelope.
- Q15 A: no pagination law. A list use-case owns its PL page shape.
