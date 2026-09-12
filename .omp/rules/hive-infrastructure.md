---
description: Cell driven adapters: sea-orm, vendors, outbox drain, and work drain.
globs:
  - crates/cells/**/src/infrastructure/**
---

sea-orm adapter holds the named pool newtype. [Persistence](../../docs/adr/0007-persistence.md)
Replay and append live in the adapter. [Event sourcing](../../docs/adr/0008-event-sourcing.md)
Every entity sets `schema_name`. `search_path` is a belt on the role. [Persistence](../../docs/adr/0007-persistence.md)
No `query!`. Compile-time wall is crate privacy only. [Persistence](../../docs/adr/0007-persistence.md)
Handwritten `Model` / `ActiveModel` stay in `infrastructure/`. Domain never imports `sea_orm`. [Persistence](../../docs/adr/0007-persistence.md)
Infrastructure MAY serde+garde a foreign wire into SPI types; that failure is not `VALIDATION_FAILED`. [Validation](../../docs/adr/0010-validation.md)
This cell's sea-orm adapter MAY import kernel `Money` and `Instant`. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Adapters MAY log and count through kernel ports. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
Work drain and outbox drain live in infrastructure; the cell exports `spawn_work_drain` and `spawn_outbox_drain`. [Workers](../../docs/adr/0014-workers.md)
The cell does not spawn at `new`. [Workers](../../docs/adr/0014-workers.md)
Event rows are unique on `(stream_id, version)`; unique violation is the concurrency conflict. [Event sourcing](../../docs/adr/0008-event-sourcing.md)
Adapter impls stay AFIT. [Communication](../../docs/adr/0005-communication.md)
