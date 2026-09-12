---
condition: 'use\s+(sqlx|sea_orm)\b|(sqlx|sea_orm)::'
globs:
  - crates/cells/**/src/domain/**
  - crates/cells/**/src/application/**
scope:
  - tool:edit(crates/cells/**/src/domain/**)
  - tool:write(crates/cells/**/src/domain/**)
  - tool:edit(crates/cells/**/src/application/**)
  - tool:write(crates/cells/**/src/application/**)
interruptMode: tool-only
---

Keep persistence in the sea-orm adapter. [Persistence](../../docs/adr/0007-persistence.md)
