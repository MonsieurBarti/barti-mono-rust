# How do we observe a REST hive process?

Type: grilling
Status: resolved
Label: wayfinder:grilling
Blocked by: 10

## Question

Port chapter 13 onto REST-only axum. Do not assume Datadog.

Locked: public HTTP is REST; problem+json carries `correlationId` when present. Surface: [What is the public driving-adapter surface?](10-public-driving-adapters.md).

Decide: vendor; who owns tracing, structured logging, and request context; cell-local Logger and Metrics SPIs; `X-Correlation-ID`; metric name pattern; one wide event per REST request.

Do not design tick or queue log lines here. AuthN is [How does AuthN/AuthZ work at the REST process edge?](13-authn-rest-edge.md). Do not write the chapter here.

## Answer

`app` owns OpenTelemetry. `tracing` plus OTLP live in `telemetry.rs`. The sink is ops, not a hive crate. Cells import none of `tracing`, OpenTelemetry, or a task-local crate. Crate majors wait on [Write the stack-pin ADR](22-stack-pin-adr.md).

Each cell declares a Logger SPI and a Metrics SPI under `domain/spi/`. Domain never logs and never counts. Application, infrastructure, and presentation may. Both SPIs are synchronous and non-throwing. `CorrelationId` and `ActorId` are not SPI arguments. `app` mixes `correlationId`, `actorId` when present, and `source` (`api` or `webhook`). No `organizationId`. No `ip`. No `userAgent` on log lines. Implementations live in `app` and bind as generics on `new`. Tests pass fakes. Trait types wait on [What lives in the kernel crate?](20-kernel-crate.md).

Header `X-Correlation-ID`. Optional. Accept UUID v4 or v7. Generate UUIDv7 when missing. Never 400. Echo the header on every HTTP response, including health. Problem+json still carries `correlationId` in the body. Tag the OpenTelemetry root span with `correlationId`. The trace id is a different id. Do not copy `CorrelationId` into `trace_id`. Clients need not send `traceparent`. Baggage stays out until a second hive process. InProc reads request context. Domain events do not carry `CorrelationId`. `CorrelationId` is not an idempotency key.

Metrics names are `hive.<area>.<subject>`. Bounded tag `cell:`. Untrusted tag values allowlisted. Never hand-tag `env` / `service` / `version`. No `_total` suffix. First cells fill `<area>` when named.

One wide event per REST request that hits a cell handler, including unpublished webhooks. Not per InProc hop. Health emits none. `debug` on success at or under 3 s, `info` when slow, `warn` on 4xx, `error` on 5xx. Dotted `msg`. Production `LOG_LEVEL=info`. Fields: `msg`, `correlationId`, `actorId` when present, HTTP method and path, `duration_ms`, HTTP `status`, envelope `type` on errors, `source`. Never log JWT bodies, request or response bodies with user content, full names, emails, phones, or IPs. Redact keys at any depth: `password`, `token`, `secret`, `authorization`, `creditCard`, `creditCardNumber`, `cvv`, `accessToken`, `refreshToken`, `email`, `phone`, `phoneNumber`.

Tick and queue lines wait on [Where do ticks, queues, and workers live?](18-workers-and-ticks.md). Mutating REST idempotency is [How are mutating REST commands made idempotent?](23-rest-command-idempotency.md).

Glossary: [CONTEXT.md](../../../CONTEXT.md) **Logger SPI**, **Metrics SPI**, **CorrelationId**, **Wide event**.

## Comments

### Round 1

Six arrows accepted:

- Q1 A: OpenTelemetry in `app` via `tracing` plus OTLP. Cells import none of those crates. The sink is ops, not a hive crate. Stack-pin ADR owns majors.
- Q2 A: `app` owns tracing, structured logging, and request context in `telemetry.rs`. Cells import none of the vendor crates, none of `tracing`, none of a task-local crate. InProc reads the same request context. Domain events do not carry `correlationId`.
- Q3 A: cell-local Logger SPI and Metrics SPI under `domain/spi/`. Domain never logs and never counts. Application, infrastructure, and presentation may. Both SPIs are synchronous and non-throwing. `correlationId` and `ActorId` are not SPI arguments. Tests fake the SPIs. Kernel types wait on [What lives in the kernel crate?](20-kernel-crate.md). Adapter location is round 2.
- Q4 A: header `X-Correlation-ID`. Optional. Accept v4 or v7. Generate UUIDv7 when missing. Never 400. Echo `correlationId` on problem+json when present. Tag the root span. Health does not require the header.
- Q5 A: `hive.<area>.<subject>`. Bounded tag `cell:`. Untrusted tag values allowlisted. Never hand-tag `env` / `service` / `version`. No `_total` suffix. First cells fill `<area>` when named.
- Q6 A: one wide event per REST request that hits a cell handler, including unpublished webhooks. Not per InProc hop. Health emits none. `debug` on success at or under 3 s, `info` when slow, `warn` on 4xx, `error` on 5xx. Dotted `msg`. Production `LOG_LEVEL=info`. Never log JWT bodies, request or response bodies with user content, full names, emails, phones, or IPs.

### Round 2

Five arrows accepted:

- Q7 A: Logger and Metrics implementations live in `app`, passed into `new` as generics. The cell crate declares the SPI. Tests pass fakes. No cell file imports `tracing` or OpenTelemetry.
- Q8 A: echo `X-Correlation-ID` on every HTTP response, success and problem+json, including health. Body field stays problem+json only.
- Q9 A: mix `correlationId`, `actorId` when present, `source` `api` or `webhook`. No `organizationId`. No `ip`. No `userAgent` on log lines.
- Q10 A: OpenTelemetry trace id and `CorrelationId` both exist and differ. Do not copy `CorrelationId` into `trace_id`. Clients need not send `traceparent`. Tag the root span with `correlationId`. Baggage out until a second hive process.
- Q11 A: wide event fields `msg`, `correlationId`, `actorId` when present, HTTP method and path, `duration_ms`, HTTP `status`, envelope `type` on errors, `source`. Redact key list.

### Round 3

Two arrows accepted:

- Q12 A: `CorrelationId` is not an idempotency key. An idempotency key, if a cell has one, is a different field. Each HTTP attempt emits one wide event.
- Q13 B: graduate a grilling ticket How are mutating REST commands made idempotent? Blocked by this ticket and [What is the public driving-adapter surface?](10-public-driving-adapters.md). Do not design the store or header here.

### Round 4

One arrow accepted:

- Q14: shared understanding confirmed. Close this ticket. Graduate [How are mutating REST commands made idempotent?](23-rest-command-idempotency.md).
