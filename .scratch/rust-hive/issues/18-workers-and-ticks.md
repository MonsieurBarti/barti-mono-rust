# Where do ticks, queues, and workers live?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 08, 10, 14

## Question

Port naboo chapter 16. There are no legacy neighbour cron modules.

Locked: outbox drain is producer infrastructure, not a presentation worker. Communication: [How do cells communicate in Rust hive-strict?](08-hive-strict-communication.md). REST folders: [What is the public driving-adapter surface?](10-public-driving-adapters.md). Tick ports are unpublished and take no `actor_id`. CQRS must already be decided: [How does CQRS work inside a Rust cell?](14-cqrs-inside-a-cell.md).

Decide: where ticks and queue consumers live; that a tick lists due work and enqueues, and does not run a tenanted command; one process versus a second worker process; a queue crate or skip for v1.

Do not pick an observability vendor here. Do not write the chapter here.

## Answer

Ticks live in `presentation/ticks/<use-case>/`. Thin driving adapters. They call an unpublished tick API port. No `actor_id`. No pool.

A tick is an unpublished command. It lists due rows through a read SPI and enqueues through a cell WorkSink. It does not `save` an aggregate. It must not run a tenanted command.

WorkSink has one method per work type. Each takes that work’s Published Language struct, including `work_key`. The sqlx adapter fills `work_type` and inserts into that cell’s `work` table. Unique `(work_type, work_key)`. Duplicate enqueue is `ON CONFLICT DO NOTHING`. No FIFO. Payload is PL JSON. `correlation_id` is copied onto the row. `actor_id` is present only when the work port is a REST twin. Integration events stay on the outbox. No queue crate. No pgmq, apalis, or sqlxmq.

Work drain is infrastructure, same grain as the outbox. The cell owns the table, claim SQL, and loop body, and exports `spawn_work_drain`. `app` binds unpublished work ports, spawns, owns poll interval and shutdown. One drain task per cell per process. Claim `FOR UPDATE SKIP LOCKED`, call one work port by `work_type`, mark done or backoff. Poison row does not block the head. Cell does not spawn at `new`. System work ports take no `actor_id`.

Cell exports `spawn_ticks`. Each tick is `tokio::time::interval` calling the presentation adapter. Interval is a cell constant. No cron crate. `app` spawns and shuts down. Composition-root env skips `spawn_ticks` only. Work drains still spawn in serve. One `app` process. HTTP, ticks, outbox drains, and work drains share it. A second worker binary is out.

Duplicate ticks across replicas are legal. No leader lock. Once-work uniqueness is the work-table claim.

Tick mints a UUIDv7 `CorrelationId` and sets `source` to `tick`. Drain keeps that id and sets `source` to `work`. One wide event per tick invocation and per work item. Fields: `msg`, `correlationId`, `actorId` when present, `duration_ms`, envelope `type` on errors, `source`. No HTTP method/path/status. Domain events do not carry `CorrelationId`. Logger mix in `app` is `api` | `webhook` | `tick` | `work`.

Tick use-case and work drain are application integration: real sqlx, fake leaving SPIs, no HTTP, no `app`. Tick presentation adapter has no e2e floor. Cell tests never boot `app` and spawn neither loop.

A cell with no ticks has no `work` table, no `spawn_ticks`, and no `spawn_work_drain`.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Tick**, **Work**, **WorkSink**, **Work drain**.


## Comments

### Round 1

Four arrows accepted (user: yes):

- Q1 A: tick lists due work and enqueues. Once-work is a cell-owned Postgres work table. No queue crate. Integration events stay on the outbox.
- Q2 A: one `app` process. HTTP, ticks, outbox drains, work drains. Second worker binary is out. Env skips ticks.
- Q3 A: `presentation/ticks/<use-case>/`. Work drain is infrastructure beside the outbox, not presentation. Drop `presentation/workers/`.
- Q4 A: duplicate ticks across replicas are legal. No leader lock. Once-work uniqueness is the work-table claim.

Local SNS/SQS stand-ins checked: pgmq (SQS-on-Postgres, shared `pgmq` schema), apalis 1.0-rc + apalis-postgres (worker framework, `PostgresStorage::setup` tables), sqlxmq (`mq*` schema, job macros). Rejected for cell schema isolation and hexagon ownership. Replacement is the cell work table plus the outbox claim loop.

### Round 2

Six arrows accepted (user: lgtm):

- Q5 A: unpublished tick command. Read SPI lists due. WorkSink enqueues. One `work` table per cell that has ticks. Unique `(work_type, work_key)`. ON CONFLICT DO NOTHING. No FIFO. No aggregate save.
- Q6 A: cell exports `spawn_work_drain`. `app` binds unpublished work ports, spawns, shutdown. One drain per cell per process. Claim one row, call one work port.
- Q7 A: cell exports `spawn_ticks`. `tokio::time::interval`. `app` spawns and shuts down. Env skips ticks. No cron crate.
- Q8 A: system work takes no `actor_id`. REST-twin work may carry `actor_id` on the row.
- Q9 A: tick mints UUIDv7 `CorrelationId`, `source` `tick`. Work row copies it, drain `source` `work`. One wide event per tick and per work item.
- Q10 A: tick use-case and work drain are application integration. Tick adapter has no e2e floor. No `app`. No queue crate.

### Round 3

Two arrows accepted (user: lgtm):

- Q11 A: WorkSink has one method per work type. Each takes that work’s PL struct. Adapter fills `work_type`.
- Q12 A: composition-root env skips `spawn_ticks` only. Work drains still spawn in serve. Cell tests spawn neither.

### Round 4

Two arrows accepted (user: lgtm):

- Q13 A: record the draft, close the ticket, write those glossary terms.
