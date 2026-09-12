# Architecture

Greenfield Rust hive-strict process. Host product is a B2B freight brokerage.

ADRs stay living law. This file teaches. They must agree.

Glossary lives in [`CONTEXT.md`](../CONTEXT.md). This file uses those terms.

## Stack pins

Pin grain is majors. [ADR 0001](adr/0001-stack-pins.md) owns minors and patches. Bump a pin by changing that ADR.

| Pin                    | Major        |
| ---------------------- | ------------ |
| rustc                  | 1.98         |
| edition                | 2024         |
| `axum`                 | 0.8          |
| `tokio`                | 1            |
| `hyper`                | 1            |
| `tower`                | 0.5          |
| `tower-http`           | 0.6, not 0.7 |
| `sea-orm`              | 2            |
| `sea-orm-migration`    | 2            |
| `sea-orm-cli`          | 2            |
| `sqlx`                 | 0.9, driver  |
| `serde` / `serde_json` | 1            |
| `garde`                | 0.23         |
| `time`                 | 0.3          |
| `uuid`                 | 1            |
| `tracing`              | 0.1          |
| OpenTelemetry / OTLP   | 0.32         |
| `cargo-deny`           | 0.20         |
| `cargo-nextest`        | 0.9          |
| `cargo-llvm-cov`       | 0.9          |

Kernel depends on `time` only. It stays sqlx-free and sea-orm-free. Cells import none of `tracing`, OpenTelemetry, or a task-local crate.

Not pinned: a JWT crate, a CQRS bus crate, a queue crate, a cron crate, an OpenAPI crate, a GraphQL crate, an MCP crate, testcontainers, mockall as law, `tonic`.

## Out of scope

- Implementing the freight product. This spec hands off to implementation.
- Extracting a cell into another deployable.
- Frontend packages.
- Gateway product, IdP vendor, and a roles catalog.
- Auth issuance REST and SSO pages in this process.
- Rate limiting.

## Reading

Each chapter is Context, Decision, and Consequences, plus a link to its grill. Cells are written `<cell>`. Domain folders are written `<domain>`. Path target is `crates/cells/<domain>/<cell>`. A domain folder is a path segment, not a crate. Skim chapter 1 first.

GraphQL, MCP, and legacy promotion are not chapters.

---

## 1. Language

Grill: [What ubiquitous language does the Rust hive keep?](../.scratch/rust-hive/issues/06-rust-hive-glossary.md) · ADR: [Language](adr/0002-language.md)

### Context

Everyday word is **cell**. Four in-cell layers stay. Packaging word is **crate**. A **domain folder** is a path segment.

### Decision

Presentation and InProc call an API port. The application use-case implements that API port. Adapters never implement an API port.

sea-orm, vendor, and InProc adapters implement an SPI.

Open Host is the set of API ports this cell exports. An integration event is not Open Host.

InProc is composition-root infrastructure that calls a provider API port. An in-memory test double is not InProc.

A domain folder is a path segment. It is not a bounded context.

The cell-edge envelope is `{ type, context }`. It is not a command wrapper.

Terms and Avoid lists live in [`CONTEXT.md`](../CONTEXT.md). Replacements this hive uses:

| Died                                                       | Lives                           |
| ---------------------------------------------------------- | ------------------------------- |
| Nest module                                                | crate                           |
| AppModule                                                  | composition root (`app` binary) |
| mongoose adapter                                           | sea-orm adapter                 |
| Zod                                                        | codec                           |
| sheriff / dependency-cruiser                               | import wall                     |
| collection                                                 | table                           |
| promote, slice, neighbour, legacy bridge, hex platform kit | (gone)                          |

Fatten stays.

#### Naming

Path target is `crates/cells/<domain>/<cell>`. Package name is the bare cell name. Kind folders, then a grouping key. One crate per cell. No per-aggregate crate. No `open-host/`. No generic `vendors/`. No cell-root `ports/`. No `Cargo.toml` at the domain folder.

```
crates/
  kernel/
  cells/<domain>/<cell>/
    src/
      lib.rs
      domain/
        api/
        spi/
        entities/
        events/
        errors/
      application/
        commands/
        queries/
        event-handlers/
      infrastructure/
      presentation/
        http/
        ticks/
  app/
    src/
      main.rs
      wiring/
      inproc/
      http.rs
      telemetry.rs
```

| Kind folder                           | Group by                      |
| ------------------------------------- | ----------------------------- |
| `domain/entities`, `events`, `errors` | Entity                        |
| `domain/api`                          | Use-case or integration event |
| `domain/spi`                          | Consumer need                 |
| `application/commands`, `queries`     | Use-case                      |
| `application/event-handlers`          | Need                          |
| `presentation/http`                   | Use-case                      |
| `presentation/ticks`                  | Use-case                      |
| `infrastructure/`                     | The SPI it implements         |

SPI folder is this cell's need, never the provider's cell name. InProc lives in `app/inproc/<consumer>/<provider>.rs`.

Tests sit next to the unit. Lane follows the layer. Chapter 10.

A third-party folder (`infrastructure/<vendor>/`) exists only in cells that talk to that system. Domain and application never import the vendor crate.

HTTP and InProc reach the same command. The API-port trait is the token they call. The application struct implements that token.

### Consequences

No repo-root `test/` that spans cells. Runner is chapter 10.

---

## 2. Cell

Grill: [How is a cell packaged, and what is the composition root without Nest?](../.scratch/rust-hive/issues/07-cell-packaging-and-composition-root.md) · ADR: [Cell](adr/0003-cell.md)

### Context

One cell is one bounded context, one hexagon, the tests for that hexagon, and exclusive tables in one schema. Multi-aggregate is allowed. Team ownership is not an identity test. A domain folder organises cells on disk. It is not a fourth kind.

### Decision

Target path is `crates/cells/<domain>/<cell>`. Identity is the exclusive language, the exclusive tables, and the Open Host API ports. Coverage follows the cell.

Three kinds:

1. **Cell** — `crates/cells/<domain>/<cell>`.
2. **Composition root** — the `app` binary that imports every cell and binds leaving SPIs (chapter 3). Not a business crate.
3. **Kernel** — library at `crates/kernel` (chapter 15). No Open Host. No exclusive datastore. A domain-level shared folder is not a cell.

A directory under `crates/cells/` with no `Cargo.toml` is a domain folder, not a cell.

New production business logic must be a cell.

**Too-small** (folder, not a cell): no exclusive language, or no exclusive tables, or exactly one consumer that speaks that same language. One consumer is legal when languages differ.

**Too-big** (split): two ubiquitous languages, or two data clusters that do not share invariants.

**Chatty**, counted on one driving-adapter invocation (one HTTP request):

1. Sync chain across `≥3` cells (A→B→C).
2. RPC trap: more than one provider API port call to the same provider. Count at the InProc adapter, not at the consumer SPI.

Detect in design review. Wrong boundary is a review reject. No hop-count CI.

Remedy, in order:

1. Fatten the provider Open Host so that invocation makes one API call.
2. If the need is a query, give the consumer a local read model.
3. Merge only when the fattened API would be the other cell's whole language, or they need same-request write atomicity. Exclusive tables forbid a distributed transaction. That need is a merge signal.

Open Host is `domain/api/`, not HTTP.

First cells are chapter 16.

### Consequences

Import wall is chapter 3. Postgres isolation is chapter 6.

---

## 3. Composition root

Grill: [How is a cell packaged, and what is the composition root without Nest?](../.scratch/rust-hive/issues/07-cell-packaging-and-composition-root.md) · ADR: [Composition root](adr/0004-composition-root.md)

### Context

Virtual workspace. Hand-rolled DI. One process.

### Decision

`crates/app` is the composition root. Files: `main.rs`, `wiring/<cell>.rs`, `inproc/<consumer>/<provider>.rs`, `http.rs`, `telemetry.rs`. `app` path-depends on every cell crate. Explicit wiring only. No aggregator crate that re-exports cells. The domain folder has no `Cargo.toml`.

One rlib per cell. Intra-cell layers are modules: `domain/` (with `api/` and `spi/`), `application/`, `infrastructure/`, `presentation/`. The cell crate may import kernel, its own four layers, and axum in `presentation/`. It must not import another cell crate or `app`.

`lib.rs` re-exports Open Host API-port traits, their Published Language types, the leaving SPI traits, and `new`. Writes may sit in the Open Host set. Tick, webhook, and REST-only ports that other cells must not call stay `pub(crate)`. SPI implementations, entities, and persistence stay private.

Leaving SPIs bind as generic type parameters on the cell struct, not `Arc<dyn _>`. AFIT traits are not dyn-compatible. One in-process adapter does not earn a vtable and a box per hop.

`app` constructs named pools, clocks, Logger, Metrics, and InProc adapters, then calls `new`. The static cell crate never names a provider cell.

Inter-cell permission is which Open Host tokens `app` binds to which consumer InProc adapter. Different consumers get different ports and different Published Language shapes. No Policy SPI between cells. No caller identity on the port.

Human AuthN/AuthZ stays at the process edge (chapter 12). Cells receive `ActorId`. The domain folder is not that edge.

Import wall: rustc crate privacy, `pub(crate)`, `unreachable_pub`, and cargo-deny `bans.deny` with `wrappers = ["app"]` per cell crate. Reject cell to cell, cell to `app`, kernel to cell. Features are not a wall.

InProc adapters live in `app`. No cell crate path-depends on another cell. There is no legal cell-to-cell edge to allow-list.

### Consequences

Tests construct the cell with fake SPIs. They never boot `app`. Hop-count stays review-only. Postgres isolation stays chapter 6. The domain folder is not a privilege boundary.

---

## 4. Communication

Grill: [How do cells communicate in Rust hive-strict?](../.scratch/rust-hive/issues/08-hive-strict-communication.md) · ADR: [Communication](adr/0005-communication.md)

### Context

Hive-strict for cell to cell. Same deployable. A cell's application never imports a foreign cell. That is a Cargo fact.

### Decision

The consumer owns the SPI under `domain/spi/`, in consumer primitive types. `app` InProc implements it by calling the provider API port and mapping Published Language both ways. Application, domain, and presentation import no foreign cell.

API ports and SPIs declare `fn m(&self, …) -> impl Future<Output = Result<…, Envelope>> + Send`. No `dyn`. No `async_trait`. No `trait_variant`. Presentation is generic over the cell's SPI type parameters.

`domain/api/<use-case>/` owns the PL structs, the garde derives, and `decode`/`encode`. The application use-case implements the API port. Its first act is `decode(input)`. Inner domain and application never `use serde` or `use garde`. The cell crate may depend on both. Presentation may deserialize HTTP JSON into the same PL struct, then call the port. The port still decodes. Consumer InProc does not revalidate a typed reply. Both-sides validation returns when the hop becomes HTTP.

The cell-edge envelope is a per-port error enum: `#[serde(tag = "type", content = "context", rename_all = "SCREAMING_SNAKE_CASE")]`. Variants carry primitive-only structs. Kernel owns only the `VALIDATION_FAILED` violations payload. `app` matching a provider error enum is exhaustive and legal. Cells never downcast `thiserror` domain errors.

Leaving SPI signatures name consumer-owned primitive structs, not value objects and not provider PL. InProc maps provider PL to those structs. The consumer application maps them to value objects. A producer's `IntegrationEventSink` names producer PL event structs.

Default edge is request/response. Domain events stay in `domain/events/` and never leave the cell. Integration events are PL under `domain/api/<event>/`. They are not Open Host of the producer. The consumer's inbound use-case port is ordinary Open Host of the consumer.

An integration event enters a consumer through that API port, implemented by an idempotent use case and exported from `lib.rs`. The subscriber in `app/inproc/<consumer>/<provider>.rs` maps provider PL to consumer PL and calls that port. `app` never writes consumer tables. A local read model is written by the consumer's own use case through its own write SPI.

The producer cell owns the outbox table, the claim SQL, the loop body, and a leaving `IntegrationEventSink` with one method per integration event. It exports `spawn_outbox_drain`. `app` binds the sink, spawns the task, owns poll interval and shutdown. The cell does not spawn at construction.

Every event carries `event_id` (uuid v7) and `event_type`. Each consumer keeps `processed_event` in its own schema, written in the same transaction as the effect. Duplicate delivery is a unique-index conflict and returns ok.

One drain task per producer cell per process. Sequential await over bound subscribers. Mark published when all return ok. Redeliver the whole row on any failure. `attempts` and `next_attempt_at` with backoff in the claim predicate. A poison row does not block the head of the queue. No ordering guarantee across rows.

### Consequences

Swap InProc for HTTP/gRPC later. Domain and application of both cells stay unchanged. Extracting a cell into another app is out of scope. Public mapping of `{ type, context }` is chapter 8. Money, id, and date wire format is chapter 14.

---

## 5. CQRS

Grill: [How does CQRS work inside a Rust cell?](../.scratch/rust-hive/issues/14-cqrs-inside-a-cell.md) · ADR: [CQRS](adr/0006-cqrs.md)

### Context

No bus crate. Presentation never sees a bus. Domain events never leave the cell.

### Decision

CQRS stays. No Command type. No Handler trait.

Each use-case is one application struct that implements that API-port trait. Files live under `application/commands/<use-case>/` or `application/queries/<use-case>/`. Presentation and InProc call the port. Integration constructs the struct. `decode` is the first act.

A command may return an id or outcome as Published Language. Errors stay the per-port envelope. A query never mutates. It injects read SPIs only. No `save`. No outbox insert. No domain-event publish.

Write SPI is `get_by_id` / `save` per aggregate. Replay lives in the adapter. The use-case never imports `sea_orm`. Multi-entity work lives in the use-case, not `domain/services/`. Two `save` calls are two transactions. Same-request atomicity means one aggregate. No unit-of-work SPI.

A query injects one read SPI per need and returns a plain read model. It never loads the write-side entity. Default reads may hit the write table through that read SPI.

The entity records domain events during behavior. After `save` returns, the use-case publishes `entity.pull_events()` through a cell write-side events SPI. Infrastructure awaits matching handlers by type, after commit. Handlers live in `application/event-handlers/<need>/` and stay in-cell. They are after-commit reactions that may lag, including fan-out to another use-case in this cell. They are not how a query's required read model is written. Extra in-cell read collections stay on chapter 7. Not `tokio::sync::broadcast`. Not a bus crate.

Command integration fakes the events SPI and asserts what was published. A handler is its own application integration test: real sea-orm adapter, fake leaving SPIs.

Command pipeline is a per-cell choice, not architecture law.

### Consequences

Driving adapters call the API port. Extra read tables wait on chapter 7.

---

## 6. Persistence

Grill: [How is Postgres isolation enforced per cell?](../.scratch/rust-hive/issues/09-postgres-cell-isolation.md), [Should the persistence adapter be an ORM?](../.scratch/rust-hive/issues/25-orm-adapter.md) · ADR: [Persistence](adr/0007-persistence.md)

### Context

One database. Exclusive tables are cell identity (chapter 2). Compile-time wall is chapter 3.

### Decision

Schema per cell. Two LOGIN roles per cell: the migrator role owns the schema and runs DDL; the cell role is DML only on that schema.

Ops provisions the database, both LOGINs, `CREATE SCHEMA … AUTHORIZATION` migrator, GRANTs, and default privileges. `GRANT … ON ALL TABLES IN SCHEMA` is legal because the schema is the exclusive unit. `REVOKE ALL ON SCHEMA public FROM PUBLIC`. No `CREATE` on the schema for the cell role. No `SET ROLE`. Cell `migrations/` only change tables and indexes in that schema. A new table is a cell migration. A new schema is a role change in the same change. Cell migrations never `CREATE ROLE` or `GRANT`. Migrations are modules in the cell crate. No extra `migration` crate. Default `public` is a review reject.

`app` has a migrate entry and a serve entry. Migrate env holds migrator DSNs and runs each cell's `sea-orm-migration` Migrator with the schema passed explicitly. Serve env holds only cell-role DSNs and never migrates.

Serve: `app` opens a named `DatabaseConnection` from the cell-role DSN, wraps it in a cell-private newtype, and passes it only into that cell's sea-orm adapter. Cell `new` never takes `DatabaseConnection`. Unnamed, shared, or default connection is a review reject. Pool size is env. Domain SPI stays pool-free. `sea_orm` types stay in `infrastructure/`. `DatabaseConnection` owns a `sqlx::Pool`. Cells depend on `sea-orm`, not sqlx.

A transaction is `begin()` on that cell's connection. Two connections cannot share a `BEGIN`. Prepared transactions and 2PC stay out. Same-request write atomicity across cells is a merge signal. InProc is a second transaction on the other cell's connection.

Compile-time wall: crate privacy only. No `query!`. Every entity sets `schema_name`. `search_path` is a belt on the role. `set_schema_search_path` is not the wall. A planted foreign schema is a review reject. Runtime leftover is SQLSTATE `42501`, an operational bug, not a cell-edge envelope. Proof that GRANT bites is chapter 10.

Handwritten `Model` / `ActiveModel` live in `infrastructure/`, `pub(crate)`. The adapter maps them to domain types. Empty `ActiveModelBehavior`. Domain never imports `sea_orm`. No `sea-orm-cli generate-entity`.

The adapter maps `23505` through `DbErr` to conflict.

The domain folder is not a privilege boundary. Kernel has no exclusive datastore. Diesel `table!` stays rejected.

### Consequences

Pool size is env. GRANT proof lives in `crates/app/tests`.

---

## 7. Event sourcing

Grill: [Do we event-source, where, and what is the contract?](../.scratch/rust-hive/issues/15-event-sourcing.md) · ADR: [Event sourcing](adr/0008-event-sourcing.md)

### Context

Event sourcing is per-aggregate, never hive-wide. Temporal queries do not pick storage. The test is the domain job, not the word financial.

### Decision

The stream is the write model only where a write must be reversible without silent edit. Other aggregates stay state tables. One cell may mix both. First cells stay state unless that aggregate already fails the test. Do not inventory aggregates.

One row per event in a table in this cell's schema. Same database, cell role, and named pool. Unique `(stream_id, version)`. Append is `INSERT`. Unique violation (`23505`) is the concurrency conflict. A new event table is a cell migration. No extra schema.

Snapshots are optional in a second table in that schema. Replay without a snapshot stays correct.

EventStoreDB and a second database stay out.

Application uses the same write SPI: `get_by_id` / `save`. Replay and append live in the sea-orm adapter. Expected version sits on the aggregate the adapter loaded. One `begin()` on that cell's connection writes the stream or state row, the outbox rows, and extra in-cell read tables.

Queries never replay. They use a read SPI. The stream answers `get_by_id` only. Extra in-cell read tables are allowed when this cell's queries cannot use the write row. Same transaction as `save`. After-commit event handlers do not write a query's required read model.

Stored events stay in-cell. Domain events stay in-cell. They publish after persist through the write-side events SPI.

Event rows are immutable. New types for new facts. Adapter upcasters when an old type must be read as a new shape. Weak readers ignore unknown fields. No `UPDATE` of payloads. Copy-and-replace is a migration.

No foreign stream subscribe.

### Consequences

Drain scheduling is chapter 13.

---

## 8. REST

Grill: [What is the public driving-adapter surface?](../.scratch/rust-hive/issues/10-public-driving-adapters.md), [How are mutating REST commands made idempotent?](../.scratch/rust-hive/issues/23-rest-command-idempotency.md) · ADR: [REST](adr/0009-rest.md)

### Context

Public HTTP is REST on axum. GraphQL is not in this architecture. MCP is not in this architecture. A later client or agent surface is a new grill.

### Decision

Cell `presentation/` depends on axum. Handlers live in `presentation/http/<use-case>/`. `lib.rs` re-exports `pub fn router` beside `new`. That function is not Open Host. `app/http.rs` nests and merges. Kernel stays axum-free. Cell tests hit that router with fake SPIs and never boot `app`. Cell name and domain folder are not path segments. A cell chooses its prefix. Collision is a review reject.

Open Host stays cell-to-cell. A public REST handler may call a `pub(crate)` port. Tick, webhook, and any REST-only command other cells must not call stay `pub(crate)`.

Handlers extract `actor_id` as a string. The body omits it. JSON deserializes into the Published Language struct. The port still `decode`s. Success is PL as `application/json`. The handler chooses 200, 201, or 204. 204 has no body. Success is never an envelope. No business logic. The token never enters the cell.

Public paths are resources. Presentation maps verb plus path onto a use-case API port. No kebab use-case paths. No JSON:API. No `/v1` prefix. Additive JSON only. No pagination law. No OpenAPI crate pin.

Hide `context`. Cell presentation owns `type` → public string. `app` merges catalogs at boot. Unknown `type` is a generic string plus a server warn. Sensitive types stay generic. One composition-root suffix map, no per-cell override. Domain error carries no status.

| Envelope suffix or type | HTTP |
| ----------------------- | ---- |
| `*_NOT_FOUND`           | 404  |
| `*_CONFLICT`            | 409  |
| `UNAUTHENTICATED`       | 401  |
| `FORBIDDEN`             | 403  |
| `RATE_LIMITED`          | 429  |
| `VALIDATION_FAILED`     | 400  |
| else                    | 500  |

Public error document is RFC 9457 `application/problem+json`: `type` (envelope type string, not a URI), `status`, catalog `detail`, `instance` (request path), `correlationId` when present. No `title`. No `context`. `VALIDATION_FAILED` adds `violations: { path, code, message }[]` with no submitted values.

Unhandled: HTTP 500, `type` is `about:blank`, `detail` is `Internal server error`, `correlationId` when present. Logs keep stack and `context`.

Health lives on `app`, not a cell, not Open Host. Webhook, PDF, and stream handlers are unpublished REST in `presentation/http/`.

#### Idempotency

Human POST and PATCH require header `Idempotency-Key`. PUT, DELETE, and GET do not. `CorrelationId` is not this key. Do not echo `Idempotency-Key`.

The key is an opaque non-empty string, max 255. Missing, empty, or over-length is `VALIDATION_FAILED` with `violations[]` on `Idempotency-Key`. Presentation. Never enters a cell. Never mint. Identity `401` still runs first.

The API port takes `idempotency_key: String` like `ActorId`. REST copies the header. InProc mints a fresh UUIDv7 per call.

Store is table `idempotency_key` in that cell’s schema. Only cells with human POST/PATCH. Unique `(actor_id, key)`. Not kernel. Not `app`. Other columns are adapter-private.

Decode is the first act. Idempotency SPI `get` is a committed read. Hit plus matching fingerprint: return the stored `Result`. Hit plus mismatch: `VALIDATION_FAILED`. Miss: run the command. Write SPI `save` used by that command takes `Option` of the record. Ticks and fan-out pass `None`. Cells without the table keep two-argument `save`. The sea-orm adapter writes the row on the same transaction as the aggregate, outbox, and extra in-cell tables. No unit-of-work SPI.

Fingerprint is a deterministic checksum of port identity plus decoded input PL. Hash algorithm is not hive law.

Store success and domain 4xx. Do not store 5xx. Garde failures never reach the row. No in-progress row. No TTL. Unique at commit. `23505` on this unique is `*_CONFLICT` → 409. A retry after commit replays.

No hive-wide `If-Match` or ETag. Stream expected version stays chapter 7. State tables MAY version. Mismatch is `*_CONFLICT` → 409.

Webhook POST does not use this header. The driving adapter extracts the vendor event id. Unpublished port takes it. Unique in the same transaction as the effect. Duplicate returns ok. Handler chooses 200 or 204. Separate table. Table name is a cell choice.

Command pipeline stays a per-cell choice. This header is not a pipeline. Each HTTP attempt still emits one wide event, including a replay.

Missing header is cell e2e on `router`. Replay and `23505` are application integration, real sea-orm adapter, fake leaving SPIs.

### Consequences

AuthN mapping is chapter 12. Observability is chapter 11.

---

## 9. Validation

Grill: [Where does the codec validate versus domain invariants?](../.scratch/rust-hive/issues/16-codec-versus-invariants.md) · ADR: [Validation](adr/0010-validation.md)

### Context

The cell is garde-free and serde-free inside the inner hexagon.

### Decision

`domain/api/<use-case>/` owns input PL (`Deserialize` + garde + `deny_unknown_fields`), output PL (`Serialize`, no deny), `decode`, and the envelope enum. The use-case's first act is `decode(input)`. `decode` is the only function that names garde. It returns `Result<PlInput, Envelope>`. `Valid<T>` never leaves that module. Inner domain and application never `use serde` or `use garde`. Application maps `PlInput` to value objects, returns output PL, and maps domain errors to the envelope. Presentation serializes success and maps `serde_path_to_error` on HTTP JSON into the same `VALIDATION_FAILED` / `violations[]` document, then skips the port. InProc has no serde step. It still calls `decode`.

Codec failures are shape and field constraints on PL primitives, including cross-field rules that need no entity. Entity invariants, VO `TryFrom`, and uniqueness against a loaded row are domain or SPI and never `VALIDATION_FAILED`. The sea-orm adapter does not garde-parse this cell's own rows. A corrupt own row is a server bug. Infrastructure MAY serde+garde a foreign wire into SPI types. That failure is not `VALIDATION_FAILED`. Webhook: signature on the raw body, then this cell's input PL, then `decode`.

### Consequences

How a garde failure looks to the REST client is chapter 8. Money, ids, and dates are chapter 14.

---

## 10. Testing

Grill: [What are the test lanes, and what does each one boot?](../.scratch/rust-hive/issues/12-test-lanes.md) · ADR: [Testing](adr/0011-testing.md)

### Context

Hive: real objects, not mocks. Tests ship with the cell. Always-on Postgres. 80% line coverage per cell is in.

### Decision

Three lanes per cell, named by layer. Cargo homes differ from lane names.

**Unit** is domain. `#[cfg(test)]` next to the unit. Entities, value objects, errors, codec and persistence-mapper round-trips. No I/O. No doubles. Fluent builder next to the entity: `.build()` reconstructs, `.build_new()` creates, faker fills unused fields, never assert on faker output. Tests never call `SystemTime::now` or `Utc::now`. Tests use kernel `FakeClock`.

**Integration** is application. `#[cfg(test)]` inside the cell crate, in `integration` modules so nextest can exclude them from the unit profile. Direct `new` of the use-case with this cell’s real sea-orm adapter on the cell-role pool. Handwritten fakes for leaving SPIs. No HTTP. No `app`. This lane owns the sea-orm adapter against a real schema.

**E2E** is presentation. Cell `tests/` sees only the re-export set. Hits `new` + `router` with tower. Real sea-orm adapter, fake leaving SPIs. Never boots `app`. One command and one query per public use-case that has an HTTP handler. No handler unit tests. Unpublished REST is case by case, not a floor.

Cargo `tests/` is E2E, not application integration. Application tests must see `pub(crate)` `sea_orm` types.

GRANT proof lives in `crates/app/tests`. Open two cell-role pools. A planted cross-schema statement expects SQLSTATE `42501`. Not a product E2E. Not a coverage cell. Cell crates never name another cell’s tables. Cross-cell workflow tests are not a lane.

Always-on Postgres. Compose locally, CI service. Per-worker database. Production schema names, migrator LOGIN plus cell LOGIN, ops-shaped GRANTs. Fail fast if unreachable, before db lanes. Tests never start Postgres. Testcontainers is out. `#[sqlx::test]` is out.

A fake is a handwritten struct next to the SPI, `#[cfg(test)]`. Not InProc. mockall is not law. No workspace fake crate. An in-memory own-store exists only so the SPI contract has two adapters. Unit has no doubles.

A contract is a `pub` function next to the SPI, behind cell feature `contract`, off by default. Every adapter invokes it: fake and sea-orm adapter from cell `#[cfg(test)]`; InProc from `app` tests with the feature enabled on a `dev-dependencies` path. Prod `cargo build` does not enable it. Not a spec.

Minima: every command ≥1 happy path and ≥1 error (integration). Every domain invariant a rejection test (unit). Every persistence mapper a round-trip (unit). Every SPI method on that port’s contract.

Runner is cargo-nextest 0.9. Invocation is `cargo nextest run`. `cargo test` is a local escape hatch, not the gate. Unit profile needs no DSN and excludes `integration::` plus the `tests/` target. Db profile runs integration, e2e, and app GRANT and requires worker databases.

80% line coverage per cell is CI law via cargo-llvm-cov. Merge unit + integration + e2e. Exclude tests, fakes, builders, and contract modules. Kernel and `app` have no floor. CI fails the cell below 80%. The floor follows the cell, not the domain folder.

Event-sourcing persistence tests stay the application lane: real sea-orm adapter, in-memory double for the SPI contract only.

### Consequences

Cross-cell workflow tests are composition-root, later. Cell e2e never boots `app`.

---

## 11. Observability

Grill: [How do we observe a REST hive process?](../.scratch/rust-hive/issues/17-observability.md) · ADR: [Observability](adr/0012-observability.md)

### Context

The sink is ops, not a hive crate. Same deployable.

### Decision

`app` owns OpenTelemetry. `tracing` plus OTLP live in `telemetry.rs`. Cells import none of `tracing`, OpenTelemetry, or a task-local crate.

Kernel exports `Logger` and `Metrics` (chapter 15). Domain never logs and never counts. Application, infrastructure, and presentation may. Both ports are synchronous and non-throwing. `CorrelationId` and `ActorId` are not SPI arguments. `app` mixes `correlationId`, `actorId` when present, and `source` (`api`, `webhook`, `tick`, or `work`). No `organizationId`. No `ip`. No `userAgent` on log lines. Implementations live in `app` and bind as generics on `new`. Tests pass fakes. No per-cell trait files under `domain/spi/`.

Header `X-Correlation-ID`. Optional. Accept UUID v4 or v7. Generate UUIDv7 when missing. Never 400. Echo the header on every HTTP response, including health. Problem+json still carries `correlationId` in the body. Tag the OpenTelemetry root span with `correlationId`. The trace id is a different id. Do not copy `CorrelationId` into `trace_id`. Clients need not send `traceparent`. Baggage stays out until a second hive process. InProc reads request context. Domain events do not carry `CorrelationId`. `CorrelationId` is not an idempotency key.

Metrics names are `hive.<area>.<subject>`. Bounded tag `cell:`. Untrusted tag values allowlisted. Never hand-tag `env` / `service` / `version`. No `_total` suffix. `app` impl adds `cell:`.

One wide event per REST request that hits a cell handler, including unpublished webhooks. Not per InProc hop. Health emits none. `debug` on success at or under 3 s, `info` when slow, `warn` on 4xx, `error` on 5xx. Dotted `msg`. Production `LOG_LEVEL=info`. Fields: `msg`, `correlationId`, `actorId` when present, HTTP method and path, `duration_ms`, HTTP `status`, envelope `type` on errors, `source`. Never log JWT bodies, request or response bodies with user content, full names, emails, phones, or IPs. Redact keys at any depth: `password`, `token`, `secret`, `authorization`, `creditCard`, `creditCardNumber`, `cvv`, `accessToken`, `refreshToken`, `email`, `phone`, `phoneNumber`.

Tick and work wide events are chapter 13. Each HTTP attempt emits one wide event, including a replay.

### Consequences

Tick and queue lines are chapter 13.

---

## 12. Authentication

Grill: [How does AuthN/AuthZ work at the REST process edge?](../.scratch/rust-hive/issues/13-authn-rest-edge.md) · ADR: [Authentication](adr/0013-authentication.md)

### Context

A gateway sits in front of this hive. This process reads identity. It does not become an IdP.

### Decision

The gateway authenticates humans and allows routes. This process does not mint tokens, does not validate human Bearer JWTs, and does not serve SSO pages. Gateway product, IdP, and roles catalog are not this architecture.

`app` middleware copies gateway identity headers into request extensions. This bind is not public. Human REST requires `ActorId`. Health does not. Cell `router` has no AuthN layer. Cell e2e inserts `ActorId` via extension and never boots `app`.

Missing identity is HTTP 401, `application/problem+json`, `type` is `UNAUTHENTICATED`. It never enters a cell.

The API port takes `ActorId` as a string. Presentation extracts it. The body omits it. Roles and permissions do not enter the cell. Resource ownership lives in the use-case. `FORBIDDEN` comes from the use-case. No Policy SPI. No `organizationId` on the port.

Webhook routes are unpublished and sit off the human gateway JWT. The driving adapter verifies the vendor signature. Webhook ports take no `actor_id`. A bad or missing signature is 401 `UNAUTHENTICATED`. The cell never sees the secret.

Tick ports take no `actor_id`.

This architecture pins no JWT crate.

### Consequences

No product roles catalog. Worker `actor_id` is chapter 13.

---

## 13. Workers

Grill: [Where do ticks, queues, and workers live?](../.scratch/rust-hive/issues/18-workers-and-ticks.md) · ADR: [Workers](adr/0014-workers.md)

### Context

Outbox drain is producer infrastructure, not a presentation worker. Tick ports are unpublished and take no `actor_id`.

### Decision

Ticks live in `presentation/ticks/<use-case>/`. Thin driving adapters. They call an unpublished tick API port. No `actor_id`. No pool.

A tick is an unpublished command. It lists due rows through a read SPI and enqueues through a cell WorkSink. It does not `save` an aggregate. It must not run a tenanted command.

WorkSink has one method per work type. Each takes that work’s Published Language struct, including `work_key`. The sea-orm adapter fills `work_type` and inserts into that cell’s `work` table. Unique `(work_type, work_key)`. Duplicate enqueue is `ON CONFLICT DO NOTHING`. No FIFO. Payload is PL JSON. `correlation_id` is copied onto the row. `actor_id` is present only when the work port is a REST twin. Integration events stay on the outbox. No queue crate.

Work drain is infrastructure, same grain as the outbox. The cell owns the table, claim SQL, and loop body, and exports `spawn_work_drain`. `app` binds unpublished work ports, spawns, owns poll interval and shutdown. One drain task per cell per process. Claim `FOR UPDATE SKIP LOCKED`, call one work port by `work_type`, mark done or backoff. Poison row does not block the head. Cell does not spawn at `new`. System work ports take no `actor_id`.

Cell exports `spawn_ticks`. Each tick is `tokio::time::interval` calling the presentation adapter. Interval is a cell constant. No cron crate. `app` spawns and shuts down. Composition-root env skips `spawn_ticks` only. Work drains still spawn in serve. One `app` process. HTTP, ticks, outbox drains, and work drains share it. A second worker binary is out.

Duplicate ticks across replicas are legal. No leader lock. Once-work uniqueness is the work-table claim.

Tick mints a UUIDv7 `CorrelationId` and sets `source` to `tick`. Drain keeps that id and sets `source` to `work`. One wide event per tick invocation and per work item. Fields: `msg`, `correlationId`, `actorId` when present, `duration_ms`, envelope `type` on errors, `source`. No HTTP method/path/status. Domain events do not carry `CorrelationId`. Logger mix in `app` is `api` | `webhook` | `tick` | `work`.

Tick use-case and work drain are application integration: real sea-orm adapter, fake leaving SPIs, no HTTP, no `app`. Tick presentation adapter has no e2e floor. Cell tests never boot `app` and spawn neither loop.

A cell with no ticks has no `work` table, no `spawn_ticks`, and no `spawn_work_drain`.

### Consequences

Product job catalog is out. Deployment of a second worker process is out.

---

## 14. Published Language scalars

Grill: [How are Money, ids, and dates encoded in Published Language?](../.scratch/rust-hive/issues/19-pl-scalars.md) · ADR: [Published Language scalars](adr/0015-published-language-scalars.md)

### Context

Published Language is JSON primitives. Value objects stay inside the hexagon. Kernel `Money` is the shared-kernel domain type. Exception to “value objects stay inside the hexagon” is the kernel type itself, not the wire.

### Decision

PL Money is `{ amount, currency }`. `amount` is a JSON number, integer minor units, inside `±(2^53-1)`. `currency` is ISO 4217 alpha-3, uppercase. Signed amounts including zero are legal. No exponent. No IEEE fractional amount. No `bigint` on the wire. Unknown ISO code is `VALIDATION_FAILED`. Product allowlists stay out. Processor exponent quirks stay in that cell's vendor adapter.

Kernel `Money` is `i64` minor units (same cap) plus ISO currency. Same-currency `add`, `subtract`, `compare`. `allocate(weights)` only; remainder pennies to the first recipients; parts sum to the original. No `divide` that returns one `Money`. No FX method. `create` rejects unknown ISO and out-of-range amounts. `to_plain()` is `{ amount: i64, currency: String }` and is not serde. Kernel stays serde-free. ISO 4217 fraction digits live in kernel as data.

Domain, application, and this cell's sea-orm adapter MAY import kernel `Money` and `Instant`. Presentation, InProc, and `domain/api` MUST NOT. `domain/api` MAY import the ISO 4217 table. Application calls `Money::create` / `to_plain()` and formats instants. Decode never names `Money`.

Ids are opaque non-empty strings. New aggregates mint UUIDv7, lowercase, hyphenated (RFC 9562), via `uuid` 1. Application mints. Domain does not call `Uuid::now_v7`. Kernel has no generic `Id`. A cell MAY wrap. Reconstruct takes the stored string.

Instant PL is ISO-8601 datetime. Offset input is legal. Encode UTC `Z` with milliseconds. Decode requires seconds; fractional seconds are allowed. Nanoseconds are not on the wire. Kernel `Instant` is a newtype over `time::OffsetDateTime`.

Calendar date is `YYYY-MM-DD` on the wire and as the domain string. A cell MAY wrap. Never encode as midnight UTC.

Kernel `Clock` with `now()` returning `Instant`. `app` binds the real clock. Application passes that instant into the entity. Domain never reads the clock. Tests use a fake clock.

Postgres: `amount BIGINT`, `currency TEXT`, id `UUID`, instant `TIMESTAMPTZ`, calendar date `DATE`. Never `NUMERIC` for amount. Never `TIMESTAMPTZ` for a calendar date. A corrupt own row is a server bug, not `VALIDATION_FAILED`.

### Consequences

Price as a catalog type is out. Product currency allowlist is out.

---

## 15. Shared kernel

Grill: [What lives in the kernel crate?](../.scratch/rust-hive/issues/20-kernel-crate.md) · ADR: [Shared kernel](adr/0016-shared-kernel.md)

### Context

Target path is `crates/kernel`. The kernel is a library, not a cell. No Open Host. No exclusive datastore.

### Decision

Framework-free rlib of shared types and ports. It never depends on a cell. It stays serde-free, axum-free, sqlx-free, sea-orm-free, tracing-free, uuid-free. The only crate dep is `time`.

Exports exactly:

- `Money` as locked in chapter 14
- `Instant`, plus `checked_add_seconds(i64) -> Option<Instant>`
- `Clock` with `now() -> Instant`
- `SystemClock`
- `FakeClock` with `new(Instant)`, `set(Instant)`, `now()`
- `is_known_currency(&str) -> bool` and `fraction_digits(&str) -> Option<u8>`
- `Violation { path, code, message }` as a public struct, no serde
- `Logger`: `debug` / `info` / `warn` / `error`, each `msg: &str` plus `fields: &[(&str, &str)]`
- `Metrics`: `increment(name, value: u64, tags)` and `distribution(name, value: f64, tags)`

`Clock`, `Logger`, and `Metrics` are `Send + Sync`, synchronous, non-throwing, bound as generics on cell `new`. Not `dyn`. No per-cell trait files under `domain/spi/`.

`app` binds `SystemClock` and the Logger/Metrics implementations. Tests pass `FakeClock` and fake Logger/Metrics. Domain never logs, never counts, never reads the clock.

Forbidden in kernel: Envelope, Id, CorrelationId, ActorId, a Result alias, outbox helpers, pagination, problem+json, a Level enum, a Currency enum, macros, gauge.

Layer imports:

- Money / Instant — domain, application, this cell’s sea-orm adapter; not presentation, InProc, or `domain/api`
- Clock trait — application only
- Logger / Metrics — application, infrastructure, presentation; not inner domain, not `domain/api`
- Violation — `domain/api` only
- ISO functions — `domain/api` may; `Money::create` uses them inside kernel
- `SystemClock` / `FakeClock` — constructed in `app` and tests only

Each port envelope variant holds `Vec<Violation>` and serdes at the cell edge.

### Consequences

Composition-root wiring stays chapter 3.

---

## 16. Freight

Grill: [What freight ubiquitous language and first cells?](../.scratch/rust-hive/issues/21-freight-language-and-first-cells.md) · ADR: [Freight](adr/0017-freight.md)

### Context

Host product is a B2B freight brokerage. Sketch is post a load, quote, book, settle. Cell identity tests stay (chapter 2). Freight terms live in [`CONTEXT.md`](../CONTEXT.md).

### Decision

Domain folder `freight`. First cells: `loads`, `shipments`, `settlement`. Paths: `crates/cells/freight/loads`, `crates/cells/freight/shipments`, `crates/cells/freight/settlement`.

`loads` owns Load, Quote, and Stop. Quote is an aggregate, not a cell. A Load has a list of Stops. First cut is two. Consignee is a name and address on the delivery stop.

`shipments` owns Shipment. Book is an integration event from `loads`. Shipments copies Stops into a local read model. CustomerRate and CarrierRate lock on the Shipment at book. CarrierRate comes from the winning Quote. No asking amount on a Load.

`settlement` owns Invoice to the Shipper at CustomerRate and Payable to the Carrier at CarrierRate. It keeps a local read model of those rates.

First invariants are FTL road. Other modes fatten these cells. A new cell only when identity tests fail. No mode in a cell name. No reserved empty cells. No Booking aggregate. The broker is the system, not a party.

No `CONTEXT-MAP.md` until cell crates exist.

### Consequences

Implementing these cells is out of this map.
