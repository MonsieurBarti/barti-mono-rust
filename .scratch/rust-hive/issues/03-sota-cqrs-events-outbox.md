# What is the current SOTA Rust approach to in-process CQRS, domain events, and a transactional outbox on Postgres?

Type: research
Status: resolved
Label: wayfinder:research
Blocked by:

## Question

How should a Rust cell implement CQRS (command/query handlers), in-cell domain events, and fire-and-forget integration events via a transactional outbox on Postgres, without a Nest CQRS bus?

Constraints:

- Latest stable versions only.
- Primary sources: crate docs and Postgres docs. Do not treat blog summaries as law.
- Cover: whether a bus crate is needed, or traits + the composition root suffice; EventEmitter-equivalents; outbox in the same transaction as `save`; at-least-once drain.
- Naboo law to port: presentation never sees the bus; application use-case implements the API port; domain events never leave the cell; integration events are Published Language, not Open Host.
- Recommend a concrete crate set or a no-crate pattern. Pin majors.

Asset: `.scratch/rust-hive/research/03-sota-cqrs-events-outbox.md`

## Answer

No CQRS bus crate. API-port traits plus composition root. Domain events stay in-cell. Integration events: producer outbox on the same sqlx 0.9.0 transaction as save; drain with FOR UPDATE SKIP LOCKED. Findings: [03-sota-cqrs-events-outbox.md](../research/03-sota-cqrs-events-outbox.md).
