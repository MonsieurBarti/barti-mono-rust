# Should the persistence adapter be an ORM?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 09, 02

## Question

Ticket 09 and [ADR 0001](../../../docs/adr/0001-stack-pins.md) pin sqlx 0.9.0. The adapter writes schema-qualified `query!` SQL. Diesel `table!` is rejected. SeaORM was compared and lost: it sits on sqlx, is not compile-time SQL, and ActiveModel pulls persistence toward the hexagon.

Reopen: replace that SQL surface with an ORM that still does cell isolation.

Isolation is a Postgres GRANT wall, not an ORM feature. Keep or drop each of: schema per cell, two LOGIN roles, named pool, SPI pool-free, schema-qualified tables, `query!` against the cell-role DSN, `23505` in the adapter.

## Answer

The persistence adapter is a **sea-orm adapter**. Not sqlx `query!`. Not Diesel.

SeaORM 2. Isolation stays a Postgres GRANT wall.

One database. Schema per cell. Two LOGIN roles per cell: the migrator owns the schema and runs DDL; the cell role is DML only.

Ops provisions the database, both LOGINs, schemas, GRANTs, and default privileges. Cell migrations live in the cell crate and only change tables and indexes in that schema. No extra `migration` crate. Cell migrations never `CREATE ROLE` or `GRANT`. Default `public` is a review reject.

`app` has a migrate entry and a serve entry. Migrate env holds migrator DSNs and runs each cell’s `sea-orm-migration` `Migrator` with the schema passed explicitly. Serve env holds only cell-role DSNs and never migrates.

Serve: `app` opens a named `DatabaseConnection` from the cell-role DSN, wraps it in a cell-private newtype, and passes it only into that cell’s sea-orm adapter. Cell `new` never takes `DatabaseConnection`. Unnamed, shared, or default connection is a review reject. Pool size is env. Domain SPI stays pool-free. `sea_orm` types stay in `infrastructure/`.

A transaction is `begin()` on that cell’s connection. Two connections cannot share a `BEGIN`. 2PC stays out. Same-request write atomicity across cells is a merge signal. InProc is a second transaction on the other cell’s connection.

Compile-time wall: crate privacy only. No `query!`. Every entity sets `schema_name`. `search_path` is a belt on the role. `set_schema_search_path` is not the wall. A planted foreign schema is a review reject. Runtime leftover is SQLSTATE `42501`, an operational bug, not a cell-edge envelope. GRANT proof stays `crates/app/tests`.

Handwritten `Model` / `ActiveModel` in `infrastructure/`, `pub(crate)`. Adapter maps to domain types. Empty `ActiveModelBehavior`. Domain never imports `sea_orm`. No `sea-orm-cli generate-entity`.

Adapter maps `23505` through `DbErr` to conflict. Use-case never sees the driver error.

Diesel `table!` stays rejected.

ADR pins `sea-orm` 2.0.2, `sea-orm-migration` 2.0.2, `sea-orm-cli` 2.0.2. sqlx 0.9.0 stays as the driver pin. Cells depend on `sea-orm`, not sqlx. `sqlx-cli` goes.

Glossary: [CONTEXT.md](../../../CONTEXT.md) **sea-orm adapter**, **Named pool**.

Rewrote `CONTEXT.md`, [docs/architecture.md](../../../docs/architecture.md), and [docs/adr/0001-stack-pins.md](../../../docs/adr/0001-stack-pins.md). This ticket does not implement product code.

## Comments

### Round 1

Two arrows accepted:

- Q1 C: SeaORM 2. Adapter SQL surface is SeaORM, not sqlx `query!`.
- Q2 A: GRANT wall stays. Schema per cell, two LOGINs, named connection from cell-role DSN, SPI pool-free, `42501` in the adapter.

Q3 moved to round 2.

Settled consequence, not a question: named pool is a cell-private newtype around SeaORM `DatabaseConnection` opened from the cell-role DSN. `app` constructs it. Cell `new` never takes it. `sea_orm` types stay in `infrastructure/`. Two connections cannot share a `BEGIN`. Diesel `table!` stays rejected.

### Round 2

Five arrows accepted (user: lgtm):

- Q3 A: adapter maps `23505` through `DbErr` to conflict. Use-case never sees the driver error.
- Q4 A: drop `query!` / `query_as!`. Compile-time wall is crate privacy only. Runtime leftover is `42501` in `app` tests.
- Q5 A: SeaORM `Model` / `ActiveModel` live in `infrastructure/` only, `pub(crate)`. Adapter maps to domain types. Empty `ActiveModelBehavior`. Domain never imports `sea_orm`.
- Q6 A: `schema_name` on every entity. `search_path` is a belt on the role only. `set_schema_search_path` is not the wall. Default `public` is a review reject.
- Q7 A: `sea-orm-migration` per cell. Schema passed explicitly. Serve never migrates. `sqlx::migrate!` and `sqlx-cli` go.

### Round 3

Three arrows accepted (user: lgtm):

- Q8 A: glossary term is **sea-orm adapter**. Replaces **sqlx adapter**.
- Q9 A: handwritten entities in `infrastructure/`. No `generate-entity`.
- Q10 A: ADR pins `sea-orm` 2, `sea-orm-migration` 2, `sea-orm-cli` 2. sqlx 0.9 stays as the driver pin. Cells depend on `sea-orm`, not sqlx. `sqlx-cli` goes.

Settled consequence of [How is a cell packaged, and what is the composition root without Nest?](07-cell-packaging-and-composition-root.md): migrations are modules in the cell crate. No extra `migration` crate.

### Round 4

Q11 A accepted (user: lgtm). Draft recorded. Glossary, architecture, and ADR rewritten. Ticket closed.
