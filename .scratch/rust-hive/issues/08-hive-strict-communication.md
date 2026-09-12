# How do cells communicate in Rust hive-strict?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 03, 04, 07

## Question

Port naboo chapter 4.

Decide: consumer-owned SPI trait shape; Published Language codecs on both sides of the hop; envelope `{ type, context }`; request/response default vs integration events; how a domain event crosses to another cell.

InProc placement is already decided: adapters live in `app`, and no cell crate depends on another cell. Do not re-open that. "No application import of a foreign cell" is now a Cargo fact, not a rule this ticket writes.

Research tickets [What is the current SOTA Rust approach to in-process CQRS, domain events, and a transactional outbox on Postgres?](../issues/03-sota-cqrs-events-outbox.md) and [What is the current SOTA Rust approach to Published Language validation at a cell edge?](../issues/04-sota-cell-edge-validation.md) feed this grill. Packaging must already be decided.

## Answer

Hive-strict in Rust: the consumer owns the SPI in consumer primitive types. `app` InProc implements it by calling the provider API port and mapping Published Language both ways. No cell crate depends on another cell.

API ports and SPIs declare `fn m(&self, …) -> impl Future<Output = Result<…, Envelope>> + Send`. No `dyn`, no `async_trait`, no `trait_variant`. Presentation is generic over the cell's SPI type parameters. Research ticket 03's `Arc<dyn ApiPort>` is corrected.

`domain/api/<use-case>/` owns the PL structs, the garde derives, and `decode`/`encode`. The application use-case implements the API port. Its first act is `decode(input)`. Inner domain and application never `use serde` or `use garde`. The cell crate may depend on both. Presentation may deserialize HTTP JSON into the same PL struct, then call the port; the port still decodes, so no caller skips the gate. Consumer InProc does not revalidate a typed reply. Both-sides validation returns when the hop becomes HTTP.

The cell-edge envelope is a per-port error enum: `#[serde(tag = "type", content = "context", rename_all = "SCREAMING_SNAKE_CASE")]`. Variants carry primitive-only structs. Kernel owns only the `VALIDATION_FAILED` violations payload. `app` matching a provider error enum is exhaustive and legal: that enum is the published contract. Cells never downcast `thiserror` domain errors.

Leaving SPI signatures name consumer-owned primitive structs, not value objects and not provider PL. InProc maps provider PL to those structs. The consumer application maps them to value objects. The dual: a producer's `IntegrationEventSink` names producer PL event structs.

Default edge is request/response. Domain events stay in `domain/events/` and never leave the cell. Integration events are PL under `domain/api/<event>/`. They are not Open Host of the producer. The consumer's inbound use-case port is ordinary Open Host of the consumer.

An integration event enters a consumer through that API port, implemented by an idempotent use case and exported from `lib.rs`. The subscriber in `app/inproc/<consumer>/<provider>.rs` maps provider PL to consumer PL and calls that port. `app` never writes consumer tables. A local read model is written by the consumer's own use case through its own write SPI.

The producer cell owns the outbox table, the claim SQL, the loop body, and a leaving `IntegrationEventSink` with one method per integration event. It exports `spawn_outbox_drain`. `app` binds the sink, spawns the task, owns poll interval and shutdown. The cell does not spawn at construction.

Every event carries `event_id` (uuid v7) and `event_type`. Each consumer keeps `processed_event` in its own schema, written in the same transaction as the effect. Duplicate delivery is a unique-index conflict and returns ok.

One drain task per producer cell per process. Sequential await over bound subscribers. Mark published when all return ok. Redeliver the whole row on any failure. `attempts` and `next_attempt_at` with backoff in the claim predicate, so a poison row does not block the head of the queue. No ordering guarantee across rows.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **SPI**, **Codec**, **Envelope**, **InProc adapter**, **Domain event**, **Integration event**.


## Comments

### Round 1

Seven arrows accepted:

- Q1 A: RPITIT `-> impl Future<Output = Result<…>> + Send` on API ports and SPIs. No `dyn`, no `async_trait`, no `trait_variant`. Presentation is generic over the cell's SPI type parameters. Research ticket 03's `Arc<dyn ApiPort>` is corrected.
- Q2 A: the provider validates every inbound call. The consumer InProc adapter does not revalidate the reply. Both-sides validation returns when the hop becomes HTTP. Where the garde call lives is round 2.
- Q3 A: one PL error enum per API port, `#[serde(tag = "type", content = "context", rename_all = "SCREAMING_SNAKE_CASE")]`. Kernel owns only the `VALIDATION_FAILED` violations payload.
- Q4 A: inbound integration events enter through a consumer API port. The subscriber in `app/inproc/<consumer>/<provider>.rs` maps provider PL to consumer PL and calls that port. `app` never writes consumer tables.
- Q5 A: producer cell owns the outbox table, claim SQL, loop body, and a leaving `IntegrationEventSink` SPI, and exports `spawn_outbox_drain`. `app` binds the sink, spawns the task, owns poll interval and shutdown.
- Q6 A: `event_id` (uuid v7) and `event_type` on every event. Each consumer keeps `processed_event` in its own schema, written in the same transaction as the effect. Law, not advice.
- Q7 A: sequential await over bound subscribers, mark published when all return ok, redeliver the whole row on failure. `attempts` and `next_attempt_at` with backoff in the claim predicate. No ordering guarantee across rows. One drain task per producer cell per process.

### Round 2

Two arrows accepted:

- Q8 A: `domain/api/<use-case>/` owns PL structs and `decode`/`encode`. The application impl's first act is `decode(input)`. Inner domain and application never `use serde` or `use garde`. "Domain imports neither" means inner domain, not `domain/api/`.
- Q9 B: leaving SPI signatures name consumer-owned primitive structs, not value objects and not provider PL. `IntegrationEventSink` names producer PL event structs.
