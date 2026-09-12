# Persistence

A schema and two LOGIN roles per cell make exclusive tables a Postgres GRANT, not a review rule. `app` passes a named `DatabaseConnection` only into the sea-orm adapter, because cell `new` taking that type would leak SeaORM into the hexagon. Compile-time wall is crate privacy only. No `query!`. Prepared transactions and 2PC stay out: same-request write atomicity across cells is a merge signal.

Chapter: [6. Persistence](../architecture.md#6-persistence)

Grill: [How is Postgres isolation enforced per cell?](../../.scratch/rust-hive/issues/09-postgres-cell-isolation.md), [Should the persistence adapter be an ORM?](../../.scratch/rust-hive/issues/25-orm-adapter.md)
