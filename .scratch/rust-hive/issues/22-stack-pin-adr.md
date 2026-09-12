# Write the stack-pin ADR

Type: task
Status: resolved
Label: wayfinder:task
Blocked by: 12, 13, 17

## Question

Write the stack-pin ADR named in the destination.

Pins already locked: axum 0.8.9; tower-http 0.6; sqlx 0.9.0; serde + garde; crate-per-cell workspace; `time` 0.3; `uuid` 1.26. Add whatever [What are the test lanes, and what does each one boot?](12-test-lanes.md), [How does AuthN/AuthZ work at the REST process edge?](13-authn-rest-edge.md), [How do we observe a REST hive process?](17-observability.md), and [How are Money, ids, and dates encoded in Published Language?](19-pl-scalars.md) lock.

Latest crate versions only. Majors in the architecture doc; this ADR owns minors and patches. Path: `docs/adr/0001-stack-pins.md`. Create `docs/adr/` if needed.

This task writes the ADR. It does not write `docs/architecture.md`.

## Answer

Wrote [docs/adr/0001-stack-pins.md](../../../docs/adr/0001-stack-pins.md).

Locked pins stay at research versions: axum 0.8.9; tokio 1.53.1; hyper 1.11.1; tower 0.5.3; tower-http 0.6.11 not 0.7; sqlx 0.9.0; sqlx-cli 0.9.0; serde 1.0.229; serde_json 1.0.151; serde_path_to_error 0.1.20; garde 0.23.0; rustc 1.98.1; edition 2024; cargo-deny 0.20.2.

New pins from later tickets, latest crates.io 2026-09-12: time 0.3.55; uuid 1.26.1; tracing 0.1.44; tracing-subscriber 0.3.23; tracing-opentelemetry 0.33.0; opentelemetry 0.32.0; opentelemetry_sdk 0.32.1; opentelemetry-otlp 0.32.0; cargo-nextest 0.9.144; cargo-llvm-cov 0.9.1.

Not pinned: JWT, CQRS bus, queue, cron, OpenAPI, GraphQL, MCP, testcontainers, mockall as law, tonic.

This ticket does not write `docs/architecture.md`.

