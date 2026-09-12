---
condition: 'use\s+(serde|garde)(\b|::)'
globs:
  - crates/cells/**/src/domain/{entities,events,errors,spi}/**
  - crates/cells/**/src/domain/*.rs
scope:
  - tool:edit(crates/cells/**/src/domain/{entities,events,errors,spi}/**)
  - tool:write(crates/cells/**/src/domain/{entities,events,errors,spi}/**)
  - tool:edit(crates/cells/**/src/domain/*.rs)
  - tool:write(crates/cells/**/src/domain/*.rs)
interruptMode: tool-only
---

Put serde and garde in `domain/api`. [Validation](../../docs/adr/0010-validation.md)
