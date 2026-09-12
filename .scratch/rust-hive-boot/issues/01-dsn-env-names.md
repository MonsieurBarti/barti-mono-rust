# What DSN env names do migrate and serve use?

Type: grilling
Status: resolved

Label: wayfinder:grilling
Blocked by:

## Question

Name the environment variables `app` reads to open Postgres.

Hive law: migrate env holds migrator DSNs and runs each cell's `sqlx::migrate!`. Serve env holds only cell-role DSNs and never migrates. Pool size is env. `query!` / `query_as!` compile against the cell-role DSN, never a superuser URL. This map has one cell, `loads`.

Decide the env names for the loads migrator DSN, the loads cell-role DSN, pool size, and any sqlx compile-time URL. Decide whether HTTP bind is env.

Do not pick the compose file. Do not pick Load fields.

## Answer

Migrate reads `LOADS_MIGRATOR_DATABASE_URL`. Serve reads `LOADS_DATABASE_URL` and `LOADS_POOL_MAX`. Serve does not read the migrator DSN. Migrate does not read the cell-role DSN or the pool size.

`LOADS_POOL_MAX` is required on serve. Unset or unparseable refuses to boot.

`crates/cells/freight/loads/sqlx.toml` sets `database-url-var` to `LOADS_DATABASE_URL`. `query!`, `query_as!`, and `cargo sqlx prepare` use that env. They never use `DATABASE_URL`. They never use the migrator DSN.

HTTP bind is not env. Bind stays `127.0.0.1:8080`.

## Comments

### Round 1

Four arrows accepted:

- Q1 A: `LOADS_MIGRATOR_DATABASE_URL` and `LOADS_DATABASE_URL`. `DATABASE_URL` stays unused.
- Q2 A: `LOADS_POOL_MAX` required on serve. Unset or unparseable refuses to boot. Migrate ignores it.
- Q3 A: `sqlx.toml` `database-url-var` is the cell-role env. No second compile-time var. Never `DATABASE_URL`. Never the migrator env.
- Q4 A: HTTP bind is not env. Bind stays `127.0.0.1:8080`.

Frontier empty for this ticket. Redis is not a Postgres DSN. It is not this answer.

