---
description: Composition root: wiring, InProc, telemetry, migrate, and serve.
globs:
  - crates/app/**
---

`app` imports every cell and binds leaving SPIs. [Composition root](../../docs/adr/0004-composition-root.md)
InProc lives in `app/inproc/<consumer>/<provider>.rs`. [Language](../../docs/adr/0002-language.md)
`app` constructs named pools, clocks, Logger, Metrics, and InProc, then calls `new`. [Composition root](../../docs/adr/0004-composition-root.md)
Cell `new` never takes `PgPool`. [Persistence](../../docs/adr/0007-persistence.md)
`telemetry.rs` owns `tracing` and OpenTelemetry. [Observability](../../docs/adr/0012-observability.md)
Migrate env holds migrator DSNs; serve env holds cell-role DSNs only. [Persistence](../../docs/adr/0007-persistence.md)
Middleware copies gateway identity headers; the bind is not public. [Authentication](../../docs/adr/0013-authentication.md)
Missing identity is HTTP 401 `UNAUTHENTICATED` and never enters a cell. [Authentication](../../docs/adr/0013-authentication.md)
`app` spawns ticks, outbox drains, and work drains; it owns poll interval and shutdown. [Workers](../../docs/adr/0014-workers.md)
`app` never writes consumer tables. [Communication](../../docs/adr/0005-communication.md)
InProc maps Published Language both ways and calls `decode`. [Communication](../../docs/adr/0005-communication.md)
InProc MUST NOT import kernel `Money` or `Instant`. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Health lives on `app`, not a cell. [REST](../../docs/adr/0009-rest.md)
Header `X-Correlation-ID` is accepted or minted as UUIDv7; never 400. [Observability](../../docs/adr/0012-observability.md)
