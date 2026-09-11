# In-process CQRS, domain events, transactional outbox (Postgres)

Pinned as of 2026-09-11 from crates.io `max_stable_version` and Postgres 18 docs.

## Verdict

No CQRS bus crate.

No EventEmitter crate.

No outbox crate.

A Rust cell implements CQRS as traits plus the composition root.

Each use-case is one application struct that implements that cell's API-port trait.

Presentation and InProc call `Arc<dyn ApiPort>`.

They never see a command bus, a query bus, or a domain-event bus.

In-cell domain events stay on a kernel `DomainEventPublisher` port.

The cell infrastructure adapter dispatches after `save` commits.

It calls registered in-cell handlers by type.

It does not use `tokio::sync::broadcast` (lossy lag).

It does not use Nest-style process-global `EventBus`.

Integration events are Published Language rows in a producer-owned Postgres outbox table.

The write adapter inserts those rows on the same `sqlx::Transaction` as `save`.

An infrastructure drain (not a presentation worker) claims rows with `SELECT … FOR UPDATE SKIP LOCKED`.

Delivery is at-least-once.

Consumers are idempotent.

The consumer cell never reads the producer outbox.

Optional wake-up is Postgres `LISTEN`/`NOTIFY` via `sqlx::postgres::PgListener`.

A poll interval remains the source of truth when a notify is lost.

### Pin

| Piece | Choice | Version |
| --- | --- | --- |
| CQRS dispatch | No crate. API-port trait + application use-case. | — |
| In-cell domain events | No crate. Kernel publisher trait. Cell adapter after commit. | — |
| Outbox write + drain SQL | Cell-owned table. Same transaction as `save`. | — |
| Postgres client for that txn and optional notify | `sqlx` (`Transaction`, `PgListener`) | **0.9.0** |
| Runtime | `tokio` (not a domain bus) | **1.53.1** |

Ticket 02 owns the process-wide Postgres client.

If that ticket pins `sqlx`, this outbox uses `sqlx` 0.9.0 as above.

If it pins Diesel 2.3.13 or sea-orm 2.0.2, the SQL stays the same.

The drain still runs `FOR UPDATE SKIP LOCKED` on the cell connection.

Event sourcing stays per-aggregate and out of this pin.

Chapter 7 keeps first-merge on state rows unless an aggregate fails the reversibility test.

`cqrs-es` 0.5.0 and friends force an event store as the write model.

They are the wrong shape for hive CQRS.

## Compared

| Candidate | Stable pin | Why it lost |
| --- | --- | --- |
| Traits + composition root (winner) | no crate | Presentation injects the API port. The use-case is the handler. Domain events stay in-cell. Outbox is SQL. |
| `cqrs-es` + `postgres-es` | 0.5.0 / 0.5.0 | `CqrsFramework::execute` is a process bus over an event store. `Query::dispatch` runs after commit and the docs list "publish events to messaging service" and "trigger a command on another aggregate". That leaks domain events and invites chatty cell-to-cell commands. Hive CQRS is not event-sourced by default. |
| `eventastic` + `eventastic_postgres` | 0.5.0 / 0.5.0 | Closest outbox story ("transactional outbox pattern", mandatory transactions). Still an ES aggregate framework. Side effects ride the event-sourced `Aggregate` trait. Hive first-merge is state documents. |
| `disintegrate` + `disintegrate-postgres` | 4.0.0 / 4.0.1 | Event-stream decision engine with `PgEventListener`. Forces event sourcing. `DecisionMaker` is not an API port. |
| `esrs` | 0.18.0 (2024-11-25) | CQRS/ES with `EventBus` adapters for Kafka and Rabbit. Domain events would leave the cell. Last stable is older than the 2026 field. |
| `es-entity` | 0.12.21 | Galoy ES persistence on sqlx. Not a command/query bus. Still ES-first. |
| `distributed` | 4.12.1 | Fullstack ES+CQRS+GraphQL generator. 1377 all-time downloads. Not a cell primitive. |
| `thalo` | 0.8.0 (2023-11-21) | WASM event-sourcing runtime. Stale. |
| `eventually` / `cqrs` | 0.4.0 / 0.3.1 | Unmaintained (2020 / 2019). |
| `eventstore` / `kurrentdb` / `eventsourcingdb` | 4.0.0 / 1.2.0 / 2.0.8 | Second database. Chapter 7 forbids EventStoreDB and any second store. |
| `messagebus` | 0.15.2 (2024-02-07) | Process-global inter-module bus. Presentation would see it. Cells would share types. |
| `event-emitter-rs` / `async-event-emitter` | 0.1.4 / 0.1.5 | Stringly EventEmitter clones. 0.1.4 is 2020. Broadcast-style emitters drop or lag. Wrong for after-persist domain facts. |
| `tokio::sync::broadcast` | tokio 1.53.1 | Official MPMC broadcast. Capacity overflow returns `RecvError::Lagged` and drops messages. Domain events after persist must not drop. |
| `sqlxmq` | 0.6.0 | Postgres job queue with `mq*` schema and transactional spawn. A job runner is not a producer-owned integration-event outbox. The consumer would share a queue schema. Hive drain is cell infrastructure. |
| `pgmq` | 0.33.7 (newest 0.34.0-alpha.6) | SQS-like queue. Docs point at the Tembo Postgres extension. Hive outbox is a cell table, not an extension queue. The consumer must not read the producer collection. |
| `graphile_worker` | 0.13.5 | Durable jobs with `SKIP LOCKED` and `LISTEN`/`NOTIFY` in schema `graphile_worker`. Right primitives, wrong product. It is a worker framework, not a cell outbox. Drain is not a presentation worker and not a job runner. |
| `apalis` / `apalis-sql` | 0.7.4 | Background task processor. Same mismatch as sqlxmq. |
| `outbox-relay` | 0.1.0 (2021) | Kafka relay. One version. Stale. Ships events out of process. |

`sqlx` 0.9.0 wins the persistence slot for this pattern because `Pool::begin` yields a `Transaction` that rolls back on drop, and `PgListener` is a first-party `LISTEN` stream.

Diesel 2.3.13 and sea-orm 2.0.2 also wrap transactions.

They do not change the SQL.

## Fit to hive

Port of naboo chapters 4, 5, 7 (and the drain rule in 16). Adapt. Do not copy Nest or Mongo.

### Chapter 4 — Communication

Request/response stays the default cell edge.

HTTP, MCP, and InProc call the API port.

They do not call a bus.

Internal domain events live under `domain/events/` and never leave the cell.

Integration events are PL schemas under `domain/api/<event>/`.

They are not Open Host tokens.

The consumer owns the SPI.

A consumer infrastructure adapter subscribes to the drained PL and maps into that SPI.

Local read models: the producer emits PL through its outbox.

The consumer drain adapter writes the consumer's tables.

The consumer read path does not call the provider.

### Chapter 5 — CQRS

Naboo keeps `@nestjs/cqrs` because Nest already ships a first-party bus and forbids a hand-rolled one.

Rust has no first-party equivalent with that role.

A bus crate would be the hand-rolled bus chapter 5 rejected, in crate form.

The hive shape is:

1. `domain/api/<use-case>/` holds the API-port trait and the PL types.
2. `application/commands/<use-case>/` (or `queries/`) holds the command or query, the handler, and the struct that implements the API port.
3. That struct maps PL → command/query, then runs the handler.
4. Presentation never imports the command type, the query type, or a bus.
5. The cell crate exports the API-port implementations the composition root binds.
6. Unpublished ports stay off that export set (GraphQL-only, MCP-only, ticks).
7. A command may return an id or outcome.
8. A query never mutates.
9. Command handlers inject the write SPI (`get_by_id` / `save`), run entity behavior, persist, then publish.
10. Query handlers inject a read SPI and return a plain read model.
11. They never load the write-side entity.

After persist, in-cell handlers publish through a write-side events SPI.

The adapter is cell infrastructure.

Application does not construct `tokio::sync::broadcast` or a crate EventEmitter.

### Chapter 7 — Persist, events, outbox

Event sourcing is per-aggregate, not hive-wide.

First-merge stays state unless that aggregate already fails the reversibility test.

This ticket does not pick an ES crate.

Application still uses `get_by_id` / `save`.

Replay, if any later aggregate opts in, lives in the adapter.

One cell transaction on the cell connection writes the aggregate (stream or state row), Open Host outbox rows, and extra in-cell read collections.

Queries never replay.

They use a read SPI.

Stored events and domain events stay in-cell.

Open Host integration events always go through the transactional outbox, same transaction as `save`.

The producer cell owns the outbox table and the relay.

The consumer cell never `SELECT`s that table.

At-least-once.

Idempotent consumers.

No SLA on local read-model lag.

Request/response stays the default edge.

Outbox is only fire-and-forget (notifications, local read models).

After a later extract, the relay swaps the in-process publish for a queue.

The consumer adapter swaps the in-process subscribe for a consumer.

### Chapter 16 — Drain

The producer-owned outbox drain is per-cell infrastructure on the cell connection.

It is not a presentation worker.

It is not a cron driving adapter.

It is not a queue processor.

The cell starts the poller.

### Recommended in-cell flow

```
driving adapter
  -> Arc<dyn ApiPort>          // presentation / InProc; never a bus
    -> application use-case    // maps PL, runs command or query
      -> write SPI.save(tx)    // aggregate + optional in-cell projections + outbox INSERT
      -> commit
      -> DomainEventPublisher  // in-cell only, after commit
infrastructure drain
  -> SELECT … FOR UPDATE SKIP LOCKED
  -> map PL integration event
  -> consumer InProc / later queue  // consumer SPI, not Open Host
```

### Outbox SQL (cell-owned)

Insert on the same `sqlx::Transaction` as `save`.

`Transaction` rolls back on drop if `commit` is not called.

A failed command therefore cannot leave an outbox row.

Drain claim (Postgres 18):

```sql
SELECT id, event_type, payload
FROM <cell>_outbox
WHERE published_at IS NULL
ORDER BY created_at
FOR UPDATE SKIP LOCKED
LIMIT 100;
```

`SKIP LOCKED` skips rows another drain session already locked.

Postgres documents this as the way to avoid lock contention with multiple consumers.

It is not a general-purpose consistent snapshot.

That is the correct trade for an at-least-once relay.

Mark published in the same drain transaction after the in-process (or later queue) publish attempt succeeds.

If the process dies after publish and before the mark, the next drain redelivers.

Consumers must be idempotent.

Do not have the consumer cell read this table.

### Optional notify

`NOTIFY` inside the save transaction delivers only after commit.

Aborted saves emit nothing.

`sqlx` 0.9.0 `PgListener` listens on a channel and auto-reconnects.

Notifications that arrive while the connection is down are lost.

The drain therefore polls even when notify is enabled.

### EventEmitter equivalent

Naboo's adapter is EventEmitter2 behind a write-side events SPI.

Rust SOTA for that SPI is a cell-local list of `async` handlers registered when the cell boots.

After commit, the adapter awaits each matching handler.

`tokio::sync::broadcast` is the wrong equivalent.

It drops lagged messages.

`event-emitter-rs` is a 2020 stringly clone.

Do not put domain events on `LISTEN`/`NOTIFY`.

Notify is a wake-up, not a payload bus.

Postgres tells you to put structured data in a table and send a key.

That table is the outbox.

## Sources

- crates.io crate records (User-Agent fetch, 2026-09-11): `cqrs-es` 0.5.0, `postgres-es` 0.5.0, `eventastic` 0.5.0, `eventastic_postgres` 0.5.0, `disintegrate` 4.0.0, `disintegrate-postgres` 4.0.1, `esrs` 0.18.0, `es-entity` 0.12.21, `distributed` 4.12.1, `thalo` 0.8.0, `eventually` 0.4.0, `cqrs` 0.3.1, `eventstore` 4.0.0, `kurrentdb` 1.2.0, `eventsourcingdb` 2.0.8, `messagebus` 0.15.2, `event-emitter-rs` 0.1.4, `async-event-emitter` 0.1.5, `sqlxmq` 0.6.0, `pgmq` 0.33.7, `graphile_worker` 0.13.5, `apalis` 0.7.4, `outbox-relay` 0.1.0, `sqlx` 0.9.0, `tokio` 1.53.1, `diesel` 2.3.13, `sea-orm` 2.0.2. API: `https://crates.io/api/v1/crates/<name>`
- https://docs.rs/cqrs-es/0.5.0/cqrs_es/
- https://docs.rs/cqrs-es/0.5.0/cqrs_es/struct.CqrsFramework.html
- https://docs.rs/cqrs-es/0.5.0/cqrs_es/trait.Query.html
- https://docs.rs/postgres-es/0.5.0/postgres_es/
- https://docs.rs/eventastic/0.5.0/eventastic/
- https://docs.rs/eventastic/0.5.0/eventastic/repository/index.html
- https://docs.rs/disintegrate/4.0.0/disintegrate/
- https://docs.rs/disintegrate-postgres/4.0.1/disintegrate_postgres/
- https://docs.rs/esrs/0.18.0/esrs/
- https://docs.rs/esrs/0.18.0/esrs/bus/index.html
- https://docs.rs/es-entity/0.12.21/es_entity/
- https://docs.rs/sqlxmq/0.6.0/sqlxmq/
- https://docs.rs/pgmq/0.33.7/pgmq/
- https://docs.rs/graphile_worker/0.13.5/graphile_worker/
- https://docs.rs/sqlx/0.9.0/sqlx/struct.Transaction.html
- https://docs.rs/sqlx/0.9.0/sqlx/postgres/struct.PgListener.html
- https://docs.rs/tokio/1.53.1/tokio/sync/broadcast/index.html
- https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE (Postgres 18 `SKIP LOCKED`)
- https://www.postgresql.org/docs/current/sql-notify.html (Postgres 18 `NOTIFY` commits with the transaction)
- https://www.postgresql.org/docs/current/explicit-locking.html
- https://github.com/serverlesstechnology/cqrs
- https://github.com/jdon/eventastic
- https://github.com/disintegrate-es/disintegrate
- https://github.com/GaloyMoney/es-entity
- https://github.com/patrickleet/distributed
- Naboo law (port, not copy): `/Users/pierrelecorff/Projects/naboo/docs/architecture.md` chapters 4, 5, 7, 16
