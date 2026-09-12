# Stack pins

Hive law pins exact crate and tool versions here. `docs/architecture.md` restates majors only. Bump a pin by changing this ADR.

`tower-http` stays on 0.6.11. `axum` 0.8.9 depends on `tower-http ^0.6.8`. Do not take 0.7.

## Pins

| Crate or tool | Version | Home |
| --- | --- | --- |
| rustc | 1.98.1 | workspace |
| edition | 2024 | workspace |
| `axum` | 0.8.9 | `app`, cell `presentation/` |
| `tokio` | 1.53.1 | `app`, ticks, drains |
| `hyper` | 1.11.1 | with axum |
| `tower` | 0.5.3 | with axum |
| `tower-http` | 0.6.11 | `app` |
| `sqlx` | 0.9.0 | SeaORM driver. Cells depend on sea-orm, not sqlx. |
| `sea-orm` | 2.0.2 | cell sea-orm adapter |
| `sea-orm-migration` | 2.0.2 | cell migrations |
| `sea-orm-cli` | 2.0.2 | migrate CLI. Not generate-entity. |
| `serde` | 1.0.229 | cell edge |
| `serde_json` | 1.0.151 | cell edge |
| `serde_path_to_error` | 0.1.20 | cell edge |
| `garde` | 0.23.0 | `domain/api` |
| `time` | 0.3.55 | kernel only |
| `uuid` | 1.26.1 | application mint; not kernel |
| `tracing` | 0.1.44 | `app` `telemetry.rs` |
| `tracing-subscriber` | 0.3.23 | `app` |
| `tracing-opentelemetry` | 0.33.0 | `app` |
| `opentelemetry` | 0.32.0 | `app` |
| `opentelemetry_sdk` | 0.32.1 | `app` |
| `opentelemetry-otlp` | 0.32.0 | `app` |
| `cargo-deny` | 0.20.2 | import wall |
| `cargo-nextest` | 0.9.144 | `cargo nextest run` |
| `cargo-llvm-cov` | 0.9.1 | 80% line per cell |

Kernel depends on `time` only. Kernel is sqlx-free and sea-orm-free. Cells import none of `tracing`, OpenTelemetry, or a task-local crate. Cell e2e never boots `app`.

## Not pinned

No JWT crate. This process copies gateway identity headers. It does not validate human Bearer JWTs.

No CQRS bus crate. No queue crate. No cron crate. No OpenAPI crate. No GraphQL crate. No MCP crate. No testcontainers. mockall is not law.

`tonic` stays off until a cell needs gRPC.

