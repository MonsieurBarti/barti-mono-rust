# How are mutating REST commands made idempotent?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 10, 17

## Question

Pin hive law for retries of mutating REST, if any.

Locked: `CorrelationId` is not an idempotency key. Observability: [How do we observe a REST hive process?](17-observability.md). Public HTTP is REST: [What is the public driving-adapter surface?](10-public-driving-adapters.md). Integration-event consumers are already idempotent: [How do cells communicate in Rust hive-strict?](08-hive-strict-communication.md).

Decide: whether a hive-wide `Idempotency-Key` header exists; where the store lives; what a replay returns; whether expected revision is law or per-cell.

Do not design tick idempotency here. Tick and queue lines are [Where do ticks, queues, and workers live?](18-workers-and-ticks.md). Do not write the chapter here.

## Answer

Human POST and PATCH require header `Idempotency-Key`. PUT, DELETE, and GET do not. `CorrelationId` is not this key. Do not echo `Idempotency-Key`.

The key is an opaque non-empty string, max 255. Missing, empty, or over-length is `VALIDATION_FAILED` with `violations[]` on `Idempotency-Key`. Presentation. Never enters a cell. Never mint. Identity `401` still runs first.

The API port takes `idempotency_key: String` like `ActorId`. REST copies the header. InProc mints a fresh UUIDv7 per call.

Store is table `idempotency_key` in that cell’s schema. Only cells with human POST/PATCH. Unique `(actor_id, key)`. Not kernel. Not `app`. Other columns are adapter-private.

Decode is the first act. Idempotency SPI `get` is a committed read. Hit plus matching fingerprint: return the stored `Result`. Hit plus mismatch: `VALIDATION_FAILED`. Miss: run the command. Write SPI `save` used by that command takes `Option` of the record. Ticks and fan-out pass `None`. Cells without the table keep two-argument `save`. The sqlx adapter writes the row on the same `Transaction` as the aggregate, outbox, and extra in-cell tables. No unit-of-work SPI.

Fingerprint is a deterministic checksum of port identity plus decoded input PL. Hash algorithm is not hive law.

Store success and domain 4xx. Do not store 5xx. Garde failures never reach the row. No in-progress row. No TTL. Unique at commit. `23505` on this unique is `*_CONFLICT` → 409. A retry after commit replays.

No hive-wide `If-Match` or ETag. Stream expected version stays [Do we event-source, where, and what is the contract?](15-event-sourcing.md). State tables MAY version. Mismatch is `*_CONFLICT` → 409.

Webhook POST does not use this header. The driving adapter extracts the vendor event id. Unpublished port takes it. Unique in the same transaction as the effect. Duplicate returns ok. Handler chooses 200 or 204. Separate table. Table name is a cell choice.

Command pipeline stays a per-cell choice. This header is not a pipeline. Each HTTP attempt still emits one wide event, including a replay.

Missing header is cell e2e on `router`. Replay and `23505` are application integration, real sqlx, fake leaving SPIs.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Idempotency-Key**.

## Comments

### Round 1

Three arrows accepted (user: lgtm):

- Q1 A: hive law. Human POST and PATCH require header `Idempotency-Key`. PUT, DELETE, and GET do not.
- Q2 B: no REST `If-Match` / ETag as hive law. Stream expected version stays ticket 15. State tables MAY version. Mismatch is `*_CONFLICT` → 409.
- Q3 B: webhook use-case is idempotent on the vendor event id, unique in that cell’s schema, same grain as `processed_event`. Not the human header.

### Round 2

Four arrows accepted (user: lgtm):

- Q4 A: cell schema, only cells with human POST/PATCH. Port takes `idempotency_key: String`. Use-case writes in the same transaction as `save`. Cell SPI, sqlx adapter. Not kernel. Not `app`.
- Q5 A: opaque non-empty string, max 255.
- Q6 A: missing, empty, or over-length is `VALIDATION_FAILED` on `Idempotency-Key`. Never enters a cell. Never mint. Ticket 10 stays.
- Q7 A: webhook adapter extracts vendor event id. Unpublished port takes it. Unique in the same transaction. Duplicate returns ok. Separate from the human table. Table name is a cell choice.

### Round 3

Six arrows accepted (user: lgtm):

- Q8 A: unique `(actor_id, key)` per cell. Fingerprint is port identity plus decoded input PL. Mismatch is `VALIDATION_FAILED`. Hash algorithm is not hive law.
- Q9 A: use-case returns the stored `Result`. Store success and domain 4xx. Do not store 5xx. Garde failures never reach the row.
- Q10 A: no in-progress row. Unique at commit. `23505` → `*_CONFLICT` → 409. Retry after commit replays.
- Q11 A: no TTL in v1.
- Q12 A: port always takes `idempotency_key: String`. InProc mints a fresh UUIDv7 per call.
- Q13 A: do not echo `Idempotency-Key`.

### Round 4

Two arrows accepted (user: lgtm):

- Q14 A: Idempotency SPI is `get`. Write SPI `save` on a human POST/PATCH command takes `Option` of the record. Adapter writes the row on the same `Transaction`. No UoW. Cells without the table keep two-argument `save`.
- Q15 A: hive table name `idempotency_key`. Unique `(actor_id, key)`. Other columns adapter-private. Webhook tables stay cell-named.

### Round 5

Q16 A accepted (user: lgtm). Draft recorded. Glossary written. Ticket closed.
