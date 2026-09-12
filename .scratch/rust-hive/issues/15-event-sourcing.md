# Do we event-source, where, and what is the contract?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 09, 14

## Question

Port chapter 7 onto this cell's Postgres schema.

Locked: sqlx; one schema per cell; outbox on the same save transaction. Persistence: [How is Postgres isolation enforced per cell?](09-postgres-cell-isolation.md). CQRS must already be decided: [How does CQRS work inside a Rust cell?](14-cqrs-inside-a-cell.md).

Decide: per-aggregate opt-in versus skip for v1; whether the stream is the write model; snapshots; the same write SPI for state and stream aggregates; queries never replay.

EventStoreDB and a second database stay out. Do not write the chapter here.

## Answer

Event sourcing is per-aggregate, never hive-wide. The stream is the write model only where a write must be reversible without silent edit. Other aggregates stay state tables. One cell may mix both. First cells stay state unless that aggregate already fails the test. Do not inventory aggregates. Temporal queries do not pick storage. The test is the domain job, not the word financial.

One row per event in a table in this cell's schema. Same database, cell role, and named pool. Unique `(stream_id, version)`. Append is `INSERT`. Unique violation (`23505`) is the concurrency conflict. A new event table is a cell migration. No extra schema.

Snapshots are optional in a second table in that schema. Replay without a snapshot stays correct.

EventStoreDB and a second database stay out.

Application uses the same write SPI: `get_by_id` / `save`. Replay and append live in the sqlx adapter. Expected version sits on the aggregate the adapter loaded. One `pool.begin()` on that cell's pool writes the stream or state row, the outbox rows, and extra in-cell read tables.

Queries never replay. They use a read SPI. The stream answers `get_by_id` only. Extra in-cell read tables are allowed when this cell's queries cannot use the write row. Same transaction as `save`. After-commit event handlers do not write a query's required read model.

Stored events stay in-cell. Domain events stay in-cell. They publish after persist through the write-side events SPI.

Event rows are immutable. New types for new facts. Adapter upcasters when an old type must be read as a new shape. Weak readers ignore unknown fields. No `UPDATE` of payloads. Copy-and-replace is a migration.

No foreign stream subscribe. Drain stays on [Where do ticks, queues, and workers live?](18-workers-and-ticks.md).

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Stream**.

## Comments

### Round 1

Seven arrows accepted:

- Q1 A: per-aggregate opt-in. Stream only where a write must be reversible without silent edit. Other aggregates stay state tables. One cell may mix both. First cells stay state unless that aggregate already fails the test. Do not inventory. The test is the domain job, not the word financial.
- Q2 A: the stream is the write model. No state row beside it for that aggregate.
- Q3 A: one row per event in this cell's schema. Unique `(stream_id, version)`. Append is INSERT. `23505` is the concurrency conflict. New event table is a cell migration. No extra schema.
- Q4 A: snapshots optional in a second table in that schema. Replay without a snapshot stays correct.
- Q5 A: state and stream share `get_by_id` / `save`. Replay and append live in the sqlx adapter. Expected version sits on the aggregate. One `pool.begin()` writes the stream or state row, the outbox, and extra in-cell read tables.
- Q6 A: queries never replay. Stream answers `get_by_id` only. Extra in-cell read tables when this cell's queries cannot use the write row, same txn as `save`. After-commit handlers do not write a query's required read model.
- Q7 A: stored events are immutable. New types for new facts. Adapter upcasters. Weak readers ignore unknown fields. No UPDATE of payloads. Copy-and-replace is a migration.

### Round 2

Q8 A: record that draft, add **Stream** to the glossary, close. Snapshot stays out of the glossary.
