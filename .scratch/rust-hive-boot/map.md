# Boot a Cargo workspace you can curl

Label: wayfinder:map

## Destination

A booting Cargo workspace you can run locally against Postgres. Kernel rlib, app bin, freight/loads cell, one human POST that creates a Load, nextest green on unit + integration + that e2e. Something a human can curl.

## Notes

- Domain: greenfield Rust hive-strict. Host product: B2B freight brokerage. First language: FTL road.
- Skills every session: grilling, domain-modeling, codebase-design. Use tdd when a ticket writes code. Tracker: `docs/agents/issue-tracker.md`.
- This map implements. Pins and majors come from [ADR 0001](../../docs/adr/0001-stack-pins.md) and [docs/architecture.md](../../docs/architecture.md). Do not rewrite them. Do not re-grill them.
- Virtual workspace: `crates/kernel`, `crates/cells/freight/loads`, `crates/app`. Hive-strict. InProc lives in `app`. Leaving SPIs bind as generics.
- Public HTTP is REST on axum. Human POST requires `Idempotency-Key`. One database, schema per cell, two LOGINs, named pool, sqlx 0.9.
- Tests: unit=domain, integration=application with real sqlx, e2e=`router`. Never boot `app` from a cell test.
- Glossary is [CONTEXT.md](../../CONTEXT.md). Quote and Stop live in `loads`. Book, shipments, and settlement stay out of this map.
- Standing for this map only, not hive law: ActorId header is `X-Actor-Id`. Serve bind is `127.0.0.1:8080`. Health is `GET /health` on `app`. Create-Load path is `POST /loads`. Load is a state table. GRANT proof plants a second schema in compose init; it is not a second cell. Compose is Postgres only.

## Decisions so far

- [Rust hive architecture for a freight brokerage](../rust-hive/map.md) — hive law locked in `docs/architecture.md`, `docs/adr/0001-stack-pins.md`, and `CONTEXT.md`.
- [What DSN env names do migrate and serve use?](issues/01-dsn-env-names.md) — `LOADS_MIGRATOR_DATABASE_URL`, `LOADS_DATABASE_URL`, `LOADS_POOL_MAX`; sqlx.toml points macros at the cell-role env; bind is not env.
- [What compose file boots local Postgres?](issues/02-compose-file.md) — `compose.yaml` plus `postgres/init.sql`; `postgres:18` on `127.0.0.1:5432`; database `hive`; `loads` schema with `loads_migrator` / `loads`; plant `grant_proof.probe`.
- [Which Load fields does the first POST require?](issues/03-create-load-fields.md) — body `shipperId` + two typed stops; mint Load/Stop ids and `createdAt`; 201 is the Load; `ActorId` stored, not JSON.
- [Scaffold the Cargo workspace](issues/04-workspace-skeleton.md) — virtual workspace `kernel` / `loads` / `app`; rustc 1.98.1, edition 2024; cargo-deny `wrappers = ["app"]` on `loads`; `compose.yaml` plus `postgres/init.sql`; `.env.example`.
- [Export the kernel crate](issues/05-kernel-exports.md) — chapter 15 export set; crate dep `time` 0.3.55; ISO 4217 list-one 2026-01-01.
- [Boot app migrate, serve, telemetry, and HTTP](issues/06-app-migrate-serve.md) — `app migrate` / `app serve` split by DSN; `loads._sqlx_migrations` via app `sqlx.toml`; `telemetry.rs` owns tracing plus optional OTLP; health, correlation echo, and 401 problem+json on `127.0.0.1:8080`.
- [Persist a Load through the write SPI](issues/07-loads-write-spi.md) — `loads.load` / `loads.stop` / `loads.idempotency_key`; write SPI `get_by_id` / `save`; sqlx adapter holds `LoadsPool`; fake plus `contract` feature.




## Not yet specified

- Quote HTTP, GET Load, and list. This destination is one create POST.
- A local OTLP collector. Serve may run with no collector.
- CI Postgres as a service. This destination is local compose plus nextest.

## Out of scope

- `shipments` and `settlement` cells.
- Gateway product, IdP, JWT crate, SSO pages.
- Frontend, GraphQL, MCP, OpenAPI.
- Extracting a cell, a second process, a queue crate.
- Rewriting `docs/architecture.md` or ADR 0001.
- Ticks, work table, and outbox. This `loads` cut has no ticks.
- Redis. This destination is Postgres.

