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

<!-- the index — one line per closed ticket -->

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
