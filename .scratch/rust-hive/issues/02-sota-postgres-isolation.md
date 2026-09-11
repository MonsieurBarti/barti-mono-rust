# What is the current SOTA Rust Postgres stack for exclusive per-cell schemas and roles?

Type: research
Status: claimed
Label: wayfinder:research
Blocked by:

## Question

Which Postgres access crate is current state of the art for a hive where each cell owns exclusive tables, and the runtime wall is a Postgres role that cannot read another cell's schema?

Constraints:

- Latest stable versions only.
- Compare sqlx, diesel, sea-orm, tokio-postgres, and any 2026 successor against official docs.
- Cover: compile-time vs runtime checked SQL, migrations, connection-per-cell, transactions that cannot span cells, schema-per-cell vs database-per-cell, `GRANT` on schemas vs tables.
- Naboo law to port: exclusive collections, one credential carrier per cell, no cross-cell transaction. Mongo users become Postgres roles.
- Recommend one stack and the isolation mechanism (schema + role, separate database, or other). Pin majors.

Asset: `.scratch/rust-hive/research/02-sota-postgres-isolation.md`
