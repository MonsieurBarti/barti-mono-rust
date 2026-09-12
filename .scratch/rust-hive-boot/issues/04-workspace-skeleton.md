# Scaffold the Cargo workspace

Type: task
Status: resolved

Label: wayfinder:task
Blocked by: 01, 02

## Question

Create the virtual workspace so later tickets have crates to fill.

Write `crates/kernel`, `crates/cells/freight/loads`, and `crates/app`. Package names are `kernel`, `loads`, and `app`. Domain folder `freight` has no `Cargo.toml`. Pin rustc 1.98.1 and edition 2024 from ADR 0001. Add `cargo-deny` bans that reject cell to cell, cell to `app`, and kernel to cell, with `wrappers = ["app"]` per cell crate.

Write the compose file and env example from [What compose file boots local Postgres?](02-compose-file.md) and [What DSN env names do migrate and serve use?](01-dsn-env-names.md). Crates must compile. No product behavior. Use tdd only for a compile smoke, not domain tests.

Do not export kernel types. Do not wire HTTP. Do not write loads migrations.

## Answer

Virtual workspace, resolver 3, rustc 1.98.1, edition 2024. Members are `crates/kernel` (rlib `kernel`), `crates/cells/freight/loads` (rlib `loads`), and `crates/app` (bin `app`). Domain folder `freight` has no `Cargo.toml`.

`deny.toml` bans `loads` with `wrappers = ["app"]`. That rejects cell to cell and kernel to cell. Cell to `app` is a cargo error: `app` has no lib.

`compose.yaml` boots `postgres:18` on `127.0.0.1:5432`, database `hive`, named volume at `/var/lib/postgresql`. Init is `postgres/init.sql`. `.env.example` names `LOADS_MIGRATOR_DATABASE_URL`, `LOADS_DATABASE_URL`, and `LOADS_POOL_MAX=10`.

Compile smoke is `cargo test --workspace`. No kernel exports, no HTTP, no loads migrations.

