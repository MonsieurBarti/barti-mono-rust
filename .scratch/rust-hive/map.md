# Rust hive architecture for a freight brokerage

Label: wayfinder:map

## Destination

A locked `docs/architecture.md` and stack-pin ADR for a greenfield Rust hive-cells Cargo workspace on Postgres, instantiated as a B2B freight brokerage with named first cells, ready to hand off to implementation.

## Notes

- Domain: port naboo hive/cells into Rust. Host product: B2B freight brokerage.
- Skills every session: grilling, domain-modeling, research, codebase-design. Tracker: `docs/agents/issue-tracker.md`.
- Standing: local markdown, never Linear. Latest crate versions only. Greenfield hive-strict. No legacy neighbours. Plan; do not implement product code on this map.
- Source law: `/Users/pierrelecorff/Projects/naboo/docs/architecture.md`. Adapt. Do not copy Nest, Mongo, or Linear links.
- Assumptions locked at charting: spec-only destination; freight brokerage; `AGENTS.md`; default triage labels.

## Decisions so far

- [What is the current SOTA Rust HTTP stack for a hive composition root?](issues/01-sota-http-stack.md) — axum 0.8.9 on tokio/hyper/tower; tower-http 0.6 not 0.7.
- [What is the current SOTA Rust Postgres stack for exclusive per-cell schemas and roles?](issues/02-sota-postgres-isolation.md) — sqlx 0.9.0; schema + LOGIN role + PgPool per cell.
- [What is the current SOTA Rust approach to in-process CQRS, domain events, and a transactional outbox on Postgres?](issues/03-sota-cqrs-events-outbox.md) — no bus crate; traits + outbox on the save transaction.
- [What is the current SOTA Rust approach to Published Language validation at a cell edge?](issues/04-sota-cell-edge-validation.md) — serde + garde; VOs stay inside the hexagon.
- [How should a Cargo workspace express one crate per cell, a kernel crate, and a composition-root binary?](issues/05-sota-cargo-workspace-cells.md) — crate-per-cell rlib, kernel rlib, app bin, cargo-deny wall.

## Not yet specified

- Freight ubiquitous language and the first vertical slice (post a load → quote → book → settle).
- AuthN/AuthZ at the process edge.
- Event sourcing: per-aggregate opt-in, or skip for v1.
- Observability vendor and trace/metric names.
- Testing lanes (unit / integration / e2e) without Nest harnesses.
- Published Language scalars: money, ids, dates.
- Workers, ticks, drains, and the outbox consumer.
- Whether every naboo chapter is ported or some are dropped as brownfield-only.

## Out of scope

- Linear as the issue tracker.
- Mongo, Nest, TypeScript, and naboo production code.
- Frontend packages.
- Extracting a cell into another deployable.
- Implementing the freight product. This map ends at the spec.
