---
condition: '\basync_trait\b'
globs:
  - crates/cells/**
scope:
  - tool:edit(crates/cells/**)
  - tool:write(crates/cells/**)
interruptMode: tool-only
---

Return `impl Future` from the trait. [Communication](../../docs/adr/0005-communication.md)
