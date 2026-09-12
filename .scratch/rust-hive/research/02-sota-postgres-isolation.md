# SOTA Rust Postgres stack for exclusive per-cell schemas and roles

As of 2026-09-11. Crate versions are `max_stable_version` on crates.io. Postgres docs are current 18.

## Verdict

**sqlx 0.9.0** (`sqlx-cli` 0.9.0) with **schema-per-cell**, **LOGIN role per cell**, and **`PgPool` per cell**.

Isolation is a Postgres privilege wall, not an ORM feature.

- One database. One schema per cell. One LOGIN role per cell. One `sqlx::PgPool` per cell.
- The composition root opens each pool with that cell's credentials. The pool is the credential carrier.
- Cell SQL is schema-qualified. `search_path` is not the wall.
- Migrations live in the cell crate. `sqlx.toml` names the schema and the migrations table (`<cell>._sqlx_migrations`).
- A planted read of another cell's table fails at runtime with SQLSTATE `42501` (`insufficient_privilege`).
- A cell transaction is `pool.begin()` on that cell's pool. Cross-cell transactions are illegal. Same-request write atomicity across cells is a merge signal.

Do not use database-per-cell. Do not use `SET ROLE` on a shared pool. Do not grant `CREATE` on the cell schema to the runtime role.

Pins:

| Piece    | Version                |
| -------- | ---------------------- |
| sqlx     | 0.9.0                  |
| sqlx-cli | 0.9.0                  |
| Postgres | 18 (docs current 18.6) |

sqlx 0.9 is the 2026 successor of 0.8. It adds per-crate `sqlx.toml`, renameable `DATABASE_URL`, renameable `_sqlx_migrations`, and a first-party multi-tenant / multi-schema example. No other crate replaced this stack.

## Compared

| Crate                 | Latest stable                                                                 | SQL checking                                                                        | Migrations                                                                                                | Pool                                                            | Async                               | Hive fit                                           | Why it lost                                                                                                                                                                                 |
| --------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- | ----------------------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **sqlx**              | 0.9.0                                                                         | Compile-time `query!` against the DB (or offline cache). Runtime `query()`. No DSL. | Built-in `migrate!` + `sqlx-cli`. Per-crate `sqlx.toml`: `create-schemas`, schema-qualified `table-name`. | `sqlx::Pool` / `PgPool`                                         | Native tokio                        | Winner                                             | —                                                                                                                                                                                           |
| diesel + diesel-async | diesel 2.3.13, diesel-async 0.9.2, diesel_cli 2.3.13, diesel_migrations 2.3.2 | Compile-time query builder. `table!` modules are typed.                             | diesel CLI `up.sql` / `down.sql`. `AsyncMigrationHarness` in diesel-async.                                | Extra: r2d2 / bb8 / deadpool features on diesel-async           | Sync core. Async is a second crate. | Strong compile-time table types. Weak process fit. | Sync default. Extra crate for tokio. ORM-shaped `table!` objects leak across cells if crates allow the import. No first-party multi-schema `sqlx.toml` equivalent.                          |
| sea-orm               | 2.0.2 (`sea-orm-migration` 2.0.2, `sea-orm-cli` 2.0.2, `sea-query` 1.0.2)     | Dynamic ORM. 2.0 adds strongly-typed columns. Still not compile-time SQL.           | `sea-orm-migration`.                                                                                      | Owns a `sqlx::Pool` under `DatabaseConnection`.                 | Async via sqlx (`sqlx-postgres`).   | Extra ORM on the winner.                           | SeaORM sits on sqlx. ActiveModel / entity files pull persistence into the hexagon. Docs push `search_path` / `set_schema_search_path`. GraphQL Seaography and Pro RBAC are out of hive law. |
| tokio-postgres        | 0.7.18 (sync sibling `postgres` 0.19.14)                                      | None. String SQL.                                                                   | None. Pair with refinery 0.9.2.                                                                           | None. Pair with deadpool-postgres 0.14.2 or bb8-postgres 0.9.0. | Native tokio                        | Driver, not a stack.                               | Rebuilds sqlx: pool + migrate + compile-time checks. bb8-postgres last release 2024-12-09.                                                                                                  |
| ormlite               | 0.24.5                                                                        | ORM on sqlx.                                                                        | Not a hive primitive.                                                                                     | sqlx                                                            | Async via sqlx                      | Not a successor.                                   | Thin ORM. Same driver as sqlx with less hive value.                                                                                                                                         |

sqlx is not an ORM. Macros take ordinary SQL. The compiler talks to the development database (or the offline cache) and checks the statement. That matches a hexagonal adapter: the persistence adapter writes SQL, maps rows to domain types, and never exports a query-builder object.

Diesel wins on typed table modules. Those modules are the analog of a planted mongoose schema import. They lose because the hive wants adapters that own SQL, tokio without a wrapper crate, and per-cell migration config that sqlx 0.9 already ships.

SeaORM 2.0 is a real 2026 major. It still wraps sqlx. The hive persistence adapter should call sqlx, not ActiveModel.

tokio-postgres remains the low-level client. sqlx-postgres is the production driver built on that problem space. Use tokio-postgres only if the hive later needs pipelined copy or a custom protocol feature sqlx does not expose.

## Fit to hive

chapter 6 is the law. Mongo user becomes Postgres role. Named mongoose connection becomes named `PgPool`. Exclusive collections become exclusive schema. Unauthorized becomes `42501`.

### Port map

| chapter 6                                          | Rust hive                                                                                                                    |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| Same cluster, same database                        | One Postgres database. Schemas inside it.                                                                                    |
| One Mongo user per cell                            | One LOGIN role per cell. Password in the cell DSN.                                                                           |
| `connectionName = <cell>`                          | Composition root opens `PgPool` with that DSN. Cell adapter takes only that pool.                                            |
| Exclusive collections by exact name                | Exclusive schema. No table-prefix glob as the privilege.                                                                     |
| No `dropCollection` / `dropIndex`                  | Runtime role is not the owner. `DROP` is inherent to the owner and is not grantable.                                         |
| Prefixes are not privileges                        | Schema name is the privilege boundary. `ALTER DEFAULT PRIVILEGES IN SCHEMA` covers new tables in that schema only.           |
| Cell A against B's collection fails `Unauthorized` | Cell A against B's schema fails `42501`. Compiling B's SQL onto A's pool does not grant access.                              |
| Cell transaction on the named connection           | `pool.begin()` on that cell's pool.                                                                                          |
| Cross-cell transactions illegal                    | Two pools cannot share one `BEGIN`. Prepared transactions / 2PC stay out. Merge if same-request write atomicity is required. |
| Compile-time wall is chapter 3 (import cruise)     | Cargo crate wall. Persistence adapter must not import another cell's SQL, migrations, or row types.                          |
| Unnamed `InjectConnection` is a review reject      | A cell must not accept a shared or default pool.                                                                             |
| Legacy default connection                          | No legacy neighbours in this hive. Composition root holds no wildcard pool.                                                  |

### Schema-per-cell, not database-per-cell

A client connection reaches one database. Schemas inside that database are namespaces. A role without `USAGE` on a schema cannot use objects in it.

Database-per-cell is a harder wall and a worse port:

- Each extra database needs its own DSN, backup, and connection budget.
- Cross-database SQL is not ordinary SQL. Foreign data wrappers are a second system.
- InProc between cells stays in-process. The data wall is GRANT, not a catalog boundary.

Schema-per-cell matches exclusive collections. The schema is the collection namespace.

### Role and GRANT

Two roles per cell.

1. **Owner / migrator.** Owns the schema. Runs `CREATE` / `ALTER` / `DROP`. Used by `sqlx-cli` and `sqlx::migrate!` at boot or in a migrate job. Not used to serve requests.
2. **Runtime LOGIN.** Connects from the cell pool. DML only.

Pattern (names are examples):

```sql
CREATE ROLE cell_load_owner NOLOGIN;
CREATE ROLE cell_load LOGIN PASSWORD '...';
CREATE SCHEMA load AUTHORIZATION cell_load_owner;

GRANT CONNECT ON DATABASE hive TO cell_load;
GRANT USAGE ON SCHEMA load TO cell_load;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA load TO cell_load;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA load TO cell_load;

ALTER DEFAULT PRIVILEGES FOR ROLE cell_load_owner IN SCHEMA load
  GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO cell_load;
ALTER DEFAULT PRIVILEGES FOR ROLE cell_load_owner IN SCHEMA load
  GRANT USAGE, SELECT ON SEQUENCES TO cell_load;
```

Hardening:

- `REVOKE ALL ON SCHEMA public FROM PUBLIC`. Postgres 15+ already drops public `CREATE`. Still revoke `USAGE` on `public` if the hive does not use it.
- Do not `GRANT` the cell role onto another cell's schema.
- Do not `GRANT CREATE ON SCHEMA load TO cell_load`. Runtime must not create tables.
- Do not grant `TRUNCATE`, `REFERENCES`, `TRIGGER`, or `MAINTAIN` unless a cell SPI needs them.
- `GRANT ALL TABLES IN SCHEMA` is a schema glob. That is allowed because the schema is the exclusive unit. It is not a table-name prefix glob across schemas.
- A new table in the cell schema is a migration in that cell. Default privileges give the runtime role DML. A new schema is a role change in the same change.
  `SET ROLE` is not the wall. `SET ROLE` changes `current_user` on an existing session. The session user is still the login. Named connection carries credentials. A shared superuser pool that `SET ROLE`s per request is one leaked connection from every cell. Open a pool as the cell LOGIN instead.

`search_path` is not the wall. Default is `"$user", public`. sqlx's own multi-tenant example warns that a wide `search_path` makes unqualified `_sqlx_migrations` hit the wrong schema. Qualify every table as `load.shipments`. Pin `search_path` to the cell schema on the role (`ALTER ROLE cell_load SET search_path = load`) only as a belt. GRANT remains the privilege.

### Connection-per-cell

`Pool` is `Send + Sync + Clone`. Create it once. Share the handle inside the cell.

Composition root:

1. Read per-cell DSNs from env (`LOAD_DATABASE_URL`, …). sqlx 0.9 `sqlx.toml` `common.database-url-var` does this for macros and `sqlx-cli`.
2. `PgPool::connect` (or `PoolOptions` with `max_connections` from env).
3. Pass the pool into that cell's persistence adapters only.

A cell transaction is `pool.begin()`. Nested savepoints exist. They stay inside one connection, so they stay inside one cell.

Postgres default `max_connections` is typically 100, minus superuser reserves. Sum of per-cell `max_connections` must stay under the server cap.

### Compile-time vs runtime

Runtime wall: GRANT. Cell A issues `SELECT * FROM quote.quotes` on the load pool. Postgres returns `42501`.

Compile-time wall: crate graph plus sqlx macros.

- `query!` / `query_as!` check SQL against the crate's database URL. Schema-qualified names appear in the SQL string. A planted `quote.quotes` in the load adapter is a review reject and, if the compile-time role lacks `USAGE` on `quote`, a compile failure.
- Offline mode is always on in current sqlx. CI can prepare without a live DB.
- Do not compile macros against a superuser URL that sees every schema. That hides planted cross-schema SQL.

Diesel's `table!` module is a stronger typed import ban if cells do not depend on each other. sqlx still wins because the hive persistence layer should own SQL strings, not a global DSL.

### Migrations

sqlx 0.9 first-party shape (from the multi-tenant example):

```toml
# cells/load/sqlx.toml
[migrate]
create-schemas = ["load"]
table-name = "load._sqlx_migrations"
```

Each cell crate owns `migrations/`. `sqlx::migrate!()` embeds them. `sqlx-cli` reads `sqlx.toml`. Run migrate per crate, as the example does (`(cd accounts && sqlx db setup)`).

Migrator uses the owner role. Runtime pool uses the LOGIN role. Mixing those credentials lets a request `DROP TABLE`.

### What stays out of the hexagon

sqlx types (`PgPool`, `Transaction`, row structs) live in `infrastructure/persistence`. Domain SPI stays pool-free. Published Language stays JSON primitives. Kernel stays framework-free.

## Sources

Crate versions (crates.io API, 2026-09-11):

- https://crates.io/crates/sqlx (`max_stable_version` 0.9.0, 2026-05-21)
- https://crates.io/crates/sqlx-cli (0.9.0)
- https://crates.io/crates/diesel (2.3.13, 2026-09-04)
- https://crates.io/crates/diesel-async (0.9.2)
- https://crates.io/crates/diesel_cli (2.3.13)
- https://crates.io/crates/diesel_migrations (2.3.2)
- https://crates.io/crates/sea-orm (2.0.2, 2026-08-12)
- https://crates.io/crates/sea-orm-migration (2.0.2)
- https://crates.io/crates/sea-orm-cli (2.0.2)
- https://crates.io/crates/sea-query (1.0.2)
- https://crates.io/crates/tokio-postgres (0.7.18, 2026-06-12)
- https://crates.io/crates/postgres (0.19.14)
- https://crates.io/crates/deadpool-postgres (0.14.2)
- https://crates.io/crates/bb8-postgres (0.9.0, 2024-12-09)
- https://crates.io/crates/refinery (0.9.2)
- https://crates.io/crates/ormlite (0.24.5)

sqlx:

- https://docs.rs/crate/sqlx/0.9.0 — compile-time SQL, not an ORM, `Pool`, `migrate` feature
- https://github.com/launchbadge/sqlx/blob/main/CHANGELOG.md — 0.9.0 (2026-05-06): `sqlx.toml`, multi-database `DATABASE_URL` rename, `_sqlx_migrations` rename, multi-tenant example; repo moving to https://github.com/transact-rs/
- https://github.com/launchbadge/sqlx/blob/main/sqlx-core/src/config/reference.toml — `database-url-var`, `migrate.table-name`, `migrate.migrations-dir`
- https://github.com/launchbadge/sqlx/blob/main/examples/postgres/multi-tenant/README.md — one crate per schema, schema-qualified names, do not rely on `search_path`
- https://github.com/launchbadge/sqlx/blob/main/examples/postgres/multi-tenant/accounts/sqlx.toml — `create-schemas`, `table-name = "accounts._sqlx_migrations"`
- https://docs.rs/sqlx/0.9.0/sqlx/struct.Pool.html — `Send + Sync + Clone`, `begin()`, create once
- https://docs.rs/sqlx/0.9.0/sqlx/macro.migrate.html — embed migrations, `sqlx.toml` schema create / table rename

diesel:

- https://diesel.rs/ — compile-time correctness, Diesel 2.3
- https://diesel.rs/guides/getting-started/ — diesel CLI, `up.sql` / `down.sql`, generated `schema.rs`, `PgConnection`
- https://docs.rs/diesel/2.3.13/diesel/macro.table.html — `table!` typed modules
- https://docs.rs/diesel-async/0.9.2/diesel_async/ — async IO on diesel, `AsyncPgConnection`, pool features, `AsyncMigrationHarness`

sea-orm:

- https://www.sea-ql.org/SeaORM/docs/introduction/whats-new/ — 2.0
- https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime/ — `sqlx-postgres` driver
- https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/ — owns `sqlx::Pool`; `options=--search_path=`; `set_schema_search_path`
- https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-format/ — `schema_name` on entities; ActiveModel

tokio-postgres:

- https://docs.rs/tokio-postgres/0.7.18/tokio_postgres/ — async client, no pool, no migrations

Postgres 18:

- https://www.postgresql.org/docs/current/ddl-schemas.html — schemas vs databases; connection sees one database; `USAGE` required; secure `$user` pattern; `REVOKE CREATE ON SCHEMA public FROM PUBLIC`
- https://www.postgresql.org/docs/current/ddl-priv.html — privileges; owner-only drop/alter; no default table grants to `PUBLIC`
- https://www.postgresql.org/docs/current/sql-grant.html — `GRANT … ON SCHEMA`; `ALL TABLES IN SCHEMA`; `CONNECT` on database
- https://www.postgresql.org/docs/current/sql-alterdefaultprivileges.html — future tables in a schema
- https://www.postgresql.org/docs/current/sql-set-role.html — `SET ROLE` is session-scoped; not a login
- https://www.postgresql.org/docs/current/errcodes-appendix.html — `42501` `insufficient_privilege`
- https://www.postgresql.org/docs/current/tutorial-transactions.html — `BEGIN` / `COMMIT` on one session
- https://www.postgresql.org/docs/current/manage-ag-templatedbs.html — `CREATE DATABASE` copies a template (ops cost of database-per-cell)
