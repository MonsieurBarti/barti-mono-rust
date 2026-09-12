---
description: Cell domain folder: entities, SPI, events, errors, and domain/api codecs.
globs:
  - crates/cells/**/src/domain/**
---

Entity behavior records domain events. [CQRS](../../docs/adr/0006-cqrs.md)
Write SPI is `get_by_id` / `save` for one aggregate. [CQRS](../../docs/adr/0006-cqrs.md)
Leaving SPI arguments are this cell's primitives. [Communication](../../docs/adr/0005-communication.md)
Fake and contract sit next to the SPI. [Testing](../../docs/adr/0011-testing.md)
Domain MAY import kernel `Money` and `Instant`. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Domain never logs, never counts, never reads the clock. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
Domain events stay in the cell. [Communication](../../docs/adr/0005-communication.md)
Domain does not call `Uuid::now_v7`. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Ports are AFIT: no `async_trait`, no `Arc<dyn`. [Communication](../../docs/adr/0005-communication.md)
Domain SPI stays pool-free. [Persistence](../../docs/adr/0007-persistence.md)

## domain/api

`domain/api` is the only place serde and garde are legal inside the cell. [Validation](../../docs/adr/0010-validation.md)
`decode` is the only function that names garde. [Validation](../../docs/adr/0010-validation.md)
`Valid<T>` never leaves that module. [Validation](../../docs/adr/0010-validation.md)
Envelope is a per-port tagged enum `{ type, context }`. [Communication](../../docs/adr/0005-communication.md)
`Violation` lives in `domain/api` only. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
PL Money is `{ amount, currency }` integer minor units. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Ids on the wire are opaque non-empty strings. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Instant PL is ISO-8601 datetime; calendar date is `YYYY-MM-DD`. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
`domain/api` MUST NOT import kernel `Money` or `Instant`. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
`VALIDATION_FAILED` names shape and field constraints on PL, not entity invariants. [Validation](../../docs/adr/0010-validation.md)

## Tests

Unit tests sit `#[cfg(test)]` next to the unit. [Testing](../../docs/adr/0011-testing.md)
Unit has no I/O and no doubles. [Testing](../../docs/adr/0011-testing.md)
Fluent builder sits next to the entity: `.build()` reconstructs, `.build_new()` creates. [Testing](../../docs/adr/0011-testing.md)
Unit uses kernel `FakeClock`. [Testing](../../docs/adr/0011-testing.md)
Every domain invariant gets a rejection test. [Testing](../../docs/adr/0011-testing.md)
Every persistence mapper gets a round-trip. [Testing](../../docs/adr/0011-testing.md)
