# Persistence

A schema and two LOGIN roles per cell make exclusive tables a Postgres GRANT, not a review rule. `app` passes a named pool only into the sqlx adapter, because cell `new` taking `PgPool` would leak sqlx into the hexagon. Prepared transactions and 2PC stay out: same-request write atomicity across cells is a merge signal.

Chapter: [6. Persistence](../architecture.md#6-persistence)

Grill: [How is Postgres isolation enforced per cell?](../../.scratch/rust-hive/issues/09-postgres-cell-isolation.md)
