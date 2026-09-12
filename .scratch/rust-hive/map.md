# Rust hive architecture for a freight brokerage

Label: wayfinder:map

## Destination

A locked `docs/architecture.md` and stack-pin ADR for a greenfield Rust hive-cells Cargo workspace on Postgres, instantiated as a B2B freight brokerage with named first cells, ready to hand off to implementation.

## Notes

- Domain: greenfield Rust hive-strict. Host product: B2B freight brokerage.
- Skills every session: grilling, domain-modeling, research, codebase-design. Tracker: `docs/agents/issue-tracker.md`.
- Standing: local markdown, never Linear. Latest crate versions only. Greenfield hive-strict. No legacy neighbours. Plan; do not implement product code on this map. Hive law only in spec and argument.
- Assumptions locked at charting: spec-only destination; freight brokerage; `AGENTS.md`; default triage labels. Tickets 01–25 stand as hive law. Destination `docs/architecture.md` is written. [Should the persistence adapter be an ORM?](issues/25-orm-adapter.md) supersedes the sqlx `query!` adapter surface.

## Decisions so far

- [What is the current SOTA Rust HTTP stack for a hive composition root?](issues/01-sota-http-stack.md) — axum 0.8.9 on tokio/hyper/tower; tower-http 0.6 not 0.7.
- [What is the current SOTA Rust Postgres stack for exclusive per-cell schemas and roles?](issues/02-sota-postgres-isolation.md) — sqlx 0.9.0; schema + LOGIN role + PgPool per cell.
- [What is the current SOTA Rust approach to in-process CQRS, domain events, and a transactional outbox on Postgres?](issues/03-sota-cqrs-events-outbox.md) — no bus crate; traits + outbox on the save transaction.
- [What is the current SOTA Rust approach to Published Language validation at a cell edge?](issues/04-sota-cell-edge-validation.md) — serde + garde; VOs stay inside the hexagon.
- [How should a Cargo workspace express one crate per cell, a kernel crate, and a composition-root binary?](issues/05-sota-cargo-workspace-cells.md) — crate-per-cell rlib, kernel rlib, app bin, cargo-deny wall.
- [What ubiquitous language does the Rust hive keep](issues/06-rust-hive-glossary.md) — hive terms stay; crate, sqlx adapter, codec, composition root, import wall, schema, table, cell role; promote, slice, neighbour die.
- [How is a cell packaged, and what is the composition root without Nest?](issues/07-cell-packaging-and-composition-root.md) — crate-per-cell rlib with layer modules; InProc lives in `app`, so no cell-to-cell edge exists; leaving SPIs bind as generics.
- [How do cells communicate in Rust hive-strict?](issues/08-hive-strict-communication.md) — consumer SPI in primitives; InProc in `app` maps both codecs; per-port envelope enum; events enter a consumer API port via a producer outbox drain.
- [How is Postgres isolation enforced per cell?](issues/09-postgres-cell-isolation.md) — two LOGINs; ops GRANT; named pool in the sqlx adapter; no cross-cell txn; query! on the cell role, 42501 at runtime.
- [What is the public driving-adapter surface?](issues/10-public-driving-adapters.md) — REST on axum; no GraphQL; no MCP; cell exports `router`; problem+json.
- [Which chapters drop as brownfield-only?](issues/11-brownfield-chapters-drop.md) — drop 8, 10, 14, Appendix A; strip brownfield paragraphs; still grill CQRS, ES, validation, observability, workers, PL scalars, kernel, freight cells.
- [What are the test lanes, and what does each one boot?](issues/12-test-lanes.md) — unit=domain, integration=application with real sqlx, e2e=`router`; nextest 0.9; 80% llvm-cov; GRANT in `app` tests.
- [How does AuthN/AuthZ work at the REST process edge?](issues/13-authn-rest-edge.md) — gateway in front; `app` copies identity headers; cell owns resource `FORBIDDEN`; webhooks verify vendor signatures.
- [How does CQRS work inside a Rust cell?](issues/14-cqrs-inside-a-cell.md) — one use-case struct implements the port; no Command/Handler types; `get_by_id`/`save`; events SPI after commit; query never mutates.
- [Do we event-source, where, and what is the contract?](issues/15-event-sourcing.md) — per-aggregate opt-in; stream is the write model; same `get_by_id`/`save`; queries never replay.
- [Where does the codec validate versus domain invariants?](issues/16-codec-versus-invariants.md) — decode is garde in `domain/api`; presentation serde; own rows not re-validated; foreign wire MAY parse, never `VALIDATION_FAILED`.
- [How do we observe a REST hive process?](issues/17-observability.md) — OTEL in `app`; cell Logger/Metrics SPIs as generics; `X-Correlation-ID`; `hive.<area>.<subject>`; one wide event per cell REST request.
- [Where do ticks, queues, and workers live?](issues/18-workers-and-ticks.md) — `presentation/ticks/`; cell `work` table + `spawn_work_drain`; no queue crate; one process.
- [How are Money, ids, and dates encoded in Published Language?](issues/19-pl-scalars.md) — `{ amount, currency }` minor units; UUIDv7 strings; ISO-8601 Instant; `YYYY-MM-DD`; kernel Clock.
- [What lives in the kernel crate?](issues/20-kernel-crate.md) — Money, Instant, Clock, Logger, Metrics, Violation, ISO functions; `time` only; no envelope, ids, or HTTP types.
- [What freight ubiquitous language and first cells?](issues/21-freight-language-and-first-cells.md) — `freight` folder; `loads`, `shipments`, `settlement`; Load/Quote/Stop, Shipment rates at book, Invoice/Payable.
- [Write the stack-pin ADR](issues/22-stack-pin-adr.md) — `docs/adr/0001-stack-pins.md`; axum 0.8.9; tower-http 0.6.11; sqlx 0.9.0; tracing 0.1.44 + OTLP 0.32; nextest 0.9.144.
- [How are mutating REST commands made idempotent?](issues/23-rest-command-idempotency.md) — human POST/PATCH require `Idempotency-Key`; cell `idempotency_key` table; no If-Match hive law.
- [Write the architecture document](issues/24-write-architecture-md.md) — `docs/architecture.md`; 16 chapters; majors only; REST includes `Idempotency-Key`.
- [Should the persistence adapter be an ORM?](issues/25-orm-adapter.md) — sea-orm adapter; GRANT wall stays; no `query!`; handwritten entities; sqlx 0.9 driver only.

## Not yet specified

## Out of scope

- Linear as the issue tracker.
- Frontend packages.
- Extracting a cell into another deployable.
- Implementing the freight product. This map ends at the spec.
- Gateway product, IdP vendor, and roles catalog.
- Auth issuance REST and SSO pages in this process.
