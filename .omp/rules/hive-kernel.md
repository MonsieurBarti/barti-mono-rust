---
description: Shared kernel library: Money, Instant, Clock, Logger, Metrics, Violation.
globs:
  - crates/kernel/**
---

Kernel is a library, not a cell. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
The only crate dep is `time`. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
Exports exactly: `Money`, `Instant`, `Clock`, `SystemClock`, `FakeClock`, ISO helpers, `Violation`, `Logger`, `Metrics`. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
Kernel stays serde-free, axum-free, sqlx-free, tracing-free, uuid-free. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
Forbidden: Envelope, Id, CorrelationId, ActorId, a Result alias, outbox helpers, pagination, problem+json, a Level enum, a Currency enum, macros, gauge. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
`Clock`, `Logger`, and `Metrics` are `Send + Sync`, synchronous, non-throwing, not `dyn`. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
Kernel `Money` is i64 minor units plus ISO currency; same-currency arithmetic only. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
