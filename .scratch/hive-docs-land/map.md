# Land cell docs and OpenAPI

Label: wayfinder:map

## Destination

The repo matches [the locked spec](../hive-docs/spec.md). Handbook files, glossaries, OpenAPI law, the `app` generator, CI gates, and the `cell-docs` skill are in place.

## Notes

- Domain: greenfield Rust hive-strict. Host product: B2B freight brokerage. Law is [docs/architecture.md](../../docs/architecture.md), [docs/adr/](../../docs/adr/), and [CONTEXT.md](../../CONTEXT.md). Spec is [../hive-docs/spec.md](../hive-docs/spec.md).
- Skills every session: grilling, domain-modeling, research, writing-for-agents, ship-ticket. Tracker: `docs/agents/issue-tracker.md`.
- Standing: local markdown, never Linear. This map executes. Write every doc this effort produces with writing-for-agents.
- Locked at charting:
  - Land the spec Next map checklist in the repo. Not a second spec.
  - Extra binary in the `app` crate writes `docs/openapi/openapi.json`. `migrate` and `serve` stay the two process entries.
  - `cell-docs` lives at `.omp/skills/cell-docs/`. Model-invoked. It updates the cell handbook page, cell `CONTEXT.md`, the `CONTEXT-MAP.md` row, and public OpenAPI annotations.
  - `ship-ticket` gains a numbered step that runs `cell-docs` when the ticket touches those surfaces. Skip when it does not. Effort-short is `docs-land`.
  - `cell-docs` may drop terms from root `CONTEXT.md` that now live in a cell glossary. `docs/architecture.md` and `docs/adr/` stay banned except on tickets that reopen law.
  - `review-change` stays Law and Ticket.
- Name contract. Effort slug: `hive-docs-land`. Research assets: `.scratch/hive-docs-land/research/NN-<slug>.md`.

## Decisions so far

- [What versions do we pin for oasdiff and lychee?](issues/01-oasdiff-lychee-pins.md) — oasdiff 1.31.0, oasdiff-action v0.1.15, lychee 0.24.2, lychee-action v2.9.0; CI uses the action tags.
- [How does utoipa 5 inject 401 and 500 after merge?](issues/02-utoipa-inject-401-500.md) — walk `OpenApi` after `split_for_parts()`; no first-class inject API.
- [Land the handbook files](issues/03-land-handbook-files.md) — README, kernel, app, and loads pages from the spec fences.
- [Land the glossaries and context map](issues/04-land-glossaries.md) — CONTEXT-MAP.md and loads glossary from the spec fences; six loads terms left root ### Freight.
- [Apply the OpenAPI law change](issues/05-apply-openapi-law.md) — chapter 8 and ADR 0001 take `utoipa` 5.5.0, `utoipa-axum` 0.2.0, and the four CI tool pins.
- [Emit OpenAPI from loads presentation](issues/06-emit-openapi-from-loads.md) — `loads` `router` returns `OpenApiRouter`; PL `ToSchema`; `routes!(create_load)`; paste RUSTSEC ignored.
- [Merge OpenAPI in app and write the document](issues/07-merge-and-write-openapi.md) — `app` merges from `AppApi`; extra bin writes `docs/openapi/openapi.json`.
- [Inject 401 and 500 after merge](issues/08-inject-401-500.md) — walk merged `OpenApi`; `or_insert` 401 and 500 with `$ref` `Problem`.


## Not yet specified


## Out of scope

- Site generator and handbook tooling.
- Prose linter.
- OpenAPI versioning and compatibility.
- Handbook pages for `shipments` and `settlement`.
- A `review-change` docs axis.
- Serving Swagger UI from the hive process.
- Agent-facing docs other than `cell-docs`, the `ship-ticket` step, and the `AGENTS.md` pointer.
- CODEOWNERS and GitHub review requests.
