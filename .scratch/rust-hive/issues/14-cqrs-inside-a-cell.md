# How does CQRS work inside a Rust cell?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 03, 07, 08

## Question

Port naboo chapter 5 to Rust.

Locked: no CQRS bus crate; API-port traits plus composition root; presentation never sees a bus; the application use-case implements the API port; domain events never leave the cell. Research: [What is the current SOTA Rust approach to in-process CQRS, domain events, and a transactional outbox on Postgres?](03-sota-cqrs-events-outbox.md). Packaging: [How is a cell packaged, and what is the composition root without Nest?](07-cell-packaging-and-composition-root.md).

Decide: one application file per use-case versus split files; whether a command may return a result; that a query never mutates; the write SPI as `get_by_id` / `save`; where in-cell event handlers live; command pipeline as a per-cell choice, not architecture law.

Do not pick event sourcing here. Do not write the chapter here.

## Answer

CQRS stays. No bus crate. No Command type. No Handler trait.

Each use-case is one application struct that implements that API-port trait. Files live under `application/commands/<use-case>/` or `application/queries/<use-case>/`. Presentation and InProc call the port. Integration constructs the struct. `decode` is the first act.

A command may return an id or outcome as Published Language. Errors stay the per-port envelope. A query never mutates. It injects read SPIs only. No `save`. No outbox insert. No domain-event publish.

Write SPI is `get_by_id` / `save` per aggregate. Replay lives in the adapter. The use-case never imports sqlx. Multi-entity work lives in the use-case, not `domain/services/`. Two `save` calls are two transactions. Same-request atomicity means one aggregate. No unit-of-work SPI.

A query injects one read SPI per need and returns a plain read model. It never loads the write-side entity. Default reads may hit the write table through that read SPI.

The entity records domain events during behavior. After `save` returns, the use-case publishes `entity.pull_events()` through a cell write-side events SPI. Infrastructure awaits matching handlers by type, after commit. Handlers live in `application/event-handlers/<need>/` and stay in-cell. They are after-commit reactions that may lag, including fan-out to another use-case in this cell. They are not how a query's required read model is written. Extra in-cell read collections stay on [Do we event-source, where, and what is the contract?](15-event-sourcing.md). Not `tokio::sync::broadcast`. Not a bus crate. Kernel-versus-cell for that SPI waits on [What lives in the kernel crate?](20-kernel-crate.md).

Command integration fakes the events SPI and asserts what was published. A handler is its own application integration test: real sqlx, fake leaving SPIs.

Command pipeline is a per-cell choice, not architecture law.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Command**, **Query**, **Write SPI**, **Read SPI**.


## Comments

### Round 1

Seven arrows accepted:

- Q1 A: one struct per use-case implements the API-port trait. No Command type. No Handler trait. File at `application/commands/<use-case>/` or `application/queries/<use-case>/`. Integration constructs that struct.
- Q2 A: a command may return an id or outcome as Published Language. Errors stay the per-port envelope.
- Q3 A: a query never mutates. Law. Read SPIs only. No `save`. No outbox insert. No domain-event publish.
- Q4 A: write SPI is `get_by_id` / `save` per aggregate. Replay lives in the adapter. The use-case never imports sqlx. Multi-entity work lives in the use-case, not `domain/services/`.
- Q5 A: use-case calls a cell write-side events SPI. Infrastructure awaits matching handlers by type, after commit. Handlers live in `application/event-handlers/<need>/` and stay in-cell. Not `tokio::sync::broadcast`. Not a bus crate. Kernel-versus-cell for that SPI waits on [What lives in the kernel crate?](20-kernel-crate.md).
- Q6 A: command pipeline is a per-cell choice, not architecture law.
- Q7 A: one read SPI per need. Plain read model. Never loads the write-side entity. Default reads may hit the write table through that read SPI.

### Round 2

Four arrows accepted:

- Q8 A: event handlers are after-commit in-cell reactions that may lag, including fan-out to another use-case in this cell. They are not how a query's required read model is written. Extra in-cell read collections stay on [Do we event-source, where, and what is the contract?](15-event-sourcing.md).
- Q9 A: the entity records domain events during behavior. The use-case `save`s, then `events.publish(entity.pull_events())`.
- Q10 A: command integration fakes the events SPI and asserts what was published. A handler is its own application integration test: real sqlx, fake leaving SPIs.
- Q11 A: two `save` calls are two transactions. If they must be atomic, they are one aggregate. No unit-of-work SPI.

### Round 3

Q12 A: record that draft, close the ticket, add those glossary terms.



