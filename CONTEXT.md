# Hive

Greenfield Rust hive-strict process for a B2B freight brokerage.

## Language

**Cell**:
One bounded context, one hexagon, one crate, exclusive tables in one Postgres schema, and the tests for that hexagon.
_Avoid_: BC, cluster, hub, module-as-cell, Nest module, slice, promote, neighbour

**Domain folder**:
A path segment that groups cells. Not a crate, not a bounded context, not an auth edge.
_Avoid_: cluster, hub, treating it as a bounded context

**Bounded context**:
The DDD facet of a cell. Never a different unit.
_Avoid_: a second unit beside the cell

**Hexagon**:
The ports-and-adapters facet of a cell. Never a different unit.
_Avoid_: a second unit beside the cell

**Crate**:
Packaging only. One rlib per cell. Not a synonym for cell.
_Avoid_: module as a hive term, Nest module

**Composition root**:
The only place that imports every cell and binds leaving SPIs. The `app` binary.
_Avoid_: AppModule, FintechModule, CellsModule, DomainModule, treating `main` as a cell

**Open Host**:
The set of API ports this cell exports. Writes may sit in that set.
_Avoid_: `I{Name}Client`, calling an integration event Open Host

**API port**:
One use-case interface plus a Published Language codec. Presentation and InProc call it. The application use-case implements it.
_Avoid_: one file for the whole cell, an adapter implementing it

**Command**:
Write intent at an API port. May return an id or outcome. Not a type.
_Avoid_: Command struct, ICommandHandler, bus.execute, Handler

**Query**:
Read intent at an API port. Never mutates. Returns a plain read model.
_Avoid_: loading the write-side entity, Query struct, IQueryHandler

**SPI**:
Every driven port this cell owns: store, vendor, other cell. Cross-cell SPI arguments are consumer-owned primitives, not value objects and not provider Published Language.
_Avoid_: SPI-means-only-cross-cell, calling an API port an SPI, value objects on a leaving SPI, provider PL on a consumer SPI

**Write SPI**:
`get_by_id` / `save` for one aggregate. Replay lives in the adapter.
_Avoid_: find_by on this port, application importing sqlx, a unit-of-work port

**Stream**:
The write model for an event-sourced aggregate: immutable event rows in that cell's schema.
_Avoid_: adjunct journal, EventStore, treating a snapshot as the write model

**Read SPI**:
Query-side port. Never loads the write-side entity. Default reads may hit the write table through this port.
_Avoid_: query calling get_by_id on the write SPI

**Published Language**:
JSON primitives at the cell edge. Value objects stay inside the hexagon.
_Avoid_: kernel Money on the wire

**Codec**:
The cell-edge validator for Published Language, owned beside the API port.
_Avoid_: Zod, calling a value object a codec

**Driving adapter**:
Stimulus entry into a cell. Public HTTP is an axum handler under `presentation/http/`. A tick adapter lives under `presentation/ticks/`.
_Avoid_: GraphQL resolver, MCP tool, injecting a foreign cell from presentation, InProc as public HTTP, `presentation/workers`

**Driven adapter**:
An adapter that implements an SPI.
_Avoid_: calling an adapter an API port or an SPI

**sqlx adapter**:
The driven adapter that talks to this cell's Postgres schema through sqlx. It holds that cell's named pool.
_Avoid_: mongoose adapter, repository as the SPI, taking another cell's pool

**InProc adapter**:
Composition-root infrastructure that implements a consumer SPI by calling a provider API port, or a producer IntegrationEventSink by calling a consumer API port. Maps Published Language.
_Avoid_: consumer-crate infrastructure, naming it Client, treating it as Open Host, an in-memory test double, writing consumer tables

**Kernel**:
Framework-free shared types and ports.
_Avoid_: hex platform kit, a domain-level shared folder as a cell

**Money**:
A kernel value object: integer minor units and an ISO 4217 currency. Same-currency arithmetic only. Not a wire type.
_Avoid_: kernel Money on the wire, major-unit decimals, an FX type

**Instant**:
A point in time in UTC. Not a calendar date.
_Avoid_: encoding a calendar date as an instant, Date, Temporal

**Calendar date**:
A day on the calendar, written `YYYY-MM-DD`. Not an instant.
_Avoid_: midnight UTC, DateTime

**Clock**:
The kernel port that returns the current Instant. Application passes that Instant into the entity.
_Avoid_: DateProvider, domain reading the clock

**Hive-strict**:
Cell to cell is consumer SPI plus InProc only.
_Avoid_: application importing a foreign cell

**Envelope**:
A per-port tagged enum that serializes to `{ type, context }`. `type` is SCREAMING_SNAKE. `context` is primitives.
_Avoid_: command wrapper, `status` / `title` / `detail`, a stringly kernel envelope, downcasting `thiserror` across cells

**Violation**:
A kernel struct `{ path, code, message }` carried on `VALIDATION_FAILED`. Not a wire type of its own.
_Avoid_: serde on kernel, a stringly kernel envelope

**ActorId**:
A string on the API port. Presentation extracts it. The cell never sees the token. Roles and permissions do not enter the cell.
_Avoid_: token on the port, tick ports taking actorId, webhook ports taking actorId, roles on the port

**Gateway**:
The AuthN process in front of this hive. Not a cell. Not the composition root.
_Avoid_: AuthN in `app`, SSO pages in this process, treating the gateway as a cell

**Unpublished**:
An API port absent from Open Host. Public REST may still call it. Tick, webhook, and REST-only ports that other cells must not call stay unpublished.
_Avoid_: exporting every port, treating HTTP visibility as Open Host

**Domain event**:
An in-cell fact published after persist. It never leaves the cell.
_Avoid_: putting a domain event on the wire, calling an integration event a domain event

**Integration event**:
A Published Language fact a producer emits through its outbox. Not Open Host of the producer. A consumer receives it through its own API port.
_Avoid_: calling it Open Host, consumer SPI as the inbound entry, app writing consumer tables

**Tick**:
An unpublished command that lists due rows and enqueues Work. Its driving adapter lives in `presentation/ticks/`. No `actor_id`.
_Avoid_: running a tenanted command, `@Cron`, Worker, `actor_id` on the tick port

**Work**:
A once-work row in the cell’s `work` table, unique on `(work_type, work_key)`.
_Avoid_: SNS, SQS, queue crate, integration event on this table, FIFO

**WorkSink**:
The cell SPI the tick calls to enqueue Work. One method per work type.
_Avoid_: stringly enqueue, IntegrationEventSink for in-cell work

**Work drain**:
Infrastructure poller that claims Work and calls an unpublished work port. The cell exports `spawn_work_drain`.
_Avoid_: presentation worker, a second worker process, spawning at cell `new`

**Import wall**:
The compile-time forbid on illegal crate and layer imports.
_Avoid_: sheriff, dependency-cruiser

**Schema**:
The Postgres namespace one cell owns. One database for the hive. One schema per cell.
_Avoid_: one shared schema, Mongo database-per-cell as the default

**Table**:
A relation in that schema. A cell has one or more. No other cell may read or write it.
_Avoid_: collection, sharing tables across cells

**Cell role**:
The Postgres LOGIN role that can use only that cell's schema. Request pools connect as this role. DML only.
_Avoid_: user for the database principal, one role shared across cells, serving requests as the migrator role

**Migrator role**:
The Postgres LOGIN that owns one cell schema and runs DDL. The `app` migrate entry connects as this role. Serve does not.
_Avoid_: NOLOGIN owner plus SET ROLE, one role that both migrates and serves, request pools using it, migrator DSN in serve env

**Named pool**:
The sqlx `PgPool` opened with that cell role's DSN, wrapped in a cell-private newtype, held by that cell's sqlx adapter. `app` constructs it. Cell `new` never takes it.
_Avoid_: default pool, shared pool, SET ROLE, cell crate connecting itself

**Fake**:
A handwritten test double that implements an SPI. Lives next to that SPI. Not InProc.
_Avoid_: mockall, a workspace fake crate, calling it InProc

**Builder**:
A test-only fluent constructor next to the entity. `.build()` reconstructs. `.build_new()` creates.
_Avoid_: a fixtures crate, asserting on faker output

**Contract**:
A function next to the SPI that every adapter of that SPI must pass. Not a spec.
_Avoid_: a spec named contract, contract next to the sqlx adapter only

**Fatten**:
Add a use case to an existing cell that already speaks that language.
_Avoid_: a new cell for the same language

**Logger SPI**:
The kernel port for structured log lines. Bound as a generic on cell `new`. Domain never uses it.
_Avoid_: tracing in a cell crate, Pino, injecting the composition-root logger, a per-cell Logger trait

**Metrics SPI**:
The kernel port for counters and distributions. Bound as a generic on cell `new`. Domain never uses it.
_Avoid_: DogStatsD in a cell crate, hand-tagging env / service / version, a per-cell Metrics trait

**CorrelationId**:
An application UUID that groups work from one initiating action. HTTP header `X-Correlation-ID`. A tick mints one; Work copies it.
_Avoid_: using the OpenTelemetry trace id as this id, putting it on a domain event, using it as an idempotency key

**Idempotency-Key**:
The client-minted header that identifies one human POST or PATCH intent for retry.
_Avoid_: CorrelationId as this key, an `app` or kernel store, using it on PUT, DELETE, GET, webhooks, or ticks

**Wide event**:
One canonical log line per REST request that hits a cell handler, per tick invocation, and per work item.
_Avoid_: one line per InProc hop, a health wide event, one line per GraphQL operation

### Freight

**Load**:
A Shipper's posted demand to move freight from origin to destination.
_Avoid_: Order, job, tender as this noun, Shipment

**Shipment**:
The contracted move after a Load is booked with a Carrier.
_Avoid_: Load, Booking as an aggregate

**Quote**:
A Carrier's priced offer on a Load.
_Avoid_: Bid, Rate as this noun, a shipper-facing quote aggregate

**CustomerRate**:
The Money the Shipper pays, locked on the Shipment at book. Not an aggregate.
_Avoid_: Quote for this amount, Rate as a type

**CarrierRate**:
The Money the Carrier is paid, locked on the Shipment at book from the winning Quote. Not an aggregate.
_Avoid_: Quote for this amount, Payable as this amount

**Invoice**:
A request for payment to the Shipper for a Shipment, at CustomerRate.
_Avoid_: Bill, Settlement as this noun, Payable

**Payable**:
An amount owed to the Carrier for a Shipment, at CarrierRate.
_Avoid_: Payout, Invoice, Settlement as this noun

**Shipper**:
The party that posts a Load.
_Avoid_: Customer, client, account

**Carrier**:
The party that hauls a Shipment.
_Avoid_: Trucker, vendor, supplier

**Consignee**:
The receiving name and address on a destination stop. Not a party with identity.
_Avoid_: Consignee aggregate, treating Consignee as a Shipper

**Stop**:
A pickup or delivery location on a Load. First cut is two.
_Avoid_: Leg, unstructured origin and destination fields
