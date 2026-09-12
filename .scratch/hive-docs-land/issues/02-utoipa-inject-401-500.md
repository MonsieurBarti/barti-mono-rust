# How does utoipa 5 inject 401 and 500 after merge?

Type: research
Label: wayfinder:research
Status: resolved
Blocked by:

## Question

How does utoipa 5.5.0 with utoipa-axum 0.2.0 add 401 and 500 to every public operation after `app` merges cell `OpenApiRouter`s?

The spec says `app` injects those statuses. Cells document the statuses they choose. `GET /health` stays off the spec.

Name the API. Show one code shape. Note forks if more than one mechanism works.

Write findings to [../research/02-utoipa-inject-401-500.md](../research/02-utoipa-inject-401-500.md).

## Answer

No first-class inject API. Walk the merged `OpenApi` after `split_for_parts()`. Insert 401 and 500 with `application/problem+json` `$ref` `Problem`. `or_insert` keeps a cell status. Findings: [../research/02-utoipa-inject-401-500.md](../research/02-utoipa-inject-401-500.md).
