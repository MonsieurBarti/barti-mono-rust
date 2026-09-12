# Boot app migrate, serve, telemetry, and HTTP

Type: task
Label: wayfinder:task
Blocked by: 01, 04, 05

## Question

Give `crates/app` a migrate entry, a serve entry, telemetry, and an HTTP edge a human can hit.

Migrate reads migrator DSNs from [What DSN env names do migrate and serve use?](01-dsn-env-names.md) and runs `sqlx::migrate!` for `loads`. Serve reads only cell-role DSNs, opens the named loads pool, binds `SystemClock` plus Logger and Metrics implementations, and never migrates.

`telemetry.rs` owns `tracing` plus OTLP. Cells import none of those crates. `http.rs` serves `GET /health` with no ActorId, echoes `X-Correlation-ID`, copies `X-Actor-Id` into extensions, and nests the loads `router` when that export exists. Missing ActorId on human REST is 401 `application/problem+json` with `type` `UNAUTHENTICATED`. Bind `127.0.0.1:8080`. Pins from ADR 0001.

Use tdd. App tests may boot `app`. Cell tests still must not. Loads handlers can wait on [POST a Load with Idempotency-Key](08-create-load-post.md); health and identity must work now.

Do not write loads tables. Do not add a second cell.
