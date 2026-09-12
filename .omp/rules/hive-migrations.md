---
description: Cell SQL migrations in that cell's schema.
globs:
  - crates/cells/**/migrations/**
---

Migrations only change tables and indexes in that cell's schema. [Persistence](../../docs/adr/0007-persistence.md)
A new table is a cell migration. [Persistence](../../docs/adr/0007-persistence.md)
Migrations never `CREATE ROLE` or `GRANT`. [Persistence](../../docs/adr/0007-persistence.md)
`amount BIGINT`, `currency TEXT`, id `UUID`, instant `TIMESTAMPTZ`, calendar date `DATE`. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Never `NUMERIC` for amount. Never `TIMESTAMPTZ` for a calendar date. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Event table unique `(stream_id, version)`. [Event sourcing](../../docs/adr/0008-event-sourcing.md)
Work table unique `(work_type, work_key)`. [Workers](../../docs/adr/0014-workers.md)
