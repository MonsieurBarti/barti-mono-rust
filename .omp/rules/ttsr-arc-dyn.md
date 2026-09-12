---
condition: 'Arc\s*<\s*dyn\b'
globs:
  - crates/cells/**
scope:
  - tool:edit(crates/cells/**)
  - tool:write(crates/cells/**)
interruptMode: tool-only
---

Bind the port as a generic type parameter. [Communication](../../docs/adr/0005-communication.md)
