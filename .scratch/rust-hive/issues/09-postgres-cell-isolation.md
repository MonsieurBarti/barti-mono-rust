# How is Postgres isolation enforced per cell?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 02, 07

## Question

Port naboo chapter 6 from Mongo users/collections to Postgres. Shape is already locked: one database, schema per cell, one or more exclusive tables, one cell role per schema.

Decide: GRANT shape, named pool per cell, no cross-cell transaction, how a planted import of another cell's tables fails at compile time and at runtime.

Research ticket [What is the current SOTA Rust Postgres stack for exclusive per-cell schemas and roles?](../issues/02-sota-postgres-isolation.md) feeds this grill. Packaging must already be decided.

## Answer

One database. Schema per cell. Two LOGIN roles per cell: the migrator role owns the schema and runs DDL; the cell role is DML only on that schema.

Ops provisions the database, both LOGINs, `CREATE SCHEMA … AUTHORIZATION` migrator, GRANTs, and default privileges. `GRANT … ON ALL TABLES IN SCHEMA` is legal because the schema is the exclusive unit. `REVOKE ALL ON SCHEMA public FROM PUBLIC`. No `CREATE` on the schema for the cell role. No `SET ROLE`. Cell `migrations/` only change tables and indexes in that schema. A new table is a cell migration. A new schema is a role change in the same change. Cell migrations never `CREATE ROLE` or `GRANT`.

`app` has a migrate entry and a serve entry. Migrate env holds migrator DSNs and runs each cell's `sqlx::migrate!`. Serve env holds only cell-role DSNs and never migrates.

Serve: `app` opens a named pool from the cell-role DSN, wraps it in a cell-private newtype, and passes it only into that cell's sqlx adapter. Cell `new` never takes `PgPool`. Unnamed, shared, or default pool is a review reject. Pool size is env. Domain SPI stays pool-free. sqlx types stay in `infrastructure/`.

A transaction is `pool.begin()` on that cell's pool. Two pools cannot share a `BEGIN`. Prepared transactions / 2PC stay out. Same-request write atomicity across cells is a merge signal. InProc is a second transaction on the other cell's pool.

Compile-time wall: crate privacy plus `query!` / `query_as!` compiled against the cell-role DSN, never a superuser URL. Schema-qualify every table. `search_path` is a belt. A planted foreign schema fails compile without `USAGE`. Runtime `query()` is a review reject except where the macro cannot express the SQL. Runtime leftover is SQLSTATE `42501`, an operational bug, not a cell-edge envelope. Proof that GRANT bites is [What are the test lanes, and what does each one boot?](12-test-lanes.md).

The domain folder is not a privilege boundary. Kernel has no exclusive datastore. Diesel `table!` stays rejected.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Cell role**, **Migrator role**, **Named pool**, **sqlx adapter**, **Schema**, **Table**.

## Comments

### Round 1

Four arrows accepted:

- Q1 A: two LOGIN roles per cell. Migrator owns the schema and runs DDL. Cell role is DML only. `GRANT … ON ALL TABLES IN SCHEMA` is legal because the schema is the exclusive unit. `REVOKE ALL ON SCHEMA public FROM PUBLIC`. No `SET ROLE`.
- Q2 A: `app` opens the named pool from the cell-role DSN, wraps it in a cell-private newtype, and passes it only into that cell's sqlx adapter. Cell `new` never takes `PgPool`. Unnamed, shared, or default pool is a review reject. Pool size stays env.
- Q3 A: a transaction is `pool.begin()` on that cell's pool. Two pools cannot share a `BEGIN`. 2PC stays out. Same-request write atomicity across cells is a merge signal. InProc is a second transaction on the other pool.
- Q4 A: `query!` / `query_as!` compile against the cell-role DSN, never a superuser URL. Schema-qualify every table. `search_path` is a belt. Planted cross-schema SQL fails compile without `USAGE`. Runtime leftover is `42501`, an operational bug, not an envelope. Proof lane is [What are the test lanes, and what does each one boot?](12-test-lanes.md).

Diesel is the Drizzle-shaped crate. Stack stays sqlx. Not reopened.

### Round 2

Two arrows accepted:

- Q5 A: ops provisions roles, schemas, and GRANTs. Cell migrations only change tables and indexes. No `CREATE ROLE` or `GRANT` in cell SQL.
- Q6 C: same `app` binary, distinct migrate entry. Migrate env has migrator DSNs. Serve env has only cell-role DSNs and never calls `migrate!`.
