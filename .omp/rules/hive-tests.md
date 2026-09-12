---
description: Cargo tests/ e2e lane and app GRANT proof.
globs:
  - crates/**/tests/**
---

Cargo `tests/` is e2e. [Testing](../../docs/adr/0011-testing.md)
Cell `tests/` sees only the re-export set. [Testing](../../docs/adr/0011-testing.md)
E2E hits `new` + `router` with tower. [Testing](../../docs/adr/0011-testing.md)
E2E injects fake leaving SPIs into `new`. [Testing](../../docs/adr/0011-testing.md)
Cell e2e never boots `app`. [Testing](../../docs/adr/0011-testing.md)
One command and one query per public use-case that has an HTTP handler. [Testing](../../docs/adr/0011-testing.md)
No handler unit tests. [Testing](../../docs/adr/0011-testing.md)
GRANT proof lives in `crates/app/tests`: a planted cross-schema `query` expects SQLSTATE `42501`. [Testing](../../docs/adr/0011-testing.md)
Tests never start Postgres. [Testing](../../docs/adr/0011-testing.md)
Cell tests spawn neither tick loop nor drain. [Workers](../../docs/adr/0014-workers.md)
