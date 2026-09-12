---
description: Cell application use-cases: commands, queries, event-handlers, and tick commands.
globs:
  - crates/cells/**/src/application/**
---

One use-case struct implements the API port. [CQRS](../../docs/adr/0006-cqrs.md)
`decode` is the first act. [Validation](../../docs/adr/0010-validation.md)
Commands live under `application/commands/<use-case>/`. [CQRS](../../docs/adr/0006-cqrs.md)
Queries live under `application/queries/<use-case>/` and never mutate. [CQRS](../../docs/adr/0006-cqrs.md)
Application maps `PlInput` to value objects and domain errors to the envelope. [Validation](../../docs/adr/0010-validation.md)
Application mints new aggregate ids as UUIDv7. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Application reads `Clock` and passes `Instant` into the entity. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
After `save`, publish `entity.pull_events()` through the events SPI. [CQRS](../../docs/adr/0006-cqrs.md)
Application MAY import kernel `Money`, `Instant`, `Clock`, `Logger`, and `Metrics`. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
The use-case never imports sqlx. [CQRS](../../docs/adr/0006-cqrs.md)
Application stays serde-free and garde-free. [Validation](../../docs/adr/0010-validation.md)
No Command type. No Handler trait. No unit-of-work SPI. [CQRS](../../docs/adr/0006-cqrs.md)
Two `save` calls are two transactions. [CQRS](../../docs/adr/0006-cqrs.md)
A tick command lists due rows and enqueues WorkSink; it does not `save` an aggregate. [Workers](../../docs/adr/0014-workers.md)
Resource `FORBIDDEN` comes from the use-case. Roles stay out. [Authentication](../../docs/adr/0013-authentication.md)

## Tests

Integration tests live in `integration` modules inside the cell crate. [Testing](../../docs/adr/0011-testing.md)
Integration constructs the use-case with this cell's real sqlx adapter. [Testing](../../docs/adr/0011-testing.md)
Integration has no HTTP. [Testing](../../docs/adr/0011-testing.md)
Every command has ≥1 happy path and ≥1 error. [Testing](../../docs/adr/0011-testing.md)
Every SPI method is on that port's contract. [Testing](../../docs/adr/0011-testing.md)
