---
description: Cell driving adapters: HTTP handlers and tick adapters.
globs:
  - crates/cells/**/src/presentation/**
---

Handlers live in `presentation/http/<use-case>/`. [REST](../../docs/adr/0009-rest.md)
Ticks live in `presentation/ticks/<use-case>/`. [Workers](../../docs/adr/0014-workers.md)
Handler extracts `actor_id` as a string; the body omits it. [Authentication](../../docs/adr/0013-authentication.md)
Success is Published Language as `application/json`. [REST](../../docs/adr/0009-rest.md)
Errors are RFC 9457 `application/problem+json` with no `context`. [REST](../../docs/adr/0009-rest.md)
No business logic in the handler. [REST](../../docs/adr/0009-rest.md)
Cell `router` has no AuthN layer. [Authentication](../../docs/adr/0013-authentication.md)
Tick and webhook ports take no `actor_id`. [Authentication](../../docs/adr/0013-authentication.md)
Presentation MUST NOT import kernel `Money` or `Instant`. [Published Language scalars](../../docs/adr/0015-published-language-scalars.md)
Handlers MAY log and count through kernel ports. [Shared kernel](../../docs/adr/0016-shared-kernel.md)
Presentation is generic over the cell's SPI type parameters. [Communication](../../docs/adr/0005-communication.md)
Human POST and PATCH require header `Idempotency-Key`. [REST](../../docs/adr/0009-rest.md)
One wide event per REST request that hits a cell handler. [Observability](../../docs/adr/0012-observability.md)
Webhook verifies the signature on the raw body, then this cell's input PL, then `decode`. [Validation](../../docs/adr/0010-validation.md)
