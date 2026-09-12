# How does presentation emit OpenAPI, and which crate do we pin?

Type: grilling
Status: resolved

Label: wayfinder:grilling
Blocked by: 03

## Question

Which OpenAPI crate does chapter 8 pin, and where do the annotations live?

Presentation maps verb plus path onto a use-case API port. Published Language structs already live in `domain/api`. Handlers live in `presentation/http`. Choose handler annotations, PL derives, or both. Name the [Stack pins](../../../docs/adr/0001-stack-pins.md) change.

## Answer

Pin `utoipa` 5.5.0 and `utoipa-axum` 0.2.0. `utoipa` lives in `domain/api` and cell `presentation/`. `utoipa-axum` lives in cell `presentation/` only.

Drop "No OpenAPI crate" from [Stack pins](../../../docs/adr/0001-stack-pins.md). Replace chapter 8's "No OpenAPI crate pin" with those pins. Public REST handlers carry `#[utoipa::path]`. Unpublished REST is not annotated.

Published Language in `domain/api` derives `ToSchema` beside `Deserialize`, `Serialize`, and `Validate`. Envelope types do not derive `ToSchema`. The public error document is presentation's RFC 9457 `Problem`. `domain/entities`, `domain/spi`, and `application/` never name `utoipa`.

Ignore `RUSTSEC-2024-0436` for transitive `paste` until the pin bumps to a pastey `utoipa-axum`. Do not set `unmaintained = "workspace"`. Write `routes!(create_load)` with no turbofish.

Document assembly stays on [One OpenAPI document or per-cell specs merged at app?](10-openapi-document-grain.md).
