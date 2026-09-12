# Boot app migrate, serve, telemetry, and HTTP

Type: task
Status: resolved


Label: wayfinder:task
Blocked by: 01, 04, 05

## Question

Give `crates/app` a migrate entry, a serve entry, telemetry, and an HTTP edge a human can hit.

Migrate reads migrator DSNs from [What DSN env names do migrate and serve use?](01-dsn-env-names.md) and runs `sqlx::migrate!` for `loads`. Serve reads only cell-role DSNs, opens the named loads pool, binds `SystemClock` plus Logger and Metrics implementations, and never migrates.

`telemetry.rs` owns `tracing` plus OTLP. Cells import none of those crates. `http.rs` serves `GET /health` with no ActorId, echoes `X-Correlation-ID`, copies `X-Actor-Id` into extensions, and nests the loads `router` when that export exists. Missing ActorId on human REST is 401 `application/problem+json` with `type` `UNAUTHENTICATED`. Bind `127.0.0.1:8080`. Pins from ADR 0001.

Use tdd. App tests may boot `app`. Cell tests still must not. Loads handlers can wait on [POST a Load with Idempotency-Key](08-create-load-post.md); health and identity must work now.

Do not write loads tables. Do not add a second cell.

## Answer

`app` is one binary with two entries. `app migrate` reads `LOADS_MIGRATOR_DATABASE_URL` only and runs `sqlx::migrate!` for the loads cell. `app serve` reads `LOADS_DATABASE_URL` and `LOADS_POOL_MAX` only, opens the named loads pool, binds `SystemClock`, `TracingLogger`, and `AppMetrics`, and never migrates. Unknown argument exits 2.

`crates/app/sqlx.toml` sets `table-name = "loads._sqlx_migrations"`, so bookkeeping lives in the cell schema and migrate never writes `public`. `crates/cells/freight/loads/sqlx.toml` sets `database-url-var = "LOADS_DATABASE_URL"`, so cell macros compile against the cell role. The loads `migrations/` directory is empty; ticket 07 fills it.

`telemetry.rs` owns `tracing`, `tracing-subscriber` JSON, and OTLP over HTTP. `LOG_LEVEL` defaults to `info`. OTLP starts only when `OTEL_EXPORTER_OTLP_ENDPOINT` is set, so serve runs with no collector. Kernel `Logger` and `Metrics` implementations live here. The loads crate gained no telemetry dependency.

`http.rs` layers correlation outside identity. Correlation accepts UUID v4 or v7, mints v7 when missing or unparseable, never 400s, and echoes `X-Correlation-ID` on every response including health. Identity exempts `GET /health`, copies `X-Actor-Id` into extensions, and answers missing or empty identity with 401 `application/problem+json` carrying `type`, `status`, `detail`, `instance`, and `correlationId`, with no `title` and no `context`. Bind is `127.0.0.1:8080`, not env.

`loads` exports no `router` yet, so `http.rs` nests nothing and carries the seam comment for ticket 08.

Proof: 51 tests green (18 app, 33 kernel), `cargo fmt --check`, `cargo clippy --workspace --all-targets -D warnings`, and `cargo deny check bans` clean. Against a real `postgres:18`, `app migrate` created `loads._sqlx_migrations`, and `app serve` answered health 200 with a minted v7, echoed a supplied v4, and returned the 401 problem document on `/loads`.

A host-native Postgres already owns `127.0.0.1:5432`, so the compose port lost the bind. The smoke test used `5433`. Compose itself is unchanged.

