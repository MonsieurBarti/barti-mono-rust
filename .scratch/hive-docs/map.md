# Cell documentation and OpenAPI

Label: wayfinder:map

## Destination

A locked spec for two products. OpenAPI covers every public REST endpoint, for API consumers. A maintainer handbook explains what each cell does, how it works, and how it talks to others, with a plain-language lead. The spec also lands `CONTEXT-MAP.md` and the first cell glossaries. Ready to hand off to implementation.

## Notes

- Domain: greenfield Rust hive-strict. Host product: B2B freight brokerage. Law is [docs/architecture.md](../../docs/architecture.md), [docs/adr/](../../docs/adr/), and [CONTEXT.md](../../CONTEXT.md).
- Skills every session: grilling, domain-modeling, research, writing-for-agents. Tracker: `docs/agents/issue-tracker.md`.
- Standing: local markdown, never Linear. Plan; do not implement the factory or emit OpenAPI on this map. Write every doc this effort produces with writing-for-agents.
- Locked at charting:
  - Spec-only. Implementation is the next map.
  - Two products, two audiences. OpenAPI is for API consumers. The factory is for maintainers. They do not share an entrypoint.
  - Agent docs stay in `AGENTS.md`, rules, and ADRs.
  - Cells get handbook pages. Kernel and `app` get short pages. Law stays in `docs/architecture.md` and ADRs. The maintainer entrypoint links law. It does not rewrite it.
  - `CONTEXT.md` stays a glossary. This spec includes `CONTEXT-MAP.md` and per-cell `CONTEXT.md`. The handbook is a separate narrative layer. The maintainer entrypoint links both.
  - Non-technical is a writing constraint. Maintainers are the readers. The lead section is plain language.
  - A cell page names Open Host ports and leaving SPIs. The entrypoint holds a one-page graph. Neither restates [Communication](../../docs/adr/0005-communication.md).
  - OpenAPI is public REST only. Unpublished ports stay out.
  - The spec proposes a law change: an OpenAPI crate pin and a presentation duty. Reopen chapter 8 and [Stack pins](../../docs/adr/0001-stack-pins.md).
  - The spec includes a filled `loads` handbook page, `loads` `CONTEXT.md`, and a `loads` OpenAPI slice.
  - CODEOWNERS is out of scope.
- Name contract. Effort slug: `hive-docs`. Research assets: `.scratch/hive-docs/research/NN-<slug>.md`.

## Decisions so far

- [How should we write good maintainer documentation?](issues/01-good-maintainer-docs.md) — handbook is Diátaxis explanation; lead is freight-plain; how-it-works lists edges then why; eight rules.
- [What does a documentation factory look like for a multi-cell hive?](issues/02-documentation-factory.md) — book assembler, one maintainer TOC; OpenAPI off that TOC; rustdoc is not the handbook; site generator unset.
- [How do DDD codebases keep a context map and per-context glossaries?](issues/05-context-map-glossaries.md) — three glossary files; freight nouns leave root; map lists cells and contact points; write files after cell crates exist.
- [How do CI gates enforce documentation without becoming theater?](issues/04-ci-documentation-gates.md) — five mechanical checks on the existing `ci` job; `required_status_checks` makes them real; no prose taste.
- [What is SOTA for generating OpenAPI from an axum presentation layer?](issues/03-sota-openapi-axum.md) — pin utoipa 5 + utoipa-axum 0.2; ToSchema on PL and path attrs on handlers; reopens chapter 8 and stack pins.
- [What is a cell handbook page?](issues/06-cell-handbook-page.md) — untitled lead; How it works lists Open Host, leaving SPIs, schema, then why; Invariants; See also; ≤500 words.
- [Where do handbook files live, and what is the maintainer entrypoint?](issues/07-handbook-entrypoint.md) — central `docs/handbook/`; README is start page and graph; one `cells/<cell>.md`; kernel and `app` off the graph.
- [What belongs in CONTEXT-MAP.md and per-cell CONTEXT.md?](issues/08-context-map-contents.md) — two headings; loads glossary at crate path; shipments and settlement named-only; leftover freight stays in root.
- [How does presentation emit OpenAPI, and which crate do we pin?](issues/09-openapi-crate-and-annotations.md) — pin utoipa 5.5.0 and utoipa-axum 0.2.0; ToSchema on PL, path attrs on handlers; ignore paste RUSTSEC until the pin bumps.
- [One OpenAPI document or per-cell specs merged at app?](issues/10-openapi-document-grain.md) — cell emits `OpenApiRouter`; `app` merges and owns `info`; `Problem` shared by name; `app` injects 401/500.
- [How do API consumers get the OpenAPI document?](issues/11-openapi-consumer-publish.md) — committed `docs/openapi/openapi.json`; pretty JSON; one merged file; `app` does not serve it.
- [What does the CI gate require?](issues/12-ci-gate-contract.md) — crate listing and closed trees; drift on openapi.json; oasdiff validate and breaking; lychee offline; nextest coverage and Problem identity.
- [What are the kernel and app short pages?](issues/13-kernel-and-app-pages.md) — hive-plain leads; group lists then why; ≤200 words; not a second architecture.md.
- [Write the cell-docs and OpenAPI spec](issues/14-write-the-spec.md) — [spec.md](spec.md) locks handbook, glossaries, OpenAPI law, CI, and the `loads` slice.



## Not yet specified

- Site generator and handbook tooling.
- Prose linter.
- OpenAPI versioning and compatibility.
- Whether `review-change` learns a docs axis.

## Out of scope

- CODEOWNERS and GitHub review requests. GitHub does not request a review from the pull-request author, and every path is owned by `@MonsieurBarti`.
- Building the factory or emitting OpenAPI on this map.
- Agent-facing docs, `AGENTS.md`, layer rules, and skills.
- Rewriting `docs/architecture.md` into the handbook.
- GraphQL, MCP, frontend, Linear.
- Serving Swagger UI from the hive process.
