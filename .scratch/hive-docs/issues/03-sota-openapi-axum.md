# What is SOTA for generating OpenAPI from an axum presentation layer?

Type: research
Status: resolved
Label: wayfinder:research
Blocked by:

## Question

Which crate and pattern should a hive cell's `presentation/http` use to emit OpenAPI for public REST?

Latest stable crates.io versions only. Compare `utoipa`, `aide`, and at least one other current option against crate docs and axum docs.

Hive law that binds the answer:

- axum 0.8. Cell exports `router`. `app/http.rs` nests and merges.
- Published Language lives in `domain/api`. Handlers live in `presentation/http/<use-case>/`.
- Public errors are RFC 9457 `application/problem+json`. Envelope `context` never reaches the client.
- No OpenAPI crate pin today. Chapter 8 says so.
- Unpublished ports stay out of the public spec.
- `loads` has no `presentation/` yet. The pattern is greenfield.

Recommend one crate, a major pin, and where annotations live: handler, PL type, or both.

Asset: `.scratch/hive-docs/research/03-sota-openapi-axum.md`

## Answer

Pin `utoipa` 5 (5.5.0) with `utoipa-axum` 0.2 (0.2.0). Annotations live in both layers: `#[derive(ToSchema)]` on Published Language in `domain/api`, `#[utoipa::path(...)]` on the handler in `presentation/http/<use-case>/`. utoipa core names no web framework, so `domain/api` stays axum-free, and it declares the request body instead of inferring it, which is what a hive handler needs because chapter 9 forces a `Bytes` body rather than a `Json<T>` extractor. aide infers from extractors and produced `application/octet-stream` with no schema against that handler, and its non-optional `tracing` dependency breaks `deny.toml`. oasgen replaces the router with its own `Server`, so the cell could not export `router`.

Two things to carry into the pin: write `routes!(create_load)` without a turbofish, since the macro cannot destructure generic arguments and every cell handler is generic over `C, L, M`; and `utoipa-axum` pulls `paste`, which fails cargo-deny's default unmaintained check.

Reopens chapter 8 ("No OpenAPI crate pin", plus a presentation duty because the cell now exports a spec beside the `axum::Router`) and [Stack pins](../../../docs/adr/0001-stack-pins.md) ("No OpenAPI crate").

Findings: [../research/03-sota-openapi-axum.md](../research/03-sota-openapi-axum.md)
